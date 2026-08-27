# smallcard-libs 组：官方小卡地图库（smallcard_get_items / smallcard_inventory / smallcard_mail）

模块数：19。来源：服务端 `package.loaded` dump + api-13 解密源码对照。

源码覆盖：13/19 有源码；6 键无源码（`get_items/module`、`inventory/proto`、`inventory/proto_v2`、`inventory/score_save`、`mail/mail`、`mail/module`），以 dump 值树为主证据。

## 组级结论（draft_v2 新增）

1. **dump 同时装载双端模块（重要背景）**：本组键既有服务端入口（`src/main.lua`）也有客户端文件（`ui/script/trigger_*` 等，值=`true`）——编辑器 PIE 的 StateGame 单 Lua state 同时加载双端代码。因此"服务端 dump"中的 ui 键不代表服务端可调用其功能。
2. **触发编辑器模块装载范式（三键 main 共用）**：包入口源码尾部（编译期附加）模式为
   `local ret = {["<包名>"] = <包名>命名空间}; for k,v in pairs(____module or {}) do ret["<包名>"][k]=v end; for k,v in pairs(____return or {}) do ret[k]=v end; return ret`。
   dump 值与此精确吻合：`@smallcard_inventory/main` 值 = `{打开玩家背包, 关闭玩家背包, smallcard_inventory={open_player_inventory, close_player_inventory}}`【dump 实锤】；`@smallcard_mail/main` 值 = `{smallcard_mail={}}`（无 ____module/____return，命名空间为空）【dump 实锤】。
3. **`@smallcard_get_items/main` 返回值含 2 个无源码类**（`获得局外资源`、`lobby_resource_instance`）：全量 api-13 源码与 dump 项目（p_55a3）源码均检索不到定义——**触发编辑器按地图触发器数据编译进包命名空间的自定义事件/数据类**【反查推测】。其中 `获得局外资源` 继承 `TriggerEvent`（`____super` 实锤，但 super.prototype 三处字段被 `<max depth exceeded>` 截断，**字段不全**）【dump 实锤】。
4. **无源码三协议/存档模块的能力面由客户端反查补全**（见各节）：inventory 客户端→服务端 9 个 `smallcard_inventory_*` 消息、mail 4+2 个消息、get_items 2 个上行 + 8 个下行——这些消息的 handler 必在未下发的 `proto`/`proto_v2`/`score_save`/`module` 内【反查推测】。

---

## smallcard_get_items 包（v111，获得物品/局外资源弹窗库）

### `@smallcard_get_items/main`

- 来源：smallcard_get_items 包 v111
- 加载：包服务端入口（`require '@common'` → `require 'module'` → `module:open()`）
- 状态：✅ 有源码（`smallcard_get_items\111\src\main.lua`）；dump 值 = table（1 键 `smallcard_get_items` 命名空间，7 成员）

**源码侧行为**（顶层副作用，【源码实锤】）

- 定义局部函数 `smallcard_get_items_refresh_ticket(player)`：经 `base.s.score_init(player, {ok=...}, 'smallcard_get_items_ticket_time_info', 'ticket_op_id')` 读特权卡时间 → `base.s.list_query(player, 'smallcard_get_items_ticket_buffer', ...)` 取缓冲 → 逐条结算到期时间（含无限卡/自然日逻辑）→ `base.s.get_commit()` 事务 `list_delete`+`score_set`+`name_new('更新特权时间', 'ticket_<user_id>_<op_id>', user_id)`+`commit` 落云 → 派发 `base.game:event_dispatch('局外资源-更新凭证时间', {player, ticket_info_map})` + `player:ui 'ticket_info_update'{...}` 通知客户端 → 写 `module.ticket_info_map[player_id]`。
- `base.wait(1000, ...)`：开局 1 秒后为全部真人玩家 `score_seti('open_singin_unread', 1)` 并 `commit('红点状态更新')`。
- 事件挂钩：`base.game:event('玩家-连入', ...)` 与 `base.game:event('局外资源-刷新权限', ...)` → 刷新特权卡。

