//! 自研 host 控制面：TCP 服务端（0xF000 段协议，编辑器/本应用 CLI 接入）。
//!
//! 协议权威 = doc/research/scegame-reverse.md §8 + editor-debug-channels.md §2。
//! 起点参照 sce_app_editor-patch/examples/host_stub.rs（编辑器侧全链已验证）：
//! - EditorLogin(0xF000) → EditorLoginResult(0xF001)（本地放行任意 token，编辑器侧为 "qwert"）
//! - 上传 0xF004（整发/大文件空声明）/ 0xF008（分块）/ 0xF00A（结束）→ **逐文件回 0xF010 ack**
//!   （编辑器本地接入纪律：缺一即卡死；分块序列 = 0xF004 空声明 → 0xF008×N → 0xF00A，收齐才 ack）
//! - EditorStartGame(0xF012) → EditorStartGameRes(0xF018){f1=0, f5=session_id}
//! - EditorPing(0xF011) → 0xF017；0xF01F 编辑器心跳 → 安全忽略；0xF01B 销毁通知 → 停局 teardown
//! - 0xF00C NotifyEditorLog：host 主动推服务端日志到编辑器控制台
//!
//! 接收的上传文件落盘 `<runtime>/User/host_upload/<project>/`（R4 GameHost 加载服务端 lua 用）。

use crate::core::host;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// 控制面共享状态（与 game_host 会话面联动）
pub struct ControlState {
    /// 起局后 = Some((project, session_id, 上传落盘目录))
    pub game: Option<GameInfo>,
    /// 收到 0xF01B / 连接断开 → 停局信号
    pub teardown: bool,
    /// 待推送到编辑器控制台的日志行（0xF00C 通道；容量封顶 PENDING_LOGS_MAX，溢出丢最旧）
    pub pending_logs: Vec<LogEntry>,
    /// 积压超限累计丢弃条数（随下一条入队日志以一条提示带出后清零）
    pub logs_dropped: u64,
    /// 在线控制连接数（引用计数：编辑器 + CLI 多连接并存时互不踩踏，editor_online 由它派生）
    pub editor_conns: u32,
    /// 当前控制连接是否在线（日志推送目标存在性）
    pub editor_online: bool,
}

/// pending_logs 容量上限（编辑器不在线时防内存只增不减）
const PENDING_LOGS_MAX: usize = 2000;

#[derive(Clone)]
pub struct GameInfo {
    pub project: String,
    pub session_id: u64,
    pub upload_dir: PathBuf,
    /// 依赖库表（EditorStartGame f12：(库名, 版本)；R4 lua host 解析 @lib_* 用）
    pub libs: Vec<(String, String)>,
}

pub type ControlRef = Arc<Mutex<ControlState>>;

/// 0xF00C 日志行（pos=代码位置/来源，frame=逻辑帧号——编辑器「调试信息面板」的位置/帧号列）
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub pos: String,
    pub frame: u64,
    pub text: String,
}

pub fn new_state() -> ControlRef {
    Arc::new(Mutex::new(ControlState {
        game: None,
        teardown: false,
        pending_logs: Vec::new(),
        logs_dropped: 0,
        editor_conns: 0,
        editor_online: false,
    }))
}

/// 往编辑器控制台推一条日志（0xF00C NotifyEditorLog）
pub fn push_log(state: &ControlRef, text: &str) {
    push_log_ex(state, "shell-host", 0, text);
}

