//! 脱机运行时（sce_app_mini-runtime）：基于 bgd_appsdk 的标准应用骨架
//!
//! 入口：CLI 子命令（auth/debug）→ 否则 GUI（bgd_appsdk::app::run 全托管公共逻辑）。

// Windows 下不弹出黑色控制台窗口
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use sce_app_mini_runtime::core;
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
            "host" => {
                attach_parent_console();
                cli_host(&args[2..]);
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
            "local" => {
                attach_parent_console();
                cli_local(&args[2..]);
                return Ok(());
            }
            "staging" => {
                attach_parent_console();
                cli_staging(&args[2..]);
                return Ok(());
            }
            "selftest" => {
                attach_parent_console();
                cli_selftest();
                return Ok(());
            }
            "locate" => {
                attach_parent_console();
                cli_locate(&args[2..]);
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
                Ok(text) => println!("验证通过: {}", text.chars().take(200).collect::<String>()),
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
                    // 重名自动追加序号（与 UI 导入行为一致，避免静默覆盖已有凭证）
                    let base = label.trim();
                    let mut final_label = base.to_string();
                    let mut n = 2;
                    while store.items.contains_key(&final_label) {
                        final_label = format!("{base}-{n}");
                        n += 1;
                    }
                    store.harvest(&final_label, &locate.env_domain, info);
                    match store.save() {
                        Ok(_) => println!("已导入凭证: {final_label}（编辑器原凭证未动）"),
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
                    }, None);
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
        Some("refresh") => {
            // auth refresh <凭证名> [--runtime <载荷目录>] [--timeout <秒>]——起脱机客户端真实登录，抓 userid/昵称写回凭证库
            let Some(label) = args.get(1) else {
                eprintln!("用法: auth refresh <凭证名> [--runtime <载荷目录>] [--timeout <秒>]");
                return;
            };
            let mut runtime = None;
            let mut timeout = 60u64;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    "--timeout" => { timeout = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(timeout); i += 2; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                std::env::current_exe()
                    .map(|e| e.with_file_name("runtime"))
                    .unwrap_or_else(|_| PathBuf::from("runtime"))
            });
            let mut store = core::auth::CredentialStore::load();
            let Some(cred) = store.items.get(label) else {
                eprintln!("凭证不存在: {label}");
                return;
            };
            println!("起脱机客户端登录中（最长 {timeout}s）...");
            match core::login_state::fetch_identity(
                &runtime_dir,
                &cred.info,
                &cred.env_domain,
                std::time::Duration::from_secs(timeout),
            ) {
                Ok(id) => {
                    let uid = id.userid_i64().unwrap_or(0);
                    let name = id.user_name_opt();
                    match store.update_identity(label, uid, name.clone()) {
                        Ok(_) => println!("已刷新：userid={uid} 昵称={}", name.unwrap_or_else(|| "（无）".into())),
                        Err(e) => eprintln!("写回凭证库失败: {e}"),
                    }
                }
                Err(e) => eprintln!("登录态获取失败: {e}"),
            }
        }
        _ => {
            eprintln!("auth 子命令: list | verify [凭证名] | import <凭证名> <项目路径> | login <项目路径> | refresh <凭证名> [--runtime <载荷目录>]");
        }
    }
}

