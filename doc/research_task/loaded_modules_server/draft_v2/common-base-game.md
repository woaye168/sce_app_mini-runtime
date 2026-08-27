# common-base-game 组：`@common/base/game/*`（引擎内嵌 game 子层）

模块数：15。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）+ `_G` 值树对照。

源码覆盖：**0/15 直接命中**；但 script 包 v199 存在 8 个同名文件可作部分对照（见各节与组级说明）。

## 组级结论（draft_v2 新增，相对 draft_v1 的重大修正）

1. **值形态**：15 键中 14 键值为 `true`（已加载、无表导出，属"扩展 base.game / base.* 全局"的副作用模块）；唯一例外是 `@common/base/game/scene`，值是 **7 函数的表**（见该节）。
2. **引擎内嵌接管 script 同名文件（新发现，置信【反查推测】→ 部分【dump 实锤】）**：
   - script 包 v199 的 `common/base/` 下存在同名文件 `error_info.lua / game.lua / game_result.lua / item.lua / player.lua / scene.lua / select_hero.lua / unit.lua`，且 `common/base/init.lua` 对 `select_hero`（L122 `include "base.select_hero"`）与 `game_result`（L164 StateGame 分支 `require 'base.game_result'`）有显式加载。
   - 但 dump 中**不存在** `@common/base/error_info`、`@common/base/game_result`、`@common/base/scene`、`@common/base/select_hero` 键——只存在 `game/*` 变体。
   - 实锤案例：script 版 `select_hero.lua` 定义 `base.select_hero` = `hero_list/select_hero/click_hero/click_random_hero/show_timer/show_hero/show_random`（7 函数，客户端向，内部调引擎 `game.request_pick_hero`）；而 dump 的 `_G.base.select_hero` = `on_click/on_init/on_random/on_select/op_click/op_init/op_select/op_stop`（8 函数，服务端向），**两套完全不同**——服务端 StateGame 上引擎内嵌的 `game/select_hero` 整体覆盖了 script 版。**draft_v1 中 select_hero 节的反查清单（hero_list 等）实为 script 客户端变体的源码签名，服务端不存在**，已在本版修正。
   - 推论：`@common/base/game/*` 是引擎内嵌 common 包（非 script 包下发）的 `base/game/` 目录模块，服务端加载时接管/替代 script 包同名文件；script 版仅在客户端语义下有效。要拿到这 15 个模块的真实源码，需走引擎内嵌包提取路线（见 sce_app_editor-patch `doc/research/pak-extract-guide.md`）——**待办**。
3. **运行时证据主来源**：`_G.base.game` 子树（130 键：124 函数 + 6 子表）与 `_G.base.select_hero`（8 函数）、`base.game.select_hero` 子表（4 函数）。由于 14 个键无表导出，无法把 base.game 的 124 个函数逐一归属到具体子模块，故函数全集在组级列出一次（见下节），各模块节只做语义关联标注。
4. **draft_v1 组级反查清单的口径修正**：draft_v1 头部列出的 `base.game.debug_draw_* / circle_selector / camera_focus / chat / lock_camera / ensure_one_lobby / one_more_round / load_combined_map / get_model_anim_point_info / is_camera_locked / set_dynamic_point_light / pathing_way_points` 等形态**不在服务端 dump 的 base.game 124 函数内**——它们来自客户端语料（或客户端变体模块），服务端 StateGame 不可用。本文档函数表以 dump 为准（服务端权威面）。

## 组级运行时面：`_G.base.game`（130 键，【dump 实锤】）

