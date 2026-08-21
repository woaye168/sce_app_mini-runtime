//! 脱机运行时（sce_app_mini-runtime）：基于 bgd_appsdk 的标准应用骨架
//!
//! 入口：CLI 子命令（auth/debug）→ 否则 GUI（bgd_appsdk::app::run 全托管公共逻辑）。

// Windows 下不弹出黑色控制台窗口
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod core;
mod ui;

use std::path::PathBuf;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = "脱机运行时";

/// 命中 CLI 时附加到父进程控制台，使 println 输出到调用方终端（同 bgd_sce_tools）
#[cfg(windows)]
fn attach_parent_console() {
    unsafe {
        // ATTACH_PARENT_PROCESS = -1 (0xFFFFFFFF)
        windows_sys::Win32::System::Console::AttachConsole(0xFFFFFFFF);
    }
}

#[cfg(not(windows))]
fn attach_parent_console() {}

fn main() -> eframe::Result<()> {
    // CLI 子命令（命中即以控制台模式执行，不进 GUI）
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "auth" => {
                attach_parent_console();
                cli_auth(&args[2..]);
                return Ok(());
            }
            "debug" => {
                attach_parent_console();
                cli_debug(&args[2..]);
                return Ok(());
            }
            "capture" => {
                attach_parent_console();
                cli_capture(&args[2..]);
                return Ok(());
            }
            "payload" => {
                attach_parent_console();
                cli_payload(&args[2..]);
                return Ok(());
            }
            _ => {}
        }
    }

    bgd_appsdk::app::run(
        bgd_appsdk::app::AppOptions {
            app_name: APP_NAME,
            inner_size: [760.0, 600.0],
            min_size: [640.0, 480.0],
            si_prefix: None,
            is_valid_project: Some(|p| p.join(".bgd").is_dir() || p.join("script").join("tsconfig.json").is_file()),
            app: App::default(),
        },
        APP_VERSION,
    )
}

fn cli_auth(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("list") => {
            let store = core::auth::CredentialStore::load();
            if store.items.is_empty() {
                println!("凭证库为空");
            }
            for (label, cred) in &store.items {
                let active = store.active_label.as_deref() == Some(label.as_str());
                println!(
                    "{}{}  env={}  token_type={}({})  sign={}",
                    if active { "* " } else { "  " },
                    label,
                    cred.env_domain,
                    cred.info.token_type,
                    cred.info.token_type_name(),
                    cred.info.can_sign()
                );
            }
        }
        Some("verify") => {
            let label = args.get(1).map(|s| s.as_str());
            let store = core::auth::CredentialStore::load();
            let Some(label) = label.or(store.active_label.as_deref()) else {
                eprintln!("用法: auth verify [凭证名]（缺省取当前激活）");
                return;
            };
            let Some(cred) = store.items.get(label) else {
                eprintln!("凭证不存在: {label}");
                return;
            };
            match core::verify::verify(&cred.info, &cred.env_domain) {
                Ok(text) => println!("验证通过: {}", &text[..text.len().min(200)]),
                Err(e) => eprintln!("验证失败: {e}"),
            }
        }
        Some("import") => {
            // auth import <凭证名> <项目路径>——读编辑器当前凭证复制进库（不写回，不踢编辑器）
            let (Some(label), Some(project)) = (args.get(1), args.get(2)) else {
                eprintln!("用法: auth import <凭证名> <项目路径>");
                return;
            };
            let locate = match core::locate::locate(std::path::Path::new(project)) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("编辑器定位失败: {e}");
                    return;
                }
            };
            let path = locate.user_info_file();
            match core::auth::read_user_info(&path) {
                Ok(info) => {
                    let mut store = core::auth::CredentialStore::load();
                    store.harvest(label, &locate.env_domain, info);
                    match store.save() {
                        Ok(_) => println!("已导入凭证: {label}（编辑器原凭证未动）"),
                        Err(e) => eprintln!("保存失败: {e}"),
                    }
                }
                Err(e) => eprintln!("读取编辑器凭证失败: {e}"),
            }
        }
        Some("login") => {
            // auth login <项目路径>——扫码自登录：二维码落盘给路径，后台轮询到成功写凭证文件
            let Some(project) = args.get(1) else {
                eprintln!("用法: auth login <项目路径>");
                return;
            };
            let locate = match core::locate::locate(std::path::Path::new(project)) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("编辑器定位失败: {e}");
                    return;
                }
            };
            match core::login::request_device_code(&locate.env_domain) {
                Ok(grant) => {
                    match core::login::qrcode_png(&grant.qrcode_url, 300) {
                        Ok(png) => {
                            let qr = std::env::current_exe()
                                .map(|e| e.with_file_name("login_qrcode.png"))
                                .unwrap_or_else(|_| "login_qrcode.png".into());
                            let _ = std::fs::write(&qr, &png);
                            println!("二维码已保存: {}", qr.display());
                        }
                        Err(e) => eprintln!("二维码生成失败: {e}"),
                    }
                    println!("请用 TapTap 手机客户端扫码（{} 秒内有效）...", grant.expires_in);
                    let state = core::login::poll_device_token(&locate.env_domain, &grant, |st| {
                        match st {
                            core::login::LoginState::WaitingConfirm => println!("已扫码，请在手机上确认..."),
                            _ => {}
                        }
                    });
                    match state {
                        core::login::LoginState::Done(info) => {
                            let path = locate.user_info_file();
                            match core::auth::write_user_info(&path, &info) {
                                Ok(_) => println!("登录成功，凭证已落盘: {}", path.display()),
                                Err(e) => eprintln!("凭证落盘失败: {e}"),
                            }
                        }
                        other => eprintln!("登录未完成: {other:?}"),
                    }
                }
                Err(e) => eprintln!("申请 device_code 失败: {e}"),
            }
        }
        _ => {
            eprintln!("auth 子命令: list | verify [凭证名] | import <凭证名> <项目路径> | login <项目路径>");
        }
    }
}

