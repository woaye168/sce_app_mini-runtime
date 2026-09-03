//! 服务器日志总线（0.5.1）：host 侧线程日志（原 println!）tee 进内存环形缓冲，
//! 供「本地服务器」标签页日志面板消费；CLI 场景仍照常打印 stdout。
//!
//! 用 `srv_log!` 宏（与 println! 同参），禁止在持锁临界区内调用（push 内会再拿总线锁）。

use std::collections::VecDeque;
use std::sync::Mutex;

/// 环形缓冲容量（行）
const CAP: usize = 5000;

struct Bus {
    next_seq: u64,
    lines: VecDeque<(u64, String)>,
}

static BUS: Mutex<Option<Bus>> = Mutex::new(None);

/// 推一行日志（同时打印 stdout）
pub fn push(line: String) {
    println!("{line}");
    let mut g = BUS.lock().unwrap();
    let bus = g.get_or_insert_with(|| Bus {
        next_seq: 1,
        lines: VecDeque::new(),
    });
    let seq = bus.next_seq;
    bus.next_seq += 1;
    if bus.lines.len() >= CAP {
        bus.lines.pop_front();
    }
    bus.lines.push_back((seq, line));
}

/// 取 last_seq 之后的新日志，返回（最新 seq, 新行）。
/// 缓冲满溢出丢最旧行会造成 seq 空洞：在新行首部插一条提示行，
/// 让消费者能区分「无新日志」与「中间丢了一段」（首次拉取 last_seq=0 不报）
pub fn fetch_after(last_seq: u64) -> (u64, Vec<String>) {
    let g = BUS.lock().unwrap();
    let Some(bus) = g.as_ref() else {
        return (last_seq, Vec::new());
    };
    let mut out = Vec::new();
    let mut max = last_seq;
    if last_seq > 0 {
        if let Some((first, _)) = bus.lines.front() {
            if *first > last_seq + 1 {
                out.push(format!("……（日志缓冲溢出，丢失 {} 行）……", first - last_seq - 1));
            }
        }
    }
    for (seq, line) in bus.lines.iter() {
        if *seq > last_seq {
            out.push(line.clone());
            max = *seq;
        }
    }
    (max, out)
}

/// 服务器日志宏（println! 同参）：打印 + 进总线
#[macro_export]
macro_rules! srv_log {
    ($($t:tt)*) => { $crate::core::logbus::push(format!($($t)*)) };
}
