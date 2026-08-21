//! 解码 user_info 凭证里 token 的 kid 段（base64url）。
//! （decode_kid.py 的 Rust 版）
//!
//! 用法：
//!   cargo run --example decode_kid -- <user_info.json>

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: decode_kid <user_info.json>");
        std::process::exit(1);
    }
    let text = std::fs::read_to_string(&args[1]).expect("读取文件失败");
    let ui: serde_json::Value = serde_json::from_str(&text).expect("JSON 解析失败");
    let tok = ui["token"].as_str().expect("缺 token 字段");
    let kid = tok.splitn(2, '$').nth(1).expect("token 无 $ 分隔的 kid 段");
    let raw = URL_SAFE_NO_PAD
        .decode(kid.trim_end_matches('='))
        .expect("kid base64url 解码失败");
    println!("kid bytes len {}", raw.len());
    println!(
        "{}",
        raw[..raw.len().min(80)]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    );
    println!("{:?}", String::from_utf8_lossy(&raw[..raw.len().min(200)]));
}
