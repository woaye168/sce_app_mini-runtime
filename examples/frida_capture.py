# Frida 抓取编辑器调试 launcher（version-13/SCE）的 ws2_32 收发字节
# 用途：逆向 DebugManager 控制协议（EditorLogin/UploadDelta/EditorStartGame）的真实线上格式
# 用法：python frida_capture.py <out.jsonl>
import sys, json, time, frida

SCE = r"D:\sce_online\version-13\SCE"
ARGS = [
    "-server=editor-pd.spark.xd.com", "-use_local_res", "-launcher=星火编辑器.exe",
    "-editor_api_version=13", "-no_ask_editor_api_version", "-generate_and_debug_map",
    r"-file_path=c:\Users\woaye\Documents\SCE Projects\test_res002\project.sce",
]

JS = r"""
let hooked = false;
function hex(buf, len) {
    if (len <= 0) return "";
    const n = Math.min(len, 16 * 1024 * 1024);
    try { return Array.from(new Uint8Array(buf.readByteArray(n))).map(b=>b.toString(16).padStart(2,'0')).join(''); }
    catch(e) { return "<err:" + e + ">"; }
}
function parseSockaddr(p) {
    try {
        const fam = p.readU16();
        if (fam === 2) {
            const port = ((p.add(2).readU8() << 8) | p.add(3).readU8());
            const ip = [p.add(4).readU8(), p.add(5).readU8(), p.add(6).readU8(), p.add(7).readU8()].join('.');
            return ip + ':' + port;
        }
        return 'fam=' + fam;
    } catch(e) { return '?'; }
}
const sockAddr = {};
function emit(rec) { send({ t: 'io', rec: rec }); }

function installHooks() {
    if (hooked) return;
    let ws;
    try { ws = Process.getModuleByName('ws2_32.dll'); } catch(e) { setTimeout(installHooks, 500); return; }
    hooked = true;
    emit({op:'hooks_installed'});
    Interceptor.attach(ws.findExportByName('connect'), {
        onEnter(args) { this.s = args[0].toString(); this.addr = parseSockaddr(args[1]); },
        onLeave(rv) { sockAddr[this.s] = this.addr; emit({op:'connect', sock:this.s, addr:this.addr, ret:rv.toInt32()}); }
    });
    Interceptor.attach(ws.findExportByName('send'), {
        onEnter(args) {
            const s = args[0].toString(); const len = args[2].toInt32();
            emit({op:'send', sock:s, addr:sockAddr[s]||'?', len:len, data:hex(args[1], len)});
        }
    });
    Interceptor.attach(ws.findExportByName('recv'), {
        onEnter(args) { this.s = args[0].toString(); this.buf = args[1]; this.len = args[2].toInt32(); },
        onLeave(rv) {
            const n = rv.toInt32();
            if (n > 0) emit({op:'recv', sock:this.s, addr:sockAddr[this.s]||'?', len:n, data:hex(this.buf, n)});
        }
    });
    Interceptor.attach(ws.findExportByName('WSASend'), {
        onEnter(args) {
            const s = args[0].toString();
            const cnt = args[2].toInt32();
            const bufs = args[1];
            for (let i = 0; i < cnt; i++) {
                const b = bufs.add(i * Process.pointerSize * 2);
                const len = b.readU32();
                const ptr = b.add(Process.pointerSize).readPointer();
                emit({op:'WSASend', sock:s, addr:sockAddr[s]||'?', len:len, data:hex(ptr, len)});
            }
        }
    });
    Interceptor.attach(ws.findExportByName('WSARecv'), {
        onEnter(args) { this.s = args[0].toString(); this.bufs = args[1]; this.cnt = args[2].toInt32(); this.got = args[3]; },
        onLeave(rv) {
            try {
                const n = this.got.readU32();
                if (n > 0) {
                    const b = this.bufs;
                    const len = b.readU32();
                    const ptr = b.add(Process.pointerSize).readPointer();
                    emit({op:'WSARecv', sock:this.s, addr:sockAddr[this.s]||'?', len:Math.min(len,n), data:hex(ptr, Math.min(len,n))});
                }
            } catch(e) {}
        }
    });
    Interceptor.attach(ws.findExportByName('closesocket'), {
        onEnter(args) { emit({op:'close', sock:args[0].toString()}); }
    });
}
installHooks();
"""

def main():
    out_path = sys.argv[1]
    device = frida.get_local_device()
    pid = device.spawn([SCE] + ARGS, cwd=r"D:\sce_online")
    print(f"spawned pid {pid}")
    session = device.attach(pid)
    script = session.create_script(JS)
    n = [0]
    with open(out_path, 'a', encoding='utf-8') as f:
        def on_message(msg, data):
            if msg.get('type') == 'send':
                rec = msg['payload']['rec']
                n[0] += 1
                f.write(json.dumps(rec, ensure_ascii=False) + '\n')
                f.flush()
                print(f"[{n[0]}] {rec['op']} {rec.get('addr','')} len={rec.get('len','')}", flush=True)
            else:
                print("MSG:", msg, flush=True)
        script.on('message', on_message)
        script.load()
        device.resume(pid)
        print("resumed, capturing 240s...", flush=True)
        time.sleep(240)
    print("done")

if __name__ == '__main__':
    main()
