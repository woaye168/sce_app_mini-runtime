# 一次性探针：hook UnitChangeModel 本地应用函数（SCEEngine.dll RVA 0x1785350）
# dump 入参新串 + unit+0x550 当前存储串，揭示引擎期望的模型路径形态
# 用法：python unit_model_hook.py [秒数] [pid ...]
import frida, sys, time, json

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 90
PIDS = [int(x) for x in sys.argv[2:]] or [56864, 34036]
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\unit_model_hook.jsonl"

JS_HOOK = r"""
const base = Process.getModuleByName('SCEEngine.dll').base;
const fn = base.add(0x1785350);
function readStrStruct(p) {
  try {
    const len = p.readU32();
    const ptr = p.add(8).readPointer();
    if (len === 0 || ptr.isNull()) return null;
    return ptr.readUtf8String(len);
  } catch (e) { return '<err>'; }
}
Interceptor.attach(fn, {
  onEnter(args) {
    const unit = this.context.rcx;
    const strp = this.context.rdx;
    send({
      t: 'apply',
      newStr: readStrStruct(strp),
      cur: readStrStruct(unit.add(0x550))
    });
  }
});
send({ t: 'hooked', base: base.toString() });
"""

sessions = []
def mk(pid):
    def on_msg(msg, data):
        if msg.get("type") == "send":
            msg["payload"]["pid"] = pid
        line = json.dumps(msg, ensure_ascii=False)
        print(line, flush=True)
        with open(OUT, "a", encoding="utf-8") as f:
            f.write(line + "\n")
    return on_msg

for pid in PIDS:
    try:
        s = frida.attach(pid)
        sc = s.create_script(JS_HOOK)
        sc.on("message", mk(pid))
        sc.load()
        sessions.append(s)
        print("hooked pid", pid, flush=True)
    except Exception as e:
        print("pid", pid, "ERR", repr(e)[:100], flush=True)

if not sessions:
    sys.exit(1)
time.sleep(DUR)
for s in sessions:
    try: s.detach()
    except Exception: pass
print("done", flush=True)
