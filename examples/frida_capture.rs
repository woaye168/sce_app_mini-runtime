//! Frida 抓取编辑器调试 launcher（version-13/SCE）的 ws2_32 收发字节。
//! （frida_capture.py 的 Rust 版；注入脚本为 examples/ws_hook.js，经 include_str! 嵌入）
//!
//! 用途：逆向 DebugManager 控制协议（EditorLogin/上传/EditorStartGame）的真实线上格式。
//!
//! 用法：
//!   cargo run --release --example frida_capture -- <out.jsonl> [选项] [-- <目标进程参数>...]
//! 选项：
//!   --exe <路径>     目标程序（默认 D:/sce_online/version-13/SCE）
//!   --cwd <目录>     工作目录（默认 D:/sce_online）
//!   --seconds <秒>   抓包时长（默认 240）
//!
//! 默认目标参数 = 编辑器「生成并调试」test_res002 项目的 launcher 命令行。

use frida::{DeviceManager, Frida, Message, ScriptHandler, ScriptOption, SpawnOptions};
use std::io::Write;

const JS: &str = include_str!("ws_hook.js");

struct Handler {
    out: std::fs::File,
    n: usize,
}

impl ScriptHandler for Handler {
    fn on_message(&mut self, message: Message, _data: Option<Vec<u8>>) {
        // 自定义 send() 的 payload 不匹配 crate 内置的 SendPayload（RPC 形状），
        // 反序列化失败会落入 Message::Other({"error","data": 原始json串})，在此还原。
        let raw = match &message {
            Message::Other(v) => v.get("data").and_then(|d| d.as_str()).map(String::from),
            Message::Error(e) => {
                eprintln!("[script error] {} ({}:{})", e.description, e.line_number, e.column_number);
                None
            }
            Message::Log(l) => {
                eprintln!("[script log] {}", l.payload);
                None
            }
            other => {
                eprintln!("[msg] {other:?}");
                None
            }
        };
        let Some(raw) = raw else { return };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else { return };
        let Some(rec) = v.get("payload").and_then(|p| p.get("rec")) else { return };
        self.n += 1;
        let line = serde_json::to_string(rec).unwrap();
        let _ = writeln!(self.out, "{line}");
        let _ = self.out.flush();
        println!(
            "[{}] {} {} len={}",
            self.n,
            rec["op"].as_str().unwrap_or(""),
            rec["addr"].as_str().unwrap_or(""),
            rec["len"].as_i64().map(|x| x.to_string()).unwrap_or_default()
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out_path: Option<String> = None;
    let mut exe = r"D:/sce_online/version-13/SCE".to_string();
    let mut cwd = r"D:/sce_online".to_string();
    let mut seconds: u64 = 240;
    let mut targs: Vec<String> = Vec::new();
    let mut it = args.iter().peekable();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--exe" => exe = it.next().expect("--exe 缺值").clone(),
            "--cwd" => cwd = it.next().expect("--cwd 缺值").clone(),
            "--seconds" => seconds = it.next().expect("--seconds 缺值").parse().expect("秒数非法"),
            "--" => targs.extend(it.by_ref().cloned()),
            s if !s.starts_with('-') && out_path.is_none() => out_path = Some(s.to_string()),
            _ => {
                eprintln!("用法: frida_capture <out.jsonl> [--exe 路径] [--cwd 目录] [--seconds 秒] [-- 目标参数...]");
                std::process::exit(1);
            }
        }
    }
    let Some(out_path) = out_path else {
        eprintln!("缺输出文件路径");
        std::process::exit(1);
    };
    if targs.is_empty() {
        targs = [
            "-server=editor-pd.spark.xd.com",
            "-use_local_res",
            "-launcher=星火编辑器.exe",
            "-editor_api_version=13",
            "-no_ask_editor_api_version",
            "-generate_and_debug_map",
            r"-file_path=c:\Users\woaye\Documents\SCE Projects\test_res002\project.sce",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
    }

    let frida = unsafe { Frida::obtain() };
    let device_manager = DeviceManager::obtain(&frida);
    let mut device = device_manager.get_local_device().expect("获取本机设备失败");

    let argv: Vec<String> = std::iter::once(exe.clone()).chain(targs.iter().cloned()).collect();
    let cwd_c = std::ffi::CString::new(cwd).unwrap();
    let options = SpawnOptions::new().argv(argv).cwd(cwd_c.as_c_str());
    let pid = device.spawn(&exe, &options).expect("spawn 失败");
    println!("spawned pid {pid}");

    let session = device.attach(pid).expect("attach 失败");
    let mut script = session
        .create_script(JS, &mut ScriptOption::default())
        .expect("create_script 失败");
    let out = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out_path)
        .expect("打开输出文件失败");
    script
        .handle_message(Handler { out, n: 0 })
        .expect("handle_message 失败");
    script.load().expect("script load 失败");
    device.resume(pid).expect("resume 失败");
    println!("resumed, capturing {seconds}s...");
    std::thread::sleep(std::time::Duration::from_secs(seconds));
    drop(script);
    drop(session);
    let _ = device.kill(pid);
    println!("done");
}
