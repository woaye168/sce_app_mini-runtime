//! luaL_Reg 注册表导出 + capstone 签名推断。
//! 机制：锚点字符串 → VA → 全文件扫 8 字节引用定位注册表条目 → 前后走表 →
//! 对每个函数反汇编，跟踪 `mov edx, imm` + `call [rip+x]`(IAT) 命中 lua54 取参 API，
//! 推断参数下标/类型；最后一个 `mov eax, imm` 近似返回值个数。
//!
//! 用法：
//!   lua_api_dump <PE路径> <锚点字符串>
//! 例：
//!   lua_api_dump D:/sce_online/version-13/sceengine.dll get_app_dir
//!   lua_api_dump D:/sce_pc_tester/tester_1089/Win/scegame get_app_dir

mod util;

use std::collections::HashMap;
use util::PeInfo;

fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64le(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}

/// 导入表 → IAT 槽位 VA → "dll!func"
fn parse_iat(data: &[u8], pe: &PeInfo) -> HashMap<u64, String> {
    let mut map = HashMap::new();
    let pe_off = u32le(data, 0x3C) as usize;
    let opt = pe_off + 4 + 20;
    let imp_rva = u32le(data, opt + 120);
    if imp_rva == 0 {
        return map;
    }
    let mut off = match pe.offset_from_rva(imp_rva) {
        Some(o) => o as usize,
        None => return map,
    };
    loop {
        let ilt_rva = u32le(data, off);
        let name_rva = u32le(data, off + 12);
        let iat_rva = u32le(data, off + 16);
        if ilt_rva == 0 && name_rva == 0 {
            break;
        }
        let dll = pe
            .offset_from_rva(name_rva)
            .map(|no| {
                let no = no as usize;
                let end = data[no..].iter().position(|&b| b == 0).unwrap_or(0);
                String::from_utf8_lossy(&data[no..no + end]).to_string()
            })
            .unwrap_or_default();
        let ilt_off = pe
            .offset_from_rva(if ilt_rva != 0 { ilt_rva } else { iat_rva })
            .map(|o| o as usize);
        if let Some(mut io) = ilt_off {
            let mut i = 0u32;
            loop {
                let e = u64le(data, io);
                if e == 0 {
                    break;
                }
                if e & 0x8000_0000_0000_0000 == 0 {
                    if let Some(hno) = pe.offset_from_rva(e as u32) {
                        let ns = hno as usize + 2;
                        let end = data[ns..].iter().position(|&b| b == 0).unwrap_or(0);
                        let fname = String::from_utf8_lossy(&data[ns..ns + end]).to_string();
                        let slot_va = pe.image_base + (iat_rva + i * 8) as u64;
                        map.insert(slot_va, format!("{dll}!{fname}"));
                    }
                }
                io += 8;
                i += 1;
            }
        }
        off += 20;
    }
    map
}

fn cstr_at(data: &[u8], off: usize) -> Option<String> {
    let end = data.get(off..)?.iter().position(|&b| b == 0)?;
    let s = &data[off..off + end];
    if s.len() < 2 || s.len() > 64 || !s.iter().all(|&c| (32..127).contains(&c)) {
        return None;
    }
    Some(String::from_utf8_lossy(s).to_string())
}

fn va_to_off(pe: &PeInfo, va: u64) -> Option<usize> {
    if va < pe.image_base {
        return None;
    }
    pe.offset_from_rva((va - pe.image_base) as u32).map(|o| o as usize)
}

/// 条目合法性：name 指向可打印 C 串，func 在 .text
fn entry_valid(data: &[u8], pe: &PeInfo, off: usize, text: &util::Section) -> bool {
    if off + 16 > data.len() {
        return false;
    }
    let name_va = u64le(data, off);
    let func_va = u64le(data, off + 8);
    let no = match va_to_off(pe, name_va) {
        Some(o) => o,
        None => return false,
    };
    if cstr_at(data, no).is_none() {
        return false;
    }
    let frva = (func_va.wrapping_sub(pe.image_base)) as u64;
    frva >= text.vaddr as u64 && frva < (text.vaddr + text.vsize.max(text.raw_size)) as u64
}

