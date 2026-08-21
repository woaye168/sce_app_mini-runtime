//! 抓包分析器：解析 frida_capture 产出的 control_capture.jsonl（ws2_32 收发记录）。
//! 合并原 protocol_parse.py / analyze_capture.py / block_seq.py 三个工具。
//!
//! 用法：
//!   cargo run --example capture_parse -- frames <capture.jsonl> [host前缀=106.]
//!   cargo run --example capture_parse -- dump   <capture.jsonl> [host前缀=106.]
//!   cargo run --example capture_parse -- blocks <capture.jsonl> [host前缀=106.]
//!
//! 帧格式（已实证）: u32 LE total_len(含自身) + u8 0x00 + envelope{ f1: { f1 varint msg_type, f2 bytes body } }
//!
//! 子命令：
//!   frames —— 切出控制会话全部帧，按消息类型逐帧 dump body 字段
//!   dump   —— 打印会话时间线全部记录（hex + 递归 wire 解码）
//!   blocks —— 提取大文件分块序列（0xF004/0xF008/0xF00A）的精确字段布局

use std::collections::HashMap;

mod util;
use util::{get_varint, hex, is_printable, parse_fields, FieldVal};

fn msg_name(t: u64) -> String {
    match t {
        0xF000 => "EditorLogin".into(),
        0xF001 => "EditorLoginResult".into(),
        0xF004 => "SendWriteFile".into(),
        0xF008 => "SendFileBlock".into(),
        0xF00A => "FileEnd".into(),
        0xF00C => "NotifyEditorLog".into(),
        0xF010 => "SendWriteFileAck".into(),
        0xF011 => "EditorPing".into(),
        0xF012 => "EditorStartGame".into(),
        0xF017 => "EditorPingRes".into(),
        0xF018 => "StartGameRes".into(),
        _ => format!("type_{t:#x}"),
    }
}

/// 从一段字节流切出全部帧，返回 (msg_type, body, frame_len)；坏帧返回 None 元素中止。
fn frames_of(data: &[u8]) -> Vec<(u64, Vec<u8>, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 5 <= data.len() {
        let total = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as usize;
        if total < 6 || i + total > data.len() {
            eprintln!("BADFRAME at {i}: total={total} rest={}", data.len() - i);
            break;
        }
        let frame = &data[i..i + total];
        i += total;
        // frame: total(4) + flag(1) + 0x0a <len> env{ 0x08 <type> 0x12 <len> body }
        let mut j = 5usize;
        let Some(_tag) = get_varint(frame, &mut j) else { break };
        let Some(elen) = get_varint(frame, &mut j) else { break };
        let env = &frame[j..j + elen as usize];
        let fields = parse_fields(env);
        let (mut mtype, mut body) = (0u64, Vec::new());
        for (f, v) in fields {
            match (f, v) {
                (1, FieldVal::Varint(t)) => mtype = t,
                (2, FieldVal::Bytes(b)) => body = b,
                _ => {}
            }
        }
        out.push((mtype, body, total));
    }
    out
}

fn dump_body(body: &[u8], indent: &str) {
    for (f, v) in parse_fields(body) {
        match v {
            FieldVal::Varint(x) => println!("{indent}f{f} varint = {x} ({x:#x})"),
            FieldVal::Bytes(b) if is_printable(&b) => {
                let s = String::from_utf8_lossy(&b);
                println!("{indent}f{f} len={} str = \"{:.160}\"", b.len(), s);
            }
            FieldVal::Bytes(b) => println!(
                "{indent}f{f} len={} bytes = {}{}",
                b.len(),
                hex(&b[..b.len().min(48)]),
                if b.len() > 48 { "..." } else { "" }
            ),
            FieldVal::I32(x) => println!("{indent}f{f} i32 = {}", hex(&x)),
            FieldVal::I64(x) => println!("{indent}f{f} i64 = {}", hex(&x)),
        }
    }
}

fn wire_dump(b: &[u8], depth: usize, maxdepth: usize) {
    let pad = "  ".repeat(depth);
    for (f, v) in parse_fields(b) {
        match v {
            FieldVal::Varint(x) => println!("{pad}f{f} varint = {x} ({x:#x})"),
            FieldVal::Bytes(p) if is_printable(&p) => {
                println!("{pad}f{f} len={} str = \"{:.120}\"", p.len(), String::from_utf8_lossy(&p));
            }
            FieldVal::Bytes(p) if depth < maxdepth && p.len() > 1 && !parse_fields(&p).is_empty() => {
                // 启发式：能按 wire 解出字段就递归当嵌套消息
                println!("{pad}f{f} len={} msg = {{", p.len());
                wire_dump(&p, depth + 1, maxdepth);
                println!("{pad}}}");
            }
            FieldVal::Bytes(p) => println!(
                "{pad}f{f} len={} bytes = {}{}",
                p.len(),
                hex(&p[..p.len().min(32)]),
                if p.len() > 32 { "..." } else { "" }
            ),
            FieldVal::I32(x) => println!("{pad}f{f} i32 = {}", hex(&x)),
            FieldVal::I64(x) => println!("{pad}f{f} i64 = {}", hex(&x)),
        }
    }
}

