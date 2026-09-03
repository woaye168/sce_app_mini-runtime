//! 本地服务器游玩编排（0.5.0 R5「本地服务器」标签页）：
//! 按本地账号拉起客户端连真本地 host（127.0.0.1:5003）。
//! 局未起时自动走控制面（上传 + EditorStartGame）；控制连接全局持有（断开即失去 0xF00C 日志通道）。

use crate::core::local_accounts::LocalAccount;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct LocalPlayParams {
    pub project_root: PathBuf,
    pub runtime_dir: PathBuf,
    pub env_domain: String,
    pub account: LocalAccount,
    /// host 绑定地址（127.0.0.1 本地 / 0.0.0.0 局域网+外网），UI 绑定范围选择传入
    pub bind_addr: String,
}

/// 局控制连接（全局持有；上传/起局各一次）
static GAME_CTL: Mutex<Option<crate::core::host::HostControl>> = Mutex::new(None);

/// 局是否已起（控制面状态）
pub fn game_active() -> Option<String> {
    let state = crate::core::game_host::control_state()?;
    let g = state.lock().unwrap();
    g.game.as_ref().map(|i| format!("{}（session={}）", i.project, i.session_id))
}

/// 停局（清控制连接 + 通知 host teardown）
pub fn stop_game() {
    let ctl = GAME_CTL.lock().unwrap().take();
    if let Some(ctl) = ctl {
        ctl.shutdown();
    }
    // 通知 host 停局（0xF01B 等价物走控制连接 shutdown；host 侧连接断开即释放）
}

/// 按账号拉起客户端：ensure host → 局未起则上传起局 → 合成凭证注入 → spawn。返回 pid。
pub fn launch(params: &LocalPlayParams, log: &mut dyn FnMut(String)) -> Result<u32> {
    let (project_name, api_version) = crate::core::debug::read_map_settings(&params.project_root)?;
    // ① host 幂等
    let port = crate::core::game_host::ensure_running(crate::core::game_host::GameHostParams {
        port: 5003,
        runtime_dir: params.runtime_dir.clone(),
        env_domain: params.env_domain.clone(),
        bind_addr: params.bind_addr.clone(),
    })?;
    log(format!("本地 host 已就绪: 127.0.0.1:{port}"));

    let dir_name = params
        .project_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| project_name.clone());
    // staging 只在局未起时生成：局运行中客户端正占用 staging 内文件（os error 32 文件锁），
    // 后续账号直接复用本局已上传的 staging 目录
    let staging_dir = params
        .runtime_dir
        .join("User")
        .join("debug")
        .join(&dir_name);

    // ② 局：未起则生成 staging + 控制面上传 + 起局；
    // 已起且 staging 就绪（入口包装已生成）则跳过上传直接拉起——秒进
    if game_active().is_none() {
        let t0 = std::time::Instant::now();
        let staging_dir = crate::core::staging::create(&params.project_root, &params.runtime_dir, &project_name)?;
        log(format!("staging 生成完成（{:.1?}），控制连接上传项目...", t0.elapsed()));
        let t1 = std::time::Instant::now();
        // 本地 host 同机：直读目标端做内容级增量（变化的文件才走 TCP 上传）
        let upload_dir = params.runtime_dir.join("User").join("host_upload").join(&project_name);
        let mut ctl = crate::core::host::HostControl::connect(
            &crate::core::host::HostInfo {
                ip: "127.0.0.1".into(),
                port,
                token: "local".into(),
            },
            params.account.userid,
        )?;
        let count = ctl.upload_project_incremental(&staging_dir, &project_name, &upload_dir)?;
        log(format!("上传完成（{count} 个文件，{:.1?}），EditorStartGame...", t1.elapsed()));
        let libs = crate::core::debug::resolve_libs(
            &params.project_root,
            &params.runtime_dir,
            &params.env_domain,
        )
        .unwrap_or_default();
        ctl.start_game(&project_name, api_version, &libs)?;
        let session_id = ctl.wait_start_game_res(std::time::Duration::from_secs(120))?;
        log(format!("局已起: session_id={session_id}"));
        *GAME_CTL.lock().unwrap() = Some(ctl);
    } else if !staging_dir.join("ui").join("script").join("main.lua").is_file() {
        // 局在跑但 staging 缺入口（旧产物不完整，如 0.5.0 之前生成）：补一次 staging。
        // staging::create 本身是增量的（内容一致跳过），天然只补缺不重建，
        // 不会因重写未变化文件去撞运行中客户端的文件锁
        crate::core::staging::create(&params.project_root, &params.runtime_dir, &project_name)?;
    }

    // ③ 合成凭证注入（单文件互斥：调用方负责间隔；客户端可能仍持读句柄， sharing 冲突重试）
    let user_info_path = params
        .runtime_dir
        .join("User")
        .join(format!("user_info-{}.json", params.env_domain));
    let info = crate::core::local_accounts::synth_user_info(&params.account);
    let mut last_err = None;
    for _ in 0..20 {
        match crate::core::auth::write_user_info(&user_info_path, &info) {
            Ok(_) => {
                last_err = None;
                break;
            }
            Err(e) => {
                last_err = Some(e);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
    if let Some(e) = last_err {
        return Err(e);
    }

    // ④ spawn 客户端
    let kind = crate::core::runtimes::RuntimeKind::EditorApi(api_version);
    let exe = kind.client_exe(&params.runtime_dir);
    let args = crate::core::debug::build_client_args(
        "127.0.0.1",
        port,
        params.account.userid,
        &staging_dir,
        &dir_name,
        api_version,
        &params.env_domain,
    );
    let pid = crate::core::debug::spawn_detached_pub(&exe, &args, &params.runtime_dir)?;
    log(format!(
        "客户端已拉起 pid={pid}（账号 {} / userid={}）",
        params.account.name, params.account.userid
    ));
    Ok(pid)
}
