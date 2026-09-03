//! Entrance 协议帧明文 dump（云变量 ScoreArchive 逆向工具，已实证）。
//!
//! 原理：hook scegame 进程 native 两点（TLS 之前，直接拿明文帧）：
//!   1. Entrance 发送函数入口（默认 RVA 0x1e87be0）：send(conn, msgid=edx, frame=r8, len=r9d)
//!   2. "Receive entrance message" 日志点（默认 RVA 0x1e85f59）：msgid=[rsp+0x50]，完整帧=[rbp-0x18]
//! 附带 libgmessl SSL_read/write 与 ws2_32 send/recv/WSASend/WSARecv/sendto/recvfrom 挂钩（getpeername 归属）。
//!
//! 用法：
//!   cargo run --release --example entrance_sniff -- spawn <scegame路径> <工作目录> <秒数> <输出.jsonl> [exe参数...]
//!   cargo run --release --example entrance_sniff -- attach <pid> <秒数> <输出.jsonl>
//!   可选环境变量：ENT_SEND_RVA / ENT_RECV_RVA（换引擎构建时重定位，用 find_xref 找
//!                 "Send message to entrance, message : 0x%X" / "Receive entrance message"）
//!
//! 输出 jsonl 每行：{t:'ent_send'|'ent_recv'|'send'|... , msgid/edx, frame(hex), ...}
//! 离线解码：CEProto 结构见 doc/research/lowlevel/cloudvar-04-protocol.md。
//!
//! 定位记录（scegame BuildPCBox v152，tester_1089）：send RVA 0x1e87be0 / recv RVA 0x1e85f59。
//! 前置：LIBCLANG_PATH（bindgen 需要；pip install libclang 后在 clang/native 下）。

use frida::{DeviceManager, Frida, Message, ScriptHandler, ScriptOption, SpawnOptions};
use std::io::Write;

const JS: &str = r#"
const SEND_RVA = __SEND_RVA__;
const RECV_RVA = __RECV_RVA__;
function hexbuf(p, n){ try { return Array.from(new Uint8Array(p.readByteArray(Math.min(n, 16384)))).map(b=>b.toString(16).padStart(2,'0')).join(''); } catch(e){ return null; } }
function printable(p, n){
  try {
    const bytes = new Uint8Array(p.readByteArray(Math.min(n, 4096)));
    let s = '';
    for (const b of bytes) s += (b>=32 && b<127) ? String.fromCharCode(b) : '.';
    return s;
  } catch(e){ return '<err>'; }
}
function parseSockaddr(p){
  try {
    if (p.readU16() === 2) {
      const port = (p.add(2).readU8()<<8) | p.add(3).readU8();
      return [p.add(4).readU8(),p.add(5).readU8(),p.add(6).readU8(),p.add(7).readU8()].join('.')+':'+port;
    }
  } catch(e){}
  return '?';
}

// ---- libgmessl SSL（走 gmesdk 的 TLS 通道才有效；Entrance WSS 走 libhv 静态 OpenSSL，此处抓不到）----
let sslHooked = false;
function hookSSL(){
  if (sslHooked) return;
  let gmessl;
  try { gmessl = Process.getModuleByName('libgmessl-1_1-x64.dll'); } catch(e){}
  if (!gmessl) { setTimeout(hookSSL, 500); return; }
  sslHooked = true;
  function attachRead(name){
    const f = gmessl.findExportByName(name);
    if(!f) return;
    Interceptor.attach(f, {
      onEnter(a){ this.buf=a[1]; },
      onLeave(rv){ const n=rv.toInt32(); if(n>0) send({t:name, len:n, data:hexbuf(this.buf,n), txt:printable(this.buf,n)}); }
    });
  }
  function attachWrite(name){
    const f = gmessl.findExportByName(name);
    if(!f) return;
    Interceptor.attach(f, { onEnter(a){ const n=a[2].toInt32(); if(n>0) send({t:name, len:n, data:hexbuf(a[1],n), txt:printable(a[1],n)}); } });
  }
  attachRead('SSL_read'); attachRead('SSL_read_ex');
  attachWrite('SSL_write'); attachWrite('SSL_write_ex');
  send({t:'info', msg:'ssl hooks on'});
}
hookSSL();