**函数**（dump 值 `smallcard_get_items` 命名空间内，即源码 `____module` 导出，签名取自源码）

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `get_ticket_remain_time(player, ticket)` | `(player, ticket)` | 转发 `module:get_ticket_remain_time` | 【dump 实锤】+【源码实锤】 |
| `resource_gain(player, v2_committer, resource_list)` | `(player, v2_committer, resource_list)` | 转发 `module:get_items_commit_v2_for_trigger`（发放局外资源，触发器入口） | 【dump 实锤】+【源码实锤】 |
| `get_player_resource_buffer(player, key)` | `(player, key)` | 读玩家资源缓冲 | 【dump 实锤】+【源码实锤】 |
| `set_player_resource_buffer(player, key, value)` | `(player, key, value)` | 写玩家资源缓冲 | 【dump 实锤】+【源码实锤】 |
| `clear_player_resource_buffer(player, key)` | `(player, key)` | 清玩家资源缓冲 | 【dump 实锤】+【源码实锤】 |

**类**（值树实锤，包内与项目侧均无源码——触发编辑器编译产物【反查推测】）

| 类 | 继承 | 构造/方法 | 置信 |
| --- | --- | --- | --- |
| `smallcard_get_items.获得局外资源` | `____super = TriggerEvent`（TSTL 类） | `prototype.____constructor` 存在；super.prototype 的 `__index/constructor/____constructor` 三处 `<max depth exceeded>` 截断（字段不全）；`____superTypeArgumentsFunc` 存在 | 【dump 实锤】 |
| `smallcard_get_items.lobby_resource_instance` | 无 super | 仅 `name` + `prototype.____constructor`（prototype 3 键，与 lualib_bundle/tds_score 等共享基类原型内联） | 【dump 实锤】 |

**与 draft_v1 差异**：draft_v1 列的 `smallcard_get_items_refresh_ticket / ok / error` 等是源码内部局部函数/回调，不导出；dump 值证明实际导出为 `____module` 5 函数 + 2 类。`require 'require_libs'` 行在服务端入口引用的是 `ui/script/require_libs.lua`（`lib_common_sounds = require"@lib_common_sounds.main"`）。

### `@smallcard_get_items/module`

- 来源：smallcard_get_items 包（服务端协议/资源结算核心，**不下发编辑器**）
- 加载：被 `src/main.lua` `require 'module'` 并以冒号法调用
- 状态：⚠️ 无源码；dump 值 = table（**28 键：20 函数 + 1 标量 + 7 空表**）——本组无源码键中证据最足的一个

**函数**

| 函数 | 签名/形态 | 说明 | 置信 |
| --- | --- | --- | --- |
| `open` | `(self)`（`module:open()`） | 模块初始化/开启 | 【dump 实锤】+【源码实锤】（调用点） |
| `get_ticket_remain_time` | `(self, player, ticket)` | 特权卡剩余时间 | 【dump 实锤】+【源码实锤】（调用点） |
| `get_items_commit_v2_for_trigger` | `(self, player, v2_committer, resource_list)` | 资源发放提交（触发器入口，main.lua 的 `resource_gain` 转发至此） | 【dump 实锤】+【源码实锤】（调用点） |
| `get_player_resource_buffer` / `set_player_resource_buffer` / `clear_player_resource_buffer` | `(self, player, key[, value])` | 玩家资源缓冲读写 | 【dump 实锤】+【源码实锤】（调用点） |
| `get_items_commit` `get_items_commit_new` `get_items_commit_v2` `get_items_commit_new_v2` | 未知 | 发放提交四代同堂（v2/new 为演进版） | 【dump 实锤】（名）【语义推测】 |
| `cost_items_commit` `cost_items_commit_v2` | 未知 | 资源消耗提交 | 【dump 实锤】（名）【语义推测】 |
| `register_custom_add_func` `register_custom_cost_func` | 未知 | 注册自定义获得/消耗处理（配 `custom_add_function_map`/`custom_cost_function_map`） | 【dump 实锤】（名）【语义推测】 |
| `event_get_resource` | 未知 | 获得资源事件处理 | 【dump 实锤】（名）【语义推测】 |
| `get_resource_count` `get_token_count` | 未知 | 资源/代币数量查询 | 【dump 实锤】（名）【语义推测】 |
| `refresh_token` `check_ticket_active` | 未知 | 代币刷新/特权卡生效检查 | 【dump 实锤】（名）【语义推测】 |
| `regist_auto_sync_money` | 未知 | 注册货币自动同步（配 `player_money_cache`/`player_money_callback_cache`） | 【dump 实锤】（名）【语义推测】 |

