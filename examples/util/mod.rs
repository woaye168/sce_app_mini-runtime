//! examples 共享工具：protobuf wire 解析 / PE64 头解析 / hex。
//! 仅供 examples/*.rs 以 `mod util;` 引用（cargo 不会把无 main.rs 的子目录当 example）。
#![allow(dead_code)]

// ---------------- protobuf wire ----------------

pub fn get_varint(b: &[u8], i: &mut usize) -> Option<u64> {
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

#[derive(Debug)]
pub enum FieldVal {
    Varint(u64),
    Bytes(Vec<u8>),
    I32([u8; 4]),
    I64([u8; 8]),
}

/// 解析一段 protobuf wire 数据为字段列表（遇非法结构截断，返回已解析部分）。
pub fn parse_fields(b: &[u8]) -> Vec<(u32, FieldVal)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let Some(tag) = get_varint(b, &mut i) else { break };
        let (fn_, wt) = ((tag >> 3) as u32, tag & 7);
        match wt {
            0 => {
                let Some(v) = get_varint(b, &mut i) else { break };
                out.push((fn_, FieldVal::Varint(v)));
            }
            2 => {
                let Some(ln) = get_varint(b, &mut i) else { break };
                let ln = ln as usize;
                if i + ln > b.len() {
                    break;
                }
                out.push((fn_, FieldVal::Bytes(b[i..i + ln].to_vec())));
                i += ln;
            }
            5 => {
                if i + 4 > b.len() {
                    break;
                }
                out.push((fn_, FieldVal::I32(b[i..i + 4].try_into().unwrap())));
                i += 4;
            }
            1 => {
                if i + 8 > b.len() {
                    break;
                }
                out.push((fn_, FieldVal::I64(b[i..i + 8].try_into().unwrap())));
                i += 8;
            }
            _ => break,
        }
    }
    out
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub fn is_printable(b: &[u8]) -> bool {
    !b.is_empty() && b.iter().all(|&c| (32..127).contains(&c) || matches!(c, 9 | 10 | 13))
}

// ---------------- PE64 ----------------

fn u16le(b: &[u8], o: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(o..o + 2)?.try_into().ok()?))
}
fn u32le(b: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?))
}
fn u64le(b: &[u8], o: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(o..o + 8)?.try_into().ok()?))
}

pub struct Section {
    pub name: String,
    pub vsize: u32,
    pub vaddr: u32,
    pub raw_size: u32,
    pub raw_off: u32,
}

pub struct PeInfo {
    pub image_base: u64,
    pub sections: Vec<Section>,
}

impl PeInfo {
    pub fn parse(data: &[u8]) -> Option<PeInfo> {
        let pe_off = u32le(data, 0x3C)? as usize;
        if data.get(pe_off..pe_off + 4)? != b"PE\0\0" {
            return None;
        }
        let coff = pe_off + 4;
        let nsec = u16le(data, coff + 2)? as usize;
        let opt_size = u16le(data, coff + 16)? as usize;
        let opt = coff + 20;
        if u16le(data, opt)? != 0x20B {
            return None; // 仅支持 PE32+
        }
        let image_base = u64le(data, opt + 24)?;
        let sec_tbl = opt + opt_size;
        let mut sections = Vec::with_capacity(nsec);
        for k in 0..nsec {
            let s = sec_tbl + k * 40;
            let name_bytes = data.get(s..s + 8)?;
            sections.push(Section {
                name: String::from_utf8_lossy(name_bytes)
                    .trim_end_matches('\0')
                    .to_string(),
                vsize: u32le(data, s + 8)?,
                vaddr: u32le(data, s + 12)?,
                raw_size: u32le(data, s + 16)?,
                raw_off: u32le(data, s + 20)?,
            });
        }
        Some(PeInfo {
            image_base,
            sections,
        })
    }

    pub fn rva_from_offset(&self, off: u32) -> Option<u32> {
        self.sections
            .iter()
            .find(|s| off >= s.raw_off && off < s.raw_off + s.raw_size)
            .map(|s| s.vaddr + (off - s.raw_off))
    }

    pub fn offset_from_rva(&self, rva: u32) -> Option<u32> {
        self.sections
            .iter()
            .find(|s| rva >= s.vaddr && rva < s.vaddr + s.vsize.max(s.raw_size))
            .map(|s| s.raw_off + (rva - s.vaddr))
    }

    pub fn text_section(&self) -> Option<&Section> {
        self.sections.iter().find(|s| s.name.starts_with(".text"))
    }
}