/// 本地服务器 CLI（0.5.0 R5）：local account list/create/remove + local play（按本地账号拉起客户端，多开多账号）
fn cli_local(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("account") => match args.get(1).map(|s| s.as_str()) {
            Some("list") => {
                for acc in core::local_accounts::list().unwrap_or_default() {
                    println!("{}  userid={}  {}", acc.name, acc.userid, acc.created_at);
                }
            }
            Some("create") => {
                let Some(name) = args.get(2) else { eprintln!("用法: local account create <名字>"); return; };
                match core::local_accounts::create(name) {
                    Ok(acc) => println!("已创建: {} userid={}", acc.name, acc.userid),
                    Err(e) => eprintln!("创建失败: {e}"),
                }
            }
            Some("remove") => {
                let Some(name) = args.get(2) else { eprintln!("用法: local account remove <名字>"); return; };
                let found = core::local_accounts::list().unwrap_or_default().into_iter().find(|a| &a.name == name);
                match found {
                    Some(acc) => match core::local_accounts::remove(acc.id) {
                        Ok(()) => println!("已删除: {name}"),
                        Err(e) => eprintln!("删除失败: {e}"),
                    },
                    None => eprintln!("账号不存在: {name}"),
                }
            }
            _ => eprintln!("用法: local account list|create <名字>|remove <名字>"),
        },
        Some("play") => {
            // local play --project <路径> --account <名字> [--account <名字2> ...] [--runtime <载荷目录>] [--bind <地址>]
            let mut project = None;
            let mut runtime = None;
            let mut names: Vec<String> = Vec::new();
            let mut bind_addr = "127.0.0.1".to_string();
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    "--account" => { names.push(args.get(i + 1).cloned().unwrap_or_default()); i += 2; }
                    "--bind" => { bind_addr = args.get(i + 1).cloned().unwrap_or(bind_addr); i += 2; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let Some(project) = project else {
                eprintln!("用法: local play --project <路径> --account <名字> [--account <名字2> ...] [--runtime <载荷目录>] [--bind <地址>]");
                return;
            };
            if names.is_empty() {
                eprintln!("至少一个 --account");
                return;
            }
            let accounts = core::local_accounts::list().unwrap_or_default();
            let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                std::env::current_exe().map(|e| e.with_file_name("runtime")).unwrap_or_else(|_| PathBuf::from("runtime"))
            });
            for (idx, name) in names.iter().enumerate() {
                let Some(acc) = accounts.iter().find(|a| &a.name == name) else {
                    eprintln!("账号不存在: {name}（先 local account create）");
                    continue;
                };
                match core::local_play::launch(
                    &core::local_play::LocalPlayParams {
                        project_root: PathBuf::from(&project),
                        runtime_dir: runtime_dir.clone(),
                        env_domain: "editor-pd.spark.xd.com".into(),
                        account: acc.clone(),
                        bind_addr: bind_addr.clone(),
                    },
                    &mut |msg| println!("[play] {msg}"),
                ) {
                    Ok(pid) => println!("账号 {name} 客户端 pid={pid}"),
                    Err(e) => {
                        eprintln!("账号 {name} 启动失败: {e}");
                        return;
                    }
                }
                // 凭证注入互斥（user_info 单文件）：下一个账号等 6s 让上一个读完
                if idx + 1 < names.len() {
                    std::thread::sleep(std::time::Duration::from_secs(6));
                }
            }
            // host 线程活在本进程：常驻承载
            println!("本地 host 常驻中（Ctrl+C 停止，客户端随之中断）...");
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600));
            }
        }
        _ => eprintln!("用法: local account list|create|remove / local play --project <路径> --account <名字>..."),
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
            let mut kind_str: Option<String> = None;
            let mut host_mode = core::debug::HostMode::Cloud;
            let mut extra_clients_n: usize = 0;
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
                    "--kind" => { kind_str = args.get(i + 1).cloned(); i += 2; }
                    "--clients" => {
                        extra_clients_n = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
                        i += 2;
                    }
                    "--host" => {
                        // 0.5.0 语义切换：local = 真本地（旧 0.4.x 的 local=中继改由 relay 承接）；
                        // shell 为 0.5.0 开发期内部值，映射 local 并提示
                        host_mode = match args.get(i + 1).map(|s| s.as_str()) {
                            Some("cloud") => core::debug::HostMode::Cloud,
                            Some("relay") => core::debug::HostMode::Relay,
                            Some("local") => core::debug::HostMode::Local,
                            Some("shell") => {
                                eprintln!("[warn] --host shell 已更名 local（0.5.0 正式三态 cloud|relay|local）");
                                core::debug::HostMode::Local
                            }
                            other => { eprintln!("--host 只支持 cloud|relay|local，收到: {other:?}"); return; }
                        };
                        i += 2;
                    }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let Some(project) = project else {
                eprintln!("用法: debug start --project <路径> [--user <userid>] [--staging <暂存目录>] [--runtime <载荷目录>] [--env <域>] [--cred <凭证名>] [--clients N] [--kind editor-<api>|tester_test|tester_prod] [--host cloud|relay|local]");
                return;
            };
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
            // --user 缺省时取凭证登录态（auth refresh 抓回的 userid）
            let userid: i64 = match user.as_deref().map(str::trim) {
                Some(s) if !s.is_empty() => match s.parse() {
                    Ok(v) => v,
                    Err(_) => { eprintln!("--user 必须是数字 userid"); return; }
                },
                _ => match cred.info.userid {
                    Some(v) => v,
                    None => {
                        eprintln!("未指定 --user 且凭证 {label} 无登录态（先 auth refresh {label}）");
                        return;
                    }
                },
            };
            let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                std::env::current_exe()
                    .map(|e| e.with_file_name("runtime"))
                    .unwrap_or_else(|_| PathBuf::from("runtime"))
            });
            // 附加客户端：按凭证库顺序取（排除主凭证），须有登录态 userid
            let mut extra_clients: Vec<(core::auth::UserInfo, i64)> = Vec::new();
            if extra_clients_n > 0 {
                for (l, c) in &store.items {
                    if *l == label {
                        continue;
                    }
                    let Some(uid2) = c.info.userid else {
                        eprintln!("[warn] 凭证 {l} 无登录态，跳过（auth refresh {l} 后可用于多开）");
                        continue;
                    };
                    extra_clients.push((c.info.clone(), uid2));
                    if extra_clients.len() >= extra_clients_n {
                        break;
                    }
                }
                if extra_clients.len() < extra_clients_n {
                    eprintln!("[warn] 可用附加凭证不足：要 {extra_clients_n} 个，实际 {} 个", extra_clients.len());
                }
            }
            let params = core::debug::DebugParams {
                project_root: PathBuf::from(&project),
                runtime_dir,
                staging_dir: staging.map(PathBuf::from),
                cred: cred.info.clone(),
                userid,
                env_domain,
                runtime_kind: kind_str.map(|s| core::runtimes::parse(&s, 13)),
                host_mode,
                extra_clients,
            };
            match core::debug::DebugSession::start(&params) {
                Ok(mut session) => {
                    println!("调试局已启动: session_id={} 客户端pid={}", session.session_id, session.pid());
                    // 本地自建 host 模式：host 线程活在本进程，CLI 必须常驻（隐含 --hold 永久）
                    let hold_forever = host_mode == core::debug::HostMode::Relay
                        || host_mode == core::debug::HostMode::Local;
                    if hold_forever || hold_secs.is_some() {
                        let secs: u64 = hold_secs.as_deref().and_then(|s| s.parse().ok()).unwrap_or(0);
                        if hold_forever {
                            println!("本地 host 模式：本进程是 host 载体，保持运行（Ctrl+C 停止，客户端随之中断）...");
                        } else {
                            println!("保持控制连接 {secs}s，收取 host 日志...");
                        }
                        let mut shown = 0usize;
                        // 泵消息体：收 session 事件 + 打印新增 host 日志
                        let mut pump = |shown: &mut usize| {
                            session.poll();
                            if let Some(ctl) = &session.ctl {
                                while *shown < ctl.host_logs.len() {
                                    println!("[host] {}", ctl.host_logs[*shown]);
                                    *shown += 1;
                                }
                            }
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        };
                        if hold_forever {
                            // 常驻分支直接永久循环，不用「超大 deadline」表达（Instant+Duration 溢出即 panic，平台相关）
                            loop {
                                pump(&mut shown);
                            }
                        } else {
                            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(secs);
                            while std::time::Instant::now() < deadline {
                                pump(&mut shown);
                            }
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
            eprintln!("debug 子命令: start --project <路径> [--user <userid>] [--staging <暂存目录>] [--runtime <载荷目录>] [--env <域>] [--cred <凭证名>] | stop [--staging <暂存目录> | --runtime <载荷目录> --project <路径>]");
        }
    }
}

/// host start --project <路径> [--port 5003] [--env <域>] [--cred <凭证名>] [--capture <jsonl路径>]
/// 前台跑自建 host（中继模式）：编辑器「调试(本地服务器)」（use_local_host → 127.0.0.1:5003）直接接入
fn cli_host(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("start") => {
            let mut project = None;
            let mut port = 5003u16;
            let mut env_domain = "editor-pd.spark.xd.com".to_string();
            let mut cred_label = None;
            let mut capture = None;
            let mut shell = false;
            let mut bind_addr = "127.0.0.1".to_string();
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    "--port" => { port = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(port); i += 2; }
                    "--env" => { env_domain = args.get(i + 1).cloned().unwrap_or(env_domain); i += 2; }
                    "--cred" => { cred_label = args.get(i + 1).cloned(); i += 2; }
                    "--capture" => { capture = args.get(i + 1).cloned(); i += 2; }
                    "--bind" => { bind_addr = args.get(i + 1).cloned().unwrap_or(bind_addr); i += 2; }
                    "--shell" | "--local" => { shell = true; i += 1; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let Some(project) = project else {
                eprintln!("用法: host start --project <路径> [--port 5003] [--env <域>] [--cred <凭证名>] [--capture <jsonl路径>] [--shell]");
                return;
            };
            // 壳 host（0.5.0 R3 真本地）：不 assign、不联云端、无需凭证
            if shell {
                let runtime_dir = std::env::current_exe()
                    .map(|e| e.with_file_name("runtime"))
                    .unwrap_or_else(|_| PathBuf::from("runtime"));
                println!("项目 {project}，壳 host（真本地，无云端）启动...");
                if let Err(e) = core::game_host::run(
                    core::game_host::GameHostParams { port, runtime_dir, env_domain: env_domain.clone(), bind_addr },
                    None,
                ) {
                    eprintln!("壳 host 退出: {e}");
                }
                return;
            }
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
            let (_, api_version) = match core::debug::read_map_settings(std::path::Path::new(&project)) {
                Ok(v) => v,
                Err(e) => { eprintln!("读项目失败: {e}"); return; }
            };
            let capture_path = capture.map(PathBuf::from).unwrap_or_else(|| {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                std::env::current_exe()
                    .map(|e| e.with_file_name(format!("host_capture-{ts}.jsonl")))
                    .unwrap_or_else(|_| PathBuf::from(format!("host_capture-{ts}.jsonl")))
            });
            println!("项目 {}（api={api_version}）凭证 {label}", project);
            let params = core::local_host::LocalHostParams {
                port,
                cred: cred.info.clone(),
                env_domain,
                api_version,
                capture_path: Some(capture_path),
            };
            if let Err(e) = core::local_host::run(params, None) {
                eprintln!("自建 host 退出: {e}");
            }
        }
        Some("probe") => {
            // host probe [--port 5003] [--bind 127.0.0.1]——真本地 host（game_host）控制面协议探针
            let mut port = 5003u16;
            let mut bind_addr = "127.0.0.1".to_string();
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--port" => { port = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(port); i += 2; }
                    "--bind" => { bind_addr = args.get(i + 1).cloned().unwrap_or(bind_addr); i += 2; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            cli_host_probe(port, &bind_addr);
        }
        _ => {
            eprintln!("host 子命令: start --project <路径> [--port 5003] [--env <域>] [--cred <凭证名>] [--capture <jsonl路径>] | probe [--port 5003] [--bind 127.0.0.1]");
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
            let mut kind_str: Option<String> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    "--api" => { api = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(api); i += 2; }
                    "--dry-run" => { dry_run = true; i += 1; }
                    "--kind" => { kind_str = args.get(i + 1).cloned(); i += 2; }
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
                runtime_kind: kind_str.map(|s| core::runtimes::parse(&s, api)),
            };
            let mut log = |msg: String| println!("{msg}");
            match core::payload::sync(&params, &mut log) {
                Ok(_) => println!("载荷同步完成"),
                Err(e) => eprintln!("载荷同步失败: {e}"),
            }
        }
        _ => eprintln!("payload 子命令: sync [--project <路径>] [--runtime <载荷目录>] [--api <版本>] [--kind editor-<api>|tester_test|tester_prod] [--dry-run]"),
    }
}

/// 递归统计目录下文件数（staging 结果报告用）
fn count_files(dir: &std::path::Path) -> u32 {
    let mut n = 0u32;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                n += count_files(&p);
            } else {
                n += 1;
            }
        }
    }
    n
}

