# schannel EncryptMessage/DecryptMessage 抓 update-info 请求/响应明文
import sys, time, frida

JS = r"""
function bufData(pMsgDesc) {
    // SecBufferDesc: ulVersion(4) cBuffers(4) pBuffers(ptr)
    let out = [];
    try {
        const cBuf = pMsgDesc.add(4).readU32();
        const pBufs = pMsgDesc.add(8).readPointer();
        for (let i = 0; i < cBuf && i < 16; i++) {
            const b = pBufs.add(i * 16);
            const cb = b.readU32();
            const ty = b.add(4).readU32();
            const pv = b.add(8).readPointer();
            if (ty === 1 && cb > 0 && cb < 1048576) {  // SECBUFFER_DATA
                out.push(pv.readByteArray(cb));
            }
        }
    } catch(e) {}
    return out;
}
function install() {
    let sch;
    try { sch = Process.getModuleByName('schannel.dll'); } catch(e) { setTimeout(install, 500); return; }
    const enc = sch.findExportByName('EncryptMessage');
    const dec = sch.findExportByName('DecryptMessage');
    if (enc) Interceptor.attach(enc, {
        onEnter(args) {
            for (const buf of bufData(args[2])) {
                const bytes = new Uint8Array(buf);
                let s = '';
                try { s = String.fromCharCode.apply(null, bytes.subarray(0, Math.min(bytes.length, 16384))); } catch(e) { return; }
                if (s.includes('update-info')) { send({t:'hit-req', data: s}); }
            }
        }
    });
    if (dec) Interceptor.attach(dec, {
        onEnter(args) { this.desc = args[2]; },
        onLeave(rv) {
            for (const buf of bufData(this.desc)) {
                const bytes = new Uint8Array(buf);
                let s = '';
                try { s = String.fromCharCode.apply(null, bytes.subarray(0, Math.min(bytes.length, 65536))); } catch(e) { return; }
                if (s.includes('"url"') || s.includes('windows_game.7z') || s.includes('version')) { send({t:'hit-resp', data: s}); }
            }
        }
    });
    send({t:'hooked'});
}
install();
"""

def on_message(msg, data):
    p = msg.get('payload')
    if msg.get('type') == 'send' and p:
        t = p.get('t')
        if t == 'hit-req':
            print('=== REQUEST ===')
            print(p['data'][:3000])
        elif t == 'hit-resp':
            print('=== RESPONSE ===')
            print(p['data'][:6000])
        elif t == 'hooked':
            print('hooked schannel')
        sys.stdout.flush()

def main():
    device = frida.get_local_device()
    pid = device.spawn([
        r"d:\sce_online\Res\maps\sce_app_mini-runtime\runtime\scegame.exe",
        "-inner", "-headless", "-server=editor-pd.spark.xd.com",
    ], cwd=r"d:\sce_online\Res\maps\sce_app_mini-runtime\runtime")
    session = device.attach(pid)
    s = session.create_script(JS)
    s.on('message', on_message)
    s.load()
    device.resume(pid)
    print('spawned', pid)
    sys.stdout.flush()
    time.sleep(45)
    try:
        device.kill(pid)
    except Exception:
        pass
    print('done')

if __name__ == '__main__':
    main()
