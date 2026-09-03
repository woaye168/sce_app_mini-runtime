//! host 侧 HTTP 文件分发服务（0.6.0）：把 staging 目录分发给局域网/外网远端客户端。
//!
//! 手写 std::net::TcpListener 极简 HTTP/1.x（不引入新依赖）：
//! - `GET /manifest` → 实时扫 staging 白名单（与 staging.rs 同集合），逐文件 xxh64 + size 出 JSON
//! - `GET /file?path=<相对路径>` → 回文件字节（路径穿越防护 + 白名单限定）
//! - keep-alive 多请求复用连接（对端逐文件下载量上千，一请求一连接会把路由器
//!   NAT 连接跟踪表打满——外网实测中途断连；30s 空闲超时自动关）
//!
//! 服务模式与 host_server.rs 一致：非阻塞 accept 轮询（100ms）+ stop 信号退出。

use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

/// staging 白名单（与 staging.rs 的 DIRS/FILES 保持同集合；那边是私有的，此处复制并注释同步义务）
const DIRS: &[&str] = &[
    "atmosphere", "block", "game_hud", "i18n", "project", "ref", "res", "scene", "script",
    "src", "table", "ui",
];
const FILES: &[&str] = &["config.ini", "libs.json", "project.sce"];

/// 请求头读取上限（防恶意超长头占内存）
const MAX_HEADER: usize = 16 * 1024;
/// 并发连接上限（防慢/恶意连接批量建连耗尽宿主线程）
const MAX_CONNS: usize = 64;
/// 活跃连接计数（配合 MAX_CONNS）
static ACTIVE_CONNS: AtomicUsize = AtomicUsize::new(0);
/// /base_assets 临时目录连接级唯一后缀（同进程并发请求共用 pid 会互踩）
static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

pub struct DistribParams {
    pub port: u16,
    pub staging_dir: PathBuf,
    pub bind_addr: String,
    /// 环境域名（基座资产 = <runtime>/Update/<域>/Res，由 RuntimeKind 决定，不能硬编码编辑器域）
    pub env_domain: String,
}

/// 分发服务监听线程体（game_host 拉起）
/// stop 置位后退出（非阻塞 accept 轮询）
/// bind_addr：127.0.0.1 = 仅本机；0.0.0.0 = 局域网/外网（远端客户端拉 staging 入局）
pub fn run(params: DistribParams, stop: Arc<AtomicBool>) -> Result<()> {
    let addr = format!("{}:{}", params.bind_addr, params.port);
    let listener = TcpListener::bind(&addr).map_err(|e| anyhow!("分发服务监听失败 {addr}: {e}"))?;
    listener.set_nonblocking(true)?;
    crate::srv_log!("[distrib] 分发服务已监听 {addr}（staging = {}）", crate::core::disp(&params.staging_dir));
    let staging_dir = Arc::new(params.staging_dir);
    let env_domain = Arc::new(params.env_domain);
    while !stop.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                // Windows 下 accept 的套接字会继承 listener 的 nonblocking 标志，
                // 必须显式还原为阻塞模式，否则读请求即 10035 WSAEWOULDBLOCK 断连
                if let Err(e) = stream.set_nonblocking(false) {
                    crate::srv_log!("[distrib] 还原阻塞模式失败: {e}");
                    continue;
                }
                if ACTIVE_CONNS.load(Ordering::Relaxed) >= MAX_CONNS {
                    crate::srv_log!("[distrib] 活跃连接达上限 {MAX_CONNS}，拒绝新连接");
                    continue; // stream 落出作用域即关闭
                }
                ACTIVE_CONNS.fetch_add(1, Ordering::Relaxed);
                let dir = Arc::clone(&staging_dir);
                let env = Arc::clone(&env_domain);
                std::thread::spawn(move || {
                    if let Err(e) = handle_conn(stream, &dir, &env) {
                        crate::srv_log!("[distrib] 连接结束: {e}");
                    }
                    ACTIVE_CONNS.fetch_sub(1, Ordering::Relaxed);
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => crate::srv_log!("[distrib] accept 失败: {e}"),
        }
    }
    crate::srv_log!("[distrib] 分发服务已停止 {addr}");
    Ok(())
}

