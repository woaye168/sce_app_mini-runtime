//! 脱机调试（方案 B）：自托管编排——暂存目录 → assign_host → 控制协议上传+起局 → 拉起 scegame 客户端。
//! 全程不碰编辑器；链路实证见 doc/research/scegame-reverse.md §7/§8。

use crate::core::auth::UserInfo;
use crate::core::host::{self, HostControl};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// 调试 host 模式：云端直连（默认）/ 本地中继（观测抓包）/ 真本地（脱机 lua 服务端，0.5.0 R3+R4）
/// 0.5.0 语义切换：CLI/GUI 的 local 从「中继」改为「真本地」，中继由 relay 承接（Release notes 已标注）
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HostMode {
    Cloud,
    Relay,
    Local,
}

/// 调试会话参数
pub struct DebugParams {
    /// 项目根目录（含 project.sce / project/map_settings.json）
    pub project_root: PathBuf,
    /// 运行时载荷目录（含 scegame.exe / Update/ / User/）
    pub runtime_dir: PathBuf,
    /// 暂存目录（-map_path）；为 None 时自动生成（core/staging.rs，白名单复制+入口包装）
    pub staging_dir: Option<PathBuf>,
    /// 当前凭证
    pub cred: UserInfo,
    /// 账号数字 userid（控制连接与客户端 -user 都要）
    pub userid: i64,
    /// 环境域（默认 editor-pd.spark.xd.com）
    pub env_domain: String,
    /// 运行时种类（缺省 = 编辑器-api<项目 api_version>）
    pub runtime_kind: Option<crate::core::runtimes::RuntimeKind>,
    /// host 模式（缺省 = 云端直连）
    pub host_mode: HostMode,
    /// 附加客户端（多开）：每个 = (凭证, userid)，在主客户端之后串行拉起（凭证注入互斥需间隔）
    pub extra_clients: Vec<(UserInfo, i64)>,
}

/// 调试会话
pub struct DebugSession {
    pid: u32,
    /// 附加客户端 pid（多开）
    extra_pids: Vec<u32>,
    pub started: Instant,
    pub runtime_dir: PathBuf,
    pub session_id: u64,
    /// 控制连接（持有以续收 host 日志；断开不影响局）
    pub ctl: Option<HostControl>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DebugStatus {
    Starting,
    Running,
    Exited(i32),
    Failed(String),
}

/// 读项目 project/map_settings.json：ProjectName + api_version（兼容字符串/数字/对象三种格式）
pub fn read_map_settings(project_root: &Path) -> Result<(String, u32)> {
    let path = project_root.join("project").join("map_settings.json");
    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("读 map_settings.json 失败 {}: {e}", path.display()))?;
    let v: Value = serde_json::from_str(&content)?;
    let name = v
        .get("ProjectName")
        .and_then(|x| x.as_str())
        .ok_or_else(|| anyhow!("map_settings.json 缺 ProjectName"))?
        .to_string();
    let api = match v.get("api_version") {
        Some(Value::Object(o)) => o
            .get("api_version")
            .and_then(|x| x.as_u64())
            .ok_or_else(|| anyhow!("api_version 对象缺数值"))? as u32,
        Some(Value::Number(n)) => n.as_u64().ok_or_else(|| anyhow!("api_version 数值异常"))? as u32,
        Some(Value::String(s)) => s.parse().map_err(|_| anyhow!("api_version 字符串异常: {s}"))?,
        None => return Err(anyhow!("map_settings.json 缺 api_version")),
        _ => return Err(anyhow!("api_version 格式未知")),
    };
    Ok((name, api))
}

