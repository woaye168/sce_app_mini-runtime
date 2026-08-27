# FINDINGS-game-smallcard.md — common-base-game / smallcard-libs 分组研究发现（draft_v2）

> 研究子代理落盘，2026-08-27。补充 FINDINGS.md（不改动原文件）。产出：`draft_v2/common-base-game.md`（15 键）、`draft_v2/smallcard-libs.md`（19 键）+ 两份 status JSON。

## 新发现

### G1. 引擎内嵌 `game/*` 接管 script 包同名文件（common-base-game 组核心结论）

- script 包 v199 `common/base/` 下有同名文件 `error_info/game/game_result/item/player/scene/select_hero/unit.lua`（8 个），但 dump 中**没有** `@common/base/error_info`、`@common/base/game_result`、`@common/base/scene`、`@common/base/select_hero` 键——只有 `game/*` 变体。
- 实锤：script 版 `select_hero.lua` 定义 `base.select_hero` = hero_list/select_hero/click_hero/click_random_hero/show_timer/show_hero/show_random（客户端向，调引擎 `game.request_pick_hero`）；dump 的 `_G.base.select_hero` = on_click/on_init/on_random/on_select/op_click/op_init/op_select/op_stop（服务端向）。**两套完全不同 → 服务端引擎变体整体覆盖 script 版**。
- 旁证：`common/base/init.lua` L164 StateGame 分支 `require 'base.game_result'`，但 dump 无对应 `@common/base/game_result` 键 → 加载被重定向到 `@common/base/game/game_result`。
- **draft_v1 修正**：select_hero 节的 hero_list 等反查清单实为 script 客户端变体签名，服务端不存在。
- 待办：要拿到这 15 个引擎内嵌模块的真实源码，走引擎内嵌包提取（sce_app_editor-patch `doc/research/pak-extract-guide.md`）。

### G2. `_G.base.game` 130 键精确构成（修正 FINDINGS.md "130 键" 的细分）

124 函数 + 6 子表：`_scene_copy{}`、`_scene_object_map`（1 键 default，其子树 6 字段全部 `<max depth exceeded>` 截断）、`_events`（77 个事件的运行时处理器注册表，非 API，子项截断）、`select_hero`（4 函数：add_hero/enable/enable_same_hero/set_random_mode——**新发现的游戏侧选人配置面**）、`unit_map{}`、`unit{}`。

### G3. `@common/base/game/scene` 是本组唯一有表导出的键

7 函数中 5 个（set_scene_activated/set_scene_not_activated/is_scene_activated/get_activated_scenes/get_obj_scene_events）与 script/199/common/base/scene.lua 返回值同名同语义（引擎扩展变体）；`init_region`、`check_event_scene_region_remove` 为引擎新增（后者全语料零命中）。调用方形态 `scene_manager.xxx` 见 script `common/base/trigger.lua` L246-361。

### G4. draft_v1 组级反查清单口径错误

draft_v1 头部的 `base.game.debug_draw_* / circle_selector / camera_focus / chat / lock_camera / ensure_one_lobby / one_more_round / load_combined_map` 等均**不在服务端 dump 的 base.game 124 函数内**——客户端语料混入。draft_v2 函数表以 dump 为服务端权威面。

### G5. smallcard 组：dump 同 state 装载双端

`ui/script/trigger_*` 客户端文件以值=true 出现在"服务端" dump——编辑器 PIE StateGame 单 Lua state 同时加载双端代码。ui 键存在 ≠ 服务端可用。

### G6. 触发编辑器模块装载范式被 dump 值证实

包入口编译尾 `ret={[包名]=命名空间} + ____module 合并 + ____return 合并`：`@smallcard_inventory/main` 值 = {打开玩家背包, 关闭玩家背包, smallcard_inventory={open/close_player_inventory}}；`@smallcard_mail/main` 值 = {smallcard_mail={}}（无导出）。**draft_v1 对 inventory/main "未提取到顶层函数" 的研判错误**，已修正。