/// staging create --project <路径> [--staging <目录>] [--runtime <载荷目录>]
/// 独立触发 staging 生成（core::staging::create 同款逻辑；警告/增量明细经 logbus 直接打印）
fn cli_staging(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("create") => {
            let mut project = None;
            let mut staging = None;
            let mut runtime = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--project" => { project = args.get(i + 1).cloned(); i += 2; }
                    "--staging" => { staging = args.get(i + 1).cloned(); i += 2; }
                    "--runtime" => { runtime = args.get(i + 1).cloned(); i += 2; }
                    other => { eprintln!("未知参数: {other}"); return; }
                }
            }
            let Some(project) = project else {
                eprintln!("用法: staging create --project <路径> [--staging <目录>] [--runtime <载荷目录>]");
                return;
            };
            let project_root = PathBuf::from(&project);
            // 项目名取 map_settings.json 的 ProjectName（与 debug 编排一致）
            let (project_name, _) = match core::debug::read_map_settings(&project_root) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("读项目失败: {e}");
                    std::process::exit(1);
                }
            };
            let result = match staging {
                // 显式指定目录：直生成（create_at 不含布局推导）
                Some(dir) => core::staging::create_at(&project_root, std::path::Path::new(&dir), &project_name),
                // 缺省推导与 debug 模块一致：<runtime>/User/debug/<项目目录名>（runtime 缺省 = exe 旁 runtime）
                None => {
                    let runtime_dir = runtime.map(PathBuf::from).unwrap_or_else(|| {
                        std::env::current_exe()
                            .map(|e| e.with_file_name("runtime"))
                            .unwrap_or_else(|_| PathBuf::from("runtime"))
                    });
                    core::staging::create(&project_root, &runtime_dir, &project_name)
                }
            };
            match result {
                Ok(dir) => println!("staging 已生成: {}（{} 个文件）", core::disp(&dir), count_files(&dir)),
                Err(e) => {
                    eprintln!("staging 生成失败: {e}");
                    std::process::exit(1);
                }
            }
        }
        _ => eprintln!("staging 子命令: create --project <路径> [--staging <目录>] [--runtime <载荷目录>]"),
    }
}

