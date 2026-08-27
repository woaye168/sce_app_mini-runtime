# 官方地图库（defaultui / global_default / lib_common_ai / lib_control / lib_game_options / lib_common_sounds）

模块数：42。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）。

源码覆盖：19/42；其余标注 ⚠️ 无源码并附调用点反查/语义推测。

---

### `@defaultui/actor`

- 归属：defaultui 包（未找到源码）
- 研判：defaultui 的 UI 侧 actor 封装，未随包分发（推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.actor_anim_play(actor, anim, time, time_type, start_offset, blend_time, priority)`
  - `base.actor_anim_play(actor:actor, anim:string, time:number, time_type:integer, start_offset:number, blend_time:integer, priority:integer)`
  - `base.actor_anim_play_bracket(actor, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority)`
  - `base.actor_anim_play_bracket(actor, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority, sync)`
  - `base.actor_anim_play_bracket(actor:actor, anim_birth:string, anim_stand:string, anim_death:string, force_one_shot:boolean, kill_on_finish:boolean, priority:integer)`
  - `base.actor_anim_play_bracket(actor:actor, anim_birth:string, anim_stand:string, anim_death:string, force_one_shot:boolean, kill_on_finish:boolean, priority:integer, sync:boolean)`
  - `base.actor_anim_set_paused_all(actor, paused)`
  - `base.actor_anim_set_paused_all(actor:actor, paused:boolean)`
  - `base.actor_attach_to_actor(actor, host, socket)`
  - `base.actor_attach_to_actor(actor:actor, host:actor, socket:string)`
  - `base.actor_attach_to_unit(actor, host, socket)`
  - `base.actor_attach_to_unit(actor:actor, host:unit, socket:string)`
  - `base.actor_destroy(actor, flag)`
  - `base.actor_destroy(actor:actor, flag:表现摧毁方式)`
  - `base.actor_enable_raycast(actor, enable)`
  - `base.actor_from_id(id)`
  - `base.actor_from_sid(id)`
  - `base.actor_get_parent(obj)`
  - `base.actor_info()`
  - `base.actor_pause(actor)`
  - `base.actor_pause(actor:actor)`
  - `base.actor_play(actor)`
  - `base.actor_play(actor:actor)`
  - `base.actor_resume(actor)`
  - `base.actor_resume(actor:actor)`
  - `base.actor_set_anim_mapping(obj, name_from, name_to)`
  - `base.actor_set_anim_mapping_map(obj, name_map)`
  - `base.actor_set_asset_model(actor, asset)`
  - `base.actor_set_asset_model(actor:actor, asset:model_id)`
  - `base.actor_set_asset_sound(actor, asset)`
  - `base.actor_set_asset_sound(actor:actor, asset:sound_id)`
  - `base.actor_set_facting(actor, angle)`
  - `base.actor_set_facting(actor:actor, angle:angle)`
  - `base.actor_set_grid_range(actor, start_x, start_y, range_x, range_y)`
  - `base.actor_set_grid_range(actor:actor, start_x:integer, start_y:integer, range_x:integer, range_y:integer)`
  - `base.actor_set_grid_size(actor, size_x, size_y)`
  - `base.actor_set_grid_size(actor:actor, size_x:number, size_y:number)`
  - `base.actor_set_grid_state(actor, id_x, id_y, state)`
  - `base.actor_set_grid_state(actor:actor, id_x:integer, id_y:integer, state:integer)`
  - `base.actor_set_grount_height(actor, height)`
  - `base.actor_set_grount_height(actor:actor, height:number)`
  - `base.actor_set_owner(actor, owner)`
  - `base.actor_set_owner(actor:actor, owner:number)`
  - `base.actor_set_position(actor, point)`
  - `base.actor_set_position(actor:actor, point:point)`
  - `base.actor_set_scale(actor, scale)`
  - `base.actor_set_scale(actor:actor, scale:number)`
  - `base.actor_set_shadow(actor, enable)`
  - `base.actor_set_shadow(actor:actor, enable:是否)`
  - `base.actor_set_time_scale_global(actor, time_scale)`
  - `base.actor_set_time_scale_global(actor:actor, time_scale:number)`
  - `base.actor_set_volume(actor, volume)`
  - `base.actor_set_volume(actor:actor, volume:number)`
  - `base.actor_set_volume(actor:actor,volume:number)`
  - `base.actor_stop(actor)`
  - `base.actor_stop(actor:actor)`

### `@defaultui/default_ui`

- 归属：defaultui 包（未找到源码）
- 研判：目录命名空间模块：对应 ui/script/default_ui/ 下 31 个默认控件模块（button/label/panel/skill/rank 等），由 loader 聚合成命名空间。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@defaultui/default_ui/minimap_camera_control`

