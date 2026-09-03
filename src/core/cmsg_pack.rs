//! cmsg_pack：msgpack 变体（字符串走 bin 家族 0xc4/0xc5/0xc6）——玩法消息体序列化。
//! 字节级契约已经 VM oracle 实证（`cmsg_pack.pack({type='Req_PlayerList',args={}})` 与抓包逐字节一致，
//! 见 doc/research/scegame-reverse.md §13.5）。零依赖纯算法。

/// cmsg_pack 值（pack 的输入 / unpack 的输出）
#[derive(Debug, Clone, PartialEq)]
pub enum CVal {
    Nil,
    Bool(bool),
    /// 整数（pack 时按最小宽度编码：正数 fixint/u8/u16/u32/u64，负数 i8/i16/i32/i64）
    Int(i64),
    U64(u64),
    F64(f64),
    /// 字符串（线格式 = bin 家族）
    Str(Vec<u8>),
    Array(Vec<CVal>),
    /// map 保持插入序（lua 表无稳定序，消费方不依赖键序）
    Map(Vec<(CVal, CVal)>),
}

fn put_uint(out: &mut Vec<u8>, v: u64) {
    if v <= 0x7f {
        out.push(v as u8);
    } else if v <= 0xff {
        out.extend_from_slice(&[0xcc, v as u8]);
    } else if v <= 0xffff {
        out.push(0xcd);
        out.extend_from_slice(&(v as u16).to_be_bytes());
    } else if v <= 0xffff_ffff {
        out.push(0xce);
        out.extend_from_slice(&(v as u32).to_be_bytes());
    } else {
        out.push(0xcf);
        out.extend_from_slice(&v.to_be_bytes());
    }
}

pub fn pack(v: &CVal, out: &mut Vec<u8>) {
    match v {
        CVal::Nil => out.push(0xc0),
        CVal::Bool(b) => out.push(if *b { 0xc3 } else { 0xc2 }),
        CVal::Int(i) => {
            let i = *i;
            if i >= 0 {
                put_uint(out, i as u64);
            } else if i >= -32 {
                out.push(i as i8 as u8);
            } else if i >= -128 {
                out.extend_from_slice(&[0xd0, i as i8 as u8]);
            } else if i >= -32768 {
                out.push(0xd1);
                out.extend_from_slice(&(i as i16).to_be_bytes());
            } else if i >= -2147483648 {
                out.push(0xd2);
                out.extend_from_slice(&(i as i32).to_be_bytes());
            } else {
                out.push(0xd3);
                out.extend_from_slice(&i.to_be_bytes());
            }
        }
        CVal::U64(u) => put_uint(out, *u),
        CVal::F64(f) => {
            out.push(0xcb);
            out.extend_from_slice(&f.to_be_bytes());
        }
        CVal::Str(s) => {
            let n = s.len();
            if n <= 0xff {
                out.extend_from_slice(&[0xc4, n as u8]);
            } else if n <= 0xffff {
                out.push(0xc5);
                out.extend_from_slice(&(n as u16).to_be_bytes());
            } else {
                out.push(0xc6);
                out.extend_from_slice(&(n as u32).to_be_bytes());
            }
            out.extend_from_slice(s);
        }
        CVal::Array(a) => {
            let n = a.len();
            if n <= 0x0f {
                out.push(0x90 | n as u8);
            } else if n <= 0xffff {
                out.push(0xdc);
                out.extend_from_slice(&(n as u16).to_be_bytes());
            } else {
                out.push(0xdd);
                out.extend_from_slice(&(n as u32).to_be_bytes());
            }
            for x in a {
                pack(x, out);
            }
        }
        CVal::Map(m) => {
            let n = m.len();
            if n <= 0x0f {
                out.push(0x80 | n as u8);
            } else if n <= 0xffff {
                out.push(0xde);
                out.extend_from_slice(&(n as u16).to_be_bytes());
            } else {
                out.push(0xdf);
                out.extend_from_slice(&(n as u32).to_be_bytes());
            }
            for (k, x) in m {
                pack(k, out);
                pack(x, out);
            }
        }
    }
}

pub fn pack_to_vec(v: &CVal) -> Vec<u8> {
    let mut out = Vec::new();
    pack(v, &mut out);
    out
}

/// 调试短格式（出站打点用）：截断字符串/长数组，防刷屏
pub fn debug_short(v: &CVal) -> String {
    match v {
        CVal::Nil => "nil".into(),
        CVal::Bool(b) => format!("{b}"),
        CVal::Int(i) => format!("{i}"),
        CVal::U64(u) => format!("{u}"),
        CVal::F64(f) => format!("{f}"),
        CVal::Str(s) => {
            let t = String::from_utf8_lossy(s);
            let t: String = t.chars().take(40).collect();
            format!("\"{t}\"")
        }
        CVal::Array(a) => {
            let items: Vec<String> = a.iter().take(6).map(debug_short).collect();
            format!("[{}{}]", items.join(","), if a.len() > 6 { ",.." } else { "" })
        }
        CVal::Map(m) => {
            let items: Vec<String> = m
                .iter()
                .take(8)
                .map(|(k, x)| format!("{}={}", debug_short(k), debug_short(x)))
                .collect();
            format!("{{{}{}}}", items.join(","), if m.len() > 8 { ",.." } else { "" })
        }
    }
}

