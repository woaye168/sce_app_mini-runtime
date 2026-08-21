//! dump PE 导出表（无第三方依赖）。用法：pe_exports <dll路径> [过滤子串]

mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: pe_exports <dll路径> [过滤子串]");
        std::process::exit(1);
    }
    let data = std::fs::read(&args[1]).expect("读取失败");
    let filter = args.get(2).map(|s| s.to_lowercase());
    let pe = util::PeInfo::parse(&data).expect("PE 解析失败");
    // 导出目录：OptionalHeader.DataDirectory[0] (export table RVA/size)
    // 简化：直接从字节按 PE 结构读导出目录。这里用节数据定位。
    // Export directory RVA 在 optional header offset 96+8*0? 标准：PE32+ DD[0] at opt+112
    let pe_off = u32::from_le_bytes(data[0x3C..0x40].try_into().unwrap()) as usize;
    let opt = pe_off + 4 + 20;
    let exp_rva = u32::from_le_bytes(data[opt + 112..opt + 116].try_into().unwrap());
    let exp_size = u32::from_le_bytes(data[opt + 116..opt + 120].try_into().unwrap());
    if exp_rva == 0 {
        println!("无导出表");
        return;
    }
    let exp_off = pe.offset_from_rva(exp_rva).expect("导出目录越界") as usize;
    let d = &data[exp_off..exp_off + exp_size as usize];
    let num_names = u32::from_le_bytes(d[24..28].try_into().unwrap()) as usize;
    let names_rva = u32::from_le_bytes(d[32..36].try_into().unwrap());
    let funcs_rva = u32::from_le_bytes(d[28..32].try_into().unwrap());
    let ords_rva = u32::from_le_bytes(d[36..40].try_into().unwrap());
    let names_off = pe.offset_from_rva(names_rva).unwrap() as usize;
    let funcs_off = pe.offset_from_rva(funcs_rva).unwrap() as usize;
    let ords_off = pe.offset_from_rva(ords_rva).unwrap() as usize;
    let mut count = 0;
    for i in 0..num_names {
        let name_rva = u32::from_le_bytes(data[names_off + i * 4..names_off + i * 4 + 4].try_into().unwrap());
        let name_off = match pe.offset_from_rva(name_rva) {
            Some(o) => o as usize,
            None => continue,
        };
        let end = data[name_off..].iter().position(|&b| b == 0).unwrap_or(0);
        let name = String::from_utf8_lossy(&data[name_off..name_off + end]).to_string();
        if let Some(f) = &filter {
            if !name.to_lowercase().contains(f) {
                continue;
            }
        }
        let ord = u16::from_le_bytes(data[ords_off + i * 2..ords_off + i * 2 + 2].try_into().unwrap()) as usize;
        let func_rva = u32::from_le_bytes(data[funcs_off + ord * 4..funcs_off + ord * 4 + 4].try_into().unwrap());
        println!("{name}  (rva {func_rva:#x})");
        count += 1;
    }
    println!("---- 共 {count} / {num_names} 个导出");
}
