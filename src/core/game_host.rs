//! GameHost 编排（0.5.0 R3 壳 host：真本地会话面，玩法缺席；R4 将在此换入 lua 编排）。
//!
//! 职责（host 侧消息序列唯一来源 = scegame-reverse.md §13.9）：
//! - 登录（c2h msg 1）→ 回 type 2（登录应答模板）+ 0x15（小状态模板）
//! - 客户端自驱加载进度（msg 3: 30/45/95/100）+ msg 5 → 触发进图初始化消息群（模板原序）
//! - 0x31007 服务器 tick：200ms 周期持续下发（客户端逻辑帧驱动）
//! - 0xF100 时钟同步 → 0xf101 原样回显；0x1001 周期探测 → 0x1108 应答（f1 回显 + session_id）
//! - 0x6011（UI 视图同步）/ msg 5（状态）收到即弃（明确丢弃不断连）；0x7006 玩法上行 R3 忽略（壳无玩法）
//! - h2c 发送一律 ZCompress 原样模式（0x00 + 明文，§13.8 旁路）
//!
//! 进程模型：控制面（host_server.rs，TCP 5003）+ 会话面（kcp_server.rs，UDP 5003+5053）。
//! PIE 入口：先 `host start` 起本服务再点编辑器「调试(本地服务器)」；debug start 入口由常驻进程承载。

use crate::core::host;
use crate::core::host_server::{self, ControlRef, GameInfo};
use crate::core::host_templates as tpl;
use crate::core::kcp_server::{Event, KcpServer};
use crate::core::zcompress;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// tick 周期（官方实测 ~200ms）
const TICK_INTERVAL: Duration = Duration::from_millis(200);
/// 游戏会话 id（壳 host 固定为基准 capture 的值：登录应答模板/__sync_game_info/0x1108 回显三处自带它，
/// 客户端 ping 校验「发送==接收」——固定即免补丁。R4 换真实生成）
const GAME_SESSION_ID: u64 = 7680844811838488582;

pub struct GameHostParams {
    /// 控制/会话端口（默认 5003 = 官方 use_local_host 固定端口；KCP 会话 = +50）
    pub port: u16,
    /// 运行时目录（上传落盘 = <runtime>/User/host_upload/<project>/）
    pub runtime_dir: PathBuf,
}

/// 单个游戏会话的编排状态
struct SessionBrain {
    conv: u32,
    userid: u64,
    progress: u32,
    burst_done: bool,
    tick_counter: u32,
    last_tick: Instant,
}

impl SessionBrain {
    fn new(conv: u32) -> Self {
        Self {
            conv,
            userid: 0,
            progress: 0,
            burst_done: false,
            tick_counter: 0,
            last_tick: Instant::now(),
        }
    }
}

/// KCP 消息信封：外层 {f1: header_bytes}，header = {f1 varint msg_type, f2 bytes body}
fn parse_envelope(data: &[u8]) -> Option<(u64, Vec<u8>)> {
    let mut pos = 0;
    let tag = host::get_varint(data, &mut pos).ok()?;
    if tag != 0x0A {
        return None;
    }
    let hlen = host::get_varint(data, &mut pos).ok()? as usize;
    if pos + hlen > data.len() {
        return None;
    }
    let header = &data[pos..pos + hlen];
    let mut hp = 0;
    let t1 = host::get_varint(header, &mut hp).ok()?;
    if t1 != 0x08 {
        return None;
    }
    let msg_type = host::get_varint(header, &mut hp).ok()?;
    let t2 = host::get_varint(header, &mut hp).ok()?;
    if t2 != 0x12 {
        return Some((msg_type, Vec::new()));
    }
    let blen = host::get_varint(header, &mut hp).ok()? as usize;
    Some((msg_type, header[hp..hp + blen].to_vec()))
}

/// 组应用消息：envelope(msg_type, body) → ZCompress 原样帧
fn build_message(msg_type: u64, body: &[u8]) -> Vec<u8> {
    let mut header = Vec::new();
    host::put_field_varint(&mut header, 1, msg_type);
    host::put_field_bytes(&mut header, 2, body);
    let mut env = Vec::new();
    host::put_field_bytes(&mut env, 1, &header);
    zcompress::encode_frame_raw(&env)
}

