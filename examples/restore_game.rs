//! SCE 加密包一键还原工具：TNND 解密 → 7z 解压 → UPAK 解包 → 图片(KTX)还原。
//! （restore_game.py 的 Rust 版；BC 纹理解码改用纯 Rust bcdec_rs，顺带补上了 BC2/DXT3）
//!
//! 用法：
//!   cargo run --release --example restore_game -- <加密7z路径> [-o 输出目录] [--keep-temp] [--no-decode-images]
//!
//! 只读研究工具：不修改输入文件。

use std::fs;
use std::path::{Component, Path, PathBuf};

const TNND_MAGIC: &[u8; 4] = b"TNND";
const TNND_KEY: &[u8; 10] = b"CREATEEASY";
const UPAK_MAGIC: &[u8; 4] = b"UPAK";
const KTX_MAGIC: &[u8; 12] = b"\xabKTX 11\xbb\r\n\x1a\n";

const IFMT_BC7: u32 = 0x8E8C; // GL_COMPRESSED_RGBA_BPTC_UNORM
const IFMT_RGBA8: u32 = 0x8058;
const IFMT_RGB8: u32 = 0x8051;
const IFMT_DXT1: u32 = 0x83F1; // BC1
const IFMT_DXT3: u32 = 0x83F2; // BC2
const IFMT_DXT5: u32 = 0x83F3; // BC3

// ---------------- TNND ----------------

fn is_tnnd(path: &Path) -> bool {
    fs::read(path).map(|b| b.starts_with(TNND_MAGIC)).unwrap_or(false)
}

fn tnnd_decrypt(src: &Path, dst: &Path) -> std::io::Result<usize> {
    let data = fs::read(src)?;
    if !data.starts_with(TNND_MAGIC) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "不是 TNND 文件"));
    }
    let body = &data[TNND_MAGIC.len()..];
    let mut out = body.to_vec();
    for (i, x) in out.iter_mut().enumerate() {
        *x ^= TNND_KEY[i % TNND_KEY.len()];
    }
    fs::write(dst, &out)?;
    Ok(out.len())
}

// ---------------- 7z ----------------