/// selftest：核心算法自检（zcompress / cmsg_pack / from_lua 环检测）。
/// 全过打印 PASS 明细；任一失败打印 FAIL 并以 exit code 1 退出
fn cli_selftest() {
    let mut failed = 0u32;
    let mut check = |name: &str, r: Result<(), String>| match r {
        Ok(()) => println!("[PASS] {name}"),
        Err(e) => {
            println!("[FAIL] {name}: {e}");
            failed += 1;
        }
    };
    check("zcompress 压缩/解压 roundtrip", selftest_zcompress());
    check("cmsg_pack pack/unpack roundtrip", selftest_cmsg_roundtrip());
    check("cmsg_pack 超深度嵌套拒包（不爆栈）", selftest_cmsg_depth());
    check("cmsg_pack from_lua 循环引用拒绝（不 abort）", selftest_from_lua_cycle());
    if failed > 0 {
        println!("selftest 完成：{failed} 项失败");
        std::process::exit(1);
    }
    println!("selftest 全部通过");
}

/// zcompress：多组样本 压缩→解压 roundtrip 逐字节一致（单解码器逐帧推进，连接级状态机同款用法）
fn selftest_zcompress() -> Result<(), String> {
    use core::zcompress::{decode_frame, encode_frame_raw, ZDecoder};
    // 伪随机字节（xorshift，免引入 rand 依赖）
    let mut seed = 0x1234_5678u32;
    let mut rand_bytes = Vec::with_capacity(4096);
    for _ in 0..4096 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        rand_bytes.push((seed & 0xff) as u8);
    }
    let samples: Vec<(&str, Vec<u8>)> = vec![
        ("空样本", Vec::new()),
        ("短样本", b"hello zcompress".to_vec()),
        ("长重复样本", b"abcabc123".repeat(2000)),
        ("随机字节样本", rand_bytes),
        ("全字节值样本", (0..=255u8).collect::<Vec<u8>>().repeat(4)),
    ];
    let mut dec = ZDecoder::new();
    for (name, sample) in &samples {
        let enc = encode_frame_raw(sample);
        let back = decode_frame(&mut dec, &enc).map_err(|e| format!("{name} 解码失败: {e}"))?;
        if back != *sample {
            return Err(format!("{name} roundtrip 逐字节不一致（{} 字节）", sample.len()));
        }
    }
    Ok(())
}

