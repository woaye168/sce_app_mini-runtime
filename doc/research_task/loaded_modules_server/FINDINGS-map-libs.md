# FINDINGS-map-libs.md — map-libs 组 draft_v2 新发现

> 2026-08-27，map-libs 分组（42 键）dump 值树核验过程发现。

## M1. 地图库入口的运行时返回模式（解释 dump 单键表形态）

各地图库 `ui/script/main.lua` 尾部统一为【源码实锤】：

```lua
local ret = {["<包名>"] = <包名>全局表}
for k, v in pairs(____module or {}) do ret["<包名>"][k] = v end
for k, v in pairs(____return or {}) do ret[k] = v end
return ret
```

服务端 dump 中 `@defaultui/main`=`{defaultui={...}}`、`@lib_common_ai/main`=`{lib_common_ai={...}}`、`@lib_control/main`=`{lib_control={}}` 均为此模式的产物——**单顶层键 = 包命名空间**，不是模块自身 API 少。

## M2. lib_common_ai 运行时证据链闭合（`_G.poi` ≡ class/* 模块）

- `_G.poi.ai`（9 函数）与 `@lib_common_ai/class/new` dump 值**逐函数相同**（new/remove/reset/on_tick/change_state/get_path/find_enemy/attack_skill/provoke）。
- `_G.poi.ai_enmity` ≡ `class/behavior/enmity`（start/reset/remove_enmity）。
- `_G.poi.ai_state.{back,move,attack,none,pursue}` 对应 `class/state/*`，但字段名不同：poi 侧为 `{on_enter, on_leave, priority(标量)}`，模块侧导出 `on_enter_<state>/on_leave_<state>`——说明存在装配层（疑为 `class/init`，dump=true 无导出）把模块函数重命名挂进状态表并附优先级。
- 结论：AI 类设施的运行时访问点是全局 `poi.*`；`@lib_common_ai/main` 值中的 `lib_common_ai.AI` 类（33 方法）是另一套（新版 OO）AI 基类，与 poi 的 class 系并存。

## M3. `@global_default/lua_declare` 归属修正（draft_v1 高估）

- draft_v1 从源码提取的 730 函数/60+ 事件类构造，**dump 值树证实大部分不是本模块自有**：Unit(386 键)/Skill(97)/Item(73)/EffectParam(74)/Timer(15)/ScreenPos(8) 与 `@common/base/{unit,skill,item,eff_param,timer,position}` 及 `_G` 同内容哈希（`test/loaded_modules_server/parsed/shared_tables.json` 实锤，20+ 条共享记录跨 lua_declare 与 common 键）。
- 且 module_source_map 把键映射到 `ui/script/lua_declare.lua`（客户端声明桩，exports={TriggerEvent,Target,Unit,base}），而服务端 dump 值顶层 12 键（含 UnitPropertyBonus/Cast/middle_game_key/UserID/UnitTable）与该文件 exports 不一致——**服务端 lua_declare 是另一份未分发文件**。draft_v2 已按 dump 值重写。
- 截断 7 处集中在 `_descriptors.*` 属性描述符（target_type/inv_index 的 get/enumerable/configurable）。

## M4. `@defaultui/main` 的 NewActor 类为运行时注入

- dump 值含完整 TSTL 类 `defaultui.NewActor`（33 方法 + id_count/actor_map）与工厂函数 `create_NewActor_at`，但语料（D:\sce_open\api-13\2026_08_27 全量）grep `NewActor` **0 命中**——编辑器侧任何包都不含该类源码。最可能由 `@defaultui/actor`（dump=true，无导出）在运行时注册。lua_plus 的 `base.actor_*` 扁平 API 是其下层引擎面。

## M5. ui/script 来源键在服务端 dump 中几乎全为 true

minimap_camera_control / move_joystick / trigger_module_main_1 / trigger_validator / require_libs 等 12 个有源码键中，10 个源码在 `ui/script/`（客户端），其服务端 dump 值均为 `true`。唯二 dump 有表值的有源码键是 `@defaultui/main` 与 `@lib_control/main`（`src/` 服务端入口）。**读地图库"服务端 API"时 ui/script 源码签名只代表客户端行为**。

## M6. lib_game_options 是官方库中服务端云数据用法的完整范例

`src/{gift_code,rename,user_info}.lua` 三个模块实锤了一整套服务端模式，可供云变量服务端研究直接引用：

- Redis 消息通道：`base.s.subscribe_message('Redis.Server2Host.Channel.ExchangeCode.<game>_<userId>', {ok,error,timeout})` / `base.s.publish_message('Redis.Host2Server.Channel.ExchangeCode', {...})`——host↔server 双向通道命名规范。
- 云数据：`base.s.score_init(db, player, {ok(score,iscore,sscore),error,timeout}, key...)`、`base.s.name_exist(db,'nick',name,cbs)`、`base.s.get_commit(备注,db)` → `c.score_sets/score_seti/name_delete/name_new` → `c.commit(备注,db,cbs)`。
- 协议：`base.ui.proto.<C2S名> = function(player, data)`，回包 `player:ui '<S2C名>' {...}`；超时兜底 `base.wait(3000, ...)`。
- 其他：`base.detection.check_text(name, cb)`（易盾敏感词，suggestion 0/1/3 三分支）、`base.backend.init_game_config()`（main 入口必调）、`base.auxiliary.get_player_id(player)`。

## M7. lib_control 服务端实有内容：施法失败码表

`@lib_control/main` 的 `src/main.lua` 含 `fail_code_enum`（施法失败码 0~26 全量中文文案）与 `技能-施法失败` 事件 handler（码≠12 且 `cast.is_user` 时 `unit:error_info(文案)`）——这是失败码语义的权威来源（此前仅见触编侧零星引用）。
