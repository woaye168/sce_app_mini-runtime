//! ZCompress 复刻（h2c 传输层压缩，无密钥纯格式）。
//!
//! 格式逆向自 sceengine.dll（version-13，editor 构建）反汇编，函数锚点与语义对照：
//! - 解压入口 VA 0x181ac0820：首字节有符号 >= 0（0x00-0x7F）→ 原样模式，载荷 = data[1..]；
//!   否则（>= 0x80）→ 压缩模式，bit0 = 标志位 1，从位偏移 1 起解码。
//! - 位读取器（0x181ae3f60 初始化 / 0x181ae4030 读 1 位 / 0x180e43130 读 n 位）：MSB-first。
//! - 解码主体 0x181ac3d50：LZ77（0x8102 字节环窗）+ 自适应 Huffman。
//!   字母表 0x123 = 291 符号：0x00-0xFF 字面量 / 0x100-0x121 长度 / 0x122 END。
//!   长度符号与距离符号共享字母表（读完长度符号后下一符号按距离解释）：
//!     距离：sym < 0x100 → sym + 9；0x100 <= sym < 0x108 → sym - 0xFF；>= 0x108 → extra(sym-0x100) + 0x100
//!     长度：extra(sym - 0x100) + 3
//!     extra(c)（0x181ac4040）：c < 8 → c；否则 e = (c>>1)-3，剩余位不足 e 返回 c（容错），
//!     否则 read_bits(e) + (1<<(e+1)) + ((c&1)<<e) + 4。
//! - 权重/重建（0x181ac4b40 / 0x181ac4660）：每消息解码后该消息全部符号权重 +1
//!   （0x7FFFFFFF 饱和时整批回滚并永久停更）；输出字节数 > 100 且（重建计数 < 2000 或输出 > 1000）→ 全量重建树。
//!   重建 = 按权重桶式优先队列（同权重 FIFO）合并建 Huffman 树，child0=先弹出者（位 0）。
//!   树状态**跨消息持续**（连接级对象），首条消息起始于全 1 权重的初始树。
//!
//! 详见 doc/research/scegame-reverse.md §13.6。

type Result<T> = std::result::Result<T, String>;

/// 环窗容量（引擎硬编码 0x8102）
const WINDOW: usize = 0x8102;
/// 符号总数（0..=0x122）
const NUM_SYMBOLS: usize = 0x123;
/// 结束符号
const SYM_END: u16 = 0x122;
/// 权重饱和值
const WEIGHT_MAX: i32 = 0x7FFF_FFFF;
/// 重建触发：输出字节数阈值
const REBUILD_MIN_OUT: u64 = 100;
/// 重建计数上限（超过后仅大消息触发重建）
const REBUILD_COUNT_MAX: u32 = 2000;
/// 重建计数上限后的大消息阈值
const REBUILD_BIG_OUT: u64 = 1000;

// ---------------- 位读取器（MSB-first） ----------------

