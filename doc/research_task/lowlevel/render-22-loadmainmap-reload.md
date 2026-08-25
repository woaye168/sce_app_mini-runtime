# render-22 — xdeditor「强制重新加载项目」机制移植实验：模块级 LoadMainMap/reset 游戏侧判死

> 研究日期：2026-08-25 | 状态：✅ 机制全解 + 游戏侧运行时换图判死（三轮 frida 直调实证）
> 线索（用户 2026-08-25 20:26）：xdeditor「强制重新加载项目」与「打开」本质是运行时重载项目——编辑器本身就是个动态加载其他游戏资源的游戏，研究其机制或可突破
> 前置：render-19（模块级 load_map/LoadMainMap/reset 存在记录）、render-21（set_asset/注册表动态逆向）

## 0. 一句话结论

1. **xdeditor 重载链全解**：「文件/强制重新加载项目」= `EDITOR.unload_map()` + `EDITOR.load_map(map_path)`（menu_bar.lua:1378-1396）；「文件/打开」= `EDITOR.unload_map()` + `EDITOR.update_map_libs(path)` + `EDITOR.load_map(path, true, cb)`（menu_bar.lua:1144-1197）。这些 `EDITOR.*` 由 native 注入 **xdeditor lua 状态**（编辑器壳进程），**游戏 lua 状态（PIE 客户端）无此绑定**（G34a 实证：全局/game/base/base.game 四层 dump 全无 load_map/LoadMainMap/reset/SaveJson/EDITOR）。
2. **模块级 native 游戏侧判死（三轮 frida 直调实证）**：sceengine.dll 的模块级 `LoadMainMap/load_map`(0x181337a40)、`reset/Reset`(0x181337a70)、`SaveJson`(0x181337ac0) 在运行中 PIE 游戏进程里：同名=**设计内 no-op**（strcmp 门控 je 分支=清串即返 0，反汇编实证）；异名=整分支跑但**进程死亡**（内存爆冲 12GB→32GB 后崩溃，无任何 map/table 加载日志）；reset=**拆卸即 AV**。→ **游戏客户端会话无法承受换图**，重载必须编辑器侧完整编排（unload_map + host 协调）。
3. **frida 直调 native 通路打通**：ctx getter（thunk 0x181cbcb01）签名=`(lua_State* L, 0xfff0b9d7 magic) → 游戏上下文`（Win64 参数序 rcx/rdx 坑 + 壳进程 ctx 无效域校验）；lua54.dll 165 导出可用（lua_replace 是宏无导出→settop(0)+pushstring 造参）；三进程按 [ctx+0x10] 有效性鉴别真游戏进程。

## 1. xdeditor 重载链（源码实证，xdeditor-169 明文）

| 功能 | 位置 | 调用序列 |
| --- | --- | --- |
| 文件/强制重新加载项目 | `xdeditor/ui/menu_bar.lua:1378` | `EDITOR.unload_map()` → `EDITOR.load_map(map_path)` |
| 文件/打开 | `xdeditor/ui/menu_bar.lua:1144` | `MainFrame:OpenMap()` → `EDITOR.unload_map()` → `EDITOR.update_map_libs(path)` → `EDITOR.load_map(path, true, cb)` |
| 重载所有项目资源（美术台） | `art_workbench/.../menus.lua:886` | `reload_project_res` → project_res_manager |
| EDITOR 全局来源 | `xdeditor/global/global.lua:2` | `EDITOR = { test = true }` 桩表，native 运行时填充（编辑器壳 lua 状态） |

- 编辑器多进程架构旁证：`sub_process_enter_point/test.lua` 有 `process_manager:open_project(abs_path)`、`ProcessInfo.message_manager` 跨进程消息——**编辑器壳以子进程承载项目**（与 PIE 三进程布局吻合：壳 + 2 游戏态进程）。

## 2. 模块级 native（sceengine.dll api13）

luaL_Reg 表（.rdata file 0x26cd100，VA 0x1826ce300）：