**字段/子表**

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `IsOpen` | scalar `true` | 模块开启标志 | 【dump 实锤】 |
| `ticket_info_map` | table（0 键） | 玩家特权卡时间缓存（`[player_id]=ticket_info_map`，main.lua 写入） | 【dump 实锤】+【源码实锤】（写入点） |
| `custom_add_function_map` `custom_cost_function_map` | table（0 键） | 自定义获得/消耗函数注册表 | 【dump 实锤】 |
| `player_money_cache` `player_money_callback_cache` | table（0 键） | 货币缓存/回调缓存 | 【dump 实锤】 |
| `player_commit_buffer` | table（0 键） | 提交缓冲 | 【dump 实锤】 |
| `lobby_resource_money_list` | table（0 键） | 局外货币列表 | 【dump 实锤】 |

**协议面（客户端反查）**：上行 `exchange_resource{resource_link, count, exchange_link}`、`choose_resource_index{resource_link, index}`（ui/script 多处 `base.game:server` 调用点）【反查推测】；下行 `get_items`/`get_items_new`/`get_items_custom`/`choose_items`/`ticket_info_update`/`alert_message_new`/`lack_alert_message_new`/`smallcard_get_items_refresh_token`（客户端 `base.proto.*` 注册点，由本模块或引擎侧推送）【源码实锤】（客户端注册点）。

### `@smallcard_get_items/require_libs`

- 来源：smallcard_get_items 包；状态：✅ 有源码（`ui\script\require_libs.lua`）；dump 值 = `true`
- 内容（全文 1 行副作用）：`lib_common_sounds = require"@lib_common_sounds.main"`——挂载音效库全局。【源码实锤】

### `@smallcard_get_items/trigger_module_main_1`

- 来源：smallcard_get_items 包；状态：✅ 有源码（`ui\script\trigger_module_main_1.lua`，客户端）；dump 值 = `true`（无 return）
- 行为：声明 35 个 TSTL 引擎类全局（RegionCircle…Array），建 `smallcard_get_items` 命名空间并注册 1 个类：

| 类 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `smallcard_get_items.DialogButtonConfig` | `prototype.____constructor(self, ButtonContent, ButtonStyle)` | 对话框按钮配置（`@name 对话框按钮配置`），纯数据类 | 【源码实锤】 |

- **draft_v1 修正**：此类定义在客户端文件内；服务端 dump 值=true 仅因 StateGame 同 state 装载，服务端不可用其语义。

### `@smallcard_get_items/trigger_validator`

- 来源：smallcard_get_items 包；状态：✅ 有源码（`ui\script\trigger_validator.lua`）；dump 值 = `true`
- 行为：声明 TSTL 类全局 + `validator = validator or {}` + 空调用 `init_validator_0(nil)`（触发校验器骨架，无实际逻辑）。【源码实锤】

---

## smallcard_inventory 包（v80，背包库）

### `@smallcard_inventory/main`

- 来源：smallcard_inventory 包 v80
- 加载：包服务端入口（`require 'require_libs' 'proto' 'proto_v2' 'score_save' '打开玩家背包' '关闭玩家背包'`）
- 状态：✅ 有源码（`smallcard_inventory\80\src\main.lua`）；dump 值 = table（3 键）——与源码 `____return`/`____module` 合并尾精确一致【dump 实锤】

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `打开玩家背包(player)` | `(player:player)` | lua_plus 触发器函数（`---@ui 打开玩家~1~的背包界面`），实现 = `player:ui'__smallcard_inventory_open_inv'{}` | 【dump 实锤】+【源码实锤】 |
| `关闭玩家背包(player)` | `(player:player)` | 同上，`player:ui'__smallcard_inventory_close_inv'{}` | 【dump 实锤】+【源码实锤】 |
| `smallcard_inventory.open_player_inventory` | = `打开玩家背包` | 命名空间别名（____module 导出） | 【dump 实锤】+【源码实锤】 |
| `smallcard_inventory.close_player_inventory` | = `关闭玩家背包` | 命名空间别名 | 【dump 实锤】+【源码实锤】 |

**与 draft_v1 差异**：draft_v1 称"未提取到顶层函数定义，可能为纯数据/副作用模块"——错误，实际导出上述 4 个函数（源码 `____return`/`____module` 模式未被旧提取器识别）。