/// 单连接处理：keep-alive 多请求循环（对端上千个文件逐一下载，一请求一连接
/// 会让路由器 NAT 连接跟踪表剧烈抖动——外网实测中途 "error sending request"）。
/// 读到 \r\n\r\n 即止（GET 无 body），30s 空闲超时自动关闭防线程悬挂。
fn handle_conn(mut s: TcpStream, staging_dir: &Path, env_domain: &str) -> Result<()> {
    s.set_read_timeout(Some(std::time::Duration::from_secs(30)))?;
    // 写超时：大文件响应对端不收数据时防线程永久卡在 write_all
    s.set_write_timeout(Some(std::time::Duration::from_secs(60)))?;
    let mut buf: Vec<u8> = Vec::with_capacity(4096);
    let mut chunk = [0u8; 4096];
    loop {
        let head_end = loop {
            if let Some(p) = find_subslice(&buf, b"\r\n\r\n") {
                break p;
            }
            if buf.len() >= MAX_HEADER {
                return respond(&mut s, 431, "text/plain", b"request header too large", false);
            }
            match s.read(&mut chunk) {
                Ok(0) => return Ok(()), // 对端关闭
                Ok(n) => buf.extend_from_slice(&chunk[..n]),
                // 读超时（WouldBlock/TimedOut）= 空闲 keep-alive 到期，正常关闭
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    return Ok(())
                }
                Err(e) => return Err(anyhow!("读请求失败: {e}")),
            }
        };
        let head = String::from_utf8_lossy(&buf[..head_end]).to_string();
        buf.drain(..head_end + 4);
        // HTTP/1.1 默认 keep-alive；仅显式 Connection: close 时响应后关闭
        let keep_alive = !head
            .lines()
            .any(|l| l.eq_ignore_ascii_case("connection: close"));
        let request_line = head.lines().next().unwrap_or("");
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("");
        let target = parts.next().unwrap_or("").to_string();
        if method != "GET" {
            return respond(&mut s, 405, "text/plain", b"method not allowed", false);
        }
        route(&mut s, &target, staging_dir, env_domain, keep_alive)?;
        if !keep_alive {
            return Ok(());
        }
    }
}

/// 路由：解析 target 并写响应（keep-alive 由 handle_conn 循环维持）。
/// 可预期错误一律转成 4xx/500 响应写出后再关连接（静默断连与 NAT 断连故障形态相同，无法排障）。
fn route(s: &mut TcpStream, target: &str, staging_dir: &Path, env_domain: &str, keep_alive: bool) -> Result<()> {

    let (route, query) = target.split_once('?').unwrap_or((target, ""));
    match route {
        "/manifest" => match build_manifest(staging_dir) {
            Ok(body) => respond(s, 200, "application/json", body.as_bytes(), keep_alive),
            Err(e) => {
                crate::srv_log!("[distrib] manifest 生成失败: {e}");
                respond(s, 500, "text/plain", b"internal error", keep_alive)
            }
        },
        "/base_assets" => {
            // 基座资产（update-info 不分发的编辑器资产）：房主本机已同步到
            // <runtime>/Update/<env>/Res + <runtime>/Res，打 zip 整体下发（对端零 GitHub token）。
            // staging 同级定位 runtime 根：staging = <runtime>/User/debug/<项目>
            match build_base_assets(staging_dir, env_domain) {
                Ok(bytes) => {
                    crate::srv_log!("[distrib] 基座资产打包下发 {} MB", bytes.len() / 1024 / 1024);
                    respond(s, 200, "application/octet-stream", &bytes, keep_alive)
                }
                Err(e) => {
                    crate::srv_log!("[distrib] 基座资产不可用: {e}");
                    respond(s, 404, "text/plain", b"base assets unavailable", keep_alive)
                }
            }
        }
        "/file" => {
            let Some(raw) = query.split('&').find_map(|kv| kv.strip_prefix("path=")) else {
                return respond(s, 400, "text/plain", b"missing path parameter", keep_alive);
            };
            let rel = url_decode(raw);
            match resolve_file(staging_dir, &rel) {
                Ok(path) => {
                    let bytes = std::fs::read(&path)
                        .map_err(|e| anyhow!("读文件失败 {}: {e}", crate::core::disp(&path)))?;
                    respond(s, 200, "application/octet-stream", &bytes, keep_alive)
                }
                Err(e) => {
                    crate::srv_log!("[distrib] 拒绝文件请求 path={raw}: {e}");
                    respond(s, 404, "text/plain", b"not found", keep_alive)
                }
            }
        }
        _ => respond(s, 404, "text/plain", b"not found", keep_alive),
    }
}

