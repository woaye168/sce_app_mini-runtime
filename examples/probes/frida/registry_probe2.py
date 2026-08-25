# 一次性探针 v2：链式解析 set_asset impl → vfunc+0x60(manager) → vfunc+0x58/+0x50 注册表查找
# dump：manager/容器/entry 布局 + (link,typeid) 对（破哈希）
# 依据 render-18 §1.2 + registry_probe v1 实测（ModelActor::SetAsset=RVA0x17837d0, EffectActor::SetAsset=RVA0x179c0a0, 串结构{+0:len/cap,+8:ptr}）
# 用法：python registry_probe2.py [秒数]
import frida, sys, time, json

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 120
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\registry_probe2.jsonl"

JS_HOOK = r"""
const mod = Process.getModuleByName('SCEEngine.dll');
const base = mod.base;
const hookedMgr = {};
const hookedFn60 = {};
let mgrDumped = false;
let entryDumped = false;

function readEngStr(strObj) {
  try {
    const len = strObj.readU32();
    const ptr = strObj.add(8).readPointer();
    if (len === 0 || len > 512 || ptr.isNull()) return null;
    return ptr.readUtf8String(len);
  } catch (e) { return null; }
}
function dumpQwords(p, n) {
  const rows = [];
  for (let i = 0; i < n; i++) {
    let v;
    try { v = p.add(i * 8).readPointer(); } catch (e) { break; }
    const row = { off: i * 8, val: v.toString() };
    try {
      const s = v.readUtf8String(16);
      if (s && /^[\x20-\x7e]{4,}/.test(s)) row.ascii = s;
    } catch (e) {}
    rows.push(row);
  }
  return rows;
}

// 注册表查找函数（manager vfunc+0x58：typeid 键 / +0x50：link 字符串键）
// fn60 返回的是包装结构指针（栈上），真 manager = [ret]；再取 vtbl=[manager]（render-18 §1.1: mov rcx,[rax]; mov rax,[rcx]）
function hookLookup(wrapper) {
  let mgr, vtbl;
  try {
    mgr = wrapper.readPointer();
    vtbl = mgr.readPointer();
  } catch (e) { send({ t: 'mgr.resolve_fail', err: e.toString() }); return; }
  const key = vtbl.toString();
  if (hookedMgr[key]) return;
  hookedMgr[key] = true;
  const fn58 = vtbl.add(0x58).readPointer();
  const fn50 = vtbl.add(0x50).readPointer();
  send({ t: 'mgr.resolved', mgr: mgr.toString(), vtbl: key,
         fn58: fn58.toString(), fn58rva: fn58.sub(base).toString(),
         fn50: fn50.toString(), fn50rva: fn50.sub(base).toString() });
  try {
    Interceptor.attach(fn58, {
      onEnter(args) {
        this.out = this.context.rdx;
        send({ t: 'lk58.enter', mgr: this.context.rcx.toString(), typeid: this.context.r8.toString(), tid: this.threadId });
      },
      onLeave(ret) {
        let entry = null;
        try { entry = this.out.readPointer(); } catch (e) {}
        send({ t: 'lk58.leave', ret: ret.toString(), entry: entry ? entry.toString() : null, tid: this.threadId });
        if (!mgrDumped && entry && !entry.isNull()) {
          mgrDumped = true;
          send({ t: 'mgr.dump', mgr: mgr.toString(), qwords: dumpQwords(mgr, 80) });
        }
        if (!entryDumped && entry && !entry.isNull()) {
          entryDumped = true;
          send({ t: 'entry.dump', entry: entry.toString(), qwords: dumpQwords(entry, 48) });
        }
      }
    });
  } catch (e) { send({ t: 'attach_fail', which: 'fn58', err: e.toString() }); }
  try {
    Interceptor.attach(fn50, {
      onEnter(args) {
        this.out = this.context.rdx;
        let link = null;
        try { link = this.context.r8.add(8).readPointer().readUtf8String(200); } catch (e) {}
        send({ t: 'lk50.enter', mgr: this.context.rcx.toString(), link: link, tid: this.threadId });
      },
      onLeave(ret) {
        let entry = null;
        try { entry = this.out.readPointer(); } catch (e) {}
        send({ t: 'lk50.leave', ret: ret.toString(), entry: entry ? entry.toString() : null, tid: this.threadId });
      }
    });
  } catch (e) { send({ t: 'attach_fail', which: 'fn50', err: e.toString() }); }
}

// vfunc+0x60 取 manager 接口（挂一次）
function hookFn60(fn60) {
  const key = fn60.toString();
  if (hookedFn60[key]) return;
  hookedFn60[key] = true;
  try {
    Interceptor.attach(fn60, {
      onLeave(ret) {
        if (ret.isNull()) return;
        send({ t: 'fn60.ret', mgr: ret.toString() });
        hookLookup(ret);
      }
    });
  } catch (e) { send({ t: 'attach_fail', which: 'fn60', err: e.toString() }); }
}

// set_asset impl 入口：dump actor + 路径串 + 解析 [actor+0x28]→vtbl+0x60
function hookSetAsset(rva, tag) {
  try {
    Interceptor.attach(base.add(rva), {
      onEnter(args) {
        const actor = this.context.rcx;
        const s = readEngStr(this.context.rdx);
        send({ t: 'setasset.impl', tag: tag, actor: actor.toString(), str: s, tid: this.threadId });
        try {
          const comp = actor.add(0x28).readPointer();
          const vtbl = comp.readPointer();
          const fn60 = vtbl.add(0x60).readPointer();
          send({ t: 'chain', tag: tag, comp: comp.toString(), fn60: fn60.toString(), fn60rva: fn60.sub(base).toString() });
          hookFn60(fn60);
        } catch (e) { send({ t: 'chain', tag: tag, err: e.toString() }); }
      }
    });
    send({ t: 'hooked', tag: tag, rva: rva.toString(16) });
  } catch (e) { send({ t: 'attach_fail', which: tag, err: e.toString() }); }
}

hookSetAsset(0x17837d0, 'ModelActor');
hookSetAsset(0x179c0a0, 'EffectActor');
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
            print(json.dumps(msg, ensure_ascii=False)[:300], flush=True)

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
                print(json.dumps({"t": "attached", "pid": pid}), flush=True)
            except Exception as e:
                attached.add(pid)
                print(json.dumps({"t": "attach_skip", "pid": pid, "err": repr(e)[:120]}), flush=True)
        time.sleep(0.3)
    for s in sessions:
        try: s.detach()
        except Exception: pass
    print("done", flush=True)

if __name__ == "__main__":
    main()