/// 带位置/帧号的日志（lua 侧日志走这里）
pub fn push_log_ex(state: &ControlRef, pos: &str, frame: u64, text: &str) {
    let mut g = state.lock().unwrap();
    // 容量封顶（参考 pending_broadcast 的丢最旧策略）：编辑器未接入/断开时唯一消费点停摆，
    // 无上限 push 会无限涨内存
    if g.pending_logs.len() >= PENDING_LOGS_MAX {
        g.pending_logs.remove(0);
        g.logs_dropped += 1;
    }
    // 丢弃计数随下一条入队日志带出一条提示（避免静默丢日志）
    if g.logs_dropped > 0 {
        let n = g.logs_dropped;
        g.logs_dropped = 0;
        g.pending_logs.push(LogEntry {
            pos: "shell-host".into(),
            frame,
            text: format!("[host] 日志积压超限，已丢弃 {n} 条"),
        });
    }
    g.pending_logs.push(LogEntry {
        pos: pos.to_string(),
        frame,
        text: text.to_string(),
    });
}

// ---------- 帧读写 ----------

fn read_frame(s: &mut TcpStream) -> Result<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match s.read_exact(&mut len_buf) {
        Ok(_) => {}
        Err(e)
            if e.kind() == std::io::ErrorKind::UnexpectedEof
                || e.kind() == std::io::ErrorKind::ConnectionReset =>
        {
            return Ok(None)
        }
        Err(e) => return Err(anyhow!("读帧头失败: {e}")),
    }
    let total = u32::from_le_bytes(len_buf) as usize;
    if !(6..=64 * 1024 * 1024).contains(&total) {
        return Err(anyhow!("帧长度异常: {total}"));
    }
    let mut frame = len_buf.to_vec();
    frame.resize(total, 0);
    s.read_exact(&mut frame[4..]).map_err(|e| anyhow!("读帧体失败: {e}"))?;
    Ok(Some(frame))
}

fn send(s: &mut TcpStream, msg_type: u64, body: &[u8]) -> Result<()> {
    let f = host::encode_frame(msg_type, body);
    s.write_all(&f).map_err(|e| anyhow!("控制面发送失败: {e}"))
}

/// 0xF00C 日志帧：{f1 时间戳串, f2 level, f3 帧号, f4 位置串, f5 空, f6 内容, f7 项目名, f8 1}
fn send_editor_log(s: &mut TcpStream, project: &str, entry: &LogEntry) {
    let mut body = Vec::new();
    let ts = {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        format!("{:02}:{:02}:{:02}", secs / 3600 % 24, secs / 60 % 60, secs % 60)
    };
    host::put_field_bytes(&mut body, 1, ts.as_bytes());
    host::put_field_varint(&mut body, 2, 1);
    host::put_field_varint(&mut body, 3, entry.frame);
    host::put_field_bytes(&mut body, 4, entry.pos.as_bytes());
    host::put_field_bytes(&mut body, 5, &[]);
    host::put_field_bytes(&mut body, 6, entry.text.as_bytes());
    host::put_field_bytes(&mut body, 7, project.as_bytes());
    host::put_field_varint(&mut body, 8, 1);
    let _ = send(s, host::MSG_NOTIFY_EDITOR_LOG, &body);
}

/// 上传会话的收集中状态（分块文件）
struct UploadCtx {
    project: String,
    dir: PathBuf,
    /// 上传开始时刻（增量判定基准：目标 mtime ≥ 此时刻 = 本轮已写过，直接落盘）
    start: std::time::SystemTime,
    /// 当前分块收集中的文件（0xF004 空声明创建）
    pending: HashMap<String, Vec<u8>>,
}

fn write_upload(ux: &mut UploadCtx, path: &str, content: &[u8]) -> Result<()> {
    // path = "<project>/<相对路径>"（全小写）；防路径穿越
    let rel = path
        .strip_prefix(&format!("{}/", ux.project))
        .unwrap_or(path);
    // 逐分量校验（同 distrib::resolve_file 思路）：拒绝 .. / 盘符 / 根（绝对路径）等一切非常规分量——
    // 仅挡 ".." 不够，Windows 下 join 遇绝对路径参数会丢弃基目录直接逃逸 upload_root
    let rel_path = std::path::Path::new(rel);
    if rel.is_empty()
        || !rel_path
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
    {
        return Err(anyhow!("上传路径穿越: {path}"));
    }
    let abs = ux.dir.join(rel_path);
    if let Some(p) = abs.parent() {
        std::fs::create_dir_all(p)?;
    }
    std::fs::write(&abs, content)?;
    Ok(())
}