/// 写响应（Content-Length 定长；keep_alive 决定 Connection 头，循环由调用方维持）
fn respond(s: &mut TcpStream, code: u16, content_type: &str, body: &[u8], keep_alive: bool) -> Result<()> {
    let reason = match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        431 => "Request Header Fields Too Large",
        500 => "Internal Server Error",
        _ => "Error",
    };
    let conn = if keep_alive { "keep-alive" } else { "close" };
    let head = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: {conn}\r\n\r\n",
        body.len()
    );
    s.write_all(head.as_bytes())?;
    s.write_all(body)?;
    Ok(())
}

/// 基座资产打包：从 runtime 根（staging 上三级：staging=User/debug/<项目>）收集
/// Update/<env>/Res/{ui,fonts} + Res/{characters,effect}，打 7z 到内存返回。
/// env_domain 由房主运行时种类决定（payload 落位用同一域）；临时目录带连接级唯一后缀防并发互踩。
fn build_base_assets(staging_dir: &Path, env_domain: &str) -> Result<Vec<u8>> {
    // staging = <runtime>/User/debug/<项目> → runtime = 上三级
    let runtime = staging_dir
        .ancestors()
        .nth(3)
        .ok_or_else(|| anyhow!("staging 路径层级不足"))?
        .to_path_buf();
    let res_root = runtime.join("Update").join(env_domain).join("Res");
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp_x = std::env::temp_dir().join(format!("bgd_base_x_{}_{}", std::process::id(), seq));
    let _ = std::fs::remove_dir_all(&tmp_x);
    // 复刻 base_assets.7z 结构：ui/+fonts/（Res 下）+ characters/+effect/（runtime/Res 下）
    let pairs = [
        (res_root.join("ui"), tmp_x.join("ui")),
        (res_root.join("fonts"), tmp_x.join("fonts")),
        (runtime.join("Res").join("characters"), tmp_x.join("characters")),
        (runtime.join("Res").join("effect"), tmp_x.join("effect")),
    ];
    let mut any = false;
    for (src, dst) in &pairs {
        if src.is_dir() {
            any = true;
            copy_tree(src, dst)?;
        }
    }
    if !any {
        let _ = std::fs::remove_dir_all(&tmp_x);
        return Err(anyhow!("runtime 内无基座资产（房主需先跑一次完整 payload sync）"));
    }
    // 注：bsdtar 不能写 7z（只读），改打 zip（read 端 `tar -xf` 自动识别，payload 解包无需改）
    let tmp = std::env::temp_dir().join(format!("bgd_base_{}_{}.zip", std::process::id(), seq));
    let status = crate::core::silent_command("tar")
        .args(["-acf"])
        .arg(&tmp)
        .arg("-C")
        .arg(&tmp_x)
        .arg(".")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| anyhow!("tar 打包失败: {e}"))?;
    let bytes = if status.success() {
        std::fs::read(&tmp).map_err(|e| anyhow!("读 7z 失败: {e}"))?
    } else {
        let _ = std::fs::remove_dir_all(&tmp_x);
        return Err(anyhow!("tar 打包退出码异常"));
    };
    let _ = std::fs::remove_file(&tmp);
    let _ = std::fs::remove_dir_all(&tmp_x);
    Ok(bytes)
}

