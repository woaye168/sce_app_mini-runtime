# 一次性探针 v3：运行时改写生证——bucket 链遍历定位 MODEL 条目 + 就地改写 Asset 路径
# 机制：djb2(link) 定位 bucket → 链走查 → entry=node+0x20 → 扫描字符串字段 → 改写 Asset 指针
# 预期：G28 set_asset('$$p_55a3.model.nezha.root') 本应换哪吒；被改写后加载吉鲁鲁 prefab
# 用法：python registry_probe3.py [秒数]
import frida, sys, time, json

DUR = int(sys.argv[1]) if len(sys.argv) > 1 else 90
OUT = r"d:\sce_online\Res\maps\sce_app_mini-runtime\test\temp\registry_probe3.jsonl"

JS_HOOK = r"""
// v3.1：manager vtbl+0x60（SetAsset 实际走的字符串查找）钩返回 → 直接改返回条目
// 不再猜容器（v3 教训：fn58 的 mgr+0x230 是 ACTOR 表，MODEL 条目走 vfunc+0x60 的另一容器）
const base = Process.getModuleByName('SCEEngine.dll').base;
const OLD_PREFIX = 'characters/_user/p_55a3_nazha';
const NEW_PATH = 'characters/_user/p_55a3_jilulu_19ec_a8oz/model.prefab';
let done = false;
let lookupHooked = false;

function patchEntry(entry) {
  if (!entry || entry.isNull()) { send({ t: 'patch.nullentry' }); return; }
  // v3.3：按已确认偏移直改（+0x20={len,cap} +0x28=ptr），先显式读旧值（带错误捕获）
  let oldStr = null, readErr = null;
  try { oldStr = entry.add(0x28).readPointer().readUtf8String(120); } catch (e) { readErr = e.toString(); }
  send({ t: 'patch.before', entry: entry.toString(), vtbl: entry.readPointer().toString(),
         lenCap: entry.add(0x20).readU64().toString(16), old: oldStr, readErr: readErr });
  const buf = Memory.allocUtf8String(NEW_PATH);
  entry.add(0x20).writeU32(NEW_PATH.length);
  entry.add(0x24).writeU32(NEW_PATH.length + 1);
  entry.add(0x28).writePointer(buf);
  let back = null;
  try { back = entry.add(0x28).readPointer().readUtf8String(120); } catch (e) {}
  send({ t: 'patch.done', new: NEW_PATH, back: back });
  done = true;
}

function armLookupHook(mgr) {
  if (lookupHooked) return;
  lookupHooked = true;
  try {
    const vtbl = mgr.readPointer();
    const fnLookup = vtbl.add(0x60).readPointer();
    send({ t: 'lookup.resolved', mgr: mgr.toString(), fnLookup: fnLookup.toString(), rva: fnLookup.sub(base).toString() });
    Interceptor.attach(fnLookup, {
      onEnter(args) {
        this.str = null;
        try { this.str = readEngStr(this.context.r8); } catch (e) {}
        this.out = this.context.rdx;
      },
      onLeave(ret) {
        send({ t: 'lookup.hit', str: this.str, tid: this.threadId });
        if (!done && this.str && this.str.indexOf('$$p_55a3.model.nezha') === 0) {
          let entry = null;
          try { entry = this.out.readPointer(); } catch (e) {}
          patchEntry(entry);
        }
      }
    });
  } catch (e) { send({ t: 'lookup.err', err: e.toString() }); }
}

function readEngStr(strObj) {
  try {
    const len = strObj.readU32();
    const ptr = strObj.add(8).readPointer();
    if (len === 0 || len > 512 || ptr.isNull()) return null;
    return ptr.readUtf8String(len);
  } catch (e) { return null; }
}

// 链解析：ModelActor::SetAsset 命中时 → [actor+0x28]→vtbl+0x60=fn60（取 manager 接口）
Interceptor.attach(base.add(0x17837d0), {
  onEnter(args) {
    if (lookupHooked) return;
    try {
      const actor = this.context.rcx;
      const comp = actor.add(0x28).readPointer();
      const fn60 = comp.readPointer().add(0x60).readPointer();
      Interceptor.attach(fn60, {
        onLeave(ret) {
          if (lookupHooked) return;
          try { armLookupHook(ret.readPointer()); } catch (e) { send({ t: 'chain.err', err: e.toString() }); }
        }
      });
      send({ t: 'chain.armed' });
    } catch (e) { send({ t: 'chain.err', err: e.toString() }); }
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
            except Exception:
                attached.add(pid)
        time.sleep(0.3)
    for s in sessions:
        try: s.detach()
        except Exception: pass
    print("done", flush=True)

if __name__ == "__main__":
    main()
