//! 从指定文件偏移向前 back 字节开始线性反汇编到偏移处（x64）。
//! （disasm_at.py 的 Rust 版；反汇编用 capstone crate）
//!
//! 用法：
//!   cargo run --example disasm_at -- <PE路径> <偏移hex> <向前字节数hex>
//! 示例：
//!   cargo run --example disasm_at -- sceengine.dll 0x1a2b3c 0x200

mod util;
use capstone::prelude::*;
use util::PeInfo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("用法: disasm_at <PE路径> <偏移hex> <向前字节数hex>");
        std::process::exit(1);
    }
    let data = std::fs::read(&args[1]).expect("读取 PE 失败");
    let off = usize::from_str_radix(args[2].trim_start_matches("0x"), 16).expect("偏移解析失败");
    let back = usize::from_str_radix(args[3].trim_start_matches("0x"), 16).expect("back 解析失败");
    let pe = PeInfo::parse(&data).expect("PE 解析失败（仅支持 PE32+）");

    if off >= data.len() {
        eprintln!("偏移 {off:#x} 超出文件大小 {:#x}", data.len());
        std::process::exit(1);
    }
    let start = off.saturating_sub(back).min(data.len());
    let blob = &data[start..(off + 0x40).min(data.len())];
    let base = pe.image_base + pe.rva_from_offset(start as u32).expect("起点不在任何节内") as u64;

    let cs = Capstone::new()
        .x86()
        .mode(capstone::arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()
        .expect("capstone 初始化失败");
    for ins in cs.disasm_all(blob, base).expect("反汇编失败").iter() {
        println!(
            "{:#x}  {:10} {}",
            ins.address(),
            ins.mnemonic().unwrap_or(""),
            ins.op_str().unwrap_or("")
        );
    }
}