fn cli_debug(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("start") => {
            // debug start --project <路径> --staging <暂存目录> --user <userid> [--runtime <载荷目录>] [--env <域>] [--cred <凭证名>]
            let mut project = None;
            let mut staging = None;
            let mut user = None;
            let mut runtime = None;
            let mut env_domain = "editor-pd.spark.xd.com".to_string();
            let mut cred_label = None;
            let mut hold_secs = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    "--staging" => { staging = args.get(i + 1).cloned(); i += 2; }
                    "--user" => { user = args.get(i + 1).cloned(); i += 2; }
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    "--env" => { env_domain = args.get(i + 1).cloned().unwrap_or(env_domain); i += 2; }
                    "--cred" => { cred_label = args.get(i + 1).cloned(); i += 2; }
                    "--hold" => { hold_secs = args.get(i + 1).cloned(); i += 2; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let (Some(project), Some(user)) = (project, user) else {
                eprintln!("用法: debug start --project <路径> --user <userid> [--staging <暂存目录>] [--runtime <载荷目录>] [--env <域>] [--cred <凭证名>]");
                return;
            };
            let userid: i64 = match user.parse() {
                Ok(v) => v,
                Err(_) => { eprintln!("--user 必须是数字 userid"); return; }
            };
            let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                std::env::current_exe()
                    .map(|e| e.with_file_name("runtime"))
                    .unwrap_or_else(|_| PathBuf::from("runtime"))
            });
            let store = core::auth::CredentialStore::load();
            let label = cred_label.or(store.active_label.clone());
            let Some(label) = label else {
                eprintln!("无可用凭证（先用 auth import/login）");
                return;
            };
            let Some(cred) = store.items.get(&label) else {
                eprintln!("凭证不存在: {label}");
                return;
            };
            let params = core::debug::DebugParams {
                project_root: PathBuf::from(&project),
                runtime_dir,
                staging_dir: staging.map(PathBuf::from),
                cred: cred.info.clone(),
                userid,
                env_domain,
            };
            match core::debug::DebugSession::start(&params) {
                Ok(mut session) => {
                    println!("调试局已启动: session_id={} 客户端pid={}", session.session_id, session.pid());
                    // --hold <秒>：保持控制连接，持续 dump host 服务端日志（诊断用）
                    if let Some(hold) = &hold_secs {
                        let secs: u64 = hold.parse().unwrap_or(0);
                        println!("保持控制连接 {secs}s，收取 host 日志...");
                        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
                        let mut shown = 0usize;
                        while std::time::Instant::now() < deadline {
                            session.poll();
                            if let Some(ctl) = &session.ctl {
                                while shown < ctl.host_logs.len() {
                                    println!("[host] {}", ctl.host_logs[shown]);
                                    shown += 1;
                                }
                            }
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        }
                    }
                    println!("（客户端独立运行，关闭其窗口即结束；debug stop 可远程停止）");
                }
                Err(e) => eprintln!("启动失败: {e}"),
            }
        }
        Some("stop") => {
            // debug stop [--staging <暂存目录>] [--runtime <载荷目录>] [--project <路径>]
            let mut staging = None;
            let mut runtime = None;
            let mut project = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--staging" => { staging = args.get(i + 1).cloned(); i += 2; }
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let staging_dir = match staging {
                Some(s) => PathBuf::from(s),
                None => {
                    let Some(project) = project else {
                        eprintln!("用法: debug stop [--staging <暂存目录>] 或 [--runtime <载荷目录> --project <路径>]");
                        return;
                    };
                    let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                        std::env::current_exe()
                            .map(|e| e.with_file_name("runtime"))
                            .unwrap_or_else(|_| PathBuf::from("runtime"))
                    });
                    let dir_name = PathBuf::from(&project)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    runtime_dir.join("User").join("debug").join(dir_name)
                }
            };
            match core::debug::DebugSession::stop_by_pidfile(&staging_dir) {
                Ok(_) => println!("已停止（taskkill /T /F）"),
                Err(e) => eprintln!("停止失败: {e}"),
            }
        }
        _ => {
            eprintln!("debug 子命令: start --project <路径> --user <userid> [--staging <暂存目录>] [--runtime <载荷目录>] [--env <域>] [--cred <凭证名>] | stop [--staging <暂存目录> | --runtime <载荷目录> --project <路径>]");
        }
    }
}

