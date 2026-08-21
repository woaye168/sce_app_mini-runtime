//! dump PE 导入表（无第三方依赖）。用法：pe_imports <exe/dll路径> [过滤子串]

mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: pe_imports <exe/dll路径> [过滤子串]");
        std::process::exit(1);
    }
    let data = std::fs::read(&args[1]).expect("读取失败");
    let filter = args.get(2).map(|s| s.to_lowercase());
    let pe = util::PeInfo::parse(&data).expect("PE 解析失败");
    let pe_off = u32::from_le_bytes(data[0x3C..0x40].try_into().unwrap()) as usize;
    let opt = pe_off + 4 + 20;
    // Import Directory = DataDirectory[1]：PE32+ 在 opt+112+8
    let imp_rva = u32::from_le_bytes(data[opt + 120..opt + 124].try_into().unwrap());
    if imp_rva == 0 {
        println!("无导入表");
        return;
    }
    let mut off = pe.offset_from_rva(imp_rva).expect("导入目录越界") as usize;
    loop {
        let ilt_rva = u32::from_le_bytes(data[off..off + 4].try_into().unwrap());
        let name_rva = u32::from_le_bytes(data[off + 12..off + 16].try_into().unwrap());
        let iat_rva = u32::from_le_bytes(data[off + 16..off + 20].try_into().unwrap());
        if ilt_rva == 0 && name_rva == 0 {
            break;
        }
        let dll_name = {
            let no = pe.offset_from_rva(name_rva).unwrap() as usize;
            let end = data[no..].iter().position(|&b| b == 0).unwrap_or(0);
            String::from_utf8_lossy(&data[no..no + end]).to_string()
        };
        if let Some(f) = &filter {
            if !dll_name.to_lowercase().contains(f) {
                off += 20;
                continue;
            }
        }
        println!("== {dll_name}");
        // 遍历 ILT（原 First Thunk），每个是 8 字节（PE32+）
        let ilt_off = pe.offset_from_rva(if ilt_rva != 0 { ilt_rva } else { iat_rva }).unwrap() as usize;
        let mut i = 0;
        loop {
            let e = u64::from_le_bytes(data[ilt_off + i * 8..ilt_off + i * 8 + 8].try_into().unwrap());
            if e == 0 {
                break;
            }
            // 高位=ordinal 导入；否则 Hint/Name RVA
            if e & 0x8000_0000_0000_0000 != 0 {
                println!("  ordinal #{}", e & 0xFFFF);
            } else {
                let hn = e as u32;
                if let Some(hno) = pe.offset_from_rva(hn) {
                    let hno = hno as usize;
                    let name_start = hno + 2; // 跳过 2 字节 hint
                    let end = data[name_start..].iter().position(|&b| b == 0).unwrap_or(0);
                    let fname = String::from_utf8_lossy(&data[name_start..name_start + end]).to_string();
                    if let Some(f) = &filter {
                        if fname.to_lowercase().contains(f) || dll_name.to_lowercase().contains(f) {
                            println!("  {fname}");
                        }
                    } else {
                        println!("  {fname}");
                    }
                }
            }
            i += 1;
        }
        off += 20;
    }
}
