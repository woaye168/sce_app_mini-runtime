//! Entrance 直连客户端：绕过引擎直读直写星火云变量（ScoreArchive）。
//!
//! 协议依据：doc/research/lowlevel/cloudvar-04-protocol.md（帧格式）、
//!          cloudvar-05-direct-poc.md（PoC 实录）、cloudvar-06-op-matrix.md（op 码/MessagePack）。
//!
//! 用法：
//!   cargo run --release --example entrance_client -- read  <key> [key2...]
//!   cargo run --release --example entrance_client -- seti  <key> <整数>
//!   cargo run --release --example entrance_client -- sets  <key> <字符串>
//!   cargo run --release --example entrance_client -- set   <key> <json值>     # 表值写入（op0 score_set, MessagePack）
//!   cargo run --release --example entrance_client -- ladd  <key> <json值>     # 列表追加（op15 list_add, MessagePack）
//!   cargo run --release --example entrance_client -- list  <key> [limit]
//!   cargo run --release --example entrance_client -- money                 # MoneyInit（f3=10）
//!   cargo run --release --example entrance_client -- rank  <key> [start end] # 排行榜（f3=36）
//!   cargo run --release --example entrance_client -- urank <key>           # 我的名次（f3=38）
//!   cargo run --release --example entrance_client -- qitem <key>           # 物品查询（f3=70）
//!   cargo run --release --example entrance_client -- names <key> <子串>    # 名字搜索（f3=20）
//! 选项（环境变量）：
//!   ENTRANCE_URL   默认 wss://entrance-new-pd.tapsce.cn
//!   ENTRANCE_CRED  凭证 json（默认 <仓库>/runtime/User/user_info-editor-pd.spark.xd.com.json）
//!   ENTRANCE_UID   用户 id（默认 38672742）
//!   ENTRANCE_MAP   target_map（默认 ClientReadWriteMap；只读区 ClientReadonlyMap）
//!   ENTRANCE_SOURCE Msg.f1 source 名（默认 'entrance_client'）
//!   ENT_F4         登录帧 f4 flags（默认 0x1000040；isGameFlag 实验用，16 进制）
//!
//! 注意：写真实线上数据，低频使用；计数/风控维度见 cloudvar-05 §4。

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

// ---------- protobuf wire ----------
fn varint(mut v: u64) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let b = (v & 0x7F) as u8;
        v >>= 7;
        if v != 0 {
            out.push(b | 0x80);
        } else {
            out.push(b);
            break;
        }
    }
    out
}
fn f_var(fn_: u64, v: u64) -> Vec<u8> {
    [varint(fn_ << 3), varint(v)].concat()
}
fn f_bytes(fn_: u64, b: &[u8]) -> Vec<u8> {
    [varint(fn_ << 3 | 2), varint(b.len() as u64), b.to_vec()].concat()
}
fn f_str(fn_: u64, s: &str) -> Vec<u8> {
    f_bytes(fn_, s.as_bytes())
}
fn envelope(msgid: u64, body: &[u8]) -> Vec<u8> {
    [f_var(1, msgid), f_bytes(2, body)].concat()
}

fn get_varint(b: &[u8], i: &mut usize) -> Option<u64> {
    let mut v = 0u64;
    let mut s = 0u32;
    loop {
        if *i >= b.len() || s > 63 {
            return None;
        }
        let x = b[*i];
        *i += 1;
        v |= ((x & 0x7F) as u64) << s;
        if x & 0x80 == 0 {
            break;
        }
        s += 7;
    }
    Some(v)
}
/// 粗解析：返回 (字段号, varint 或 bytes)
fn parse_fields(b: &[u8]) -> Vec<(u32, u64, Option<Vec<u8>>)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let Some(tag) = get_varint(b, &mut i) else { break };
        let (fn_, wt) = ((tag >> 3) as u32, tag & 7);
        match wt {
            0 => {
                let Some(v) = get_varint(b, &mut i) else { break };
                out.push((fn_, v, None));
            }
            2 => {
                let Some(ln) = get_varint(b, &mut i) else { break };
                if i + ln as usize > b.len() {
                    break;
                }
                out.push((fn_, 0, Some(b[i..i + ln as usize].to_vec())));
                i += ln as usize;
            }
            _ => break,
        }
    }
    out
}