/// 增量判定：目标存在且 mtime < 本轮上传开始时刻 = 历史轮已上传过且内容无时间线索引 →
/// 需要逐字节比对才敢说一致（跳过读+比对的成本与直接写相当），故保守直接写。
/// 真正的省时在客户端侧（只传变化文件）。本函数保留 mtime 快判通道：
/// 目标 mtime ≥ start = 本轮重复上传（编辑器可能重发），直接跳过。
fn maybe_skip_upload(ux: &UploadCtx, path: &str) -> bool {
    // 与 write_upload 同款小写前缀 strip（两侧必须一致）
    let rel = path
        .strip_prefix(&format!("{}/", ux.project.to_lowercase()))
        .unwrap_or(path);
    let abs = ux.dir.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
    std::fs::metadata(&abs)
        .and_then(|m| m.modified())
        .map(|mt| mt >= ux.start)
        .unwrap_or(false)
}

/// 单条控制连接的处理（编辑器或本应用 CLI）
fn handle_conn(mut s: TcpStream, state: ControlRef, upload_root: PathBuf) -> Result<()> {
    s.set_nodelay(true)?;
    // 首帧 = EditorLogin
    let frame = read_frame(&mut s)?.ok_or_else(|| anyhow!("首帧前断开"))?;
    let parsed = host::decode_frame(&frame)?;
    if parsed.msg_type != host::MSG_EDITOR_LOGIN {
        return Err(anyhow!("首帧非 EditorLogin: {:#x}", parsed.msg_type));
    }
    let userid = host::body_varint(&parsed.body, 1).unwrap_or(0);
    let token = host::body_string(&parsed.body, 2).unwrap_or_default();
    crate::srv_log!("[host-ctl] EditorLogin: userid={userid} token={token}（本地放行）");
    let mut body = Vec::new();
    host::put_field_varint(&mut body, 1, 0);
    host::put_field_varint(&mut body, 2, 0);
    send(&mut s, host::MSG_EDITOR_LOGIN_RESULT, &body)?;
    {
        let mut g = state.lock().unwrap();
        g.editor_conns += 1;
        g.editor_online = true;
    }

    let mut upload: Option<UploadCtx> = None;
    let mut project = String::new();
    loop {
        // 先把积压日志推出去（锁内仅 drain 取数据、立即释放锁，锁外再做阻塞 socket 写——
        // 编辑器停止读取时 write_all 可无限阻塞，持锁写会冻结整个 game_host 会话面）
        let logs: Vec<LogEntry> = state.lock().unwrap().pending_logs.drain(..).collect();
        for entry in &logs {
            send_editor_log(&mut s, &project, entry);
        }
        let frame = match read_frame(&mut s)? {
            Some(f) => f,
            None => break,
        };
        let Ok(parsed) = host::decode_frame(&frame) else { continue };
        match parsed.msg_type {
            host::MSG_SEND_WRITE_FILE => {
                let path = host::body_string(&parsed.body, 1).unwrap_or_default();
                let proj = host::body_string(&parsed.body, 2).unwrap_or_default();
                if project.is_empty() && !proj.is_empty() {
                    project = proj.clone();
                    upload = Some(UploadCtx {
                        project: proj.clone(),
                        dir: upload_root.join(&proj),
                        start: std::time::SystemTime::now(),
                        pending: HashMap::new(),
                    });
                    // 不再先清空旧目录：write_upload 本就是覆盖写，秒级起步比目录洁癖重要
                    crate::srv_log!("[host-ctl] 上传开始: {proj} → {}", crate::core::disp(&upload_root.join(&proj)));
                }
                let Some(ux) = &mut upload else { continue };
                // f3 内容：无内容 = 大文件空声明（host 据此创建文件）或增量跳过
                let content = {
                    let mut pos = 0;
                    let mut found = None;
                    while pos < parsed.body.len() {
                        let Ok(tag) = host::get_varint(&parsed.body, &mut pos) else { break };
                        let field = (tag >> 3) as u32;
                        match tag & 7 {
                            0 => {
                                let _ = host::get_varint(&parsed.body, &mut pos);
                            }
                            2 => {
                                let Ok(len) = host::get_varint(&parsed.body, &mut pos) else { break };
                                let len = len as usize;
                                // checked_add 防 pos+len 溢出回绕绕过边界检查（len 攻击者可控至 u64::MAX）
                                let Some(end) = pos.checked_add(len).filter(|&e| e <= parsed.body.len()) else { break };
                                if field == 3 && len > 0 {
                                    found = Some(parsed.body[pos..end].to_vec());
                                }
                                pos = end;
                            }
                            5 => pos += 4,
                            1 => pos += 8,
                            _ => break,
                        }
                    }
                    found
                };
                match content {
                    Some(c) => {
                        if !maybe_skip_upload(ux, &path) {
                            write_upload(ux, &path, &c)?;
                        }
                    }
                    None => {
                        // 空声明：登记分块收集
                        ux.pending.insert(path.clone(), Vec::new());
                    }
                }
                // 逐文件 ack（编辑器本地接入纪律：缺一即卡死）
                let mut inner = Vec::new();
                host::put_field_bytes(&mut inner, 1, path.as_bytes());
                host::put_field_bytes(&mut inner, 2, proj.as_bytes());
                let mut ack = Vec::new();
                host::put_field_varint(&mut ack, 1, 0);
                host::put_field_bytes(&mut ack, 2, &inner);
                send(&mut s, host::MSG_WRITE_FILE_ACK, &ack)?;
            }
            host::MSG_SEND_FILE_BLOCK => {
                let path = host::body_string(&parsed.body, 1).unwrap_or_default();
                if let Some(ux) = &mut upload {
                    // f2 = 块内容
                    let mut pos = 0;
                    while pos < parsed.body.len() {
                        let Ok(tag) = host::get_varint(&parsed.body, &mut pos) else { break };
                        let field = (tag >> 3) as u32;
                        match tag & 7 {
                            0 => {
                                let _ = host::get_varint(&parsed.body, &mut pos);
                            }
                            2 => {
                                let Ok(len) = host::get_varint(&parsed.body, &mut pos) else { break };
                                let len = len as usize;
                                // checked_add 防 pos+len 溢出回绕绕过边界检查（同上）
                                let Some(end) = pos.checked_add(len).filter(|&e| e <= parsed.body.len()) else { break };
                                if field == 2 {
                                    ux.pending
                                        .entry(path.clone())
                                        .or_default()
                                        .extend_from_slice(&parsed.body[pos..end]);
                                }
                                pos = end;
                            }
                            5 => pos += 4,
                            1 => pos += 8,
                            _ => break,
                        }
                    }
                }
            }
            host::MSG_FILE_END => {
                let path = host::body_string(&parsed.body, 1).unwrap_or_default();
                if let Some(ux) = &mut upload {
                    if let Some(content) = ux.pending.remove(&path) {
                        write_upload(ux, &path, &content)?;
                    }
                }
            }
            host::MSG_EDITOR_PING => {
                let mut b = Vec::new();
                host::put_field_varint(&mut b, 1, 0);
                send(&mut s, host::MSG_EDITOR_PING_RES, &b)?;
            }
            host::MSG_EDITOR_START_GAME => {
                let proj = host::body_string(&parsed.body, 1).unwrap_or_default();
                // f12 repeated 依赖库 {f1 版本, f2 库名}（R4 lua host 解析 @lib_* 用）
                let libs: Vec<(String, String)> = host::body_msgs(&parsed.body, 12)
                    .iter()
                    .map(|m| {
                        let ver = host::body_string(m, 1).unwrap_or_default();
                        let name = host::body_string(m, 2).unwrap_or_default();
                        (name, ver)
                    })
                    .filter(|(n, _)| !n.is_empty())
                    .collect();
                let session_id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0)
                    | 0x1000_0000_0000_0000; // 高位标记，避免与官方 id 混淆
                crate::srv_log!("[host-ctl] EditorStartGame: {proj} → session {session_id}（依赖库 {} 个）", libs.len());
                {
                    let mut g = state.lock().unwrap();
                    g.game = Some(GameInfo {
                        project: proj.clone(),
                        session_id,
                        upload_dir: upload_root.join(&proj),
                        libs,
                    });
                    g.teardown = false;
                }
                let mut b = Vec::new();
                host::put_field_varint(&mut b, 1, 0);
                host::put_field_varint(&mut b, 5, session_id);
                send(&mut s, host::MSG_EDITOR_START_GAME_RES, &b)?;
            }
            host::MSG_DESTROY_GAME => {
                crate::srv_log!("[host-ctl] 收到销毁通知（0xF01B），停局");
                let mut g = state.lock().unwrap();
                g.game = None;
                g.teardown = true;
            }
            host::MSG_EDITOR_HEARTBEAT => {} // 编辑器心跳：安全忽略
            other => {
                crate::srv_log!("[host-ctl] 未识别消息 {other:#x}（忽略）");
            }
        }
    }
    {
        let mut g = state.lock().unwrap();
        // 引用计数：多控制连接并存时，一条断开不清零仍在连接的另一条
        g.editor_conns = g.editor_conns.saturating_sub(1);
        g.editor_online = g.editor_conns > 0;
        // 控制连接断开 = 编辑器/CLI 离开，按 0xF01B 同等停局（会话面由 game_host 自行保活到客户端断开）
        if g.game.is_some() {
            g.game = None;
            g.teardown = true;
        }
    }
    Ok(())
}

