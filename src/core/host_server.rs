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
    /// 待推送到编辑器控制台的日志行（0xF00C 通道）
    pub pending_logs: Vec<String>,
    /// 当前控制连接是否在线（日志推送目标存在性）
    pub editor_online: bool,
}

#[derive(Clone)]
pub struct GameInfo {
    pub project: String,
    pub session_id: u64,
    pub upload_dir: PathBuf,
}

pub type ControlRef = Arc<Mutex<ControlState>>;

pub fn new_state() -> ControlRef {
    Arc::new(Mutex::new(ControlState {
        game: None,
        teardown: false,
        pending_logs: Vec::new(),
        editor_online: false,
    }))
}

/// 往编辑器控制台推一条日志（0xF00C NotifyEditorLog）
pub fn push_log(state: &ControlRef, text: &str) {
    state.lock().unwrap().pending_logs.push(text.to_string());
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

/// 0xF00C 日志帧：{f1 时间戳串, f2 level, f3 0, f4 位置串, f5 空, f6 内容, f7 项目名, f8 1}
fn send_editor_log(s: &mut TcpStream, project: &str, text: &str) {
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
    host::put_field_varint(&mut body, 3, 0);
    host::put_field_bytes(&mut body, 4, b"shell-host");
    host::put_field_bytes(&mut body, 5, &[]);
    host::put_field_bytes(&mut body, 6, text.as_bytes());
    host::put_field_bytes(&mut body, 7, project.as_bytes());
    host::put_field_varint(&mut body, 8, 1);
    let _ = send(s, host::MSG_NOTIFY_EDITOR_LOG, &body);
}

/// 上传会话的收集中状态（分块文件）
struct UploadCtx {
    project: String,
    dir: PathBuf,
    /// 当前分块收集中的文件（0xF004 空声明创建）
    pending: HashMap<String, Vec<u8>>,
}

fn write_upload(ux: &mut UploadCtx, path: &str, content: &[u8]) -> Result<()> {
    // path = "<project>/<相对路径>"（全小写）；防路径穿越
    let rel = path
        .strip_prefix(&format!("{}/", ux.project))
        .unwrap_or(path);
    if rel.contains("..") {
        return Err(anyhow!("上传路径穿越: {path}"));
    }
    let abs = ux.dir.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
    if let Some(p) = abs.parent() {
        std::fs::create_dir_all(p)?;
    }
    std::fs::write(&abs, content)?;
    Ok(())
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
    println!("[host-ctl] EditorLogin: userid={userid} token={token}（本地放行）");
    let mut body = Vec::new();
    host::put_field_varint(&mut body, 1, 0);
    host::put_field_varint(&mut body, 2, 0);
    send(&mut s, host::MSG_EDITOR_LOGIN_RESULT, &body)?;
    state.lock().unwrap().editor_online = true;

    let mut upload: Option<UploadCtx> = None;
    let mut project = String::new();
    loop {
        // 先把积压日志推出去
        {
            let mut g = state.lock().unwrap();
            for text in g.pending_logs.drain(..) {
                send_editor_log(&mut s, &project, &text);
            }
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
                        pending: HashMap::new(),
                    });
                    let _ = std::fs::remove_dir_all(upload_root.join(&proj));
                    println!("[host-ctl] 上传开始: {proj} → {}", crate::core::disp(&upload_root.join(&proj)));
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
                                if pos + len > parsed.body.len() {
                                    break;
                                }
                                if field == 3 && len > 0 {
                                    found = Some(parsed.body[pos..pos + len].to_vec());
                                }
                                pos += len;
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
                        write_upload(ux, &path, &c)?;
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
                                if pos + len > parsed.body.len() {
                                    break;
                                }
                                if field == 2 {
                                    ux.pending
                                        .entry(path.clone())
                                        .or_default()
                                        .extend_from_slice(&parsed.body[pos..pos + len]);
                                }
                                pos += len;
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
                let session_id = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0)
                    | 0x1000_0000_0000_0000; // 高位标记，避免与官方 id 混淆
                println!("[host-ctl] EditorStartGame: {proj} → session {session_id}");
                {
                    let mut g = state.lock().unwrap();
                    g.game = Some(GameInfo {
                        project: proj.clone(),
                        session_id,
                        upload_dir: upload_root.join(&proj),
                    });
                    g.teardown = false;
                }
                let mut b = Vec::new();
                host::put_field_varint(&mut b, 1, 0);
                host::put_field_varint(&mut b, 5, session_id);
                send(&mut s, host::MSG_EDITOR_START_GAME_RES, &b)?;
            }
            host::MSG_DESTROY_GAME => {
                println!("[host-ctl] 收到销毁通知（0xF01B），停局");
                let mut g = state.lock().unwrap();
                g.game = None;
                g.teardown = true;
            }
            host::MSG_EDITOR_HEARTBEAT => {} // 编辑器心跳：安全忽略
            other => {
                println!("[host-ctl] 未识别消息 {other:#x}（忽略）");
            }
        }
    }
    state.lock().unwrap().editor_online = false;
    // 控制连接断开 = 编辑器/CLI 离开，按 0xF01B 同等停局（会话面由 game_host 自行保活到客户端断开）
    Ok(())
}

/// 控制面监听线程体（game_host::run 拉起）
pub fn run(port: u16, state: ControlRef, upload_root: PathBuf) -> Result<()> {
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).map_err(|e| anyhow!("控制面监听失败 {addr}: {e}"))?;
    println!("[host-ctl] 控制面已监听 {addr}");
    let _ = std::fs::create_dir_all(&upload_root);
    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                let state = Arc::clone(&state);
                let root = upload_root.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle_conn(stream, state, root) {
                        println!("[host-ctl] 连接结束: {e}");
                    }
                });
            }
            Err(e) => println!("[host-ctl] accept 失败: {e}"),
        }
    }
    Ok(())
}