- 归属：defaultui 包
- 研判：目录命名空间模块：对应 ui/script/default_ui/ 下 31 个默认控件模块（button/label/panel/skill/rank 等），由 loader 聚合成命名空间。
- 状态：✅ 有源码（`defaultui\63\ui\script\default_ui\minimap_camera_control.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:define` | `()` |  |
| `setter` | `(value)` |  |
| `setter` | `(value)` |  |
| `split` | `(full_string, separator)` |  |
| `set_camera_board` | `(self , w , h , offset_x , offset_y)` |  |
| `update_camera_position` | `(self , x , y , w , h)` |  |
| `mt:init` | `()` |  |
| `self.bind.on_mouse_down` | `()` |  |
| `self.bind.on_click` | `()` |  |
| `mt:on_remove` | `()` |  |

### `@defaultui/default_ui/move_joystick`

- 归属：defaultui 包
- 研判：目录命名空间模块：对应 ui/script/default_ui/ 下 31 个默认控件模块（button/label/panel/skill/rank 等），由 loader 聚合成命名空间。
- 状态：✅ 有源码（`defaultui\63\ui\script\default_ui\move_joystick.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get_angle_base_y_vector` | `(x , y)` |  |
| `func` | `(x , y)` |  |
| `func` | `(x , y)` |  |
| `func` | `(self)` |  |
| `func` | `(self)` |  |
| `func` | `(self)` |  |
| `func` | `(self)` |  |
| `update_joystick_size` | `(self , joystick_size)` |  |
| `mt:define` | `()` |  |
| `setter` | `(value)` |  |
| `setter` | `(value)` |  |
| `setter` | `(value)` |  |
| `setter` | `(value)` |  |
| `setter` | `(value)` |  |
| `setter` | `(is_show)` |  |
| `setter` | `(value)` |  |
| `setter` | `(show)` |  |
| `setter` | `(move_offset)` |  |
| `setter` | `(move_region)` |  |
| `setter` | `(type)` |  |
| `setter` | `(type)` |  |
| `setter` | `(image_path)` |  |
| `setter` | `(image_path)` |  |
| `setter` | `(data)` |  |
| `setter` | `(percent)` |  |
| `setter` | `(percent)` |  |
| `setter` | `(percent)` |  |
| `setter` | `(rate)` |  |
| `setter` | `(active)` |  |
| `setter` | `(value)` |  |
| `default` | `(x, y, percent)` |  |
| `setter` | `(func)` |  |
| `self.props.joystick_press` | `(x, y, percent)` |  |
| `default` | `(x, y, percent)` |  |
| `setter` | `(func)` |  |
| `self.props.joystick_release` | `(x, y, percent)` |  |
| `default` | `(x, y, percent)` |  |
| `setter` | `(func)` |  |
| `self.props.joystick_move_start` | `(x, y, percent)` |  |
| `default` | `(x, y, percent)` |  |
| `setter` | `(func)` |  |
| `self.props.joystick_move` | `(x, y, percent)` |  |
| `default` | `(x, y, percent)` |  |
| `setter` | `(func)` |  |
| `self.props.joystick_move_end` | `(x, y, percent)` |  |
| `mt:on_move` | `(x , y)` | 移动处理 |
| `mt:on_change_state` | `(state)` | 执行状态切换 |
| `mt:change_move_state` | `(x , y , state)` | 切换移动状态 |
| `mt:check_move_state` | `(x , y , percent)` | 检查移动状态 |
| `mt:init` | `()` |  |
| `update_joystick_center` | `(x , y , percent , on_press)` |  |
| `self.bind.on_vj_press` | `(x , y , percent)` |  |
| `self.bind.on_vj_release` | `(x , y , percent)` |  |
| `self.bind.on_vj_move_start` | `(x , y , percent)` |  |
| `self.bind.on_vj_move` | `(x , y , percent)` |  |
| `self.bind.on_vj_move_end` | `(x , y , percent)` |  |
| `mt:on_remove` | `()` |  |
| `mt:register_keyboard_control_event` | `()` |  |
| `mt:keyboard_control_update` | `()` |  |
| `mt:register_joystick_control_event` | `()` |  |
| `mt:register_skill_control` | `()` |  |