字段/子表：

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.game._scene_copy` | table（0 键） | 场景副本注册表，dump 时为空 | 【dump 实锤】 |
| `base.game._scene_object_map` | table（1 键：`default`） | 场景对象映射；`default` 子树 6 键但全部 `<max depth exceeded>` 截断（areas/__index/default_name/key/scene_actors/triggers），**字段不全** | 【dump 实锤】（截断） |
| `base.game._events` | table（77 键） | 事件名→处理器列表的**运行时注册状态**（如 `玩家-连入`=13 个处理器、`单位-死亡`=6 个）；内容随地图触发器变化，非 API；各事件子项被截断（每项 `.1`/`.label` 超深），**字段不全** | 【dump 实锤】（运行时状态，截断） |
| `base.game.select_hero` | table（4 函数） | 选人流程游戏侧控制面：`add_hero / enable / enable_same_hero / set_random_mode` | 【dump 实锤】 |
| `base.game.unit_map` | table（0 键） | 单位映射注册表，dump 时为空 | 【dump 实锤】 |
| `base.game.unit` | table（0 键） | 单位相关命名空间，dump 时为空 | 【dump 实锤】 |

函数（124 个，函数名确定、签名未知；按语义分组排列）：

| 函数 | 说明 | 置信 |
| --- | --- | --- |
| `event` `event_dispatch` `event_notify` `event_subscribe` `event_unsubscribe` `event_has` | 游戏级事件总线（`base.game:event('玩家-连入', f)` 为常用范式） | 【dump 实锤】 |
| `load_scene` `load_scene_internal` `close_scene` `close_scene_internal` `has_scene` `get_all_scene_name` `get_all_template_scene_name` `get_default_scene_name` `get_scene_scale` `is_world_scene` | 场景加载/关闭/查询 | 【dump 实锤】 |
| `create_scene_copy` `create_scene_copy_internal` `load_create_scene_copy` | 场景副本 | 【dump 实锤】 |
| `get_scene_object` `get_scene_object_by_key` `get_scene_object_list` `close_scene_object` | 场景对象 | 【dump 实锤】 |
| `set_surrounding` `set_surrounding_scene` `set_surrounding_scene_internal` | 周边场景（无缝地图） | 【dump 实锤】 |
| `end_game` `default_game_result` `set_winner` `get_winner` `get_winner_team` | 对局结束/结算 | 【dump 实锤】（`default_game_result({result='win'/'failed', player=...})` 实参形态【反查推测】） |
| `time_stop` `cancel_time_stop` `is_time_stop` `set_game_speed` `control_game_speed` | 时停/游戏速度 | 【dump 实锤】 |
| `game_valid` `is_ready` `status` `keep_alive` `cancel_keep_alive` `on_reborn` | 对局状态/保活/复活回调 | 【dump 实锤】 |
| `admin_init` `admin_stop` `admin_update_online_time` | 管理员/在线时长 | 【dump 实锤】 |
| `ensure_one_game` | 保证唯一对局（draft_v1 反查的 `ensure_one_lobby` 不在 dump，注意区分） | 【dump 实锤】 |
| `set` `get` `has_property` `get_property` `get_tag` `server_tag` `get_server_tag` | 游戏级键值/属性/标签（`set(key,value)` 实参形态【反查推测】；是否云变量通道待实测） | 【dump 实锤】 |
| `get_session_id` `set_session_start_time` `get_mode_key` `game_mode` `get_env` `get_world_id` `get_lobby_name` | 会话/模式/环境信息 | 【dump 实锤】 |
| `get_friends` `get_friend_apply_list` `invite_friend` `set_invite_friend_callback` `get_friend_middle_game_key` | 好友 | 【dump 实锤】 |
| `create_team` `get_all_team_id` `get_original_match_team_info` `set_middle_game_request_callback` | 队伍/匹配 | 【dump 实锤】 |
| `get_landform` `get_landscape_zaxis` `get_map_size` `get_surface_count` `get_surface_z_all` `get_placement_point` | 地形/地表 | 【dump 实锤】 |
| `pathing_way_points` `ray_cast` `set_sight_block` | 寻路/射线/视野阻挡 | 【dump 实锤】 |
| `get_fow_mode` `switch_fow_mode` `set_fow_sight_height` `get_fow_sight_height` `switch_fov_mode` | 战争迷雾/FOV | 【dump 实锤】 |
| `get_default_unit` `get_default_units` `get_default_item` `init_units` `load_controlled_sync_units` | 物编默认表/单位初始化 | 【dump 实锤】 |
| `mover_function` `mover_line` `mover_target` | 运动器 | 【dump 实锤】 |
| `player_attribute_add` `player_attribute_del` `player_attribute_sync` | 玩家自定义属性同步 | 【dump 实锤】 |
| `unit_attribute_add` `unit_attribute_del` `unit_attribute_max` `unit_attribute_min` `unit_attribute_sync` | 单位自定义属性同步 | 【dump 实锤】 |
| `add_damage` `enable_ai` `disable_ai` | 伤害/AI 开关 | 【dump 实锤】 |
| `play_music` `play_sound` | 全局音频 | 【dump 实锤】 |
| `message` `ui` `filter_word` | 消息/UI/敏感词过滤 | 【dump 实锤】 |
| `json_encode` `json_decode` `do_json_table` `lni` `load_json_file` `load_lua_file` | 序列化/文件加载 | 【dump 实锤】 |
| `object_store_value` `object_restore_value` | 对象附加值存取 | 【dump 实锤】 |
| `statistics_create_actor` `auto_test_log` `allow_record_visit_score` `set_score_random_delay` | 统计/自动化测试/存档 | 【dump 实锤】 |
| `is_editor_debug` `get_editor_debug_mode` `get_editor_debug_user_id` | 编辑器调试态 | 【dump 实锤】 |
| `wtf` `__tostring` | 调试/元方法 | 【dump 实锤】 |

## 组级运行时面：选人（select_hero）双层结构【dump 实锤】

| 路径 | 函数 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.select_hero`（8） | `op_init` `op_click` `op_select` `op_stop` `on_init` `on_click` `on_select` `on_random` | 服务端选人状态机：`op_*` 为主动操作（初始化/点选/确认/停止），`on_*` 为事件回调（初始化/点击/选择/随机）【语义推测】；与 script 客户端变体（hero_list 等）完全不同套 | 【dump 实锤】 |
| `base.game.select_hero`（4） | `add_hero` `enable` `enable_same_hero` `set_random_mode` | 游戏侧选人配置面：加可选英雄/开启选人/允许同英雄/随机模式【语义推测】 | 【dump 实锤】 |