/// 读项目 libs.json，版本号按载荷 api_pak_version.json[<api_version>] 注册表解析（与编辑器一致），
/// 注册表缺失时回退载荷 _m/maps/<subpath>/ 最大数字目录名。返回 [(库名, 版本号字符串)]
pub fn resolve_libs(project_root: &Path, runtime_dir: &Path, env_domain: &str) -> Result<Vec<(String, String)>> {
    let path = project_root.join("libs.json");
    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("读 libs.json 失败 {}: {e}", path.display()))?;
    let v: Value = serde_json::from_str(&content)?;
    let obj = v.as_object().ok_or_else(|| anyhow!("libs.json 非对象"))?;
    let update_root = runtime_dir.join("Update").join(env_domain);
    let maps_root = update_root.join("Res").join("_m").join("maps");
    // 官方版本注册表（api_pak_version.json[<api>] = {库短名: 版本}）
    let registry: Option<Value> = std::fs::read_to_string(update_root.join("api_pak_version.json"))
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok());
    let (_, api_version) = read_map_settings(project_root)?;
    let api_table = registry
        .as_ref()
        .and_then(|r| r.get(api_version.to_string()))
        .and_then(|x| x.as_object());
    let mut libs = Vec::new();
    for (name, sub) in obj {
        let subpath = sub.as_str().ok_or_else(|| anyhow!("libs.json[{name}] 非字符串"))?;
        // ① 注册表（官方权威）
        let reg_ver = api_table
            .and_then(|t| t.get(name))
            .and_then(|x| x.as_u64())
            .map(|n| n.to_string());
        // ② 回退：载荷目录最大数字版本
        let ver = reg_ver.or_else(|| {
            let lib_dir = maps_root.join(subpath);
            std::fs::read_dir(&lib_dir)
                .ok()
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|e| e.file_name().to_string_lossy().parse::<u32>().ok())
                .max()
                .map(|n| n.to_string())
        });
        match ver {
            Some(n) => libs.push((name.clone(), n)),
            None => log_warn(&format!("依赖库 {name}（{subpath}）版本无法确定，跳过")),
        }
    }
    // 固定附加三个本地库（版本 -1 = 本地未发布库，随地图上传；抓包实证）
    for name in ["server_lua_plus", "server_common", "global_default"] {
        libs.push((name.to_string(), "-1".to_string()));
    }
    Ok(libs)
}

fn log_warn(msg: &str) {
    eprintln!("[debug] {msg}");
}

/// 拉起客户端进程：bInheritHandles=FALSE（关键——防止父进程管道句柄被游戏进程继承，
/// 否则调用方（python subprocess/终端工具）会因管道 EOF 不到而一直阻塞到游戏窗口关闭）。
/// 返回 pid。
#[cfg(windows)]
fn spawn_detached(exe: &Path, args: &[String], cwd: &Path) -> Result<u32> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::{CloseHandle, GetLastError};
    use windows_sys::Win32::System::Threading::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};

    fn wide(s: &std::ffi::OsStr) -> Vec<u16> {
        s.encode_wide().chain(std::iter::once(0)).collect()
    }
    // 命令行：exe + 引号包裹的参数（含空格/引号的参数加引号）
    fn quote_arg(a: &str) -> String {
        if a.is_empty() || a.chars().any(|c| c == ' ' || c == '\t' || c == '"') {
            format!("\"{}\"", a.replace('"', "\\\""))
        } else {
            a.to_string()
        }
    }
    let mut cmdline = format!("\"{}\"", exe.display());
    for a in args {
        cmdline.push(' ');
        cmdline.push_str(&quote_arg(a));
    }
    let mut cmd_w: Vec<u16> = cmdline.encode_utf16().chain(std::iter::once(0)).collect();
    let cwd_w = wide(cwd.as_os_str());

    unsafe {
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        let ok = CreateProcessW(
            std::ptr::null(),
            cmd_w.as_mut_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            0, // bInheritHandles = FALSE
            0,
            std::ptr::null(),
            cwd_w.as_ptr(),
            &si,
            &mut pi,
        );
        if ok == 0 {
            return Err(anyhow!("拉起 scegame 失败: CreateProcessW err={}", GetLastError()));
        }
        let _ = CloseHandle(pi.hThread);
        let _ = CloseHandle(pi.hProcess);
        Ok(pi.dwProcessId)
    }
}