// ---------- MessagePack 编码（JSON→mp，复刻服务端观测编码：字符串走 bin8/bin16 家族，cloudvar-06 §1） ----------
fn mp_encode(v: &serde_json::Value, out: &mut Vec<u8>) {
    match v {
        serde_json::Value::Null => out.push(0xc0),
        serde_json::Value::Bool(b) => out.push(if *b { 0xc3 } else { 0xc2 }),
        serde_json::Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                if u <= 0x7f {
                    out.push(u as u8);
                } else if u <= 0xff {
                    out.extend([0xcc, u as u8]);
                } else if u <= 0xffff {
                    out.push(0xcd);
                    out.extend((u as u16).to_be_bytes());
                } else if u <= 0xffff_ffff {
                    out.push(0xce);
                    out.extend((u as u32).to_be_bytes());
                } else {
                    out.push(0xcf);
                    out.extend(u.to_be_bytes());
                }
            } else if let Some(i) = n.as_i64() {
                if i >= -32 && i < 0 {
                    out.push((i as i8) as u8);
                } else if i >= i8::MIN as i64 && i < 0 {
                    out.extend([0xd0, (i as i8) as u8]);
                } else if i >= i16::MIN as i64 && i <= i16::MAX as i64 {
                    out.push(0xd1);
                    out.extend((i as i16).to_be_bytes());
                } else if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    out.push(0xd2);
                    out.extend((i as i32).to_be_bytes());
                } else {
                    out.push(0xd3);
                    out.extend(i.to_be_bytes());
                }
            } else if let Some(f) = n.as_f64() {
                out.push(0xcb);
                out.extend(f.to_be_bytes());
            }
        }
        serde_json::Value::String(s) => {
            let b = s.as_bytes();
            if b.len() <= 0xff {
                out.extend([0xc4, b.len() as u8]);
            } else if b.len() <= 0xffff {
                out.push(0xc5);
                out.extend((b.len() as u16).to_be_bytes());
            } else {
                out.push(0xc6);
                out.extend((b.len() as u32).to_be_bytes());
            }
            out.extend(b);
        }
        serde_json::Value::Array(a) => {
            if a.len() <= 15 {
                out.push(0x90 | a.len() as u8);
            } else if a.len() <= 0xffff {
                out.push(0xdc);
                out.extend((a.len() as u16).to_be_bytes());
            } else {
                out.push(0xdd);
                out.extend((a.len() as u32).to_be_bytes());
            }
            for x in a {
                mp_encode(x, out);
            }
        }
        serde_json::Value::Object(m) => {
            if m.len() <= 15 {
                out.push(0x80 | m.len() as u8);
            } else if m.len() <= 0xffff {
                out.push(0xde);
                out.extend((m.len() as u16).to_be_bytes());
            } else {
                out.push(0xdf);
                out.extend((m.len() as u32).to_be_bytes());
            }
            for (k, x) in m {
                mp_encode(&serde_json::Value::String(k.clone()), out);
                mp_encode(x, out);
            }
        }
    }
}

