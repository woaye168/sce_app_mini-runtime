//! KCP 会话抓包分析器：解析 local_host 中继产出的 host_capture-*.jsonl（udp.c2h / udp.h2c 记录）。
//!
//! 用法：
//!   cargo run --example kcp_capture_parse -- stats <capture.jsonl>
//!   cargo run --example kcp_capture_parse -- flow  <capture.jsonl> [每方向条数=60]
//! 本机 dev-deps（frida-sys）缺 libclang 时可用 rustc 直编（零外部依赖）：
//!   rustc --edition 2021 -O examples/kcp_capture_parse.rs -o test/temp/kcp_parse.exe
//!
//! 协议结论（2026-09-02 实证，详见 doc/research/self-host.md §10）：
//!   握手：ASCII 魔法 CE1SYN → CE1SYACK（服务器下发 conv）→ CE1ACK → CE1SYNACK
//!   KCP 段：conv(4 LE) cmd(1) frg(1) wnd(2 LE) ts(4 LE) sn(4 LE) una(4 LE) len(4 LE) + payload
//!   cmd：0x51=PUSH 0x52=ACK；payload = 3 字节 LE 长度前缀 + 消息体（-kcp_stream 流式分帧）
//!   c2h 明文 protobuf（f1{ f1=msg_type, f2=body }）；h2c 加密/混淆（登录后即起，算法未定）

use std::collections::HashMap;

mod util;
use util::{hex, is_printable, parse_fields, FieldVal};

// ---------------- jsonl 行内提取（免 serde_json，rustc 直编可用） ----------------

fn extract_str<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":\"");
    let start = line.find(&pat)? + pat.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(&rest[..end])
}

fn extract_num(line: &str, key: &str) -> Option<u64> {
    let pat = format!("\"{key}\":");
    let start = line.find(&pat)? + pat.len();
    let rest = &line[start..];
    let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
    rest[..end].parse().ok()
}

struct Seg {
    ts: u64,
    chan: String,
    conv: u32,
    cmd: u8,
    sn: u32,
    payload: Vec<u8>,
}

fn load(path: &str) -> (Vec<String>, Vec<Seg>) {
    let text = std::fs::read_to_string(path).expect("读取 capture 文件失败");
    let mut handshakes = Vec::new();
    let mut segs = Vec::new();
    for line in text.lines() {
        if !line.contains("\"udp") {
            continue;
        }
        let Some(chan) = extract_str(line, "chan").map(|s| s.to_string()) else { continue };
        let Some(hexs) = extract_str(line, "hex") else { continue };
        let ts = extract_num(line, "ts").unwrap_or(0);
        let raw: Vec<u8> = (0..hexs.len().saturating_sub(1))
            .step_by(2)
            .filter_map(|i| u8::from_str_radix(&hexs[i..i + 2], 16).ok())
            .collect();
        if raw.starts_with(b"CE1") {
            handshakes.push(format!(
                "[{ts:>8}] {chan}: {}",
                String::from_utf8_lossy(&raw[..raw.len().min(24)])
            ));
            continue;
        }
        if raw.len() < 24 {
            continue;
        }
        let u32le = |o: usize| u32::from_le_bytes(raw[o..o + 4].try_into().unwrap());
        let len = u32le(20) as usize;
        segs.push(Seg {
            ts,
            chan,
            conv: u32le(0),
            cmd: raw[4],
            sn: u32le(12),
            payload: raw[24..(24 + len).min(raw.len())].to_vec(),
        });
    }
    (handshakes, segs)
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: kcp_capture_parse <stats|flow> <capture.jsonl> [条数]");
        std::process::exit(1);
    }
    let (cmd, path) = (args[1].as_str(), args[2].as_str());
    let (handshakes, segs) = load(path);

    println!("=== 握手序列（前 12 条）===");
    for h in handshakes.iter().take(12) {
        println!("{h}");
    }

    match cmd {
        "stats" => {
            let mut cmd_count: HashMap<(String, u8), usize> = HashMap::new();
            let mut push_sn: HashMap<(String, u32), usize> = HashMap::new();
            let mut convs: HashMap<u32, usize> = HashMap::new();
            for s in &segs {
                *cmd_count.entry((s.chan.clone(), s.cmd)).or_default() += 1;
                *convs.entry(s.conv).or_default() += 1;
                if s.cmd == 0x51 {
                    *push_sn.entry((s.chan.clone(), s.sn)).or_default() += 1;
                }
            }
            println!("\n=== conv 分布 ===");
            let mut cv: Vec<_> = convs.iter().collect();
            cv.sort();
            for (c, n) in cv {
                println!("conv={c:#x}: {n} 包");
            }
            println!("\n=== cmd 分布 ===");
            let mut kv: Vec<_> = cmd_count.iter().collect();
            kv.sort();
            for ((chan, cmd), n) in kv {
                println!("{chan} cmd={cmd:#04x}: {n}");
            }
            let retrans: usize = push_sn.values().filter(|&&n| n > 1).count();
            println!("\nPUSH 唯一段 {}，重传段 {retrans}", push_sn.len());
        }
        "flow" => {
            let limit: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(60);
            for dir in ["udp.c2h", "udp.h2c"] {
                let mut seen: HashMap<u32, &Seg> = HashMap::new();
                for s in segs.iter().filter(|s| s.chan == dir && s.cmd == 0x51) {
                    seen.entry(s.sn).or_insert(s);
                }
                let mut uniq: Vec<&Seg> = seen.into_values().collect();
                uniq.sort_by_key(|s| s.ts);
                println!("\n===== {dir} 唯一 PUSH 段 {} 条（dump 前 {limit}）=====", uniq.len());
                for (i, s) in uniq.iter().take(limit).enumerate() {
                    println!(
                        "\n--- [{i}] ts={} conv={:#x} sn={} len={} hex={:.96}{}",
                        s.ts,
                        s.conv,
                        s.sn,
                        s.payload.len(),
                        hex(&s.payload),
                        if s.payload.len() > 48 { "..." } else { "" }
                    );
                    // 剥 3 字节 LE 长度前缀（-kcp_stream 流式分帧）
                    if s.payload.len() >= 3 {
                        let body = &s.payload[3..];
                        wire_dump(body, 1, 4);
                        // ASCII 串扫描（msgpack 内嵌字符串）
                        let mut run = Vec::new();
                        for &b in body.iter().chain(std::iter::once(&0u8)) {
                            if (32..127).contains(&b) {
                                run.push(b);
                            } else {
                                if run.len() >= 4 {
                                    println!("  str> {}", String::from_utf8_lossy(&run));
                                }
                                run.clear();
                            }
                        }
                    }
                }
            }
        }
        _ => eprintln!("未知子命令: {cmd}（stats|flow）"),
    }
}
