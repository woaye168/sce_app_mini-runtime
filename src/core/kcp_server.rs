//! KCP 会话面服务端（自研 host 的游戏会话层）。
//!
//! 协议（doc/research/scegame-reverse.md §13.1-13.3 实证）：
//! - CE1 握手族（ASCII 魔法）：c→ CE1SYN(13B: magic + 00000163b80305 固定尾) →
//!   h→ CE1SYACK(magic8 + conv(4 LE) + 4B 任意) → c→ CE1ACK(6B) →
//!   h→ CE1SYNACK(magic9 + 000000 + conv(4 LE)，重发至首个 KCP PUSH)
//! - 控制魔法：CE1DISCONNECT / CE1TIMEOUT / CE1REP（本实现只被动处理 DISCONNECT/TIMEOUT）
//! - KCP 段：conv(4) cmd(1) frg(1) wnd(2) ts(4) sn(4) una(4) len(4) 全 LE；
//!   cmd 0x51=PUSH 0x52=ACK（ACK 的 ts 回显被 ACK 段的 ts）；**一个数据报可含多个段（逐段迭代）**
//! - 流式分帧（-kcp_stream）：PUSH payload 拼接为字节流，消息 = 3B LE 长度（含自身）+ 消息体
//!
//! 面向 localhost 的简化：立即 ACK、RTO 重传兜底、无拥塞控制；单会话（不做多房间）。

use std::collections::BTreeMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

const MAGIC_SYN: &[u8] = b"CE1SYN";
const MAGIC_ACK: &[u8] = b"CE1ACK";
const MAGIC_SYACK: &[u8] = b"CE1SYACK";
const MAGIC_SYNACK: &[u8] = b"CE1SYNACK";
const CMD_PUSH: u8 = 0x51;
const CMD_ACK: u8 = 0x52;
/// 通告窗口（与官方一致 512）
const WND: u16 = 512;
/// 单段 PUSH 最大载荷
const SEG_PAYLOAD_MAX: usize = 1000;
/// SYNACK 重发间隔
const SYNACK_RETX: Duration = Duration::from_millis(150);
/// PUSH 重传 RTO（localhost 足够宽松）
const RTO: Duration = Duration::from_millis(200);
/// 空闲超时（之后视为断开；官方发 CE1TIMEOUT，我们直接清理）
const IDLE_TIMEOUT: Duration = Duration::from_secs(120);

fn now_ms() -> u32 {
    // KCP ts 只需单调；从进程启动起算的 ms 即可
    static T0: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    T0.get_or_init(Instant::now).elapsed().as_millis() as u32
}

/// 会话事件
pub enum Event {
    /// 新会话建立（CE1 握手完成）
    NewSession { conv: u32 },
    /// 收到一条完整应用消息（3B 帧体，已按序重组）
    Message { conv: u32, body: Vec<u8> },
    /// 会话结束（对端 CE1DISCONNECT/TIMEOUT 或空闲超时）
    Closed { conv: u32 },
}

struct Unacked {
    sn: u32,
    data: Vec<u8>,
    last_sent: Instant,
}

struct Session {
    conv: u32,
    addr: SocketAddr,
    /// CE1 握手是否完成（等 CE1ACK 中 / 已建立）
    established: bool,
    last_synack: Instant,
    first_push_seen: bool,
    // 接收侧
    rcv_nxt: u32,
    rcv_buf: BTreeMap<u32, (u32, Vec<u8>)>, // sn → (ts, payload)
    stream: Vec<u8>,
    // 发送侧
    snd_nxt: u32,
    unacked: Vec<Unacked>,
    snd_stream: Vec<u8>, // 待发送流字节
    last_recv: Instant,
}

impl Session {
    fn new(conv: u32, addr: SocketAddr) -> Self {
        Self {
            conv,
            addr,
            established: false,
            last_synack: Instant::now() - SYNACK_RETX,
            first_push_seen: false,
            rcv_nxt: 0,
            rcv_buf: BTreeMap::new(),
            stream: Vec::new(),
            snd_nxt: 0,
            unacked: Vec::new(),
            snd_stream: Vec::new(),
            last_recv: Instant::now(),
        }
    }
}

pub struct KcpServer {
    sock: UdpSocket,
    sessions: BTreeMap<u32, Session>,
    next_conv: u32,
}

impl KcpServer {
    pub fn bind(port: u16) -> io::Result<Self> {
        let sock = UdpSocket::bind(format!("0.0.0.0:{port}"))?;
        sock.set_nonblocking(true)?;
        Ok(Self {
            sock,
            sessions: BTreeMap::new(),
            next_conv: 1,
        })
    }