// ---------- ScoreArchive 消息 ----------
fn msg(source: &str, target_map: &str, sub_type: u64, sub_body: &[u8]) -> Vec<u8> {
    [
        f_str(1, source),
        f_str(2, target_map),
        f_var(3, sub_type),
        f_bytes(4, sub_body),
        f_bytes(5, b""),
        f_var(6, 0),
    ]
    .concat()
}
fn score_init(uid: u64, keys: &[String]) -> Vec<u8> {
    let mut sub = f_bytes(1, &varint(uid));
    for k in keys {
        sub.extend(f_str(2, k));
    }
    sub
}
fn commit(ops: Vec<Vec<u8>>, desc: &str) -> Vec<u8> {
    let mut body = Vec::new();
    for op in ops {
        body.extend(f_bytes(1, &op));
    }
    body.extend(f_str(2, desc));
    body
}
fn op_seti(key: &str, uid: u64, val: i64) -> Vec<u8> {
    [f_str(1, key), f_var(2, 3), f_var(3, uid), f_var(5, val as u64)].concat()
}
fn op_sets(key: &str, uid: u64, val: &str) -> Vec<u8> {
    [f_str(1, key), f_var(2, 7), f_var(3, uid), f_str(6, val)].concat()
}
/// op0 score_set 任意值（f4 = MessagePack 编码）
fn op_set_mp(key: &str, uid: u64, mp: &[u8]) -> Vec<u8> {
    [f_str(1, key), f_var(2, 0), f_var(3, uid), f_bytes(4, mp)].concat()
}
/// op15 list_add（f4 = MessagePack 编码值）
fn op_ladd_mp(key: &str, uid: u64, mp: &[u8]) -> Vec<u8> {
    [f_str(1, key), f_var(2, 15), f_var(3, uid), f_bytes(4, mp)].concat()
}
/// op13 money_add（f5 varint 金额，cloudvar-06 §1）
fn op_money_add(key: &str, uid: u64, amount: i64) -> Vec<u8> {
    [f_str(1, key), f_var(2, 13), f_var(3, uid), f_var(5, amount as u64)].concat()
}
/// op18 item_add（f4=msgpack extra, f5=count, f7=item_name, f9=expire_type, f10=expire_time, cloudvar-06 §1）
fn op_item_add(key: &str, uid: u64, item_name: &str, count: u64, extra_mp: &[u8]) -> Vec<u8> {
    [
        f_str(1, key),
        f_var(2, 18),
        f_var(3, uid),
        f_bytes(4, extra_mp),
        f_var(5, count),
        f_str(7, item_name),
        f_var(9, 0),
        f_str(10, "\"9999-12-31 23:59:59\""),
    ]
    .concat()
}
fn query_list(uid: u64, key: &str, limit: u64) -> Vec<u8> {
    [f_var(1, uid), f_str(2, key), f_var(3, limit)].concat()
}
/// MoneyInit（f3=10，cloudvar-06 §2）
fn money_init(uid: u64) -> Vec<u8> {
    f_var(1, uid)
}
/// QueryRankList（f3=36）：{f1 key, f3 start, f4 end, f5 'iscore'}
fn query_rank_list(key: &str, start: u64, end: u64) -> Vec<u8> {
    [f_str(1, key), f_var(3, start), f_var(4, end), f_str(5, "iscore")].concat()
}
/// QueryRank（f3=38）：{f1 key, f2 'iscore'}
fn query_rank(key: &str) -> Vec<u8> {
    [f_str(1, key), f_str(2, "iscore")].concat()
}
/// QueryItem（f3=70）：{f1 uid, f2 key}
fn query_item(uid: u64, key: &str) -> Vec<u8> {
    [f_var(1, uid), f_str(2, key)].concat()
}
/// NameSearch（f3=20）：{f1 key, f2 name_substr}
fn name_search(key: &str, substr: &str) -> Vec<u8> {
    [f_str(1, key), f_str(2, substr)].concat()
}

fn show_resp(body: &[u8]) {
    let f = parse_fields(body);
    let sub_type = f.iter().find(|(n, _, _)| *n == 3).map(|(_, v, _)| *v);
    let sub_body = f
        .iter()
        .find(|(n, _, b)| *n == 4 && b.is_some())
        .and_then(|(_, _, b)| b.clone())
        .unwrap_or_default();
    match sub_type {
        Some(100) => {
            let ef = parse_fields(&sub_body);
            let code = ef.iter().find(|(n, _, _)| *n == 1).map(|(_, v, _)| *v).unwrap_or(u64::MAX);
            let reason = ef
                .iter()
                .find(|(n, _, b)| *n == 2 && b.is_some())
                .and_then(|(_, _, b)| b.clone())
                .map(|b| String::from_utf8_lossy(&b).into_owned());
            println!("  Result: code={} reason={:?}", code as i64, reason);
        }
        Some(t) => println!("  subtype={} body={}", t, sub_body.iter().map(|b| format!("{b:02x}")).collect::<String>()),
        None => println!("  raw={}", body.iter().map(|b| format!("{b:02x}")).collect::<String>()),
    }
}

