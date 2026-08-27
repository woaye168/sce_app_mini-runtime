# FINDINGS-common-base.md — common-base 组 draft_v2 研究新发现

> 2026-08-27，研究子代理。对 draft_v1 的修正、新确认的引擎行为、坑。产物：`draft_v2/common-base.md`（93 键全覆盖）、`status-common-base.json`、生成器 `gen_draft_v2_common_base.py`。**注：status/生成器等过程产物 2026-08-27 起统一在仓库 `test/loaded_modules_server/`。**

## 对 draft_v1 的修正

1. **`@common/preload/lni_loader` 与 `@common/preload/reload` 是转发桩**（`return require '@base.preload.xxx'`，注释「代码迁移到client_base 保留入口」），draft_v1 误判为「未提取到顶层函数定义」。dump 值树直接揭示 client_base 实现侧导出：lni_loader = `loader/packager/format/normalize/set_marco/initialize_computed` 6 函数；reload = `reload/reload_event/raw_include` 3 函数【dump 实锤】。
2. **`@common/base/margin` 源码与运行时矛盾**：包内 margin.lua 本体是客户端空函数（注释「逻辑全在服务端，客户端只需要空函数」），但服务端 dump 值 = `RegionMargin` 类表（extends Region，方法 init_margin_region/get_width/get_height 等 7 个）——**服务端运行时加载的是另一实现**，本包源码不代表服务端行为【dump 实锤】。
3. **`@common/base/ad` 源码与运行时矛盾**：源码末尾 `return { show_reward_video_ad = ... }`（ad.lua:60-62），dump 值却为 `true`（运行时 package.loaded 未保留返回表，疑平台守卫早退或返回值被丢弃）【语义推测，待实测】。
4. **`@common/base/item` 的 include 之谜**：init.lua:121 以 `include` 加载（KB 称 include 不走 package.loaded 缓存），但 dump 中本键是完整 Item 类表——include 实现可能仍写 package.loaded，或被其他模块以 require 再加载【反查推测】。
5. **draft_v1 六个桩（class/co/deque/event_deque/exception/try）的「实现不可考」被 dump 推翻**：桩模块的 dump 值就是转发目标的运行时导出——co = `async/wrap/sleep/sleep_one_frame/async_next/will_async/call/thread_to_tsCo/tsCo_to_thread` + Coroutine 类；exception = `throw/to_exception` + Exception 旧式类（16 键）；try = `try/try_wrap` + `FINALLY_RETURN` 哨兵表；deque/event_deque 各 2 个 create_* 函数。仅 class 值为 true。

## 新确认的引擎行为

6. **19 个无源码 dump-table 模块全部是 TSTL 类模块**（DamageInstance/HealInstance/Channeler/Channeled/Slot/Inventory/MoverLine/MoverTarget/MatchInfo/DateTime/AI/AISearcher + tds_score 的 Score 族 41 类），api-13 全语料 grep 无任何 Lua 源码 → **引擎内嵌 TS 编译产物，不随 script 包分发**（与 lualib_bundle 的 CLASSES 注册表同一份引用，also_in 互证）。
7. **lualib_bundle 的 dump 值含 `CLASSES` 全库 TS 类注册表（141 类）**：各 `@common/base/<模块>` 键的类值与 `CLASSES.<类名>` 是同一表引用——模块键是「按模块归属」视图，CLASSES 是「按名索引」视图。
8. **截断集中于 TSTL `_descriptors` 属性访问器**（get/set/enumerable/configurable）：tds_score 109 处、lualib_bundle 244 处、co/item/skill/ai_searcher 各 3~9 处，全部不损失真实 API 信息。例外：trigger 197 处截断在 `Trigger.prototype.event.evt_args.*`（事件参数构造器表），由源码侧 `args.event_*` 清单完整补位。
9. **服务端 Unit 方法面远大于 client 包源码**：dump Unit.prototype 386 键/376 方法，源码 unit.lua 仅提取 ~150 个；多出部分为 Target 继承（is_ally/is_enemy/is_neutral_to 等）与服务端扩展（kill/ride_on/blink/create_unit/ai_damage 等）【dump 实锤】。
10. **utility 运行时返回 = `{ Mover, Region, Target }`**（client_base utility 的 TS 基类导出）；`Target` 是 ScenePoint/Unit/Snapshot 共同基类（`_G.Target = utility.Target`）【dump 实锤 + 源码实锤】。
11. **`@common/base/json_decode` 无源码但 dump 实锤为 JSON 编解码模块**：`encode/decode/null` + `EMPTY_OBJECT/EMPTY_ARRAY` 空表哨兵。
12. **tds_score 结构定稿**：顶层 36 键 = 35 个 `Score*Param`/`Score*Data` TS 参数/数据类 + `ScoreCommitter` 提交器（16 方法：commit/set/add/money_set/money_add/money_cost/money_add_ex/rank_add/rank_set/list_add/list_modify/list_delete/withlimit_add/name_new/clear/add_finish_callback）。与 lua_plus 包装器对照发现**命名不一致**：包装器调 `c.addi{...}`，dump 方法名是 `add`（疑版本漂移或动态别名）。读取侧在 `_G.score`（28 函数），不在本模块。
13. 官方源码残留调试垃圾：`base/init.lua:156` `base.game.fff = function() error "22222" end`【源码实锤】。

## 统计

- 93/93 键全覆盖。主置信级分布：源码实锤 53 / dump 实锤 19 / 反查推测 4（auxiliary/detection/room/validator）/ 语义推测 17（admin/crop/effect/fish/force_movement/gameplay/isolation/lni/load_done/loot/loot_pool/old_junk/scene_object/selector/shop/table_attr/turn）。
- 转发桩 8 个（base 侧 6 + preload 侧 2）。
- 含截断 8 键：co(3)、item(3)、skill(4)、ai_searcher(9)、response(14)、tds_score(109)、trigger(197)、lualib_bundle(244)。
