# render-21 — ★★ set_asset/注册表动态逆向全解 + typeid=djb2(link) 破解 + 条目运行时改写首轮（纯路径改写判死）

> 研究日期：2026-08-25 | 状态：✅ 机制全解（汇编+动态双重实证）；Tier-1 纯路径改写视觉判死
> 对象：sceengine.dll api13（PIE 三进程均带 SCEEngine.dll，基址 0x7ffce24d0000 本次会话）
> 工具：test/temp/registry_probe.py / registry_probe2.py / registry_probe3.py（链式解析探针，一次性）、test/temp/addr_test.py
> 前置：render-18（静态逆向）render-20（虚拟数编终审）

## 0. 一句话结论

1. **set_asset 全链闭合（汇编+frida 双证）**：lua wrapper 零解析 → `actor->vtable[+0xa0]` 分派 → **ModelActor::SetAsset=RVA 0x17837d0**（manager `vfunc+0x60` MODEL 表查找）、**EffectActor::SetAsset=RVA 0x179c0a0**（manager `vfunc+0x70` EFFECT 表查找）——查找 miss 即 `je` 静默返回（G28/G30 死亡点汇编级定位）；**两侧均无路径回退分支**。
2. **native 注册表容器与哈希全解**：manager（GameDataManagerImp）vtbl 暴露**按类型分表的查找族**（+0x50/+0x58/+0x60/+0x70…）；ACTOR 表 = `mgr+0x230` 桶链，node `{+0:next,+0x18:typeid,+0x20:entry内联}`，**typeid = djb2-32(link) 经捕获值精确验证**（`'$$p_55a3.actor.bgd_demo_effect.root'` → 0x8df9751e 命中）——**任意 link 的注册键可离线预算，为伪造/注入条目扫清最后障碍**。
3. **Tier-1 条目改写实证与判死**：在 MODEL 表查找返回前把 nezha 条目 Asset 指针（entry+0x28）改写成吉鲁鲁 prefab 路径（内存级成功），但 set_asset 应用后**目标位置无模型渲染**——apply 链消费的不止路径（预载 mesh/archetype 句柄未同步）→ **纯路径改写 ≠ 换资产**；下一轮 hook apply 链定位真实消费字段。
4. **渲染管线渲染侧确认**：G17 UIWorld + G28/G30 同屏再验证（吉鲁鲁+横幅特效正常）。

## 1. frida 17 新坑（工具纪律）

| 坑 | 现象 | 对策 |
| --- | --- | --- |
| 3 字节 `call [rax+0x50]`（FF 50 50） | `unable to intercept function at ...`（0x1816fa2f5） | 读点前移 4 字节到上一条指令起点（0x1816fa2f1，rax 已是 vtable）；6 字节 `call [rax+0xa0]`（FF 90 A0…，0x181345aaf）可直接钩 |
| `readUtf8String(N)` 带长度 | 读满 N 字节遇 NUL 抛 `can't decode byte 0x00 in position X`——v3.2 扫描全 miss 的真凶 | 未知长度 C 串用 `readCString()` 或不带参 `readUtf8String()`；带长度只用于引擎 String{len,ptr} 精确读 |
| 后台探针残留 | python 进程存活致旧钩子刷屏/冲突 | 每轮先 `Get-Process python` 清点；探针 DUR 不宜过长 |

## 2. set_asset 分派链（终版）

```
lua: actor:set_asset(x)
  └─ wrapper 0x181345a10：checkudata(GameActor) + luaL_checkstring，零校验
       └─ call [actor.vtbl+0xa0]（0x181345aaf）——按类分派：
            ModelActor  → 0x1817837d0：[actor+0x28]→vfunc+0x60=manager；manager vfunc+0x60(串) 查 MODEL 表
                            mov rbx,[out]; test rbx,rbx; je ret   ← G28 裸路径死亡点（静默）
                            hit: entry 存 actor+0x1e8 → apply 0x18177d5a0/0x181769c30/0x181783e30
            EffectActor → 0x18179c0a0：同形，manager vfunc+0x70(串) 查 EFFECT 表
                            mov rcx,[out]; test rcx,rcx; je ret   ← G30 裸路径死亡点（静默）
                            hit: entry 存 actor+0x178 → apply 0x18179c1a0…
            GameUnit    → 未捕（同 wrapper，vtable+0xa0 自有一覆写）
```

- render-18 §3 候选 **0x18129fb50 排除**（非 EffectActor::SetAsset；其「哈希桶链+路径回退加载」形态符合 UI particle 控件 effect setter——与 render-02 特效直路径实证一致，G30 矛盾闭环）。
- 引擎串结构再证：`{u32 len, u32 cap, char* ptr}`（nezha link len=25、asset len=64 实测吻合）。

## 3. 注册表容器与哈希（终版）

### 3.1 manager vtbl 查找族（GameDataManagerImp，实例本次=0x3cc7c87cb00，vtbl=0x7ffce4c059e0）

| vfunc | 地址（RVA） | 用途 | 容器 |
| --- | --- | --- | --- |
| +0x50 | 0x18202d0 | link 字符串查 ACTOR 表（CreateActor 路） | 同 +0x58 |
| +0x58 | 0x18202f0 | **typeid(u32) 查 ACTOR 表** | `mgr+0x230` 桶链 |
| +0x60 | 0x181e1b0 | 字符串查 MODEL 表（SetAsset 路） | `mgr+0x308` 链式 strcmp 遍历 |
| +0x70 | （未解析） | 字符串查 EFFECT 表（EffectActor::SetAsset 路） | 待查 |