| lua 名 | wrapper VA | impl / 语义 |
| --- | --- | --- |
| load_map / LoadMainMap | 0x181337a40 | tail jmp 0x181337240（真实现：luaL_checkstring(L,1) → strcmp(输入, [ctx+0xd0] 当前图绝对路径) → **相等=no-op 返 0**；不等=整图加载分支） |
| reset / Reset | 0x181337a70 | `[ctx+0xc0]→0x1809524d0` 拆卸 + 清 ctx+0xc8 串 + `[ctx+0xd8]→0x1806df840` |
| SaveJson | 0x181337ac0 | （未展开） |

- **当前图标识 = 绝对路径**（[ctx+0xd0] 引擎串；同名判定的依据）。
- 引擎串结构注意：部分 ctx 字段不是 {len,ptr} 直读形态（读出垃圾 len），疑为带 vtable 的 String 对象——读 ctx 字符串需按类 String 布局解析（vtable 在 +0）。

## 3. frida 直调三轮实证（探针 test/temp/loadmainmap_probe*.py）

| 轮 | 调用 | 结果 |
| --- | --- | --- |
| R1 | `LoadMainMap(test_res002 绝对路径)`（同名） | ret=0，无日志无效果 → no-op 分支（反汇编佐证：0x1813375de 起=清串+ret0） |
| R2 | `LoadMainMap(probe_map001)`（异名，项目克隆+注入条目） | **ret=0**（真游戏进程 pid 3292；两壳/残留进程 AV 0x10）但随后**内存爆冲至 32GB、编辑器整体死亡**，native 日志零 map/table 加载行，lua 状态未重启（G36 未复燃） |
| R3 | `reset(L)` → `LoadMainMap(probe_map001)` | **reset 即 AV**（0x7ffcf4000098，拆卸函数内部）——游戏态 ctx 不允许此拆卸 |

- 注入实验设施（可复用）：robocopy 克隆 test_res002→probe_map001（46MB 不含 .git/.bgd）→ `script/obj/effect/actor/data.lua` 追加 module_datas 块 + `dict.lua` 追加 dict 行（结构：init.lua 链 data→dict；dict 表收尾 `    }\n    return dict`）。因 R2 进程死亡未走到验证；**native 表真实数据源是否=script/obj 仍未证实**（下次换图成功时才能回答）。
- 探针坑沉淀：① Win64 参数序——frida NativeFunction 首参进 rcx，wrapper 的 magic 在 rdx（首 AV 根因）；② ctx getter 对**编辑器壳 lua 状态**也返回非空 ctx（但 [ctx+0x10] 无效）——按 [ctx+0x10] 非空鉴别真游戏进程；③ frida 捕获的 native AV 以 JS error 抛回不杀进程（reset/坏 ctx 的 AV 均存活）；④ getter 调用频次极高（每帧多次），`called` 守卫每进程独立（各进程脚本实例自有 JS 变量）。

## 4. 结论与通道重排

**「编辑器作为游戏动态加载其他游戏」的洞察成立，但重载发生在编辑器壳（xdeditor lua 态 + EDITOR.*），不在游戏客户端**；游戏客户端的模块级 LoadMainMap/reset 不是运行时重载通道（no-op/崩溃）。对「运行时免数编注入」目标：

1. **大厅/mini-runtime 通道（官方换游戏流程，线上可达）**——平台级「加载其他游戏」的正解；按坑 9 用 mini-runtime 组装大厅盒子流程 + dll 补丁触发（这是线上生产通道，优先级提升）。
2. **编辑器侧自动化**——bgd_mcp_bridge 在 xdeditor 态直调 EDITOR.load_map/unload_map（编辑器工作流自动化；非线上）。
3. **注册表外科路线（render-21 遗留）**——apply 链消费面逆向（完整条目改写）/克隆伪造条目/桶链注入（djb2+结构已备）。

## 5. 附带实证

- **UIWorld GetActorFactory 对 lua 缓存缺项不防御**：`world3:CreateActor(完全未注册 link)`（lua 缓存无此项）→ `defaultui/uiworldscript.lua:279: attempt to ind(ex)` lua 错误（pcall 可捕）——与「lua 有缓存但 native 无→返回 nil」（G33）是**两种不同的死亡形态**，生产代码需自行先查 `base.eff.cache(link)` 非空。
- G35 复证 `game.unit_change_model(1, prefab路径)` PIE 可用（英雄模型秒换吉鲁鲁，render-06 通道稳定）。
- MemoryStat 日志行可用作进程存活心跳（约每 30s 一条）。