pub struct BitReader<'a> {
    buf: &'a [u8],
    /// 当前字节偏移
    pos: usize,
    /// 当前字节内位偏移（0-7，0 = MSB）
    bit: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0, bit: 0 }
    }

    /// 剩余位数（对应 extra 的容错检查：(end - cur)*8 - bit_off）
    fn remaining_bits(&self) -> u64 {
        (self.buf.len().saturating_sub(self.pos) as u64) * 8 - self.bit as u64
    }

    /// 读 1 位（无边界检查，调用方负责；对应 0x181ae4030）
    fn read_bit(&mut self) -> u32 {
        let b = (self.buf[self.pos] << self.bit) & 0x80;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.pos += 1;
        }
        (b != 0) as u32
    }

    /// 读 n 位（MSB-first，n <= 32；对应 0x180e43130，含越界容错语义）
    fn read_bits(&mut self, n: u8) -> u32 {
        let n = n.min(32);
        if n == 0 {
            return 0;
        }
        if self.pos >= self.buf.len() {
            return 0;
        }
        let mut result: u32 = 0;
        let mut remaining = n;
        // 首字节（可能是不满 8 位的部分）
        let take = (8 - self.bit).min(remaining);
        if take > 0 {
            let shift = 8 - self.bit - take;
            let chunk = ((self.buf[self.pos] >> shift) as u32) & ((1u32 << take) - 1);
            self.bit += take;
            if self.bit == 8 {
                self.bit = 0;
                self.pos += 1;
            }
            result = chunk << (remaining - take);
            remaining -= take;
        }
        // 整字节段：字节不够时整段跳过直接返回（镜像原实现的提前返回语义）
        let full = remaining >> 3;
        if full > 0 {
            if self.pos + full as usize > self.buf.len() {
                return result;
            }
            let mut shift = remaining - 8;
            for _ in 0..full {
                result |= (self.buf[self.pos] as u32) << shift;
                self.pos += 1;
                shift = shift.saturating_sub(8);
            }
            remaining -= full * 8;
        }
        // 末尾不足 8 位
        if remaining > 0 {
            if self.pos >= self.buf.len() {
                return result;
            }
            let chunk = ((self.buf[self.pos] >> (8 - remaining)) as u32) & ((1u32 << remaining) - 1);
            self.bit += remaining;
            result |= chunk;
            if self.bit == 8 {
                self.bit = 0;
                self.pos += 1;
            }
        }
        result
    }
}

// ---------------- Huffman 树 ----------------

/// 树节点（16B：weight i32 / sym u16 / child0 i32 / child1 i32；叶子 child0 == -1）
#[derive(Clone, Copy, Default)]
struct Node {
    weight: i32,
    sym: u16,
    child: [i32; 2],
}

/// 桶式优先队列：按权重升序，同权重 FIFO（镜像引擎的桶 + 链表结构）
#[derive(Default)]
struct BucketQueue {
    /// weight -> FIFO 桶（BTreeMap 保升序）
    buckets: std::collections::BTreeMap<i32, std::collections::VecDeque<u16>>,
    len: usize,
}

impl BucketQueue {
    fn push(&mut self, weight: i32, node: u16) {
        self.buckets.entry(weight).or_default().push_back(node);
        self.len += 1;
    }

    fn pop_min(&mut self) -> Option<u16> {
        let mut entry = self.buckets.first_entry()?;
        let q = entry.get_mut();
        let v = q.pop_front();
        if q.is_empty() {
            entry.remove_entry();
        }
        if v.is_some() {
            self.len -= 1;
        }
        v
    }
}

// ---------------- 解码器（连接级有状态对象） ----------------

pub struct ZDecoder {
    /// 符号权重（0x8128，init 全 1）
    weights: [i32; NUM_SYMBOLS],
    /// Huffman 树节点（0x247 个，16B/个）
    nodes: Vec<Node>,
    /// 根节点索引（0x8ecc）
    root: i32,
    /// 环窗（内联在对象头部）
    ring: [u8; WINDOW],
    /// 写位置（0x8108）
    write_pos: u32,
    /// 0x8104：环满时缓存的写位置
    f8104: u32,
    /// 0x8110：写入计数（封顶 0x8102 语义见 emit）
    f8110: u64,
    /// 0x8118：环内最旧字节的绝对位置
    f8118: u64,
    /// 0x8120：环索引越界错误标志
    err_ring: bool,
    /// 0xb901：错误/停止标志
    err_stop: bool,
    /// 0xb908：重建计数
    rebuild_count: u32,
    /// 0xb90c：权重饱和标志（置位后永久停更）
    saturated: bool,
    /// 诊断：开启后往 trace_log 写符号级轨迹（R1/R2 调试）
    pub trace: bool,
    /// 诊断：符号轨迹（L<n>=长度 D<n>=距离，数字=符号）
    pub trace_log: Vec<String>,
}

impl Default for ZDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl ZDecoder {
    /// 构造（对应对象初始化 0x181ac4180：权重全 1 + 初始建树）
    pub fn new() -> Self {
        let mut d = Self {
            weights: [1; NUM_SYMBOLS],
            nodes: vec![Node::default(); 0x247],
            root: -1,
            ring: [0; WINDOW],
            write_pos: 0,
            f8104: 0,
            f8110: 0,
            f8118: 0,
            err_ring: false,
            err_stop: false,
            rebuild_count: 0,
            saturated: false,
            trace: false,
            trace_log: Vec::new(),
        };
        d.rebuild();
        d
    }