struct Rec {
    op: String,
    sock: String,
    addr: String,
    len: i64,
    data: Vec<u8>,
}

fn load(path: &str, host_prefix: &str) -> (Vec<Rec>, Vec<String>) {
    let text = std::fs::read_to_string(path).expect("读取 capture 文件失败");
    let mut recs = Vec::new();
    let mut sock2addr: HashMap<String, String> = HashMap::new();
    for line in text.lines() {
        let v: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let op = v["op"].as_str().unwrap_or("").to_string();
        let sock = v["sock"].as_str().unwrap_or("").to_string();
        let addr = v["addr"].as_str().unwrap_or("").to_string();
        if op == "connect" {
            sock2addr.insert(sock.clone(), addr.clone());
        }
        recs.push(Rec {
            op,
            sock,
            addr,
            len: v["len"].as_i64().unwrap_or(0),
            data: v["data"]
                .as_str()
                .filter(|s| !s.is_empty() && !s.starts_with('<'))
                .and_then(|s| (0..s.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, _>>()
                    .ok())
                .unwrap_or_default(),
        });
    }
    // WSASend 记录 addr='?' 的用 sock 补
    for r in &mut recs {
        if r.addr == "?" || r.addr.is_empty() {
            if let Some(a) = sock2addr.get(&r.sock) {
                r.addr = a.clone();
            }
        }
    }
    let host_socks: Vec<String> = sock2addr
        .iter()
        .filter(|(_, a)| a.starts_with(host_prefix))
        .map(|(s, _)| s.clone())
        .collect();
    println!("host socks: {:?}", host_socks.iter()
        .map(|s| format!("{s}={}", sock2addr[s])).collect::<Vec<_>>());
    (recs, host_socks)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: capture_parse <frames|dump|blocks> <capture.jsonl> [host前缀=106.]");
        std::process::exit(1);
    }
    let (cmd, path) = (args[1].as_str(), args[2].as_str());
    let prefix = args.get(3).map(String::as_str).unwrap_or("106.");
    let (recs, host_socks) = load(path, prefix);
    let is_io = |op: &str| matches!(op, "send" | "recv" | "WSASend" | "WSARecv");

    match cmd {
        "frames" => {
            for r in &recs {
                if !host_socks.contains(&r.sock) || !is_io(&r.op) || r.data.is_empty() {
                    continue;
                }
                for (mtype, body, total) in frames_of(&r.data) {
                    println!("\n== {} {} (frame {total}B, body {}B)", r.op, msg_name(mtype), body.len());
                    dump_body(&body, "    ");
                }
            }
        }
        "dump" => {
            let timeline: Vec<&Rec> = recs
                .iter()
                .filter(|r| r.addr.starts_with(prefix) || host_socks.contains(&r.sock))
                .collect();
            println!("\n=== control session records: {} ===", timeline.len());
            for (idx, r) in timeline.iter().enumerate() {
                println!("\n--- [{idx}] {} sock={} addr={} len={}", r.op, r.sock, r.addr, r.len);
                if !r.data.is_empty() {
                    println!("hex: {:.200}{}", hex(&r.data), if r.data.len() > 100 { "..." } else { "" });
                    if is_io(&r.op) && r.len > 0 {
                        wire_dump(&r.data, 0, 3);
                    }
                }
            }
        }
        "blocks" => {
            let mut seq = Vec::new();
            for r in &recs {
                if host_socks.contains(&r.sock) && r.op == "send" {
                    for (mtype, body, _) in frames_of(&r.data) {
                        seq.push((mtype, body));
                    }
                }
            }
            println!("total frames: {}", seq.len());
            let (mut cur_path, mut count) = (String::new(), 0u32);
            for (mtype, body) in &seq {
                if !matches!(mtype, 0xF004 | 0xF008 | 0xF00A) {
                    continue;
                }
                let fs = parse_fields(body);
                let path = fs
                    .iter()
                    .find_map(|(f, v)| match (f, v) {
                        (1, FieldVal::Bytes(b)) => Some(String::from_utf8_lossy(b).to_string()),
                        _ => None,
                    })
                    .unwrap_or_else(|| "?".into());
                if path != cur_path {
                    cur_path = path.clone();
                    count = 0;
                    println!("\n### {path}");
                }
                count += 1;
                let desc: Vec<String> = fs
                    .iter()
                    .map(|(f, v)| match v {
                        FieldVal::Bytes(b) if b.len() > 200 => {
                            format!("f{f}=bytes[{}] head={}...", b.len(), hex(&b[..8]))
                        }
                        FieldVal::Bytes(b) => format!("f{f}=\"{}\"", String::from_utf8_lossy(b)),
                        FieldVal::Varint(x) => format!("f{f}={x}"),
                        FieldVal::I32(x) => format!("f{f}=i32:{}", hex(x)),
                        FieldVal::I64(x) => format!("f{f}=i64:{}", hex(x)),
                    })
                    .collect();
                println!("  [{count}] type={mtype:#x}: {}", desc.join(" "));
                if count > 30 {
                    println!("  ...(截断)");
                    cur_path.clear();
                }
            }
        }
        _ => eprintln!("未知子命令: {cmd}（frames|dump|blocks）"),
    }
}
