# PROGRESS.md — 216 键核验进度看板

> 由 `test/loaded_modules_server/gen_progress.py` / `merge_status.py` 生成。状态机：未开始 → 值树已解析 → 已核验 → 已定稿。
> 当前：全部 216 键 draft_v2 已定稿（值树+源码/反查双证）；实测批次 1~4 完成，【实测】结论已回写 draft_v2（backend/room/tds_score/协程模型）。

## 分组进度总览

| 分组 | 键数 | 有源码 | 无源码 | 已定稿 | 文档 |
| --- | --- | --- | --- | --- | --- |
| common-base | 93 | 60 | 33 | 93 | draft_v2/common-base.md |
| common-base-game | 15 | 0 | 15 | 15 | draft_v2/common-base-game.md |
| lua-plus | 47 | 42 | 5 | 47 | draft_v2/lua-plus.md |
| map-libs | 42 | 19 | 23 | 42 | draft_v2/map-libs.md |
| smallcard-libs | 19 | 13 | 6 | 19 | draft_v2/smallcard-libs.md |
| stdlib(范围外附录) | 11 | - | - | - | 00-INDEX 附录 |

## common-base（93）

| 键 | 值形态 | 函数数 | 截断 | 有源码 | 置信 | 状态 |
| --- | --- | --- | --- | --- | --- | --- |
| `@common` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/actor` | table | 44 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/ad` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/admin` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/ai` | table | 0 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/ai_searcher` | table | 3 | ⚠️9 | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/anim_handlers` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/array` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/auxiliary` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/behavior` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/buff` | table | 52 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/channeler` | table | 11 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/cheat` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/circle` | table | 7 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/class` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/cmd_result` | table | 6 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/co` | table | 12 | ⚠️3 | ✅ | dump实锤 | 已定稿 |
| `@common/base/collision_flags` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/crop` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/damage` | table | 14 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/datetime` | table | 1 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/deque` | table | 2 |  | ✅ | dump实锤 | 已定稿 |
| `@common/base/detection` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/eff` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/eff_param` | table | 77 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/effect` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/event` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/event_deque` | table | 2 |  | ✅ | dump实锤 | 已定稿 |
| `@common/base/exception` | table | 12 |  | ✅ | dump实锤 | 已定稿 |
| `@common/base/fish` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/force` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/force_movement` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/friend` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/game` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/gameplay` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/group` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/hashtable` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/heal` | table | 6 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/inventory` | table | 18 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/isolation` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/item` | table | 69 | ⚠️3 | ✅ | 源码实锤 | 已定稿 |
| `@common/base/json_decode` | table | 3 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/line` | table | 3 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/lni` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/lni_writer` | function | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/load_done` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/log` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/loot` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/loot_pool` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/lualib_bundle` | table | 1425 | ⚠️244 | ✅ | 源码实锤 | 已定稿 |
| `@common/base/margin` | table | 7 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/match_info` | table | 1 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/math` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/mover_line` | table | 1 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/mover_target` | table | 1 |  | ⚠️ | dump实锤 | 已定稿 |
| `@common/base/obj_check` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/old_junk` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/player` | table | 98 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/point` | table | 50 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/position` | table | 3 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/promise` | table | 3 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/quest` | table | 66 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/rect` | table | 8 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/response` | table | 26 | ⚠️14 | ✅ | 源码实锤 | 已定稿 |
| `@common/base/room` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/rpc` | table | 2 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/scene_object` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/scene_point` | table | 43 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/selector` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/shop` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/skill` | table | 192 | ⚠️4 | ✅ | 源码实锤 | 已定稿 |
| `@common/base/snapshot` | table | 13 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/state_machine` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/table` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/table_attr` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/target_filter` | table | 9 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/tds_score` | table | 116 | ⚠️109 | ⚠️ | dump 实锤 | 已定稿（关联键） |
| `@common/base/team` | table | 4 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/thirdordermatrix` | table | 5 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/timer` | table | 12 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/trigger` | table | 17 | ⚠️197 | ✅ | 源码实锤 | 已定稿 |
| `@common/base/trigger_editor_v2` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/trigger_editor_v2/array` | table | 1 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/try` | table | 2 |  | ✅ | dump实锤 | 已定稿 |
| `@common/base/turn` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/unit` | table | 377 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/utility` | table | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/validator` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/vector` | table | 5 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/base/voice` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@common/preload/lni_loader` | table | 6 |  | ✅ | dump实锤 | 已定稿 |
| `@common/preload/reload` | table | 3 |  | ✅ | dump实锤 | 已定稿 |

## common-base-game（15）

| 键 | 值形态 | 函数数 | 截断 | 有源码 | 置信 | 状态 |
| --- | --- | --- | --- | --- | --- | --- |
| `@common/base/game/error_info` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/game` | true | 0 |  | ⚠️ | dump 实锤（面）/反查推测（归属） | 已定稿 |
| `@common/base/game/game_message` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/game_result` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/game/item` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/game/learn_skill` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/level_up` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/player` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@common/base/game/reborn` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/scene` | table | 7 |  | ⚠️ | dump 实锤+源码实锤（5/7） | 已定稿 |
| `@common/base/game/select_hero` | true | 0 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@common/base/game/shop` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/template_buff` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/ui` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@common/base/game/unit` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |

## lua-plus（47）

| 键 | 值形态 | 函数数 | 截断 | 有源码 | 置信 | 状态 |
| --- | --- | --- | --- | --- | --- | --- |
| `@lua_plus/base` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/actor` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/advertise` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/ai` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/ai_attack` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/area` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/attack` | true | 0 |  | ✅ | 源码实锤（块注释）+dump 实锤（反面） | 已定稿 |
| `@lua_plus/base/base_lua_plus/buff` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/camera` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/capturer` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/cheat` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/common` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/damage` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/eff_param` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/game` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/gamechat` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/global_variable` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/hook` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/item` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/lightning` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/localization` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/loot_pool` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/minimap` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/mover` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/player` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/point` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/quest` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/simple_ui` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/skill` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/snapshot` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/sound` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/tds_score` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/test` | true | 0 |  | ✅ | 源码实锤+dump 实锤（反面） | 已定稿 |
| `@lua_plus/base/base_lua_plus/timer` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/timershow` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/trigger` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/unit` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/单位组` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/单位组_玩家组api` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/单位过滤器` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/base_lua_plus/附着点` | true | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@lua_plus/base/obj_check` | true | 0 |  | ✅ | 源码实锤+dump 实锤（反面） | 已定稿 |
| `@tds_score` | true | 0 |  | ⚠️ | dump 实锤（true）+语义推测 | 已定稿 |
| `@tds_score/new_base_score` | true | 0 |  | ⚠️ | dump 实锤（true）+语义推测 | 已定稿 |
| `@tds_score/score` | true | 0 |  | ⚠️ | dump 实锤（true）+语义推测 | 已定稿 |
| `@tds_score/tds_score` | true | 0 |  | ⚠️ | dump 实锤（true）+语义推测 | 已定稿 |
| `@tds_score/use_mysql` | true | 0 |  | ⚠️ | dump 实锤（true）+语义推测 | 已定稿 |

## map-libs（42）

| 键 | 值形态 | 函数数 | 截断 | 有源码 | 置信 | 状态 |
| --- | --- | --- | --- | --- | --- | --- |
| `@defaultui/actor` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@defaultui/default_ui` | true | 0 |  | ⚠️ | dump 实锤+源码实锤 | 已定稿 |
| `@defaultui/default_ui/minimap_camera_control` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@defaultui/default_ui/move_joystick` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@defaultui/main` | table | 35 |  | ✅ | dump 实锤 | 已定稿 |
| `@defaultui/require_libs` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@defaultui/trigger_module_main_1` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@defaultui/trigger_validator` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@global_default/lua_declare` | table | 730 | ⚠️7 | ✅ | dump 实锤 | 已定稿 |
| `@lib_common_ai/ai/ai_common` | table | 3 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/ai/default_ai` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@lib_common_ai/ai_templates/default_ai` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/主动召唤物` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/仅移动ai` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/召唤物` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/攻城车` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/自定义ai` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/通用ai` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/野怪` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/ai_templates/镖车` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@lib_common_ai/class/behavior/enmity` | table | 3 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/class/init` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@lib_common_ai/class/new` | table | 9 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/class/state/attack` | table | 3 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/class/state/back` | table | 2 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/class/state/move` | table | 2 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/class/state/none` | table | 2 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/class/state/pursue` | table | 2 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@lib_common_ai/customscript` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@lib_common_ai/main` | table | 42 | ⚠️5 | ✅ | dump 实锤 | 已定稿 |
| `@lib_common_ai/trigger_module_main_1` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_common_ai/trigger_validator` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_common_ai/utility` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@lib_common_sounds/main` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_control/main` | table | 0 |  | ✅ | dump 实锤+源码实锤 | 已定稿 |
| `@lib_control/require_libs` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_control/trigger_module_main_1` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_control/trigger_validator` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_game_options/gift_code` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_game_options/main` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_game_options/rename` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@lib_game_options/user_info` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |

## smallcard-libs（19）

| 键 | 值形态 | 函数数 | 截断 | 有源码 | 置信 | 状态 |
| --- | --- | --- | --- | --- | --- | --- |
| `@smallcard_get_items/main` | table | 8 | ⚠️3 | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@smallcard_get_items/module` | table | 20 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@smallcard_get_items/require_libs` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_get_items/trigger_module_main_1` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_get_items/trigger_validator` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_inventory/main` | table | 4 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@smallcard_inventory/proto` | true | 0 |  | ⚠️ | 反查推测 | 已定稿 |
| `@smallcard_inventory/proto_v2` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@smallcard_inventory/require_libs` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_inventory/score_save` | true | 0 |  | ⚠️ | 语义推测 | 已定稿 |
| `@smallcard_inventory/trigger_module_main_1` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_inventory/trigger_validator` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_inventory/关闭玩家背包` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_inventory/打开玩家背包` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_mail/mail` | table | 1 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@smallcard_mail/main` | table | 0 |  | ✅ | 源码实锤+dump 实锤 | 已定稿 |
| `@smallcard_mail/module` | table | 9 |  | ⚠️ | dump 实锤 | 已定稿 |
| `@smallcard_mail/trigger_module_main_1` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
| `@smallcard_mail/trigger_validator` | true | 0 |  | ✅ | 源码实锤 | 已定稿 |