// ---- ws2_32 全通道（connect 抓不全：libhv 用 ConnectEx，故用 getpeername 兜底归属）----
let ws;
try { ws = Process.getModuleByName('ws2_32.dll'); } catch(e){}
if (ws) {
  const getpeername = new NativeFunction(ws.findExportByName('getpeername'), 'int', ['pointer','pointer','pointer']);
  const saBuf = Memory.alloc(64);
  const saLen = Memory.alloc(8);
  const addrCache = {};
  function resolveAddr(s){
    if (addrCache[s]) return addrCache[s];
    try {
      saLen.writeU32(64);
      if (getpeername(ptr(s), saBuf, saLen) === 0 && saBuf.readU16() === 2) {
        const port = (saBuf.add(2).readU8()<<8) | saBuf.add(3).readU8();
        const a = saBuf.add(4).readU8()+'.'+saBuf.add(5).readU8()+'.'+saBuf.add(6).readU8()+'.'+saBuf.add(7).readU8()+':'+port;
        addrCache[s] = a; return a;
      }
    } catch(e){}
    return '?';
  }
  Interceptor.attach(ws.findExportByName('connect'), {
    onEnter(a){ this.s = a[0].toInt32(); this.addr = parseSockaddr(a[1]); },
    onLeave(rv){ if (rv.toInt32() === 0) { addrCache[this.s] = this.addr; } send({t:'connect', addr:this.addr}); }
  });
  const MAXDUMP = 1024;
  Interceptor.attach(ws.findExportByName('send'), {
    onEnter(a){
      const s = a[0].toInt32(); const n = a[2].toInt32();
      if (n > 0) send({t:'send', sock:s, addr:resolveAddr(s), len:n, data:hexbuf(a[1],Math.min(n,MAXDUMP)), txt:printable(a[1],Math.min(n,MAXDUMP))});
    }
  });
  Interceptor.attach(ws.findExportByName('recv'), {
    onEnter(a){ this.s = a[0].toInt32(); this.buf = a[1]; },
    onLeave(rv){
      const n = rv.toInt32();
      if (n > 0) send({t:'recv', sock:this.s, addr:resolveAddr(this.s), len:n, data:hexbuf(this.buf,Math.min(n,MAXDUMP)), txt:printable(this.buf,Math.min(n,MAXDUMP))});
    }
  });
  function dumpWSABuf(bufs){
    try {
      const n0 = bufs.readU32(); const b0 = bufs.add(Process.pointerSize).readPointer();
      if (n0 > 0 && n0 < 16777216) return {p:b0, n:n0};
    } catch(e){}
    return null;
  }
  Interceptor.attach(ws.findExportByName('WSASend'), {
    onEnter(a){
      this.s = a[0].toInt32(); const d = dumpWSABuf(a[1]);
      if (d) send({t:'WSASend', sock:this.s, addr:resolveAddr(this.s), len:d.n, data:hexbuf(d.p,Math.min(d.n,MAXDUMP)), txt:printable(d.p,Math.min(d.n,MAXDUMP))});
    }
  });
  Interceptor.attach(ws.findExportByName('WSARecv'), {
    onEnter(a){ this.s = a[0].toInt32(); this.d = dumpWSABuf(a[1]); this.cnt = a[3]; },
    onLeave(rv){
      // SOCKET_ERROR（含 overlapped 未完成）时计数指针不可靠，整条丢弃；
      // 计数读取失败也不再回退用缓冲全长（避免把未接收的垃圾当收包 dump）
      if (rv.toInt32() !== 0) return;
      if (this.d) {
        let n;
        try { const got = this.cnt.readU32(); if (got <= 0) return; n = Math.min(got, this.d.n); } catch(e){ return; }
        send({t:'WSARecv', sock:this.s, addr:resolveAddr(this.s), len:n, data:hexbuf(this.d.p,Math.min(n,MAXDUMP)), txt:printable(this.d.p,Math.min(n,MAXDUMP))});
      }
    }
  });
  Interceptor.attach(ws.findExportByName('sendto'), {
    onEnter(a){
      const s = a[0].toInt32(); const n = a[2].toInt32();
      let addr = parseSockaddr(a[4]); if (addr === '?') addr = resolveAddr(s);
      if (n > 0) send({t:'sendto', sock:s, addr:addr, len:n, data:hexbuf(a[1],Math.min(n,MAXDUMP)), txt:printable(a[1],Math.min(n,MAXDUMP))});
    }
  });
  Interceptor.attach(ws.findExportByName('recvfrom'), {
    onEnter(a){ this.s = a[0].toInt32(); this.buf = a[1]; this.from = a[4]; },
    onLeave(rv){
      const n = rv.toInt32();
      if (n > 0) {
        let addr = parseSockaddr(this.from); if (addr === '?') addr = resolveAddr(this.s);
        send({t:'recvfrom', sock:this.s, addr:addr, len:n, data:hexbuf(this.buf,Math.min(n,MAXDUMP)), txt:printable(this.buf,Math.min(n,MAXDUMP))});
      }
    }
  });
  send({t:'info', msg:'ws2_32 hooks on'});
}