/// cmsg_pack：pack→unpack roundtrip（整数/负整数/字符串/二进制/数组/嵌套 map 各形态）
fn selftest_cmsg_roundtrip() -> Result<(), String> {
    use core::cmsg_pack::{pack_to_vec, unpack, CVal};
    let cases: Vec<(&str, CVal)> = vec![
        ("nil", CVal::Nil),
        ("bool", CVal::Bool(true)),
        ("零", CVal::Int(0)),
        ("正整数 fixint", CVal::Int(127)),
        ("正整数 u16 宽度", CVal::Int(65536)),
        ("负整数 fixint", CVal::Int(-1)),
        ("负整数 i8 宽度", CVal::Int(-128)),
        ("负整数 i64 宽度", CVal::Int(i64::MIN)),
        ("u64 最大值", CVal::U64(u64::MAX)),
        ("f64", CVal::F64(std::f64::consts::PI)),
        ("字符串（UTF-8）", CVal::Str("你好 cmsg".as_bytes().to_vec())),
        ("二进制（含 0x00）", CVal::Str(vec![0, 1, 0, 255, 0])),
        ("数组", CVal::Array(vec![CVal::Int(1), CVal::Int(2), CVal::Str(b"x".to_vec())])),
        ("嵌套 map", CVal::Map(vec![
            (CVal::Str(b"type".to_vec()), CVal::Str(b"Req_Test".to_vec())),
            (CVal::Str(b"args".to_vec()), CVal::Map(vec![
                (CVal::Str(b"n".to_vec()), CVal::Int(-42)),
                (CVal::Str(b"list".to_vec()), CVal::Array(vec![CVal::Bool(false), CVal::Nil])),
            ])),
        ])),
    ];
    for (name, v) in &cases {
        let bytes = pack_to_vec(v);
        let Some((back, used)) = unpack(&bytes) else {
            return Err(format!("{name} unpack 返回 None"));
        };
        if used != bytes.len() {
            return Err(format!("{name} 消费长度不符: {used}/{}", bytes.len()));
        }
        if back != *v {
            return Err(format!("{name} roundtrip 不一致: {back:?}"));
        }
    }
    Ok(())
}

/// cmsg_pack 深度上限：超深度嵌套样本 unpack 必须拒绝（返回 None）而非栈溢出
fn selftest_cmsg_depth() -> Result<(), String> {
    use core::cmsg_pack::{pack_to_vec, unpack, CVal};
    // 上限内对照组（64 层 < MAX_DEPTH 128）：必须正常 roundtrip
    let mut ok = CVal::Int(1);
    for _ in 0..64 {
        ok = CVal::Array(vec![ok]);
    }
    let bytes = pack_to_vec(&ok);
    if unpack(&bytes).is_none() {
        return Err("64 层嵌套（上限内）被误拒".into());
    }
    // 超深度（200 层 > MAX_DEPTH 128）：必须拒包
    let mut deep = CVal::Int(1);
    for _ in 0..200 {
        deep = CVal::Array(vec![deep]);
    }
    let bytes = pack_to_vec(&deep);
    if unpack(&bytes).is_some() {
        return Err("200 层超深度嵌套未被拒绝".into());
    }
    Ok(())
}