### `@smallcard_inventory/proto`

- 来源：smallcard_inventory 包（服务端协议注册，**不下发编辑器**）
- 加载：被 `src/main.lua` `require 'proto'`
- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出，纯副作用注册）

**协议面（客户端反查，handler 必在本模块/proto_v2 内）**【反查推测】：

| 客户端→服务端消息 | 触发场景（调用点文件） | 置信 |
| --- | --- | --- |
| `smallcard_inventory_pick_item` | 拾取物品（interact.lua / pick_up.lua / uieditor/pick_list*.lua / smart_pick_button.lua） | 【源码实锤】（客户端调用点） |
| `smallcard_inventory_drop_item` | 丢弃（drop_confirm.lua） | 【源码实锤】（调用点） |
| `smallcard_inventory_move_item` | 移动格子（main_panel.lua ×2） | 【源码实锤】（调用点） |
| `smallcard_inventory_drop_to_unit` / `smallcard_inventory_drop_to_point` | 丢给单位/丢到点（main_panel.lua） | 【源码实锤】（调用点） |
| `smallcard_inventory_use_item` | 使用（main_panel.lua） | 【源码实锤】（调用点） |
| `smallcard_inventory_sale_item` | 出售（main_panel.lua） | 【源码实锤】（调用点） |
| `smallcard_inventory_unequip_item` / `smallcard_inventory_equip_item` | 卸下/装备（main_panel.lua） | 【源码实锤】（调用点） |

服务端→客户端下行（客户端注册点）：`__smallcard_inventory_open_inv`/`__smallcard_inventory_close_inv`（inventory/main.lua）、`smallcard_inventory_toast/toast2/toast3`（inventory/toast.lua）、`smallcard_inventory_drop_event`（inventory/item_sync.lua）【源码实锤】（客户端注册点）。

**draft_v1 修正**：draft_v1 本节列的 40 个形态（`InGame_S2C_*`、`__gm_debug_*`、`bind/clock/reload` 等）是**全语料 proto 注册表的混杂反查，非本模块归属**，已全部移除；以 dump 值=true + 上述 inventory 专属消息链为准。

### `@smallcard_inventory/proto_v2`

- 来源：smallcard_inventory 包（不下发）
- 加载：被 `src/main.lua` `require 'proto_v2'`
- 状态：⚠️ 无源码；dump 值 = `true`

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | proto 的 v2 演进版协议注册（与 proto 分工不明，可能按物编版本分流）；无任何可见导出 | 【语义推测】 |

### `@smallcard_inventory/require_libs`

- 来源：smallcard_inventory 包；状态：✅ 有源码（`ui\script\require_libs.lua`）；dump 值 = `true`
- 内容：`defaultui = require"@defaultui.main".defaultui`——挂载默认 UI 库全局。【源码实锤】

### `@smallcard_inventory/score_save`

- 来源：smallcard_inventory 包（背包云端存档，**不下发编辑器**）
- 加载：被 `src/main.lua` `require 'score_save'`（仅服务端入口引用）
- 状态：⚠️ 无源码；dump 值 = `true`

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| （值 `true`） | — | 背包数据云端存档模块；底层通道应为 `base.s.*`（score/list 系列，39 函数 dump 实锤）；与 game/item 的 `save_score_to_unit`/`load_score_to_unit`（反查形态）功能呼应 | 【语义推测】 |

**待实测**：背包存档的 score key 命名、读写时机（下线保存/定时保存）、与 `@common/base/tds_score` 的通道关系（见 FINDINGS-game-smallcard.md）。

### `@smallcard_inventory/trigger_module_main_1`

- 来源：smallcard_inventory 包；状态：✅ 有源码（`ui\script\trigger_module_main_1.lua`，客户端）；dump 值 = `true`
- 行为：TSTL 类全局声明 + `smallcard_inventory` 命名空间注册 2 个空函数桩 + 4 个类：

