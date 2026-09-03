//! 登录态获取：起一次脱机 scegame 做真实 entrance 登录，从游戏日志读出 userid。
//! **完全不依赖编辑器**（用脱机 scegame 自己的登录链）。
//!
//! 原理（credential-userid.md §4）：凭证文件不含 userid（login 字段是登录状态 0/1，
//! kid 是 opaque 随机字节）。userid 只在 entrance 登录响应的内存态 latest_login_info 里。
//! 脱机 scegame 的 startup/entrance 链自动 account.login()（win 平台 auto_guest 分支，
//! 凭证有效即 token 登录）→ 跑 app_box 默认图时 GamePlayOnline 把 userid 打进游戏日志
//! `GamePlayOnline request login, userid: <N>, username: <N>`（2026-08-21 实测）。
//! 读 logs/game/ 最新日志抓该行即得 userid——无需改 pak（载荷 lua 是包级 TNND 加密，改明文太重）。

use crate::core::auth::UserInfo;
use crate::core::debug::spawn_detached_pub;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// 登录态结果
#[derive(Debug, Clone, serde::Deserialize)]
pub struct LoginIdentity {
    pub user_id: String,
    #[serde(default)]
    pub user_name: String,
    #[serde(default)]
    pub login_way: Option<serde_json::Value>,
}

impl LoginIdentity {
    pub fn userid_i64(&self) -> Option<i64> {
        self.user_id.parse::<i64>().ok()
    }
    pub fn user_name_opt(&self) -> Option<String> {
        let n = self.user_name.trim();
        if n.is_empty() || n == "nil" || n == "Not_Logon" || n.chars().all(|c| c.is_ascii_digit()) {
            // username 暂与 userid 相同（数字）时视为无昵称
            None
        } else {
            Some(n.to_string())
        }
    }
}

/// 起一次脱机游戏客户端做真实登录，读回 userid/user_name。
/// runtime_dir = 载荷目录（含 scegame.exe 或 version-<api>/SCE）；凭证 cred 会先注入 runtime/User/。
/// timeout 内未拿到则报错（凭证失效/网络问题）。
pub fn fetch_identity(runtime_dir: &Path, cred: &UserInfo, env_domain: &str, timeout: Duration) -> Result<LoginIdentity> {
    let (kind, exe) = crate::core::runtimes::detect_client_exe(runtime_dir)
        .ok_or_else(|| anyhow!("游戏客户端不存在于 {}（先 payload sync）", runtime_dir.display()))?;
    // 注入凭证（客户端 account 启动时读取）。
    // 关键：login 字段必须置 1——startup/entrance/main.lua 的 after_update() 里
    // `account.get_login_state() == 1` 才会自动 account.login()（editor-patch 镜像 client_base-78 实证），
    // 否则大厅停在「显示登录按钮」等人点，永远走不到 GamePlayOnline。
    let mut injected = cred.clone();
    injected.login = 1;
    let user_info_path = runtime_dir
        .join("User")
        .join(format!("user_info-{env_domain}.json"));
    crate::core::auth::write_user_info(&user_info_path, &injected)?;

    // 记录现有最新日志（路径 + 长度）：客户端可能复用同一日志文件追加写（按日期命名等），
    // 只比路径会永远跳过读取 → 同路径但长度前进也视为有新内容
    let log_dir = runtime_dir.join("logs").join("game");
    let before = latest_log(&log_dir);

    // 起客户端：大厅链（自动登录 → app_box 默认图 → 打 userid 日志）
    let mut args = vec![
        "-env=game".to_string(),
        format!("-server={env_domain}"),
        "-use_local_res".to_string(),
        "-no_update".to_string(),
        "-width=640".to_string(),
        "-height=480".to_string(),
    ];
    // 编辑器运行时（version-<api>/SCE）需要声明 api 版本才会走游戏模式
    if let crate::core::runtimes::RuntimeKind::EditorApi(n) = kind {
        args.push(format!("-editor_api_version={n}"));
    }
    let pid = spawn_detached_pub(&exe, &args, runtime_dir)?;

    // 轮询日志抓 userid 行
    let deadline = Instant::now() + timeout;
    let marker = "GamePlayOnline request login, userid:";
    loop {
        if let Some((log_path, len)) = latest_log(&log_dir) {
            let is_new = match &before {
                Some((b, blen)) => log_path != *b || len > *blen,
                None => true,
            };
            if is_new {
                if let Ok(text) = std::fs::read_to_string(&log_path) {
                    if let Some(id) = parse_userid_from_log(&text, marker) {
                        let _ = kill_pid(pid);
                        return Ok(id);
                    }
                }
            }
        }
        if Instant::now() >= deadline {
            let _ = kill_pid(pid);
            return Err(anyhow!(
                "登录态获取超时（{}s）——凭证可能已失效或网络不通",
                timeout.as_secs()
            ));
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

/// 最新日志（路径, 长度）：长度供调用方判断同路径文件是否有追加
fn latest_log(dir: &Path) -> Option<(PathBuf, u64)> {
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter(|e| e.path().extension().map(|x| x == "log").unwrap_or(false))
        .filter_map(|e| {
            let m = e.metadata().ok()?;
            Some((e.path(), m.modified().ok()?, m.len()))
        })
        .max_by_key(|(_, m, _)| *m)
        .map(|(p, _, len)| (p, len))
}

/// 从日志文本抓最后一处 `GamePlayOnline request login, userid: <N>, username: <N>`
fn parse_userid_from_log(text: &str, marker: &str) -> Option<LoginIdentity> {
    let mut found: Option<LoginIdentity> = None;
    for line in text.lines() {
        if let Some(idx) = line.find(marker) {
            let rest = &line[idx + marker.len()..];
            let uid: String = rest
                .trim_start()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if !uid.is_empty() {
                let uname = rest
                    .split("username:")
                    .nth(1)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default();
                found = Some(LoginIdentity {
                    user_id: uid,
                    user_name: uname,
                    login_way: None,
                });
            }
        }
    }
    found
}

fn kill_pid(pid: u32) -> Result<()> {
    let _ = crate::core::silent_command("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    Ok(())
}