    /// 全量重建 Huffman 树（0x181ac4660）
    fn rebuild(&mut self) {
        for n in &mut self.nodes {
            *n = Node {
                weight: -1,
                sym: 0xFFFF,
                child: [-1, -1],
            };
        }
        let mut pq = BucketQueue::default();
        for sym in 0..NUM_SYMBOLS as u16 {
            let w = self.weights[sym as usize];
            self.nodes[sym as usize] = Node {
                weight: w,
                sym,
                child: [-1, -1],
            };
            pq.push(w, sym);
        }
        let mut next = NUM_SYMBOLS as u16;
        while pq.len > 1 {
            let a = pq.pop_min().unwrap();
            let b = pq.pop_min().unwrap();
            let w = self.nodes[a as usize].weight + self.nodes[b as usize].weight;
            self.nodes[next as usize] = Node {
                weight: w,
                sym: next,
                child: [a as i32, b as i32],
            };
            pq.push(w, next);
            next += 1;
        }
        self.root = pq.pop_min().unwrap() as i32;
    }

    /// 树走读一个符号（0x181ac40c0）：位流耗尽 → 置 err_stop 并返回 0
    fn read_symbol(&mut self, r: &mut BitReader) -> u16 {
        let mut node = self.root;
        loop {
            let n = self.nodes[node as usize];
            if n.child[0] == -1 {
                return n.sym;
            }
            if r.pos >= r.buf.len() {
                self.err_stop = true;
                return 0;
            }
            let bit = r.read_bit();
            node = n.child[bit as usize];
        }
    }

    /// extra-bits 值（0x181ac4040）：长度/距离符号共用
    fn extra_bits(&mut self, code: u32, r: &mut BitReader) -> u32 {
        if code < 8 {
            return code;
        }
        let e = (code >> 1) - 3;
        if e == 0 {
            return code;
        }
        if r.remaining_bits() < e as u64 {
            return code;
        }
        let v = r.read_bits(e as u8);
        v + (1 << (e + 1)) + ((code & 1) << e) + 4
    }

    /// 输出一字节（写输出 + 环窗簿记，镜像 0x181ac3f87 / 0x181ac3ee0 段）
    fn emit(&mut self, out: &mut Vec<u8>, b: u8) {
        out.push(b);
        if self.f8104 == self.write_pos && self.f8110 == WINDOW as u64 {
            self.f8118 += 1;
        } else {
            self.f8110 += 1;
        }
        self.ring[self.write_pos as usize] = b;
        let mut wp = self.write_pos + 1;
        if wp == WINDOW as u32 {
            wp = 0;
        }
        self.write_pos = wp;
        if self.f8110 == WINDOW as u64 {
            self.f8104 = wp;
        }
    }

