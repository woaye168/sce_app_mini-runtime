# 一次性探针：解析 GetActorTableEntry 实际地址 + dump 注册表管理器/容器/条目 + set_asset vtable 分派 + 特效注册表函数命中
# 依据 render-18 §6 hook 点（sceengine.dll api13, imagebase 0x180000000）
# 用法：python registry_probe.py [秒数]
import frida, sys, time, json, os

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 120
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\registry_probe.jsonl"

JS_HOOK = r"""
const mod = Process.getModuleByName('SCEEngine.dll');
const base = mod.base;
const RVA = {
  createActorImpl: 0x16fa290,   // CreateActor native impl（VA 0x1816fa290）
  callSite:        0x16fa2f1,   // lea rdx 指令点（VA 0x1816fa2f1）：此处 rax=manager vtable（0x16fa2f5 的 call 指令 frida17 拒钩，前移 4 字节）
  setAssetCall:    0x1345aaf,   // set_asset wrapper 内 call [rax+0xa0] 精确指令地址（VA 0x181345aaf，已反汇编确认）
  effRegistry:     0x129fb50,   // 特效注册表函数候选（VA 0x18129fb50）
};
let getEntryAddr = null;
let getEntryHooked = false;
let managerPtr = null;

function cstr(p, max) {
  try { return p.readUtf8String(max || 200); } catch (e) { return null; }
}
function dumpQwords(p, n, tags) {
  const rows = [];
  for (let i = 0; i < n; i++) {
    let v;
    try { v = p.add(i * 8).readPointer(); } catch (e) { break; }
    const row = { off: i * 8, val: v.toString() };
    // 指针指向内存时试读前 16 字节当 ascii（辅助辨认字符串成员）
    try {
      const s = v.readUtf8String(16);
      if (s && /^[\x20-\x7e]{4,}/.test(s)) row.ascii = s;
    } catch (e) {}
    rows.push(row);
  }
  return rows;
}
function hookGetEntry(addr) {
  if (getEntryHooked) return;
  getEntryHooked = true;
  send({ t: 'getentry.resolved', addr: addr.toString(), rva: addr.sub(base).toString() });
  Interceptor.attach(addr, {
    onEnter(args) {
      this.mgr = this.context.rcx;
      this.outPtr = this.context.rdx;
      const key = this.context.r8;
      const link = cstr(key.add(8).readPointer());
      send({ t: 'getentry.enter', mgr: this.mgr.toString(), keyLo: key.readU64().toString(), link: link });
    },
    onLeave(ret) {
      let entry = null;
      try { entry = this.outPtr.readPointer(); } catch (e) {}
      send({ t: 'getentry.leave', ret: ret.toString(), entry: entry ? entry.toString() : null });
      if (managerPtr === null && entry && !entry.isNull()) {
        managerPtr = this.mgr;
        send({ t: 'mgr.dump', mgr: managerPtr.toString(), qwords: dumpQwords(managerPtr, 64) });
        send({ t: 'entry.dump', entry: entry.toString(), qwords: dumpQwords(entry, 40) });
      }
    }
  });
}

// 1) CreateActor impl 入口：确认存活 + dump key link
try {
Interceptor.attach(base.add(RVA.createActorImpl), {
  onEnter(args) {
    try {
      const key = this.context.r9;
      const link = cstr(key.add(8).readPointer());
      send({ t: 'createActor.enter', this: this.context.rcx.toString(), link: link });
    } catch (e) { send({ t: 'createActor.enter', err: e.toString() }); }
  }
});
} catch (e) { send({ t: 'attach_fail', which: 'createActorImpl', err: e.toString() }); }

// 2) lea rdx 指令点（call 前 4 字节，rax 已是 manager vtable）：解析 GetActorTableEntry 实际地址
try {
Interceptor.attach(base.add(RVA.callSite), {
  onEnter(args) {
    try {
      const vtbl = this.context.rax;
      const fn = vtbl.add(0x50).readPointer();
      send({ t: 'callsite', vtbl: vtbl.toString(), mgr: this.context.rcx.toString(), fn: fn.toString() });
      hookGetEntry(fn);
    } catch (e) { send({ t: 'callsite', err: e.toString() }); }
  }
});
} catch (e) { send({ t: 'attach_fail', which: 'callSite', err: e.toString() }); }

// 3) set_asset wrapper 内 call [rax+0xa0] 指令点：dump actor 对象/vtable/impl 地址/路径串
try {
Interceptor.attach(base.add(RVA.setAssetCall), {
  onEnter(args) {
    try {
      const actor = this.context.rcx;
      const vtbl = this.context.rax;
      const impl = vtbl.add(0xa0).readPointer();
      const strObj = this.context.rdx;
      let path = null, pathAlt = null;
      try { path = strObj.add(8).readPointer().readUtf8String(200); } catch (e) {}
      try { pathAlt = strObj.add(0x10).readPointer().readUtf8String(200); } catch (e) {}
      send({
        t: 'setasset.dispatch', actor: actor.toString(), vtbl: vtbl.toString(),
        impl: impl.toString(), implRva: impl.sub(base).toString(),
        path: path, pathAlt: pathAlt,
        strQwords: dumpQwords(strObj, 4)
      });
    } catch (e) { send({ t: 'setasset.dispatch', err: e.toString() }); }
  }
});
} catch (e) { send({ t: 'attach_fail', which: 'setAssetCall', err: e.toString() }); }

// 4) 特效注册表函数：命中记录（归属判定用）
try {
Interceptor.attach(base.add(RVA.effRegistry), {
  onEnter(args) {
    let s = null;
    // lua 绑定形态：rsi=lua_State*, rdi=引擎对象；试读 rdx/rcx 附近字符串不保证，先记命中与对象
    send({ t: 'effreg.hit', rdi: this.context.rdi.toString(), rsi: this.context.rsi.toString() });
  }
});
} catch (e) { send({ t: 'attach_fail', which: 'effRegistry', err: e.toString() }); }

send({ t: 'hooked', base: base.toString(), pid: Process.id });
"""

def find_game_pids():
    """枚举进程，返回含 SCEEngine.dll 的 pid 列表（游戏态进程）"""
    dev = frida.get_local_device()
    hits = []
    for p in dev.enumerate_processes():
        nm = p.name.lower()
        if nm in ('sce', 'sce.exe'):
            hits.append(p.pid)
    return hits

def main():
    seen_sessions = []
    def on_msg(msg, data):
        if msg.get("type") == "send":
            line = json.dumps(msg, ensure_ascii=False)
            print(line, flush=True)
            with open(OUT, "a", encoding="utf-8") as f:
                f.write(line + "\n")
        elif msg.get("type") == "error":
            print(json.dumps(msg, ensure_ascii=False), flush=True)

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
                # 确认模块存在（getModuleByName 失败会在 script load 时抛错）
                attached.add(pid)
                seen_sessions.append(s)
                print(json.dumps({"t": "attached", "pid": pid}), flush=True)
            except Exception as e:
                attached.add(pid)  # 避免刷屏重试
                print(json.dumps({"t": "attach_skip", "pid": pid, "err": repr(e)[:120]}), flush=True)
        time.sleep(0.3)
    for s in seen_sessions:
        try: s.detach()
        except Exception: pass
    print("done", flush=True)

if __name__ == "__main__":
    main()
