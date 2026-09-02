//! KCP 会话抓包全解码分析器：解析 local_host 中继产出的 host_capture-*.jsonl。
//!
//! 用法：
//!   cargo run --example kcp_capture_parse -- stats  <capture.jsonl>
//!   cargo run --example kcp_capture_parse -- flow   <capture.jsonl> [每方向条数=60]
//!   cargo run --example kcp_capture_parse -- decode <capture.jsonl> [conv hex=全部] [每方向帧数上限=0 不限]
//!   cargo run --example kcp_capture_parse -- msgs   <capture.jsonl>
//! 本机 dev-deps（frida-sys）缺 libclang 时用 rustc 直编（零外部依赖）：
//!   rustc --edition 2021 -O examples/kcp_capture_parse.rs -o test/temp/kcp_parse.exe
//!
//! 协议结论（详见 doc/research/scegame-reverse.md §13）：
//!   握手：ASCII 魔法 CE1SYN → CE1SYACK（服务器下发 conv）→ CE1ACK → CE1SYNACK
//!   KCP 段：conv(4 LE) cmd(1) frg(1) wnd(2 LE) ts(4 LE) sn(4 LE) una(4 LE) len(4 LE) + payload
//!   cmd：0x51=PUSH 0x52=ACK；**一个 UDP 数据报可携带多个 KCP 段（host 合并发送，逐段解析！）**
//!   -kcp_stream 流式分帧 = 3 字节 LE 长度（含自身）+ 消息体
//!   c2h 明文 protobuf（f1{ f1=msg_type, f2=body }）；h2c = ZCompress（src/core/zcompress.rs 复刻，§13.8）
//!   玩法协议（0x7006）消息体 = cmsg_pack（msgpack 变体：字符串走 bin 家族 0xc4/0xc5/0xc6）

use std::collections::HashMap;
use std::fmt::Write as _;

mod util;
use util::{hex, is_printable, parse_fields, FieldVal};

#[path = "../src/core/zcompress.rs"]
mod zcompress;

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

#[derive(Clone)]
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
        // 一个数据报可携带多个 KCP 段（host 合并发送）：逐段迭代
        let mut off = 0usize;
        while off + 24 <= raw.len() {
            let len = u32::from_le_bytes(raw[off + 20..off + 24].try_into().unwrap()) as usize;
            if off + 24 + len > raw.len() {
                break;
            }
            segs.push(Seg {
                ts,
                chan: chan.clone(),
                conv: u32::from_le_bytes(raw[off..off + 4].try_into().unwrap()),
                cmd: raw[off + 4],
                sn: u32::from_le_bytes(raw[off + 12..off + 16].try_into().unwrap()),
                payload: raw[off + 24..off + 24 + len].to_vec(),
            });
            off += 24 + len;
        }
    }
    (handshakes, segs)
}

// ---------------- cmsg_pack（msgpack 变体）极简解析 ----------------

#[derive(Debug)]
enum CVal {
    Nil,
    Bool(bool),
    Int(i64),
    F64(f64),
    Str(String),
    Bin(Vec<u8>),
    Array(Vec<CVal>),
    Map(Vec<(CVal, CVal)>),
}

