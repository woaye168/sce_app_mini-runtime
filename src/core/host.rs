//! 远端调试 host：assign_host（HTTP 签名）+ 控制协议客户端（EditorLogin/上传/EditorStartGame）。
//! 协议格式见 doc/research/scegame-reverse.md §8（Frida 抓包实证）。

use crate::core::auth::UserInfo;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::path::Path;
use std::time::Duration;

// ---------- protobuf wire 手写编码（官方就是手写 wire，无 descriptor） ----------

pub(crate) fn put_varint(buf: &mut Vec<u8>, mut v: u64) {
    loop {
        let b = (v & 0x7F) as u8;
        v >>= 7;
        if v == 0 {
            buf.push(b);
            break;
        }
        buf.push(b | 0x80);
    }
}

pub(crate) fn put_field_varint(buf: &mut Vec<u8>, field: u32, v: u64) {
    put_varint(buf, ((field << 3) as u64) | 0);
    put_varint(buf, v);
}

pub(crate) fn put_field_bytes(buf: &mut Vec<u8>, field: u32, data: &[u8]) {
    put_varint(buf, ((field << 3) as u64) | 2);
    put_varint(buf, data.len() as u64);
    buf.extend_from_slice(data);
}

pub(crate) fn get_varint(data: &[u8], pos: &mut usize) -> Result<u64> {
    let mut v: u64 = 0;
    let mut shift = 0;
    loop {
        if *pos >= data.len() {
            return Err(anyhow!("varint 越界"));
        }
        let b = data[*pos];
        *pos += 1;
        v |= ((b & 0x7F) as u64) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(v)
}

// ---------- 帧编解码 ----------

/// 消息类型（0xF000 段，抓包实证）
pub(crate) const MSG_EDITOR_LOGIN: u64 = 0xF000;
pub(crate) const MSG_EDITOR_LOGIN_RESULT: u64 = 0xF001;
pub(crate) const MSG_SEND_WRITE_FILE: u64 = 0xF004;
pub(crate) const MSG_SEND_FILE_BLOCK: u64 = 0xF008;
pub(crate) const MSG_FILE_END: u64 = 0xF00A;
pub(crate) const MSG_NOTIFY_EDITOR_LOG: u64 = 0xF00C;
pub(crate) const MSG_EDITOR_PING: u64 = 0xF011;
pub(crate) const MSG_EDITOR_START_GAME: u64 = 0xF012;
pub(crate) const MSG_EDITOR_PING_RES: u64 = 0xF017;
pub(crate) const MSG_EDITOR_START_GAME_RES: u64 = 0xF018;
pub(crate) const MSG_UPLOAD_PROGRESS: u64 = 0xF01A;

/// 大文件分块阈值与块长（抓包实证：85KB 的走整发，168KB 的走 101400 分块）
const BLOCK_SIZE: usize = 101400;

/// 组帧：u32 LE 总长（含自身）+ 0x00 + envelope{ f1: header{ f1 type, f2 body } }
pub(crate) fn encode_frame(msg_type: u64, body: &[u8]) -> Vec<u8> {
    let mut header = Vec::new();
    put_field_varint(&mut header, 1, msg_type);
    put_field_bytes(&mut header, 2, body);
    let mut env = Vec::new();
    put_field_bytes(&mut env, 1, &header);
    let total = 4 + 1 + env.len();
    let mut frame = Vec::with_capacity(total);
    frame.extend_from_slice(&(total as u32).to_le_bytes());
    frame.push(0);
    frame.extend_from_slice(&env);
    frame
}

/// 解帧后的消息
#[derive(Debug)]
pub struct Frame {
    pub msg_type: u64,
    pub body: Vec<u8>,
}

pub(crate) fn decode_frame(data: &[u8]) -> Result<Frame> {
    let mut pos = 4; // 跳过 total_len
    let _flag = data[pos];
    pos += 1;
    // envelope f1 wt2
    let tag = get_varint(data, &mut pos)?;
    if tag != 0x0A {
        return Err(anyhow!("envelope tag 异常: {tag:#x}"));
    }
    let elen = get_varint(data, &mut pos)? as usize;
    let env = &data[pos..pos + elen];
    let mut epos = 0;
    let t1 = get_varint(env, &mut epos)?;
    if t1 != 0x08 {
        return Err(anyhow!("header type tag 异常: {t1:#x}"));
    }
    let msg_type = get_varint(env, &mut epos)?;
    let t2 = get_varint(env, &mut epos)?;
    if t2 != 0x12 {
        return Err(anyhow!("header body tag 异常: {t2:#x}"));
    }
    let blen = get_varint(env, &mut epos)? as usize;
    let body = env[epos..epos + blen].to_vec();
    Ok(Frame { msg_type, body })
}

/// body 里读 varint 字段（按字段号过滤）
pub fn body_varint(body: &[u8], want_field: u32) -> Option<u64> {
    let mut pos = 0;
    while pos < body.len() {
        let tag = get_varint(body, &mut pos).ok()?;
        let field = (tag >> 3) as u32;
        match tag & 7 {
            0 => {
                let v = get_varint(body, &mut pos).ok()?;
                if field == want_field {
                    return Some(v);
                }
            }
            2 => {
                let len = get_varint(body, &mut pos).ok()? as usize;
                pos += len;
            }
            5 => pos += 4,
            1 => pos += 8,
            _ => return None,
        }
    }
    None
}

// ---------- assign_host ----------

#[derive(Debug, Clone)]
pub struct HostInfo {
    pub ip: String,
    pub port: u16,
    pub token: String,
}

#[derive(Deserialize)]
struct AssignResp {
    result: i64,
    host_info: Option<AssignHostInfo>,
}

#[derive(Deserialize)]
struct AssignHostInfo {
    ip: String,
    port: String,
    token: String,
}

/// 申请远端调试 host（map_starter.lua query_assign_host 复现：POST http://<域>:9007/api/v1/assign_host）
pub fn assign_host(info: &UserInfo, env_domain: &str, api_version: u32) -> Result<HostInfo> {
    let headers = crate::core::verify::sign_headers(info)?;
    let url = format!("http://{env_domain}:9007/api/v1/assign_host");
    let mut builder = reqwest::blocking::Client::builder().timeout(Duration::from_secs(20));
    if let Some(p) = crate::core::verify::proxy() {
        builder = builder.proxy(p);
    }
    let client = builder.build()?;
    let mut req = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(format!("{{\"api_version\":{api_version}}}"));
    for (k, v) in headers {
        req = req.header(k, v);
    }
    let resp = req.send().map_err(|e| anyhow!("assign_host 请求失败: {e}"))?;
    let text = resp.text()?;
    let parsed: AssignResp =
        serde_json::from_str(&text).map_err(|e| anyhow!("assign_host 响应解析失败: {e}（{text}）"))?;
    if parsed.result != 0 {
        return Err(anyhow!("assign_host 返回 result={}（{text}）", parsed.result));
    }
    let hi = parsed
        .host_info
        .ok_or_else(|| anyhow!("assign_host 无 host_info（服务器拥挤？）（{text}）"))?;
    Ok(HostInfo {
        ip: hi.ip,
        port: hi.port.parse().map_err(|_| anyhow!("host port 异常: {}", hi.port))?,
        token: hi.token,
    })
}

// ---------- 控制连接 ----------

pub struct HostControl {
    stream: TcpStream,
    /// 读缓冲（跨 recv 组帧）
    rbuf: Vec<u8>,
    /// 起局结果
    pub session_id: Option<u64>,
    /// host 推过来的服务端日志（NotifyEditorLog 的 f6 内容）
    pub host_logs: Vec<String>,
}

impl HostControl {
    pub fn connect(host: &HostInfo, userid: i64) -> Result<Self> {
        let addr = format!("{}:{}", host.ip, host.port);
        let stream = TcpStream::connect(&addr)
            .map_err(|e| anyhow!("控制连接失败 {addr}: {e}"))?;
        stream.set_read_timeout(Some(Duration::from_millis(200)))?;
        stream.set_nodelay(true)?;
        let mut ctl = Self {
            stream,
            rbuf: Vec::new(),
            session_id: None,
            host_logs: Vec::new(),
        };
        // EditorLogin { f1 userid, f2 host_token }
        let mut body = Vec::new();
        put_field_varint(&mut body, 1, userid as u64);
        put_field_bytes(&mut body, 2, host.token.as_bytes());
        ctl.send_frame(MSG_EDITOR_LOGIN, &body)?;
        // 等 EditorLoginResult
        let frame = ctl.wait_for(MSG_EDITOR_LOGIN_RESULT, Duration::from_secs(15))?;
        let result = body_varint(&frame.body, 1).unwrap_or(u64::MAX);
        if result != 0 {
            return Err(anyhow!("EditorLogin 被拒绝: result={result}"));
        }
        Ok(ctl)
    }

    fn send_frame(&mut self, msg_type: u64, body: &[u8]) -> Result<()> {
        let frame = encode_frame(msg_type, body);
        self.stream
            .write_all(&frame)
            .map_err(|e| anyhow!("控制连接发送失败: {e}"))
    }

    /// 发心跳（可选）
    pub fn ping(&mut self, seq: u64) -> Result<()> {
        let mut body = Vec::new();
        put_field_varint(&mut body, 2, seq);
        self.send_frame(MSG_EDITOR_PING, &body)
    }

    /// 上传项目文件（全量带内容；相对路径正斜杠、带 <project>/ 前缀）
    pub fn upload_project(&mut self, staging: &Path, project: &str) -> Result<u32> {
        let mut files = Vec::new();
        collect_files(staging, staging, &mut files)?;
        files.sort();
        let mut count = 0u32;
        for rel in &files {
            let abs = staging.join(rel);
            let content = std::fs::read(&abs)
                .map_err(|e| anyhow!("读项目文件失败 {}: {e}", abs.display()))?;
            let rel_unix = rel.replace('\\', "/");
            // host 是 Linux（/work/bin/...）大小写敏感；官方上传路径一律小写（抓包实证：
            // 盘上 ToSceClass.lua → 上传 tosceclass.lua）
            let path = format!("{project}/{rel_unix}").to_lowercase();
            if content.len() <= BLOCK_SIZE {
                // SendWriteFile 整发 { f1 path, f2 project, f3 content }
                let mut body = Vec::new();
                put_field_bytes(&mut body, 1, path.as_bytes());
                put_field_bytes(&mut body, 2, project.as_bytes());
                put_field_bytes(&mut body, 3, &content);
                self.send_frame(MSG_SEND_WRITE_FILE, &body)?;
            } else {
                // 大文件：先 0xF004 声明（f1 path, f2 project，无 f3——host 据此创建文件），
                // 再 0xF008 分块追加 { f1 path, f2 chunk, f3 project }（抓包实证序列）
                let mut decl = Vec::new();
                put_field_bytes(&mut decl, 1, path.as_bytes());
                put_field_bytes(&mut decl, 2, project.as_bytes());
                self.send_frame(MSG_SEND_WRITE_FILE, &decl)?;
                for chunk in content.chunks(BLOCK_SIZE) {
                    let mut body = Vec::new();
                    put_field_bytes(&mut body, 1, path.as_bytes());
                    put_field_bytes(&mut body, 2, chunk);
                    put_field_bytes(&mut body, 3, project.as_bytes());
                    self.send_frame(MSG_SEND_FILE_BLOCK, &body)?;
                }
            }
            // FileEnd { f1 path, f2 project }
            let mut body = Vec::new();
            put_field_bytes(&mut body, 1, path.as_bytes());
            put_field_bytes(&mut body, 2, project.as_bytes());
            self.send_frame(MSG_FILE_END, &body)?;
            count += 1;
            // 每 20 个文件抽干一次接收缓冲（防 TCP 窗口堵死）
            if count % 20 == 0 {
                self.drain();
            }
        }
        Ok(count)
    }

    /// EditorStartGame：f1/f2 项目名、f5=0、f10 空、f11 api_version、f12 依赖库版本表
    pub fn start_game(&mut self, project: &str, api_version: u32, libs: &[(String, String)]) -> Result<()> {
        let mut body = Vec::new();
        put_field_bytes(&mut body, 1, project.as_bytes());
        put_field_bytes(&mut body, 2, project.as_bytes());
        put_field_varint(&mut body, 5, 0);
        put_field_bytes(&mut body, 10, &[]);
        put_field_bytes(&mut body, 11, api_version.to_string().as_bytes());
        for (name, version) in libs {
            let mut entry = Vec::new();
            put_field_bytes(&mut entry, 1, version.as_bytes());
            put_field_bytes(&mut entry, 2, name.as_bytes());
            put_field_bytes(&mut body, 12, &entry);
        }
        self.send_frame(MSG_EDITOR_START_GAME, &body)
    }

    /// 等 EditorStartGameRes，拿 session_id
    pub fn wait_start_game_res(&mut self, timeout: Duration) -> Result<u64> {
        let frame = self.wait_for(MSG_EDITOR_START_GAME_RES, timeout)?;
        let result = body_varint(&frame.body, 1).unwrap_or(u64::MAX);
        if result != 0 {
            return Err(anyhow!("EditorStartGame 失败: result={result}"));
        }
        let session = body_varint(&frame.body, 5)
            .ok_or_else(|| anyhow!("EditorStartGameRes 缺 session_id"))?;
        self.session_id = Some(session);
        Ok(session)
    }

    /// 抽干接收缓冲：处理 ack/进度/日志（非阻塞聚合）
    pub fn drain(&mut self) {
        let mut tmp = [0u8; 65536];
        loop {
            match self.stream.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    self.rbuf.extend_from_slice(&tmp[..n]);
                    self.process_rbuf();
                }
                Err(_) => break,
            }
        }
    }

    fn process_rbuf(&mut self) {
        loop {
            if self.rbuf.len() < 5 {
                return;
            }
            let total = u32::from_le_bytes([self.rbuf[0], self.rbuf[1], self.rbuf[2], self.rbuf[3]]) as usize;
            if total < 6 || total > 64 * 1024 * 1024 {
                // 失步：丢弃 1 字节重对齐（不应发生）
                self.rbuf.remove(0);
                continue;
            }
            if self.rbuf.len() < total {
                return;
            }
            let frame_data: Vec<u8> = self.rbuf[..total].to_vec();
            self.rbuf.drain(..total);
            if let Ok(frame) = decode_frame(&frame_data) {
                match frame.msg_type {
                    MSG_NOTIFY_EDITOR_LOG => {
                        // f6 = 日志内容
                        if let Some(text) = body_string(&frame.body, 6) {
                            self.host_logs.push(text);
                        }
                    }
                    MSG_UPLOAD_PROGRESS | MSG_EDITOR_PING_RES => {}
                    _ => {}
                }
            }
        }
    }

    /// 等指定类型的帧（期间聚合其余帧）
    fn wait_for(&mut self, want_type: u64, timeout: Duration) -> Result<Frame> {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            // 先在已聚合缓冲里找
            if let Some(frame) = self.take_from_rbuf(want_type)? {
                return Ok(frame);
            }
            if std::time::Instant::now() > deadline {
                return Err(anyhow!("等待 {want_type:#x} 超时"));
            }
            let mut tmp = [0u8; 65536];
            match self.stream.read(&mut tmp) {
                Ok(0) => return Err(anyhow!("控制连接被关闭")),
                Ok(n) => self.rbuf.extend_from_slice(&tmp[..n]),
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock
                        && e.kind() != std::io::ErrorKind::TimedOut
                    {
                        return Err(anyhow!("控制连接读失败: {e}"));
                    }
                }
            }
        }
    }

    /// 从 rbuf 里切出第一个 want_type 帧（其余按类型聚合/丢弃）
    fn take_from_rbuf(&mut self, want_type: u64) -> Result<Option<Frame>> {
        let mut consumed = 0usize;
        while consumed + 5 <= self.rbuf.len() {
            let total = u32::from_le_bytes([
                self.rbuf[consumed],
                self.rbuf[consumed + 1],
                self.rbuf[consumed + 2],
                self.rbuf[consumed + 3],
            ]) as usize;
            if total < 6 || total > 64 * 1024 * 1024 {
                return Err(anyhow!("帧失步"));
            }
            if consumed + total > self.rbuf.len() {
                break; // 帧不完整
            }
            let frame_data: Vec<u8> = self.rbuf[consumed..consumed + total].to_vec();
            consumed += total;
            let frame = decode_frame(&frame_data)?;
            if frame.msg_type == want_type {
                // 把剩余未消费部分留回 rbuf——需要重组：已消费段前的都处理过
                let rest = self.rbuf[consumed..].to_vec();
                self.rbuf = rest;
                return Ok(Some(frame));
            } else if frame.msg_type == MSG_NOTIFY_EDITOR_LOG {
                if let Some(text) = body_string(&frame.body, 6) {
                    self.host_logs.push(text);
                }
            }
        }
        // 保留未完整部分
        let rest = self.rbuf[consumed..].to_vec();
        self.rbuf = rest;
        Ok(None)
    }

    pub fn shutdown(&self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }

    /// 拆出底层 TcpStream（local_host 中继模式转 raw 双向转发用；清掉读超时转阻塞）
    pub(crate) fn into_stream(self) -> TcpStream {
        let _ = self.stream.set_read_timeout(None);
        self.stream
    }
}

/// body 里读字符串字段
pub(crate) fn body_string(body: &[u8], want_field: u32) -> Option<String> {
    let mut pos = 0;
    while pos < body.len() {
        let tag = get_varint(body, &mut pos).ok()?;
        let field = (tag >> 3) as u32;
        match tag & 7 {
            0 => {
                let _ = get_varint(body, &mut pos).ok()?;
            }
            2 => {
                let len = get_varint(body, &mut pos).ok()? as usize;
                if pos + len > body.len() {
                    return None;
                }
                if field == want_field {
                    return Some(String::from_utf8_lossy(&body[pos..pos + len]).to_string());
                }
                pos += len;
            }
            5 => pos += 4,
            1 => pos += 8,
            _ => return None,
        }
    }
    None
}

/// 递归收集相对路径（跳过目录，纯文件列表）
fn collect_files(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, out)?;
        } else {
            let rel = path
                .strip_prefix(root)
                .map_err(|_| anyhow!("路径前缀异常"))?
                .to_string_lossy()
                .to_string();
            out.push(rel);
        }
    }
    Ok(())
}