### `@defaultui/main`

- 归属：defaultui 包
- 状态：✅ 有源码（`defaultui\63\src\main.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@defaultui/require_libs`

- 归属：defaultui 包
- 状态：✅ 有源码（`defaultui\63\ui\script\require_libs.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@defaultui/trigger_module_main_1`

- 归属：defaultui 包
- 状态：✅ 有源码（`defaultui\63\ui\script\trigger_module_main_1.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `ICustomAnimParams.prototype.____constructor` | `(self)` |  |

### `@defaultui/trigger_validator`

- 归属：defaultui 包
- 状态：✅ 有源码（`defaultui\63\ui\script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `init_validator_0` | `(self)` |  |

### `@global_default/lua_declare`

- 归属：global_default 包
- 状态：✅ 有源码（`global_default\60\ui\script\lua_declare.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `TriggerEvent.prototype.____constructor` | `(self)` |  |
| `Target.prototype.____constructor` | `(self)` |  |
| `Unit.prototype.____constructor` | `(self)` |  |
| `self.play_anim_ex` | `(____, 动画名, 参数)` |  |
| `base.set_spellbuild_spin_clockwise` | `()` |  |
| `单位进入视野.prototype.____constructor` | `(self, obj, evt_name, unit)` |  |
| `单位离开视野.prototype.____constructor` | `(self, obj, evt_name, unit)` |  |
| `单位选中.prototype.____constructor` | `(self, obj, evt_name, player, unit)` |  |
| `单位取消选中.prototype.____constructor` | `(self, obj, evt_name, player, unit)` |  |
| `单位属性变化.prototype.____constructor` | `(self, obj, evt_name, unit, property, value_n, value_s)` |  |
| `单位施法开始.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `单位施法引导.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `单位施法出手.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `单位施法完成.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `单位施法停止.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `单位获得状态.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `单位失去状态.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `单位状态层数变化.prototype.____constructor` | `(self, obj, evt_name, buff, stack, unit)` |  |
| `技能获得.prototype.____constructor` | `(self, obj, evt_name, unit, skill)` |  |
| `技能失去.prototype.____constructor` | `(self, obj, evt_name, unit, skill)` |  |
| `技能属性变化.prototype.____constructor` | `(self, obj, evt_name, skill, property, value_n)` |  |
| `技能等级变化.prototype.____constructor` | `(self, obj, evt_name, skill, level)` |  |
| `技能层数变化.prototype.____constructor` | `(self, obj, evt_name, skill, stack)` |  |
| `技能槽位变化.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `技能可用状态变化.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `技能学习状态变化.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `技能冷却完成.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `技能冷却激活.prototype.____constructor` | `(self, obj, evt_name, skill, time_remaining, time_total)` |  |
| `技能充能激活.prototype.____constructor` | `(self, obj, evt_name, skill, time_remaining, time_total)` |  |
| `状态获得.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `状态失去.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `状态层数变化.prototype.____constructor` | `(self, obj, evt_name, buff, stack, unit)` |  |
| `玩家改变英雄.prototype.____constructor` | `(self, obj, evt_name, player, unit)` |  |
| `玩家改变队伍.prototype.____constructor` | `(self, obj, evt_name, player, team)` |  |
| `玩家属性变化.prototype.____constructor` | `(self, obj, evt_name, player, property, value_n, value_s)` |  |
| `玩家断线.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `玩家重连.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `游戏开始.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `游戏结束.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `游戏更新.prototype.____constructor` | `(self, obj, evt_name, delta)` |  |
| `游戏属性变化.prototype.____constructor` | `(self, obj, evt_name, property, value_s)` |  |
| `游戏阶段切换.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `场景加载完成.prototype.____constructor` | `(self, obj, evt_name, scene_name)` |  |
| `消息技能.prototype.____constructor` | `(self, obj, evt_name, msg)` |  |
| `消息错误.prototype.____constructor` | `(self, obj, evt_name, msg, duration)` |  |
| `消息公告.prototype.____constructor` | `(self, obj, evt_name, msg, duration)` |  |
| `消息聊天.prototype.____constructor` | `(self, obj, evt_name, player, duration)` |  |
| `画面分辨率变化.prototype.____constructor` | `(self, obj, evt_name, width, height)` |  |
| `画面分辨率缩放变化.prototype.____constructor` | `(self, obj, evt_name, scale)` |  |
| `按键按下.prototype.____constructor` | `(self, obj, evt_name, key_keyboard)` |  |
| `按键松开.prototype.____constructor` | `(self, obj, evt_name, key_keyboard)` |  |
| `鼠标按下.prototype.____constructor` | `(self, obj, evt_name, key)` |  |
| `鼠标松开.prototype.____constructor` | `(self, obj, evt_name, key)` |  |
| `鼠标移动.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `表现动画事件开始.prototype.____constructor` | `(self, obj, evt_name, actor, msg, anmi)` |  |
| `表现动画事件结束.prototype.____constructor` | `(self, obj, evt_name, anmi, msg, actor)` |  |
| `表现音效事件.prototype.____constructor` | `(self, obj, evt_name, msg, actor)` |  |
| `对话开始.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `对话选择.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link)` |  |
| `鼠标点击物品栏中物品.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `对话结束时.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `对话跳过时.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `服务器请求切换场景.prototype.____constructor` | `(self, obj, old_scene, new_scene)` |  |
| `玩家暂时离开.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `游戏点击.prototype.____constructor` | `(self, obj, evt_name, screen_pos, actors_ID, button)` |  |
| `单位失去物品.prototype.____constructor` | `(self, obj, evt_name, unit, item, drop_mode)` |  |
| `单位获得物品.prototype.____constructor` | `(self, obj, evt_name, unit, item)` |  |
| `鼠标长按物品栏中物品.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `鼠标长按物品栏中物品抬起.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `联合场景区域通知.prototype.____constructor` | `(self, obj, evt_name, from_scene, from_area, to_scene, to_area)` |  |
| `游戏进入前台.prototype.____constructor` | `(self, obj, evt_name, key)` |  |
| `联合场景跨越区域.prototype.____constructor` | `(self, obj, evt_name, from_scene, from_area, to_scene, to_area)` |  |
| `联合场景进入区域.prototype.____constructor` | `(self, obj, evt_name, scene, area, target_scene)` |  |
| `联合场景离开区域.prototype.____constructor` | `(self, obj, evt_name, scene, area, target_scene)` |  |
| `玩家回到游戏.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `建造预放置开始.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `建造预放置确认.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `建造预放置取消.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `消息提示显示时.prototype.____constructor` | `(self, obj, toast, text, source)` |  |
| `菜单栏按钮按下时.prototype.____constructor` | `(self, obj, evt_name, Key)` |  |

### `@lib_common_ai/ai/ai_common`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai/default_ai`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/default_ai`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/主动召唤物`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/仅移动ai`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/召唤物`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/攻城车`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/自定义ai`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/通用ai`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/野怪`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/ai_templates/镖车`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/behavior/enmity`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/init`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/new`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/state/attack`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.attack_active_cd(attack:attack, max_cd:number)`
  - `base.attack_add_damage(attack:attack, source:unit, target:unit, damage:number)`
  - `base.attack_get_cd(attack:attack)`
  - `base.attack_get_name(attack:attack)`
  - `base.attack_is_common_attack(attack:attack)`
  - `base.attack_is_skill(attack:attack)`
  - `base.attack_set_cd(attack:attack, cd:number)`
  - `base.attack_stop(attack:attack)`
  - `base.attack_table(name, key)`