// ---- Entrance native 帧 hook ----
function hookEntrance(mod){
  const sendFn = mod.base.add(SEND_RVA);
  Interceptor.attach(sendFn, {
    onEnter(a){
      const n = a[3].toInt32() >>> 0;
      const rec = {t:'ent_send', msgid:a[1].toInt32() >>> 0, len:n};
      if (n > 0 && n < 1048576) { const h = hexbuf(a[2], n); if (h) rec.frame = h; }
      send(rec);
    }
  });
  const recvLog = mod.base.add(RECV_RVA);
  Interceptor.attach(recvLog, {
    onEnter(a){
      const c = this.context;
      const rec = {t:'ent_recv', msgid:(c.rsp.add(0x50).readU32() >>> 0)};
      try {
        const fp = c.rbp.sub(0x18).readPointer();
        function rdvarint(p){ let v=0,s=0,i=0; for(;;){ const x=p.add(i).readU8(); i++; v|=(x&0x7f)<<s; if(!(x&0x80))break; s+=7; if(i>9)break; } return {v:v, n:i}; }
        let o = 0;
        const t1 = rdvarint(fp); o += t1.n;
        const m1 = rdvarint(fp.add(o)); o += m1.n;
        const t2 = rdvarint(fp.add(o)); o += t2.n;
        const l2 = rdvarint(fp.add(o)); o += l2.n;
        const total = o + l2.v;
        if (total > 0 && total < 1048576) { const h = hexbuf(fp, Math.min(total, 16384)); if (h) rec.frame = h; }
      } catch(e){ rec.err = String(e); }
      send(rec);
    }
  });
  send({t:'info', msg:'entrance native hooks on'});
}
function waitModule(){
  let mod = null;
  // 编辑器/wineditor 构建（version-<api> 运行时）Entrance 在 sceengine.dll；
  // scegame（BuildPCBox tester）在主 exe。先 dll 后 exe，避免 exe 基址误配 dll RVA。
  for (const name of ['SCEEngine.dll', 'sceengine.dll', 'scegame.exe', 'scegame', 'SCE', 'SCE.exe', 'sce']) {
    try { mod = Process.getModuleByName(name); if (mod) break; } catch(e){}
  }
  if (!mod) { setTimeout(waitModule, 300); return; }
  send({t:'info', msg:'entrance module: ' + mod.name});
  hookEntrance(mod);
}
waitModule();
"#;