/// capture [--title <窗口标题子串>] [--out <输出.png>]——截取运行中的游戏窗口（默认「星火对战平台」）
fn cli_capture(args: &[String]) {
    let mut title = "星火对战平台".to_string();
    let mut out = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => { title = args.get(i + 1).cloned().unwrap_or(title); i += 2; }
            "--out" => { out = args.get(i + 1).cloned(); i += 2; }
            other => { eprintln!("未知参数: {other}"); return; }
        }
    }
    let out_path = out.map(PathBuf::from).unwrap_or_else(|| {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        std::env::current_exe()
            .map(|e| e.with_file_name(format!("capture_{ts}.png")))
            .unwrap_or_else(|_| PathBuf::from(format!("capture_{ts}.png")))
    });
    match core::capture::capture_by_title(&title, &out_path) {
        Ok((w, h)) => println!("已截取 {w}x{h} -> {}", out_path.display()),
        Err(e) => eprintln!("截取失败: {e}"),
    }
}

/// payload sync [--project <路径>] [--runtime <载荷目录>] [--api <版本>] [--dry-run]
/// 从官方 update-info+OSS 通道自举下载运行时载荷（版本跟随服务器）
fn cli_payload(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("sync") => {
            let mut project = None;
            let mut runtime = None;
            let mut api = 13u32;
            let mut dry_run = false;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    "--api" => { api = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(api); i += 2; }
                    "--dry-run" => { dry_run = true; i += 1; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                std::env::current_exe()
                    .map(|e| e.with_file_name("runtime"))
                    .unwrap_or_else(|_| PathBuf::from("runtime"))
            });
            // 项目依赖库（libs.json 键）
            let mut project_libs = Vec::new();
            if let Some(p) = &project {
                if let Ok(content) = std::fs::read_to_string(PathBuf::from(p).join("libs.json")) {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(obj) = v.as_object() {
                            project_libs = obj.keys().cloned().collect();
                        }
                    }
                }
            }
            let params = core::payload::SyncParams {
                runtime_dir,
                env_domain: "editor-pd.spark.xd.com".to_string(),
                api_version: api,
                project_libs,
                project_root: project.map(PathBuf::from),
                dry_run,
            };
            let mut log = |msg: String| println!("{msg}");
            match core::payload::sync(&params, &mut log) {
                Ok(_) => println!("载荷同步完成"),
                Err(e) => eprintln!("载荷同步失败: {e}"),
            }
        }
        _ => eprintln!("payload 子命令: sync [--project <路径>] [--runtime <载荷目录>] [--api <版本>] [--dry-run]"),
    }
}