    /// 解码一条压缩消息（对应 0x181ac3d50 + 0x181ac4ab0；输入不含首标志字节，从位偏移 1 起）
    /// 返回输出字节；错误时返回 Err（状态冻结：不做权重更新/重建）
    fn decode_compressed(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let mut r = BitReader::new(data);
        // bit0 = 压缩标志（1），消费之
        let _flag = r.read_bit();
        let mut out = Vec::new();
        let mut symbols: Vec<u16> = Vec::new();
        let mut pending_len: i64 = -1;
        loop {
            let sym = self.read_symbol(&mut r);
            symbols.push(sym);
            if self.trace {
                self.trace_log
                    .push(format!("sym={sym} @byte{}bit{}", r.pos, r.bit));
            }
            if sym == SYM_END {
                break;
            }
            if pending_len != -1 {
                // 距离符号
                let dist: i64 = if sym < 0x100 {
                    sym as i64 + 9
                } else if sym < 0x108 {
                    sym as i64 - 0xFF
                } else {
                    self.extra_bits(sym as u32 - 0x100, &mut r) as i64 + 0x100
                };
                if self.trace {
                    self.trace_log.push(format!("  D{dist} len={pending_len}"));
                }
                let src = self.f8110 as i64 - dist + self.f8118 as i64;
                let end = src + pending_len;
                if src < 0 || pending_len < 0 || src > end {
                    self.err_stop = true;
                    break;
                }
                let mut pos = src;
                while pos < end {
                    let b = if pos >= self.f8118 as i64 {
                        let mut idx = self.f8104 as i64 - self.f8118 as i64 + pos;
                        if idx >= WINDOW as i64 {
                            idx -= WINDOW as i64;
                        }
                        if idx >= WINDOW as i64 {
                            self.err_ring = true;
                            0
                        } else {
                            self.ring[idx as usize]
                        }
                    } else {
                        0
                    };
                    self.emit(&mut out, b);
                    pos += 1;
                }
                pending_len = -1;
            } else if sym < 0x100 {
                // 字面量
                self.emit(&mut out, sym as u8);
            } else {
                // 长度符号
                pending_len = self.extra_bits(sym as u32 - 0x100, &mut r) as i64 + 3;
                if self.trace {
                    self.trace_log.push(format!("  L{pending_len}"));
                }
            }
            if self.err_stop {
                break;
            }
        }
        if self.err_ring {
            return Err("ZCompress 环窗索引越界".into());
        }
        if self.err_stop {
            return Err("ZCompress 位流耗尽/距离异常".into());
        }
        // 权重更新 + 条件重建（0x181ac4b40）
        self.update_weights(&symbols, out.len() as u64);
        Ok(out)
    }

    /// 诊断：清错误标志（真实引擎置位后不再解码；抓包分析跳帧场景用）
    pub fn reset_errors(&mut self) {
        self.err_ring = false;
        self.err_stop = false;
    }

    /// 每消息解码后的权重更新与重建策略（0x181ac4b40；b900/b90d 实为 0）
    fn update_weights(&mut self, symbols: &[u16], out_len: u64) {
        let mut bumped = 0usize;
        for &sym in symbols {
            let w = &mut self.weights[sym as usize];
            if *w == WEIGHT_MAX {
                self.saturated = true;
                break;
            }
            *w += 1;
            bumped += 1;
        }
        if self.saturated {
            // 整批回滚（一旦饱和永久停更）
            for &sym in &symbols[..bumped] {
                self.weights[sym as usize] -= 1;
            }
        }
        if out_len > REBUILD_MIN_OUT {
            self.rebuild_count += 1;
            if self.rebuild_count < REBUILD_COUNT_MAX || out_len > REBUILD_BIG_OUT {
                self.rebuild();
            }
        }
    }
}

/// 解一帧 ZCompress 载荷（3B 流分帧体 = 完整一条消息）
///
/// 首字节 < 0x80 → 原样模式（载荷 = data[1..]，无状态动作）；
/// >= 0x80 → 压缩模式（自适应状态机推进）。
pub fn decode_frame(decoder: &mut ZDecoder, data: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Err("ZCompress 空帧".into());
    }
    if (data[0] as i8) >= 0 {
        return Ok(data[1..].to_vec());
    }
    decoder.decode_compressed(data)
}

/// 编码一帧（原样模式旁路：首字节 0x00 + 明文）。
///
/// 引擎解码器对首字节 < 0x80 的帧直接投递 data[1..]（0x181ac0a22 实锤），
/// 这是引擎自带的"不压缩"消息级标志，host 侧合法可用。
pub fn encode_frame_raw(plain: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(plain.len() + 1);
    out.push(0);
    out.extend_from_slice(plain);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_roundtrip() {
        let mut d = ZDecoder::new();
        let plain = b"hello zcompress";
        let enc = encode_frame_raw(plain);
        assert_eq!(decode_frame(&mut d, &enc).unwrap(), plain);
    }

    #[test]
    fn initial_tree_is_deterministic() {
        let d = ZDecoder::new();
        assert!(d.root >= 0);
        // 全 1 权重建树：290 次合并，根 = 节点 580
        assert_eq!(d.root, 580);
    }
}
