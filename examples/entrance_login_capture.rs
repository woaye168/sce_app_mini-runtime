//! WSS 明文截获通用工具：spawn scegame（挂起）→ 立即 hook libgmessl SSL_read/write
//! + ws2_32 connect（标记 entrance 连接归属）→ 抓明文帧。
//! 通用点：凡是 scegame 内经 libgmessl-1_1-x64.dll（OpenSSL 变体）的 TLS 流量都能在此
//! 还原为明文——entrance WSS / HTTPS API / host 控制通道（若走 TLS）通用。
//!
//! 用法：cargo run --release --example entrance_login_capture -- [scegame路径] [秒数]
//!   缺省 scegame = <仓库>/runtime/scegame.exe，默认抓 90 秒。
//! 前置：runtime/User/user_info-editor-pd.spark.xd.com.json 是登录态凭证。
//!
//! 输出：exe 同目录 wss_capture_<时间戳>.jsonl，每条 = {t, len, data(hex), txt(可打印化)}

use frida::{DeviceManager, Frida, Message, ScriptHandler, ScriptOption, SpawnOptions};
use std::io::Write;

const JS: &str = r#"
// ---- 编码工具 ----
function hexbuf(p, n){ try { return Array.from(new Uint8Array(p.readByteArray(Math.min(n, 8192)))).map(b=>b.toString(16).padStart(2,'0')).join(''); } catch(e){ return '<err>'; } }
function printable(p, n){
  try {
    const bytes = new Uint8Array(p.readByteArray(Math.min(n, 8192)));
    let s = '';
    for (const b of bytes) s += (b>=32 && b<127) ? String.fromCharCode(b) : '.';
    return s;
  } catch(e){ return '<err>'; }
}
// ws2_32 connect 记录（标 entrance 连接：连到 entrance-new-pd / 139.x 等）
const entranceAddrs = [];
function parseSockaddr(p){
  try {
    if (p.readU16() === 2) {
      const port = (p.add(2).readU8()<<8) | p.add(3).readU8();
      return [p.add(4).readU8(),p.add(5).readU8(),p.add(6).readU8(),p.add(7).readU8()].join('.')+':'+port;
    }
  } catch(e){}
  return '?';
}

// ---- SSL 明文挂钩（等 gmesdk+libgmessl 都加载后挂钩）----
// entrance WSS 的 TLS 走 gmesdk.dll（导入 libgmessl SSL_read/write）。
// 关键：SSL_read/write 是 gmesdk 经 IAT 调的，挂钩 libgmessl 导出即命中。
let sslHooked = false;
function hookSSL(){
  if (sslHooked) return;
  let gmessl, gmesdk;
  try { gmessl = Process.getModuleByName('libgmessl-1_1-x64.dll'); } catch(e){}
  try { gmesdk = Process.getModuleByName('gmesdk.dll'); } catch(e){}
  if (!gmessl || !gmesdk) { setTimeout(hookSSL, 300); return; }
  sslHooked = true;
  send({t:'info', msg:'ssl base=' + gmessl.base + ' gmesdk base=' + gmesdk.base});
  function attachRead(name){
    const f = gmessl.findExportByName(name);
    if(!f) { send({t:'info', msg:'no export '+name}); return; }
    Interceptor.attach(f, {
      onEnter(a){ this.buf=a[1]; this.n=a[2].toInt32(); },
      onLeave(rv){ const n=rv.toInt32(); if(n>0) send({t:name, len:n, data:hexbuf(this.buf,n), txt:printable(this.buf,n)}); }
    });
    send({t:'info', msg:'hooked read '+name});
  }
  function attachWrite(name){
    const f = gmessl.findExportByName(name);
    if(!f) { send({t:'info', msg:'no export '+name}); return; }
    Interceptor.attach(f, { onEnter(a){ const n=a[2].toInt32(); if(n>0) send({t:name, len:n, data:hexbuf(a[1],n), txt:printable(a[1],n)}); } });
    send({t:'info', msg:'hooked write '+name});
  }
  attachRead('SSL_read'); attachRead('SSL_read_ex');
  attachWrite('SSL_write'); attachWrite('SSL_write_ex');
  send({t:'info', msg:'ssl hooks on'});
}
hookSSL();

// ---- ws2_32 connect 对照 ----
let ws;
try { ws = Process.getModuleByName('ws2_32.dll'); } catch(e){ setTimeout(arguments.callee, 500); }
if (ws) {
  Interceptor.attach(ws.findExportByName('connect'), {
    onEnter(a){ const addr=parseSockaddr(a[1]); send({t:'connect', addr:addr}); }
  });
}
"#;

struct Handler {
    out: std::fs::File,
}
impl ScriptHandler for Handler {
    fn on_message(&mut self, message: Message, _data: Option<Vec<u8>>) {
        let raw = match &message {
            Message::Other(v) => v.get("data").and_then(|d| d.as_str()).map(String::from),
            Message::Error(e) => {
                eprintln!("[script error] {}", e.description);
                None
            }
            _ => None,
        };
        let Some(raw) = raw else { return };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else { return };
        let line = serde_json::to_string(&v).unwrap();
        let _ = writeln!(self.out, "{line}");
        let _ = self.out.flush();
        let p = &v["payload"];
        match p["t"].as_str().unwrap_or("?") {
            "info" => println!("[info] {}", p["msg"].as_str().unwrap_or("")),
            "connect" => println!("[connect] {}", p["addr"].as_str().unwrap_or("?")),
            t => {
                let len = p["len"].as_i64().unwrap_or(0);
                let txt = p["txt"].as_str().unwrap_or("");
                println!("[{t}] len={len} txt={}", &txt[..txt.len().min(160)]);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sce = args
        .first()
        .filter(|s| !s.is_empty())
        .cloned()
        .unwrap_or_else(|| concat!(env!("CARGO_MANIFEST_DIR"), "/runtime/scegame.exe").to_string());
    let secs: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(90);
    let cwd = std::path::Path::new(&sce).parent().unwrap().to_path_buf();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let out_path = std::env::current_exe()
        .map(|e| e.with_file_name(format!("wss_capture_{ts}.jsonl")))
        .unwrap();

    let frida = unsafe { Frida::obtain() };
    let dm = DeviceManager::obtain(&frida);
    let mut device = dm.get_local_device().expect("获取本机设备失败");

    let argv = vec![
        sce.clone(),
        "-env=game".into(),
        "-server=editor-pd.spark.xd.com".into(),
        "-use_local_res".into(),
        "-no_update".into(),
        "-width=1280".into(),
        "-height=720".into(),
    ];
    let cwd_c = std::ffi::CString::new(cwd.to_string_lossy().to_string()).unwrap();
    let options = SpawnOptions::new().argv(argv).cwd(cwd_c.as_c_str());
    let pid = device.spawn(&sce, &options).expect("spawn 失败");
    println!("spawned pid {pid}（挂起）-> {}", out_path.display());

    let session = device.attach(pid).expect("attach 失败");
    let mut script = session
        .create_script(JS, &mut ScriptOption::default())
        .expect("create_script 失败");
    let out = std::fs::File::create(&out_path).expect("建输出文件失败");
    script.handle_message(Handler { out }).expect("handle_message 失败");
    script.load().expect("load 失败");
    println!("hooks 装完，resume 开始登录...");
    device.resume(pid).expect("resume 失败");

    std::thread::sleep(std::time::Duration::from_secs(secs));
    drop(script);
    drop(session);
    let _ = device.kill(pid);
    println!("done -> {}", out_path.display());
}