/// from_lua 环检测：mlua 造自引用表（t.self = t），from_lua 必须返回 Err 而非递归爆栈 abort
fn selftest_from_lua_cycle() -> Result<(), String> {
    let lua = mlua::Lua::new();
    let t: mlua::Table = lua
        .load("local t = {} t.self = t return t")
        .eval()
        .map_err(|e| format!("构造自引用表失败: {e}"))?;
    match core::cmsg_pack::from_lua(&mlua::Value::Table(t)) {
        Err(_) => Ok(()),
        Ok(_) => Err("自引用表未被拒绝（环检测失效）".into()),
    }
}

/// locate <项目路径>——编辑器定位推导结果打印（core::locate::locate）
fn cli_locate(args: &[String]) {
    let Some(project) = args.first() else {
        eprintln!("用法: locate <项目路径>");
        return;
    };
    match core::locate::locate(std::path::Path::new(project)) {
        Ok(l) => {
            println!("api_version : {}", l.api_version);
            println!("env_domain  : {}", l.env_domain);
            println!("editor_root : {}", core::disp(&l.editor_root));
            println!("engine_root : {}", core::disp(&l.engine_root));
            println!("version_dir : {}", core::disp(&l.version_dir()));
            println!("user_info   : {}", core::disp(&l.user_info_file()));
            println!("editor_exe  : {}", core::disp(&l.editor_exe()));
        }
        Err(e) => {
            eprintln!("定位失败: {e}");
            std::process::exit(1);
        }
    }
}

// ---------- host probe：真本地 host 控制面协议探针 ----------

/// 探针读一帧（u32 LE 总长 + 帧体，与 host_server::read_frame 同款线格式）；EOF/对端关闭 = Ok(None)
fn probe_read_frame(s: &mut std::net::TcpStream) -> Result<Option<core::host::Frame>, String> {
    use std::io::Read;
    let mut len_buf = [0u8; 4];
    match s.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e)
            if e.kind() == std::io::ErrorKind::UnexpectedEof
                || e.kind() == std::io::ErrorKind::ConnectionReset =>
        {
            return Ok(None)
        }
        // 读超时单列（连接仍存活语义的判定依赖它，不能与 RST 混淆）
        Err(e)
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut =>
        {
            return Err("读超时".into())
        }
        Err(e) => return Err(format!("读帧头失败: {e}")),
    }
    let total = u32::from_le_bytes(len_buf) as usize;
    if !(6..=64 * 1024 * 1024).contains(&total) {
        return Err(format!("帧长度异常: {total}"));
    }
    let mut frame = len_buf.to_vec();
    frame.resize(total, 0);
    match s.read_exact(&mut frame[4..]) {
        Ok(()) => {}
        Err(e)
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut =>
        {
            return Err("读超时".into())
        }
        Err(e) => return Err(format!("读帧体失败: {e}")),
    }
    let parsed = core::host::decode_frame(&frame).map_err(|e| format!("解帧失败: {e}"))?;
    Ok(Some(parsed))
}

/// 等指定类型的帧（跳过 0xF00C 日志推送等其余类型；读超时即失败）
fn probe_wait(s: &mut std::net::TcpStream, want_type: u64) -> Result<core::host::Frame, String> {
    loop {
        match probe_read_frame(s)? {
            Some(f) if f.msg_type == want_type => return Ok(f),
            Some(_) => continue,
            None => return Err("连接被关闭".into()),
        }
    }
}

/// 发一帧（core::host::encode_frame 组帧，与 host_server 解析严格对齐）
fn probe_send(s: &mut std::net::TcpStream, msg_type: u64, body: &[u8]) -> Result<(), String> {
    use std::io::Write;
    let frame = core::host::encode_frame(msg_type, body);
    s.write_all(&frame).map_err(|e| format!("发送失败: {e}"))
}

/// 连接控制口并发合法 0xF000 登录帧 {f1 userid, f2 token}，校验 0xF001 应答 result=0
fn probe_login(s: &mut std::net::TcpStream, userid: u64) -> Result<(), String> {
    let mut body = Vec::new();
    core::host::put_field_varint(&mut body, 1, userid);
    core::host::put_field_bytes(&mut body, 2, b"probe");
    probe_send(s, core::host::MSG_EDITOR_LOGIN, &body)?;
    let f = probe_wait(s, core::host::MSG_EDITOR_LOGIN_RESULT)?;
    let result = core::host::body_varint(&f.body, 1).unwrap_or(u64::MAX);
    if result != 0 {
        return Err(format!("登录应答 result={result}"));
    }
    Ok(())
}

/// 探针 connect（带读写超时，防探针自身悬挂）
fn probe_connect(bind_addr: &str, port: u16) -> Result<std::net::TcpStream, String> {
    let s = std::net::TcpStream::connect(format!("{bind_addr}:{port}"))
        .map_err(|e| format!("TCP 连接失败 {bind_addr}:{port}: {e}"))?;
    let t = std::time::Duration::from_secs(3);
    let _ = s.set_read_timeout(Some(t));
    let _ = s.set_write_timeout(Some(t));
    let _ = s.set_nodelay(true);
    Ok(s)
}