#[cfg(not(windows))]
fn spawn_detached(exe: &Path, args: &[String], cwd: &Path) -> Result<u32> {
    let child = Command::new(exe)
        .current_dir(cwd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(child.id())
}

/// 公开包装：供 login_state 等复用（拉起脱机 scegame）
pub fn spawn_detached_pub(exe: &Path, args: &[String], cwd: &Path) -> Result<u32> {
    spawn_detached(exe, args, cwd)
}

/// 客户端命令行参数（B 模式契约；多开与「本地服务器」标签页复用）
#[allow(clippy::too_many_arguments)]
pub fn build_client_args(
    host_ip: &str,
    host_port: u16,
    userid: i64,
    staging_dir: &Path,
    dir_name: &str,
    api_version: u32,
    env_domain: &str,
) -> Vec<String> {
    vec![
        "-env=game".to_string(),
        "-editor_server_debug".to_string(),
        format!("-editor_api_version={api_version}"),
        "-no_update".to_string(),
        "-save_replay".to_string(),
        "-kcp_stream".to_string(),
        "-map_kind=0".to_string(),
        format!("-server={env_domain}"),
        "-use_local_res".to_string(),
        format!("-host_ip={host_ip}"),
        format!("-host_port={host_port}"),
        "-local_test".to_string(),
        format!("-user={userid}"),
        format!("-map_path={}", staging_dir.display()),
        format!("-to_download_list={dir_name}"),
        "-width=1600".to_string(),
        "-height=900".to_string(),
    ]
}

impl DebugSession {
    /// 完整 B 模式启动：assign_host → 控制连接上传 → EditorStartGame → spawn 客户端
    pub fn start(params: &DebugParams) -> Result<Self> {
        let (project_name, api_version) = read_map_settings(&params.project_root)?;
        log_warn(&format!("项目 {project_name}（api_version={api_version}）"));

        // ⓪ 暂存目录：未指定则自动生成（白名单复制 + 入口包装）
        let staging_dir = match &params.staging_dir {
            Some(p) => p.clone(),
            None => {
                let p = crate::core::staging::create(
                    &params.project_root,
                    &params.runtime_dir,
                    &project_name,
                )?;
                log_warn(&format!("暂存目录已生成: {}", p.display()));
                p
            }
        };

        // ① host：云端直连 = assign_host；本地自建 = 起中继 host（线程常驻）后指 127.0.0.1
        let host = match params.host_mode {
            HostMode::Cloud => {
                log_warn("assign_host...");
                let host = host::assign_host(&params.cred, &params.env_domain, api_version)?;
                log_warn(&format!("host: {}:{} token={}", host.ip, host.port, host.token));
                host
            }
            HostMode::Relay => {
                log_warn("启动本地自建 host（中继模式）...");
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let port = crate::core::local_host::ensure_running(
                    crate::core::local_host::LocalHostParams {
                        port: 5003,
                        cred: params.cred.clone(),
                        env_domain: params.env_domain.clone(),
                        api_version,
                        capture_path: Some(
                            params
                                .runtime_dir
                                .join("User")
                                .join(format!("host_capture-{ts}.jsonl")),
                        ),
                    },
                )?;
                log_warn(&format!("本地 host 已就绪: 127.0.0.1:{port}"));
                host::HostInfo {
                    ip: "127.0.0.1".to_string(),
                    port,
                    token: "local".to_string(),
                }
            }
            HostMode::Local => {
                log_warn("启动本地壳 host（真本地，无云端）...");
                let port = crate::core::game_host::ensure_running(crate::core::game_host::GameHostParams {
                    port: 5003,
                    runtime_dir: params.runtime_dir.clone(),
                    env_domain: params.env_domain.clone(),
                })?;
                log_warn(&format!("壳 host 已就绪: 127.0.0.1:{port}"));
                host::HostInfo {
                    ip: "127.0.0.1".to_string(),
                    port,
                    token: "shell".to_string(),
                }
            }
        };

        // ①.5 凭证注入：写 runtime/User/user_info-<env>.json（客户端 account 模块启动时读取）
        let user_info_path = params
            .runtime_dir
            .join("User")
            .join(format!("user_info-{}.json", params.env_domain));
        crate::core::auth::write_user_info(&user_info_path, &params.cred)?;
        log_warn(&format!("凭证已注入: {}", user_info_path.display()));

        // ② 控制连接 + 上传
        log_warn(&format!("控制连接 EditorLogin（userid={}）...", params.userid));
        let mut ctl = HostControl::connect(&host, params.userid)?;
        log_warn("EditorLogin 成功，上传项目...");
        let count = ctl.upload_project(&staging_dir, &project_name)?;
        log_warn(&format!("上传完成（{count} 个文件），EditorStartGame..."));

        // ③ EditorStartGame
        let libs = resolve_libs(&params.project_root, &params.runtime_dir, &params.env_domain)
            .unwrap_or_else(|e| {
                log_warn(&format!("依赖库解析失败（继续不带依赖表）: {e}"));
                Vec::new()
            });
        ctl.start_game(&project_name, api_version, &libs)?;
        let session_id = ctl.wait_start_game_res(Duration::from_secs(120))?;
        log_warn(&format!("远端局已起: session_id={session_id}"));

        // ④ spawn 客户端（实捕官方命令行契约，CreateProcess 数组传参无 shell 拆参问题）
        // 引擎目标按运行时种类：编辑器=version-<api>/SCE（sceengine.dll 壳）；对战平台=scegame.exe
        let kind = params
            .runtime_kind
            .unwrap_or(crate::core::runtimes::RuntimeKind::EditorApi(api_version));
        let exe = kind.client_exe(&params.runtime_dir);
        if !exe.is_file() {
            return Err(anyhow!(
                "游戏客户端不存在: {}（运行时 {} 未就绪，先 payload sync）",
                crate::core::disp(&exe),
                kind.display_name()
            ));
        }
        let dir_name = params
            .project_root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| project_name.clone());
        let client_args = build_client_args(
            &host.ip,
            host.port,
            params.userid,
            &staging_dir,
            &dir_name,
            api_version,
            &params.env_domain,
        );
        let child_pid = spawn_detached(&exe, &client_args, &params.runtime_dir)?;
        log_warn(&format!("客户端已拉起 pid={child_pid}（引擎 {}）", kind.display_name()));

        // ④.5 附加客户端（多开）：凭证注入是单文件互斥（user_info-<env>.json），
        // 串行「写凭证 → 拉起 → 等其读取」逐个来；间隔 6s 实测够客户端启动期读完
        let mut extra_pids = Vec::new();
        for (cred2, userid2) in &params.extra_clients {
            crate::core::auth::write_user_info(&user_info_path, cred2)?;
            let mut args2 = client_args.clone();
            for a in args2.iter_mut() {
                if a.starts_with("-user=") {
                    *a = format!("-user={userid2}");
                }
            }
            let pid2 = spawn_detached(&exe, &args2, &params.runtime_dir)?;
            log_warn(&format!("附加客户端已拉起 pid={pid2}（userid={userid2}）"));
            extra_pids.push(pid2);
            std::thread::sleep(Duration::from_secs(6));
        }
        // 恢复主凭证注入（后续重连/下局仍用主号）
        if !params.extra_clients.is_empty() {
            crate::core::auth::write_user_info(&user_info_path, &params.cred)?;
        }

        // pidfile：<staging>.pid（debug stop 用；多开 = 每行一个 pid）
        let pid_file = staging_dir.with_extension("pid");
        let mut pids_text = child_pid.to_string();
        for p in &extra_pids {
            pids_text.push_str(&format!("\n{p}"));
        }
        let _ = std::fs::write(&pid_file, pids_text);

        Ok(Self {
            pid: child_pid,
            extra_pids,
            started: Instant::now(),
            runtime_dir: params.runtime_dir.clone(),
            session_id,
            ctl: Some(ctl),
        })
    }

    /// 按 pidfile 停止（CLI debug stop：宿主已退出时用）
    pub fn stop_by_pidfile(staging_dir: &Path) -> Result<()> {
        let pid_file = staging_dir.with_extension("pid");
        let text = std::fs::read_to_string(&pid_file)
            .map_err(|e| anyhow!("读 pidfile 失败 {}: {e}", pid_file.display()))?;
        for line in text.lines() {
            let pid = line.trim();
            if pid.is_empty() {
                continue;
            }
            let _ = crate::core::silent_command("taskkill")
                .args(["/PID", pid, "/T", "/F"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        let _ = std::fs::remove_file(&pid_file);
        Ok(())
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn poll(&mut self) -> DebugStatus {
        // 顺手抽干 host 日志
        if let Some(ctl) = &mut self.ctl {
            ctl.drain();
        }
        match process_exit_code(self.pid) {
            Some(code) => DebugStatus::Exited(code),
            None => DebugStatus::Running,
        }
    }

    /// 停止：杀客户端进程树（含多开附加客户端）+ 断控制连接
    pub fn stop(&mut self) {
        let mut pids: Vec<String> = vec![self.pid.to_string()];
        pids.extend(self.extra_pids.iter().map(|p| p.to_string()));
        for pid in pids {
            let _ = crate::core::silent_command("taskkill")
                .args(["/PID", &pid, "/T", "/F"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        if let Some(ctl) = &self.ctl {
            ctl.shutdown();
        }
        self.ctl = None;
    }
}

/// 查询进程退出码（None = 仍在运行）
#[cfg(windows)]
fn process_exit_code(pid: u32) -> Option<i32> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    const STILL_ACTIVE: u32 = 259;
    unsafe {
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h.is_null() {
            return Some(-1); // 进程句柄打不开 = 已退出（或无权限，按退出处理）
        }
        let mut code = 0u32;
        let ok = GetExitCodeProcess(h, &mut code);
        let _ = CloseHandle(h);
        if ok == 0 || code != STILL_ACTIVE {
            Some(code as i32)
        } else {
            None
        }
    }
}

#[cfg(not(windows))]
fn process_exit_code(pid: u32) -> Option<i32> {
    let alive = Path::new(&format!("/proc/{pid}")).exists();
    if alive {
        None
    } else {
        Some(-1)
    }
}
