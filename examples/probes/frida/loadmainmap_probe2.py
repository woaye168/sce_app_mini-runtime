# 一次性探针 v2：钩 ctx getter thunk（0x181cbcb01）观察引擎自调 → 捕获 (L, ctx) → 直调 LoadMainMap
# 触发：G35 game.unit_change_model（render-06 已实证 PIE 可用）+ G28 set_asset
import frida, sys, time, json

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 90
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\loadmainmap_probe2.jsonl"
MAP_ARG = 'C:/Users/woaye/Documents/SCE Projects/probe_map001'

JS_HOOK = r"""
const base = Process.getModuleByName('SCEEngine.dll').base;
const lua = Process.getModuleByName('lua54.dll');
const loadMainMap = new NativeFunction(base.add(0x1337240), 'int', ['pointer', 'pointer']);
const resetWrapper = new NativeFunction(base.add(0x1337a70), 'int', ['pointer']);
const loadMainMapWrapper = new NativeFunction(base.add(0x1337a40), 'int', ['pointer']);
const lua_pushstring = new NativeFunction(lua.getExportByName('lua_pushstring'), 'pointer', ['pointer', 'pointer']);
const lua_settop = new NativeFunction(lua.getExportByName('lua_settop'), 'void', ['pointer', 'int']);
const MAP_ARG = '""" + MAP_ARG + r"""';
const mapArgBuf = Memory.allocUtf8String(MAP_ARG);
let called = false;

Interceptor.attach(base.add(0x1cbcb01), {
  onEnter(args) {
    this.L = this.context.rcx;
    this.magic = this.context.rdx;
  },
  onLeave(ret) {
    send({ t: 'getter', L: this.L.toString(), magic: this.magic.toString(16), ctx: ret.toString() });
    if (called || ret.isNull()) return;
    // 验证 ctx 有效：[ctx+0x10] 非空（impl 首句即读它；壳进程的 ctx 此域为空→上次 AV 教训）
    let c10 = NULL;
    try { c10 = ret.add(0x10).readPointer(); } catch (e) { send({ t: 'ctx.invalid' }); return; }
    if (c10.isNull()) { send({ t: 'ctx.skip', ctx: ret.toString() }); return; }
    called = true;
    try {
      const ctx = ret;
      for (const off of [0xc8, 0xd0]) {
        try {
          const len = ctx.add(off).readU32();
          const ptr = ctx.add(off + 8).readPointer();
          if (len > 0 && len < 512 && !ptr.isNull()) {
            send({ t: 'ctx.str', off: off, s: ptr.readUtf8String(len) });
          } else {
            send({ t: 'ctx.str', off: off, len: len });
          }
        } catch (e) { send({ t: 'ctx.str', off: off, err: e.toString() }); }
      }
      lua_settop(this.L, 0);
      // reset wrapper 只需 L（内部自取 ctx）：先拆再载
      send({ t: 'reset.begin' });
      const rr = resetWrapper(this.L);
      send({ t: 'reset.ret', ret: rr });
      lua_pushstring(this.L, mapArgBuf);
      send({ t: 'call.begin', arg: MAP_ARG });
      const r = loadMainMapWrapper(this.L);
      send({ t: 'call.ret', ret: r });
    } catch (e) {
      send({ t: 'call.err', err: e.toString(), stack: e.stack });
    }
  }
});
send({ t: 'ready', base: base.toString(), pid: Process.id });
"""

def find_game_pids():
    dev = frida.get_local_device()
    return [p.pid for p in dev.enumerate_processes() if p.name.lower() in ('sce', 'sce.exe')]

def main():
    sessions = []
    def on_msg(msg, data):
        if msg.get("type") == "send":
            line = json.dumps(msg["payload"], ensure_ascii=False)
            print(line, flush=True)
            with open(OUT, "a", encoding="utf-8") as f:
                f.write(line + "\n")
        elif msg.get("type") == "error":
            print(json.dumps(msg, ensure_ascii=False)[:400], flush=True)

    deadline = time.time() + DUR
    attached = set()
    while time.time() < deadline:
        for pid in find_game_pids():
            if pid in attached:
                continue
            try:
                s = frida.attach(pid)
                sc = s.create_script(JS_HOOK)
                sc.on("message", on_msg)
                sc.load()
                attached.add(pid)
                sessions.append(s)
            except Exception:
                attached.add(pid)
        time.sleep(0.3)
    for s in sessions:
        try: s.detach()
        except Exception: pass
    print("done", flush=True)

if __name__ == "__main__":
    main()