| 成员 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `smallcard_inventory.open_right_inv()` | `()`（`@noSelf`，空体） | 「打开右侧背包」触发器桩 | 【源码实锤】 |
| `smallcard_inventory.open_left_inv()` | `()`（`@noSelf`，空体） | 「打开左侧背包」触发器桩 | 【源码实锤】 |
| `smallcard_inventory.背包界面关闭时`（类） | 继承 `TriggerEvent`；`____constructor(self, obj)`，`event_name='背包界面-关闭'`，`autoForward=false` | 自定义事件类 | 【源码实锤】 |
| `smallcard_inventory.trigger_class`（类） | `____constructor(self)`（`pick_list_page=nil`） | 「通用背包库」数据类 | 【源码实锤】 |
| `smallcard_inventory.物品提示信息框显示`（类） | 继承 `TriggerEvent`；`____constructor(self, obj, ui, item)`，`event_name='物品提示信息框-显示'` | 自定义事件类（tips 注明：点击多功能格子打开时 item 可能为空） | 【源码实锤】 |
| `smallcard_inventory.物品提示信息框隐藏`（类） | 继承 `TriggerEvent`；`____constructor(self, obj)`，`event_name='物品提示信息框-隐藏'` | 自定义事件类 | 【源码实锤】 |

### `@smallcard_inventory/trigger_validator`

- 来源：smallcard_inventory 包；状态：✅ 有源码（`ui\script\trigger_validator.lua`）；dump 值 = `true`
- 行为：与 get_items 版同构——TSTL 类全局 + `validator or {}` + 空 `init_validator_0(nil)`。【源码实锤】

### `@smallcard_inventory/关闭玩家背包`

- 来源：smallcard_inventory 包；状态：✅ 有源码（`src\关闭玩家背包.lua`）；dump 值 = `true`（lua_plus 全局函数模块，函数挂全局不进返回值）
- 内容：`function 关闭玩家背包(player:player)`（`---@ui 关闭玩家~1~的背包界面` / `---@belong game`）→ `player:ui'__smallcard_inventory_close_inv'{}`。【源码实锤】

### `@smallcard_inventory/打开玩家背包`

- 来源：smallcard_inventory 包；状态：✅ 有源码（`src\打开玩家背包.lua`）；dump 值 = `true`
- 内容：`function 打开玩家背包(player:player)`（`---@ui 打开玩家~1~的背包界面` / `---@belong game`）→ `player:ui'__smallcard_inventory_open_inv'{}`。【源码实锤】

---

## smallcard_mail 包（v75，邮件库）

### `@smallcard_mail/mail`

- 来源：smallcard_mail 包（邮件数据类，**不下发编辑器**）
- 状态：⚠️ 无源码；dump 值 = table（9 键）——邮件记录模板 + 构造函数

**字段/子表**

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `id` | scalar `''` | 邮件 id | 【dump 实锤】 |
| `title` | scalar `''` | 标题 | 【dump 实锤】 |
| `content` | scalar `''` | 正文 | 【dump 实锤】 |
| `sender` | scalar `''` | 发件人 | 【dump 实锤】 |
| `receiver` | scalar `''` | 收件人 | 【dump 实锤】 |
| `awards` | table（0 键） | 附件奖励列表 | 【dump 实锤】 |
| `send_time` | scalar `''` | 发送时间 | 【dump 实锤】 |
| `end_time` | scalar `''` | 过期时间 | 【dump 实锤】 |

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `new` | 未知 | 构造一封邮件记录（默认字段即上表） | 【dump 实锤】（名）【语义推测】 |

### `@smallcard_mail/main`

- 来源：smallcard_mail 包 v75
- 加载：包服务端入口（`require '@common'` → `require 'module'` → `module:open()`）
- 状态：✅ 有源码（`smallcard_mail\75\src\main.lua`）；dump 值 = table（1 键 `smallcard_mail` 空命名空间）——与源码无 `____module`/`____return` 导出一致【dump 实锤】

**源码侧行为**：注册 7 个客户端→服务端协议处理器（`base.ui.proto.*`）【源码实锤】：