fn extract_7z(archive: &Path, outdir: &Path) -> Result<(), String> {
    // 优先 7z/7za，其次 Windows 10+ 自带 bsdtar（支持 7z）
    let candidates = ["7z", "7za"]
        .into_iter()
        .map(String::from)
        .chain([
            r"C:\Program Files\7-Zip\7z.exe".to_string(),
            r"C:\Program Files (x86)\7-Zip\7z.exe".to_string(),
        ]);
    for exe in candidates {
        if which(&exe) {
            let ok = std::process::Command::new(&exe)
                .args(["x", &archive.to_string_lossy(), &format!("-o{}", outdir.display()), "-y"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if ok {
                return Ok(());
            }
        }
    }
    let ok = std::process::Command::new("tar")
        .args(["-xf", &archive.to_string_lossy(), "-C", &outdir.to_string_lossy()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        Ok(())
    } else {
        Err("找不到可用的 7z 解压方式：请安装 7-Zip 或使用 Win10+ 自带 tar".into())
    }
}

fn which(exe: &str) -> bool {
    if Path::new(exe).exists() {
        return true;
    }
    std::process::Command::new("where")
        .arg(exe)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ---------------- UPAK ----------------

/// 防路径穿越：归一化分隔符，剔除 .. / 盘符 / 绝对路径。
fn safe_rel(name: &str) -> PathBuf {
    let mut out = PathBuf::new();
    for part in name.replace('\\', "/").split('/') {
        match part {
            "" | "." => {}
            ".." => out.push("__"),
            p if p.contains(':') => out.push(p.replace(':', "_")),
            p => out.push(p),
        }
    }
    if out.as_os_str().is_empty() {
        out.push("_unnamed");
    }
    // 双保险：最终路径不得含根/前缀组件
    assert!(out.components().all(|c| matches!(c, Component::Normal(_))));
    out
}

/// 解包 SCE UPAK（条目 = 名字\0 + u32 offset + u32 size + u32 checksum）。返回条目数。
fn upak_extract(pak: &Path, outdir: &Path) -> Result<usize, String> {
    let data = fs::read(pak).map_err(|e| e.to_string())?;
    if !data.starts_with(UPAK_MAGIC) {
        return Err(format!("不是 UPAK 文件: {}", pak.display()));
    }
    let u32at = |o: usize| -> Result<u32, String> {
        data.get(o..o + 4)
            .and_then(|b| b.try_into().ok())
            .map(u32::from_le_bytes)
            .ok_or_else(|| "索引越界".to_string())
    };
    let count = u32at(4)?; // 偏移 8 为 u32 总校验，索引区从 12 开始
    let mut p = 12usize;
    let mut ok = 0usize;
    for _ in 0..count {
        let end = data[p..]
            .iter()
            .position(|&b| b == 0)
            .map(|x| p + x)
            .ok_or("条目名缺少 \\0 结尾，索引损坏")?;
        let name = String::from_utf8_lossy(&data[p..end]).to_string();
        p = end + 1;
        let (offset, size) = (u32at(p)? as usize, u32at(p + 4)? as usize);
        p += 12; // offset + size + checksum（比标准 Urho3D 多 4 字节校验）
        let content = data.get(offset..offset + size).ok_or("条目内容越界")?;
        let target = outdir.join(safe_rel(&name));
        fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
        fs::write(&target, content).map_err(|e| e.to_string())?;
        ok += 1;
    }
    Ok(ok)
}

// ---------------- 图片还原（伪 KTX → PNG） ----------------

fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// 解码单个伪 KTX 文件并就地保存为 PNG。非 KTX 返回 Ok(false)。
///
/// 格式（伪装成 KTX 纹理）：
///   - 12 字节魔数 AB 4B 54 58 20 31 31 BB 0D 0A 1A 0A
///   - 偏移 28: glInternalFormat（0x8E8C=BC7, 0x8058=RGBA8, 0x8051=RGB8,
///               0x83F1=DXT1/BC1, 0x83F2=DXT3/BC2, 0x83F3=DXT5/BC3）
///   - 偏移 36/40: 宽 / 高；偏移 64: imgSize（最低位为填充标志 pad，
///     数据大小 = pad ? imgSize>>8 : imgSize）；偏移 68+pad: 图像数据
fn decode_ktx_image(src: &Path) -> Result<bool, String> {
    let buf = fs::read(src).map_err(|e| e.to_string())?;
    if !buf.starts_with(KTX_MAGIC) {
        return Ok(false);
    }
    let ifmt = u32le(&buf, 28);
    let (w, h) = (u32le(&buf, 36) as usize, u32le(&buf, 40) as usize);
    let img_size = u32le(&buf, 64) as usize;
    let pad = img_size & 1;
    let data_size = if pad != 0 { img_size >> 8 } else { img_size };
    let data = buf
        .get(68 + pad..68 + pad + data_size)
        .ok_or("图像数据越界")?;

    // BC 压缩格式 → (期望数据字节数, 是否 BC)；bcdec_rs 输出 RGBA，无需换通道
    let rgba: Vec<u8> = match ifmt {
        IFMT_BC7 | IFMT_DXT1 | IFMT_DXT3 | IFMT_DXT5 => {
            let block_bytes = if ifmt == IFMT_DXT1 { 8 } else { 16 };
            let expected = ((w + 3) / 4) * ((h + 3) / 4) * block_bytes;
            if data.len() != expected {
                return Err(format!(
                    "数据长度异常: {} != {expected} (0x{ifmt:04x}, {w}x{h})",
                    data.len()
                ));
            }
            let mut out = vec![0u8; w * h * 4];
            match ifmt {
                IFMT_BC7 => bcdec_rs::bc7(data, &mut out, w * 4),
                IFMT_DXT1 => bcdec_rs::bc1(data, &mut out, w * 4),
                IFMT_DXT3 => bcdec_rs::bc2(data, &mut out, w * 4),
                _ => bcdec_rs::bc3(data, &mut out, w * 4),
            }
            out
        }
        IFMT_RGBA8 => data.to_vec(),
        IFMT_RGB8 => {
            let mut out = Vec::with_capacity(w * h * 4);
            for px in data.chunks_exact(3) {
                out.extend_from_slice(&[px[0], px[1], px[2], 255]);
            }
            out
        }
        _ => return Err(format!("不支持的格式: 0x{ifmt:04x}")),
    };

    let dst = src.with_extension("png");
    image::save_buffer(&dst, &rgba, w as u32, h as u32, image::ColorType::Rgba8)
        .map_err(|e| format!("PNG 编码失败: {e}"))?;
    if dst != src {
        let _ = fs::remove_file(src);
    }
    Ok(true)
}

fn decode_images_inplace(root: &Path) -> (usize, usize) {
    let (mut ok, mut fail) = (0, 0);
    for entry in walk(root) {
        if !entry.is_file() {
            continue;
        }
        let probe = fs::read(&entry).unwrap_or_default();
        if !probe.starts_with(KTX_MAGIC) {
            continue;
        }
        match decode_ktx_image(&entry) {
            Ok(_) => ok += 1,
            Err(e) => {
                fail += 1;
                println!("    [图片失败] {}: {e}", entry.strip_prefix(root).unwrap().display());
            }
        }
    }
    (ok, fail)
}

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        if let Ok(rd) = fs::read_dir(&d) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else {
                    out.push(p);
                }
            }
        }
    }
    out.sort();
    out
}