struct CParser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> CParser<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.i + n > self.b.len() {
            return None;
        }
        let s = &self.b[self.i..self.i + n];
        self.i += n;
        Some(s)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn be(&mut self, n: usize) -> Option<u64> {
        let s = self.take(n)?;
        let mut v = 0u64;
        for &x in s {
            v = (v << 8) | x as u64;
        }
        Some(v)
    }
    fn val(&mut self) -> Option<CVal> {
        let t = self.u8()?;
        Some(match t {
            0x00..=0x7f => CVal::Int(t as i64),
            0xe0..=0xff => CVal::Int(t as i8 as i64),
            0xc0 => CVal::Nil,
            0xc2 => CVal::Bool(false),
            0xc3 => CVal::Bool(true),
            0xcc => CVal::Int(self.u8()? as i64),
            0xcd => CVal::Int(self.be(2)? as i64),
            0xce => CVal::Int(self.be(4)? as i64),
            0xcf => CVal::Int(self.be(8)? as i64),
            0xd0 => CVal::Int(self.u8()? as i8 as i64),
            0xd1 => CVal::Int(self.be(2)? as i16 as i64),
            0xd2 => CVal::Int(self.be(4)? as i32 as i64),
            0xd3 => CVal::Int(self.be(8)? as i64),
            0xca => CVal::F64(f32::from_bits(self.be(4)? as u32) as f64),
            0xcb => CVal::F64(f64::from_bits(self.be(8)?)),
            // cmsg_pack 变体：字符串走 bin 家族
            0xc4 | 0xc5 | 0xc6 => {
                let n = match t {
                    0xc4 => self.u8()? as usize,
                    0xc5 => self.be(2)? as usize,
                    _ => self.be(4)? as usize,
                };
                let s = self.take(n)?;
                match std::str::from_utf8(s) {
                    Ok(v) => CVal::Str(v.to_string()),
                    Err(_) => CVal::Bin(s.to_vec()),
                }
            }
            0xd9 | 0xda | 0xdb => {
                let n = match t {
                    0xd9 => self.u8()? as usize,
                    0xda => self.be(2)? as usize,
                    _ => self.be(4)? as usize,
                };
                let s = self.take(n)?;
                CVal::Str(String::from_utf8_lossy(s).to_string())
            }
            0xa0..=0xbf => {
                let n = (t & 0x1f) as usize;
                let s = self.take(n)?;
                CVal::Str(String::from_utf8_lossy(s).to_string())
            }
            0x90..=0x9f => {
                let n = (t & 0x0f) as usize;
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    v.push(self.val()?);
                }
                CVal::Array(v)
            }
            0xdc | 0xdd => {
                let n = if t == 0xdc { self.be(2)? } else { self.be(4)? } as usize;
                let mut v = Vec::with_capacity(n.min(65536));
                for _ in 0..n {
                    v.push(self.val()?);
                }
                CVal::Array(v)
            }
            0x80..=0x8f => {
                let n = (t & 0x0f) as usize;
                let mut v = Vec::with_capacity(n);
                for _ in 0..n {
                    let k = self.val()?;
                    let val = self.val()?;
                    v.push((k, val));
                }
                CVal::Map(v)
            }
            0xde | 0xdf => {
                let n = if t == 0xde { self.be(2)? } else { self.be(4)? } as usize;
                let mut v = Vec::with_capacity(n.min(65536));
                for _ in 0..n {
                    let k = self.val()?;
                    let val = self.val()?;
                    v.push((k, val));
                }
                CVal::Map(v)
            }
            _ => return None,
        })
    }
}

fn cmsg_unpack(b: &[u8]) -> Option<CVal> {
    let mut p = CParser { b, i: 0 };
    let v = p.val()?;
    // 必须全量消费，否则视为非 cmsg_pack（防首字节 0x0a 被误读为 int）
    if p.i != b.len() {
        return None;
    }
    Some(v)
}

fn cval_short(v: &CVal, depth: usize, out: &mut String) {
    if depth > 4 {
        let _ = write!(out, "..");
        return;
    }
    match v {
        CVal::Nil => out.push_str("nil"),
        CVal::Bool(b) => { let _ = write!(out, "{b}"); }
        CVal::Int(i) => { let _ = write!(out, "{i}"); }
        CVal::F64(f) => { let _ = write!(out, "{f}"); }
        CVal::Str(s) => {
            if s.chars().count() > 24 {
                let t: String = s.chars().take(24).collect();
                let _ = write!(out, "\"{t}..\"");
            } else {
                let _ = write!(out, "\"{s}\"");
            }
        }
        CVal::Bin(b) => { let _ = write!(out, "<bin {}B>", b.len()); }
        CVal::Array(a) => {
            out.push('[');
            for (i, x) in a.iter().take(6).enumerate() {
                if i > 0 {
                    out.push(',');
                }
                cval_short(x, depth + 1, out);
            }
            if a.len() > 6 {
                let _ = write!(out, "..+{}", a.len() - 6);
            }
            out.push(']');
        }
        CVal::Map(m) => {
            out.push('{');
            for (i, (k, x)) in m.iter().take(6).enumerate() {
                if i > 0 {
                    out.push(',');
                }
                cval_short(k, depth + 1, out);
                out.push(':');
                cval_short(x, depth + 1, out);
            }
            if m.len() > 6 {
                let _ = write!(out, "..+{}", m.len() - 6);
            }
            out.push('}');
        }
    }
}