### G7. `@smallcard_get_items/main` 值含 2 个"无处定义"的类

`smallcard_get_items.获得局外资源`（继承 TriggerEvent，super.prototype 3 处截断）与 `smallcard_get_items.lobby_resource_instance`：api-13 全量源码、dump 所属项目（p_55a3 值树）、test_res002 项目三处检索均无定义 → 触发编辑器按地图触发器数据编译进包命名空间的自定义事件/数据类【反查推测】。

### G8. 无源码模块能力面被客户端反查补全

- inventory：上行 9 消息（pick_item/drop_item/move_item/drop_to_unit/drop_to_point/use_item/sale_item/unequip_item/equip_item，客户端 `base.game:server` 调用点）→ handler 必在未下发的 proto/proto_v2；下行 5 消息（open/close_inv、toast×3、drop_event，客户端 `base.proto` 注册点）。
- mail：上行 receive_mail/receive_all_mail/read_mail/smallcard_mail_get_mail_list（`base.ui.proto` 注册，src/main.lua 实锤，3 个 official_awards 系列已废弃为空函数）；下行 smallcard_mail_get_mail_list/_part_list。
- get_items：上行 exchange_resource/choose_resource_index；下行 get_items(_new/_custom)/choose_items/ticket_info_update/alert_message_new/lack_alert_message_new/smallcard_get_items_refresh_token。
- `@smallcard_mail/mail` = 9 字段邮件数据模板（id/title/content/sender/receiver/awards/send_time/end_time 默认空串/空表）+ new()。
- `@smallcard_get_items/module`（28 键 20 函数）与 `@smallcard_mail/module`（13 键 9 函数）值树完整无截断；均有 `IsOpen=true` 标志 + 若干运行时缓存空表。

### G9. 佐证：`base.s` 39 函数含 `score_init/money_init/world_data_init/list_query/get_commit/stat_upload/test_cloud_value` 等

与 smallcard_get_items src/main.lua 的 `base.s.score_init/list_query/get_commit` 用法吻合——服务端云数据通道面（详归 lua-plus/common-base 组文档）。

## 待实测问题清单（交主控安排实机验证）

1. **backend.url 边界**：除 GET 外是否支持 POST/headers/body？超时？返回结构（status/body/headers?）？是否计次限频？（url_demo.lua 已验证基本可用【实测】，边界未知）
2. **backend query/admin 系列**：`query_score_log*`/`query_user_payment`/`query_user_email`/`query_admin_email`/`send_email`/`stay`/`set_log_sample_per_sessions` 的参数与返回结构；`init_game_config` 是否为其他函数的前置。
3. **room 三函数**：`find_game_list`/`find_room`/`sync_room_info` 的语义（房间列表？跨房匹配？）与返回结构。
4. **select_hero 双层交互**：`base.game.select_hero.enable/add_hero/...`（配置）→ `base.select_hero.op_init/op_click/...`（流程）的调用顺序；`on_*` 回调的挂接方式与触发时机；与客户端 script 变体（hero_list/request_pick_hero 链）的协议对接。
5. **base.game.set/get**：是否云变量/全局同步通道？键空间（语料有 'hp'/'scene'/'task.accept'/'session_id' 等）与服务端语义；与 `base.s.*` 云存档的关系。
6. **协程模型通用规则**：除 url 外，query_*/room/score 系列哪些必须在 `co.async` 协程内调用；`base.wait` 回调范式边界（以 url_demo.lua 为起点归纳）。
7. **score_save（inventory）**：背包云存档的 score key 命名、读写时机、与 `@common/base/tds_score`（116 函数）的通道关系。
8. **引擎内嵌包提取**：`@common/base/game/*` 15 键的真实源码获取（pak-extract 路线），可一次性解决 G1/G3 的全部【语义推测】项。