/// 进图初始化消息群（模板原序，跳过登录应答对；tick 模板由 tick 循环接管故跳过）
fn send_burst(kcp: &mut KcpServer, conv: u32) {
    for &(ty, hexs) in tpl::H2C_SEQ.iter().skip(2) {
        if ty == 200711 {
            continue; // 0x31007 tick 由周期循环持续下发
        }
        let body = tpl::unhex(hexs);
        let frame = build_message(ty, &body);
        kcp.send(conv, &frame);
    }
}

fn send_tick(kcp: &mut KcpServer, brain: &mut SessionBrain) {
    let mut body = Vec::new();
    host::put_field_varint(&mut body, 1, brain.tick_counter as u64);
    brain.tick_counter += 1;
    let frame = build_message(0x31007, &body);
    kcp.send(brain.conv, &frame);
}

/// 0x1001 周期探测 → 0x1108 应答（f1 回显 + session_id + 官方常量尾，布局见 §13.9 dump）
fn send_1108(kcp: &mut KcpServer, conv: u32, probe_f1: u64, session_id: u64) {
    let mut body = Vec::new();
    host::put_field_varint(&mut body, 1, probe_f1);
    host::put_field_varint(&mut body, 2, session_id);
    host::put_field_varint(&mut body, 3, 0);
    // f4 = fixed32 0
    host::put_varint(&mut body, (4 << 3) | 5);
    body.extend_from_slice(&0u32.to_le_bytes());
    host::put_field_varint(&mut body, 5, 1);
    host::put_field_varint(&mut body, 6, 0);
    // f7 = fixed64 0x2020202020202020（官方实发值，空格×8）
    host::put_varint(&mut body, (7 << 3) | 1);
    body.extend_from_slice(&0x2020202020202020u64.to_le_bytes());
    host::put_field_varint(&mut body, 8, 0);
    host::put_field_varint(&mut body, 9, 9);
    let frame = build_message(0x1108, &body);
    kcp.send(conv, &frame);
}

