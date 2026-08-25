# 一次性探针：frida 直调模块级 LoadMainMap（0x181337240）——验证运行时整图/表重载
# 机制：ctxGetter(0xfff0b9d7)→游戏上下文；set_asset wrapper 入口捕 lua_State；
#       onLeave 里 lua_pushstring+lua_replace(L,1) 造参 → NativeFunction 调 impl(ctx, L)
# 观察：native 日志是否出现第二次 "Begin loading table"
import frida, sys, time, json

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 90
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\loadmainmap_probe.jsonl"
MAP_ARG = 'C:/Users/woaye/Documents/SCE Projects/test_res002'

JS_HOOK = r"""
const base = Process.getModuleByName('SCEEngine.dll').base;
const lua = Process.getModuleByName('lua54.dll');
const ctxGetter = new NativeFunction(base.add(0x1cbcb01), 'pointer', ['pointer', 'uint']);
const loadMainMap = new NativeFunction(base.add(0x1337240), 'int', ['pointer', 'pointer']);
const lua_pushstring = new NativeFunction(lua.getExportByName('lua_pushstring'), 'pointer', ['pointer', 'pointer']);
const lua_settop = new NativeFunction(lua.getExportByName('lua_settop'), 'void', ['pointer', 'int']);
const MAP_ARG = '""" + MAP_ARG + r"""';
const mapArgBuf = Memory.allocUtf8String(MAP_ARG);

function readCtxStr(ctx, off, tag) {
  try {
    const hdr = ctx.add(off).readU64();
    const len = hdr.and(0xffffffff).toNumber();
    const ptr = ctx.add(off + 8).readPointer();
    if (len > 0 && len < 512 && !ptr.isNull()) {
      const s = ptr.readUtf8String(len);
      send({ t: 'ctx.str', off: off, len: len, s: s });
    } else {
      send({ t: 'ctx.str', off: off, len: len, ptr: ptr.toString() });
    }
  } catch (e) { send({ t: 'ctx.str', off: off, err: e.toString() }); }
}

Interceptor.attach(base.add(0x1345a10), {
  onEnter(args) {
    this.L = this.context.rcx;
  },
  onLeave(ret) {
    try {
      const ctx = ctxGetter(this.L, 0xfff0b9d7); // 签名实证：getter(lua_State* L, magic)（wrapper 里 rcx=L 原样传入）
      send({ t: 'ctx', ctx: ctx.toString() });
      readCtxStr(ctx, 0xc8, 'c8');
      readCtxStr(ctx, 0xd0, 'd0');
      // 造参：settop(L,0) 清栈 → pushstring 即 index 1（lua_replace 是宏无导出，改此方案）
      lua_settop(this.L, 0);
      lua_pushstring(this.L, mapArgBuf);
      send({ t: 'call.begin', arg: MAP_ARG });
      const r = loadMainMap(ctx, this.L);
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
