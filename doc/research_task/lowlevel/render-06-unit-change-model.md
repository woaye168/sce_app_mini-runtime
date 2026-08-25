# render-06 — unit_change_model 完整破解（本地模型直渲染）

> 2026-08-24 深夜攻坚。结论：**`game.unit_change_model(单位id, 'prefab相对路径')` 可以直接把项目内本地模型渲染到世界内**，无需数编注册。此前「所有形态均无效」的根因是**作用对象错了**（客户端 preview 单位负 id），不是参数形态错了。

## 一句话用法

```lua
-- id 必须是真实单位（正 id；玩家英雄 id 见下）。客户端 game.create_unit 造出的负 id 单位无效！
game.unit_change_model(1, 'characters/_user/jilulu_19ec/model.prefab')
game.unit_play_animation(1, 'Run_Battle_00')
-- 验证：
game.get_unit_model_path(1)  --> 'characters/_user/jilulu_19ec/model.prefab'
game.get_unit_asset(1)       --> '$$p_55a3.unit.主控.Model'（数编 link 不变）
```

## 实测矩阵（test_res002 PIE，api13 引擎 version-13/SCEEngine.dll）

| 对象 | id | change_model 效果 | get_unit_model_path |
| --- | --- | --- | --- |
| `game.create_unit('雷神')` preview 单位 | -20/-21（负） | **完全无效**（native 层 lookup 失败直接跳过，无报错） | nil（前后都 nil） |
| 玩家英雄（真实单位） | 1（正） | **生效**：库存 prefab / 本地吉鲁鲁 prefab / 坏路径全部写入并视觉切换 | 即时更新为新值 |

- 英雄原始 model_path = `characters/general/sk_basic2/model.prefab` → 证实参数形态就是 **prefab 相对路径**（不带扩展名的目录形态/纯名/数编 link 未再必要，路径形态即官方形态）。
- 坏路径（`nonexistent/xxx/model.prefab`）也静默写入，无报错日志（native 无校验日志串）。
- **截图证据（三态对比，决定性）**：
  - 原模型 sk_basic2 = 深色小人：`capture_1787512476.png` / `capture_1787512642.png`
  - 换本地吉鲁鲁 = 蓝紫小人：`capture_1787512892.png`（U16 单态驻留）；`capture_1787512115.png`（U11 误截到的"before"实为换模后——计时踩坑，见下）
  - 换库存大平面 = 角色消失、草地只剩红色标记框（平面贴地不可见）：`capture_1787512769.png`（U15）
- 玩家英雄正 id 来源：client 日志 `common/base/player.lua:422 sync player hero id: 1.0`（`base.local_player():get_hero()` 在本项目恒 nil，但 id 已同步；可直接用数值 id 调 game.* 原生函数）。

## attach_model 实测（U12，同轮）

- native 签名反汇编（impl RVA 0x12a20d0 → 核心调用 0x18176e8b0）：`unit_attach_model(id, path[, hand_point[, hold_point[, bool]]])`——arg2 path 必填字符串，arg3/arg4 挂点字符串可省，arg5 布尔可省；同样要求单位在注册表（正 id）。
- PIE 实测 `game.unit_attach_model(1, 'deco/engine/SM_Plane_A01/model.prefab')`：pcall ok=true 无报错，但**不传挂点时无可见效果**（截图 capture_1787512428 无平面）；detach 同样 ok=true。挂点名称规范待查（骨骼挂点名，如 hand_r 之类，需从模型 mdl 或官方用例取）。

## base.wait 并发注册语义坑（测试编排必读）

同一 tick 内连续写 `base.wait(5000,A); base.wait(10000,B); base.wait(15000,C)`，三个定时器**全部相对注册时刻**触发（t0+5/+10/+15），不是顺序串联延迟！要"状态驻留 N 秒再进下一态"必须嵌套回调（A 里再注册 B）或单态一局一跑。U13/U14 两轮截图窗口全错过就是踩的这个。

## native 定位链（version-13 sceengine.dll，50,869,176 B）