static OUT_PATH: std::sync::OnceLock<String> = std::sync::OnceLock::new();

struct Handler;
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
        // frida 回调线程里 handler 实例状态不可靠（实测 File 句柄 os error 6 / String 读到 NUL），
        // 输出路径走全局 OnceLock，逐条 append 打开最稳
        if let Some(path) = OUT_PATH.get() {
            match std::fs::OpenOptions::new().append(true).open(path) {
                Ok(mut f) => {
                    let _ = writeln!(f, "{line}");
                    let _ = f.flush();
                }
                Err(e) => eprintln!("[open err] {e}"),
            }
        }
        let p = &v["payload"];
        match p["t"].as_str().unwrap_or("?") {
            "info" => println!("[info] {}", p["msg"].as_str().unwrap_or("")),
            "connect" => println!("[connect] {}", p["addr"].as_str().unwrap_or("?")),
            t @ ("ent_send" | "ent_recv") => {
                println!(
                    "[{t}] msgid={:#x} len={}",
                    p["msgid"].as_u64().unwrap_or(0),
                    p["len"].as_u64().unwrap_or(0)
                );
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("用法:");
        eprintln!("  entrance_sniff spawn <scegame路径> <工作目录> <秒数> <输出.jsonl> [exe参数...]");
        eprintln!("  entrance_sniff attach <pid> <秒数> <输出.jsonl>");
        eprintln!("环境变量: ENT_SEND_RVA / ENT_RECV_RVA（默认 0x1e87be0 / 0x1e85f59，scegame v152）");
        std::process::exit(1);
    }
    let send_rva = std::env::var("ENT_SEND_RVA").unwrap_or_else(|_| "0x1e87be0".into());
    let recv_rva = std::env::var("ENT_RECV_RVA").unwrap_or_else(|_| "0x1e85f59".into());
    let js = JS.replace("__SEND_RVA__", &send_rva).replace("__RECV_RVA__", &recv_rva);

    let frida = unsafe { Frida::obtain() };
    let dm = DeviceManager::obtain(&frida);
    let mut device = dm.get_local_device().expect("获取本机设备失败");

    let (pid, spawned, secs, out_path): (u32, bool, u64, String) = match args[0].as_str() {
        "spawn" => {
            let exe = &args[1];
            let cwd = &args[2];
            let secs: u64 = args[3].parse().expect("秒数");
            let out = args[4].clone();
            let extra: Vec<String> = args.iter().skip(5).cloned().collect();
            let mut argv = vec![exe.clone()];
            argv.extend(extra);
            let cwd_c = std::ffi::CString::new(cwd.as_str()).unwrap();
            let options = SpawnOptions::new().argv(argv).cwd(cwd_c.as_c_str());
            let pid = device.spawn(exe, &options).expect("spawn 失败");
            (pid, true, secs, out)
        }
        "attach" => {
            let pid: u32 = args[1].parse().expect("pid");
            let secs: u64 = args[2].parse().expect("秒数");
            let out = args[3].clone();
            (pid, false, secs, out)
        }
        _ => {
            eprintln!("未知模式 {}", args[0]);
            std::process::exit(1);
        }
    };

    println!("pid={pid} -> {out_path}");
    let session = device.attach(pid).expect("attach 失败");
    let mut script = session
        .create_script(js.as_str(), &mut ScriptOption::default())
        .expect("create_script 失败");
    std::fs::write(&out_path, "{\"marker\":\"file-ok\"}\n").expect("建输出文件失败");
    let _ = OUT_PATH.set(out_path.clone());
    script.handle_message(Handler).expect("handle_message 失败");
    script.load().expect("load 失败");
    if spawned {
        device.resume(pid).expect("resume 失败");
        println!("hooks 装完，resume");
    }
    std::thread::sleep(std::time::Duration::from_secs(secs));
    drop(script);
    drop(session);
    if spawned {
        let _ = device.kill(pid);
    }
    println!("done -> {out_path}");
}
