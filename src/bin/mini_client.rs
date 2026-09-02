//! mini-client：对端（朋友机器）的轻量联机启动器。
//! 双击 → 输房主 IP/端口/userid → 异步下载引擎（官方 OSS）与游戏（房主分发口）→ 进局。
//! 核心逻辑全部复用库 crate `sce_app_mini_runtime::core`（payload/debug/auth/runtimes）。
//!
//! 房主侧约定：
//! - 游戏连接端口 = 本地服务器 host 端口（默认 5003）
//! - 分发端口 = 连接端口 + 80（默认 5083）：GET /manifest、GET /file?path=<URL编码>

// Windows 下不弹出黑色控制台窗口
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use anyhow::{anyhow, Result};
use sce_app_mini_runtime::core;
use std::path::{Component, Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

/// 编辑器运行时 api_version（对端写死 13，与房主侧编辑器包一致）
const API_VERSION: u32 = 13;
/// 环境域（编辑器正式环境；凭证文件名与 -server 参数都要）
const ENV_DOMAIN: &str = "editor-pd.spark.xd.com";
/// 分发口相对连接口的固定偏移（5003 → 5083）
const DIST_PORT_OFFSET: u16 = 80;

fn main() -> eframe::Result<()> {
    // 无界面冒烟测试钩子：`mini-client --connect <ip> <port> <userid>`（联调/CI 用，不走 GUI）
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 5 && args[1] == "--connect" {
        let port: u16 = args[3].parse().expect("端口须为数字");
        let userid: i64 = args[4].parse().expect("userid 须为数字");
        let r = run_connect(&args[2], port, userid, &std::sync::mpsc::channel().0);
        match r {
            Ok(msg) => { println!("OK: {msg}"); std::process::exit(0); }
            Err(e) => { eprintln!("FAIL: {e}"); std::process::exit(1); }
        }
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 360.0])
            .with_min_inner_size([420.0, 320.0])
            // 窗口标题栏/任务栏图标（exe 文件图标由 build.rs 嵌 assets/app.ico）
            .with_icon(
                eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icon.png"))
                    .expect("内嵌图标 PNG 解码失败"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "联机客户端",
        options,
        Box::new(|cc| {
            // egui 默认字体无中文字形，装系统中文字体（微软雅黑，appsdk 壳同款）
            bgd_appsdk::ui::setup_chinese_font(&cc.egui_ctx);
            Ok(Box::new(MiniClientApp::load()))
        }),
    )
}

/// exe 旁文件定位（记忆文件/缓存/运行时全在 exe 同级，绿色免安装）
fn exe_sibling(name: &str) -> PathBuf {
    std::env::current_exe()
        .map(|e| e.with_file_name(name))
        .unwrap_or_else(|_| PathBuf::from(name))
}

/// 上次输入记忆（exe 旁 mini_client.ini，serde_json 存）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Ini {
    ip: String,
    port: String,
    userid: String,
}

impl Default for Ini {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".into(),
            port: "5003".into(),
            userid: String::new(),
        }
    }
}

impl Ini {
    fn load() -> Self {
        std::fs::read_to_string(exe_sibling("mini_client.ini"))
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    }
    fn save(&self) {
        if let Ok(c) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(exe_sibling("mini_client.ini"), c);
        }
    }
}

/// 工作线程 → UI 的消息
enum Msg {
    /// 进度：frac=None 表示不确定进度（引擎下载阶段），Some 表示 0..=1
    Progress(Option<f32>, String),
    /// 流程结束（Ok=已进局提示语，Err=错误原因）
    Done(Result<String, String>),
}

struct MiniClientApp {
    ip: String,
    port: String,
    userid: String,
    /// 状态文本（进度/结果/错误共用一行展示区）
    status: String,
    /// 状态是否错误（红色显示）
    status_err: bool,
    /// 当前进度 0..=1（None=不确定，转圈）
    frac: Option<f32>,
    /// 工作线程回传通道（Some = 连接中）
    rx: Option<Receiver<Msg>>,
}

impl MiniClientApp {
    fn load() -> Self {
        let ini = Ini::load();
        Self {
            ip: ini.ip,
            port: ini.port,
            userid: ini.userid,
            status: "输入房主 IP、端口与userid后点「连接」".into(),
            status_err: false,
            frac: None,
            rx: None,
        }
    }