| 环节 | 位置 |
| --- | --- |
| 字符串 `unit_change_model` | VA 0x1826bcec0（file off 0x26bbcc0），无 lea xref |
| lua 注册表项（luaL_Reg {name,func}） | .rdata file off 0x26b85f0：name 指针 + func=RVA **0x12a7f90**；邻项 unit_detach_model/UnitDetachModel 共用 RVA 0x12a7fc0，unit_set_meta_human_part_value=0x12a81d0 |
| wrapper 0x12a7f90 | 取游戏上下文（magic edx=0xfff0b9d7）→ tail jmp 实现 0x12a23b0 |
| 实现 0x12a23b0 | arg1=id（int），0x1816fbc50(manager,&out,id) 按 id 找单位，**找不到直接 return 0（无任何日志）**；arg2=lua string → 自定义串{u32 len,u32 pad,char* ptr} → 调本地应用 0x1785350(unit,&str) → 组消息（键值：路径串+单位id）虚函数广播到其他端 |
| 本地应用 0x1785350 | `strcmp(unit->0x558, 新串)` 相同则跳过；不同则虚函数 `[组件+0xc8]` 换模 + 拷入 unit+0x550 → tail 刷新 0x178ab50 |

**防重复坑**：因为 native 层有 strcmp 短路，连续两次同路径调用第二次是 no-op；测试轮换时交替两个不同路径才每次都有动作。

## 方法论沉淀（本次新踩坑）

1. **frida 在 frida 17 python 的 API 变化**：`Module.getBaseAddress` 已删除 → 用 `Process.getModuleByName('SCEEngine.dll').base`；`session.enumerate_modules()` 也没有 → JS 侧 `Process.enumerateModules()`。
2. **模块名大小写**：磁盘是 `sceengine.dll`，进程内模块名是 **`SCEEngine.dll`**（getModuleByName 大小写敏感）。
3. **editor PIE 进程布局**：3 个 `sce` 进程（exe 无扩展名，D:\sce_online\version-13\SCE）：pid 早启动的（含 SCEModule.dll/SCECustomControl.dll，276 模块）= 编辑器壳；另两个（142 模块）= 游戏态进程。本机实测：24572=壳，34036/56864=游戏态。
4. **lua 注册函数定位法**（无 lea xref 时）：字符串 VA → 全二进制搜 8 字节小尾 VA → 命中 .rdata 的 luaL_Reg 表项 → +8 即函数指针。本套路对 game.*/ui.*/score.* 全部适用。
5. **先查 getter 再上 frida**：script-199 `common/base/unit.lua:105-111` 早有 `mt:get_asset()`/`mt:get_model_path()`（native `get_unit_asset`/`get_unit_model_path` 443059/443061），直接 lua 侧 getter 复读就能判定 native 是否生效，比 frida 内存 dump 省事得多。**本次 frida 钩子（0x1785350）零命中反而成为「负 id lookup 失败」的旁证**。
6. 探针一次性脚本：`test/temp/unit_model_hook.py`（frida hook 模板，含三进程识别）、`test/temp/enum_mods.py`（模块枚举）。

## 与 render-04 结论的关系修正

render-04 曾记「unit_change_model 所有形态均不改变外观——参数形态未明」。**修正**：参数形态一直是 prefab 相对路径（正确），失效根因是作用对象为客户端 preview 单位（负 id，不在 native 单位注册表）。数编注册链结论（entry_data.ini 是唯一源头）仍然成立，但**对「项目内本地模型渲染」需求而言数编不是必需的——真实单位 + change_model 即可**。

## 遗留

- `unit_attach_model` 挂点名称规范待查（不传挂点无可见效果）；`unit_detach_model` 同。
- 换模后动画名映射（AnimMapping）走新模型的 anim 集；`unit_play_animation` 用新模型自带动画名（吉鲁鲁 `Run_Battle_00` 实测可播）。
- 服务端侧 change_model 是否走同一 native（广播方向相反）未验证；双人/线上环境同步行为未验证。
- 坏路径的视觉表现 = 模型消失（U15 平面态佐证：无网格可渲时只剩选中标记）。
