# 一次性探针：hook unit_attach_model 三级调用（render-09 死亡点定位）
# wrapper 0x12a20d0 / 核心 0x18176e8b0 / 内层 0x1817af940（SCEEngine.dll RVA）
# 自动扫描所有名为 SCE 的进程，hook 含 SCEEngine.dll 的游戏态进程
# 用法：python attach_hook.py [秒数]
import frida, sys, time, json

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 120
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\attach_hook.jsonl"

JS_HOOK = r"""
const base = Process.getModuleByName('SCEEngine.dll').base;
function readStrStruct(p) {
  try {
    const len = p.readU32();
    const ptr = p.add(8).readPointer();
    if (len === 0 || ptr.isNull()) return null;
    if (len > 1024) return '<len? ' + len + '>';
    return ptr.readUtf8String(len);
  } catch (e) { return '<err>'; }
}
// wrapper：rcx=ctx, rdx=lua_State（只记入口）
Interceptor.attach(base.add(0x12a20d0), {
  onEnter(args) { send({ t: 'wrapper' }); }
});
// 核心 attach(组件=rcx, path=rdx, bool=r8b, hand=r9, hold=[rsp+0x20])（VA 0x18176e8b0 → RVA 0x176e8b0）
Interceptor.attach(base.add(0x176e8b0), {
  onEnter(args) {
    const ctx = this.context;
    send({
      t: 'core',
      path: readStrStruct(ctx.rdx),
      flag: ctx.r8.toInt32() & 0xff,
      hand: readStrStruct(ctx.r9),
      hold: readStrStruct(ctx.rsp.add(0x20).readPointer())
    });
  }
});
// 内层 VA 0x1817af940 → RVA 0x17af940（node_mgr=rcx, 组件=rdx, path=r8, 资源=r9, ...）
Interceptor.attach(base.add(0x17af940), {
  onEnter(args) {
    const ctx = this.context;
    send({ t: 'inner', path: readStrStruct(ctx.r8) });
  }
});
// 对照：change_model 本地应用（已知会触发）
Interceptor.attach(base.add(0x1785350), {
  onEnter(args) {
    send({ t: 'change_model_apply', newStr: readStrStruct(this.context.rdx) });
  }
});
send({ t: 'hooked', base: base.toString() });
"""

def on_msg_factory(pid):
    def on_msg(msg, data):
        if msg.get("type") == "send":
            msg["payload"]["pid"] = pid
        line = json.dumps(msg, ensure_ascii=False)
        print(line, flush=True)
        with open(OUT, "a", encoding="utf-8") as f:
            f.write(line + "\n")
    return on_msg

sessions = []
for p in frida.get_local_device().enumerate_processes():
    if p.name.lower() != "sce":
        continue
    try:
        s = frida.attach(p.pid)
        sc = s.create_script(JS_HOOK)
        sc.on("message", on_msg_factory(p.pid))
        sc.load()
        sessions.append(s)
        print("hooked pid", p.pid, flush=True)
    except Exception as e:
        print("pid", p.pid, "skip", repr(e)[:80], flush=True)
        try: s.detach()
        except Exception: pass

if not sessions:
    print("no SCE game process hooked", flush=True)
    sys.exit(1)
time.sleep(DUR)
for s in sessions:
    try: s.detach()
    except Exception: pass
print("done", flush=True)
