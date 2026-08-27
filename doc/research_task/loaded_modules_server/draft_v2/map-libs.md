# map-libs 分组：官方地图库服务端模块 API 参考（draft_v2）

> 范围：`@defaultui`(8) / `@global_default`(1) / `@lib_common_ai`(22) / `@lib_common_sounds`(1) / `@lib_control`(4) / `@lib_game_options`(4) = **42 键全覆盖**。
> 证据：服务端 StateGame `package.loaded` dump 值树（`test/loaded_modules_server/parsed/fields/`）+ api-13 解密源码（`D:\sce_open\api-13\2026_08_27\`，只读）+ 语料反查。
> 置信级：【实测】【dump 实锤】【源码实锤】【反查推测】【语义推测】（约定见 00-INDEX）。
>
> **分组共性结论（v2 新增，实锤）**：
> 1. 地图库 `src/`=服务端、`ui/`=客户端。本 dump 是**服务端**，因此 `ui/script` 来源的键（trigger_module_main_1 / trigger_validator / default_ui/* 控件等）dump 值几乎全为 `true`（已加载无表导出）——它们的真实 API 面在客户端。
> 2. 地图库入口的运行时返回模式（客户端 ui/script/main.lua 尾部实锤）：`ret = {["<包名>"] = <包名>全局表}` 再合并 `____module`/`____return`——这解释了 `@defaultui/main`、`@lib_common_ai/main`、`@lib_control/main` 的 dump 值为何都是 `{<包名> = ...}` 单键表。
> 3. **lib_common_ai 的 AI 实现未随编辑器侧包分发**：`src/main.lua` 仅 4 行（`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / `pcall(require 'ai_loader')`），ai/、class/、ai_templates/ 全部无源码。运行时证据链：dump 值树 + `_G.poi`（`poi.ai` = class/new、`poi.ai_enmity` = class/behavior/enmity、`poi.ai_state.*` = class/state/*，逐一对应，见 FINDINGS-map-libs）+ server_lua_plus 平行实现 + smallcard/lib_game_td 消费实例。
> 4. `@global_default/lua_declare` dump 值中的 Unit/Skill/Item/EffectParam/Timer 等大类是 **common 库共享类的内联展开**（shared_tables.json 实锤：与 `@common/base/unit|skill|item|eff_param|timer|position|lualib_bundle` 同内容哈希），方法全集归属 common-base 组文档，本节只列结构不重复抄录。

---

## 一、@defaultui（默认 UI 库，8 键）

### `@defaultui/main`

- 来源：defaultui 包 v63
- 加载：地图库服务端入口（`require '@defaultui.main'`）
- 状态：✅ 有源码（`defaultui\63\src\main.lua`）＋ dump 表值

源码全文【源码实锤】：

```lua
--- lua_plus ---
require 'require_libs'
require'@common.base'
require'default_ui'
```

源码无导出语句，但 dump 值为表——**运行时存在但源码未见**（服务端入口由装载器按 `ret={defaultui=...}` 模式组装，`NewActor` 类语料反查 0 命中，实现未随编辑器侧包分发）：

**字段/子表**

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `defaultui` | table(2) | 包命名空间 | 【dump 实锤】 |
| `defaultui.NewActor` | class | 表现封装类（无父类），见下 | 【dump 实锤】 |
| `defaultui.NewActor.id_count` | number(-1) | 实例自增 id 计数 | 【dump 实锤】 |
| `defaultui.NewActor.actor_map` | table(0) | 实例登记表（dump 时为空） | 【dump 实锤】 |
| `defaultui.create_NewActor_at` | function | 在指定位置创建 NewActor（签名未知） | 【dump 实锤】 |

**类：`NewActor`**（无 ____super；prototype 35 键，含 33 方法 + `constructor`/`__index` 循环引用）

| 方法 | 说明 | 置信 |
| --- | --- | --- |
| `____constructor` / `on_normal_init` / `do_subclass_action` | 构造 / 常规初始化 / 子类行为钩子 | 【dump 实锤】名，语义【语义推测】 |
| `play` / `stop` / `show` / `destroy` | 播放 / 停止 / 显示 / 摧毁 | 【dump 实锤】 |
| `anim_operation` / `anim_set_paused_all` / `sync_attribute` | 动画操作 / 全体动画暂停 / 属性同步 | 【dump 实锤】 |
| `attach_to` / `detach` / `get_visible_slots` | 挂接 / 脱挂 / 可见挂点查询 | 【dump 实锤】 |
| `set_position` / `set_position_from` / `set_ground_z` / `set_launch_position` / `set_launch_ground_z` / `set_launch_site` / `set_impact_site` / `set_bearings` | 位置/发射点/着弹点/朝向族 | 【dump 实锤】 |
| `set_rotation` / `set_facing` | 旋转 / 朝向 | 【dump 实锤】 |
| `set_scale` / `set_scale_xyz` | 缩放 | 【dump 实锤】 |
| `set_asset` / `set_text` / `set_volume` / `set_shadow` / `set_owner` / `set_time_scale_global` | 资源/文字/音量/阴影/归属/时缩 | 【dump 实锤】 |
| `set_grid_state` / `set_grid_range` / `set_grid_size` | 格子状态/范围/尺寸 | 【dump 实锤】 |

> 对照：lua_plus 有 `base.actor_*` 扁平封装全集（源码实锤，见 lua-plus 组文档），NewActor 应是其上的 OO 封装【语义推测】。

### `@defaultui/actor`

- 来源：defaultui 包（编辑器侧无此文件）
- 状态：⚠️ 无源码；dump 值 = `true`（**已加载无表导出**）
- 研判：`@defaultui/main` 值中的 `NewActor` 类语料反查 0 命中，本键（命名即 actor）最可能是其实现模块——纯副作用注册（往 `defaultui` 全局挂类），故无表导出【反查推测】。lua_plus `base.actor_*` 扁平 API 有源码（见 lua-plus 组）。

### `@defaultui/default_ui`

- 来源：defaultui 包（编辑器侧无 `src/` 对应文件；客户端聚合在 `ui/script/main.lua`）
- 状态：⚠️ 无源码（服务端）；dump 值 = `true`（已加载无表导出）
- 研判：默认控件命名空间。客户端入口 `ui/script/main.lua`【源码实锤】聚合 20 个组件模块到 `____return`：`error_info / move_joystick / panel / button / exit / input / label / radio / scroll_bar / table / window / skill / stop_cast / rank / target_info / progress / heart / grid / follow / minimap_camera_control`，另有 `default_ui.old/*`（require_folder 旧版 8 组件）。服务端 dump=true 说明服务端加载的是无导出变体【dump 实锤】。

### `@defaultui/default_ui/minimap_camera_control`

- 来源：defaultui 包 v63
- 状态：✅ 有源码（`defaultui\63\ui\script\default_ui\minimap_camera_control.lua`，**客户端组件**）；dump 值 = `true`（服务端已加载无表导出）
- 加载：`base.ui.component('minimap_camera_control')` 注册的 UI 组件，`return mt`

**函数**（全部【源码实锤】）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:define` | `()` | 定义 props（`z_index` 默认 10、`layout`）与 template（panel + 框图子控件） |
| `mt:init` | `()` | 绑定 on_mouse_down（锁定并 35ms 循环跟随鼠标移相机，发 `unlock_camera`）/ on_click（解锁还原相机，发 `lock_camera`） |
| `mt:on_remove` | `()` | 清理 timer |
| （内部）`split` | `(full_string, separator)` | 字符串切分工具 |
| （内部）`set_camera_board` | `(self, w, h, offset_x, offset_y)` | 更新小地图相机框位置 |
| （内部）`update_camera_position` | `(self, x, y, w, h)` | 按鼠标比例换算 `game.set_camera` 目标点 |

### `@defaultui/default_ui/move_joystick`

- 来源：defaultui 包 v63
- 状态：✅ 有源码（`defaultui\63\ui\script\default_ui\move_joystick.lua`，32KB **客户端组件**）；dump 值 = `true`（服务端已加载无表导出）
- 加载：`base.ui.component('move_joystick')` 注册，`return mt`

**组件 props**（源码头注释【源码实锤】）：`show / press_panel_grow_width(0.5) / press_panel_grow_height(1) / joystick_size(256) / move_offset{256,256} / move_region / switch_joystick(活动|固定|跟随|原生摇杆) / move_type(普通|十字) / background_image / slider_image / slider_image_center{常,按} / slider_active_percent / stop_percent / slow_percent(0.5) / slow_rate(0.35) / active / on_click_show`

**事件 bind**：`joystick_press / joystick_release / joystick_move_start / joystick_move / joystick_move_end`，回调形态 `(x, y, percent)`【源码实锤】

**函数**（摘要，【源码实锤】）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:define` / `mt:init` / `mt:on_remove` | `()` | 组件生命周期 |
| `mt:on_move` | `(x, y)` | 移动处理 |
| `mt:on_change_state` / `mt:change_move_state` / `mt:check_move_state` | `(state)` / `(x,y,state)` / `(x,y,percent)` | 移动状态机 |
| `mt:register_keyboard_control_event` / `mt:keyboard_control_update` / `mt:register_joystick_control_event` / `mt:register_skill_control` | `()` | 键盘/摇杆/技能控制注册 |
| （内部）`get_angle_base_y_vector` | `(x, y)` | 以 y 轴为基准求角度 |
| （内部）`update_joystick_size` / `update_joystick_center` | `(self, size)` / `(x,y,percent,on_press)` | 尺寸/中心更新 |

### `@defaultui/require_libs`

- 状态：✅ 有源码（`ui\script\require_libs.lua`）；dump 值 = `true`
- 内容：仅一行注释 `---require libs---`，纯占位【源码实锤】。

### `@defaultui/trigger_module_main_1`

- 状态：✅ 有源码（`ui\script\trigger_module_main_1.lua`）；dump 值 = `true`
- 内容：TSTL 类声明桩（从 `base.tsc.CLASSES` 取或 `__TS__Class2` 兜底注册 RegionCircle/Point/Unit/…/Array 共 31 个全局类名）＋ 向 `defaultui` 命名空间注册：
  - `defaultui.ICustomAnimParams`（类，@name 自定义动画播放参数，空构造）【源码实锤】

### `@defaultui/trigger_validator`

- 状态：✅ 有源码（`ui\script\trigger_validator.lua`）；dump 值 = `true`
- 内容：同上类声明桩 + `validator` 全局命名空间初始化 + 空调用 `init_validator_0(nil)`（触编校验器占位，无实际逻辑）【源码实锤】。

---

## 二、@global_default（1 键）

### `@global_default/lua_declare`

- 来源：global_default 包 v60（映射源码 `ui\script\lua_declare.lua` 为**客户端声明桩**；`src/` 仅空 init.lua）
- 加载：`require '@global_default.lua_declare'`（各地图库 ui/script/main.lua 头部统一 require）
- 状态：🔀 声明/导出聚合模块；dump 值 = table（12 顶层键，730 函数路径，14 类，**7 处截断**）

**dump 值顶层键**【dump 实锤】：`UnitPropertyBonus / Cast / middle_game_key / UserID / Timer / UnitTable / Item / ScreenPos / EffectParam / TriggerEvent / Unit / Skill`

**⚠️ 字段归属修正（重要）**：dump 值的 730 个函数路径绝大多数是 **common 库共享类内联**——`shared_tables.json` 实锤 Unit/Skill/Item/EffectParam/Timer/ScreenPos 等与 `@common/base/{unit,skill,item,eff_param,timer,position}` 及 `_G` 内容哈希相同。这些类的方法全集归 common-base 组文档，本模块的语义是"把引擎/公共类导出给触编声明层"。

**类结构**（【dump 实锤】；方法数 = dump 值树统计）

| 类 | 继承 | prototype 键数 | 说明 |
| --- | --- | --- | --- |
| `Unit` | extends `Target` | 386 | 单位类（共享自 common，方法全集见 common-base 组） |
| `Cast` | extends `Skill` | 4 | 施法类（构造 + __index/__newindex） |
| `Skill` | — | 97 | 技能类（共享） |
| `Item` | — | 73 | 物品类（共享；`_descriptors.inv_index` 的 get/enumerable/configurable 3 处**被截断，字段不全**） |
| `EffectParam` | — | 74 | 效果参数类（共享） |
| `Timer` | — | 15 | 计时器类（共享）。方法【dump 实锤】：`____constructor __tostring get_current get_current_time get_remaining_time get_remaining_time_new pause remove restart resume set_current_time set_remaining_time` |
| `ScreenPos` | — | 8 | 屏幕坐标类（共享）。方法：`____constructor __tostring get_xy` |
| `TriggerEvent` | — | 3 | 触发器事件基类（仅构造） |
| `UnitPropertyBonus` / `middle_game_key` / `UserID` / `UnitTable` | — | 各 3 | 仅构造的轻量类。`UserID.from_number` 为顶层函数【dump 实锤】 |

**截断点**（字段不全）：`Cast.____super.prototype._descriptors.target_type`、`Item.prototype._descriptors.inv_index.{get,enumerable,configurable}`、`Skill.prototype._descriptors.target_type.{get,enumerable,configurable}`【dump 实锤】

**客户端声明桩源码贡献**（`ui/script/lua_declare.lua`，【源码实锤】）：`____exports = {TriggerEvent, Target, Unit, base}`；`Unit extends Target`；及 **60+ 触编事件类**构造签名（`单位进入视野(self,obj,evt_name,unit)`、`玩家断线(self,obj,evt_name,player)`、`技能冷却激活(self,obj,evt_name,skill,time_remaining,time_total)`、`游戏更新(self,obj,evt_name,delta)`、`对话选择(…,conversation_choice_item_link)`、`建造预放置开始/确认/取消(…,owner,skill,spellbuild_unit_actor)` 等，完整清单见 draft_v1），这些事件类是触编事件参数的类型声明载体。

---

## 三、@lib_common_ai（通用 AI 库，22 键）

> **组级前提**【源码实锤】：`lib_common_ai\43\src\main.lua` 全文仅 4 行：
> ```lua
> require_folder 'ai'
> require 'utility'
> require_folder 'ai_templates'
> pcall(function() require 'ai_loader' end)
> ```
> ai/、class/、ai_templates/、utility、customscript **全部未随编辑器侧包分发**（运行时来自客户端变体/引擎），以下以 dump 值树为主证据。
> **运行时对照**：`_G.poi.ai` ≡ `@lib_common_ai/class/new`（9 函数全同）、`_G.poi.ai_enmity` ≡ `class/behavior/enmity`（3 函数全同）、`_G.poi.ai_state.{back,move,attack,none,pursue}` 对应 `class/state/*`（各含 `on_enter/on_leave/priority`）【dump 实锤】——全局 `poi.*` 是这套 AI 类设施的运行时访问点。
> **消费范式**（语料【反查推测】→实为源码直见）：`local mt = base.ai['模板名']`、`mt.pulse = 200`、`mt:add_behavior(fn)`、`mt:on_idle()`、`mt:on_add()`（smallcard_create_hero/src/瞎溜达.lua、lib_game_td/src/ai/tower_ai.lua 实例）；`base.unit_add_ai(unit, '自定义AI', {path=…, cycle=…})`（server_lua_plus ai.lua）。

### `@lib_common_ai/main`

- 来源：lib_common_ai 包 v43
- 状态：✅ 有源码（src/main.lua，仅上述 4 行）＋ dump 表值（**5 处截断**）
- dump 值 = `{lib_common_ai = <命名空间 9 键>}`【dump 实锤】

**字段/子表**

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `lib_common_ai.AI` | class | AI 基类（无父类；亦内联于 `_G`），见下 | 【dump 实锤】 |
| `lib_common_ai.ai_trig` | table(4) | AI 触发器封装：`combine_args=true`、`callback`、`callback_sync`、`events[1]`（`events[0].event_name/obj` **被截断，字段不全**） | 【dump 实锤】 |
| `lib_common_ai.line_with_offset` | function | 路线整体偏移（与 server_lua_plus 同名内部函数同形：`(line, offset_x, offset_y) -> Point[]`，逐点 clamp≥50） | 【dump 实锤】名；签名【反查推测】 |
| `lib_common_ai.unit_ai_move_to` | function | 令单位沿路线行动；lua_plus 平行实现签名 `(unit, line, cycle)`，内部 `unit_add_ai(unit,'仅移动AI',{path,cycle})` | 【dump 实锤】名；签名/语义【反查推测】 |
| `lib_common_ai.unit_ai_attack_move_to` | function | 令单位沿路线进攻；同上 `(unit, line, cycle)`，挂 `'自定义AI'` | 【dump 实锤】名；【反查推测】 |
| `lib_common_ai.unit_group_ai_move_to` | function | 单位组沿路线行动并保持队形 `(unit_group, line, cycle)`（按组中心逐单位偏移路线，偏移<1500 才生效） | 【dump 实锤】名；【反查推测】 |
| `lib_common_ai.unit_group_ai_attack_move_to` | function | 单位组沿路线进攻保持队形 `(unit_group, line, cycle)` | 【dump 实锤】名；【反查推测】 |
| `lib_common_ai.unit_pet_ai_follow` | function | 宠物跟随 AI（签名未知） | 【dump 实锤】名；【语义推测】 |
| `lib_common_ai.unit_pet_ai_attack_follow` | function | 宠物攻击跟随 AI（签名未知） | 【dump 实锤】名；【语义推测】 |

**类：`AI`**（无 ____super；prototype 35 键 = 33 方法 + __index/__newindex；另有类级表 `cache_aisearcher / cache_behavior / cache_filter / default_target_filter` 各 6 键）

| 方法 | 说明 | 置信 |
| --- | --- | --- |
| `____constructor` / `register` / `unregister` / `remove` | 构造与注册生命周期 | 【dump 实锤】 |
| `enable` / `disable` / `is_enabled` / `execute` / `on_tick` | 启停/tick 执行 | 【dump 实锤】 |
| `on_add` / `on_remove` / `on_death` / `on_revive` / `on_provoke` | 挂载/移除/死亡/复活/被挑衅钩子（消费实例中 `mt:on_add` 常见） | 【dump 实锤】名；语义【反查推测】 |
| `cast_skill` / `smart_cast` / `try_cast_skill` / `can_cast` / `is_casting` / `stop_acquired_cast` / `get_random_skill` | 施法决策族 | 【dump 实锤】 |
| `single_damage_skill` / `multi_damage_skill` / `single_healing_skill` / `multi_healing_skill` / `single_buff_skill` / `single_debuff_skill` | 单体/群体 × 伤害/治疗/buff/debuff 技能执行模板 | 【dump 实锤】名；语义【语义推测】 |
| `search_healing_target` / `search_buff_target` / `searcher_execute` / `try_approach` | 目标搜索/接近 | 【dump 实锤】名；语义【语义推测】 |
| `_descriptors.default_attack / attack_range / search_radius` | 属性描述符（**3 处被截断，字段不全**） | 【dump 实锤】 |

### `@lib_common_ai/ai/ai_common`

- 状态：⚠️ 无源码；dump 值 = table(3)【dump 实锤】
- 函数：`is_casting` / `get_random_skill` / `try_cast_skill`——AI 施法公共helper（与 AI 类同名方法对应，推测为其实现体或独立工具）【dump 实锤】名；【语义推测】

### `@lib_common_ai/ai/default_ai`

- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）
- 研判：默认 AI 行为实现，副作用注册（挂 `base.ai` 模板或注册行为）【语义推测】。

### `@lib_common_ai/class/new`

- 状态：⚠️ 无源码；dump 值 = table(9)【dump 实锤】；≡ `_G.poi.ai`
- 函数：`new / remove / reset / on_tick / change_state / get_path / find_enemy / attack_skill / provoke`——旧版（class 系）AI 状态机核心：创建/移除/重置/tick/切状态/寻路/索敌/攻击技能/挑衅【dump 实锤】名；语义【语义推测】

### `@lib_common_ai/class/init`

- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）
- 研判：class 系设施装配入口（把 state/behavior 模块组进 `poi.ai_state.*`（含 priority 标量）等运行时结构）【语义推测】。

### `@lib_common_ai/class/behavior/enmity`

- 状态：⚠️ 无源码；dump 值 = table(3)【dump 实锤】；≡ `_G.poi.ai_enmity`
- 函数：`start_enmity / reset_enmity / remove_enmity`——仇恨行为：建立/重置/移除仇恨【dump 实锤】名；语义【语义推测】

### `@lib_common_ai/class/state/attack`

- 状态：⚠️ 无源码；dump 值 = table(3)【dump 实锤】
- 函数：`on_enter_attack / on_leave_attack / on_triggle_attack`（triggle 为官方拼写）——攻击状态进入/离开/触发钩子。对照 `_G.poi.ai_state.attack` = `{on_enter, on_leave, priority}`【dump 实锤】

### `@lib_common_ai/class/state/back`

- 状态：⚠️ 无源码；dump 值 = table(2)【dump 实锤】
- 函数：`on_enter_back / on_leave_back`——回位（回出生点）状态钩子【dump 实锤】名；语义【语义推测】

### `@lib_common_ai/class/state/move`

- 状态：⚠️ 无源码；dump 值 = table(2)【dump 实锤】
- 函数：`on_enter_move / on_leave_move`——移动状态钩子【dump 实锤】

### `@lib_common_ai/class/state/none`

- 状态：⚠️ 无源码；dump 值 = table(2)【dump 实锤】
- 函数：`on_enter_none / on_leave_none`——空闲（无状态）钩子【dump 实锤】

### `@lib_common_ai/class/state/pursue`

- 状态：⚠️ 无源码；dump 值 = table(2)【dump 实锤】
- 函数：`on_enter_pursue / on_leave_pursue`——追击状态钩子【dump 实锤】

### `@lib_common_ai/ai_templates/*`（9 键，同一形态，合并描述）

`@lib_common_ai/ai_templates/{default_ai, 主动召唤物, 仅移动ai, 召唤物, 攻城车, 自定义ai, 通用ai, 野怪, 镖车}`

- 状态：⚠️ 全部无源码；dump 值均 = `true`（已加载无表导出）
- 研判：内置 AI 模板注册模块，由 `require_folder 'ai_templates'` 载入，副作用是把模板挂进 `base.ai['<名称>']` 注册表（消费实例 `base.ai['瞎溜达']`/`base.ai['默认TD_ai']`/`base.ai['summon_ai']` 见 smallcard_create_hero / lib_game_td 源码）【反查推测】；模板字段形态：`pulse`（tick 间隔 ms）、`on_add/on_idle/add_behavior` 等【反查推测】。lua_plus `base.unit_ai_move_to` 等按名引用 `'仅移动AI'`/`'自定义AI'` 两模板【源码实锤（lua_plus 侧）】。

### `@lib_common_ai/customscript`

- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）
- 研判：触编自定义脚本承载模块（各地图库均有同名键，占位/声明性质）【语义推测】。

### `@lib_common_ai/utility`

- 状态：⚠️ 无源码；dump 值 = `true`（已加载无表导出）
- 研判：AI 库内部工具模块（main.lua 第 2 行 `require 'utility'`）【源码实锤】其被加载；内容【语义推测】。

### `@lib_common_ai/trigger_module_main_1`

- 状态：✅ 有源码（`ui\script\trigger_module_main_1.lua`）；dump 值 = `true`
- 内容：TSTL 类声明桩 + `lib_common_ai = lib_common_ai or {}` 命名空间初始化（无额外导出）【源码实锤】。

### `@lib_common_ai/trigger_validator`

- 状态：✅ 有源码（`ui\script\trigger_validator.lua`）；dump 值 = `true`
- 内容：类声明桩 + `validator` 命名空间 + 空 `init_validator_0`（与 defaultui 版同构）【源码实锤】。

---

## 四、@lib_common_sounds（1 键）

### `@lib_common_sounds/main`

- 来源：lib_common_sounds 包 v16
- 状态：✅ 有源码（`src\main.lua`）；dump 值 = `true`
- 内容：全文一行 `require '@common'`——纯依赖占位入口，音效资源本体不在 Lua 层【源码实锤】。

---

## 五、@lib_control（操控库，4 键）

### `@lib_control/main`

- 来源：lib_control 包 v46
- 状态：✅ 有源码（`src\main.lua`）＋ dump 表值
- dump 值 = `{lib_control = {}}`（**空命名空间表**）【dump 实锤】——服务端无函数导出。
- 服务端源码内容【源码实锤】：`require 'require_libs'` + `fail_code_enum`（施法失败码 0~26 中文文案表：0 成功 / 2 冷却中 / 7 超出施法范围 / 10 目标非法 / 12 被lua阻止 / 17/18 客户端禁/隐技能 / 20~26 目标合法性…）+ 事件 handler：
  - `base.game:event('技能-施法失败', fn(_, unit, cast, failed_code))`：非 12 且有文案且 `cast.is_user` 时 `unit:error_info(fail_code_enum[failed_code])`（头顶错误提示）
- 客户端侧（`ui\script\main.lua`，服务端不加载，供对照）【源码实锤】：导出 `get_lib_control_main_page / ctrl_skill_set_skill / main_page_ui`。

### `@lib_control/require_libs`

- 状态：✅ 有源码（`ui\script\require_libs.lua`）；dump 值 = `true`
- 内容：`defaultui = require"@defaultui.main".defaultui`——把 defaultui 命名空间引入全局【源码实锤】。

### `@lib_control/trigger_module_main_1`

- 状态：✅ 有源码（`ui\script\trigger_module_main_1.lua`）；dump 值 = `true`

**函数**

| 函数 | 签名 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- |
| `lib_control.get_joystick_move_angle` | `(____, 摇杆)` | `base.gui_get_prop(ctrl, "move_angle")` | 获得摇杆移动时的旋转角度（触编动作，客户端） | 【源码实锤】 |

### `@lib_control/trigger_validator`

- 状态：✅ 有源码（`ui\script\trigger_validator.lua`）；dump 值 = `true`
- 内容：类声明桩 + `validator` 命名空间 + 空 `init_validator_0`【源码实锤】。

---

## 六、@lib_game_options（游戏选项库：礼包码/改名/用户信息，4 键）

> 组特点：src/ 三模块是**真实服务端业务逻辑**（云数据 + Redis 消息通道 + ui.proto 协议handler），dump 值均为 `true`（模块 return nil，功能全挂在 `base.ui.proto.*` / 事件上）【源码实锤 + dump 实锤】。

### `@lib_game_options/main`

- 状态：✅ 有源码（`src\main.lua`）；dump 值 = `true`
- 内容【源码实锤】：
  ```lua
  require '@common'
  require 'gift_code'
  require 'user_info'
  require 'rename'
  base.backend.init_game_config()
  ```
  ——入口即装配：加载三子模块并初始化后端游戏配置（`base.backend.init_game_config` 是 backend 模块实锤函数，见 common-base-game 组）。

### `@lib_game_options/gift_code`

- 状态：✅ 有源码（`src\gift_code.lua`）；dump 值 = `true`
- 功能：礼包码兑换（c2s）。

**函数/handler**（全部【源码实锤】）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.ui.proto.RequestGiftCode` | `(player, data)` | c2s 兑换入口。data={`code_text, game_map, tag`}；`base.s.publish_message('Redis.Host2Server.Channel.ExchangeCode', {userId, exchangeCode, gameId, sessionId, channel, tag})`；`base.wait(3000)` 超时兜底回 `result=false, code=9` |
| （内部）`subscribe` | `(game_name, player)` | 按 `Redis.Server2Host.Channel.ExchangeCode.<game>_<userId>` 订阅回执（`base.s.subscribe_message`，幂等）；收到后 `player:ui 'RequestGiftCodeResult' {result, code}` + `base.game:event_notify('GiftCodeResult', {player, result, code})` |
| 回调查 | `ok(result)` / `error(error_code, error_desc)` / `timeout()` | 订阅三态回调 |

### `@lib_game_options/rename`

- 状态：✅ 有源码（`src\rename.lua`）；dump 值 = `true`
- 功能：昵称检测与修改（30 天冷却，`interval_time = 30*24*60*60`）。

**函数/handler**（全部【源码实锤】）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| （内部）`check_name` | `(db, name, callback)` | 先 `base.detection.check_text` 敏感词（易盾），再 `base.s.name_exist(db,'nick',name)` 查重；callback 码：0 可用 / 1 敏感字符 / 2 易盾挂了 / 3 重名 / 4 查询超时 / 5 未达到修改时间 |
| `base.ui.proto.C2S_app_check_name` | `(player, data)` | 名称检测（data.db 默认 `'app_android'`），回 `S2C_app_check_name {error_code, nick}` |
| `base.ui.proto.C2S_app_rename` | `(player, data)` | 请求改名：`base.s.score_init(db, player, cbs, 'nick','rename_time')` 读当前昵称与上次改名时间 → 过冷却且检测通过 → `base.s.get_commit('修改昵称',db)` 事务（`score_sets / score_seti(rename_time) / name_delete / name_new`）→ commit 成功回 `S2C_app_rename` + `S2C_async_nick` + `base.game:event_notify('玩家-昵称改变', player, nick)` |

### `@lib_game_options/user_info`

- 状态：✅ 有源码（`src\user_info.lua`）；dump 值 = `true`
- 功能：连入/重连时同步会话信息与昵称。

**函数/handler**（全部【源码实锤】）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| （内部）`sync_info` | `(player)` | `player:ui '__sync_game_info' {session_id = tostring(base.game.get_session_id())}` |
| （内部）`sync_nick` | `(player)` | `base.s.score_init('app_android', player, cbs, 'nick')` 读昵称 → 写 `player._nick` → `player:ui 'S2C_async_nick' {nick}` |
| 事件 | `玩家-连入` / `玩家-重连` | 均调 sync_nick + sync_info |

---

## 附：本组统计

- 42 键：dump 表值 12（含截断 2：`@global_default/lua_declare`、`@lib_common_ai/main`），`true` 值 30。
- 有源码 19 / 无源码 23（其中 lib_common_ai 占 20）。
- 服务端真实业务逻辑集中在 lib_game_options（礼包码/改名/昵称，走 `base.s.*` 云数据 + Redis 消息 + `base.ui.proto`）；defaultui/lib_control 的 API 面基本在客户端，服务端 dump 多为 `true` 占位。