/// 递归复制目录（payload 内私有，distrib 自带一份避免改主线文件）
fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let s = entry.path();
        let d = dst.join(entry.file_name());
        if s.is_dir() {
            copy_tree(&s, &d)?;
        } else {
            std::fs::copy(&s, &d)?;
        }
    }
    Ok(())
}

fn build_manifest(staging_dir: &Path) -> Result<String> {
    let project = staging_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut files = Vec::new();
    for d in DIRS {
        let dir = staging_dir.join(d);
        if dir.is_dir() {
            collect_dir(&dir, staging_dir, &mut files)?;
        }
    }
    for f in FILES {
        let path = staging_dir.join(f);
        if path.is_file() {
            files.push(file_entry(staging_dir, &path)?);
        }
    }
    // 项目依赖库（libs.json 键）：对端 payload 同步要按它装 _m/maps 库，
    // 缺了客户端 lua 入口 require @global_default 直接失败卡加载
    let libs: Vec<String> = std::fs::read_to_string(staging_dir.join("libs.json"))
        .ok()
        .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
        .and_then(|v| v.as_object().map(|o| o.keys().cloned().collect()))
        .unwrap_or_default();
    let body = serde_json::json!({
        "project": project,
        "libs": libs,
        "files": files,
    });
    crate::srv_log!("[distrib] manifest：{} 个文件，{} 个依赖库", files.len(), libs.len());
    Ok(body.to_string())
}

/// 递归收集目录内文件条目（相对 staging 根，正斜杠路径）
fn collect_dir(dir: &Path, root: &Path, out: &mut Vec<serde_json::Value>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_dir(&path, root, out)?;
        } else {
            out.push(file_entry(root, &path)?);
        }
    }
    Ok(())
}

/// 单文件条目：{"path", "size", "hash"}（hash = xxh64 hex）
fn file_entry(root: &Path, path: &Path) -> Result<serde_json::Value> {
    let bytes = std::fs::read(path).map_err(|e| anyhow!("读文件失败 {}: {e}", crate::core::disp(path)))?;
    let rel = path
        .strip_prefix(root)
        .map_err(|_| anyhow!("路径脱离 staging 根: {}", crate::core::disp(path)))?;
    Ok(serde_json::json!({
        "path": rel.display().to_string().replace('\\', "/"),
        "size": bytes.len() as u64,
        "hash": format!("{:016x}", xxhash_rust::xxh64::xxh64(&bytes, 0)),
    }))
}

/// /file 路径解析：URL-decode 后 + 路径穿越防护（拒 .. / 绝对路径）+ 白名单限定
/// 返回 staging 内的完整文件路径；任何不合规一律 Err（上层回 404，不泄露细节）
fn resolve_file(staging_dir: &Path, rel: &str) -> Result<PathBuf> {
    if rel.is_empty() {
        return Err(anyhow!("空路径"));
    }
    let rel_path = Path::new(rel);
    // 路径穿越防护：拒绝 .. / 盘符 / 根（绝对路径）等一切非常规分量
    for comp in rel_path.components() {
        match comp {
            Component::Normal(_) => {}
            _ => return Err(anyhow!("非法路径分量: {rel}")),
        }
    }
    // 白名单限定：首段必须命中 DIRS，或整体命中根级 FILES
    let first = rel_path
        .components()
        .next()
        .and_then(|c| match c {
            Component::Normal(s) => s.to_str(),
            _ => None,
        })
        .ok_or_else(|| anyhow!("路径为空: {rel}"))?;
    let allowed = DIRS.contains(&first)
        || (rel_path.components().count() == 1 && FILES.contains(&first));
    if !allowed {
        return Err(anyhow!("不在白名单: {rel}"));
    }
    let path = staging_dir.join(rel_path);
    if !path.is_file() {
        return Err(anyhow!("文件不存在: {rel}"));
    }
    Ok(path)
}

/// URL percent-decode（%XX 十六进制转义；+ 按 query 约定转空格）
fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
                match u8::from_str_radix(hex, 16) {
                    Ok(v) => {
                        out.push(v);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}