### `@lib_common_ai/class/state/back`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/state/move`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/state/none`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/class/state/pursue`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/customscript`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_ai/main`

- 归属：lib_common_ai 包
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：✅ 有源码（`lib_common_ai\43\src\main.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lib_common_ai/trigger_module_main_1`

- 归属：lib_common_ai 包
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：✅ 有源码（`lib_common_ai\43\ui\script\trigger_module_main_1.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lib_common_ai/trigger_validator`

- 归属：lib_common_ai 包
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：✅ 有源码（`lib_common_ai\43\ui\script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `init_validator_0` | `(self)` |  |

### `@lib_common_ai/utility`

- 归属：lib_common_ai 包（未找到源码）
- 研判：lib_common_ai 地图库 src/main.lua 仅 4 行：`require_folder 'ai'` / `require 'utility'` / `require_folder 'ai_templates'` / pcall ai_loader——AI 实现模块**未随编辑器侧包分发**（客户端变体或引擎内嵌），以下按命名与调用点推测。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@lib_common_sounds/main`

- 归属：lib_common_sounds 包
- 状态：✅ 有源码（`lib_common_sounds\16\src\main.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lib_control/main`

- 归属：lib_control 包
- 状态：✅ 有源码（`lib_control\46\src\main.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lib_control/require_libs`