async fn recv_until(ws: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin), want: u64, n: usize) -> Vec<Vec<u8>> {
    let mut got = Vec::new();
    for _ in 0..n {
        let m = tokio::time::timeout(std::time::Duration::from_secs(8), ws.next()).await;
        let Ok(Some(Ok(Message::Binary(raw)))) = m else { break };
        let f = parse_fields(&raw);
        let mid = f.iter().find(|(n, _, _)| *n == 1).map(|(_, v, _)| *v).unwrap_or(0);
        let body = f
            .iter()
            .find(|(n, _, b)| *n == 2 && b.is_some())
            .and_then(|(_, _, b)| b.clone())
            .unwrap_or_default();
        println!("<< msgid={mid:#x} bodylen={}", body.len());
        if mid == 0xA000 {
            show_resp(&body);
        }
        if mid == want {
            got.push(body);
        }
    }
    got
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("用法: entrance_client <read|seti|sets|list> ...（详见文件头注释）");
        std::process::exit(1);
    }
    let url = std::env::var("ENTRANCE_URL").unwrap_or_else(|_| "wss://entrance-new-pd.tapsce.cn".into());
    let cred_path = std::env::var("ENTRANCE_CRED").unwrap_or_else(|_| {
        concat!(env!("CARGO_MANIFEST_DIR"), "/runtime/User/user_info-editor-pd.spark.xd.com.json").into()
    });
    let uid: u64 = std::env::var("ENTRANCE_UID").ok().and_then(|s| s.parse().ok()).unwrap_or(38672742);
    let map = std::env::var("ENTRANCE_MAP").unwrap_or_else(|_| "ClientReadWriteMap".into());
    let source = std::env::var("ENTRANCE_SOURCE").unwrap_or_else(|_| "entrance_client".into());
    // 登录 f4 flags（默认 0x1000040 = editor lobby 观测值；tester isGameFlag:true 的 flags 待对照，可用此变量实验）
    let login_f4: u64 = std::env::var("ENT_F4").ok().and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()).unwrap_or(0x1000040);
    // 登录帧追加字段（hex，如 '1801'=f3 varint 1、'2801'=f5 varint 1；isGameFlag 字段定位实验用）
    let login_extra: Vec<u8> = std::env::var("ENT_LOGIN_EXTRA").ok().and_then(|s| (0..s.len().saturating_sub(1)).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok()).collect()).unwrap_or_default();

    let cred: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&cred_path).expect("读凭证失败")).expect("凭证 json 解析失败");
    let token = cred["token"].as_str().expect("凭证缺 token 字段").to_string();
    let token_type: u64 = cred["token_type"].as_u64().unwrap_or(11);

    let (mut ws, _) = tokio_tungstenite::connect_async(&url).await.expect("wss 连接失败");
    println!("connected {url}");

    // 登录（结构见 cloudvar-04 §3）
    let login = [
        f_str(1, "default"),
        f_bytes(2, b""),
        f_var(4, login_f4),
        f_var(6, token_type),
        f_str(7, &token),
        f_var(19, 1),
        f_bytes(20, b""),
        login_extra.clone(),
    ]
    .concat();
    ws.send(Message::Binary(envelope(0x0001, &login))).await.expect("发送登录失败");
    println!(">> login sent");
    let resp = recv_until(&mut ws, 0x0002, 6).await;
    if resp.is_empty() {
        eprintln!("登录响应超时");
        std::process::exit(1);
    }
    println!("login ok");

    let send_score = |sub_type: u64, sub_body: &[u8]| {
        let m = msg(&source, &map, sub_type, sub_body);
        envelope(0xA000, &m)
    };

    match args[0].as_str() {
        "read" => {
            let keys = &args[1..];
            if keys.is_empty() {
                eprintln!("read 需要至少一个 key");
                std::process::exit(1);
            }
            ws.send(Message::Binary(send_score(4, &score_init(uid, keys)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "seti" | "sets" => {
            let (key, val) = (args.get(1).expect("缺 key"), args.get(2).expect("缺 value"));
            let op = if args[0] == "seti" {
                op_seti(key, uid, val.parse().expect("value 需整数"))
            } else {
                op_sets(key, uid, val)
            };
            ws.send(Message::Binary(send_score(2, &commit(vec![op], "entrance_client")))).await.unwrap();
            println!(">> commit sent");
            recv_until(&mut ws, 0xA000, 3).await;
            // 复读验证
            ws.send(Message::Binary(send_score(4, &score_init(uid, std::slice::from_ref(key))))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "list" => {
            let key = args.get(1).expect("缺 key");
            let limit: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);
            ws.send(Message::Binary(send_score(48, &query_list(uid, key, limit)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "set" | "ladd" => {
            let (key, val) = (args.get(1).expect("缺 key"), args.get(2).expect("缺 json value"));
            let json: serde_json::Value = serde_json::from_str(val).expect("value 需合法 json");
            let mut mp = Vec::new();
            mp_encode(&json, &mut mp);
            println!(">> msgpack: {}", mp.iter().map(|b| format!("{b:02x}")).collect::<String>());
            let op = if args[0] == "set" { op_set_mp(key, uid, &mp) } else { op_ladd_mp(key, uid, &mp) };
            ws.send(Message::Binary(send_score(2, &commit(vec![op], "entrance_client")))).await.unwrap();
            println!(">> commit sent");
            recv_until(&mut ws, 0xA000, 3).await;
            // 复读验证（ladd 用 list 查）
            if args[0] == "set" {
                ws.send(Message::Binary(send_score(4, &score_init(uid, std::slice::from_ref(key))))).await.unwrap();
            } else {
                ws.send(Message::Binary(send_score(48, &query_list(uid, key, 5)))).await.unwrap();
            }
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "money" => {
            ws.send(Message::Binary(send_score(10, &money_init(uid)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "rank" => {
            let key = args.get(1).expect("缺 key");
            let start: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
            let end: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(10);
            ws.send(Message::Binary(send_score(36, &query_rank_list(key, start, end)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "urank" => {
            let key = args.get(1).expect("缺 key");
            ws.send(Message::Binary(send_score(38, &query_rank(key)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "qitem" => {
            let key = args.get(1).expect("缺 key");
            ws.send(Message::Binary(send_score(70, &query_item(uid, key)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "names" => {
            let key = args.get(1).expect("缺 key");
            let substr = args.get(2).expect("缺 name_substr");
            ws.send(Message::Binary(send_score(20, &name_search(key, substr)))).await.unwrap();
            recv_until(&mut ws, 0xA000, 3).await;
        }
        // 读限流压测：单连接连发 n 次 score_init，统计响应码分布与吞吐（验证 300/min 计数维度）
        "burst" => {
            let key = args.get(1).expect("缺 key");
            let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(350);
            let t0 = std::time::Instant::now();
            for _ in 0..n {
                ws.send(Message::Binary(send_score(4, &score_init(uid, std::slice::from_ref(key))))).await.unwrap();
            }
            let send_ms = t0.elapsed().as_millis();
            println!(">> {n} reads sent in {send_ms}ms");
            let mut ok = 0usize;
            let mut codes: std::collections::BTreeMap<i64, (usize, Option<String>)> = Default::default();
            let mut got = 0usize;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            while got < n && std::time::Instant::now() < deadline {
                let m = tokio::time::timeout(std::time::Duration::from_secs(8), ws.next()).await;
                let Ok(Some(Ok(Message::Binary(raw)))) = m else { break };
                let f = parse_fields(&raw);
                let mid = f.iter().find(|(n_, _, _)| *n_ == 1).map(|(_, v, _)| *v).unwrap_or(0);
                if mid != 0xA000 { continue };
                got += 1;
                let body = f.iter().find(|(n_, _, b)| *n_ == 2 && b.is_some()).and_then(|(_, _, b)| b.clone()).unwrap_or_default();
                let mf = parse_fields(&body);
                let st = mf.iter().find(|(n_, _, _)| *n_ == 3).map(|(_, v, _)| *v);
                let sb = mf.iter().find(|(n_, _, b)| *n_ == 4 && b.is_some()).and_then(|(_, _, b)| b.clone()).unwrap_or_default();
                if st == Some(100) {
                    let ef = parse_fields(&sb);
                    let code = ef.iter().find(|(n_, _, _)| *n_ == 1).map(|(_, v, _)| *v as i64).unwrap_or(i64::MAX);
                    let reason = ef.iter().find(|(n_, _, b)| *n_ == 2 && b.is_some()).and_then(|(_, _, b)| b.clone()).map(|b| String::from_utf8_lossy(&b).into_owned());
                    let e = codes.entry(code).or_insert((0, reason));
                    e.0 += 1;
                } else {
                    ok += 1;
                }
            }
            let total_ms = t0.elapsed().as_millis();
            println!("burst done: {got}/{n} resp in {total_ms}ms ({:.1} ops/s), ok={ok}", got as f64 / (total_ms as f64 / 1000.0));
            for (code, (cnt, reason)) in &codes {
                println!("  code={code} x{cnt} reason={reason:?}");
            }
        }
        "madd" => {
            // money_add 写货币（op13）：madd <currency> <amount>
            let (key, amount) = (args.get(1).expect("缺 currency"), args.get(2).expect("缺 amount"));
            let op = op_money_add(key, uid, amount.parse().expect("amount 需整数"));
            ws.send(Message::Binary(send_score(2, &commit(vec![op], "entrance_client")))).await.unwrap();
            println!(">> money_add commit sent");
            recv_until(&mut ws, 0xA000, 3).await;
        }
        "iadd" => {
            // item_add 写物品（op18）：iadd <key> <item_name> [count] [extra_json]
            let (key, item) = (args.get(1).expect("缺 key"), args.get(2).expect("缺 item_name"));
            let count: u64 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
            let mut mp = Vec::new();
            if let Some(j) = args.get(4) {
                let json: serde_json::Value = serde_json::from_str(j).expect("extra 需合法 json");
                mp_encode(&json, &mut mp);
            }
            let op = op_item_add(key, uid, item, count, &mp);
            ws.send(Message::Binary(send_score(2, &commit(vec![op], "entrance_client")))).await.unwrap();
            println!(">> item_add commit sent");
            recv_until(&mut ws, 0xA000, 3).await;
        }
        other => {
            eprintln!("未知子命令 {other}（read|seti|sets|set|ladd|list|money|rank|urank|qitem|names|burst|madd|iadd）");
            std::process::exit(1);
        }
    }
    println!("done");
}