## 组级附注：`_G.base.backend` / `_G.base.room`（语义宿主 = base 全局，非本组 @ 键）

此两表是全任务实测重点（实测已完成，2026-08-27 编辑器 PIE 批次 1~4，探针 `test_res002/.bgd/src/server/test/probe_server_apis.lua`）：

| 路径 | 函数 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.backend`（12） | `url` `stay` `init_game_config` `parse_detail` `send_email` `query_admin_email` `query_score_log` `query_score_log_by_user` `query_score_log_detail` `query_user_email` `query_user_payment` `set_log_sample_per_sessions` | 服务端后端通道：HTTP 请求 + 日志/埋点/支付/邮件查询系列 | 【dump 实锤】+【实测】 |
| `base.room`（3） | `find_game_list` `find_room` `sync_room_info` | 房间查询/上报（详表见 common-base 组 `@common/base/room` 节） | 【dump 实锤】+【实测】 |

**backend 实测明细**【实测】（环境：编辑器 PIE / test_res002 / uid=38672742）：

| 函数 | 实测签名/形态 | 实测行为 |
| --- | --- | --- |
| `url(url)` | GET 单参 | 返回响应体**字符串**（`{code,message,data={timestamp,method,clientIp,params}}`）；**不强制协程**（主线程事件处理器直调成功，疑似同步阻塞），协程内亦可（url_demo 范式） |
| `url(url, 'POST', ...)` | 多参形态 | OK 但**无返回值**，是否真发 POST 无实锤；`(url, {method=...})` 表形态触发**协程级错误（pcall 不拦截，错误对象=table 直达 co.async 处理器）**；`(method, url)` 形态实际以 GET 命中——**实用结论：仅 GET 带响应可靠** |
| `init_game_config()` | 无参 | OK 无返回（lib_game_options 入口必调【源码实锤】） |
| `parse_detail({})` | 表参 | OK 返回 `[{}]` |
| `query_user_payment({map_name, user_id})` | 表参 | OK 返回 `0`（无支付记录；调用点 smallcard_shop【源码实锤】） |
| `query_score_log(map_name, user_id, sub_system, key, begin_time, end_time, callback)` | 7 参（引擎错误消息实锤） | 带回调调用 OK，回调收 `[{}]`（空结果） |
| `query_score_log_by_user(map_name, user_id, begin_time, end_time, callback)` | 5 参（引擎错误消息实锤） | — |
| `query_score_log_detail(map_name, request_id, inner_id, callback)` | 4 参（引擎错误消息实锤） | — |
| `query_user_email(args表, callback)` / `query_admin_email(args表, callback)` | (table, function)，经 `common/base/isolation.lua` 校验层 | 回调收 `"UnknownMySQLException"`（编辑器调试环境 MySQL 侧无表——证实**底层走 MySQL**） |
| `stay(t1, int, t2, int)` | 4 参：1/3 参为 table、2/4 参为 integer（引擎错误消息实锤） | 具体字段未究 |
| `send_email(args, cb)` | 内部需 start_time（算术 nil 错误） | 未走通 |
| `set_log_sample_per_sessions(1)` | 数值参 | OK |

**base.game 补充实测**【实测】：`set(k,v)/get(k)` 回环成功（会话级 KV）；`get_env()`→`"pd"`；`get_session_id()`→大整数；`get_server_tag()` 无参 OK 无返回。

---

### `@common/base/game/error_info`

- 来源：引擎内嵌（script 包 v199 有同名 `common/base/error_info.lua`，客户端 `include 'base.error_info'` 加载；dump 无 `@common/base/error_info` 键）
- 加载：引擎内嵌装载（`require '@common/base/game/error_info'` 形式存在于 package.loaded）
- 状态：⚠️ 无源码（引擎实现）；dump 值 = `true`（已加载无表导出）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 无表导出；按命名为游戏内错误信息收集/展示（对应 script 版 `error_info.lua` 的服务端变体） | 【语义推测】 |

### `@common/base/game/game`

- 来源：引擎内嵌（script 包有同名 `common/base/game.lua`）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

研判：引擎 game 对象本体的装配模块——`_G.base.game` 的 124 函数 + 6 子表主来源（组级表）。【dump 实锤（面）/反查推测（归属）】
常用入口（含调用形态证据）：

| 函数 | 签名/实参形态 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.game:event(name, f)` | `base.game:event('玩家-连入', function(_, player) ... end)` | 游戏事件订阅（常用范式，url_demo.lua 实测环境同款） | 【dump 实锤】+【反查推测】 |
| `base.game:event_dispatch(name, data)` | `('局外资源-更新凭证时间', {player=..., ticket_info_map=...})` | 事件派发（smallcard_get_items 源码调用点） | 【源码实锤】（调用点） |
| `base.game:event_notify(name, ...)` | `('场景-请求切换', old_scene, new_scene)` | 事件通知（script scene.lua 调用点） | 【源码实锤】（调用点） |
| `base.game:set(key, value)` / `:get(key)` | `('hp', value)` 等（draft_v1 语料） | 游戏级键值 | 【dump 实锤】+【反查推测】 |
| `base.game:load_scene(scene)` `:close_scene(scene)` | `(scene_name)` | 场景加载/关闭 | 【dump 实锤】+【反查推测】 |
| `base.game:default_game_result(data)` | `({result='win', player=player})` | 默认结算 | 【dump 实锤】+【反查推测】 |