/// lua 取参 API → (类型, 是否可选)
fn arg_kind(func: &str) -> Option<(&'static str, bool)> {
    let n = func.rsplit('!').next().unwrap_or(func);
    Some(match n {
        "luaL_checklstring" | "lua_tolstring" | "lua_tostring" | "luaL_tolstring" => {
            ("string", false)
        }
        "luaL_optlstring" => ("string", true),
        "luaL_checknumber" | "lua_tonumberx" | "lua_tonumber" => ("number", false),
        "luaL_optnumber" => ("number", true),
        "luaL_checkinteger" | "lua_tointegerx" | "lua_tointeger" => ("integer", false),
        "luaL_optinteger" => ("integer", true),
        "lua_toboolean" => ("boolean", false),
        "luaL_checkudata" | "lua_touserdata" => ("userdata", false),
        "luaL_checkany" => ("any", false),
        "lua_isfunction" | "lua_tocfunction" => ("function", false),
        "lua_istable" => ("table", false),
        "luaL_checktype" => ("typed", false),
        _ => return None,
    })
}

/// 把 call 目标解析成导入名：直接 IAT 调用 或 跳板 stub（FF 25 jmp [rip+disp]）
fn resolve_import(
    data: &[u8],
    pe: &PeInfo,
    iat: &HashMap<u64, String>,
    tgt: u64,
) -> Option<String> {
    if let Some(n) = iat.get(&tgt) {
        return Some(n.clone());
    }
    let off = va_to_off(pe, tgt)?;
    if data.get(off) == Some(&0xFF) && data.get(off + 1) == Some(&0x25) {
        let disp = u32le(data, off + 2) as u64;
        let slot = tgt + 6 + disp;
        return iat.get(&slot).cloned();
    }
    None
}

/// 注册表函数若是 thunk（头部即 jmp 真实实现），跟进到实现体
fn resolve_thunk(data: &[u8], pe: &PeInfo, mut va: u64) -> u64 {
    use capstone::prelude::*;
    let cs = match Capstone::new().x86().mode(capstone::arch::x86::ArchMode::Mode64).build() {
        Ok(c) => c,
        Err(_) => return va,
    };
    for _ in 0..2 {
        let Some(off) = va_to_off(pe, va) else { return va };
        let code = &data[off..(off + 0x30).min(data.len())];
        let Ok(insns) = cs.disasm_all(code, va) else { return va };
        let mut next = None;
        for ins in insns.iter() {
            if ins.mnemonic() == Some("jmp") {
                if let Some(t) = parse_imm(ins.op_str().unwrap_or("")) {
                    if t > 0 {
                        next = Some(t as u64);
                    }
                }
                break; // thunk 的 jmp 是收尾指令
            }
            if ins.mnemonic() == Some("ret") {
                break;
            }
        }
        match next {
            Some(t) => va = t,
            None => return va,
        }
    }
    va
}

