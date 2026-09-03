// ws2_32 收发 hook（frida_capture.rs 的注入脚本）
// 输出：send({ t: 'io', rec: {...} })，rec 逐行写入 jsonl
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
            // SOCKET_ERROR 时 lpNumberOfBytesRecvd 不写入，计数是陈旧值，整条丢弃
            if (rv.toInt32() !== 0) return;
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
