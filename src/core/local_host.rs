//! 自建 host（中继模式）：本地门面 + 云端芯。
//!
//! - TCP 127.0.0.1:<port>（默认 5003）实现编辑器调试 host 控制协议（0xF000 段）：
//!   编辑器「调试(本地服务器)」（use_local_host）或本应用 debug start --host local 直接接入；
//!   EditorLogin 拦截换真 token，其余帧双向原样转发到 assign_host 分配的云端 host。
//! - UDP 在 <port> 与 <port+50> 双端口做 KCP 会话 NAT 转发（按客户端源地址建独立上行 socket，
//!   不解析 KCP 内容）。**KCP 会话端口 = 控制端口 + 50**（2026-09-02 客户端 Network 落实锤：
//!   -host_port=5003 时引擎实际 dial 127.0.0.1:5053；云端 13738→13788 同规律）。
//! - 全部流量（TCP 帧 + UDP 包）落 jsonl capture——兼作 KCP 会话协议逆向的抓包平台。
//!
//! 背景与证据见 doc/research/self-host.md。

use crate::core::auth::UserInfo;
use crate::core::host::{self, HostControl, HostInfo};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// 中继 host 参数
pub struct LocalHostParams {
    /// 监听端口（TCP+UDP 同号，默认 5003 = 官方 use_local_host 固定端口）
    pub port: u16,
    /// 云端 assign_host 用的凭证
    pub cred: UserInfo,
    /// 环境域（默认 editor-pd.spark.xd.com）
    pub env_domain: String,
    /// 项目 api_version（assign_host 必带）
    pub api_version: u32,
    /// 流量 capture 输出（jsonl）；None = 只 stdout 摘要
    pub capture_path: Option<PathBuf>,
}

/// 云端目标（assign 后对 UDP 转发可见）
struct Shared {
    cloud: Option<HostInfo>,
    capture: Option<std::io::BufWriter<std::fs::File>>,
    started: Instant,
}

type SharedRef = Arc<Mutex<Shared>>;

fn now_ms(started: Instant) -> u128 {
    started.elapsed().as_millis()
}

fn wall_ts() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

fn hex(data: &[u8], cap: usize) -> String {
    let n = data.len().min(cap);
    let mut s = String::with_capacity(n * 2);
    for b in &data[..n] {
        s.push_str(&format!("{b:02x}"));
    }
    if data.len() > cap {
        s.push_str(&format!("..(+{}B)", data.len() - cap));
    }
    s
}

fn capture(shared: &SharedRef, chan: &str, msg: Option<u64>, data: &[u8], note: &str) {
    let mut g = shared.lock().unwrap();
    let ts = now_ms(g.started);
    if let Some(w) = &mut g.capture {
        let msg_field = msg
            .map(|m| format!(",\"msg\":\"{m:#x}\""))
            .unwrap_or_default();
        let line = format!(
            "{{\"ts\":{},\"wall\":{},\"chan\":\"{chan}\"{},\"len\":{},\"hex\":\"{}\"{}}}\n",
            ts,
            wall_ts(),
            msg_field,
            data.len(),
            hex(data, 2048),
            if note.is_empty() {
                String::new()
            } else {
                format!(",\"note\":\"{}\"", note.replace('"', "'"))
            }
        );
        let _ = w.write_all(line.as_bytes());
        let _ = w.flush();
    }
}

fn log_line(msg: &str) {
    println!("[local-host] {msg}");
}

/// 从流里读一帧（u32 LE 总长含自身 + 载荷）；Ok(None) = 对端干净关闭
fn read_frame(stream: &mut TcpStream) -> Result<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match stream.read_exact(&mut len_buf) {
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
    stream
        .read_exact(&mut frame[4..])
        .map_err(|e| anyhow!("读帧体失败: {e}"))?;
    Ok(Some(frame))
}

/// 编辑器侧一条控制连接的处理：登录拦截换 token，之后双向帧级转发
fn handle_control_conn(mut downstream: TcpStream, shared: SharedRef, params: Arc<LocalHostParams>) {
    let peer = downstream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_default();
    log_line(&format!("控制台接入: {peer}"));
    if let Err(e) = control_conn_inner(&mut downstream, &shared, &params) {
        log_line(&format!("控制台 {peer} 结束: {e}"));
    }
    capture(&shared, "tcp", None, &[], &format!("conn closed {peer}"));
}