fn cval_display(v: &CVal) -> String {
    let mut s = String::new();
    cval_short(v, 0, &mut s);
    if s.len() > 160 {
        s.truncate(160);
        s.push_str("..");
    }
    s
}

// ---------------- 应用层解码 ----------------

fn msg_type_name(t: u64) -> String {
    match t {
        1 => "登录".into(),
        2 => "登录应答?".into(),
        3 => "加载进度".into(),
        5 => "状态".into(),
        0x2000 => "心跳".into(),
        0x6011 => "UI视图同步".into(),
        0x7006 => "玩法协议".into(),
        0xF100 => "时钟同步".into(),
        _ => format!("{t:#x}"),
    }
}

/// 解信封：外层 {f1: header_bytes}，header = {f1 varint msg_type, f2 bytes body}
fn envelope(body: &[u8]) -> Option<(u64, Vec<u8>)> {
    let mut header = None;
    for (f, v) in parse_fields(body) {
        if let (1, FieldVal::Bytes(p)) = (f, &v) {
            header = Some(p.clone());
            break;
        }
    }
    let header = header?;
    let mut ty = None;
    let mut bd = None;
    for (f, v) in parse_fields(&header) {
        match (f, &v) {
            (1, FieldVal::Varint(x)) => ty = Some(*x),
            (2, FieldVal::Bytes(p)) => bd = Some(p.clone()),
            _ => {}
        }
    }
    Some((ty?, bd.unwrap_or_default()))
}

/// 0x7006 玩法消息体摘要：cmsg_pack 解出 {type,args}（c2h）或内层结构（h2c）
fn gameplay_summary(body: &[u8]) -> String {
    // 先试整包 cmsg_pack（c2h 形态 {type,args}）
    if let Some(v) = cmsg_unpack(body) {
        return cval_display(&v);
    }
    // h2c 形态：protobuf 外壳内嵌 cmsg_pack（探测各 bytes 字段）
    let mut parts = Vec::new();
    for (f, v) in parse_fields(body) {
        match v {
            FieldVal::Varint(x) => parts.push(format!("f{f}={x}")),
            FieldVal::Bytes(p) => {
                if let Some(v) = cmsg_unpack(&p) {
                    parts.push(format!("f{f}={}", cval_display(&v)));
                } else if is_printable(&p) {
                    parts.push(format!("f{f}=\"{:.40}\"", String::from_utf8_lossy(&p)));
                } else {
                    parts.push(format!("f{f}=<{}B {}>", p.len(), hex(&p[..p.len().min(16)])));
                }
            }
            _ => {}
        }
    }
    parts.join(" ")
}

/// 每 (chan, conv) 重组 KCP 流并分帧（3B LE 长度含自身）
struct Frame {
    ts: u64,
    body: Vec<u8>,
}

fn reassemble(segs: &[Seg], chan: &str, conv: u32) -> Vec<Frame> {
    let mut map: HashMap<u32, (u64, Vec<u8>)> = HashMap::new();
    for s in segs.iter().filter(|s| s.chan == chan && s.conv == conv && s.cmd == 0x51) {
        map.entry(s.sn).or_insert_with(|| (s.ts, s.payload.clone()));
    }
    let mut sns: Vec<u32> = map.keys().copied().collect();
    sns.sort();
    let mut frames = Vec::new();
    let mut stream: Vec<u8> = Vec::new();
    let mut stream_ts = 0u64;
    for sn in &sns {
        let (ts, payload) = &map[sn];
        if stream.is_empty() {
            stream_ts = *ts;
        }
        stream.extend_from_slice(payload);
        // 尽量切帧（帧可能跨段）
        loop {
            if stream.len() < 3 {
                break;
            }
            let len = stream[0] as usize | (stream[1] as usize) << 8 | (stream[2] as usize) << 16;
            if len < 3 || len > 16 * 1024 * 1024 {
                // 失步：丢弃 1 字节尝试重对齐
                stream.remove(0);
                continue;
            }
            if stream.len() < len {
                break;
            }
            frames.push(Frame {
                ts: stream_ts,
                body: stream[3..len].to_vec(),
            });
            stream.drain(..len);
            stream_ts = *ts;
        }
    }
    frames
}