/// 前台阻塞跑壳 host（host start / debug start --host local 的线程体）
pub fn run(params: GameHostParams, ready_tx: Option<std::sync::mpsc::Sender<Result<u16, String>>>) -> Result<()> {
    let state: ControlRef = host_server::new_state();
    let upload_root = params.runtime_dir.join("User").join("host_upload");
    // 控制面线程（TCP 5003）
    {
        let state = Arc::clone(&state);
        let root = upload_root.clone();
        let port = params.port;
        std::thread::spawn(move || {
            if let Err(e) = host_server::run(port, state, root) {
                println!("[host-ctl] 控制面退出: {e}");
            }
        });
    }
    // 会话面：KCP 会话端口 = 控制端口 + 50（引擎硬编码）；5003-UDP 一并监听兜底
    let mut kcp = KcpServer::bind(params.port + 50)?;
    let kcp_alt = KcpServer::bind(params.port)?; // 5003-UDP（官方中继观察到此口无流量，兜底）
    let _ = kcp_alt; // 暂不合并（客户端只 dial +50）
    let mut brains: std::collections::HashMap<u32, SessionBrain> = std::collections::HashMap::new();
    let mut game: Option<GameInfo> = None;
    host_server::push_log(&state, "[shell-host] 自研壳 host 已启动（0.5.0 R3）");
    println!("[game-host] 壳 host 已监听 TCP {} + UDP {}/{}，等待接入", params.port, params.port, params.port + 50);
    if let Some(tx) = ready_tx {
        let _ = tx.send(Ok(params.port));
    }
    loop {
        // 起局/停局信号同步（注意：锁内禁止 push_log——push_log 会再拿锁，Mutex 不可重入死锁）
        let mut new_game = None;
        let mut do_teardown = false;
        {
            let mut g = state.lock().unwrap();
            if g.teardown {
                g.teardown = false;
                do_teardown = game.take().is_some();
            }
            if let Some(info) = &g.game {
                if game.as_ref().map(|g| g.session_id) != Some(info.session_id) {
                    new_game = Some(info.clone());
                }
            }
        }
        if do_teardown {
            println!("[game-host] 停局 teardown：清理全部会话");
            for conv in brains.keys().copied().collect::<Vec<_>>() {
                kcp.close(conv);
            }
            brains.clear();
            host_server::push_log(&state, "[shell-host] 局已销毁，可接下一局");
        }
        if let Some(info) = new_game {
            println!("[game-host] 起局: {} session={}", info.project, info.session_id);
            host_server::push_log(&state, "[shell-host] 起局，等待客户端 KCP 接入");
            game = Some(info);
        }
        // 会话面事件
        for ev in kcp.poll() {
            match ev {
                Event::NewSession { conv } => {
                    println!("[game-host] KCP 会话建立 conv={conv:#x}");
                    brains.insert(conv, SessionBrain::new(conv));
                }
                Event::Message { conv, body } => {
                    let Some(brain) = brains.get_mut(&conv) else { continue };
                    let Some((ty, mbody)) = parse_envelope(&body) else { continue };
                    match ty {
                        1 => {
                            // 登录：{f1 userid, ...}
                            brain.userid = host::body_varint(&mbody, 1).unwrap_or(0);
                            println!("[game-host] 登录: userid={}", brain.userid);
                            // 登录应答（模板 type 2）+ 0x15
                            for &(ty2, hexs) in tpl::H2C_SEQ.iter().take(2) {
                                let frame = build_message(ty2, &tpl::unhex(hexs));
                                kcp.send(conv, &frame);
                            }
                            host_server::push_log(&state, &format!("[shell-host] 玩家 {} 登录", brain.userid));
                        }
                        3 => {
                            brain.progress = host::body_varint(&mbody, 1).unwrap_or(0) as u32;
                        }
                        5 => {
                            // 状态（收到即弃）；msg5 = 客户端加载完毕信号 → 触发初始化消息群
                            if !brain.burst_done {
                                brain.burst_done = true;
                                println!("[game-host] 客户端加载完成，发送初始化消息群");
                                send_burst(&mut kcp, conv);
                                host_server::push_log(&state, "[shell-host] 客户端进图，初始化消息群已下发");
                            }
                        }
                        0x2000 => {}   // 心跳：无需应答
                        0x6011 => {}   // UI 视图同步：收到即弃
                        0x7006 => {}   // 玩法上行：R3 壳无玩法（R4 接入 lua）
                        0x1001 => {
                            // 周期探测 → 0x1108 应答
                            let probe = host::body_varint(&mbody, 1).unwrap_or(0);
                            send_1108(&mut kcp, conv, probe, GAME_SESSION_ID);
                        }
                        0xF100 => {
                            // 时钟同步 → 0xf101 原样回显
                            let frame = build_message(0xF101, &mbody);
                            kcp.send(conv, &frame);
                        }
                        _ => {}
                    }
                }
                Event::Closed { conv } => {
                    println!("[game-host] KCP 会话结束 conv={conv:#x}");
                    brains.remove(&conv);
                }
            }
        }
        // tick：仅对已进图（burst 完成）的会话下发
        for brain in brains.values_mut() {
            if brain.burst_done && brain.last_tick.elapsed() >= TICK_INTERVAL {
                send_tick(&mut kcp, brain);
                brain.last_tick = Instant::now();
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// 幂等确保壳 host 在跑（GUI/CLI debug 复用）
static RUNNING: Mutex<Option<u16>> = Mutex::new(None);

pub fn ensure_running(params: GameHostParams) -> Result<u16> {
    let mut g = RUNNING.lock().unwrap();
    if let Some(port) = *g {
        return Ok(port);
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        if let Err(e) = run(params, Some(tx)) {
            println!("[game-host] 壳 host 退出: {e}");
        }
    });
    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(Ok(p)) => {
            *g = Some(p);
            Ok(p)
        }
        Ok(Err(e)) => Err(anyhow::anyhow!("壳 host 启动失败: {e}")),
        Err(_) => Err(anyhow::anyhow!("壳 host 启动超时")),
    }
}