- 归属：lib_control 包
- 状态：✅ 有源码（`lib_control\46\ui\script\require_libs.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lib_control/trigger_module_main_1`

- 归属：lib_control 包
- 状态：✅ 有源码（`lib_control\46\ui\script\trigger_module_main_1.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `lib_control.get_joystick_move_angle` | `(____, 摇杆)` |  |

### `@lib_control/trigger_validator`

- 归属：lib_control 包
- 状态：✅ 有源码（`lib_control\46\ui\script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `init_validator_0` | `(self)` |  |

### `@lib_game_options/gift_code`

- 归属：lib_game_options 包
- 状态：✅ 有源码（`lib_game_options\106\src\gift_code.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `subscribe` | `(game_name,player)` |  |
| `ok` | `(result)` |  |
| `error` | `(error_code, error_desc)` |  |
| `timeout` | `()` |  |
| `base.ui.proto.RequestGiftCode` | `(player, data)` | c2s兑换 |

### `@lib_game_options/main`

- 归属：lib_game_options 包
- 状态：✅ 有源码（`lib_game_options\106\src\main.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lib_game_options/rename`

- 归属：lib_game_options 包
- 状态：✅ 有源码（`lib_game_options\106\src\rename.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `check_name` | `(db,name,callback)` | 检测昵称的返回结果 → 0.可以使用 1.存在敏感字符 2.易盾挂了 3.重名了 4.查询超时 5.未达到修改时间 |
| `ok` | `(names)` |  |
| `error` | `(code, reason)` |  |
| `timeout` | `()` |  |
| `base.ui.proto.C2S_app_check_name` | `(player, data)` | 名称检测 |
| `base.ui.proto.C2S_app_rename` | `(player, data)` | 请求修改昵称 |
| `ok` | `(score, iscore, sscore)` |  |
| `ok` | `()` |  |
| `error` | `(code, reason)` |  |
| `timeout` | `()` |  |
| `error` | `(code, reason)` |  |
| `timeout` | `()` |  |

### `@lib_game_options/user_info`

- 归属：lib_game_options 包
- 状态：✅ 有源码（`lib_game_options\106\src\user_info.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `sync_info` | `(player)` |  |
| `sync_nick` | `(player)` |  |
| `ok` | `(score, iscore, sscore)` |  |
| `error` | `(code, reason)` |  |
| `timeout` | `()` |  |