fn control_conn_inner(
    downstream: &mut TcpStream,
    shared: &SharedRef,
    params: &Arc<LocalHostParams>,
) -> Result<()> {
    // ① 等编辑器的 EditorLogin
    let frame = read_frame(downstream)?.ok_or_else(|| anyhow!("首帧前断开"))?;
    let parsed = host::decode_frame(&frame)?;
    if parsed.msg_type != host::MSG_EDITOR_LOGIN {
        return Err(anyhow!("首帧非 EditorLogin: {:#x}", parsed.msg_type));
    }
    let userid = host::body_varint(&parsed.body, 1).unwrap_or(0) as i64;
    let client_token = host::body_string(&parsed.body, 2).unwrap_or_default();
    log_line(&format!("EditorLogin: userid={userid} token={client_token}（本地放行）"));
    capture(shared, "tcp.d2u", Some(parsed.msg_type), &parsed.body, "EditorLogin");

    // ② 惰性 assign_host + 云端控制连接（真 token）；失败必须回 0xF001 非 0——
    //    否则编辑器 update_host 的 co.call 永远悬挂，调试管线卡死（2026-09-02 实测踩坑）
    let login_fail = |downstream: &mut TcpStream, why: &str| {
        let mut body = Vec::new();
        host::put_field_varint(&mut body, 1, 1);
        host::put_field_varint(&mut body, 2, 0);
        let resp = host::encode_frame(host::MSG_EDITOR_LOGIN_RESULT, &body);
        let _ = downstream.write_all(&resp);
        anyhow!("{why}")
    };
    let cloud = match host::assign_host(&params.cred, &params.env_domain, params.api_version) {
        Ok(h) => h,
        Err(e) => return Err(login_fail(downstream, &format!("assign_host 失败: {e}"))),
    };
    log_line(&format!(
        "assign_host: {}:{} token={}...",
        cloud.ip,
        cloud.port,
        cloud.token.chars().take(8).collect::<String>()
    ));
    let upstream = match HostControl::connect(&cloud, userid) {
        Ok(c) => c.into_stream(),
        Err(e) => return Err(login_fail(downstream, &format!("云端控制连接失败: {e}"))),
    };
    log_line("云端 EditorLogin 成功（真 token）");
    // 云端地址对 UDP 转发可见
    {
        let mut g = shared.lock().unwrap();
        g.cloud = Some(cloud.clone());
    }
    // ③ 回编辑器登录成功（官方 0xF001：f1 result + f2 varint）
    let mut body = Vec::new();
    host::put_field_varint(&mut body, 1, 0);
    host::put_field_varint(&mut body, 2, 0);
    let resp = host::encode_frame(host::MSG_EDITOR_LOGIN_RESULT, &body);
    downstream
        .write_all(&resp)
        .map_err(|e| anyhow!("回 EditorLoginResult 失败: {e}"))?;

    // ④ 双向帧级转发（上行 u2d 线程 + 本线程 d2u）
    let mut up_read = upstream.try_clone()?;
    let mut up_write = upstream;
    let mut down_clone = downstream.try_clone()?;
    let shared2 = Arc::clone(shared);
    let u2d = std::thread::spawn(move || {
        loop {
            match read_frame(&mut up_read) {
                Ok(Some(frame)) => {
                    let ty = host::decode_frame(&frame).map(|f| f.msg_type).unwrap_or(0);
                    capture(&shared2, "tcp.u2d", Some(ty), &frame, "");
                    if ty == host::MSG_NOTIFY_EDITOR_LOG {
                        if let Ok(f) = host::decode_frame(&frame) {
                            if let Some(text) = host::body_string(&f.body, 6) {
                                println!("[host-log] {text}");
                            }
                        }
                    }
                    if down_clone.write_all(&frame).is_err() {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }
    });
    loop {
        match read_frame(downstream)? {
            Some(frame) => {
                let ty = host::decode_frame(&frame).map(|f| f.msg_type).unwrap_or(0);
                capture(shared, "tcp.d2u", Some(ty), &frame, "");
                up_write
                    .write_all(&frame)
                    .map_err(|e| anyhow!("转发上行失败: {e}"))?;
            }
            None => break,
        }
    }
    let _ = u2d.join();
    Ok(())
}

/// UDP KCP NAT 转发：每客户端源地址一条上行 socket；port_offset = 相对云端控制端口的偏移（0 / +50）
fn udp_relay(down: UdpSocket, shared: SharedRef, port_offset: u16) -> Result<()> {
    let down = Arc::new(down);
    let mut upstreams: HashMap<SocketAddr, Arc<UdpSocket>> = HashMap::new();
    let mut buf = [0u8; 65536];
    loop {
        let (n, client) = match down.recv_from(&mut buf) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("UDP 收包失败: {e}")),
        };
        let cloud = shared.lock().unwrap().cloud.clone();
        let Some(cloud) = cloud else {
            log_line("UDP 包到达但云端未 assign（丢弃）");
            continue;
        };
        let cloud_port = cloud.port + port_offset;
        let up = match upstreams.get(&client) {
            Some(s) => Arc::clone(s),
            None => {
                let sock = UdpSocket::bind("0.0.0.0:0")?;
                sock.connect(format!("{}:{}", cloud.ip, cloud_port))?;
                let sock = Arc::new(sock);
                // 上行收包线程：云 → 客户端
                let up_clone = Arc::clone(&sock);
                let down_clone = Arc::clone(&down);
                let shared2 = Arc::clone(&shared);
                std::thread::spawn(move || {
                    let mut rbuf = [0u8; 65536];
                    loop {
                        match up_clone.recv(&mut rbuf) {
                            Ok(m) => {
                                capture(&shared2, "udp.h2c", None, &rbuf[..m], "");
                                let _ = down_clone.send_to(&rbuf[..m], client);
                            }
                            Err(_) => break,
                        }
                    }
                });
                log_line(&format!("KCP 客户端接入: {client} → {}:{}", cloud.ip, cloud_port));
                upstreams.insert(client, Arc::clone(&sock));
                sock
            }
        };
        capture(&shared, "udp.c2h", None, &buf[..n], "");
        up.send(&buf[..n])?;
    }
}

/// 前台阻塞跑中继 host（CLI host start / debug start --host local 的线程体）
pub fn run(params: LocalHostParams, ready_tx: Option<std::sync::mpsc::Sender<Result<u16, String>>>) -> Result<()> {
    let addr = format!("127.0.0.1:{}", params.port);
    let tcp = TcpListener::bind(&addr).map_err(|e| anyhow!("TCP 监听失败 {addr}: {e}"))?;
    // KCP 会话端口 = 控制端口 + 50（引擎硬编码规律，见文件头注释）；UDP 双端口监听
    let kcp_addr = format!("127.0.0.1:{}", params.port + 50);
    let udp = UdpSocket::bind(&addr).map_err(|e| anyhow!("UDP 绑定失败 {addr}: {e}"))?;
    let udp_kcp = UdpSocket::bind(&kcp_addr).map_err(|e| anyhow!("UDP 绑定失败 {kcp_addr}: {e}"))?;
    let capture = match &params.capture_path {
        Some(p) => {
            let f = std::fs::File::create(p)
                .map_err(|e| anyhow!("capture 文件创建失败 {}: {e}", crate::core::disp(p)))?;
            Some(std::io::BufWriter::new(f))
        }
        None => None,
    };
    let shared: SharedRef = Arc::new(Mutex::new(Shared {
        cloud: None,
        capture,
        started: Instant::now(),
    }));
    log_line(&format!(
        "自建 host（中继）已监听 {addr}（TCP 控制）+ {addr}/{kcp_addr}（UDP KCP），capture={}",
        params
            .capture_path
            .as_ref()
            .map(|p| crate::core::disp(p))
            .unwrap_or_else(|| "（关）".into())
    ));
    log_line("等待接入：编辑器「调试(本地服务器)」或 debug start --host local");
    if let Some(tx) = ready_tx {
        let _ = tx.send(Ok(params.port));
    }
    let params = Arc::new(params);
    // UDP 转发线程（控制端口 + KCP 会话端口+50）
    for (sock, offset) in [(udp, 0u16), (udp_kcp, 50u16)] {
        let shared = Arc::clone(&shared);
        std::thread::spawn(move || {
            if let Err(e) = udp_relay(sock, shared, offset) {
                log_line(&format!("UDP 转发(+{offset}) 结束: {e}"));
            }
        });
    }
    // TCP accept 循环
    for conn in tcp.incoming() {
        match conn {
            Ok(stream) => {
                let _ = stream.set_nodelay(true);
                let shared = Arc::clone(&shared);
                let params = Arc::clone(&params);
                std::thread::spawn(move || handle_control_conn(stream, shared, params));
            }
            Err(e) => log_line(&format!("accept 失败: {e}")),
        }
    }
    Ok(())
}

/// 幂等确保中继在跑（GUI/CLI debug 复用）：已在跑则直接返回端口
static RUNNING: Mutex<Option<u16>> = Mutex::new(None);

pub fn ensure_running(params: LocalHostParams) -> Result<u16> {
    let mut g = RUNNING.lock().unwrap();
    if let Some(port) = *g {
        return Ok(port);
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        if let Err(e) = run(params, Some(tx)) {
            log_line(&format!("中继 host 退出: {e}"));
        }
    });
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(Ok(p)) => {
            *g = Some(p);
            Ok(p)
        }
        Ok(Err(e)) => Err(anyhow!("中继 host 启动失败: {e}")),
        Err(_) => Err(anyhow!("中继 host 启动超时")),
    }
}