/// capstone 推断单个 lua C 函数签名（先过 thunk，再跟踪 edx 下标 + 取参调用）
fn infer_sig(data: &[u8], pe: &PeInfo, iat: &HashMap<u64, String>, func_va: u64) -> String {
    use capstone::prelude::*;
    let impl_va = resolve_thunk(data, pe, func_va);
    let off = match va_to_off(pe, impl_va) {
        Some(o) => o,
        None => return "?".into(),
    };
    let code = &data[off..(off + 2500).min(data.len())];
    let cs = match Capstone::new().x86().mode(capstone::arch::x86::ArchMode::Mode64).build() {
        Ok(c) => c,
        Err(_) => return "?".into(),
    };
    let insns = match cs.disasm_all(code, impl_va) {
        Ok(i) => i,
        Err(_) => return "?".into(),
    };
    let mut args: HashMap<i64, (&str, bool)> = HashMap::new();
    let mut last_edx: Option<i64> = None;
    let mut last_eax: Option<i64> = None;
    for ins in insns.iter() {
        let mn = ins.mnemonic().unwrap_or("");
        let ops = ins.op_str().unwrap_or("");
        match mn {
            "mov" => {
                if let Some(v) = ops.strip_prefix("edx, ").and_then(parse_imm) {
                    last_edx = Some(v);
                }
                if let Some(v) = ops.strip_prefix("eax, ").and_then(parse_imm) {
                    last_eax = Some(v);
                }
            }
            "xor" if ops == "edx, edx" => last_edx = Some(0),
            "call" => {
                let import = if let Some(rest) = ops.strip_prefix("qword ptr [rip + ") {
                    parse_imm(rest.trim_end_matches(']'))
                        .map(|d| ins.address() + ins.len() as u64 + d as u64)
                        .and_then(|slot| iat.get(&slot).cloned())
                } else if let Some(rest) = ops.strip_prefix("qword ptr [rip - ") {
                    parse_imm(rest.trim_end_matches(']'))
                        .map(|d| (ins.address() + ins.len() as u64).wrapping_sub(d as u64))
                        .and_then(|slot| iat.get(&slot).cloned())
                } else {
                    // near call：可能是跳板 stub
                    parse_imm(ops)
                        .filter(|&t| t > 0)
                        .and_then(|t| resolve_import(data, pe, iat, t as u64))
                };
                if let Some(name) = import {
                    if let Some((ty, opt)) = arg_kind(&name) {
                        if let Some(idx) = last_edx {
                            if idx >= 1 {
                                args.entry(idx).or_insert((ty, opt));
                            }
                        }
                    }
                }
            }
            "ret" => break,
            _ => {}
        }
    }
    let mut idxs: Vec<i64> = args.keys().cloned().collect();
    idxs.sort();
    let params = idxs
        .iter()
        .map(|i| {
            let (ty, opt) = args[i];
            if opt { format!("{ty}?") } else { ty.to_string() }
        })
        .collect::<Vec<_>>()
        .join(", ");
    let ret = last_eax.map(|v| v.to_string()).unwrap_or("?".into());
    format!("({params}) -> {ret}")
}

fn parse_imm(s: &str) -> Option<i64> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x") {
        i64::from_str_radix(h, 16).ok()
    } else {
        s.parse().ok()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: lua_api_dump <PE路径> <锚点字符串>");
        std::process::exit(1);
    }
    let data = std::fs::read(&args[1]).expect("读取 PE 失败");
    let pe = PeInfo::parse(&data).expect("PE 解析失败");
    let text = pe.text_section().expect("无 .text");
    let iat = parse_iat(&data, &pe);

    // 锚点字符串（可能有多个副本）→ VA → 引用条目；取第一个有 qword 引用的副本
    let needle = args[2].as_bytes();
    let mut entry = None;
    let mut sva_used = 0u64;
    let mut pos = 0usize;
    while let Some(rel) = data[pos..].windows(needle.len()).position(|w| w == needle) {
        let soff = pos + rel;
        pos = soff + 1;
        let Some(srva) = pe.rva_from_offset(soff as u32) else { continue };
        let sva = pe.image_base + srva as u64;
        let nb = sva.to_le_bytes();
        for i in 0..data.len().saturating_sub(8) {
            if data[i..i + 8] == nb {
                entry = Some(i);
                sva_used = sva;
                break;
            }
        }
        if entry.is_some() {
            break;
        }
    }
    let entry = entry.expect("锚点无注册表引用");
    eprintln!("anchor \"{}\" VA {sva_used:#x}, table entry file off {entry:#x}", args[2]);

    // 回退到表头
    let mut start = entry;
    while start >= 16 && entry_valid(&data, &pe, start - 16, &text) {
        start -= 16;
    }
    // 前推到结束
    let mut entries = Vec::new();
    let mut o = start;
    while entry_valid(&data, &pe, o, &text) {
        let name = cstr_at(&data, va_to_off(&pe, u64le(&data, o)).unwrap()).unwrap();
        let func_va = u64le(&data, o + 8);
        entries.push((name, func_va));
        o += 16;
    }
    eprintln!("table: {} entries", entries.len());

    let mut cache: HashMap<u64, String> = HashMap::new();
    println!("name\tfunc_rva\tsignature");
    for (name, fva) in &entries {
        let sig = cache
            .entry(*fva)
            .or_insert_with(|| infer_sig(&data, &pe, &iat, *fva))
            .clone();
        println!("{}\t{:#x}\t{}", name, fva - pe.image_base, sig);
    }
}
