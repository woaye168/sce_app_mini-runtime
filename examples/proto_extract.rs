//! 从 protobuf C++ 二进制（sceengine.dll 等）提取内嵌的 FileDescriptorProto。
//! （proto_extract.py 的 Rust 版；解析改用 prost-types，输出 Debug 格式）
//!
//! 原理：protoc 生成的代码内嵌序列化 FileDescriptorProto，
//! blob 以 0x0A <varint len> "<name>.proto" 开头。
//!
//! 用法：
//!   cargo run --example proto_extract -- <二进制路径> <输出目录> [名字过滤子串]

use prost::Message;

fn get_varint(b: &[u8], i: &mut usize) -> Option<u64> {
    let mut v: u64 = 0;
    let mut s: u32 = 0;
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: proto_extract <二进制路径> <输出目录> [名字过滤子串]");
        std::process::exit(1);
    }
    let (path, out_dir) = (&args[1], &args[2]);
    let filter = args.get(3).map(String::as_str);
    let data = std::fs::read(path).expect("读取二进制失败");
    println!("file size: {}", data.len());

    // 全部 ".proto" 命中点
    let mut hits = Vec::new();
    let mut pos = 0usize;
    while let Some(i) = data[pos..]
        .windows(6)
        .position(|w| w == b".proto")
        .map(|x| x + pos)
    {
        hits.push(i);
        pos = i + 1;
    }
    println!(".proto hits: {}", hits.len());

    std::fs::create_dir_all(out_dir).expect("创建输出目录失败");
    let mut seen = std::collections::HashSet::new();
    for &h in &hits {
        // 向前找文件名字符串起点（可打印字符）
        let mut s = h;
        while s > 0 && (32..127).contains(&data[s - 1]) {
            s -= 1;
        }
        let fname = &data[s..h + 6];
        // blob 起点 = 0x0A <varint len> 正好编码 fname 长度
        let mut start = None;
        for back in 1..6usize {
            if s < back {
                break;
            }
            let p = s - back;
            if data[p] != 0x0A {
                continue;
            }
            let mut q = p + 1;
            if let Some(val) = get_varint(&data, &mut q) {
                if val as usize == fname.len() && q == s {
                    start = Some(p);
                    break;
                }
            }
        }
        let Some(start) = start else { continue };

        // 从 flen+2 开始逐步扩大（步长 64）到首个可完整解析的长度
        let hi = (data.len() - start).min(512 * 1024);
        let mut n = fname.len() + 2;
        let mut fd = None;
        while n <= hi {
            if let Ok(f) = prost_types::FileDescriptorProto::decode(&data[start..start + n]) {
                fd = Some(f);
                break;
            }
            n += 64;
        }
        let Some(f) = fd else { continue };
        let name = f.name.clone().unwrap_or_default();
        if name.is_empty() || !seen.insert(name.clone()) {
            continue;
        }
        if let Some(ft) = filter {
            if !name.contains(ft) {
                continue;
            }
        }
        let out = format!("{}/{}.txt", out_dir, name.replace('/', "_"));
        std::fs::write(&out, format!("{f:#?}")).expect("写出失败");
        println!(
            "extracted: {name} ({} messages, {} enums) -> {out}",
            f.message_type.len(),
            f.enum_type.len()
        );
    }
}