    /// 点「连接」：校验输入 → 起工作线程（严禁卡 UI）
    fn start_connect(&mut self) {
        let ip = self.ip.trim().to_string();
        if ip.is_empty() {
            self.status = "IP 不能为空".into();
            self.status_err = true;
            return;
        }
        let port: u16 = match self.port.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                self.status = "端口必须是数字（默认 5003）".into();
                self.status_err = true;
                return;
            }
        };
        let userid: i64 = match self.userid.trim().parse() {
            Ok(v) if v > 0 => v,
            _ => {
                self.status = "userid 必须是正整数（房主侧「本地服务器」里创建）".into();
                self.status_err = true;
                return;
            }
        };
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let result = run_connect(&ip, port, userid, &tx);
            let _ = tx.send(Msg::Done(result));
        });
        self.rx = Some(rx);
        self.frac = None;
        self.status = "连接中...".into();
        self.status_err = false;
    }
}

impl eframe::App for MiniClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 抽干工作线程消息（每帧 try_recv，不阻塞）
        if let Some(rx) = &self.rx {
            let mut done = None;
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    Msg::Progress(frac, text) => {
                        self.frac = frac;
                        self.status = text;
                        self.status_err = false;
                    }
                    Msg::Done(result) => done = Some(result),
                }
            }
            if let Some(result) = done {
                match result {
                    Ok(text) => {
                        self.status = format!("{text}（可关闭本窗口）");
                        self.status_err = false;
                        self.frac = Some(1.0);
                        // 连接成功才记忆本次输入
                        Ini {
                            ip: self.ip.clone(),
                            port: self.port.clone(),
                            userid: self.userid.clone(),
                        }
                        .save();
                    }
                    Err(e) => {
                        self.status = e;
                        self.status_err = true;
                        self.frac = None;
                    }
                }
                self.rx = None;
            }
            // 连接中保持重绘，进度才能动
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        let busy = self.rx.is_some();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("联机客户端（连房主的本地服务器）");
            ui.add_space(8.0);

            ui.add_enabled_ui(!busy, |ui| {
                ui.horizontal(|ui| {
                    ui.label("房主 IP:");
                    ui.text_edit_singleline(&mut self.ip);
                });
                ui.horizontal(|ui| {
                    ui.label("端口:    ");
                    ui.text_edit_singleline(&mut self.port);
                });
                ui.horizontal(|ui| {
                    ui.label("userid: ");
                    ui.text_edit_singleline(&mut self.userid);
                });
                ui.add_space(6.0);
                if ui.button("连接").clicked() {
                    self.start_connect();
                }
            });

            ui.add_space(10.0);
            // 进度区：不确定进度转圈，确定进度进度条
            match self.frac {
                Some(f) => {
                    ui.add(egui::ProgressBar::new(f).show_percentage());
                }
                None if busy => {
                    ui.spinner();
                }
                None => {}
            }
            let color = if self.status_err {
                egui::Color32::from_rgb(220, 60, 60)
            } else {
                ui.visuals().text_color()
            };
            ui.colored_label(color, &self.status);
        });
    }
}

/// 连接全流程（工作线程内执行）：引擎就绪 → 拉 manifest → 增量下载 → 写凭证 → 拉起客户端
fn run_connect(ip: &str, port: u16, userid: i64, tx: &Sender<Msg>) -> Result<String, String> {
    let send = |frac: Option<f32>, text: String| {
        eprintln!("[mc] {text}"); // 冒烟钩子（--connect）下进度落终端；GUI 下 stderr 无控制台无害
        let _ = tx.send(Msg::Progress(frac, text));
    };
    connect_inner(ip, port, userid, &send).map_err(|e| format!("{e:#}"))
}