/// 应用状态
struct App {
    project_root: Option<PathBuf>,
    status: String,
    // 编辑器定位（项目驱动）
    locate: Option<core::locate::EditorLocate>,
    locate_err: String,
    // 凭证
    cred_store: core::auth::CredentialStore,
    cred_new_label: String,
    verify_result: String,
    // 自登录
    login_qr: Option<egui::TextureHandle>,
    login_state: Option<core::login::LoginState>,
    login_grant: Option<core::login::DeviceGrant>,
    login_rx: Option<std::sync::mpsc::Receiver<core::login::LoginState>>,
    // 调试
    debug_session: Option<core::debug::DebugSession>,
    debug_status: Option<core::debug::DebugStatus>,
    debug_staging_input: String,
    debug_userid_input: String,
    debug_runtime_input: String,
    debug_start_rx: Option<std::sync::mpsc::Receiver<ui::debug::StartOutcome>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project_root: None,
            status: String::new(),
            locate: None,
            locate_err: String::new(),
            cred_store: core::auth::CredentialStore::load(),
            cred_new_label: String::new(),
            verify_result: String::new(),
            login_qr: None,
            login_state: None,
            login_grant: None,
            login_rx: None,
            debug_session: None,
            debug_status: None,
            debug_staging_input: String::new(),
            debug_userid_input: String::new(),
            debug_runtime_input: String::new(),
            debug_start_rx: None,
        }
    }
}

impl App {
    /// 项目变化时重定位编辑器
    fn relocate(&mut self) {
        self.locate = None;
        self.locate_err.clear();
        if let Some(p) = &self.project_root {
            match core::locate::locate(p) {
                Ok(l) => self.locate = Some(l),
                Err(e) => self.locate_err = e.to_string(),
            }
        }
    }
}

const TABS: &[bgd_appsdk::ui::ShellTab] = &[
    bgd_appsdk::ui::ShellTab { id: "auth", label: "凭证" },
    bgd_appsdk::ui::ShellTab { id: "debug", label: "调试" },
    bgd_appsdk::ui::ShellTab { id: "settings", label: "设置" },
];

impl bgd_appsdk::ui::ShellApp for App {
    fn app_title(&self) -> &'static str {
        APP_NAME
    }

    fn tabs(&self) -> &[bgd_appsdk::ui::ShellTab] {
        TABS
    }

    fn ui_tab(&mut self, ui: &mut egui::Ui, tab: &str) {
        match tab {
            "auth" => self.ui_auth(ui),
            "debug" => self.ui_debug(ui),
            "settings" => self.ui_settings(ui),
            _ => {}
        }
    }

    fn on_project_changed(&mut self, project: Option<&std::path::Path>) {
        self.project_root = project.map(|p| p.to_path_buf());
        if let Some(p) = project {
            self.status = format!("当前项目: {}", p.display());
        }
        self.relocate();
    }

    fn status_text(&self) -> String {
        self.status.clone()
    }
}