### `@common/base/game/game_message`

- 来源：引擎内嵌（script 包无同名文件）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 游戏消息（聊天/系统广播）。关联：`base.game.message` 函数在 dump 中【dump 实锤】；`base.game:chat(type,msg)`/`broadcast(name,f)` 形态见客户端语料，服务端 dump 未见【反查推测】 | 【语义推测】 |

### `@common/base/game/game_result`

- 来源：引擎内嵌（script 包有同名 `common/base/game_result.lua`，且 `common/base/init.lua` L164 在 `__lua_state_name == 'StateGame'` 分支 `require 'base.game_result'`；但 dump 无 `@common/base/game_result` 键）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

研判：结算模块。script init.lua 的 StateGame 分支 require 落到了本引擎内嵌变体上（加载链被接管）【反查推测】。关联 dump 实锤函数：`base.game.end_game / default_game_result / set_winner / get_winner / get_winner_team`。

### `@common/base/game/item`

- 来源：引擎内嵌（script 包有同名 `common/base/item.lua`——dump 中 `@common/base/item` 为独立键且 `base.item` 表 73 键存在，属 common-base 组；本键为其 game 侧扩展）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 函数（调用形态） | 说明 | 置信 |
| --- | --- | --- |
| `create_to_point(link/id, point, scene)` | 在点创建物品（语料：`create_to_point(link, hero:get_point())` 等） | 【反查推测】 |
| `create_to_unit(id, target)` | 直接给单位物品 | 【反查推测】 |
| `load_item_from_info(info, unit)` | 从存档信息还原物品 | 【反查推测】 |
| `bind_items_to_user(items, player, ok, err)` / `unbind_items_to_user(player, ok, err)` | 物品与账号绑定 | 【反查推测】 |
| `save_score_to_unit(unit, ok, err)` / `load_score_to_unit(unit, ok, err)` | 单位背包云端存档读写（回调式） | 【反查推测】 |
| `get_player_score_item_list(player)` | 取玩家存档物品列表 | 【反查推测】 |