fn connect_inner(
    ip: &str,
    port: u16,
    userid: i64,
    send: &dyn Fn(Option<f32>, String),
) -> Result<String> {
    let kind = core::runtimes::RuntimeKind::EditorApi(API_VERSION);
    let runtime_dir = exe_sibling("runtime");

    // ① 先拉游戏 manifest（分发端口 = 连接端口 + 80）：依赖库清单在载荷同步里要用
    let dist_port = port + DIST_PORT_OFFSET;
    let base = format!("http://{ip}:{dist_port}");
    send(None, format!("拉取游戏清单: {base}/manifest"));
    // LAN 直连禁用系统/环境代理（否则本机 HTTP_PROXY 会把局域网请求拐走）
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(300))
        .build()?;
    let resp = client
        .get(format!("{base}/manifest"))
        .send()
        .map_err(|e| anyhow!("连不上房主分发口 {base}（本地服务器需在「局域网/外网」模式）: {e}"))?;
    if !resp.status().is_success() {
        return Err(anyhow!("拉取游戏清单失败: HTTP {}", resp.status()));
    }
    let manifest: Manifest = resp
        .json()
        .map_err(|e| anyhow!("游戏清单解析失败: {e}"))?;
    send(
        Some(0.0),
        format!("游戏「{}」共 {} 个文件，比对本地缓存...", manifest.project, manifest.files.len()),
    );

    // ② 引擎/依赖库就绪检查：缺则走官方 update-info + OSS 下载（首次约 150MB）。
    // 基座资产 update-info 不分发、对端无 GitHub token —— 注入房主分发口 /base_assets 直下。
    // 依赖库（libs.json 键）必须随载荷落位 _m/maps —— 缺了客户端 lua 入口
    // require @global_default 直接报错，卡在「正在加载游戏逻辑」
    // SAFETY：连接流程单线程顺序执行，无并发 env 竞争
    unsafe {
        std::env::set_var(
            "MINI_RUNTIME_BASE_ASSETS_URL",
            format!("http://{ip}:{dist_port}/base_assets"),
        );
    }
    let libs_missing = !manifest.libs.is_empty() && !libs_placed(&runtime_dir, &manifest.libs);
    if !kind.engine_ready(&runtime_dir) || libs_missing {
        send(
            None,
            format!("引擎/依赖库未就绪，下载运行时中（{}，首次较慢）...", kind.display_name()),
        );
        let params = core::payload::SyncParams {
            runtime_dir: runtime_dir.clone(),
            env_domain: ENV_DOMAIN.to_string(),
            api_version: API_VERSION,
            project_libs: manifest.libs.clone(),
            project_root: None, // 对端无本机编辑器，基座资产走自分发下载
            dry_run: false,
            runtime_kind: Some(kind),
        };
        let mut log = |msg: String| send(None, format!("[载荷] {msg}"));
        core::payload::sync(&params, &mut log).map_err(|e| anyhow!("引擎下载失败: {e}"))?;
        send(None, "引擎就绪".into());
    } else {
        send(None, "引擎已就绪".into());
    }

    // ③ 增量比对本地缓存 exe 旁 cache/<project>/（size + xxh64 双判）
    let staging_dir = exe_sibling("cache").join(&manifest.project);
    let mut download_list: Vec<&ManifestFile> = Vec::new();
    for f in &manifest.files {
        if !cache_file_matches(&staging_dir, f) {
            download_list.push(f);
        }
    }
    if download_list.is_empty() {
        send(Some(1.0), "本地缓存已是最新，无需下载".into());
    }

    // ④ 逐文件下载到缓存（创建父目录，进度 i/N；外网 NAT 偶发断连，单文件最多 3 次重试）
    let total = download_list.len();
    for (i, f) in download_list.iter().enumerate() {
        send(
            Some(i as f32 / total.max(1) as f32),
            format!("正在下载 {}/{}: {}", i + 1, total, f.path),
        );
        let mut last_err: Option<anyhow::Error> = None;
        for attempt in 1..=3u8 {
            match download_one(&client, &base, f, &staging_dir) {
                Ok(()) => {
                    last_err = None;
                    break;
                }
                Err(e) => {
                    last_err = Some(e);
                    if attempt < 3 {
                        send(None, format!("下载失败，300ms 后第 {} 次重试: {}", attempt + 1, f.path));
                        std::thread::sleep(Duration::from_millis(300));
                    }
                }
            }
        }
        if let Some(e) = last_err {
            return Err(e);
        }
    }

    // ⑤ 写合成凭证（本地 host 放行任意 token；login=1 + token 形态合法即可过大厅登录闸门）
    let cred = core::auth::UserInfo {
        login: 1,
        login_type: "local".into(),
        token: format!("local-{userid}"),
        token_type: 11, // token_valid() 合法区间 [11,14]
        version: 1,
        userid: Some(userid),
        user_name: Some(format!("player-{userid}")),
        ..Default::default()
    };
    let user_info_path = runtime_dir
        .join("User")
        .join(format!("user_info-{ENV_DOMAIN}.json"));
    core::auth::write_user_info(&user_info_path, &cred)
        .map_err(|e| anyhow!("写凭证失败: {e}"))?;

    // ⑥ 拉起引擎客户端（B 模式命令行契约；CreateProcessW 防管道继承卡死）
    send(Some(1.0), "启动游戏客户端...".into());
    let exe = kind.client_exe(&runtime_dir);
    if !exe.is_file() {
        return Err(anyhow!(
            "游戏客户端不存在: {}（引擎未就绪）",
            core::disp(&exe)
        ));
    }
    let args = core::debug::build_client_args(
        ip,
        port,
        userid,
        &staging_dir,
        &manifest.project,
        API_VERSION,
        ENV_DOMAIN,
    );
    let pid = core::debug::spawn_detached_pub(&exe, &args, &runtime_dir)
        .map_err(|e| anyhow!("拉起客户端失败: {e}"))?;
    Ok(format!("已进局（pid={pid}）"))
}