/// 嵌套深度上限：防深嵌套输入（网络上行）/ 环表耗尽 Rust 栈（栈溢出不可捕获直接 abort 进程）
const MAX_DEPTH: usize = 128;

/// 解析一个值；返回 (值, 消费长度)
pub fn unpack(b: &[u8]) -> Option<(CVal, usize)> {
    let (v, used) = unpack_at(b, 0, 0)?;
    Some((v, used))
}

fn take<'a>(b: &'a [u8], i: &mut usize, n: usize) -> Option<&'a [u8]> {
    if *i + n > b.len() {
        return None;
    }
    let s = &b[*i..*i + n];
    *i += n;
    Some(s)
}

fn be_u(b: &[u8], i: &mut usize, n: usize) -> Option<u64> {
    let s = take(b, i, n)?;
    let mut v = 0u64;
    for &x in s {
        v = (v << 8) | x as u64;
    }
    Some(v)
}

fn unpack_at(b: &[u8], start: usize, depth: usize) -> Option<(CVal, usize)> {
    // 深度超限直接拒包（返回 None 走"内层非 cmsg map"分支，不递归爆栈）
    if depth > MAX_DEPTH {
        return None;
    }
    let mut i = start;
    let t = *b.get(i)?;
    i += 1;
    let v = match t {
        0x00..=0x7f => CVal::Int(t as i64),
        0xe0..=0xff => CVal::Int(t as i8 as i64),
        0xc0 => CVal::Nil,
        0xc2 => CVal::Bool(false),
        0xc3 => CVal::Bool(true),
        0xcc => CVal::Int(*take(b, &mut i, 1)? .get(0)? as i64),
        0xcd => CVal::Int(be_u(b, &mut i, 2)? as i64),
        0xce => CVal::Int(be_u(b, &mut i, 4)? as i64),
        0xcf => CVal::U64(be_u(b, &mut i, 8)?),
        0xd0 => CVal::Int(*take(b, &mut i, 1)? .get(0)? as i8 as i64),
        0xd1 => CVal::Int(be_u(b, &mut i, 2)? as i16 as i64),
        0xd2 => CVal::Int(be_u(b, &mut i, 4)? as i32 as i64),
        0xd3 => CVal::Int(be_u(b, &mut i, 8)? as i64),
        0xca => CVal::F64(f32::from_bits(be_u(b, &mut i, 4)? as u32) as f64),
        0xcb => CVal::F64(f64::from_bits(be_u(b, &mut i, 8)?)),
        0xc4 | 0xc5 | 0xc6 => {
            let n = match t {
                0xc4 => *take(b, &mut i, 1)?.first()? as usize,
                0xc5 => be_u(b, &mut i, 2)? as usize,
                _ => be_u(b, &mut i, 4)? as usize,
            };
            CVal::Str(take(b, &mut i, n)?.to_vec())
        }
        0xd9 | 0xda | 0xdb => {
            let n = match t {
                0xd9 => *take(b, &mut i, 1)?.first()? as usize,
                0xda => be_u(b, &mut i, 2)? as usize,
                _ => be_u(b, &mut i, 4)? as usize,
            };
            CVal::Str(take(b, &mut i, n)?.to_vec())
        }
        0xa0..=0xbf => CVal::Str(take(b, &mut i, (t & 0x1f) as usize)?.to_vec()),
        0x90..=0x9f => {
            let n = (t & 0x0f) as usize;
            let mut a = Vec::with_capacity(n);
            for _ in 0..n {
                let (x, end) = unpack_at(b, i, depth + 1)?;
                i = end;
                a.push(x);
            }
            CVal::Array(a)
        }
        0xdc | 0xdd => {
            let n = if t == 0xdc { be_u(b, &mut i, 2)? } else { be_u(b, &mut i, 4)? } as usize;
            let mut a = Vec::with_capacity(n.min(1 << 20));
            for _ in 0..n {
                let (x, end) = unpack_at(b, i, depth + 1)?;
                i = end;
                a.push(x);
            }
            CVal::Array(a)
        }
        0x80..=0x8f => {
            let n = (t & 0x0f) as usize;
            let mut m = Vec::with_capacity(n);
            for _ in 0..n {
                let (k, e1) = unpack_at(b, i, depth + 1)?;
                let (x, e2) = unpack_at(b, e1, depth + 1)?;
                i = e2;
                m.push((k, x));
            }
            CVal::Map(m)
        }
        0xde | 0xdf => {
            let n = if t == 0xde { be_u(b, &mut i, 2)? } else { be_u(b, &mut i, 4)? } as usize;
            let mut mm = Vec::with_capacity(n.min(1 << 20));
            for _ in 0..n {
                let (k, e1) = unpack_at(b, i, depth + 1)?;
                let (x, e2) = unpack_at(b, e1, depth + 1)?;
                i = e2;
                mm.push((k, x));
            }
            CVal::Map(mm)
        }
        _ => return None,
    };
    Some((v, i))
}