// ---------------- 主流程 ----------------

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut keep_temp = false;
    let mut decode_images = true;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "-o" | "--output" => out = it.next().map(PathBuf::from),
            "--keep-temp" => keep_temp = true,
            "--no-decode-images" => decode_images = false,
            s if !s.starts_with('-') && input.is_none() => input = Some(PathBuf::from(s)),
            _ => {
                eprintln!("用法: restore_game <加密7z> [-o 输出目录] [--keep-temp] [--no-decode-images]");
                std::process::exit(1);
            }
        }
    }
    let input = input.expect("缺输入文件");
    if !input.is_file() {
        eprintln!("输入文件不存在: {}", input.display());
        std::process::exit(1);
    }
    let out_root = out.unwrap_or_else(|| {
        input.with_file_name(format!("{}_restored", input.file_stem().unwrap().to_string_lossy()))
    });
    if let Err(e) = restore(&input, &out_root, keep_temp, decode_images) {
        eprintln!("失败: {e}");
        std::process::exit(1);
    }
}

fn restore(input: &Path, out_root: &Path, keep_temp: bool, decode_images: bool) -> Result<(), String> {
    fs::create_dir_all(out_root).map_err(|e| e.to_string())?;
    let final_dir = out_root.join("files");
    let tmpdir = out_root.join(format!("tnnd_tmp_{}", std::process::id()));
    let raw_dir = tmpdir.join("raw_7z");

    let result = (|| -> Result<(), String> {
        // 1. TNND 解密（如有）
        let dec_7z = if is_tnnd(input) {
            let p = tmpdir.join(format!("{}.dec.7z", input.file_stem().unwrap().to_string_lossy()));
            fs::create_dir_all(&tmpdir).map_err(|e| e.to_string())?;
            let n = tnnd_decrypt(input, &p).map_err(|e| e.to_string())?;
            println!("[1/4] TNND 解密: {} -> {n} 字节", input.display());
            p
        } else {
            println!("[1/4] 无 TNND 头，按明文 7z 处理: {}", input.display());
            input.to_path_buf()
        };

        // 2. 解压 7z
        fs::create_dir_all(&raw_dir).map_err(|e| e.to_string())?;
        extract_7z(&dec_7z, &raw_dir)?;
        println!("[2/4] 7z 解压完成（中间目录）");

        // 3. UPAK 解包 / TNND 解密 / 原样拷贝
        fs::create_dir_all(&final_dir).map_err(|e| e.to_string())?;
        for f in walk(&raw_dir) {
            if !f.is_file() {
                continue;
            }
            let rel = f.strip_prefix(&raw_dir).unwrap().to_path_buf();
            let probe = fs::read(&f).unwrap_or_default();
            if probe.starts_with(UPAK_MAGIC) {
                let target_dir = final_dir.join(rel.parent().unwrap()).join(f.file_stem().unwrap());
                let count = upak_extract(&f, &target_dir)?;
                println!("[3/4] UPAK 解包: {} -> {} ({count} 个文件)", rel.display(), target_dir.display());
            } else if probe.starts_with(TNND_MAGIC) {
                let target = final_dir.join(&rel);
                fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
                tnnd_decrypt(&f, &target).map_err(|e| e.to_string())?;
                println!("[3/4] TNND 解密: {}", rel.display());
            } else {
                let target = final_dir.join(&rel);
                fs::create_dir_all(target.parent().unwrap()).map_err(|e| e.to_string())?;
                fs::copy(&f, &target).map_err(|e| e.to_string())?;
                println!("[3/4] 明文拷贝: {}", rel.display());
            }
        }

        // 4. 图片还原
        if decode_images {
            let (ok, fail) = decode_images_inplace(&final_dir);
            println!("[4/4] 图片还原: {ok} 张已解码为 PNG，{fail} 张失败");
        } else {
            println!("[4/4] 已按 --no-decode-images 跳过图片还原");
        }
        println!("完成。最终产物目录: {}", final_dir.display());
        Ok(())
    })();

    if keep_temp && result.is_ok() {
        println!("临时文件保留于: {}", tmpdir.display());
    } else {
        let _ = fs::remove_dir_all(&tmpdir);
    }
    result
}