/// 房主分发口 manifest
#[derive(Debug, serde::Deserialize)]
struct Manifest {
    project: String,
    /// 项目依赖库（libs.json 键；旧版房主无此字段时为空 = 跳过依赖库检查）
    #[serde(default)]
    libs: Vec<String>,
    files: Vec<ManifestFile>,
}

/// 依赖库是否已全部登记进 api_pak_version.json（payload 落位标志）
fn libs_placed(runtime_dir: &Path, libs: &[String]) -> bool {
    let reg_path = runtime_dir
        .join("Update")
        .join(ENV_DOMAIN)
        .join("api_pak_version.json");
    let Ok(content) = std::fs::read_to_string(reg_path) else {
        return false;
    };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else {
        return false;
    };
    let Some(api_obj) = v.get(API_VERSION.to_string()).and_then(|o| o.as_object()) else {
        return false;
    };
    libs.iter().all(|l| api_obj.contains_key(l))
}

#[derive(Debug, serde::Deserialize)]
struct ManifestFile {
    path: String,
    size: u64,
    /// xxh64 内容哈希（兼容数字/十进制串/十六进制串三种形态）
    hash: serde_json::Value,
}

impl ManifestFile {
    fn hash_u64(&self) -> Option<u64> {
        match &self.hash {
            serde_json::Value::Number(n) => n.as_u64(),
            serde_json::Value::String(s) => u64::from_str_radix(s, 16)
                .ok()
                .or_else(|| s.parse::<u64>().ok()),
            _ => None,
        }
    }
}

/// 下载单个文件到缓存（供重试循环调用）
fn download_one(
    client: &reqwest::blocking::Client,
    base: &str,
    f: &ManifestFile,
    staging_dir: &Path,
) -> Result<()> {
    let url = format!("{base}/file?path={}", url_encode_path(&f.path));
    let resp = client
        .get(&url)
        .send()
        .map_err(|e| anyhow!("下载失败 {}: {e}", f.path))?;
    if !resp.status().is_success() {
        return Err(anyhow!("下载失败 {}: HTTP {}", f.path, resp.status()));
    }
    let bytes = resp
        .bytes()
        .map_err(|e| anyhow!("读取失败 {}: {e}", f.path))?;
    let dest = safe_join(staging_dir, &f.path)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, &bytes)?;
    Ok(())
}

/// 本地缓存文件是否与清单一致（存在 + size + xxh64 全对才算命中）
fn cache_file_matches(staging_dir: &Path, f: &ManifestFile) -> bool {
    let Ok(dest) = safe_join(staging_dir, &f.path) else {
        return false;
    };
    let Ok(data) = std::fs::read(&dest) else {
        return false;
    };
    if data.len() as u64 != f.size {
        return false;
    }
    match f.hash_u64() {
        Some(h) => xxhash_rust::xxh64::xxh64(&data, 0) == h,
        None => true, // 清单未带哈希时按 size 判定
    }
}

/// 缓存目录安全拼接：拒绝绝对路径与 .. 穿越（清单来自网络，防恶意路径）
fn safe_join(root: &Path, rel: &str) -> Result<PathBuf> {
    let rel = rel.replace('\\', "/");
    let mut out = root.to_path_buf();
    for seg in rel.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        let comp = Path::new(seg);
        if comp.components().count() != 1
            || matches!(
                comp.components().next(),
                Some(Component::ParentDir | Component::RootDir | Component::Prefix(_))
            )
        {
            return Err(anyhow!("清单含非法路径: {rel}"));
        }
        out.push(seg);
    }
    Ok(out)
}

/// 极简 URL 编码：path 分段保留 '/'，非字母数字/-/_/. 的字节一律 %XX（中文等多字节按 UTF-8 逐字节）
fn url_encode_path(p: &str) -> String {
    let mut out = String::with_capacity(p.len());
    for &b in p.replace('\\', "/").as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
