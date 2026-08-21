//! 在 PE .text 中找指定字符串的 RIP 相对 xref（lea reg,[rip+disp]），打印引用点地址。
//! （find_xref.py 的 Rust 版；PE 解析手写在 examples/util，零第三方依赖）
//!
//! 用法：
//!   cargo run --example find_xref -- <PE路径> <字符串>

mod util;
use util::PeInfo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: find_xref <PE路径> <字符串>");
        std::process::exit(1);
    }
    let data = std::fs::read(&args[1]).expect("读取 PE 失败");
    let needle = args[2].as_bytes();
    let pe = PeInfo::parse(&data).expect("PE 解析失败（仅支持 PE32+）");

    let Some(off) = data
        .windows(needle.len())
        .position(|w| w == needle)
    else {
        println!("string not found");
        return;
    };
    let va = pe.image_base + pe.rva_from_offset(off as u32).expect("字符串不在任何节内") as u64;
    println!(
        "string \"{}...\" at file off {off:#x} VA {va:#x}",
        String::from_utf8_lossy(&needle[..needle.len().min(60)])
    );

    let text = pe.text_section().expect("找不到 .text 段");
    let tva = pe.image_base + text.vaddr as u64;
    let tdata = &data[text.raw_off as usize..(text.raw_off + text.raw_size) as usize];

    // 扫描 lea reg, [rip+disp32]：REX.W(48/4C) 8D /r，modrm mod=00 rm=101
    let mut found = Vec::new();
    for i in 0..tdata.len().saturating_sub(7) {
        if !matches!(tdata[i], 0x48 | 0x4C) || tdata[i + 1] != 0x8D {
            continue;
        }
        if tdata[i + 2] & 0xC7 != 0x05 {
            continue;
        }
        let disp = i32::from_le_bytes(tdata[i + 3..i + 7].try_into().unwrap()) as i64;
        let target = (tva + i as u64 + 7).wrapping_add(disp as u64);
        if target == va {
            found.push(tva + i as u64);
        }
    }
    for f in &found {
        let rva = (f - pe.image_base) as u32;
        match pe.offset_from_rva(rva) {
            Some(o) => println!("xref at VA {f:#x} (file off {o:#x})"),
            None => println!("xref at VA {f:#x}"),
        }
    }
    if found.is_empty() {
        println!("no lea xref found (string may be referenced via other means)");
    }
}