关联 dump 实锤：`base.game.get_default_item`；`base.item`（73 键）归 `@common/base/item`。

### `@common/base/game/learn_skill`

- 来源：引擎内嵌（script 包无同名文件）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 学习技能流程（关联 dump 事件 `_events['技能-获得']`/`['单位-学习技能完成']` 运行时注册） | 【语义推测】 |

### `@common/base/game/level_up`

- 来源：引擎内嵌（script 包无同名文件）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 升级流程（关联 dump 事件 `_events['单位-升级']`） | 【语义推测】 |

### `@common/base/game/player`

- 来源：引擎内嵌（script 包有同名 `common/base/player.lua`，`@common/base/player` 为独立键，属 common-base 组；本键为 game 侧扩展）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 函数（调用形态） | 说明 | 置信 |
| --- | --- | --- |
| `create(user_id, type, behavior...)` | 创建 AI 玩家（语料：`create(ai_info[i].user_id, ai_info[i].type, ai_info[i].behavior_...)`） | 【反查推测】 |

lua_plus 扁平封装 `base.player_*`（约 30 个，签名+@ui 注解齐全）属 lua-plus 组文档，不在此重复。

### `@common/base/game/reborn`

- 来源：引擎内嵌（script 包无同名文件）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 复活流程；关联 dump 实锤 `base.game.on_reborn` 函数与 `_events['单位-复活']` 注册 | 【语义推测】+【dump 实锤】（关联面） |

### `@common/base/game/scene`（本组唯一有表导出的键）