    /// 往会话追加一条应用消息（内部做 3B 流分帧 + PUSH 分段）
    pub fn send(&mut self, conv: u32, msg: &[u8]) {
        let Some(s) = self.sessions.get_mut(&conv) else { return };
        let len = (msg.len() + 3) as u32;
        s.snd_stream.extend_from_slice(&len.to_le_bytes()[..3]);
        s.snd_stream.extend_from_slice(msg);
    }

    pub fn close(&mut self, conv: u32) {
        if let Some(s) = self.sessions.remove(&conv) {
            let _ = self.sock.send_to(b"CE1DISCONNECT", s.addr);
        }
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// 轮询一次：收包/重传/定时器，返回本批事件
    pub fn poll(&mut self) -> Vec<Event> {
        let Self { sock, sessions, .. } = self;
        let mut events = Vec::new();
        let mut buf = [0u8; 65536];
        loop {
            match sock.recv_from(&mut buf) {
                Ok((n, from)) => {
                    on_packet(sock, sessions, &mut self.next_conv, &buf[..n], from, &mut events);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }
        // 发送侧：刷流 → 分段 PUSH；重传；SYNACK 重发；空闲清理
        let mut dead = Vec::new();
        for (conv, s) in sessions.iter_mut() {
            flush_send(sock, s);
            // 重传
            let rto_unacked: Vec<u32> = s
                .unacked
                .iter()
                .filter(|u| u.last_sent.elapsed() >= RTO)
                .map(|u| u.sn)
                .collect();
            for sn in rto_unacked {
                if let Some(pos) = s.unacked.iter().position(|u| u.sn == sn) {
                    let data = s.unacked[pos].data.clone();
                    let _ = sock.send_to(&data, s.addr);
                    s.unacked[pos].last_sent = Instant::now();
                }
            }
            // SYNACK 重发（已建立但未见首个 PUSH）
            if s.established && !s.first_push_seen && s.last_synack.elapsed() >= SYNACK_RETX {
                send_synack(sock, s);
                s.last_synack = Instant::now();
            }
            if s.last_recv.elapsed() >= IDLE_TIMEOUT {
                dead.push(*conv);
            }
        }
        for conv in dead {
            sessions.remove(&conv);
            events.push(Event::Closed { conv });
        }
        events
    }
}

// ---------- 自由函数（规避 &self 方法与字段可变借用的冲突） ----------

fn send_synack(sock: &UdpSocket, s: &Session) {
    let mut pkt = MAGIC_SYNACK.to_vec();
    pkt.extend_from_slice(&[0, 0, 0]);
    pkt.extend_from_slice(&s.conv.to_le_bytes());
    let _ = sock.send_to(&pkt, s.addr);
}

fn send_ack(sock: &UdpSocket, s: &Session, sn: u32, ts: u32) {
    let mut pkt = Vec::with_capacity(24);
    pkt.extend_from_slice(&s.conv.to_le_bytes());
    pkt.push(CMD_ACK);
    pkt.push(0); // frg
    pkt.extend_from_slice(&WND.to_le_bytes());
    pkt.extend_from_slice(&ts.to_le_bytes()); // 回显被 ACK 段的 ts
    pkt.extend_from_slice(&sn.to_le_bytes());
    pkt.extend_from_slice(&s.rcv_nxt.to_le_bytes()); // una
    pkt.extend_from_slice(&0u32.to_le_bytes());
    let _ = sock.send_to(&pkt, s.addr);
}

/// 把待发送流切成 PUSH 段发出去
fn flush_send(sock: &UdpSocket, s: &mut Session) {
    while !s.snd_stream.is_empty() {
        let take = s.snd_stream.len().min(SEG_PAYLOAD_MAX);
        let payload: Vec<u8> = s.snd_stream.drain(..take).collect();
        let mut pkt = Vec::with_capacity(24 + take);
        pkt.extend_from_slice(&s.conv.to_le_bytes());
        pkt.push(CMD_PUSH);
        pkt.push(0);
        pkt.extend_from_slice(&WND.to_le_bytes());
        pkt.extend_from_slice(&now_ms().to_le_bytes());
        pkt.extend_from_slice(&s.snd_nxt.to_le_bytes());
        pkt.extend_from_slice(&s.rcv_nxt.to_le_bytes());
        pkt.extend_from_slice(&(take as u32).to_le_bytes());
        pkt.extend_from_slice(&payload);
        let _ = sock.send_to(&pkt, s.addr);
        s.unacked.push(Unacked {
            sn: s.snd_nxt,
            data: pkt,
            last_sent: Instant::now(),
        });
        s.snd_nxt += 1;
    }
}

fn on_packet(
    sock: &UdpSocket,
    sessions: &mut BTreeMap<u32, Session>,
    next_conv: &mut u32,
    pkt: &[u8],
    from: SocketAddr,
    events: &mut Vec<Event>,
) {
    // CE1 魔法族
    if pkt.starts_with(MAGIC_SYN) {
        // 同一来源的重复 SYN（客户端 ~10ms 重发）复用既有会话，不重复分配 conv
        if let Some((&conv, _)) = sessions.iter().find(|(_, s)| s.addr == from && !s.first_push_seen) {
            let mut syack = MAGIC_SYACK.to_vec();
            syack.extend_from_slice(&conv.to_le_bytes());
            syack.extend_from_slice(&now_ms().to_le_bytes());
            let _ = sock.send_to(&syack, from);
            return;
        }
        let conv = *next_conv;
        *next_conv += 1;
        let mut syack = MAGIC_SYACK.to_vec();
        syack.extend_from_slice(&conv.to_le_bytes());
        syack.extend_from_slice(&now_ms().to_le_bytes());
        let _ = sock.send_to(&syack, from);
        sessions.insert(conv, Session::new(conv, from));
        return;
    }
    if pkt.starts_with(b"CE1") {
        // CE1ACK / CE1DISCONNECT / CE1TIMEOUT / CE1REP 等
        let mut hit = None;
        for (conv, s) in sessions.iter_mut() {
            if s.addr != from {
                continue;
            }
            if pkt == MAGIC_ACK {
                s.established = true;
                s.last_recv = Instant::now();
                send_synack(sock, s);
                s.last_synack = Instant::now();
                hit = Some((*conv, false));
            } else if pkt.starts_with(b"CE1DISCONNECT") || pkt.starts_with(b"CE1TIMEOUT") {
                hit = Some((*conv, true));
            }
            break;
        }
        if let Some((conv, closed)) = hit {
            if closed {
                sessions.remove(&conv);
                events.push(Event::Closed { conv });
            } else {
                events.push(Event::NewSession { conv });
            }
        }
        return;
    }
    // KCP 段（一个数据报可含多段）
    let mut off = 0usize;
    while off + 24 <= pkt.len() {
        let conv = u32::from_le_bytes(pkt[off..off + 4].try_into().unwrap());
        let cmd = pkt[off + 4];
        let ts = u32::from_le_bytes(pkt[off + 8..off + 12].try_into().unwrap());
        let sn = u32::from_le_bytes(pkt[off + 12..off + 16].try_into().unwrap());
        let len = u32::from_le_bytes(pkt[off + 20..off + 24].try_into().unwrap()) as usize;
        if off + 24 + len > pkt.len() {
            break;
        }
        let payload = &pkt[off + 24..off + 24 + len];
        let Some(s) = sessions.get_mut(&conv) else { return };
        s.last_recv = Instant::now();
        match cmd {
            CMD_PUSH => {
                s.first_push_seen = true;
                // 立即 ACK（ts 回显）
                send_ack(sock, s, sn, ts);
                if sn >= s.rcv_nxt {
                    s.rcv_buf.insert(sn, (ts, payload.to_vec()));
                    // 按序交付到流
                    while let Some((_, p)) = s.rcv_buf.remove(&s.rcv_nxt) {
                        s.stream.extend_from_slice(&p);
                        s.rcv_nxt += 1;
                    }
                    // 切 3B 帧
                    loop {
                        if s.stream.len() < 3 {
                            break;
                        }
                        let flen = s.stream[0] as usize
                            | (s.stream[1] as usize) << 8
                            | (s.stream[2] as usize) << 16;
                        if flen < 3 || flen > 16 * 1024 * 1024 {
                            // 失步（不应发生）：丢弃 1 字节重对齐
                            s.stream.remove(0);
                            continue;
                        }
                        if s.stream.len() < flen {
                            break;
                        }
                        let body = s.stream[3..flen].to_vec();
                        s.stream.drain(..flen);
                        events.push(Event::Message { conv, body });
                    }
                }
            }
            CMD_ACK => {
                s.unacked.retain(|u| u.sn != sn);
            }
            _ => {}
        }
        off += 24 + len;
    }
}