// ---------- 与 mlua 的互转（lua_host 用；放在这里让 cmsg_pack 保持单一实现） ----------

mod lua_conv {
    use super::{CVal, MAX_DEPTH};
    use mlua::{Lua, Result, Value};
    use std::collections::HashSet;

    pub fn to_lua(lua: &Lua, v: &CVal) -> Result<Value> {
        to_lua_at(lua, v, 0)
    }

    fn to_lua_at(lua: &Lua, v: &CVal, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(mlua::Error::external("cmsg_pack 嵌套深度超限"));
        }
        Ok(match v {
            CVal::Nil => Value::Nil,
            CVal::Bool(b) => Value::Boolean(*b),
            CVal::Int(i) => Value::Integer(*i),
            CVal::U64(u) => {
                if *u <= i64::MAX as u64 {
                    Value::Integer(*u as i64)
                } else {
                    Value::Number(*u as f64)
                }
            }
            CVal::F64(f) => Value::Number(*f),
            CVal::Str(s) => Value::String(lua.create_string(s)?),
            CVal::Array(a) => {
                let t = lua.create_table()?;
                for (i, x) in a.iter().enumerate() {
                    t.set(i + 1, to_lua_at(lua, x, depth + 1)?)?;
                }
                Value::Table(t)
            }
            CVal::Map(m) => {
                let t = lua.create_table()?;
                for (k, x) in m {
                    t.set(to_lua_at(lua, k, depth + 1)?, to_lua_at(lua, x, depth + 1)?)?;
                }
                Value::Table(t)
            }
        })
    }

    pub fn from_lua(v: &Value) -> Result<CVal> {
        let mut visited = HashSet::new();
        from_lua_at(v, 0, &mut visited)
    }

    /// visited：当前递归路径上的表指针集合（仅检测环，跨分支共享子表不误报）
    fn from_lua_at(v: &Value, depth: usize, visited: &mut HashSet<usize>) -> Result<CVal> {
        if depth > MAX_DEPTH {
            return Err(mlua::Error::external("cmsg_pack 嵌套深度超限"));
        }
        Ok(match v {
            Value::Nil => CVal::Nil,
            Value::Boolean(b) => CVal::Bool(*b),
            Value::Integer(i) => CVal::Int(*i),
            Value::Number(n) => CVal::F64(*n),
            Value::String(s) => CVal::Str(s.as_bytes().to_vec()),
            Value::Table(t) => {
                // 环检测：lua 表可自指（t.self = t），遇环返回可捕获的 lua error 而不是栈溢出崩进程
                let ptr = t.to_pointer() as usize;
                if !visited.insert(ptr) {
                    return Err(mlua::Error::external("cmsg_pack 不支持循环引用表"));
                }
                let r: Result<CVal> = (|| {
                    // 数组判定：键必须【恰好】为 1..=len 且无额外键——稀疏整数键（如商店 bought={1..4,11..32}）
                    // raw_len 只量连续前缀，误判为数组会丢稀疏尾部（test_res002 已售罄不更新根因）
                    let mut pairs_v = Vec::new();
                    for pair in t.clone().pairs::<Value, Value>() {
                        pairs_v.push(pair?);
                    }
                    let len = t.raw_len();
                    let is_array = len > 0
                        && pairs_v.len() == len
                        && pairs_v
                            .iter()
                            .all(|(k, _)| matches!(k, Value::Integer(i) if *i >= 1 && *i as usize <= len));
                    if is_array {
                        let mut a = vec![CVal::Nil; len];
                        for (k, x) in pairs_v {
                            let Value::Integer(i) = k else { unreachable!() };
                            a[(i - 1) as usize] = from_lua_at(&x, depth + 1, visited)?;
                        }
                        Ok(CVal::Array(a))
                    } else {
                        let mut m = Vec::with_capacity(pairs_v.len());
                        for (k, x) in pairs_v {
                            m.push((
                                from_lua_at(&k, depth + 1, visited)?,
                                from_lua_at(&x, depth + 1, visited)?,
                            ));
                        }
                        Ok(CVal::Map(m))
                    }
                })();
                visited.remove(&ptr);
                r?
            }
            Value::LightUserData(_) | Value::Function(_) | Value::Thread(_) | Value::UserData(_) => {
                CVal::Nil
            }
            Value::Error(e) => return Err(mlua::Error::external(format!("cmsg_pack 不支持 error 值: {e}"))),
            _ => CVal::Nil,
        })
    }
}

pub use lua_conv::{from_lua, to_lua};