### 3.2 ACTOR 表桶链（fn58 反汇编全解）

```
buckets = [mgr+0x230] {+0x4: bucketCount, +0x10: bucket[]}
bucket  = typeid & (bucketCount-1)
node    = {+0x0: next, +0x18: typeid(u32), +0x20: entry 内联}
sentinel= [mgr+0x228]（链尾判停）
```

- **typeid = djb2-32(link 全串)**（h=5381, h=h*0x21+c）：捕获 0x8df9751e ↔ `'$$p_55a3.actor.bgd_demo_effect.root'` 精确命中。预算：nezha model link=0x9c14972f（注：MODEL 表不走此键，见 §4）、jilulu actor=0xe42efc36。
- ACTOR 条目布局（ActorEffect 实例）：`+0x0 vtbl(0x7ffce4c05810) / +0x8 共享对象 / +0x10 typeid / +0x18 {len,cap} / +0x20 link ptr / +0x138 疑似 u64 哈希 / +0x140 {len,cap} / +0x148 Inherit 父 link ptr`。
- manager 本体 +0x90/+0x98 = 地图路径串（`c:/users/.../test_res002`）；+0xa8 起为另一按小整数索引的容器（疑似单位 id 表）。

### 3.3 MODEL 条目布局（nezha 实例 dump）

```
+0x00 vtbl 0x7ffce4c057c0
+0x10 {len=25, cap=26}  +0x18 → '$$p_55a3.model.nezha.root'
+0x20 {len=64, cap=65}  +0x28 → 'characters/_user/p_55a3_nazha_wuwuqi_xin1_85sc_w72l/model.prefab'  ← Asset 字段
+0x30 float 1.0（Scale）
+0x38 {len=4,cap=8} +0x40 → 短串（疑 anim 名）
+0x58/+0x60/+0x68/+0x70 子对象指针（疑预载 mesh/archetype 句柄）
+0xa8.. UTF-16 内联串 "…fault.png"（default.png）
+0xc8..+0x130 内联 ASCII 槽位名串（'[P[show_list_main_panel][P[show_list_main_inner][P[list_info_15][P[list_info_left_icon][P[fo…'）
```

## 4. Tier-1 条目运行时改写（首轮实证与判死）

**手法**（registry_probe3.py v3.3）：钩 ModelActor::SetAsset 入口 → 挂 vfunc+0x60（manager 接口）返回钩 → 解析 manager → 钩 manager `vfunc+0x60`（MODEL 查找 0x181e1b0）→ nezha 查找返回前**就地改写 entry+0x20/+0x28** 为 `characters/_user/p_55a3_jilulu_19ec_a8oz/model.prefab`。

**结果**：内存改写成功（lenCap 0x4100000040→0x3e…，回读验证）；但 G28 的 `set_asset('$$p_55a3.model.nezha.root')` 应用后，(-150,0,0) 处**既非哪吒也非吉鲁鲁——无模型渲染**（截图 capture_1787657984；对照 G16 原点吉鲁鲁、G30 横幅特效均正常）。

**结论**：apply 链（0x18177d5a0/0x181769c30/0x181783e30）消费的不止路径——条目内预载 mesh/archetype 句柄（+0x58 系子对象）与路径不一致 → 应用失败/空渲。**纯路径改写判死**。

**下轮候选（按优先级）**：
1. hook apply 链三函数，看它们实际读 entry 哪些字段（确定换资产的真实消费面）；
2. 改写路径**同时置空预载句柄**逼懒加载（若 apply 有句柄非空短路）；
3. 条目光整体克隆：钩 vfunc+0x60 直接返回**伪造条目指针**（克隆 jilulu MODEL 条目内存 + 改 link/asset/typeid 键），绕过「原地改」一致性难题；
4. Tier-2 注入：djb2 可预算 + 桶链结构已解 → 克隆 ACTOR 节点挂入桶链 + lua merge_cache 双侧同步，造全新注册 link。

## 5. 当前攻防全图

```
lua 层虚拟数编（merge_cache）        → 仅 lua 消费者有效（render-20 三入口终审）
load_map / set_map_dir               → 永不触发表加载（render-20 §2）
set_asset 裸路径                     → 查找 miss 静默返回（本文 §2 汇编双证）
创建假 link                          → native 查表 miss=nil（render-17）
条目纯路径改写（Tier-1）             → 不一致条目空渲判死（本文 §4）
─────────────────────────────────────────────
剩余活路：
  ① apply 链消费面逆向 → 完整条目改写（本文 §4-1/2）
  ② 查找钩返回伪造条目（克隆+改字段）（本文 §4-3）
  ③ 桶链注入全新注册条目（djb2+桶链已解）（本文 §4-4）
  ④ 已有免数编通道生产化：unit_change_model（主世界真实单位）/UI particle 特效直路径/spine/材质 ResourceCache/renderpath xml
```

## 6. 遗留

- GameUnit 的 vtable+0xa0 覆写地址未捕（G2 关闭状态无触发）。
- EFFECT 表查找（vfunc+0x70）地址与容器未解析（同法可解）。
- MODEL 表 `mgr+0x308` 链节点精确布局（strcmp 遍历，注入点与 ACTOR 桶链不同需另解）。
- apply 链三函数消费字段清单（下轮头号目标）。