/// 控制面监听 socket 预绑定（game_host 在报 ready 前调用：bind 失败即启动失败，不误报就绪）
/// bind_addr：127.0.0.1 = 仅本机；0.0.0.0 = 局域网/外网（远端客户端连控制面入局）
pub fn bind_listener(port: u16, bind_addr: &str) -> Result<TcpListener> {
    let addr = format!("{bind_addr}:{port}");
    let listener = TcpListener::bind(&addr).map_err(|e| anyhow!("控制面监听失败 {addr}: {e}"))?;
    listener.set_nonblocking(true)?;
    Ok(listener)
}

/// 控制面监听线程体（game_host::run 拉起；listener 由 bind_listener 预绑定）
/// stop 置位后退出（非阻塞 accept 轮询；game_host 停止/重启用）
pub fn run(listener: TcpListener, state: ControlRef, upload_root: PathBuf, stop: Arc<std::sync::atomic::AtomicBool>) -> Result<()> {
    let addr = listener.local_addr().map(|a| a.to_string()).unwrap_or_else(|_| "?".into());
    crate::srv_log!("[host-ctl] 控制面已监听 {addr}");
    let _ = std::fs::create_dir_all(&upload_root);
    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                // Windows 下 accept 的套接字会继承 listener 的 nonblocking 标志，
                // 必须显式还原为阻塞模式，否则 handle_conn 读帧即 10035 WSAEWOULDBLOCK 断连
                if let Err(e) = stream.set_nonblocking(false) {
                    crate::srv_log!("[host-ctl] 还原阻塞模式失败: {e}");
                    continue;
                }
                let state = Arc::clone(&state);
                let root = upload_root.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle_conn(stream, state, root) {
                        crate::srv_log!("[host-ctl] 连接结束: {e}");
                    }
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => crate::srv_log!("[host-ctl] accept 失败: {e}"),
        }
    }
    crate::srv_log!("[host-ctl] 控制面已停止 {addr}");
    Ok(())
}