fn cmd_stats(segs: &[Seg]) {
    let mut cmd_count: HashMap<(String, u8), usize> = HashMap::new();
    let mut push_sn: HashMap<(String, u32), usize> = HashMap::new();
    let mut convs: HashMap<u32, usize> = HashMap::new();
    for s in segs {
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

fn cmd_flow(segs: &[Seg], limit: usize) {
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
            if s.payload.len() >= 3 {
                let body = &s.payload[3..];
                wire_dump(body, 1, 4);
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

/// decode 子命令：全量解码回放时间线
fn cmd_decode(segs: &[Seg], conv_filter: Option<u32>, limit: usize) {
    let convs: Vec<u32> = {
        let mut v: Vec<u32> = segs.iter().filter(|s| s.cmd == 0x51).map(|s| s.conv).collect();
        v.sort();
        v.dedup();
        v
    };
    for conv in convs {
        if let Some(f) = conv_filter {
            if conv != f {
                continue;
            }
        }
        println!("\n########## conv {conv:#x} ##########");
        let c2h = reassemble(segs, "udp.c2h", conv);
        let h2c = reassemble(segs, "udp.h2c", conv);
        let mut dec = zcompress::ZDecoder::new();
        // 双向按 ts 合并回放
        let mut merged: Vec<(u64, bool, Vec<u8>)> = Vec::new(); // (ts, is_h2c, frame_body)
        for f in c2h {
            merged.push((f.ts, false, f.body));
        }
        for f in h2c {
            merged.push((f.ts, true, f.body));
        }
        merged.sort_by_key(|(ts, _, _)| *ts);
        let mut shown = 0usize;
        let mut h2c_err = 0usize;
        for (ts, is_h2c, body) in &merged {
            let plain = if *is_h2c {
                match zcompress::decode_frame(&mut dec, body) {
                    Ok(p) => p,
                    Err(e) => {
                        h2c_err += 1;
                        println!("[{ts:>8}] h2c 帧 {}B ZCompress 解码失败: {e}", body.len());
                        continue;
                    }
                }
            } else {
                body.clone()
            };
            shown += 1;
            if limit > 0 && shown > limit {
                println!("...（已达上限 {limit}，剩余省略）");
                break;
            }
            let dir = if *is_h2c { "h→c" } else { "c→h" };
            match envelope(&plain) {
                Some((ty, bd)) => {
                    let name = msg_type_name(ty);
                    if ty == 0x7006 || ty == 0x7008 {
                        println!("[{ts:>8}] {dir} {name}: {}", gameplay_summary(&bd));
                    } else {
                        let brief = if bd.len() <= 24 {
                            format!("body={}", hex(&bd))
                        } else {
                            format!("body={}B", bd.len())
                        };
                        println!("[{ts:>8}] {dir} {name} {brief}");
                    }
                }
                None => {
                    println!(
                        "[{ts:>8}] {dir} <非信封> {}B: {}{}",
                        plain.len(),
                        hex(&plain[..plain.len().min(32)]),
                        if plain.len() > 32 { "..." } else { "" }
                    );
                }
            }
        }
        println!("---- conv {conv:#x} 回放结束（h2c 解码失败 {h2c_err}）----");
    }
}

/// msgs 子命令：消息类型聚合表
fn cmd_msgs(segs: &[Seg]) {
    let convs: Vec<u32> = {
        let mut v: Vec<u32> = segs.iter().filter(|s| s.cmd == 0x51).map(|s| s.conv).collect();
        v.sort();
        v.dedup();
        v
    };
    let mut table: HashMap<(bool, String), usize> = HashMap::new();
    for conv in convs {
        let c2h = reassemble(segs, "udp.c2h", conv);
        let h2c = reassemble(segs, "udp.h2c", conv);
        let mut dec = zcompress::ZDecoder::new();
        for (is_h2c, frames) in [(false, &c2h), (true, &h2c)] {
            for f in frames {
                let plain = if is_h2c {
                    match zcompress::decode_frame(&mut dec, &f.body) {
                        Ok(p) => p,
                        Err(_) => {
                            *table.entry((true, "<解码失败>".into())).or_default() += 1;
                            continue;
                        }
                    }
                } else {
                    f.body.clone()
                };
                let key = match envelope(&plain) {
                    Some((ty, bd)) => {
                        if ty == 0x7006 || ty == 0x7008 {
                            let inner = gameplay_summary(&bd);
                            // 提取 type 名
                            let name = inner
                                .strip_prefix('{')
                                .and_then(|s| s.split(':').next())
                                .unwrap_or("")
                                .trim_matches('"')
                                .to_string();
                            if inner.starts_with("{\"type\"") || inner.starts_with("{type") {
                                format!("0x7006 {}", name)
                            } else {
                                format!("0x7006 ({})", name)
                            }
                        } else {
                            msg_type_name(ty)
                        }
                    }
                    None => "<非信封>".into(),
                };
                *table.entry((is_h2c, key)).or_default() += 1;
            }
        }
    }
    println!("\n=== 消息聚合表 ===");
    let mut rows: Vec<_> = table.iter().collect();
    rows.sort_by_key(|((h, k), _)| (*h, k.clone()));
    for ((h2c, k), n) in rows {
        println!("{} {:<40} ×{n}", if *h2c { "h→c" } else { "c→h" }, k);
    }
}

/// dump 子命令：按 msg_type 全量 dump（hex + wire 结构 + cmsg 摘要）
fn cmd_dump(segs: &[Seg], conv_filter: Option<u32>, want: u64, count: usize) {
    let convs: Vec<u32> = {
        let mut v: Vec<u32> = segs.iter().filter(|s| s.cmd == 0x51).map(|s| s.conv).collect();
        v.sort();
        v.dedup();
        v
    };
    let mut shown = 0usize;
    for conv in convs {
        if let Some(f) = conv_filter {
            if conv != f {
                continue;
            }
        }
        let c2h = reassemble(segs, "udp.c2h", conv);
        let h2c = reassemble(segs, "udp.h2c", conv);
        let mut dec = zcompress::ZDecoder::new();
        for (is_h2c, frames) in [(false, &c2h), (true, &h2c)] {
            for f in frames {
                let plain = if is_h2c {
                    match zcompress::decode_frame(&mut dec, &f.body) {
                        Ok(p) => p,
                        Err(_) => continue,
                    }
                } else {
                    f.body.clone()
                };
                if let Some((ty, bd)) = envelope(&plain) {
                    if ty == want {
                        println!("\n===== [{}] {} {:#x} body {}B =====", f.ts, if is_h2c { "h→c" } else { "c→h" }, ty, bd.len());
                        println!("hex: {}", hex(&bd));
                        wire_dump(&bd, 1, 4);
                        if let Some(v) = cmsg_unpack(&bd) {
                            println!("cmsg: {}", cval_display(&v));
                        }
                        for (f2, v) in parse_fields(&bd) {
                            if let FieldVal::Bytes(p) = v {
                                if let Some(v) = cmsg_unpack(&p) {
                                    println!("f{f2} cmsg: {}", cval_display(&v));
                                }
                            }
                        }
                        shown += 1;
                        if shown >= count {
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: kcp_capture_parse <stats|flow|decode|msgs> <capture.jsonl> [参数]");
        std::process::exit(1);
    }
    let (cmd, path) = (args[1].as_str(), args[2].as_str());
    let (handshakes, segs) = load(path);

    println!("=== 握手序列（前 12 条）===");
    for h in handshakes.iter().take(12) {
        println!("{h}");
    }

    match cmd {
        "stats" => cmd_stats(&segs),
        "flow" => {
            let limit: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(60);
            cmd_flow(&segs, limit);
        }
        "decode" => {
            let conv = args.get(3).and_then(|s| u32::from_str_radix(s, 16).ok());
            let limit: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
            cmd_decode(&segs, conv, limit);
        }
        "msgs" => cmd_msgs(&segs),
        "dump" => {
            let conv = args.get(3).and_then(|s| u32::from_str_radix(s, 16).ok());
            let want = u64::from_str_radix(args.get(4).map(|s| s.trim_start_matches("0x")).unwrap_or("7008"), 16)
                .or_else(|_| args.get(4).unwrap().parse())
                .expect("msg_type 解析失败");
            let count: usize = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(3);
            cmd_dump(&segs, conv, want, count);
        }
        _ => eprintln!("未知子命令: {cmd}（stats|flow|decode|msgs|dump）"),
    }
}