| 协议（消息名） | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `receive_all_mail(player)` | `(player)` | 一键领取全部：按 `poi.not_get_items_v2` 分流 `module:receive_mail(player)` / `module:receive_mail_v2(player)` | 【源码实锤】 |
| `receive_mail(player, data)` | `(player, data)` | 领取指定邮件：`data.custom_id_list or {}` → `module:receive_mail(_v2)` | 【源码实锤】 |
| `read_mail(player, data)` | `(player, data)` | 标记已读 → `module:read_mail(player, custom_id_list)` | 【源码实锤】 |
| `smallcard_mail_get_mail_list(player)` | `(player)` | 拉取邮件列表 → `module:get_mail_list(player)` | 【源码实锤】 |
| `send_official_awards(player, rewards_str_list)` | — | **待废弃**（空函数） | 【源码实锤】 |
| `get_official_awards(player)` | — | 空函数 | 【源码实锤】 |
| `delete_official_awards(player, id)` | — | 空函数 | 【源码实锤】 |

客户端发送点（ui/script 各皮肤 mail_dialog.lua）：`receive_mail{...}`/`receive_all_mail{}`/`read_mail{...}`/`smallcard_mail_get_mail_list{}`【源码实锤】；服务端下行：`smallcard_mail_get_mail_list` / `smallcard_mail_get_mail_part_list`（客户端 `base.proto.*` 注册点，ui/script/main.lua L73/77）【源码实锤】。

### `@smallcard_mail/module`

- 来源：smallcard_mail 包（邮件服务端核心，**不下发编辑器**）
- 加载：被 `src/main.lua` `require 'module'` 并冒号调用
- 状态：⚠️ 无源码；dump 值 = table（**13 键：9 函数 + 1 标量 + 3 空表**）

**函数**

| 函数 | 签名/形态 | 说明 | 置信 |
| --- | --- | --- | --- |
| `open` | `(self)` | 模块初始化 | 【dump 实锤】+【源码实锤】（调用点） |
| `receive_mail` | `(self, player[, custom_id_list])` | 领取邮件附件（旧版） | 【dump 实锤】+【源码实锤】（调用点） |
| `receive_mail_v2` | `(self, player[, custom_id_list])` | 领取（v2，get_items v2 通道） | 【dump 实锤】+【源码实锤】（调用点） |
| `read_mail` | `(self, player, custom_id_list)` | 标记已读 | 【dump 实锤】+【源码实锤】（调用点） |
| `get_mail_list` | `(self, player)` | 拉取并下行邮件列表 | 【dump 实锤】+【源码实锤】（调用点） |
| `fetch_mails` | 未知 | 从云端拉邮件（内部） | 【dump 实锤】（名）【语义推测】 |
| `decode_attachment` | 未知 | 附件解码 | 【dump 实锤】（名）【语义推测】 |
| `check_mail_red_point` | 未知 | 红点检查 | 【dump 实锤】（名）【语义推测】 |
| `mail_red_point_refresh` | 未知 | 红点刷新 | 【dump 实锤】（名）【语义推测】 |

**字段/子表**

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `IsOpen` | scalar `true` | 模块开启标志 | 【dump 实锤】 |
| `player_mail_list` | table（0 键） | 玩家邮件列表缓存 | 【dump 实锤】 |
| `official_mail_list` | table（0 键） | 官方邮件列表 | 【dump 实锤】 |
| `player_new_player_time` | table（0 键） | 玩家"新手期"时间记录（疑用于新手邮件资格） | 【dump 实锤】（名）【语义推测】 |

### `@smallcard_mail/trigger_module_main_1`

- 来源：smallcard_mail 包；状态：✅ 有源码（`ui\script\trigger_module_main_1.lua`，客户端）；dump 值 = `true`
- 行为：TSTL 类全局声明（此包多 `middle_game_key`/`UserID` 两类）+ `smallcard_mail` 命名空间注册 2 个自定义事件类：

| 类 | 继承/构造 | 说明 | 置信 |
| --- | --- | --- | --- |
| `smallcard_mail.mail_main_dialog_opened` | `TriggerEvent`；`____constructor(self, obj)`，`event_name='邮件界面被打开了'` | 邮件界面打开事件 | 【源码实锤】 |
| `smallcard_mail.mail_main_dialog_closed` | `TriggerEvent`；`____constructor(self, obj)`，`event_name='邮件界面被关闭了'` | 邮件界面关闭事件 | 【源码实锤】 |

### `@smallcard_mail/trigger_validator`

- 来源：smallcard_mail 包；状态：✅ 有源码（`ui\script\trigger_validator.lua`）；dump 值 = `true`
- 行为：TSTL 类全局 + `validator or {}` + 空 `init_validator_0(nil)` 骨架（与另两包同构）。【源码实锤】