- 来源：引擎内嵌；**部分对照源码**：script 包 v199 `common/base/scene.lua`（返回 5 函数，与本键 7 函数中的 5 个同名同语义——引擎内嵌版是其扩展变体）
- 加载：`require '@common/base/game/scene'` 返回函数表；调用方（script `common/base/trigger.lua`）以 `scene_manager.set_scene_activated(...)` 等形态使用【源码实锤】
- 状态：⚠️ 无源码（引擎变体）；dump 值 = table（7 函数）

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `set_scene_activated(scene)` | `(scene)` | 标记场景激活并订阅该场景事件（script 版 L81-85：`scenes[scene]=true` + `subscribe_scene_events`） | 【dump 实锤】+【源码实锤】（script 变体签名） |
| `set_scene_not_activated(scene)` | `(scene)` | 取消激活并退订事件（script 版 L87-91） | 【dump 实锤】+【源码实锤】 |
| `is_scene_activated(scene)` | `(scene) -> boolean` | 查询激活态（script 版 L106-109） | 【dump 实锤】+【源码实锤】 |
| `get_activated_scenes()` | `() -> table` | 返回全部激活场景名列表（script 版 L111-119） | 【dump 实锤】+【源码实锤】 |
| `get_obj_scene_events(scene, obj)` | `(scene, obj) -> table` | 取/建对象在场景下的事件委托表（script 版 L129-141；obj 可为函数则延迟求值） | 【dump 实锤】+【源码实锤】 |
| `init_region` | 未知 | script 版**无此函数**；`init_region` 另见于 `common/base/circle.lua`/`rect.lua` 的 metatable 方法（`mt:init_region([filter])`）；引擎变体扩展项，语义待考 | 【dump 实锤】（名）【语义推测】 |
| `check_event_scene_region_remove` | 未知 | 全语料（api-13 全量源码）零命中；引擎变体扩展项，疑为场景事件区域移除检查 | 【dump 实锤】（名）【语义推测】 |

附注：script 版 scene.lua 还注册 `base.proto.__server_jump_scene` 处理器（派发 `场景-请求切换` 事件）并维护 `base._scene` 注册表——这些副作用在引擎变体中是否保留待考。

### `@common/base/game/select_hero`

- 来源：引擎内嵌（script 包有同名 `common/base/select_hero.lua` 客户端变体；**服务端已被引擎变体整体覆盖**，见组级结论 2）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）；运行时面经 `base.select_hero` + `base.game.select_hero` 两处暴露【dump 实锤】

函数全集见组级「选人双层结构」表。语义概括：`op_init/op_click/op_select/op_stop` 驱动服务端选人流程，`on_init/on_click/on_select/on_random` 为对应事件回调【语义推测】；`base.game.select_hero.add_hero/enable/enable_same_hero/set_random_mode` 为地图侧配置入口【语义推测】。**待实测**：两层函数的调用顺序与协议交互（见 FINDINGS-game-smallcard.md）。

### `@common/base/game/shop`

- 来源：引擎内嵌（script 包有 `common/base/shop.lua`，`@common/base/shop` 为独立键值 `true`，属 common-base 组）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）；`_G.base` 无 `shop` 字段【dump 实锤】

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 游戏内商店流程；服务端无独立全局面，能力疑并入 engine unit/item 体系 | 【语义推测】 |

### `@common/base/game/template_buff`

- 来源：引擎内嵌（script 包无同名文件；`common/base/init.lua` L135 `include "base.template"` 为另一文件）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 模板 Buff（按命名：物编模板化 buff 装配） | 【语义推测】 |

### `@common/base/game/ui`

- 来源：引擎内嵌（script 包有 `common/base/ui.lua` 由 init.lua L134 `require "base.ui"` 加载，`@common/base/ui` 属 common-base 组）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

关联证据：`base.game.ui` 函数在 dump 124 内【dump 实锤】；draft_v1 反查的 `component('xxx')`/`button(props)`/`bind_unit(unit, bind, x, y)`/`add_wait_to_create_ctrl` 等形态为客户端 UI 组件体系语料【反查推测】，服务端面待考。

### `@common/base/game/unit`

- 来源：引擎内嵌（script 包有同名 `common/base/unit.lua`，`@common/base/unit` 属 common-base 组）
- 加载：引擎内嵌装载
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）

关联证据（dump 实锤）：`base.game.unit`（空表）、`base.game.unit_map`（空表）两个命名空间子表由引擎预置；`base.game.init_units / load_controlled_sync_units / get_default_unit(s)` 为单位初始化面。lua_plus 扁平封装 `base.unit_*`（60+ 个，签名齐全）属 lua-plus 组文档。
