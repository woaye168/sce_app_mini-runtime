# 官方小卡地图库（smallcard_*）

模块数：19。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）。

源码覆盖：13/19；其余标注 ⚠️ 无源码并附调用点反查/语义推测。

---

### `@smallcard_get_items/main`

- 归属：smallcard_get_items 包
- 状态：✅ 有源码（`smallcard_get_items\111\src\main.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `smallcard_get_items_refresh_ticket` | `(player)` |  |
| `ok` | `(score, iscore, sscore)` |  |
| `ok` | `(result)` |  |
| `ok` | `()` |  |
| `error` | `()` |  |
| `get_ticket_remain_time` | `(player, ticket)` |  |
| `resource_gain` | `(player, v2_committer, resource_list)` |  |
| `get_player_resource_buffer` | `(player, key)` |  |
| `set_player_resource_buffer` | `(player, key, value)` |  |
| `clear_player_resource_buffer` | `(player, key)` |  |

### `@smallcard_get_items/module`

- 归属：smallcard_get_items 包（未找到源码）
- 研判：获得物品弹窗的服务端逻辑模块，未随包分发（推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@smallcard_get_items/require_libs`

- 归属：smallcard_get_items 包
- 状态：✅ 有源码（`smallcard_get_items\111\ui\script\require_libs.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@smallcard_get_items/trigger_module_main_1`

- 归属：smallcard_get_items 包
- 状态：✅ 有源码（`smallcard_get_items\111\ui\script\trigger_module_main_1.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `DialogButtonConfig.prototype.____constructor` | `(self, ButtonContent, ButtonStyle)` |  |

### `@smallcard_get_items/trigger_validator`

- 归属：smallcard_get_items 包
- 状态：✅ 有源码（`smallcard_get_items\111\ui\script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `init_validator_0` | `(self)` |  |

### `@smallcard_inventory/main`

- 归属：smallcard_inventory 包
- 状态：✅ 有源码（`smallcard_inventory\80\src\main.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@smallcard_inventory/proto`

- 归属：smallcard_inventory 包（未找到源码）
- 研判：背包服务器协议定义，未随编辑器侧包分发（推测为服务端 proto 注册表）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `InGame_S2C_friend_apply_fail(data)`
  - `InGame_S2C_init_friend_apply_list(data)`
  - `InGame_S2C_init_friend_list(data)`
  - `InGame_S2C_notice_friend_state(data)`
  - `__add_attribute_and_sync_client(msg)`
  - `__gm_debug_ai_order(msg)`
  - `__gm_debug_eff_destory(msg)`
  - `__gm_debug_eff_destory_all(msg)`
  - `__gm_debug_eff_info(msg)`
  - `__gm_debug_eff_keep(msg)`
  - `__gm_debug_game(msg)`
  - `__gm_debug_player(msg)`
  - `__gm_debug_unit(msg)`
  - `__item_try_drop_result(msg)`
  - `__one_more_round(data)`
  - `__return_check_text(msg)`
  - `__return_default_unit(msg)`
  - `__server_custom_event_struct_creater(msg)`
  - `__server_event_to_client(msg)`
  - `__server_jump_scene(msg)`
  - `__set_attribute_custom_format(msg)`
  - `__unit_try_pick_item_result(msg)`
  - `__update_collision_info(msg)`
  - `_lib_gamechat_cheat_info(msg)`
  - `_lib_gamechat_set_debug_mode(msg)`
  - `_set_game_speed(msg)`
  - `bind(data)`
  - `cancel_ignore_joy_stick(msg)`
  - `clock(clock)`
  - `default_game_result(data)`
  - `lobby_game_exit(data)`
  - `reload()`
  - `s2c_rpc(data)`
  - `server_package_info(table)`
  - `set_camera(msg)`
  - `skill_group_set_unit(msg)`
  - `subscribe(data)`
  - `sync_skill(msg)`
  - `unit_get_interaction_spell(msg)`
  - `unit_remove_interaction_spell(msg)`
  - `消息名(table)`

### `@smallcard_inventory/proto_v2`

- 归属：smallcard_inventory 包（未找到源码）
- 研判：背包服务器协议定义，未随编辑器侧包分发（推测为服务端 proto 注册表）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@smallcard_inventory/require_libs`

- 归属：smallcard_inventory 包
- 状态：✅ 有源码（`smallcard_inventory\80\ui\script\require_libs.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@smallcard_inventory/score_save`

- 归属：smallcard_inventory 包（未找到源码）
- 研判：背包数据云端存档（score/save 服务）模块（推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@smallcard_inventory/trigger_module_main_1`

- 归属：smallcard_inventory 包
- 状态：✅ 有源码（`smallcard_inventory\80\ui\script\trigger_module_main_1.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `smallcard_inventory.open_right_inv` | `()` |  |
| `smallcard_inventory.open_left_inv` | `()` |  |
| `背包界面关闭时.prototype.____constructor` | `(self, obj)` |  |
| `trigger_class.prototype.____constructor` | `(self)` |  |
| `物品提示信息框显示.prototype.____constructor` | `(self, obj, ui, item)` |  |
| `物品提示信息框隐藏.prototype.____constructor` | `(self, obj)` |  |

### `@smallcard_inventory/trigger_validator`

- 归属：smallcard_inventory 包
- 状态：✅ 有源码（`smallcard_inventory\80\ui\script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `init_validator_0` | `(self)` |  |

### `@smallcard_inventory/关闭玩家背包`

- 归属：smallcard_inventory 包
- 状态：✅ 有源码（`smallcard_inventory\80\src\关闭玩家背包.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `关闭玩家背包` | `(player:player)` | lua_plus --- |

### `@smallcard_inventory/打开玩家背包`

- 归属：smallcard_inventory 包
- 状态：✅ 有源码（`smallcard_inventory\80\src\打开玩家背包.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `打开玩家背包` | `(player:player)` | lua_plus --- |

### `@smallcard_mail/mail`

- 归属：smallcard_mail 包（未找到源码）
- 研判：邮件系统服务端逻辑（邮件读写/附件发放），未随包分发（推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@smallcard_mail/main`

- 归属：smallcard_mail 包
- 状态：✅ 有源码（`smallcard_mail\75\src\main.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.ui.proto.receive_all_mail` | `(player)` |  |
| `base.ui.proto.receive_mail` | `(player, data)` |  |
| `base.ui.proto.read_mail` | `(player, data)` |  |
| `base.ui.proto.smallcard_mail_get_mail_list` | `(player)` |  |
| `base.ui.proto.send_official_awards` | `(player, rewards_str_list)` | 待废弃 |
| `base.ui.proto.get_official_awards` | `(player)` |  |
| `base.ui.proto.delete_official_awards` | `(player, id)` |  |

### `@smallcard_mail/module`

- 归属：smallcard_mail 包（未找到源码）
- 研判：邮件系统服务端模块注册/入口（推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@smallcard_mail/trigger_module_main_1`

- 归属：smallcard_mail 包
- 状态：✅ 有源码（`smallcard_mail\75\ui\script\trigger_module_main_1.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mail_main_dialog_opened.prototype.____constructor` | `(self, obj)` |  |
| `mail_main_dialog_closed.prototype.____constructor` | `(self, obj)` |  |

### `@smallcard_mail/trigger_validator`

- 归属：smallcard_mail 包
- 状态：✅ 有源码（`smallcard_mail\75\ui\script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `init_validator_0` | `(self)` |  |
