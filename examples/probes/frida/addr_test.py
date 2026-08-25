# 一次性探针：测试 frida 17 对若干 mid-function 地址的 attach 可行性
import frida, sys, json

PIDS = [int(x) for x in sys.argv[1:]]
ADDRS = [0x16fa290, 0x16fa2f1, 0x16fa2f5, 0x16fa2f8, 0x1345aaf, 0x129fb50]

JS = r"""
const base = Process.getModuleByName('SCEEngine.dll').base;
const addrs = %s;
for (const rva of addrs) {
  try {
    Interceptor.attach(base.add(rva), { onEnter() {} });
    send({ t: 'ok', rva: rva.toString(16) });
  } catch (e) {
    send({ t: 'fail', rva: rva.toString(16), err: e.toString().slice(0, 90) });
  }
}
send({ t: 'done', base: base.toString() });
""" % json.dumps(ADDRS)

def on_msg(msg, data):
    if msg.get("type") == "send":
        print(json.dumps(msg["payload"], ensure_ascii=False), flush=True)
    elif msg.get("type") == "error":
        print(json.dumps(msg, ensure_ascii=False)[:200], flush=True)

for pid in PIDS:
    try:
        s = frida.attach(pid)
        sc = s.create_script(JS)
        sc.on("message", on_msg)
        sc.load()
        print("loaded pid", pid, flush=True)
        import time; time.sleep(2)
        s.detach()
    except Exception as e:
        print("pid", pid, "ERR", repr(e)[:120], flush=True)
print("done", flush=True)