/// 真本地 host 协议探针（host probe）：
/// a. 合法 0xF000 登录 → 0xF001 result=0
/// b. 畸形帧边界拒绝（声明长度远大于实际：envelope 级丢帧保连接 / 外壳级拒帧关连接），host 存活
/// c. 重连再登录 result=0（critical#1 回归：主循环未被畸形帧杀死）
/// d. 0xF01B teardown 后连接仍可用（ping/pong）
/// 任一 FAIL 最终 exit 1
fn cli_host_probe(port: u16, bind_addr: &str) {
    const TEST_USERID: u64 = 90000001;
    let mut failed = 0u32;
    let mut report = |name: &str, r: Result<(), String>| match r {
        Ok(()) => println!("[PASS] {name}"),
        Err(e) => {
            println!("[FAIL] {name}: {e}");
            failed += 1;
        }
    };

    // 起真本地 host（本进程内嵌线程；探完 stop_running 收尾）
    let runtime_dir = std::env::current_exe()
        .map(|e| e.with_file_name("runtime"))
        .unwrap_or_else(|_| PathBuf::from("runtime"));
    let ready = core::game_host::ensure_running(core::game_host::GameHostParams {
        port,
        runtime_dir,
        env_domain: "editor-pd.spark.xd.com".into(),
        bind_addr: bind_addr.to_string(),
    });
    let port = match ready {
        Ok(p) => {
            println!("host 已就绪: {bind_addr}:{p}");
            p
        }
        Err(e) => {
            report("host 启动", Err(format!("{e}")));
            println!("host probe 完成：{failed} 项失败");
            std::process::exit(1);
        }
    };

    // a. 合法登录帧 → result=0（连接留给 b 复用）
    let conn_a = probe_connect(bind_addr, port).and_then(|mut s| {
        probe_login(&mut s, TEST_USERID)?;
        Ok(s)
    });
    let mut conn_a = match conn_a {
        Ok(s) => {
            report("a. 0xF000 登录 → 0xF001 result=0", Ok(()));
            Some(s)
        }
        Err(e) => {
            report("a. 0xF000 登录 → 0xF001 result=0", Err(e));
            None
        }
    };

    // b1. 外壳长度合法、envelope f1 声明 0xFFFFFFFF（远超实际）→ checked_add 拒绝，只丢帧不断连
    if let Some(s) = &mut conn_a {
        let r = (|| -> Result<(), String> {
            use std::io::Write;
            let mut env = vec![0x0A]; // envelope f1 wt2
            core::host::put_varint(&mut env, 0xFFFF_FFFF); // 声明长度越界
            env.extend_from_slice(b"xx");
            let total = 4 + 1 + env.len();
            let mut frame = Vec::with_capacity(total);
            frame.extend_from_slice(&(total as u32).to_le_bytes());
            frame.push(0);
            frame.extend_from_slice(&env);
            s.write_all(&frame).map_err(|e| format!("发送畸形帧失败: {e}"))?;
            // 帧被丢弃后连接应仍可用：ping → pong
            let mut body = Vec::new();
            core::host::put_field_varint(&mut body, 1, 0);
            probe_send(s, core::host::MSG_EDITOR_PING, &body)?;
            probe_wait(s, core::host::MSG_EDITOR_PING_RES)?;
            Ok(())
        })();
        report("b1. envelope 声明长度越界 → 丢帧、连接存活（ping/pong）", r);
    }

    // b2. 外壳总长声明 0x7FFFFFFF（> 64MB 上限）→ read_frame 拒绝，服务端关闭本连接
    if let Some(s) = &mut conn_a {
        let r = (|| -> Result<(), String> {
            use std::io::Write;
            let mut frame = Vec::new();
            frame.extend_from_slice(&0x7FFF_FFFFu32.to_le_bytes());
            frame.push(0);
            frame.extend_from_slice(b"garbage");
            s.write_all(&frame).map_err(|e| format!("发送超大声明帧失败: {e}"))?;
            // 读到 EOF/对端 RST = 边界拒绝生效；读超时（连接仍在）= 拒绝失效
            loop {
                match probe_read_frame(s) {
                    Ok(None) => return Ok(()),
                    Ok(Some(f)) if f.msg_type == core::host::MSG_NOTIFY_EDITOR_LOG => continue,
                    Ok(Some(f)) => return Err(format!("收到意外帧 {:#x}（连接未被关闭）", f.msg_type)),
                    Err(e) if e == "读超时" => return Err("连接未被关闭（3s 读超时）".into()),
                    Err(e) if e.starts_with("读帧") => return Ok(()), // 对端 RST 等 = 连接被关
                    Err(e) => return Err(format!("连接未被关闭且收到垃圾: {e}")),
                }
            }
        })();
        report("b2. 外壳声明超大（>64MB）→ 边界拒绝，连接被关", r);
    }
    drop(conn_a);

    // c. 重连再登录 result=0（critical#1 回归：畸形帧未杀死 host 主循环）
    let r = probe_connect(bind_addr, port).and_then(|mut s| {
        probe_login(&mut s, TEST_USERID)?;
        Ok(s)
    });
    let mut conn_c = match r {
        Ok(s) => {
            report("c. 畸形帧后重连登录 result=0（host 主循环存活）", Ok(()));
            Some(s)
        }
        Err(e) => {
            report("c. 畸形帧后重连登录 result=0（host 主循环存活）", Err(e));
            None
        }
    };

    // d. 0xF01B teardown 帧：停局不应断连（ping/pong 验证连接仍可用）
    if let Some(s) = &mut conn_c {
        let r = (|| -> Result<(), String> {
            probe_send(s, core::host::MSG_DESTROY_GAME, &[])?;
            let mut body = Vec::new();
            core::host::put_field_varint(&mut body, 1, 0);
            probe_send(s, core::host::MSG_EDITOR_PING, &body)?;
            probe_wait(s, core::host::MSG_EDITOR_PING_RES)?;
            Ok(())
        })();
        report("d. 0xF01B teardown 后连接仍可用（ping/pong）", r);
    }
    drop(conn_c);

    // 收尾：停 host 释放端口
    core::game_host::stop_running();
    std::thread::sleep(std::time::Duration::from_millis(500));
    if failed > 0 {
        println!("host probe 完成：{failed} 项失败");
        std::process::exit(1);
    }
    println!("host probe 全部通过");
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
    verify_rx: Option<std::sync::mpsc::Receiver<(String, Result<String, String>)>>,
    refresh_rx: Option<std::sync::mpsc::Receiver<(String, Result<(i64, Option<String>), String>)>>,
    // 自登录
    login_qr: Option<egui::TextureHandle>,
    login_state: Option<core::login::LoginState>,
    login_grant: Option<core::login::DeviceGrant>,
    login_rx: Option<std::sync::mpsc::Receiver<core::login::LoginState>>,
    /// 扫码轮询取消标志（「取消登录」置位，后台线程尽快退出）
    login_cancel: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    // 调试
    debug_session: Option<core::debug::DebugSession>,
    debug_status: Option<core::debug::DebugStatus>,
    debug_staging_input: String,
    debug_userid_input: String,
    debug_runtime_input: String,
    debug_start_rx: Option<std::sync::mpsc::Receiver<ui::debug::StartOutcome>>,
    /// 运行时选择（0=编辑器-api（默认） 1=对战平台测试 2=对战平台正式）
    debug_kind_sel: usize,
    /// host 模式（0=云端直连（默认） 1=本地中继 2=真本地）
    debug_host_sel: usize,
    /// 附加客户端多开数量（0-3，按凭证库顺序取）
    debug_extra_clients_sel: usize,
    /// 本地服务器标签页：账号列表 / 重载标记 / 新建名 / 已拉起客户端（账号id→pid）/ 启动结果通道
    ls_accounts: Vec<crate::core::local_accounts::LocalAccount>,
    ls_need_reload: bool,
    ls_new_name: String,
    ls_clients: ui::local_server::ClientMap,
    ls_launch_rx: Option<std::sync::mpsc::Receiver<Result<(i64, u32), String>>>,
    /// host 启动/重启结果通道（工作线程 → UI，失败写状态栏）
    ls_host_rx: Option<std::sync::mpsc::Receiver<Result<u16, String>>>,
    /// 本地服务器日志面板：缓冲（logbus 拉取）/ 已消费序号 / 滚屏开关 / 关键字筛选
    ls_logs: std::collections::VecDeque<String>,
    ls_log_seq: u64,
    ls_log_scroll: bool,
    ls_log_filter: String,
    /// host 绑定范围（0=本地 127.0.0.1 / 1=局域网 / 2=外网；后两者绑 0.0.0.0）
    ls_bind_mode: usize,
    /// 启动前自动 payload sync 的进度
    debug_progress_rx: Option<std::sync::mpsc::Receiver<String>>,
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
            verify_rx: None,
            refresh_rx: None,
            login_qr: None,
            login_state: None,
            login_grant: None,
            login_rx: None,
            login_cancel: None,
            debug_session: None,
            debug_status: None,
            debug_staging_input: String::new(),
            debug_userid_input: String::new(),
            debug_runtime_input: String::new(),
            debug_start_rx: None,
            debug_kind_sel: 0,
            debug_host_sel: 0,
            debug_extra_clients_sel: 0,
            ls_accounts: Vec::new(),
            ls_need_reload: true,
            ls_new_name: String::new(),
            ls_clients: Default::default(),
            ls_launch_rx: None,
            ls_host_rx: None,
            ls_logs: Default::default(),
            ls_log_seq: 0,
            ls_log_scroll: true,
            ls_log_filter: String::new(),
            ls_bind_mode: 0,
            debug_progress_rx: None,
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
    bgd_appsdk::ui::ShellTab { id: "local_server", label: "本地服务器" },
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
            "local_server" => self.ui_local_server(ui),
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
