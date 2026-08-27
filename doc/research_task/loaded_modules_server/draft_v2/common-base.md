# script 包 · common 库（base 基础层）— draft_v2

模块数：93。来源：服务端 StateGame `package.loaded` dump（loaded_module_server_package_loaded.txt）+ script 包 v199 源码（`D:\sce_open\api-13\2026_08_27\script\199\`）+ server_lua_plus 包装器对照。

相对 draft_v1 的升级：每模块附 dump 值树证据（字段/函数/类/截断标注）、源码↔dump 对照差异、全字段置信级标注。

置信级约定：【实测】>【dump 实锤】>【源码实锤】>【反查推测】>【语义推测】（含义见研究任务提示词）。

> ⚠️ dump 值树中标注 inline_shared 的字段是被内联展开的共享表引用（如 `_descriptors`、`____super`），归属以 also_in 列表交叉判定；本组文档只把首属内容计入模块。
>
> 注：「值为 `true`」= 模块已加载但无表导出（Lua 惯例：模块无 return 时 package.loaded 记 true）；这类模块的功能经副作用挂在 `base.*` 全局上，dump 证据见 `_G.base.*`（范围外附录）。

---

### `@common`

- 来源：script 包（common 库）（`script\199\common\init.lua`）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require '(包根 init)'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

common 库入口（`common/init.lua`）：注册全局、驱动 base/preload 加载链。

### `@common/base`

- 来源：script 包（common 库）（`script\199\common\base\init.lua`）
- 加载：库装配入口本身（`require 'base'` 解析到 `base/init.lua`）；`require 'base'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

base 目录总装配入口：建 `_G.base`/`base.game`/`base.event`（=`game_events`，回调全被 xpcall 包裹）等核心全局，按平台/state 分批加载全部 base 模块；StateGame 追加 trigger_editor_v2、require_folder base_lua_plus、game_result。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `safe_callback` | `(name)` | name |  |  | 源码实锤 |
| `__newindex` | `(self, k, v)` | self, k, v |  |  | 源码实锤 |
| `base.error` | `(err,...)` | err,... |  |  | 源码实锤 |
| `base.callback_info` | `()` |  |  |  | 源码实锤 |
| `base.game.fff` | `()` |  |  |  | 源码实锤 |

### `@common/base/actor`

- 来源：script 包（common 库）（`script\199\common\base\actor.lua`）
- 加载：`require 'base.actor'`（init.lua:109，非 app 平台）；`require 'base.actor'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 44，类 1）

表现层 Actor 类（特效/模型/音效 C++ API 封装，1125 行）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `base.set_actor_map` | `(actor)` | actor |  |  | 源码实锤 |
| `base.set_actor_mode` | `(allow_ray_cast)` | allow_ray_cast |  |  | 源码实锤 |
| `base.set_unit_highlight_on` | `(unit,r,g,b,a,time)` | unit,r,g,b,a,time |  |  | 源码实锤 |
| `base.set_unit_highlight_off` | `(unit)` | unit |  |  | 源码实锤 |
| `base.actor` | `(name, sid, skip_birth, scene)` | name, sid, skip_birth, scene |  |  | 源码实锤 |
| `base.actor_from_id` | `(id)` | id | Actor |  | 源码实锤 |
| `base.actor_from_sid` | `(id)` | id | Actor |  | 源码实锤 |
| `mt:set_destroy_on_orphan` | `(destroy)` | destroy |  |  | 源码实锤 |
| `mt:is_destroy_on_orphan` | `()` |  | boolean |  | 源码实锤 |
| `mt:release` | `()` |  |  |  | 源码实锤 |
| `mt:destroy` | `(force)` | force |  |  | 源码实锤 |
| `mt:set_owner` | `(owner_id)` | owner_id |  |  | 源码实锤 |
| `mt:set_shadow` | `(enable)` | enable |  |  | 源码实锤 |
| `mt:set_point` | `(scene_point)` | scene_point |  |  | 源码实锤 |
| `mt:set_position` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:get_world_position` | `()` |  |  |  | 源码实锤 |
| `mt:get_position` | `()` |  |  |  | 源码实锤 |
| `mt:set_ground_z` | `(z)` | z |  |  | 源码实锤 |
| `mt:set_position_from` | `(target, socket)` | target, socket |  |  | 源码实锤 |
| `mt:set_rotation` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:get_rotation` | `()` |  |  |  | 源码实锤 |
| `mt:set_facing` | `(angle)` | angle |  |  | 源码实锤 |
| `mt:get_socket_position` | `(socket)` | socket |  |  | 源码实锤 |
| `mt:get_socket_rotation` | `(socket)` | socket |  |  | 源码实锤 |
| `mt:set_scale` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:set_scale_xyz` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:actor_set_scale` | `(x)` | x |  |  | 源码实锤 |
| `mt:set_asset` | `(asset)` | asset |  |  | 源码实锤 |
| `mt:set_fow` | `(enable, radius)` | enable, radius |  |  | 源码实锤 |
| `mt:set_grid_size` | `(size)` | size |  |  | 源码实锤 |
| `mt:set_grid_range` | `(start_id, range)` | start_id, range |  |  | 源码实锤 |
| `mt:set_grid_state` | `(grid_id, state)` | grid_id, state |  |  | 源码实锤 |
| `mt:set_grid_stick_to_ground` | `(enable)` | enable |  |  | 源码实锤 |
| `mt:attach_to` | `(target, socket)` | target, socket |  |  | 源码实锤 |
| `mt:attach_to_anchor` | `(anchor_name)` | anchor_name |  |  | 源码实锤 |
| `mt:set_bearings` | `(x, y, z, facing, use_ground_height)` | x, y, z, facing, use_ground_height |  |  | 源码实锤 |
| `mt:finalize_bearings` | `()` |  |  |  | 源码实锤 |
| `mt:detach` | `()` |  |  |  | 源码实锤 |
| `mt:show` | `(status)` | status |  |  | 源码实锤 |
| `mt:play` | `()` |  |  |  | 源码实锤 |
| `mt:play_anim_ex` | `(anim_name, anim_param)` | anim_name, anim_param |  |  | 源码实锤 |
| `mt:get_anims` | `()` |  | table<ICustomAnimParams> |  | 源码实锤 |
| `mt:play_animation` | `(anim, params)` | anim, params |  |  | 源码实锤 |
| `mt:stop` | `(fade)` | fade |  |  | 源码实锤 |
| `mt:pause` | `()` |  |  |  | 源码实锤 |
| `mt:resume` | `()` |  |  |  | 源码实锤 |
| `mt:set_volume` | `(volume)` | volume |  |  | 源码实锤 |
| `mt:get_highlight` | `()` |  |  |  | 源码实锤 |
| `mt:set_highlight` | `(on, ...)` | on, ... |  |  | 源码实锤 |
| `mt:set_material_parameters` | `(...)` | ... |  |  | 源码实锤 |
| `mt:set_launch_site` | `(unit, socket)` | unit, socket |  |  | 源码实锤 |
| `mt:set_impact_site` | `(unit, socket)` | unit, socket |  |  | 源码实锤 |
| `mt:set_launch_position` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:set_launch_scene_point` | `(point)` | point |  |  | 源码实锤 |
| `mt:set_launch_ground_z` | `(z)` | z |  |  | 源码实锤 |
| `mt:set_text` | `(text)` | text |  |  | 源码实锤 |
| `pos_distance` | `(p1, p2)` | p1, p2 |  |  | 源码实锤 |
| `sub_class_action.CameraShake` | `(self, cache)` | self, cache |  |  | 源码实锤 |
| `mt:do_sub_class_action` | `()` |  |  |  | 源码实锤 |
| `mt:create_actor` | `(link)` | link |  |  | 源码实锤 |
| `mt:create_actors` | `(msg)` | msg |  |  | 源码实锤 |
| `mt:destroy_actors` | `(msg)` | msg |  |  | 源码实锤 |
| `base.actor_info` | `()` |  |  |  | 源码实锤 |
| `base.get_actor_from_id` | `(id)` | id |  |  | 源码实锤 |
| `base.get_actor_from_sid` | `(id)` | id |  |  | 源码实锤 |
| `mt:anim_play` | `(anim_name, params)` | anim_name, params |  |  | 源码实锤 |
| `mt:set_time_scale_global` | `(scale)` | scale |  | 设置全局播放速度，只影响新API播放的动画 | 源码实锤 |
| `sort_bracket` | `(bracket1,bracket2)` | bracket1,bracket2 |  |  | 源码实锤 |
| `add_bracket_to_table` | `(self, bracket_anim)` | self, bracket_anim |  | 添加bracket动画 | 源码实锤 |
| `mt:anim_play_bracket` | `(anim_birth, anim_stand, anim_death, params)` | anim_birth, anim_stand, anim_death, params |  | 手动构建BSD动画，然后play动画 | 源码实锤 |
| `mt:anim_set_paused_all` | `(paused)` | paused |  |  | 源码实锤 |
| `mt:anim_operation` | `(op, params, ...)` | op, params, ... |  |  | 源码实锤 |
| `mt:register_bone_chain` | `(CHAIN_ID, bone_chain_data)` | CHAIN_ID, bone_chain_data |  | 参考 https://xindong.atlassian.net/wiki/spaces/Editor/pages/1060713486 | 源码实锤 |
| `mt:register_model_bone_chain` | `(bol)` | bol |  | 开放给触发用户用的，应用模型配的数据 | 源码实锤 |
| `mt:set_bone_chain_facing` | `(CHAIN_ID, angle, time)` | CHAIN_ID, angle, time |  |  | 源码实锤 |
| `mt:set_bone_chain_facing_v1` | `(angle, time)` | angle, time |  |  | 源码实锤 |
| `mt:reset_bone_chain_facing` | `(CHAIN_ID, time)` | CHAIN_ID, time |  |  | 源码实锤 |
| `mt:reset_bone_chain_facing_v1` | `(time)` | time |  |  | 源码实锤 |
| `base.get_actors_from_screen_xy` | `(xy)` | xy |  |  | 源码实锤 |
| `base.play_sound_effect` | `(link)` | link |  | 创建并播放2D音效 | 源码实锤 |
| `base.create_beam_effect` | `(link, source, target)` | link, source, target | Actor |  | 源码实锤 |
| `base.actor_enable_raycast` | `(actor, enable)` | actor, enable |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Actor` | TSTL 类（2 键） |  | dump 实锤 |
| `Actor.prototype` | table（46 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Actor` | 无 | `dispose`、`set_grid_state`、`anim_operation`、`destroy`、`set_launch_position`、`set_grid_range`、`mute`、`pause`、`set_volume`、`anim_play`、`set_scale_xyz`、`set_position`、`play_animation_bracket`、`attach_to`、`detach`、`get_visible_slots`、`resume`、`stop`、`play_animation`、`set_launch_ground_z`、`kill`、`set_grid_size`、`set_rotation`、`do_subclass_action`、`set_time_scale_global`、`set_impact_site`、`play`、`anim_play_bracket`、`is_valid`、`set_shadow`、`anim_set_paused_all`、`show`、`set_owner`、`on_normal_init`、`set_text`、`set_launch_scene_point`、`remove`、`set_bearings`、`set_launch_site`、`set_position_from`、`set_facing`、`set_ground_z`、`set_scale`、`set_asset` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Actor.prototype.dispose`、`Actor.prototype.mute`、`Actor.prototype.play_animation_bracket`、`Actor.prototype.get_visible_slots`、`Actor.prototype.kill`、`Actor.prototype.do_subclass_action`、`Actor.prototype.is_valid`、`Actor.prototype.on_normal_init`、`Actor.prototype.remove`

### `@common/base/ad`

- 来源：script 包（common 库）（`script\199\common\base\ad.lua`）
- 加载：`require 'base.ad'`（init.lua:136）；`require 'base.ad'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

激励视频广告封装 `show_reward_video_ad`（注册 rpc；失败回退自定义 webview 广告）。

**dump 对照差异（重要）**：源码末尾 `return { show_reward_video_ad = show_reward_video_ad }`（ad.lua:60-62）【源码实锤】，但 dump 中本键值为 `true` —— 运行时 package.loaded 未保留返回表。推测：服务端加载的同名单模块未走到 return（平台守卫或服务端变体），或返回值被加载链丢弃【语义推测】。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `show_reward_video_ad` | `(reward, reward_amount, extra, cb)` | reward, reward_amount, extra, cb |  |  | 源码实锤 |
| `cb` | `(val)` | val |  |  | 源码实锤 |

### `@common/base/admin`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.admin'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

管理员/后台接口（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/ai`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.ai'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 0，类 1）

AI 基类（TS 类 `AI`）；运行时另有 lua_plus `base.ai_attack_*` 扁平封装。

**lua_plus 扁平封装**（`server_lua_plus\14\base\base_lua_plus\` 对应模块，带触编 @ui 注解）【源码实锤】：

- `base.ai_attack_add_team_threat(ai_attack, team, threat)`
- `base.ai_attack_add_team_threat(ai_attack:ai_attack, team:integer, threat:integer)`
- `base.ai_attack_add_type_threat(ai_attack, unit_tag, threat)`
- `base.ai_attack_add_type_threat(ai_attack:ai_attack, unit_tag:单位标签, threat:integer)`
- `base.ai_attack_add_unit_threat(ai_attack, unit, threat)`
- `base.ai_attack_add_unit_threat(ai_attack:ai_attack, unit:unit, threat:integer)`
- `base.ai_attack_remove(ai_attack)`
- `base.ai_attack_remove(ai_attack:ai_attack)`

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `AI` | TSTL 类（2 键） |  | dump 实锤 |
| `AI.prototype` | table（3 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `AI` | 无 | （仅构造/元方法） |

### `@common/base/ai_searcher`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.ai_searcher'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 3，类 1）

AI 索敌器（TS 类 `AISearcher`，描述符属性 default_attack/unit/default_range）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `AISearcher` | TSTL 类（1 键） |  | dump 实锤 |
| `AISearcher.prototype` | table（5 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `AISearcher` | 无 | （仅构造/元方法） |

⚠️ **此处被截断，字段不全**：9 处（`<max depth exceeded>`），全部为 `prototype._descriptors.*` 属性访问器（get/set/enumerable/configurable），不损失 API 信息。全量截断路径见 keys_index.json / 对应 fields JSON。

### `@common/base/anim_handlers`

- 来源：script 包（common 库）（`script\199\common\base\anim_handlers.lua`）
- 加载：`require 'base.anim_handlers'`（init.lua:110，非 app 平台）；`require 'base.anim_handlers'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

动画句柄注册管理：`base.anim`/`base.bracket_anim`（三段式）及播放控制。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.get_anim_map` | `()` |  |  |  | 源码实锤 |
| `base.get_anim_bracket_map` | `()` |  |  |  | 源码实锤 |
| `base.anim` | `(anim_name, owner_type, owner_id, owner_name, params)` | anim_name, owner_type, owner_id, owner_name, params |  |  | 源码实锤 |
| `base.bracket_anim` | `(anim_birth, anim_stand, anim_death, params, owner_type, owner_id, owner_name)` | anim_birth, anim_stand, anim_death, params, owner_type, owner_id, owner_name |  |  | 源码实锤 |
| `mt:play` | `(anim, loop, speed, blend_time)` | anim, loop, speed, blend_time |  |  | 源码实锤 |
| `mt:get_unit_or_actor` | `()` |  |  |  | 源码实锤 |
| `mt:replay` | `()` |  |  |  | 源码实锤 |
| `mt:refresh_global_pause` | `(paused)` | paused |  |  | 源码实锤 |
| `mt:pause` | `()` |  |  |  | 源码实锤 |
| `mt:resume` | `()` |  |  |  | 源码实锤 |
| `mt:set_time` | `(time, trigger_events)` | time, trigger_events |  |  | 源码实锤 |
| `mt:set_time_scale` | `(scale)` | scale |  |  | 源码实锤 |
| `mt:set_time_scale_absolute` | `(scale)` | scale |  |  | 源码实锤 |
| `mt:set_percentage` | `(percentage)` | percentage |  |  | 源码实锤 |
| `mt:set_duration` | `(duration)` | duration |  |  | 源码实锤 |
| `mt:destroy` | `()` |  |  |  | 源码实锤 |
| `mt:bracket_stop` | `()` |  |  |  | 源码实锤 |
| `mt:check_valid` | `()` |  |  | 检查该句柄的有效性 | 源码实锤 |
| `mt:remove` | `()` |  |  |  | 源码实锤 |

### `@common/base/array`

- 来源：script 包（common 库）（`script\199\common\base\array.lua`）
- 加载：`require 'base.array'`（init.lua:118，非 app 平台）；`require 'base.array'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

带默认值/长度维护的数组容器 `base.array(default, t?)`。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__index` | `(pos)` | pos |  |  | 源码实锤 |
| `mt:__newindex` | `(pos, value)` | pos, value |  |  | 源码实锤 |
| `mt:__len` | `()` |  |  |  | 源码实锤 |
| `mt:__pairs` | `()` |  |  |  | 源码实锤 |
| `set_len` | `(self, len)` | self, len |  |  | 源码实锤 |
| `insert` | `(self, pos, value)` | self, pos, value |  |  | 源码实锤 |
| `remove` | `(self, pos)` | self, pos |  |  | 源码实锤 |
| `random` | `(self)` | self |  |  | 源码实锤 |
| `convert` | `(self, t)` | self, t |  |  | 源码实锤 |
| `base.array` | `(default, t)` | default, t |  |  | 源码实锤 |

### `@common/base/auxiliary`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.auxiliary'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

辅助函数集（反查得 add_animation/get_player_id/get_system_time 等形态）。

**调用点反查 API 形态**（参数以实际调用为准）【反查推测】：

- `add_animation(unit, animation_name, scale, is_loop, part)`
- `get_game_mode_args()`
- `get_map_kind()`
- `get_player_id(base.local_player()`
- `get_player_id(operatorPlayer)`
- `get_player_id(player)`
- `get_player_id(playerObj)`
- `get_system_time()`

### `@common/base/behavior`

- 来源：script 包（common 库）（`script\199\common\base\behavior.lua`）
- 加载：`require 'base.behavior'`（init.lua:127，非 app 平台）；`require 'base.behavior'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

交互行为：悬停高亮/光标形态、右键派单（proto unit_get/remove_interaction_spell）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.proto.unit_get_interaction_spell` | `(msg)` | msg |  |  | 源码实锤 |
| `base.proto.unit_remove_interaction_spell` | `(msg)` | msg |  |  | 源码实锤 |
| `base.refresh_interact_joystick` | `()` |  |  |  | 源码实锤 |
| `init` | `()` |  |  |  | 源码实锤 |

### `@common/base/buff`

- 来源：script 包（common 库）（`script\199\common\base\buff.lua`）
- 加载：`require 'base.buff'`（init.lua:113，非 app 平台）；`require 'base.buff'`
- 状态：✅ 有源码
- dump 值：table（顶层 2 键，函数 52，类 2）

Buff 类（剩余时间/层数/暂停恢复/事件）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `get_buff_name_by_hash` | `(hash)` | hash |  |  | 源码实锤 |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:get_name` | `()` |  |  |  | 源码实锤 |
| `set_remaining` | `(self, remaining)` | self, remaining |  |  | 源码实锤 |
| `mt:get_remaining` | `()` |  |  |  | 源码实锤 |
| `set_time` | `(self, time)` | self, time |  |  | 源码实锤 |
| `mt:get_time` | `()` |  |  |  | 源码实锤 |
| `mt:pause` | `()` |  |  |  | 源码实锤 |
| `mt:resume` | `()` |  |  |  | 源码实锤 |
| `mt:update_paused` | `()` |  |  |  | 源码实锤 |
| `mt:get_owner` | `()` |  |  |  | 源码实锤 |
| `set_stack` | `(self, stack , send_event)` | self, stack , send_event |  |  | 源码实锤 |
| `mt:get_stack` | `()` |  |  |  | 源码实锤 |
| `mt:event_notify` | `(name, ...)` | name, ... |  |  | 源码实锤 |
| `mt:event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `ac_buff` | `(unit_id, hash, index)` | unit_id, hash, index |  |  | 源码实锤 |
| `try_load_show_methods` | `()` |  |  |  | 源码实锤 |
| `mt:get_show_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_icon` | `()` |  |  |  | 源码实锤 |
| `mt:get_tips` | `()` |  |  |  | 源码实锤 |
| `mt:get_current_cd` | `()` |  |  |  | 源码实锤 |
| `mt:get_cd_max` | `()` |  |  |  | 源码实锤 |
| `base.event.on_buff_attached` | `(unit_id, hash, index, time, remaining, stack)` | unit_id, hash, index, time, remaining, stack |  |  | 源码实锤 |
| `base.event.on_buff_detached` | `(unit_id, hash, index)` | unit_id, hash, index |  |  | 源码实锤 |
| `base.event.on_buff_update` | `(unit_id, hash, index, time, remaining, stack)` | unit_id, hash, index, time, remaining, stack |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |
| `Buff` | TSTL 类（2 键） |  | dump 实锤 |
| `Buff.prototype` | table（25 键） |  | dump 实锤 |
| `UnitBuff` | TSTL 类（2 键） |  | dump 实锤 |
| `UnitBuff.prototype` | table（33 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Buff` | 无 | `set_stack_`、`has_category`、`is_enabled`、`_on_pulse`、`on_finish`、`get_remain_time`、`add_stack_`、`__tostring`、`init_root_effect`、`on_become_instance_enabled`、`filter_categories`、`set_level`、`unapply_attribute_change`、`apply_attribute_change`、`_on_cover`、`_on_add`、`get_name`、`get_level`、`on_become_instance_disabled`、`_on_finish`、`_on_remove`、`get_tracked_units` |
| `UnitBuff` | 无 | `new`、`enable_skill_category`、`apply_buff_states`、`apply_unit_states`、`apply_attribute_change`、`start_by_frame_height_update`、`enable_buff_category`、`enable_buff`、`on_become_enabled`、`is_enabled`、`get_instances`、`unapply_unit_states`、`update_state`、`is_valid`、`get_stack_count`、`disable_skill_category`、`setup`、`get`、`unapply_attribute_change`、`disable_buff_category`、`clear`、`disable_buff`、`on_become_disabled`、`create_actors`、`create_actor`、`unapply_buff_states`、`setup_height_update`、`disable_skill`、`destroy_actors`、`enable_skill` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Buff.prototype.set_stack_`、`Buff.prototype.has_category`、`Buff.prototype.is_enabled`、`Buff.prototype._on_pulse`、`Buff.prototype.on_finish`、`Buff.prototype.get_remain_time`、`Buff.prototype.add_stack_`、`Buff.prototype.init_root_effect`、`Buff.prototype.on_become_instance_enabled`、`Buff.prototype.filter_categories`、`Buff.prototype.set_level`、`Buff.prototype.unapply_attribute_change`、`Buff.prototype.apply_attribute_change`、`Buff.prototype._on_cover`、`Buff.prototype._on_add`、`Buff.prototype.get_level`、`Buff.prototype.on_become_instance_disabled`、`Buff.prototype._on_finish`、`Buff.prototype._on_remove`、`Buff.prototype.get_tracked_units`、`UnitBuff.prototype.new`、`UnitBuff.prototype.enable_skill_category`、`UnitBuff.prototype.apply_buff_states`、`UnitBuff.prototype.apply_unit_states`、`UnitBuff.prototype.apply_attribute_change`、`UnitBuff.prototype.start_by_frame_height_update`、`UnitBuff.prototype.enable_buff_category`、`UnitBuff.prototype.enable_buff`、`UnitBuff.prototype.on_become_enabled`、`UnitBuff.prototype.is_enabled`、`UnitBuff.prototype.get_instances`、`UnitBuff.prototype.unapply_unit_states`、`UnitBuff.prototype.update_state`、`UnitBuff.prototype.is_valid`、`UnitBuff.prototype.get_stack_count`、`UnitBuff.prototype.disable_skill_category`、`UnitBuff.prototype.setup`、`UnitBuff.prototype.get`、`UnitBuff.prototype.unapply_attribute_change`、`UnitBuff.prototype.disable_buff_category`、…（共 50 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/channeler`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.channeler'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 2 键，函数 11，类 2）

引导施法通道：`Channeler`（施法方）/`Channeled`（被引导方）两个 TS 类。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `Channeler.prototype.register` | `(?)` |  |  |  | dump 实锤 |
| `Channeler.prototype.is_channeling` | `(?)` |  |  |  | dump 实锤 |
| `Channeler.prototype.new` | `(?)` |  |  |  | dump 实锤 |
| `Channeler.prototype.clear` | `(?)` |  |  |  | dump 实锤 |
| `Channeler.prototype.start_channeling` | `(?)` |  |  |  | dump 实锤 |
| `Channeler.prototype.is_valid` | `(?)` |  |  |  | dump 实锤 |
| `Channeler.prototype.stop_channeling` | `(?)` |  |  |  | dump 实锤 |
| `Channeled.prototype.is_channeling` | `(?)` |  |  |  | dump 实锤 |
| `Channeled.prototype.start_channeling` | `(?)` |  |  |  | dump 实锤 |
| `Channeled.prototype.new` | `(?)` |  |  |  | dump 实锤 |
| `Channeled.prototype.stop_channeling` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |
| `Channeler` | TSTL 类（2 键） |  | dump 实锤 |
| `Channeler.prototype` | table（10 键） |  | dump 实锤 |
| `Channeled` | TSTL 类（2 键） |  | dump 实锤 |
| `Channeled.prototype` | table（7 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Channeler` | 无 | `register`、`is_channeling`、`new`、`clear`、`start_channeling`、`is_valid`、`stop_channeling` |
| `Channeled` | 无 | `is_channeling`、`start_channeling`、`new`、`stop_channeling` |

### `@common/base/cheat`

- 来源：script 包（common 库）（`script\199\common\base\cheat.lua`）
- 加载：`require 'base.cheat'`（init.lua:139）；`require 'base.cheat'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

作弊码/GM 命令体系（`玩家-输入作弊码` 分派 + `__gm_debug_*` proto）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `gm.showmovejoystick` | `(cmd)` | cmd |  |  | 源码实锤 |
| `set` | `(self, props)` | self, props |  |  | 源码实锤 |
| `base.proto.__gm_debug_unit` | `(msg)` | msg |  |  | 源码实锤 |
| `base.proto.__gm_debug_player` | `(msg)` | msg |  |  | 源码实锤 |
| `set` | `(self, all_trace_player_props)` | self, all_trace_player_props |  |  | 源码实锤 |
| `set` | `(self, props)` | self, props |  |  | 源码实锤 |
| `base.proto.__gm_debug_game` | `(msg)` | msg |  |  | 源码实锤 |
| `eff_destroy` | `(root_id, id, force)` | root_id, id, force |  |  | 源码实锤 |
| `eff_destroy_all` | `()` |  |  |  | 源码实锤 |
| `base.proto.__gm_debug_eff_destory_all` | `(msg)` | msg |  |  | 源码实锤 |
| `base.proto.__gm_debug_eff_destory` | `(msg)` | msg |  |  | 源码实锤 |
| `base.proto.__gm_debug_eff_info` | `(msg)` | msg |  |  | 源码实锤 |
| `get_unit_point` | `(eff_data)` | eff_data |  | 是单位则更新单位所处地点 | 源码实锤 |
| `draw_circle_area` | `(eff_data,actor, color)` | eff_data,actor, color |  |  | 源码实锤 |
| `draw_arc_area` | `(eff_data,actor)` | eff_data,actor |  |  | 源码实锤 |
| `draw_line_area` | `(eff_data,actor)` | eff_data,actor |  |  | 源码实锤 |
| `get_eff_method` | `(eff_data)` | eff_data |  |  | 源码实锤 |
| `draw_line` | `(point, parent_point, actor, color)` | point, parent_point, actor, color |  |  | 源码实锤 |
| `base.cheat.VRP` | `(eff_data)` | eff_data |  |  | 源码实锤 |
| `base.proto.__gm_debug_eff_keep` | `(msg)` | msg |  |  | 源码实锤 |
| `base.cheat.VAO_cast` | `(source_id, target_id, info)` | source_id, target_id, info |  | 将data中的目标与来源通过红线连接，并在来源头上标注技能信息 | 源码实锤 |
| `base.cheat.VAO_approach` | `(source_id, target_id, info)` | source_id, target_id, info |  |  | 源码实锤 |
| `base.cheat.VAO_approach_destory` | `(source_id)` | source_id |  |  | 源码实锤 |
| `base.proto.__gm_debug_ai_order` | `(msg)` | msg |  |  | 源码实锤 |

### `@common/base/circle`

- 来源：script 包（common 库）（`script\199\common\base\circle.lua`）
- 加载：`require 'base.circle'`（init.lua:128，非 app 平台）；`require 'base.circle'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 7，类 2）

圆形区域 `RegionCircle` 类（可挂区域进出事件）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:get_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_range` | `()` |  |  |  | 源码实锤 |
| `mt:random_point` | `()` |  |  |  | 源码实锤 |
| `mt:scene_random_point` | `()` |  |  |  | 源码实锤 |
| `mt:init_region` | `(filter)` | filter |  |  | 源码实锤 |
| `region:on_enter` | `(unit)` | unit |  |  | 源码实锤 |
| `region:on_leave` | `(unit)` | unit |  |  | 源码实锤 |
| `mt:remove_region` | `()` |  |  |  | 源码实锤 |
| `base.circle` | `(point, range, scene_name)` | point, range, scene_name |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `RegionCircle` | TSTL 类（3 键） |  | dump 实锤 |
| `RegionCircle.prototype` | table（11 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `RegionCircle` | extends `Region` | `remove_region`、`get_range`、`random_point`、`init_region`、`get_scene_point`、`scene_random_point`、`get_point` |

### `@common/base/class`

- 来源：script 包（common 库）（`script\199\common\base\class.lua`）
- 加载：被 state_machine.lua 等 `require`（桩 → client_base）；`require 'base.class'`
- 状态：🔀 转发桩（`return require '@base.base.class'`，实现在 client_base 库，不在本包）
- dump 值：`true`（已加载无表导出）

OOP class 函数（桩，实现 client_base）。

### `@common/base/cmd_result`

- 来源：script 包（common 库）（`script\199\common\base\cmd_result.lua`）
- 加载：`require 'base.cmd_result'`（init.lua:105，非 app 平台）；`require 'base.cmd_result'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 6，类 1）

命令结果 `CmdResult` 类（e_cmd 包装，支持比较元方法）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.cmd_result:new` | `()` |  | CmdResult |  | 源码实锤 |
| `base.cmd_result:__eq` | `(other)` | other | boolean |  | 源码实锤 |
| `base.cmd_result:__lt` | `(other)` | other | boolean |  | 源码实锤 |
| `base.cmd_result:__le` | `(other)` | other | boolean |  | 源码实锤 |
| `base.cmd_result:get_value` | `()` |  | integer |  | 源码实锤 |
| `base.cmd_result:get_text` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `CmdResult` | TSTL 类（2 键） |  | dump 实锤 |
| `CmdResult.prototype` | table（9 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `CmdResult` | 无 | `__le`、`get_value`、`get_text`、`__eq`、`new`、`__lt` |

### `@common/base/co`

- 来源：script 包（common 库）（`script\199\common\base\co.lua`）
- 加载：被 promise/ad/voice/lobby 等 `require`/`include`（桩 → client_base）；`require 'base.co'`
- 状态：🔀 转发桩（`return require '@base.base.co'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 10 键，函数 12，类 1）

协程工具（桩 → client_base；dump 揭示实际导出 async/wrap/sleep 等 + Coroutine 类）。

**说明**：桩文件仅 `return require '@base.base.co'`，但 dump 值树直接揭示了 client_base co 模块的运行时导出（下方函数表）【dump 实锤】。`async`/`wrap`/`sleep` 与 url_demo.lua 的 `co.async(function() ... end)` 范式吻合【反查推测：实测范例佐证】。`Coroutine` 类的 `Status` 属性走 `_descriptors` 访问器（3 处截断即此，不损失信息）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `call` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `thread_to_tsCo` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `sleep_one_frame` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `async` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `tsCo_to_thread` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `wrap` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `async_next` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `sleep` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `will_async` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（10 键） |  | dump 实锤 |
| `Coroutine` | TSTL 类（2 键） |  | dump 实锤 |
| `Coroutine.prototype` | table（5 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Coroutine` | 无 | （仅构造/元方法） |

⚠️ **此处被截断，字段不全**：3 处（`<max depth exceeded>`），全部为 `prototype._descriptors.*` 属性访问器（get/set/enumerable/configurable），不损失 API 信息。全量截断路径见 keys_index.json / 对应 fields JSON。

### `@common/base/collision_flags`

- 来源：script 包（common 库）（`script\199\common\base\collision_flags.lua`）
- 加载：`require 'base.collision_flags'`（init.lua:79）；`require 'base.collision_flags'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

碰撞标志位掩码类 `base.collision_flags(mask)`。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.collision_flags` | `(mask)` | mask |  |  | 源码实锤 |
| `mt:contains` | `(flag)` | flag |  | 是否包含某一类型碰撞 | 源码实锤 |
| `mt:each_collision` | `(callback)` | callback |  | 遍历为真的碰撞 | 源码实锤 |

### `@common/base/crop`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.crop'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

收割/采集相关（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/damage`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.damage'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 14，类 1）

伤害实例 `DamageInstance` 类（TS）；运行时另有 lua_plus `base.damage_*` 扁平封装。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `DamageInstance.prototype.on_attribute_defence` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.mul` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.kill` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.get_damage` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.get_angle` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.is_crit` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.set_current_damage` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.on_attribute_attack` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.event` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.on_attribute_crit` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.is_item` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.get_current_damage` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.is_aoe` | `(?)` |  |  |  | dump 实锤 |
| `DamageInstance.prototype.div` | `(?)` |  |  |  | dump 实锤 |

**lua_plus 扁平封装**（`server_lua_plus\14\base\base_lua_plus\` 对应模块，带触编 @ui 注解）【源码实锤】：

- `base.damage_get_angle(damage:damage)`
- `base.damage_get_current_damage(damage)`
- `base.damage_get_current_damage(damage:damage)`
- `base.damage_get_damage(damage)`
- `base.damage_get_damage(damage:damage)`
- `base.damage_get_type(damage)`
- `base.damage_get_type(damage:damage)`
- `base.damage_set_current_damage(damage, amount)`
- `base.damage_set_current_damage(damage:damage, amount:number)`

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `DamageInstance` | TSTL 类（2 键） |  | dump 实锤 |
| `DamageInstance.prototype` | table（24 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `DamageInstance` | 无 | `on_attribute_defence`、`mul`、`kill`、`get_damage`、`get_angle`、`is_crit`、`set_current_damage`、`on_attribute_attack`、`event`、`on_attribute_crit`、`is_item`、`get_current_damage`、`is_aoe`、`div` |

### `@common/base/datetime`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.datetime'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 1，类 1）

日期时间 `DateTime` 类（TS），静态方法 get_date_time。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `DateTime.get_date_time` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `DateTime` | TSTL 类（3 键） |  | dump 实锤 |
| `DateTime.prototype` | table（2 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `DateTime` | 无 | （仅构造/元方法） |

### `@common/base/deque`

- 来源：script 包（common 库）（`script\199\common\base\deque.lua`）
- 加载：`require 'base.deque'`（init.lua:88）；`require 'base.deque'`
- 状态：🔀 转发桩（`return require '@base.base.deque'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 2 键，函数 2，类 0）

双端队列（桩 → client_base；dump 揭示 create_deque/create_queue）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `create_deque` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `create_queue` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |

### `@common/base/detection`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.detection'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

文本安全检测 `check_text`（反查形态）。

**调用点反查 API 形态**（参数以实际调用为准）【反查推测】：

- `check_text(data.text,function(suggestion)`
- `check_text(name, function(suggestion)`
- `check_text(nick, function(suggestion)`
- `check_text(text,function(suggestion,list)`

### `@common/base/eff`

- 来源：script 包（common 库）（`script\199\common\base\eff.lua`）
- 加载：`require 'base.eff'`（init.lua:103，非 app 平台）；`require 'base.eff'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

效果（Effect）数编缓存与枚举：`base.eff.cache(link)` 统一入口 + e_cmd/e_site 等枚举。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `eff.init_cache` | `()` |  |  |  | 源码实锤 |
| `eff.merge_cache` | `(in_cache)` | in_cache |  |  | 源码实锤 |
| `eff.has_cache_init` | `()` |  |  |  | 源码实锤 |
| `eff.cache_init_finished` | `()` |  |  |  | 源码实锤 |
| `eff.caches` | `(node_type)` | node_type |  |  | 源码实锤 |
| `eff.all_caches` | `(node_type)` | node_type |  |  | 源码实锤 |
| `eff.cache` | `(link)` | link | table? |  | 源码实锤 |
| `eff:cache_ts` | `(link)` | link | table? |  | 源码实锤 |
| `eff.get_node_type` | `(node_type)` | node_type |  |  | 源码实锤 |
| `eff.cache_as` | `(link, node_type)` | link, node_type |  |  | 源码实锤 |
| `eff.original_data` | `()` |  |  |  | 源码实锤 |
| `eff.get_namespace` | `(link)` | link | table |  | 源码实锤 |
| `eff.find_sibling` | `(link, name)` | link, name | table |  | 源码实锤 |
| `eff.validate` | `(ref_param, do_cache)` | ref_param, do_cache | string? |  | 源码实锤 |
| `eff.execute_validators` | `(validators, ref_param, ...)` | validators, ref_param, ... |  |  | 源码实锤 |
| `execute_internal` | `(ref_param)` | ref_param | CmdResult |  | 源码实锤 |
| `eff.execute` | `(ref_param)` | ref_param | CmdResult |  | 源码实锤 |

### `@common/base/eff_param`

- 来源：script 包（common 库）（`script\199\common\base\eff_param.lua`）
- 加载：`require 'base.eff_param'`（init.lua:104，非 app 平台）；`require 'base.eff_param'`
- 状态：✅ 有源码
- dump 值：table（顶层 2 键，函数 77，类 2）

效果参数对象 `EffectParam`/`EffectParamShared`（技能/效果执行的参数载体）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `ref_param:debuginfo` | `()` |  | string |  | 源码实锤 |
| `ref_param:logfail` | `(result, info)` | result, info | string? |  | 源码实锤 |
| `ref_param:new` | `(init_tree)` | init_tree | EffectParam |  | 源码实锤 |
| `ref_param:is_root` | `()` |  | boolean |  | 源码实锤 |
| `ref_param:root` | `()` |  | EffectParam |  | 源码实锤 |
| `ref_param:create_child` | `()` |  | EffectParam |  | 源码实锤 |
| `ref_param:get_scene` | `()` |  | string? |  | 源码实锤 |
| `ref_param:set_var_point` | `(key, point)` | key, point |  |  | 源码实锤 |
| `ref_param:set_var_unit` | `(key, unit)` | key, unit |  |  | 源码实锤 |
| `ref_param:var_unit` | `(key)` | key | Unit\|nil |  | 源码实锤 |
| `ref_param:var_point` | `(key)` | key | Point\|nil |  | 源码实锤 |
| `ref_param:link_child` | `(child_param)` | child_param |  |  | 源码实锤 |
| `ref_param:set` | `(in_ref_param)` | in_ref_param |  |  | 源码实锤 |
| `ref_param:init` | `(source,default_target)` | source,default_target |  |  | 源码实锤 |
| `ref_param:set_source` | `(source)` | source |  |  | 源码实锤 |
| `ref_param:calc_target` | `()` |  |  |  | 源码实锤 |
| `ref_param:set_target` | `(target)` | target |  |  | 源码实锤 |
| `ref_param:set_launch` | `(launch)` | launch |  |  | 源码实锤 |
| `ref_param:get_level` | `()` |  |  |  | 源码实锤 |
| `ref_param:level_data` | `(data, fallbackValue, level)` | data, fallbackValue, level | boolean\|string\|number |  | 源码实锤 |
| `ref_param:set_cache` | `(link)` | link |  |  | 源码实锤 |
| `ref_param:set_buff` | `(buff)` | buff |  |  | 源码实锤 |
| `ref_param:snap_shot_values` | `(table)` | table | table |  | 源码实锤 |
| `ref_param:search` | `(link)` | link | EffectParam |  | 源码实锤 |
| `ref_param:unit_sorts` | `(group, sorts)` | group, sorts |  |  | 源码实锤 |
| `ref_param:missile_detach` | `()` |  |  |  | 源码实锤 |
| `ref_param:is_missile_detached` | `()` |  | boolean |  | 源码实锤 |
| `ref_param:set_channeler` | `(channeler)` | channeler |  |  | 源码实锤 |
| `ref_param:get_channeler` | `()` |  | Channeler |  | 源码实锤 |
| `ref_param:skill` | `()` |  | Skill |  | 源码实锤 |
| `ref_param:cast` | `()` |  | Cast |  | 源码实锤 |
| `ref_param:caster` | `()` |  | Target |  | 源码实锤 |
| `ref_param:item` | `()` |  | Target |  | 源码实锤 |
| `ref_param:user_data` | `()` |  | table |  | 源码实锤 |
| `ref_param:item_random` | `(buff_link, prop_name,a, b, is_percentage, stack_index)` | buff_link, prop_name,a, b, is_percentage, stack_index | number |  | 源码实锤 |
| `ref_param:origin` | `()` |  | Target |  | 源码实锤 |
| `ref_param:main_target` | `()` |  | Target |  | 源码实锤 |
| `ref_param:set_caster` | `(caster)` | caster |  |  | 源码实锤 |
| `ref_param:set_origin` | `(origin_target)` | origin_target |  |  | 源码实锤 |
| `ref_param:set_creator` | `(creator_player)` | creator_player |  |  | 源码实锤 |
| `ref_param:creator_player` | `()` |  |  |  | 源码实锤 |
| `ref_param:setup_caster` | `()` |  |  |  | 源码实锤 |
| `ref_param:set_damage_modifiers` | `(unit,needreset)` | unit,needreset |  |  | 源码实锤 |
| `ref_param:get_site_target` | `(site, var)` | site, var | Target |  | 源码实锤 |
| `ref_param:parse_loc` | `(loc_express, type)` | loc_express, type | Target? |  | 源码实锤 |
| `ref_param:parse_player` | `(player_express)` | player_express | Player |  | 源码实锤 |
| `ref_param:parse_angle` | `(angle_express)` | angle_express | number |  | 源码实锤 |
| `ref_param:event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `ref_param:post_event` | `(event_subname)` | event_subname |  |  | 源码实锤 |
| `ref_param:post_new_target` | `(new_target)` | new_target |  |  | 源码实锤 |
| `ref_param:init_child_on` | `(link, target)` | link, target | EffectParam |  | 源码实锤 |
| `ref_param:execute` | `()` |  |  |  | 源码实锤 |
| `ref_param:execute_child_on` | `(link, target)` | link, target | CmdResult |  | 源码实锤 |
| `get_exclude` | `(in_player, mask, scene)` | in_player, mask, scene | integer[] |  | 源码实锤 |
| `ref_param:create_actor` | `(link, position, force_no_sync)` | link, position, force_no_sync | Actor? |  | 源码实锤 |
| `ref_param:next_child` | `()` |  |  |  | 源码实锤 |
| `ref_param:stop` | `()` |  |  |  | 源码实锤 |
| `ref_param:add_buff` | `(target, link, stack, params)` | target, link, stack, params | EffectParam |  | 源码实锤 |
| `ref_param:damage` | `(target, amount, type, params)` | target, amount, type, params | EffectParam |  | 源码实锤 |
| `Amount` | `()` |  |  |  | 源码实锤 |
| `ref_param:loop` | `(loop_data)` | loop_data |  |  | 源码实锤 |
| `tick` | `()` |  |  |  | 源码实锤 |
| `early_out` | `()` |  |  |  | 源码实锤 |
| `safe_tick` | `()` |  |  |  | 源码实锤 |
| `tick_start` | `()` |  |  |  | 源码实锤 |
| `ref_param:on_channeler_cleared` | `()` |  |  |  | 源码实锤 |
| `ref_param:loop_clear_up` | `(complete)` | complete |  |  | 源码实锤 |
| `ref_param:get_node_in_module` | `(name)` | name | any |  | 源码实锤 |
| `ref_shared:new` | `(root)` | root | EffectParamShared |  | 源码实锤 |
| `ref_shared:close` | `()` |  |  |  | 源码实锤 |
| `ref_shared:is_closed` | `()` |  |  |  | 源码实锤 |
| `ref_shared:set_skill` | `(cast)` | cast |  |  | 源码实锤 |
| `ref_shared:set_level` | `(level)` | level |  |  | 源码实锤 |
| `ref_shared:set_weapon` | `(weapon)` | weapon |  |  | 源码实锤 |
| `ref_shared:set_item` | `(item)` | item |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |
| `EffectParamShared` | TSTL 类（2 键） |  | dump 实锤 |
| `EffectParamShared.prototype` | table（9 键） |  | dump 实锤 |
| `EffectParam` | TSTL 类（2 键） |  | dump 实锤 |
| `EffectParam.prototype` | table（74 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `EffectParamShared` | 无 | `set_weapon`、`new`、`set_level`、`get_snapshot_attribute`、`set_item`、`set_skill` |
| `EffectParam` | 无 | `setup_caster`、`parse_angle`、`debuginfo`、`new`、`set_launch`、`link_child`、`on_channeler_cleared`、`get_site_target`、`create_unit`、`damage`、`set_buff`、`user_data`、`stop`、`get_node_in_module`、`parse_player`、`is_missile_detached`、`get_creation_param`、`parse_loc`、`create_child`、`root`、`add_buff`、`unit_sorts`、`post_new_target`、`caster`、`event`、`is_persist`、`logfail`、`set_target`、`cast`、`set_origin`、`calc_target`、`set_creator`、`execute_child_on`、`search`、`__tostring`、`level_data`、`sequence`、`set_damage_modifiers`、`loop`、`creator_player`、`is_root`、`item`、`get_snapshot_attribute`、`apply_mover`、`loop_clear_up`、`set_var_point`、`missile_detach`、`sequence_clear_up`、`set`、`origin`、`get_level`、`var_unit`、`get_scene`、`snap_shot_values`、`set_var_unit`、`item_random`、`on_response`、`main_target`、`create_actor`、`set_cache`、…（共 70 个，全量见 `parsed/fields` 对应 JSON） |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`EffectParamShared.prototype.get_snapshot_attribute`、`EffectParam.prototype.create_unit`、`EffectParam.prototype.get_creation_param`、`EffectParam.prototype.is_persist`、`EffectParam.prototype.__tostring`、`EffectParam.prototype.sequence`、`EffectParam.prototype.get_snapshot_attribute`、`EffectParam.prototype.apply_mover`、`EffectParam.prototype.sequence_clear_up`、`EffectParam.prototype.on_response`

### `@common/base/effect`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.effect'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

特效实例相关（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/event`

- 来源：script 包（common 库）（`script\199\common\base\event.lua`）
- 加载：`require 'base.event'`（init.lua:73）；`require 'base.event'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

事件系统：`base.event_register/notify/dispatch`、跨端事件序列化、预设 TS 事件参数类（`base.单位进入视野` 等）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.assign_event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `base.forward_event_register` | `(name)` | name |  |  | 源码实锤 |
| `base.event_dispatch` | `(obj, name, ...)` | obj, name, ... |  |  | 源码实锤 |
| `is_ts_class_metatable` | `(c)` | c |  |  | 源码实锤 |
| `base.event_serialize` | `(t, depth, event_name)` | t, depth, event_name |  |  | 源码实锤 |
| `base.event_deserialize` | `(t)` | t |  |  | 源码实锤 |
| `__client_event_to_server` | `(obj, name, ...)` | obj, name, ... |  |  | 源码实锤 |
| `base.event_notify` | `(obj, name, ...)` | obj, name, ... |  |  | 源码实锤 |
| `base.event_register` | `(obj, name, f)` | obj, name, f |  |  | 源码实锤 |
| `base.game:event_dispatch` | `(name, ...)` | name, ... |  |  | 源码实锤 |
| `base.game:event_notify` | `(name, ...)` | name, ... |  |  | 源码实锤 |
| `base.game:event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `base.game:broadcast` | `(name, f)` | name, f |  |  | 源码实锤 |
| `base.custom_event_notify` | `(event_name, event_param)` | event_name, event_param |  |  | 源码实锤 |
| `base.send_custom_event` | `(event)` | event |  | 触发V2用 | 源码实锤 |
| `TriggerEvent.prototype.____constructor` | `(self, obj, event_name, periodic, time)` | self, obj, event_name, periodic, time |  |  | 源码实锤 |
| `base.单位进入视野.prototype.____constructor` | `(self, obj, evt_name, unit)` | self, obj, evt_name, unit |  |  | 源码实锤 |
| `base.消息技能.prototype.____constructor` | `(self, obj, evt_name, msg)` | self, obj, evt_name, msg |  |  | 源码实锤 |
| `base.场景加载完成.prototype.____constructor` | `(self, obj, evt_name, scene_name)` | self, obj, evt_name, scene_name |  |  | 源码实锤 |
| `base.消息错误.prototype.____constructor` | `(self, obj, evt_name, msg, duration)` | self, obj, evt_name, msg, duration |  |  | 源码实锤 |
| `base.消息聊天.prototype.____constructor` | `(self, obj, evt_name, player, duration)` | self, obj, evt_name, player, duration |  |  | 源码实锤 |
| `base.消息公告.prototype.____constructor` | `(self, obj, evt_name, msg, duration)` | self, obj, evt_name, msg, duration |  |  | 源码实锤 |
| `base.画面分辨率变化.prototype.____constructor` | `(self, obj, evt_name, width, height)` | self, obj, evt_name, width, height |  |  | 源码实锤 |
| `base.游戏阶段切换.prototype.____constructor` | `(self, obj, evt_name)` | self, obj, evt_name |  |  | 源码实锤 |
| `base.游戏更新.prototype.____constructor` | `(self, obj, evt_name, delta)` | self, obj, evt_name, delta |  |  | 源码实锤 |
| `base.玩家重连.prototype.____constructor` | `(self, obj, evt_name, player)` | self, obj, evt_name, player |  |  | 源码实锤 |
| `base.游戏属性变化.prototype.____constructor` | `(self, obj, evt_name, property, value_s)` | self, obj, evt_name, property, value_s |  |  | 源码实锤 |
| `base.游戏开始.prototype.____constructor` | `(self, obj, evt_name)` | self, obj, evt_name |  |  | 源码实锤 |
| `base.游戏结束.prototype.____constructor` | `(self, obj, evt_name)` | self, obj, evt_name |  |  | 源码实锤 |
| `base.玩家断线.prototype.____constructor` | `(self, obj, evt_name, player)` | self, obj, evt_name, player |  |  | 源码实锤 |
| `base.画面分辨率缩放变化.prototype.____constructor` | `(self, obj, evt_name, scale)` | self, obj, evt_name, scale |  |  | 源码实锤 |
| `base.按键松开.prototype.____constructor` | `(self, obj, evt_name, key_keyboard)` | self, obj, evt_name, key_keyboard |  |  | 源码实锤 |
| `base.对话选择.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link)` | self, obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link |  |  | 源码实锤 |
| `base.对话开始.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` | self, obj, evt_name, speaker, listener, ref_param, conversation_link |  |  | 源码实锤 |
| `base.鼠标点击物品栏中物品.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` | self, obj, item, item_tooltip_panel, slot_panel, inventory_panel |  |  | 源码实锤 |
| `base.鼠标长按物品栏中物品.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` | self, obj, item, item_tooltip_panel, slot_panel, inventory_panel |  |  | 源码实锤 |
| `base.鼠标长按物品栏中物品抬起.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` | self, obj, item, item_tooltip_panel, slot_panel, inventory_panel |  |  | 源码实锤 |
| `base.对话跳过时.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` | self, obj, evt_name, speaker, listener, ref_param, conversation_link |  |  | 源码实锤 |
| `base.对话结束时.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` | self, obj, evt_name, speaker, listener, ref_param, conversation_link |  |  | 源码实锤 |
| `base.按键按下.prototype.____constructor` | `(self, obj, evt_name, key_keyboard)` | self, obj, evt_name, key_keyboard |  |  | 源码实锤 |
| `base.表现音效事件.prototype.____constructor` | `(self, obj, evt_name, msg, actor)` | self, obj, evt_name, msg, actor |  |  | 源码实锤 |
| `base.表现动画事件开始.prototype.____constructor` | `(self, obj, evt_name, actor, msg, anmi)` | self, obj, evt_name, actor, msg, anmi |  |  | 源码实锤 |
| `base.鼠标按下.prototype.____constructor` | `(self, obj, evt_name, key)` | self, obj, evt_name, key |  |  | 源码实锤 |
| `base.表现动画事件结束.prototype.____constructor` | `(self, obj, evt_name, anmi, msg, actor)` | self, obj, evt_name, anmi, msg, actor |  |  | 源码实锤 |
| `base.鼠标松开.prototype.____constructor` | `(self, obj, evt_name, key)` | self, obj, evt_name, key |  |  | 源码实锤 |
| `base.鼠标移动.prototype.____constructor` | `(self, obj, evt_name)` | self, obj, evt_name |  |  | 源码实锤 |
| `base.服务器请求切换场景.prototype.____constructor` | `(self, obj, old_scene, new_scene)` | self, obj, old_scene, new_scene |  |  | 源码实锤 |
| `base.玩家属性变化.prototype.____constructor` | `(self, obj, evt_name, player, property, value_n, value_s)` | self, obj, evt_name, player, property, value_n, value_s |  |  | 源码实锤 |
| `base.玩家改变英雄.prototype.____constructor` | `(self, obj, evt_name, player, unit)` | self, obj, evt_name, player, unit |  |  | 源码实锤 |
| `base.单位施法完成.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` | self, obj, evt_name, unit, skill_id, time_elapsed, time_total |  |  | 源码实锤 |
| `base.单位施法出手.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` | self, obj, evt_name, unit, skill_id, time_elapsed, time_total |  |  | 源码实锤 |
| `base.单位施法停止.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` | self, obj, evt_name, unit, skill_id, time_elapsed, time_total |  |  | 源码实锤 |
| `base.单位失去状态.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` | self, obj, evt_name, unit, buff |  |  | 源码实锤 |
| `base.单位获得状态.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` | self, obj, evt_name, unit, buff |  |  | 源码实锤 |
| `base.单位状态层数变化.prototype.____constructor` | `(self, obj, evt_name, buff, stack, unit)` | self, obj, evt_name, buff, stack, unit |  |  | 源码实锤 |
| `base.单位施法引导.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` | self, obj, evt_name, unit, skill_id, time_elapsed, time_total |  |  | 源码实锤 |
| `base.单位属性变化.prototype.____constructor` | `(self, obj, evt_name, unit, property, value_n, value_s)` | self, obj, evt_name, unit, property, value_n, value_s |  |  | 源码实锤 |
| `base.单位离开视野.prototype.____constructor` | `(self, obj, evt_name, unit)` | self, obj, evt_name, unit |  |  | 源码实锤 |
| `base.单位施法开始.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` | self, obj, evt_name, unit, skill_id, time_elapsed, time_total |  |  | 源码实锤 |
| `base.单位选中.prototype.____constructor` | `(self, obj, evt_name, player, unit)` | self, obj, evt_name, player, unit |  |  | 源码实锤 |
| `base.单位取消选中.prototype.____constructor` | `(self, obj, evt_name, player, unit)` | self, obj, evt_name, player, unit |  |  | 源码实锤 |
| `base.玩家改变队伍.prototype.____constructor` | `(self, obj, evt_name, player, team)` | self, obj, evt_name, player, team |  |  | 源码实锤 |
| `base.技能获得.prototype.____constructor` | `(self, obj, evt_name, unit, skill)` | self, obj, evt_name, unit, skill |  |  | 源码实锤 |
| `base.技能属性变化.prototype.____constructor` | `(self, obj, evt_name, skill, property, value_n)` | self, obj, evt_name, skill, property, value_n |  |  | 源码实锤 |
| `base.技能充能激活.prototype.____constructor` | `(self, obj, evt_name, skill, time_remaining, time_total)` | self, obj, evt_name, skill, time_remaining, time_total |  |  | 源码实锤 |
| `base.技能冷却激活.prototype.____constructor` | `(self, obj, evt_name, skill, time_remaining, time_total)` | self, obj, evt_name, skill, time_remaining, time_total |  |  | 源码实锤 |
| `base.状态获得.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` | self, obj, evt_name, unit, buff |  |  | 源码实锤 |
| `base.状态层数变化.prototype.____constructor` | `(self, obj, evt_name, buff, stack, unit)` | self, obj, evt_name, buff, stack, unit |  |  | 源码实锤 |
| `base.状态失去.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` | self, obj, evt_name, unit, buff |  |  | 源码实锤 |
| `base.技能失去.prototype.____constructor` | `(self, obj, evt_name, unit, skill)` | self, obj, evt_name, unit, skill |  |  | 源码实锤 |
| `base.技能冷却完成.prototype.____constructor` | `(self, obj, evt_name, skill)` | self, obj, evt_name, skill |  |  | 源码实锤 |
| `base.技能可用状态变化.prototype.____constructor` | `(self, obj, evt_name, skill)` | self, obj, evt_name, skill |  |  | 源码实锤 |
| `base.技能等级变化.prototype.____constructor` | `(self, obj, evt_name, skill, level)` | self, obj, evt_name, skill, level |  |  | 源码实锤 |
| `base.技能学习状态变化.prototype.____constructor` | `(self, obj, evt_name, skill)` | self, obj, evt_name, skill |  |  | 源码实锤 |
| `base.技能层数变化.prototype.____constructor` | `(self, obj, evt_name, skill, stack)` | self, obj, evt_name, skill, stack |  |  | 源码实锤 |
| `base.技能槽位变化.prototype.____constructor` | `(self, obj, evt_name, skill)` | self, obj, evt_name, skill |  |  | 源码实锤 |
| `base.玩家暂时离开.prototype.____constructor` | `(self, obj, evt_name, player)` | self, obj, evt_name, player |  |  | 源码实锤 |
| `base.玩家回到游戏.prototype.____constructor` | `(self, obj, evt_name, player)` | self, obj, evt_name, player |  |  | 源码实锤 |
| `base.单位失去物品.prototype.____constructor` | `(self, obj, evt_name, player, item, drop_mode)` | self, obj, evt_name, player, item, drop_mode |  |  | 源码实锤 |
| `base.单位获得物品.prototype.____constructor` | `(self, obj, evt_name, player, item)` | self, obj, evt_name, player, item |  |  | 源码实锤 |
| `base.联合场景区域通知.prototype.____constructor` | `(self, obj, evt_name, from_scene, from_area, to_scene, to_area)` | self, obj, evt_name, from_scene, from_area, to_scene, to_area |  |  | 源码实锤 |
| `base.联合场景跨越区域.prototype.____constructor` | `(self, obj, evt_name, from_scene, from_area, to_scene, to_area)` | self, obj, evt_name, from_scene, from_area, to_scene, to_area |  |  | 源码实锤 |
| `base.联合场景进入区域.prototype.____constructor` | `(self, obj, evt_name, scene, area, target_scene)` | self, obj, evt_name, scene, area, target_scene |  |  | 源码实锤 |
| `base.联合场景离开区域.prototype.____constructor` | `(self, obj, evt_name, scene, area, target_scene)` | self, obj, evt_name, scene, area, target_scene |  |  | 源码实锤 |
| `base.建造预放置开始.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` | self, obj, evt_name, owner, skill, spellbuild_unit_actor |  |  | 源码实锤 |
| `base.建造预放置取消.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` | self, obj, evt_name, owner, skill, spellbuild_unit_actor |  |  | 源码实锤 |
| `base.建造预放置确认.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` | self, obj, evt_name, owner, skill, spellbuild_unit_actor |  |  | 源码实锤 |
| `base.消息提示显示时.prototype.____constructor` | `(self, obj, evt_name, toast, text, source)` | self, obj, evt_name, toast, text, source |  |  | 源码实锤 |
| `base.菜单栏按钮按下时.prototype.____constructor` | `(self, obj, evt_name, Key)` | self, obj, evt_name, Key |  |  | 源码实锤 |
| `base.初始化好友列表.prototype.____constructor` | `(self, obj, evt_name, friend_data_list)` | self, obj, evt_name, friend_data_list |  |  | 源码实锤 |
| `base.初始化好友申请列表.prototype.____constructor` | `(self, obj, evt_name, friend_apply_data_list)` | self, obj, evt_name, friend_apply_data_list |  |  | 源码实锤 |
| `base.申请列表状态变化.prototype.____constructor` | `(self, obj, evt_name, friend_apply_data)` | self, obj, evt_name, friend_apply_data |  |  | 源码实锤 |
| `base.join_middle_game` | `(middle_game_key)` | middle_game_key |  |  | 源码实锤 |
| `base.send_add_friend` | `(user_id)` | user_id |  |  | 源码实锤 |
| `base.send_agree_add` | `(user_id)` | user_id |  |  | 源码实锤 |
| `base.send_refuse_add` | `(user_id)` | user_id |  |  | 源码实锤 |

### `@common/base/event_deque`

- 来源：script 包（common 库）（`script\199\common\base\event_deque.lua`）
- 加载：`require 'base.event_deque'`（init.lua:89）；`require 'base.event_deque'`
- 状态：🔀 转发桩（`return require '@base.base.event_deque'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 2 键，函数 2，类 0）

事件队列（桩 → client_base；dump 揭示 create_event_queue/create_event_deque）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `create_event_queue` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `create_event_deque` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |

### `@common/base/exception`

- 来源：script 包（common 库）（`script\199\common\base\exception.lua`）
- 加载：`require 'base.exception'`（init.lua:91）；`require 'base.exception'`
- 状态：🔀 转发桩（`return require '@base.base.exception'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 3 键，函数 12，类 0）

异常对象（桩 → client_base；dump 揭示 throw/to_exception + Exception 旧式类）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `throw` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `to_exception` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.ctor` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.__tostring` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception._to_string_to_t` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.to_string` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.new` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.set_traceback` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.set_previous_exception` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception._make` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.make` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `Exception.class_name` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（3 键） |  | dump 实锤 |
| `Exception` | table（16 键） |  | dump 实锤 |
| `Exception.__supper_map` | table（1 键） |  | dump 实锤 |

### `@common/base/fish`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.fish'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

钓鱼玩法钩子（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/force`

- 来源：script 包（common 库）（`script\199\common\base\force.lua`）
- 加载：`require 'base.force'`（init.lua:116，非 app 平台）；`require 'base.force'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

势力（玩家组）：`base.force(list)` 可调用表 + 预建 all/computer/user/team 分组。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:insert` | `(player)` | player |  |  | 源码实锤 |
| `mt:remove` | `(player)` | player |  |  | 源码实锤 |
| `mt:has` | `(player)` | player |  |  | 源码实锤 |
| `mt:len` | `()` |  |  |  | 源码实锤 |
| `mt:random` | `()` |  |  |  | 源码实锤 |
| `mt:ipairs` | `()` |  |  |  | 源码实锤 |
| `mt:clear` | `()` |  |  |  | 源码实锤 |
| `base.force:__call` | `(list)` | list |  |  | 源码实锤 |
| `init` | `()` |  |  |  | 源码实锤 |

### `@common/base/force_movement`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.force_movement'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

强制位移（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/friend`

- 来源：script 包（common 库）（`script\199\common\base\friend.lua`）
- 加载：`require 'base.friend'`（init.lua:97）；`require 'base.friend'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

局内好友：申请/同意/拒绝发送 + 好友列表 proto 转事件。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.friend.send_add_friend` | `(user_id)` | user_id |  |  | 源码实锤 |
| `base.friend.send_agree_add` | `(user_id)` | user_id |  | 同意好友申请 | 源码实锤 |
| `base.friend.send_refuse_add` | `(user_id)` | user_id |  | 拒绝好友申请 | 源码实锤 |
| `base.proto.InGame_S2C_init_friend_list` | `(data)` | data |  | 好友列表 | 源码实锤 |
| `base.proto.InGame_S2C_init_friend_apply_list` | `(data)` | data |  | 好友申请列表 | 源码实锤 |
| `base.proto.InGame_S2C_notice_friend_state` | `(data)` | data |  | 申请列表状态变化 | 源码实锤 |
| `base.proto.InGame_S2C_friend_apply_fail` | `(data)` | data |  | 添加好友失败 | 源码实锤 |

### `@common/base/game`

- 来源：script 包（common 库）（`script\199\common\base\game.lua`）
- 加载：`include 'base.game'`（init.lua:82）；`require 'base.game'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

游戏实例主体（include 型）：输入事件转发、选择器、镜头、广播、`base.event.on_*` 引擎事件桥。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.game:__tostring` | `()` |  |  |  | 源码实锤 |
| `init_scene_name_map` | `()` |  |  |  | 源码实锤 |
| `base.get_scene_name_by_hash` | `(hash)` | hash | string |  | 源码实锤 |
| `base.get_scene_hash_by_name` | `(name)` | name | integer |  | 源码实锤 |
| `base.game:hotkey` | `()` |  |  | 方法 | 源码实锤 |
| `base.game:key_state` | `(key)` | key |  |  | 源码实锤 |
| `base.game:selected_unit` | `()` |  |  |  | 源码实锤 |
| `base.game:chat` | `(type, msg)` | type, msg |  |  | 源码实锤 |
| `base.game:show_timer` | `()` |  |  |  | 源码实锤 |
| `base.game:set_game_scene` | `(...)` | ... |  |  | 源码实锤 |
| `base.game:get_current_scene` | `()` |  |  |  | 源码实锤 |
| `base.game:lock_camera` | `()` |  |  |  | 源码实锤 |
| `base.game:unlock_camera` | `()` |  |  |  | 源码实锤 |
| `base.game:set_camera_attribute` | `(key, value, time)` | key, value, time |  |  | 源码实锤 |
| `base.game:input_mouse` | `()` |  |  |  | 源码实锤 |
| `base.game:loading_left` | `()` |  |  |  | 源码实锤 |
| `base.game:select_unit` | `(unit)` | unit |  |  | 源码实锤 |
| `base.game:circle_selector` | `(pos, radius, tag, ignore_center_pos)` | pos, radius, tag, ignore_center_pos |  |  | 源码实锤 |
| `base.game:line_selector` | `(pos, length, width, face, tag)` | pos, length, width, face, tag |  |  | 源码实锤 |
| `base.game:sector_selector` | `(pos, radius, degree, face, tag)` | pos, radius, degree, face, tag |  |  | 源码实锤 |
| `base.game:get_winner` | `()` |  |  |  | 源码实锤 |
| `base.game:get_winner_team` | `()` |  |  |  | 源码实锤 |
| `base.game:send_broadcast` | `(...)` | ... |  |  | 源码实锤 |
| `base.game:camera_focus` | `(unit)` | unit |  |  | 源码实锤 |
| `base.game.get_default_unit` | `(node_mark)` | node_mark |  | 客户端从服务器获取默认地编单位 | 源码实锤 |
| `base.game.object_store_value` | `(object, key, value)` | object, key, value |  |  | 源码实锤 |
| `base.game.object_restore_value` | `(object, key)` | object, key |  |  | 源码实锤 |
| `base.event.on_spell_cast_result` | `(msg)` | msg |  |  | 源码实锤 |
| `base.event.on_error_tip` | `(msg, time)` | msg, time |  |  | 源码实锤 |
| `base.event.on_system_message` | `(msg, type, time)` | msg, type, time |  |  | 源码实锤 |
| `base.event.on_notify_chat_message` | `(player_slot_id, type, msg, time)` | player_slot_id, type, msg, time |  |  | 源码实锤 |
| `base.event.on_unit_clicked` | `(id)` | id |  |  | 源码实锤 |
| `base.event.on_control_spell_assist` | `(control, spell_id, type, shape, range, width, plane_range, id)` | control, spell_id, type, shape, range, width, plane_range, id |  |  | 源码实锤 |
| `base.event.on_move_spell_assist` | `()` |  |  | [[ | 源码实锤 |
| `base.event.on_spell_assist_update` | `(spell_id , time, id)` | spell_id , time, id |  |  | 源码实锤 |
| `base.event.on_game_will_enter_foreground` | `()` |  |  |  | 源码实锤 |
| `base.event.on_game_enter_foreground` | `()` |  |  |  | 源码实锤 |
| `base.event.on_game_enter_background` | `()` |  |  |  | 源码实锤 |
| `base.event.on_click` | `(screen_pos, actorsID, button)` | screen_pos, actorsID, button |  |  | 源码实锤 |
| `key_down` | `(key)` | key |  |  | 源码实锤 |
| `key_up` | `(key)` | key |  |  | 源码实锤 |
| `update_key_state` | `(key, count)` | key, count |  |  | 源码实锤 |
| `base.event.on_key_down` | `(unkey)` | unkey |  |  | 源码实锤 |
| `base.event.on_key_up` | `(unkey)` | unkey |  |  | 源码实锤 |
| `base.event.on_mouse_down` | `(button_type)` | button_type |  |  | 源码实锤 |
| `base.event.on_mouse_up` | `(button_type)` | button_type |  |  | 源码实锤 |
| `base.event.on_mouse_move` | `()` |  |  |  | 源码实锤 |
| `base.event.on_wheel_move` | `(delta_wheel)` | delta_wheel |  |  | 源码实锤 |
| `base.event.on_joystick_button_down` | `(button_name)` | button_name |  |  | 源码实锤 |
| `base.event.on_joystick_button_up` | `(button_name)` | button_name |  |  | 源码实锤 |
| `base.event.on_joystick_axis_move` | `(axis_name, position)` | axis_name, position |  |  | 源码实锤 |
| `base.event.on_joystick_hat_move` | `(state)` | state |  |  | 源码实锤 |
| `base.event.on_start_loading` | `(time)` | time |  |  | 源码实锤 |
| `base.event.on_enter_game` | `()` |  |  |  | 源码实锤 |
| `base.event.on_replay_stopped` | `()` |  |  |  | 源码实锤 |
| `base.event.on_game_result` | `(json)` | json |  |  | 源码实锤 |
| `base.event.on_load_scene` | `(scene_name)` | scene_name |  |  | 源码实锤 |
| `base.event.on_load_scene_over` | `(scene_name)` | scene_name |  |  | 源码实锤 |
| `base.event.on_combined_scene_area_notify` | `(...)` | ... |  |  | 源码实锤 |
| `base.event.on_game_setting_changed` | `()` |  |  |  | 源码实锤 |
| `base.event.on_create_riseletter_failed` | `(riselettertype,templatename)` | riselettertype,templatename |  |  | 源码实锤 |
| `base.event.on_game_start` | `(...)` | ... |  | function param: map_name, map_kind, session_id, background_loading | 源码实锤 |
| `base.event.on_game_loading` | `(content, percent)` | content, percent |  |  | 源码实锤 |
| `base.event.on_game_started` | `(...)` | ... |  | function param: map_name, map_kind, session_id, background_loading | 源码实锤 |
| `base.event.on_game_exit` | `(map_name, map_kind, session_id, ...)` | map_name, map_kind, session_id, ... |  | function param: map_name, map_kind, session_id, background_loading | 源码实锤 |
| `base.event.on_game_kick` | `(...)` | ... |  |  | 源码实锤 |
| `base.event.on_game_reconnected` | `(...)` | ... |  |  | 源码实锤 |
| `base.event.on_url_launch` | `(map_name)` | map_name |  |  | 源码实锤 |
| `base.event.on_file_changed` | `(file_path, file_name, change_list)` | file_path, file_name, change_list |  | 监测文件夹是否变化 | 源码实锤 |
| `base.event.on_broadcast` | `(args)` | args |  |  | 源码实锤 |
| `base.event.on_sync_custom_game_attribute` | `(key, value)` | key, value |  |  | 源码实锤 |
| `base.get_game_attribute` | `(key)` | key |  |  | 源码实锤 |
| `base.event.on_actor_event` | `(actor_id, msg, anim, start)` | actor_id, msg, anim, start |  |  | 源码实锤 |
| `base.event.on_game_time_pause` | `()` |  |  |  | 源码实锤 |
| `base.event.on_game_time_resume` | `()` |  |  |  | 源码实锤 |
| `base.event.on_actor_destroy` | `(actor_id)` | actor_id |  |  | 源码实锤 |
| `base.event.on_debug_cheat` | `(cheat_codes)` | cheat_codes |  |  | 源码实锤 |
| `base.event.on_actor_finish_animation` | `(actor_id, anim, operation)` | actor_id, anim, operation |  |  | 源码实锤 |
| `base.event.on_unit_finish_animation` | `(unit_id, anim, operation)` | unit_id, anim, operation |  |  | 源码实锤 |
| `base.event.on_game_sync_unit_attribute_config` | `(attribute_config)` | attribute_config |  |  | 源码实锤 |
| `base.game:on_kick` | `(msg)` | msg |  |  | 源码实锤 |
| `base.game.create_debug_draw_actor` | `()` |  |  |  | 源码实锤 |
| `base.game.debug_draw_point` | `(actor, point, color)` | actor, point, color |  |  | 源码实锤 |
| `base.game.debug_draw_circle` | `(actor, point, euler_alpha, euler_beta, euler_gamma, radius, color, solid)` | actor, point, euler_alpha, euler_beta, euler_gamma, radius, color, solid |  |  | 源码实锤 |
| `base.game.debug_draw_line` | `(actor, s_point, e_point, color)` | actor, s_point, e_point, color |  |  | 源码实锤 |
| `base.game.debug_draw_sector` | `(actor, point, euler_alpha, euler_beta, euler_gamma, radius, angle, color, solid)` | actor, point, euler_alpha, euler_beta, euler_gamma, radius, angle, color, solid |  |  | 源码实锤 |
| `base.game.debug_draw_text` | `(actor, point, text, color, displayTop)` | actor, point, text, color, displayTop |  |  | 源码实锤 |
| `base.game.debug_draw_rectangle` | `(actor, v_point, w_point, h_point, color, solid)` | actor, v_point, w_point, h_point, color, solid |  |  | 源码实锤 |
| `base.game.clear_debug_draws` | `(actor)` | actor |  |  | 源码实锤 |
| `base.get_current_fps` | `()` |  |  |  | 源码实锤 |
| `base.get_current_ping` | `()` |  |  |  | 源码实锤 |
| `base.set_use_right_click_move` | `(use)` | use |  |  | 源码实锤 |
| `base.get_use_right_click_move` | `()` |  |  |  | 源码实锤 |
| `base.raycast_unit_at_screen_xy` | `(x, y)` | x, y |  |  | 源码实锤 |
| `base.get_units_from_rect` | `(point, width, height, face)` | point, width, height, face |  | 获取矩形区域内的所有单位  返回单位数组 | 源码实锤 |
| `base.get_units_from_sector` | `(point,radius,arc,face)` | point,radius,arc,face |  | 获取扇形区域内的所有单位  返回单位数组 | 源码实锤 |
| `base.game.load_combined_map` | `(scene, direction)` | scene, direction |  | 显示拼接场景 | 源码实锤 |
| `base.game.purge_combined_map` | `()` |  |  | 释放拼接场景 | 源码实锤 |
| `base.game.load_combined_map_deco` | `(scene, direction)` | scene, direction |  | 创建拼接场景通行模型 | 源码实锤 |
| `base.game.purge_combined_map_deco` | `()` |  |  | 释放拼接场景通行模型 | 源码实锤 |
| `base.game.load_scene_cache_and_combined` | `(scene, direction)` | scene, direction |  |  | 源码实锤 |
| `AnimPointInfo.prototype.____constructor` | `(self, tbl)` | self, tbl |  |  | 源码实锤 |
| `base.game.get_model_anim_point_info` | `(model_path, anim_name)` | model_path, anim_name |  | 给触发用的api，用ts类包了一层 | 源码实锤 |
| `base.get_obj_items` | `()` |  | table |  | 源码实锤 |
| `base.get_all_skills_id` | `()` |  | table |  | 源码实锤 |
| `base.get_all_buffs_id` | `()` |  | table |  | 源码实锤 |
| `base.get_all_units_id` | `()` |  | table |  | 源码实锤 |
| `base.game_shortcut` | `()` |  |  | 创建游戏快捷方式 | 源码实锤 |
| `base.shallow_copy` | `(tbl)` | tbl | table |  | 源码实锤 |
| `base.set_cursor_shape` | `(path)` | path |  |  | 源码实锤 |
| `base.use_system_cursor` | `()` |  |  |  | 源码实锤 |
| `base.get_ground_z` | `(x, y, bool)` | x, y, bool |  |  | 源码实锤 |
| `base.get_ground_z_from_point` | `(point, bool)` | point, bool |  |  | 源码实锤 |
| `init_gameplay` | `()` |  |  |  | 源码实锤 |
| `base.get_platform` | `()` |  |  |  | 源码实锤 |
| `base.get_platform_is_app` | `()` |  |  |  | 源码实锤 |
| `base.start_game` | `(map_name, is_to_test)` | map_name, is_to_test |  |  | 源码实锤 |
| `base.game.set_dynamic_point_light` | `(val)` | val |  |  | 源码实锤 |

### `@common/base/gameplay`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.gameplay'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

玩法配置相关（命名语义推测；数编 gameplay 节点由 eff.cache 消费）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/group`

- 来源：script 包（common 库）（`script\199\common\base\group.lua`）
- 加载：`require 'base.group'`（init.lua:115，非 app 平台）；`require 'base.group'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

弱引用对象集合 `base.group(list?)`（被 force 复用）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:insert` | `(obj)` | obj |  |  | 源码实锤 |
| `mt:remove` | `(obj)` | obj |  |  | 源码实锤 |
| `mt:has` | `(obj)` | obj |  |  | 源码实锤 |
| `mt:len` | `()` |  |  |  | 源码实锤 |
| `mt:random` | `()` |  |  |  | 源码实锤 |
| `mt:ipairs` | `()` |  |  |  | 源码实锤 |
| `mt:clear` | `()` |  |  |  | 源码实锤 |
| `base.group` | `(list)` | list |  |  | 源码实锤 |

### `@common/base/hashtable`

- 来源：script 包（common 库）（`script\199\common\base\hashtable.lua`）
- 加载：`require 'base.hashtable'`（init.lua:117，非 app 平台）；`require 'base.hashtable'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

编辑器哈希表物件（k1/k2 两级弱键表，带类型检查）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `new_wk` | `()` |  |  |  | 源码实锤 |
| `mt:save` | `(k1, k2, tp, value)` | k1, k2, tp, value |  | tp为编辑器中的值类型，在编译时生成 | 源码实锤 |
| `mt:load` | `(k1, k2, tp, def)` | k1, k2, tp, def |  | tp为编辑器中的值类型，def为该类型的默认值，在编译时生成 | 源码实锤 |
| `mt:flush` | `()` |  |  |  | 源码实锤 |
| `mt:flush_parent` | `(k1)` | k1 |  |  | 源码实锤 |
| `mt:flush_child` | `(k1, k2)` | k1, k2 |  |  | 源码实锤 |
| `base.hashtable` | `()` |  |  |  | 源码实锤 |

### `@common/base/heal`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.heal'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 6，类 1）

治疗实例 `HealInstance` 类（TS）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `HealInstance.prototype.is_skill` | `(?)` |  |  |  | dump 实锤 |
| `HealInstance.prototype.mul` | `(?)` |  |  |  | dump 实锤 |
| `HealInstance.prototype.dispatch` | `(?)` |  |  |  | dump 实锤 |
| `HealInstance.prototype.get_current_heal` | `(?)` |  |  |  | dump 实锤 |
| `HealInstance.prototype.get_heal` | `(?)` |  |  |  | dump 实锤 |
| `HealInstance.prototype.div` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `HealInstance` | TSTL 类（2 键） |  | dump 实锤 |
| `HealInstance.prototype` | table（12 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `HealInstance` | 无 | `is_skill`、`mul`、`dispatch`、`get_current_heal`、`get_heal`、`div` |

### `@common/base/inventory`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.inventory'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 2 键，函数 18，类 2）

物品栏容器：`Inventory`/`Slot` 两个 TS 类（服务端物品栏逻辑）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `Slot.prototype.met_requirement` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.unlock` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.can_absorb` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.can_hold` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.new` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.assign` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.absorb` | `(?)` |  |  |  | dump 实锤 |
| `Slot.prototype.lock` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.add_item` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.debuginfo` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.unlock` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.can_hold` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.set_state` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.get_slot` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.add_slot` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.lock` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.remove` | `(?)` |  |  |  | dump 实锤 |
| `Inventory.prototype.new` | `(?)` |  |  |  | dump 实锤 |

**调用点反查 API 形态**（参数以实际调用为准）【反查推测】：

- `new('$$lib_promotion1_inventory.item_container.升变2装备栏.root', ...)`
- `new('$$lib_promotion2_inventory.item_container.升变2装备栏.root', ...)`

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |
| `Slot` | TSTL 类（2 键） |  | dump 实锤 |
| `Slot.prototype` | table（11 键） |  | dump 实锤 |
| `Inventory` | TSTL 类（2 键） |  | dump 实锤 |
| `Inventory.prototype` | table（13 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Slot` | 无 | `met_requirement`、`unlock`、`can_absorb`、`can_hold`、`new`、`assign`、`absorb`、`lock` |
| `Inventory` | 无 | `add_item`、`debuginfo`、`unlock`、`can_hold`、`set_state`、`get_slot`、`add_slot`、`lock`、`remove`、`new` |

### `@common/base/isolation`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.isolation'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

StateGame 沙箱阉割（base 侧版本未随包分发；common 根 isolation.lua 见 common-root 组）。

**说明**：base 侧 isolation 未随包分发（脚本包内只有 common 根级 `isolation.lua`，见 common-root 组）。本键值 `true` 表明 StateGame 启动时执行过名为 `base.isolation` 的模块（引擎内嵌或未分发文件）【语义推测】。isolation 机制本体（StateGame 下禁用 io/os/debug 等）见 sce-lib-script-199 知识库。

### `@common/base/item`

- 来源：script 包（common 库）（`script\199\common\base\item.lua`）
- 加载：`include 'base.item'`（init.lua:121，非 app 平台）；`require 'base.item'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 69，类 1）

物品 Item 类（物品本质是带 sys_item_* 属性的单位）。

**dump 对照差异**：init.lua:121 以 `include` 加载（include 按 KB 不走 package.loaded 缓存），但 dump 中本键值为完整 table（Item 类）——说明 include 实现仍会写 package.loaded，或运行时被其他模块以 `require '@common/base/item'` 再加载一次【反查推测】。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__index` | `(key)` | key |  |  | 源码实锤 |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `base.item` | `(id, silence)` | id, silence |  |  | 源码实锤 |
| `mt:get_owner` | `()` |  |  |  | 源码实锤 |
| `mt:is_in_unit_cooldown` | `()` |  |  |  | 源码实锤 |
| `mt:try_drop` | `(callback)` | callback |  |  | 源码实锤 |
| `mt:get_attr_need` | `()` |  |  |  | 源码实锤 |
| `mt:foreach_attr_need` | `(func)` | func |  |  | 源码实锤 |
| `mt:get_all_extra_mod` | `(is_equip)` | is_equip |  |  | 源码实锤 |
| `mt:get_rand_mod` | `(buff_link, buff_idx, key, percentage)` | buff_link, buff_idx, key, percentage |  |  | 源码实锤 |
| `mt:get_name` | `()` |  |  |  | 源码实锤 |
| `try_load_show_methods` | `()` |  |  |  | 源码实锤 |
| `mt:get_show_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_icon` | `()` |  |  |  | 源码实锤 |
| `mt:get_tips` | `()` |  |  |  | 源码实锤 |
| `mt:get_current_cd` | `()` |  |  |  | 源码实锤 |
| `mt:get_cd_max` | `()` |  |  |  | 源码实锤 |
| `mt:get_stack` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Item` | TSTL 类（2 键） |  | dump 实锤 |
| `Item.prototype` | table（73 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Item` | 无 | `remove_pick_region`、`bind_to_user`、`debuginfo`、`new`、`foreach_extra_attr`、`init_attr_need`、`event_player_hero_item_change`、`update_score_money`、`move`、`get_bind_user`、`add_to`、`remove_extra_mod`、`randomized_value`、`carrier`、`get_attr_need`、`foreach_attr_need`、`add_extra_mod`、`bind_items_to_user`、`get_slot`、`remove`、`update_equip_state`、`event_notify`、`create_to_unit`、`cleat_extra_attr_all`、`mark_as_pending_removal`、`get_name`、`check_score_type`、`is_available`、`sync_slot_data`、`load_score_to_unit`、`set_score_custom_data`、`cleat_extra_attr`、`create_to_point`、`pick_by`、`set_randomized_value`、`create_score_item_for_player`、`get_score_custom_data`、`init_pick_region`、`set_quality`、`drop`、`set_stack`、`add_score_item_commit`、`can_use`、`create_extra_attr_target`、`score_use`、`can_be_equipped`、`has_label`、`get_player_score_item_list`、`__tostring`、`generate_rand_mod`、`is_valid`、`get_inv_index`、`unbind_items_to_user`、`get_all_extra_mod`、`load_item_from_info`、`add_attr_need`、`update_modification`、`get_item_info`、`unbind_to_user`、`save_score_to_unit`、…（共 66 个，全量见 `parsed/fields` 对应 JSON） |

⚠️ **此处被截断，字段不全**：3 处（`<max depth exceeded>`），全部为 `prototype._descriptors.*` 属性访问器（get/set/enumerable/configurable），不损失 API 信息。全量截断路径见 keys_index.json / 对应 fields JSON。

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Item.prototype.remove_pick_region`、`Item.prototype.bind_to_user`、`Item.prototype.debuginfo`、`Item.prototype.new`、`Item.prototype.foreach_extra_attr`、`Item.prototype.init_attr_need`、`Item.prototype.event_player_hero_item_change`、`Item.prototype.update_score_money`、`Item.prototype.move`、`Item.prototype.get_bind_user`、`Item.prototype.add_to`、`Item.prototype.remove_extra_mod`、`Item.prototype.randomized_value`、`Item.prototype.carrier`、`Item.prototype.add_extra_mod`、`Item.prototype.bind_items_to_user`、`Item.prototype.get_slot`、`Item.prototype.remove`、`Item.prototype.update_equip_state`、`Item.prototype.event_notify`、`Item.prototype.create_to_unit`、`Item.prototype.cleat_extra_attr_all`、`Item.prototype.mark_as_pending_removal`、`Item.prototype.check_score_type`、`Item.prototype.is_available`、`Item.prototype.sync_slot_data`、`Item.prototype.load_score_to_unit`、`Item.prototype.set_score_custom_data`、`Item.prototype.cleat_extra_attr`、`Item.prototype.create_to_point`、`Item.prototype.pick_by`、`Item.prototype.set_randomized_value`、`Item.prototype.create_score_item_for_player`、`Item.prototype.get_score_custom_data`、`Item.prototype.init_pick_region`、`Item.prototype.set_quality`、`Item.prototype.drop`、`Item.prototype.set_stack`、`Item.prototype.add_score_item_commit`、`Item.prototype.can_use`、…（共 61 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/json_decode`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.json_decode'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 5 键，函数 3，类 0）

JSON 编解码（dump 实锤 encode/decode/null + 空表哨兵）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `null` | `(?)` |  |  |  | dump 实锤 |
| `encode` | `(?)` |  |  |  | dump 实锤 |
| `decode` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（5 键） |  | dump 实锤 |

### `@common/base/line`

- 来源：script 包（common 库）（`script\199\common\base\line.lua`）
- 加载：`require 'base.line'`（init.lua:78）；`require 'base.line'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 3，类 1）

折线 `Line` 类 + 地编线获取。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:get` | `(i)` | i |  |  | 源码实锤 |
| `mt:get_length` | `()` |  |  |  | 源码实锤 |
| `base.line` | `(points)` | points |  |  | 源码实锤 |
| `base.get_scene_line` | `(scene, area_name, present)` | scene, area_name, present |  | 获取地编线 | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Line` | TSTL 类（2 键） |  | dump 实锤 |
| `Line.prototype` | table（6 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Line` | 无 | `__tostring`、`get`、`get_length` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Line.prototype.__tostring`

### `@common/base/lni`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.lni'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

lni 引擎配置数据格式读取（C++ `lni_loader` 实现，`base.game.lni`）。

**说明**：`base.game.lni = require 'lni_loader'`（base/init.lua:57，注释「lni_loader implement by c++」）【源码实锤】。本键值 `true`；引擎侧另有顶层键 `lni`（stdlib 附录，值为 function）。lni = 引擎配置数据格式（数编/物编底层序列化格式）。

### `@common/base/lni_writer`

- 来源：script 包（common 库）（`script\199\common\base\lni_writer.lua`）
- 加载：被 server.lua:201 `require`（未知消息美化日志）；`require 'base.lni_writer'`
- 状态：✅ 有源码
- dump 值：单个 function（模块返回函数）

table → lni 文本序列化器（模块返回单个函数）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `format_key` | `(name)` | name |  |  | 源码实锤 |
| `format_value` | `(value)` | value |  |  | 源码实锤 |
| `convert_table` | `(tbl)` | tbl |  |  | 源码实锤 |
| `convert_root` | `(root)` | root |  |  | 源码实锤 |

**字段/子表**：模块返回单个函数 `fun(lni): string`（table → lni 文本序列化）【源码实锤 + dump 实锤（值为 function）】。

### `@common/base/load_done`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.load_done'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

资源/场景加载完成回调登记（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/log`

- 来源：script 包（common 库）（`script\199\common\base\log.lua`）
- 加载：`require 'base.log'`（init.lua:1，装配链第一个模块）；`require 'base.log'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

log 增强：给 C++ 预置 `log` 补 format 系列 + `_G.printf`。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `fmt` | `(f, ...)` | f, ... |  |  | 源码实锤 |
| `log.debugf` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `log.infof` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `log.warnf` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `log.errorf` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `log.error` | `(...)` | ... |  |  | 源码实锤 |
| `log.alertf` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `log.fail` | `(info)` | info |  |  | 源码实锤 |
| `log.failf` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `_G.printf` | `(fmt, ...)` | fmt, ... |  |  | 源码实锤 |
| `log.traceback_debug_bp` | `(...)` | ... |  |  | 源码实锤 |

### `@common/base/loot`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.loot'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

掉落（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/loot_pool`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.loot_pool'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

掉落池（命名语义推测；lua_plus 有 loot_pool 包装器）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/lualib_bundle`

- 来源：script 包（common 库）（`script\199\common\base\lualib_bundle.lua`）
- 加载：`base.tsc = require 'base.lualib_bundle'`（init.lua:68）；`require 'base.lualib_bundle'`
- 状态：✅ 有源码
- dump 值：table（顶层 128 键，函数 1425，类 141）

TypeScriptToLua 运行时库（`base.tsc`）：`__TS__Class`/`__TS__Promise`/`__TS__Array*` 全套 + CLASSES 类注册表。

**说明**

TypeScriptToLua 编译产物的运行时支撑库（2854 行）【源码实锤】，赋给 `base.tsc`（init.lua:68）。dump 值树 128 顶层键 / 1425 函数 / **141 个 TSTL 类** / 244 处截断【dump 实锤】：

- 顶层函数即上方函数表 `__TS__*` 系列（源码/dump 双实锤）。
- `CLASSES` 子表是**全库 TS 类注册表**（dump 实锤），按名索引全部 TS 类：AI、AISearcher、Actor、AnimHandle、Array、Buff、Camera、Cast、Channeler、Channeled、Coroutine、DamageInstance、DateTime、HealInstance、Inventory、Item、Line、MatchInfo、Mover、MoverLine、MoverTarget、Player、Point、Quest、QuestCondition、Region、RegionCircle、RegionMargin、RegionRect、Response 族、ScoreCommitter、Score*Param/Data 族（36 个，与 `@common/base/tds_score` 同一份引用）、Skill、Slot、Snapshot、Team、ThirdOrderMatrix、Timer、Trigger、Unit、Vector、TriggerEvent 派生族等（完整 141 名单见 `parsed/fields/common__base__lualib_bundle.json` members.classes）。
- 244 处截断几乎全部是各类 `prototype._descriptors.*` 属性访问器（get/set/enumerable/configurable），**不损失 API 信息**。
- 各 `@common/base/<模块>` 键的类值与 `CLASSES.<类名>` 是**同一表引用**（dump 内联共享标注 also_in 互证）——模块键是类的「按模块归属」视图，CLASSES 是「按名索引」视图。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `__TS__ArrayIsArray` | `(value)` | value |  |  | 源码实锤 |
| `__TS__ArrayClone` | `(self)` | self |  |  | 源码实锤 |
| `__TS__ArrayConcat` | `(self, ...)` | self, ... |  |  | 源码实锤 |
| `__TS__Symbol` | `(description)` | description |  |  | 源码实锤 |
| `__TS__ArrayEntries` | `(array)` | array |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `__TS__ArrayEvery` | `(self, callbackfn, thisArg)` | self, callbackfn, thisArg |  |  | 源码实锤 |
| `__TS__ArrayFilter` | `(self, callbackfn, thisArg)` | self, callbackfn, thisArg |  |  | 源码实锤 |
| `__TS__ArrayForEach` | `(self, callbackFn, thisArg)` | self, callbackFn, thisArg |  |  | 源码实锤 |
| `__TS__ArrayForEachEx` | `(self, callbackFn, thisArg)` | self, callbackFn, thisArg |  |  | 源码实锤 |
| `__TS__ArrayRandom` | `(self)` | self |  |  | 源码实锤 |
| `__TS__ArrayRandoms` | `(self, number, duplicate)` | self, number, duplicate |  |  | 源码实锤 |
| `__TS__ArrayFind` | `(self, predicate, thisArg)` | self, predicate, thisArg |  |  | 源码实锤 |
| `__TS__ArrayFindIndex` | `(self, callbackFn, thisArg)` | self, callbackFn, thisArg |  |  | 源码实锤 |
| `iteratorGeneratorStep` | `(self)` | self |  |  | 源码实锤 |
| `iteratorIteratorStep` | `(self)` | self |  |  | 源码实锤 |
| `iteratorStringStep` | `(self, index)` | self, index |  |  | 源码实锤 |
| `__TS__Iterator` | `(iterable)` | iterable |  |  | 源码实锤 |
| `iteratorGeneratorStep` | `(self)` | self |  |  | 源码实锤 |
| `iteratorIteratorStep` | `(self)` | self |  |  | 源码实锤 |
| `iteratorStringStep` | `(self, index)` | self, index |  |  | 源码实锤 |
| `iteratorLuaTable` | `(self, key)` | self, key |  |  | 源码实锤 |
| `__TS__IteratorMap` | `(iterable)` | iterable |  |  | 源码实锤 |
| `arrayLikeStep` | `(self, index)` | self, index |  |  | 源码实锤 |
| `arrayLikeIterator` | `(arr)` | arr |  |  | 源码实锤 |
| `__TS__ArrayFrom` | `(arrayLike, mapFn, thisArg)` | arrayLike, mapFn, thisArg |  |  | 源码实锤 |
| `__TS__ArrayIncludes` | `(self, searchElement, fromIndex)` | self, searchElement, fromIndex |  |  | 源码实锤 |
| `__TS__ArrayIndexOf` | `(self, searchElement, fromIndex)` | self, searchElement, fromIndex |  |  | 源码实锤 |
| `__TS__ArrayJoin` | `(self, separator)` | self, separator |  |  | 源码实锤 |
| `__TS__ArrayMap` | `(self, callbackfn, thisArg)` | self, callbackfn, thisArg |  |  | 源码实锤 |
| `__TS__ArrayPush` | `(self, ...)` | self, ... |  |  | 源码实锤 |
| `__TS__ArrayPushArray` | `(self, items)` | self, items |  |  | 源码实锤 |
| `__TS__CountVarargs` | `(...)` | ... |  |  | 源码实锤 |
| `__TS__ArrayReduce` | `(self, callbackFn, ...)` | self, callbackFn, ... |  |  | 源码实锤 |
| `__TS__ArrayReduceRight` | `(self, callbackFn, ...)` | self, callbackFn, ... |  |  | 源码实锤 |
| `__TS__ArrayReverse` | `(self)` | self |  |  | 源码实锤 |
| `__TS__ArrayUnshift` | `(self, ...)` | self, ... |  |  | 源码实锤 |
| `__TS__ArraySort` | `(self, compareFn)` | self, compareFn |  |  | 源码实锤 |
| `__TS__ArraySlice` | `(self, first, last)` | self, first, last |  |  | 源码实锤 |
| `__TS__ArraySome` | `(self, callbackfn, thisArg)` | self, callbackfn, thisArg |  |  | 源码实锤 |
| `__TS__ArraySplice` | `(self, ...)` | self, ... |  |  | 源码实锤 |
| `__TS__ArrayToObject` | `(self)` | self |  |  | 源码实锤 |
| `__TS__ArrayFlat` | `(self, depth)` | self, depth |  |  | 源码实锤 |
| `__TS__ArrayFlatMap` | `(self, callback, thisArg)` | self, callback, thisArg |  |  | 源码实锤 |
| `__TS__ArraySetLength` | `(self, length)` | self, length |  |  | 源码实锤 |
| `__TS__TypeReference` | `(typeName, typeArguments)` | typeName, typeArguments |  |  | 源码实锤 |
| `__TS__Keyword` | `(keyword)` | keyword |  |  | 源码实锤 |
| `__TS__UnionType` | `(types)` | types |  |  | 源码实锤 |
| `__TS__Type.__TS__TypeArgumentCheck` | `(currentType, targetType)` | currentType, targetType |  |  | 源码实锤 |
| `__TS__Type.__TS__TypeArgumentListCheck` | `(currentArguments, targetArguments)` | currentArguments, targetArguments |  |  | 源码实锤 |
| `__TS__InstanceOf` | `(obj, classTbl)` | obj, classTbl |  |  | 源码实锤 |
| `__TS__ForceAs` | `(obj, targetTypeReference)` | obj, targetTypeReference |  |  | 源码实锤 |
| `TypeArgumentsFuncWrapper` | `(superTypeArguments, superTargetFunc)` | superTypeArguments, superTargetFunc |  |  | 源码实锤 |
| `__TS__SuperTypeArgumentsFuncWrapper` | `(classTable, currentTypeArguemnts, superTargetFunc)` | classTable, currentTypeArguemnts, superTargetFunc |  |  | 源码实锤 |
| `__TS__New` | `(target, typeArguments, ...)` | target, typeArguments, ... |  |  | 源码实锤 |
| `__TS__Class` | `(self)` | self |  |  | 源码实锤 |
| `__TS__FunctionBind` | `(fn, ...)` | fn, ... |  |  | 源码实锤 |
| `promiseDeferred` | `(self)` | self |  |  | 源码实锤 |
| `isPromiseLike` | `(self, thing)` | self, thing |  |  | 源码实锤 |
| `__TS__Promise.prototype.____constructor` | `(self, executor)` | self, executor |  |  | 源码实锤 |
| `____catch` | `(e)` | e |  |  | 源码实锤 |
| `__TS__Promise.resolve` | `(data)` | data |  |  | 源码实锤 |
| `__TS__Promise.reject` | `(reason)` | reason |  |  | 源码实锤 |
| `__TS__Promise.prototype.catch` | `(self, onRejected)` | self, onRejected |  |  | 源码实锤 |
| `__TS__Promise.prototype.finally` | `(self, onFinally)` | self, onFinally |  |  | 源码实锤 |
| `__TS__Promise.prototype.resolve` | `(self, data)` | self, data |  |  | 源码实锤 |
| `__TS__Promise.prototype.reject` | `(self, reason)` | self, reason |  |  | 源码实锤 |
| `__TS__Promise.prototype.createPromiseResolvingCallback` | `(self, f, resolve, reject)` | self, f, resolve, reject |  |  | 源码实锤 |
| `____catch` | `(e)` | e |  |  | 源码实锤 |
| `__TS__Promise.prototype.handleCallbackData` | `(self, data, resolve, reject)` | self, data, resolve, reject |  |  | 源码实锤 |
| `__TS__AsyncAwaiter` | `(generator)` | generator |  |  | 源码实锤 |
| `adopt` | `(self, value)` | self, value |  |  | 源码实锤 |
| `fulfilled` | `(self, value)` | self, value |  |  | 源码实锤 |
| `step` | `(self, result)` | self, result |  |  | 源码实锤 |
| `__TS__Await` | `(thing)` | thing |  |  | 源码实锤 |
| `__TS__ClassExtends` | `(target, base, superTypeArgumentsFunc)` | target, base, superTypeArgumentsFunc |  |  | 源码实锤 |
| `__TS__CloneDescriptor` | `(____bindingPattern0)` | ____bindingPattern0 |  |  | 源码实锤 |
| `__TS__ObjectAssign` | `(target, ...)` | target, ... |  |  | 源码实锤 |
| `__TS__ObjectGetOwnPropertyDescriptor` | `(object, key)` | object, key |  |  | 源码实锤 |
| `descriptorIndex` | `(self, key)` | self, key |  |  | 源码实锤 |
| `descriptorNewIndex` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `__TS__SetDescriptor` | `(target, key, desc, isPrototype)` | target, key, desc, isPrototype |  |  | 源码实锤 |
| `__TS__ClassIndex` | `(target, isPrototype)` | target, isPrototype |  |  | 源码实锤 |
| `__TS__Decorate` | `(decorators, target, key, desc)` | decorators, target, key, desc |  |  | 源码实锤 |
| `__TS__DecorateParam` | `(paramIndex, decorator)` | paramIndex, decorator |  |  | 源码实锤 |
| `__TS__StringIncludes` | `(self, searchString, position)` | self, searchString, position |  |  | 源码实锤 |
| `getErrorStack` | `(self, constructor)` | self, constructor |  |  | 源码实锤 |
| `wrapErrorToString` | `(self, getDescription)` | self, getDescription |  |  | 源码实锤 |
| `initErrorClass` | `(self, Type, name)` | self, Type, name |  |  | 源码实锤 |
| `____class_0.prototype.____constructor` | `(self, message)` | self, message |  |  | 源码实锤 |
| `____class_0.prototype.__tostring` | `(self)` | self |  |  | 源码实锤 |
| `createErrorClass` | `(self, name)` | self, name |  |  | 源码实锤 |
| `____class_3.prototype.____constructor` | `(self, ...)` | self, ... |  |  | 源码实锤 |
| `__TS__ObjectGetOwnPropertyDescriptors` | `(object)` | object |  |  | 源码实锤 |
| `__TS__Delete` | `(target, key)` | target, key |  |  | 源码实锤 |
| `__TS__StringAccess` | `(self, index)` | self, index |  |  | 源码实锤 |
| `__TS__DelegatedYield` | `(iterable)` | iterable |  |  | 源码实锤 |
| `generatorIterator` | `(self)` | self |  |  | 源码实锤 |
| `generatorNext` | `(self, ...)` | self, ... |  |  | 源码实锤 |
| `__TS__Generator` | `(fn)` | fn |  |  | 源码实锤 |
| `__TS__InstanceOfObject` | `(value)` | value |  |  | 源码实锤 |
| `__TS__LuaIteratorSpread` | `(self, state, firstKey)` | self, state, firstKey |  |  | 源码实锤 |
| `Map.prototype.____constructor` | `(self, entries)` | self, entries |  |  | 源码实锤 |
| `Map.prototype.clear` | `(self)` | self |  |  | 源码实锤 |
| `Map.prototype.delete` | `(self, key)` | self, key |  |  | 源码实锤 |
| `Map.prototype.forEach` | `(self, callback)` | self, callback |  |  | 源码实锤 |
| `Map.prototype.forEachEx` | `(self, callback)` | self, callback |  |  | 源码实锤 |
| `Map.prototype.get` | `(self, key)` | self, key |  |  | 源码实锤 |
| `Map.prototype.has` | `(self, key)` | self, key |  |  | 源码实锤 |
| `Map.prototype.set` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `Map.prototype.entries` | `(self)` | self |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `Map.prototype.keys` | `(self)` | self |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `Map.prototype.values` | `(self)` | self |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `__TS__MapGet` | `(self, key)` | self, key |  |  | 源码实锤 |
| `__TS__MapSet` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `__TS__MapDelete` | `(self, key)` | self, key |  |  | 源码实锤 |
| `__TS__MapClear` | `(self)` | self |  |  | 源码实锤 |
| `__TS__MapForEach` | `(self, callback)` | self, callback |  |  | 源码实锤 |
| `__TS__MapForEachEx` | `(self, callback)` | self, callback |  |  | 源码实锤 |
| `__TS__MapSize` | `(self)` | self |  |  | 源码实锤 |
| `__TS__MathSign` | `(val)` | val |  |  | 源码实锤 |
| `__TS__Modulo50` | `(a, b)` | a, b |  |  | 源码实锤 |
| `__TS__Number` | `(value)` | value |  |  | 源码实锤 |
| `__TS__NumberIsFinite` | `(value)` | value |  |  | 源码实锤 |
| `__TS__NumberIsNaN` | `(value)` | value |  |  | 源码实锤 |
| `__TS__NumberToString` | `(self, radix)` | self, radix |  |  | 源码实锤 |
| `__TS__ObjectDefineProperty` | `(target, key, desc)` | target, key, desc |  |  | 源码实锤 |
| `__TS__ObjectEntries` | `(obj)` | obj |  |  | 源码实锤 |
| `__TS__ObjectFromEntries` | `(entries)` | entries |  |  | 源码实锤 |
| `__TS__ObjectKeys` | `(obj)` | obj |  |  | 源码实锤 |
| `__TS__ObjectRest` | `(target, usedProperties)` | target, usedProperties |  |  | 源码实锤 |
| `__TS__ObjectValues` | `(obj)` | obj |  |  | 源码实锤 |
| `__TS__ParseFloat` | `(numberString)` | numberString |  |  | 源码实锤 |
| `__TS__StringSubstr` | `(self, from, length)` | self, from, length |  |  | 源码实锤 |
| `__TS__StringSubstring` | `(self, start, ____end)` | self, start, ____end |  |  | 源码实锤 |
| `__TS__ParseInt` | `(numberString, base)` | numberString, base |  |  | 源码实锤 |
| `__TS__PromiseAll` | `(iterable)` | iterable |  |  | 源码实锤 |
| `__TS__PromiseAllSettled` | `(iterable)` | iterable |  |  | 源码实锤 |
| `__TS__PromiseAny` | `(iterable)` | iterable |  |  | 源码实锤 |
| `__TS__PromiseRace` | `(iterable)` | iterable |  |  | 源码实锤 |
| `Set.prototype.____constructor` | `(self, values)` | self, values |  |  | 源码实锤 |
| `Set.prototype.add` | `(self, value)` | self, value |  |  | 源码实锤 |
| `Set.prototype.clear` | `(self)` | self |  |  | 源码实锤 |
| `Set.prototype.delete` | `(self, value)` | self, value |  |  | 源码实锤 |
| `Set.prototype.forEach` | `(self, callback)` | self, callback |  |  | 源码实锤 |
| `Set.prototype.forEachEx` | `(self, callback)` | self, callback |  |  | 源码实锤 |
| `Set.prototype.randomValues` | `(self)` | self |  |  | 源码实锤 |
| `Set.prototype.random` | `(self)` | self |  |  | 源码实锤 |
| `Set.prototype.randoms` | `(self, number, duplicate)` | self, number, duplicate |  |  | 源码实锤 |
| `Set.prototype.has` | `(self, value)` | self, value |  |  | 源码实锤 |
| `Set.prototype.entries` | `(self)` | self |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `Set.prototype.keys` | `(self)` | self |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `Set.prototype.values` | `(self)` | self |  |  | 源码实锤 |
| `next` | `(self)` | self |  |  | 源码实锤 |
| `__TS__SparseArrayNew` | `(...)` | ... |  |  | 源码实锤 |
| `__TS__SparseArrayPush` | `(sparseArray, ...)` | sparseArray, ... |  |  | 源码实锤 |
| `__TS__SparseArraySpread` | `(sparseArray)` | sparseArray |  |  | 源码实锤 |
| `WeakMap.prototype.____constructor` | `(self, entries)` | self, entries |  |  | 源码实锤 |
| `WeakMap.prototype.delete` | `(self, key)` | self, key |  |  | 源码实锤 |
| `WeakMap.prototype.get` | `(self, key)` | self, key |  |  | 源码实锤 |
| `WeakMap.prototype.has` | `(self, key)` | self, key |  |  | 源码实锤 |
| `WeakMap.prototype.set` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `WeakSet.prototype.____constructor` | `(self, values)` | self, values |  |  | 源码实锤 |
| `WeakSet.prototype.add` | `(self, value)` | self, value |  |  | 源码实锤 |
| `WeakSet.prototype.delete` | `(self, value)` | self, value |  |  | 源码实锤 |
| `WeakSet.prototype.has` | `(self, value)` | self, value |  |  | 源码实锤 |
| `__TS__SourceMapTraceBack` | `(fileName, sourceMap)` | fileName, sourceMap |  |  | 源码实锤 |
| `debug.traceback` | `(thread, message, level)` | thread, message, level |  |  | 源码实锤 |
| `replacer` | `(____, file, srcFile, line)` | ____, file, srcFile, line |  |  | 源码实锤 |
| `stringReplacer` | `(____, file, line)` | ____, file, line |  |  | 源码实锤 |
| `__TS__Spread` | `(iterable)` | iterable |  |  | 源码实锤 |
| `__TS__StringCharAt` | `(self, pos)` | self, pos |  |  | 源码实锤 |
| `__TS__StringCharCodeAt` | `(self, index)` | self, index |  |  | 源码实锤 |
| `__TS__StringEndsWith` | `(self, searchString, endPosition)` | self, searchString, endPosition |  |  | 源码实锤 |
| `__TS__StringPadEnd` | `(self, maxLength, fillString)` | self, maxLength, fillString |  |  | 源码实锤 |
| `__TS__StringPadStart` | `(self, maxLength, fillString)` | self, maxLength, fillString |  |  | 源码实锤 |
| `__TS__StringReplace` | `(source, searchValue, replaceValue)` | source, searchValue, replaceValue |  |  | 源码实锤 |
| `__TS__StringSplit` | `(source, separator, limit)` | source, separator, limit |  |  | 源码实锤 |
| `__TS__StringReplaceAll` | `(source, searchValue, replaceValue)` | source, searchValue, replaceValue |  |  | 源码实锤 |
| `__TS__StringSlice` | `(self, start, ____end)` | self, start, ____end |  |  | 源码实锤 |
| `__TS__StringStartsWith` | `(self, searchString, position)` | self, searchString, position |  |  | 源码实锤 |
| `__TS__StringTrim` | `(self)` | self |  |  | 源码实锤 |
| `__TS__StringTrimEnd` | `(self)` | self |  |  | 源码实锤 |
| `__TS__StringTrimStart` | `(self)` | self |  |  | 源码实锤 |
| `__TS__SymbolRegistryFor` | `(key)` | key |  |  | 源码实锤 |
| `__TS__SymbolRegistryKeyFor` | `(sym)` | sym |  |  | 源码实锤 |
| `__TS__TypeOf` | `(value)` | value |  |  | 源码实锤 |

⚠️ **此处被截断，字段不全**：244 处（`<max depth exceeded>`），非 _descriptors 截断样本：`CLASSES.ScoreAddWithWeekLimitParam.____super.prototype.__index` 等 206 处。全量截断路径见 keys_index.json / 对应 fields JSON。

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`__TS__Class2`、`__TS__Unpack`、`__TS__Match`、`Map.prototype.<table key>`、`Set.prototype.<table key>`、`__TS__Promise.prototype.then`、`CLASSES.Actor.prototype.dispose`、`CLASSES.Actor.prototype.set_grid_state`、`CLASSES.Actor.prototype.anim_operation`、`CLASSES.Actor.prototype.destroy`、`CLASSES.Actor.prototype.set_launch_position`、`CLASSES.Actor.prototype.set_grid_range`、`CLASSES.Actor.prototype.mute`、`CLASSES.Actor.prototype.pause`、`CLASSES.Actor.prototype.set_volume`、`CLASSES.Actor.prototype.anim_play`、`CLASSES.Actor.prototype.set_scale_xyz`、`CLASSES.Actor.prototype.set_position`、`CLASSES.Actor.prototype.play_animation_bracket`、`CLASSES.Actor.prototype.attach_to`、`CLASSES.Actor.prototype.detach`、`CLASSES.Actor.prototype.get_visible_slots`、`CLASSES.Actor.prototype.resume`、`CLASSES.Actor.prototype.stop`、`CLASSES.Actor.prototype.play_animation`、`CLASSES.Actor.prototype.set_launch_ground_z`、`CLASSES.Actor.prototype.kill`、`CLASSES.Actor.prototype.set_grid_size`、`CLASSES.Actor.prototype.set_rotation`、`CLASSES.Actor.prototype.do_subclass_action`、`CLASSES.Actor.prototype.set_time_scale_global`、`CLASSES.Actor.prototype.set_impact_site`、`CLASSES.Actor.prototype.play`、`CLASSES.Actor.prototype.anim_play_bracket`、`CLASSES.Actor.prototype.is_valid`、`CLASSES.Actor.prototype.set_shadow`、`CLASSES.Actor.prototype.anim_set_paused_all`、`CLASSES.Actor.prototype.show`、`CLASSES.Actor.prototype.set_owner`、`CLASSES.Actor.prototype.on_normal_init`、…（共 1086 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/margin`

- 来源：script 包（common 库）（`script\199\common\base\margin.lua`）
- 加载：`require 'base.margin'`（init.lua:130，非 app 平台）；`require 'base.margin'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 7，类 2）

边缘区域 `RegionMargin` 类（ extends Region ）。

**dump 对照差异（重要）**：script 包内 margin.lua 本体只有客户端空函数 `function base.margin(...) end`（注释「逻辑全在服务端，客户端只需要空函数」）【源码实锤】；但服务端 dump 值是 table（`RegionMargin` 类）——**服务端运行时加载的是另一实现**（服务端 script 变体或引擎内嵌 TS 模块），本包源码不代表服务端行为【dump 实锤 + 语义推测】。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.margin` | `(...)` | ... |  | 逻辑全在服务端，客户端只需要空函数 | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `RegionMargin` | TSTL 类（3 键） |  | dump 实锤 |
| `RegionMargin.prototype` | table（12 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `RegionMargin` | extends `Region` | `remove_region`、`get_height`、`init_region`、`init_margin_region`、`get_width`、`get_scene_point`、`get_point` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`RegionMargin.prototype.remove_region`、`RegionMargin.prototype.get_height`、`RegionMargin.prototype.init_region`、`RegionMargin.prototype.init_margin_region`、`RegionMargin.prototype.get_width`、`RegionMargin.prototype.get_scene_point`、`RegionMargin.prototype.get_point`

### `@common/base/match_info`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.match_info'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 1，类 1）

对局信息 `MatchInfo` 类（TS），方法 get_team_user_list。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `MatchInfo.prototype.get_team_user_list` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `MatchInfo` | TSTL 类（2 键） |  | dump 实锤 |
| `MatchInfo.prototype` | table（3 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `MatchInfo` | 无 | `get_team_user_list` |

### `@common/base/math`

- 来源：script 包（common 库）（`script\199\common\base\math.lua`）
- 加载：`require 'base.math'`（init.lua:70）；`require 'base.math'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

角度制数学库 `base.math`（浮点比较/随机/向量运算）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.math.sin` | `(r)` | r |  |  | 源码实锤 |
| `base.math.cos` | `(r)` | r |  |  | 源码实锤 |
| `base.math.tan` | `(r)` | r |  |  | 源码实锤 |
| `base.math.asin` | `(v)` | v |  |  | 源码实锤 |
| `base.math.acos` | `(v)` | v |  |  | 源码实锤 |
| `base.math.atan` | `(v1, v2)` | v1, v2 |  |  | 源码实锤 |
| `base.math.ceil` | `(v)` | v |  |  | 源码实锤 |
| `base.math.floor` | `(v)` | v |  |  | 源码实锤 |
| `base.math.float_eq` | `(a, b)` | a, b |  | 浮点数比较 | 源码实锤 |
| `base.math.float_ueq` | `(a, b)` | a, b |  |  | 源码实锤 |
| `base.math.float_lt` | `(a, b)` | a, b |  |  | 源码实锤 |
| `base.math.float_le` | `(a, b)` | a, b |  |  | 源码实锤 |
| `base.math.float_gt` | `(a, b)` | a, b |  |  | 源码实锤 |
| `base.math.float_ge` | `(a, b)` | a, b |  |  | 源码实锤 |
| `base.math.random_float` | `(a, b)` | a, b |  | 随机浮点数 | 源码实锤 |
| `is_int` | `(n)` | n |  |  | 源码实锤 |
| `base.math.is_int` | `(n)` | n |  |  | 源码实锤 |
| `base.math.random_int` | `(a, b)` | a, b |  | 随机整数 | 源码实锤 |
| `base.math.float_modf` | `(n)` | n |  | 浮点数小数部分（编辑器用） | 源码实锤 |
| `base.math.included_angle` | `(r1, r2)` | r1, r2 |  | 计算2个角度之间的夹角 | 源码实锤 |
| `base.math.lerp` | `(from, to, t)` | from, to, t |  | 插值运算 | 源码实锤 |
| `base.math.clamp` | `(value, left, right)` | value, left, right |  |  | 源码实锤 |
| `base.math.max` | `(...)` | ... |  |  | 源码实锤 |
| `base.math.min` | `(...)` | ... |  |  | 源码实锤 |
| `base.math.vector_add` | `(vector1, vector2)` | vector1, vector2 |  |  | 源码实锤 |
| `base.math.vector_sub` | `(vector1, vector2)` | vector1, vector2 |  |  | 源码实锤 |
| `base.math.vector_mul` | `(vector, mul)` | vector, mul |  |  | 源码实锤 |
| `base.math.dot_product` | `(vector1, vector2)` | vector1, vector2 |  |  | 源码实锤 |
| `base.math.cross_product` | `(vector1, vector2)` | vector1, vector2 |  |  | 源码实锤 |
| `base.math.sqrt` | `(x)` | x |  | 平方根 | 源码实锤 |
| `base.math.log` | `(...)` | ... |  | 对数 | 源码实锤 |
| `base.math.pow` | `(x, y)` | x, y |  | 次幂 | 源码实锤 |
| `base.math.square` | `(x)` | x |  | 平方 | 源码实锤 |
| `base.math.exp` | `(x)` | x |  | 自然指数 | 源码实锤 |
| `base.math.abs` | `(x)` | x |  | 绝对值 | 源码实锤 |

### `@common/base/mover_line`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.mover_line'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 1，类 2）

直线弹道运动器 `MoverLine` 类（ extends Mover ）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `MoverLine.prototype.mover_end` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `MoverLine` | TSTL 类（3 键） |  | dump 实锤 |
| `MoverLine.prototype` | table（20 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `MoverLine` | extends `Mover` | `mover_end` |

### `@common/base/mover_target`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.mover_target'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 1 键，函数 1，类 2）

目标追踪运动器 `MoverTarget` 类（ extends Mover ）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `MoverTarget.prototype.mover_end` | `(?)` |  |  |  | dump 实锤 |

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `MoverTarget` | TSTL 类（3 键） |  | dump 实锤 |
| `MoverTarget.prototype` | table（19 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `MoverTarget` | extends `Mover` | `mover_end` |

### `@common/base/obj_check`

- 来源：script 包（common 库）（`script\199\common\base\obj_check.lua`）
- 加载：`require 'base.obj_check'`（init.lua:72）；`require 'base.obj_check'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

参数类型校验函数群（全局 *_check）+ `base.gui_*` + UI 淡入淡出。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `unit_check` | `(unit, disable_error)` | unit, disable_error |  |  | 源码实锤 |
| `item_check` | `(item, disable_error)` | item, disable_error |  |  | 源码实锤 |
| `skill_check` | `(skill, disable_error)` | skill, disable_error |  |  | 源码实锤 |
| `player_check` | `(player, disable_error)` | player, disable_error |  |  | 源码实锤 |
| `circle_check` | `(obj, disable_error)` | obj, disable_error |  |  | 源码实锤 |
| `rect_check` | `(obj, disable_error)` | obj, disable_error |  |  | 源码实锤 |
| `area_check` | `(obj, disable_error)` | obj, disable_error |  |  | 源码实锤 |
| `point_check` | `(point, disable_error)` | point, disable_error |  |  | 源码实锤 |
| `line_check` | `(line, disable_error)` | line, disable_error |  |  | 源码实锤 |
| `buff_check` | `(buff, disable_error)` | buff, disable_error |  |  | 源码实锤 |
| `trigger_check` | `(trigger, disable_error)` | trigger, disable_error |  |  | 源码实锤 |
| `timer_check` | `(timer, disable_error)` | timer, disable_error |  |  | 源码实锤 |
| `any_unit_check` | `(unit, disable_error)` | unit, disable_error |  |  | 源码实锤 |
| `any_skill_check` | `(skill, disable_error)` | skill, disable_error |  |  | 源码实锤 |
| `any_player_check` | `(player, disable_error)` | player, disable_error |  |  | 源码实锤 |
| `id_check` | `(obj_id, disable_error)` | obj_id, disable_error |  |  | 源码实锤 |
| `event_name_check` | `(event_name, disable_error)` | event_name, disable_error |  |  | 源码实锤 |
| `time_check` | `(time, disable_error)` | time, disable_error |  |  | 源码实锤 |
| `component_check` | `(cmpt, disable_error)` | cmpt, disable_error |  |  | 源码实锤 |
| `base.gui_check` | `(cmpt)` | cmpt |  |  | 源码实锤 |
| `base.gui_get_part_as` | `(ts_type, cmpt, part_name)` | ts_type, cmpt, part_name |  |  | 源码实锤 |
| `base.gui_get_parts_ts` | `(ts_type, cmpt, part_name)` | ts_type, cmpt, part_name |  |  | 源码实锤 |
| `base.gui_get_array_child` | `(ts_type, cmpt)` | ts_type, cmpt |  |  | 源码实锤 |
| `base.gui_get_child_ui_by_name_as` | `(ts_type, cmpt, child_name)` | ts_type, cmpt, child_name |  |  | 源码实锤 |
| `base.gui_get_children` | `(ctrl)` | ctrl |  |  | 源码实锤 |
| `base.gui_get_rect` | `(ctrl)` | ctrl |  |  | 源码实锤 |
| `base.gui_get_parent` | `(ctrl)` | ctrl |  |  | 源码实锤 |
| `base.fade_in_out` | `(fade_type, fade_time, is_wait, color, opacity, curve_type, z_index)` | fade_type, fade_time, is_wait, color, opacity, curve_type, z_index |  |  | 源码实锤 |
| `base.fade_in` | `(fade_time, is_wait, color, opacity, curve_type, z_index)` | fade_time, is_wait, color, opacity, curve_type, z_index |  |  | 源码实锤 |
| `init` | `(self)` | self |  |  | 源码实锤 |
| `fade_in` | `(self)` | self |  |  | 源码实锤 |
| `fade_out` | `(self)` | self |  |  | 源码实锤 |
| `base.fade_out` | `(fade_time, is_wait, color, opacity, curve_type, z_index)` | fade_time, is_wait, color, opacity, curve_type, z_index |  |  | 源码实锤 |

### `@common/base/old_junk`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.old_junk'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

历史遗留兼容层（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/player`

- 来源：script 包（common 库）（`script\199\common\base\player.lua`）
- 加载：`require 'base.player'`（init.lua:111，非 app 平台）；`require 'base.player'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 98，类 1）

玩家 Player 类（属性/英雄/事件/注册表）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `init_one_player` | `(id, ptype, team)` | id, ptype, team |  |  | 源码实锤 |
| `init_players` | `()` |  |  |  | 源码实锤 |
| `set_team_id` | `(self, team)` | self, team |  |  | 源码实锤 |
| `mt:get_team_id` | `()` |  |  |  | 源码实锤 |
| `mt:get_team` | `()` |  |  |  | 源码实锤 |
| `set_hero` | `(self, unit)` | self, unit |  |  | 源码实锤 |
| `mt:get_hero` | `()` |  |  |  | 源码实锤 |
| `mt:is_ally` | `(other)` | other |  |  | 源码实锤 |
| `mt:is_enemy` | `(other)` | other |  |  | 源码实锤 |
| `mt:is_neutral_to` | `(other)` | other | boolean |  | 源码实锤 |
| `mt:is_neutral` | `()` |  | boolean |  | 源码实锤 |
| `mt:is_online` | `()` |  | boolean |  | 源码实锤 |
| `mt:set_hero_upper_body_facing` | `(facing, sync_to_server)` | facing, sync_to_server |  |  | 源码实锤 |
| `mt:cancel_hero_upper_body_facing` | `(time)` | time |  |  | 源码实锤 |
| `set_hero_name` | `(self, name)` | self, name |  |  | 源码实锤 |
| `mt:get_hero_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_hero_reborn` | `()` |  |  |  | 源码实锤 |
| `mt:user_name` | `()` |  |  |  | 源码实锤 |
| `mt:user_title` | `()` |  |  |  | 源码实锤 |
| `mt:user_icon` | `()` |  |  |  | 源码实锤 |
| `mt:user_border` | `()` |  |  |  | 源码实锤 |
| `mt:get` | `(key)` | key |  |  | 源码实锤 |
| `mt:get_slot_id` | `()` |  |  |  | 源码实锤 |
| `mt:controller` | `()` |  |  |  | 源码实锤 |
| `mt:game_state` | `()` |  |  |  | 源码实锤 |
| `mt:loading_progress` | `()` |  |  |  | 源码实锤 |
| `modify_table` | `(ori_tbl, modify_tbl)` | ori_tbl, modify_tbl |  |  | 源码实锤 |
| `delete_table` | `(ori_tbl, modify_tbl)` | ori_tbl, modify_tbl |  |  | 源码实锤 |
| `set_by_sync` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `set` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `mt:event_notify` | `(name, ...)` | name, ... |  |  | 源码实锤 |
| `mt:event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `base.local_player` | `()` |  |  |  | 源码实锤 |
| `base.player` | `(id)` | id |  |  | 源码实锤 |
| `base.each_player` | `(type)` | type |  |  | 源码实锤 |
| `next` | `()` |  |  |  | 源码实锤 |
| `sort_pairs` | `(t)` | t |  |  | 源码实锤 |
| `base.event.on_player_table_attributes_changed` | `(key_values)` | key_values |  |  | 源码实锤 |
| `base.event.on_player_attributes_changed` | `(key_values)` | key_values |  |  | 源码实锤 |
| `base.event.on_loading_progress_notify` | `(slot_id, progress)` | slot_id, progress |  |  | 源码实锤 |
| `mt:get_nick_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_num` | `(name, ...)` | name, ... |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Player` | TSTL 类（2 键） |  | dump 实锤 |
| `Player.prototype` | table（101 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Player` | 无 | `is_neutral_to`、`get_match_team_id`、`set_hero`、`unlock_camera`、`controller`、`error_info`、`get_team_id`、`set_sight_team_id`、`create_unit`、`effect`、`get_hero`、`play_sound`、`event_subscribe`、`set_hero_skill_sync_type`、`event_unsubscribe`、`get_friends`、`get_friend_apply_list`、`__debugger_extand`、`game_state`、`user_id`、`save_hero`、`event_notify`、`get_scene_name`、`jump_scene`、`match_mask`、`add`、`event_stat`、`get_map_size`、`message`、`get_account_prop`、`shake_camera`、`set_table`、`toggle_force_share_sight`、`ignore_sight`、`is_abort`、`input_mouse`、`get_num`、`user_level`、`create_illusion`、`ui_message`、`get_nick_name`、`kick`、`get_sync_table`、`set_camera`、`create_item`、`get_friend_middle_game_key`、`__tostring`、`get_user_name`、`share_sight_to_player`、`is_ai`、`is_enemy`、`event_has`、`get_user_info`、`is_online`、`event_dispatch`、`create_controlled_sync_unit`、`add_gold`、`create_actor`、`message_box`、`clear_hero`、…（共 98 个，全量见 `parsed/fields` 对应 JSON） |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Player.prototype.get_match_team_id`、`Player.prototype.unlock_camera`、`Player.prototype.error_info`、`Player.prototype.set_sight_team_id`、`Player.prototype.create_unit`、`Player.prototype.effect`、`Player.prototype.play_sound`、`Player.prototype.event_subscribe`、`Player.prototype.set_hero_skill_sync_type`、`Player.prototype.event_unsubscribe`、`Player.prototype.get_friends`、`Player.prototype.get_friend_apply_list`、`Player.prototype.__debugger_extand`、`Player.prototype.user_id`、`Player.prototype.save_hero`、`Player.prototype.get_scene_name`、`Player.prototype.jump_scene`、`Player.prototype.match_mask`、`Player.prototype.add`、`Player.prototype.event_stat`、`Player.prototype.get_map_size`、`Player.prototype.message`、`Player.prototype.get_account_prop`、`Player.prototype.shake_camera`、`Player.prototype.set_table`、`Player.prototype.toggle_force_share_sight`、`Player.prototype.ignore_sight`、`Player.prototype.is_abort`、`Player.prototype.input_mouse`、`Player.prototype.user_level`、`Player.prototype.create_illusion`、`Player.prototype.ui_message`、`Player.prototype.kick`、`Player.prototype.get_sync_table`、`Player.prototype.set_camera`、`Player.prototype.create_item`、`Player.prototype.get_friend_middle_game_key`、`Player.prototype.get_user_name`、`Player.prototype.share_sight_to_player`、`Player.prototype.is_ai`、…（共 78 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/point`

- 来源：script 包（common 库）（`script\199\common\base\point.lua`）
- 加载：`require 'base.point'`（init.lua:77）；`require 'base.point'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 50，类 1）

点 Point 类（带高度/场景，运算符重载点运算）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `create_point` | `(x, y, z, scene)` | x, y, z, scene | Point | 创建一个点 | 源码实锤 |
| `table_to_point` | `(table)` | table | Point? |  | 源码实锤 |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:get_xy` | `()` |  |  | 获取坐标 | 源码实锤 |
| `mt:get_x` | `()` |  |  |  | 源码实锤 |
| `mt:get_y` | `()` |  |  |  | 源码实锤 |
| `mt:get_height` | `()` |  |  |  | 源码实锤 |
| `mt:set_scene` | `(scene)` | scene |  |  | 源码实锤 |
| `mt:copy` | `()` |  |  | 复制点 | 源码实锤 |
| `mt:copy_to_scene_point` | `(scene)` | scene |  |  | 源码实锤 |
| `mt:get_point` | `()` |  |  | 返回点 | 源码实锤 |
| `mt:get_scene_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene` | `()` |  |  |  | 源码实锤 |
| `mt:get_position` | `()` |  |  | 返回位置 | 源码实锤 |
| `mt:__add` | `(data)` | data |  |  | 源码实锤 |
| `mt:__sub` | `(data)` | data |  |  | 源码实锤 |
| `mt:__mul` | `(dest)` | dest |  | 求距离(point * point) | 源码实锤 |
| `mt:__div` | `(dest)` | dest |  |  | 源码实锤 |
| `mt:__unm` | `()` |  |  |  | 源码实锤 |
| `mt:add` | `(data)` | data |  |  | 源码实锤 |
| `mt:polar_to_ex` | `(angle, distance)` | angle, distance |  |  | 源码实锤 |
| `mt:polar_to` | `(data)` | data |  | 按照极坐标系移动(point:polar_to{angle, distance} ) | 源码实锤 |
| `mt:angle` | `(dest)` | dest |  | 求方向(向量self和向量dest的夹角) | 源码实锤 |
| `mt:distance` | `(dest)` | dest |  |  | 源码实锤 |
| `mt:to_coordinate` | `(point, facing)` | point, facing |  | 将self映射到坐标系(point, facing)后, self在该坐标系里的位置 | 源码实锤 |
| `mt:set_height` | `(value)` | value |  |  | 源码实锤 |
| `mt:is_block` | `()` |  |  |  | 源码实锤 |
| `mt.has_restriction` | `(_,_)` | _,_ |  |  | 源码实锤 |
| `mt.has_label` | `(_,_)` | _,_ |  |  | 源码实锤 |
| `mt.get_attackable_radius` | `(_)` | _ |  |  | 源码实锤 |
| `mt:get_unit` | `()` |  |  |  | 源码实锤 |
| `mt:get_team_id` | `()` |  |  |  | 源码实锤 |
| `base.get_scene_point` | `(scene, area_name, present)` | scene, area_name, present |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Point` | TSTL 类（2 键） |  | dump 实锤 |
| `Point.prototype` | table（56 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Point` | 无 | `is_neutral_to`、`__call`、`get_owner`、`get_attackable_radius`、`__add`、`group_range`、`__tostring`、`get_team_id`、`is_enemy`、`is_block_ex`、`get_facing`、`play_sound`、`has_label`、`__unm`、`to_vector`、`has_restriction`、`__sub`、`is_visible_to`、`create_effect`、`get_snapshot`、`group_line`、`get_scene_point`、`group_sector`、`is_visible`、`__mul`、`get_unit`、`get_xy`、`set_height`、`path_distance`、`polar_to_ex`、`get_flag`、`polar_to`、`get_scene`、`get_scene_name`、`get_x`、`distance`、`is_block`、`__div`、`add`、`angle`、`to_coordinate`、`is_ally`、`is_neutral`、`get_y`、`get_z`、`is_valid`、`copy_to_scene_point`、`copy`、`angle_to`、`get_point` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Point.prototype.is_neutral_to`、`Point.prototype.__call`、`Point.prototype.get_owner`、`Point.prototype.group_range`、`Point.prototype.is_enemy`、`Point.prototype.is_block_ex`、`Point.prototype.get_facing`、`Point.prototype.play_sound`、`Point.prototype.to_vector`、`Point.prototype.is_visible_to`、`Point.prototype.create_effect`、`Point.prototype.get_snapshot`、`Point.prototype.group_line`、`Point.prototype.group_sector`、`Point.prototype.is_visible`、`Point.prototype.path_distance`、`Point.prototype.get_flag`、`Point.prototype.get_scene_name`、`Point.prototype.is_ally`、`Point.prototype.is_neutral`、`Point.prototype.get_z`、`Point.prototype.is_valid`、`Point.prototype.angle_to`

### `@common/base/position`

- 来源：script 包（common 库）（`script\199\common\base\position.lua`）
- 加载：`require 'base.position'`（init.lua:81）；`require 'base.position'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 3，类 1）

屏幕坐标 `ScreenPos` 类。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:get_xy` | `()` |  |  |  | 源码实锤 |
| `mt:get_x` | `()` |  |  |  | 源码实锤 |
| `mt:get_y` | `()` |  |  |  | 源码实锤 |
| `mt:get_ui_x` | `()` |  |  |  | 源码实锤 |
| `mt:get_ui_y` | `()` |  |  |  | 源码实锤 |
| `mt:get_point` | `()` |  |  |  | 源码实锤 |
| `base.mouse_screen_pos` | `()` |  |  |  | 源码实锤 |
| `base.position` | `(x, y)` | x, y |  |  | 源码实锤 |
| `base.screen_pos` | `(x, y)` | x, y |  | 用下面这个不容易误解 | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `ScreenPos` | TSTL 类（2 键） |  | dump 实锤 |
| `ScreenPos.prototype` | table（8 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `ScreenPos` | 无 | `__tostring`、`get_xy` |

### `@common/base/promise`

- 来源：script 包（common 库）（`script\199\common\base\promise.lua`）
- 加载：`require 'base.promise'`（init.lua:92）；`require 'base.promise'`
- 状态：✅ 有源码
- dump 值：table（顶层 3 键，函数 3，类 0）

promise/multi_promise 异步原语（基于 event_deque + co）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `get` | `(self, timeout, callback)` | self, timeout, callback |  |  | 源码实锤 |
| `co_result` | `(self, timeout)` | self, timeout |  |  | 源码实锤 |
| `co_error` | `(self, timeout)` | self, timeout |  |  | 源码实锤 |
| `co_get` | `(self, timeout)` | self, timeout |  |  | 源码实锤 |
| `set` | `(self, value, err)` | self, value, err |  |  | 源码实锤 |
| `try_set` | `(self, value, err)` | self, value, err |  |  | 源码实锤 |
| `set_result` | `(self, v)` | self, v |  |  | 源码实锤 |
| `try_set_result` | `(self, v)` | self, v |  |  | 源码实锤 |
| `set_error` | `(self, err)` | self, err |  |  | 源码实锤 |
| `try_set_error` | `(self, err)` | self, err |  |  | 源码实锤 |
| `ready` | `(self)` | self |  |  | 源码实锤 |
| `promise:__call` | `()` |  | promise |  | 源码实锤 |
| `get` | `(self, timeout, callback)` | self, timeout, callback |  |  | 源码实锤 |
| `co_get` | `(self, timeout)` | self, timeout |  |  | 源码实锤 |
| `_start` | `(self, promise_list, timeout)` | self, promise_list, timeout |  |  | 源码实锤 |
| `ready` | `(self)` | self |  |  | 源码实锤 |
| `multi_promise:__call` | `(promise_list, join_type, timeout)` | promise_list, join_type, timeout | multi_promise |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（3 键） |  | dump 实锤 |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`create_promise`、`create_multi_promise`、`as_promise`

### `@common/base/quest`

- 来源：script 包（common 库）（`script\199\common\base\quest.lua`）
- 加载：`require 'base.quest'`（init.lua:124，非 app 平台）；`require 'base.quest'`
- 状态：✅ 有源码
- dump 值：table（顶层 2 键，函数 66，类 2）

任务 `Quest`/任务条件 `QuestCondition` 类。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.print_table` | `(t)` | t |  |  | 源码实锤 |
| `quest_condition:new` | `(tbl)` | tbl |  |  | 源码实锤 |
| `quest_condition:update_remaining_time` | `(remaining_time)` | remaining_time |  |  | 源码实锤 |
| `quest_condition:get_remaining_time` | `()` |  |  |  | 源码实锤 |
| `quest_condition:update` | `(tbl)` | tbl |  |  | 源码实锤 |
| `quest_condition:remove` | `()` |  |  |  | 源码实锤 |
| `quest_condition:submit` | `()` |  |  |  | 源码实锤 |
| `quest_condition:get_description` | `()` |  |  |  | 源码实锤 |
| `quest:new` | `(tbl)` | tbl |  |  | 源码实锤 |
| `quest:update` | `(tbl)` | tbl |  |  | 源码实锤 |
| `quest:remove` | `()` |  |  |  | 源码实锤 |
| `quest.update_quests` | `(unit, tbl, change_table)` | unit, tbl, change_table |  |  | 源码实锤 |
| `quest:__tostring` | `()` |  |  | if base.test then | 源码实锤 |
| `quest_condition:__tostring` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |
| `Quest` | TSTL 类（2 键） |  | dump 实锤 |
| `Quest.prototype` | table（27 键） |  | dump 实锤 |
| `Quest.prototype.active_state` | table（2 键） |  | dump 实锤 |
| `Quest.prototype.complete_state` | table（3 键） |  | dump 实锤 |
| `QuestCondition` | TSTL 类（2 键） |  | dump 实锤 |
| `QuestCondition.prototype` | table（33 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionTime` | table（5 键） |  | dump 实锤 |
| `QuestCondition.prototype.Default` | table（6 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionKill` | table（1 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionEffect` | table（1 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionUnitAttribute` | table（2 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionItem` | table（2 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionPlayerAttribute` | table（2 键） |  | dump 实锤 |
| `QuestCondition.prototype.QuestConditionSet` | table（5 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Quest` | 无 | `load_score_to_unit`、`update_complete`、`bind_to_user`、`sync`、`new`、`deactivate`、`remove`、`submit`、`event_notify`、`reset`、`activate`、`get_current_condition`、`get_quest_info`、`unbind_quests_to_user`、`event_dispatch`、`get_player_score_quest_list`、`event`、`save_score_to_unit`、`unbind_to_user`、`bind_quests_to_user`、`load_quest_from_info`、`__tostring` |
| `QuestCondition` | 无 | `update_complete`、`new`、`set_complete`、`set_failed`、`reset`、`set_active_state`、`event_dispatch`、`set_complete_state`、`set_progress`、`as_quest_condition_set`、`sync`、`deactivate`、`event_notify`、`activate`、`add_progress`、`init`、`event`、`__tostring`、`set_none`、`submit` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Quest.prototype.load_score_to_unit`、`Quest.prototype.update_complete`、`Quest.prototype.bind_to_user`、`Quest.prototype.sync`、`Quest.prototype.deactivate`、`Quest.prototype.event_notify`、`Quest.prototype.reset`、`Quest.prototype.activate`、`Quest.prototype.get_current_condition`、`Quest.prototype.get_quest_info`、`Quest.prototype.unbind_quests_to_user`、`Quest.prototype.event_dispatch`、`Quest.prototype.get_player_score_quest_list`、`Quest.prototype.event`、`Quest.prototype.save_score_to_unit`、`Quest.prototype.unbind_to_user`、`Quest.prototype.bind_quests_to_user`、`Quest.prototype.load_quest_from_info`、`QuestCondition.prototype.update_complete`、`QuestCondition.prototype.QuestConditionTime.update_complete`、`QuestCondition.prototype.QuestConditionTime.init`、`QuestCondition.prototype.QuestConditionTime.reset`、`QuestCondition.prototype.QuestConditionTime.activate`、`QuestCondition.prototype.set_complete`、`QuestCondition.prototype.Default.update_complete`、`QuestCondition.prototype.Default.init`、`QuestCondition.prototype.Default.reset`、`QuestCondition.prototype.Default.activate`、`QuestCondition.prototype.Default.deactivate`、`QuestCondition.prototype.set_failed`、`QuestCondition.prototype.reset`、`QuestCondition.prototype.QuestConditionKill.activate`、`QuestCondition.prototype.set_active_state`、`QuestCondition.prototype.event_dispatch`、`QuestCondition.prototype.QuestConditionEffect.activate`、`QuestCondition.prototype.set_complete_state`、`QuestCondition.prototype.set_progress`、`QuestCondition.prototype.as_quest_condition_set`、`QuestCondition.prototype.sync`、`QuestCondition.prototype.deactivate`、…（共 54 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/rect`

- 来源：script 包（common 库）（`script\199\common\base\rect.lua`）
- 加载：`require 'base.rect'`（init.lua:129，非 app 平台）；`require 'base.rect'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 8，类 2）

矩形区域 `RegionRect` 类。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:get_scene` | `()` |  |  |  | 源码实锤 |
| `mt:get_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_start_point` | `(pos,width)` | pos,width |  |  | 源码实锤 |
| `mt:get_start_scene_point` | `(pos,width)` | pos,width |  |  | 源码实锤 |
| `mt:get_width` | `()` |  |  |  | 源码实锤 |
| `mt:get_height` | `()` |  |  |  | 源码实锤 |
| `mt:random_point` | `()` |  |  |  | 源码实锤 |
| `mt:scene_random_point` | `()` |  |  |  | 源码实锤 |
| `mt:init_region` | `()` |  |  |  | 源码实锤 |
| `region:on_enter` | `(unit)` | unit |  |  | 源码实锤 |
| `region:on_leave` | `(unit)` | unit |  |  | 源码实锤 |
| `mt:remove_region` | `()` |  |  |  | 源码实锤 |
| `base.rect` | `(...)` | ... |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `RegionRect` | TSTL 类（3 键） |  | dump 实锤 |
| `RegionRect.prototype` | table（13 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `RegionRect` | extends `Region` | `remove_region`、`get_height`、`init_region`、`scene_random_point`、`random_point`、`get_width`、`get_scene_point`、`get_point` |

### `@common/base/response`

- 来源：script 包（common 库）（`script\199\common\base\response.lua`）
- 加载：`require 'base.response'`（init.lua:108，非 app 平台）；`require 'base.response'`
- 状态：✅ 有源码
- dump 值：table（顶层 7 键，函数 26，类 14）

响应 `Response` 类族（攻击/受击等效果响应与冷却）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.response:new` | `(link)` | link | Response? |  | 源码实锤 |
| `base.response:set_cache` | `(link)` | link |  |  | 源码实锤 |
| `base.response:execute` | `(in_param, ...)` | in_param, ... |  |  | 源码实锤 |
| `base.response:response` | `(...)` | ... |  |  | 源码实锤 |
| `response_compare` | `(a, b)` | a, b | boolean |  | 源码实锤 |
| `base.response:add` | `(unit, ref_param)` | unit, ref_param |  |  | 源码实锤 |
| `base.response:remove` | `()` |  |  |  | 源码实锤 |
| `base.response:enabled` | `()` |  |  |  | 源码实锤 |
| `base.response:disabled` | `()` |  |  |  | 源码实锤 |
| `base.response.ResponseDamage:validate` | `(in_param, damage)` | in_param, damage |  |  | 源码实锤 |
| `base.response.ResponseDamage:exectue` | `(in_param, damage)` | in_param, damage |  |  | 源码实锤 |
| `base.response.ResponseMissileImpact:exectue` | `(in_param)` | in_param |  |  | 源码实锤 |
| `base.response.ResponseEffectImpact:exectue` | `(in_param)` | in_param |  |  | 源码实锤 |
| `base.response.ResponseSpell:exectue` | `(in_param, event, skill)` | in_param, event, skill |  |  | 源码实锤 |
| `has_category` | `(cache, category)` | cache, category |  |  | 源码实锤 |
| `filter_categories` | `(cache, category_filters)` | cache, category_filters | boolean |  | 源码实锤 |
| `base.response.ResponseBuff:exectue` | `(in_param, data)` | in_param, data |  |  | 源码实锤 |
| `base.response.ResponseUnit:exectue` | `(in_param, event)` | in_param, event |  |  | 源码实锤 |
| `base.response:start_cooldown` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（7 键） |  | dump 实锤 |
| `ResponseMissileImpact` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseMissileImpact.prototype` | table（3 键） |  | dump 实锤 |
| `Response` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype` | table（22 键） |  | dump 实锤 |
| `Response.prototype.ResponseEffectImpact` | table（3 键） |  | dump 实锤 |
| `Response.prototype.ResponseEffectImpact.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype.e_location` | table（2 键） |  | dump 实锤 |
| `Response.prototype.ResponseDamage` | table（4 键） |  | dump 实锤 |
| `Response.prototype.ResponseDamage.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype.ResponseUnit` | table（3 键） |  | dump 实锤 |
| `Response.prototype.ResponseUnit.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype.ResponseSpellModifier` | table（4 键） |  | dump 实锤 |
| `Response.prototype.ResponseSpellModifier.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype.ResponseSpell` | table（3 键） |  | dump 实锤 |
| `Response.prototype.ResponseSpell.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype.ResponseBuff` | table（3 键） |  | dump 实锤 |
| `Response.prototype.ResponseBuff.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `Response.prototype.ResponseMissileImpact` | table（3 键） |  | dump 实锤 |
| `Response.prototype.ResponseMissileImpact.constructor` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseSpell` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseSpell.prototype` | table（3 键） |  | dump 实锤 |
| `ResponseDamage` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseDamage.prototype` | table（4 键） |  | dump 实锤 |
| `ResponseBuff` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseBuff.prototype` | table（3 键） |  | dump 实锤 |
| `ResponseEffectImpact` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseEffectImpact.prototype` | table（3 键） |  | dump 实锤 |
| `ResponseUnit` | TSTL 类（2 键） |  | dump 实锤 |
| `ResponseUnit.prototype` | table（3 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `ResponseMissileImpact` | 无 | `exectue` |
| `Response` | 无 | `new`、`enabled`、`set_cache`、`response`、`apply_response_cooldown`、`start_cooldown`、`add`、`disabled`、`execute`、`remove` |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `<max depth exceeded>` | 无 | （仅构造/元方法） |
| `ResponseSpell` | 无 | `exectue` |
| `ResponseDamage` | 无 | `validate`、`exectue` |
| `ResponseBuff` | 无 | `exectue` |
| `ResponseEffectImpact` | 无 | `exectue` |
| `ResponseUnit` | 无 | `exectue` |

⚠️ **此处被截断，字段不全**：14 处（`<max depth exceeded>`），非 _descriptors 截断样本：`Response.prototype.ResponseEffectImpact.constructor.name` 等 14 处。全量截断路径见 keys_index.json / 对应 fields JSON。

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Response.prototype.apply_response_cooldown`

### `@common/base/room`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.room'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

房间服务：`_G.base.room` = find_game_list/find_room/sync_room_info（见 FINDINGS F3）。

**说明**：本键值为 `true`（已加载无表导出），但 `_G.base.room` 在 dump 全局表中有完整方法面：`find_game_list` / `find_room` / `sync_room_info`（3 函数）【dump 实锤，经 `_G` 值树】。语义宿主是 `base.room` 全局而非本模块键（FINDINGS F3）。调用点反查形态：`find_room({ game_name=..., room_mode=..., ... })`、`sync_room_info({ room_code=..., room_cur_number=0 })`【反查推测】。

**调用点反查 API 形态**（参数以实际调用为准）【反查推测】：

- `find_room({ game_name = data.game_name, room_mode = data.room_mode,...)`
- `sync_room_info({ room_code = data.room_code, room_cur_number = 0 })`

**实测结果**（2026-08-27 编辑器 PIE，探针 probe_server_apis 批次2）【实测】：

| 函数 | 实测调用 | 实测返回/行为 |
| --- | --- | --- |
| `sync_room_info({room_code='__PROBE__', room_cur_number=0})` | OK | 无返回（上报/同步语义，fire-and-forget） |
| `find_game_list({})` | OK | 返回二值 `({}, 3)`（房间列表表 + 数值，编辑器环境无房） |
| `find_room({game_name='p_55a3', room_mode='', extra={tag=''}})` | OK | 引擎 log `find_room error`（common/base/room/init.lua:147）+ 返回错误码 **-2**（调试环境无房可找） |

- **实现位置实锤**：引擎内嵌 Lua `common/base/room/init.lua`（traceback :141/:147，api-13 解密包无此文件——与上方"无源码"一致，pak-extract 路线可挖）。
- 调用点源码（lib_lobby 中途局.lua）：`find_room` 在 `co.async` 协程内调用、返回 code 经 `player:ui` 回包；`sync_room_info` 同步直调【源码实锤（调用点）】。

### `@common/base/rpc`

- 来源：script 包（common 库）（`script\199\common\base\rpc.lua`）
- 加载：被 ad.lua:3 / voice.lua 等 `require`；`require 'base.rpc'`
- 状态：✅ 有源码
- dump 值：table（顶层 2 键，函数 2，类 0）

简易 RPC（经 `__simple_rpc__` 消息；`rpc.xxx(...)` 调用 / `rpc.xxx = f` 注册）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `rpc_call` | `(k, ...)` | k, ... |  |  | 源码实锤 |
| `__index` | `(t, k)` | t, k |  |  | 源码实锤 |
| `__newindex` | `(t, k, v)` | t, k, v |  |  | 源码实锤 |
| `make_args` | `(owner, ...)` | owner, ... |  |  | 源码实锤 |
| `rpc_accept` | `(owner, k, ...)` | owner, k, ... |  |  | 源码实锤 |
| `rpc.callback` | `(id, ...)` | id, ... |  |  | 源码实锤 |
| `proto.__simple_rpc__` | `(call)` | call |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |

### `@common/base/scene_object`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.scene_object'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

场景物件（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/scene_point`

- 来源：script 包（common 库）（`script\199\common\base\scene_point.lua`）
- 加载：`require 'base.scene_point'`（init.lua:80）；`require 'base.scene_point'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 43，类 2）

带场景的点 `ScenePoint` 类（继承 Target，支持错误点标记）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `create_scene_point` | `(x, y, z, scene, error_mark)` | x, y, z, scene, error_mark | ScenePoint |  | 源码实锤 |
| `create_scene_point_by_hash` | `(x, y, z, scene_hash, error_mark)` | x, y, z, scene_hash, error_mark | ScenePoint |  | 源码实锤 |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:get_xy` | `()` |  |  | 获取坐标 | 源码实锤 |
| `mt:get_x` | `()` |  |  |  | 源码实锤 |
| `mt:get_y` | `()` |  |  |  | 源码实锤 |
| `mt:get_z` | `()` |  |  | 获取z | 源码实锤 |
| `mt:get_height` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene_point` | `()` |  |  |  | 源码实锤 |
| `mt:copy` | `()` |  |  | 复制点 | 源码实锤 |
| `mt:copy_to_scene_point` | `(scene)` | scene |  |  | 源码实锤 |
| `mt:get_point` | `()` |  |  | 返回点 | 源码实锤 |
| `mt:to_vector` | `(height)` | height |  | 转换为矢量 | 源码实锤 |
| `mt:__add` | `(data)` | data |  |  | 源码实锤 |
| `mt:__sub` | `(data)` | data |  | 按照极坐标系移动(point:polar_to({angle, distance})) | 源码实锤 |
| `mt:__mul` | `(dest)` | dest |  | 求距离(point * point) | 源码实锤 |
| `mt:__div` | `(dest)` | dest |  |  | 源码实锤 |
| `mt:__unm` | `()` |  |  |  | 源码实锤 |
| `mt:add` | `(data)` | data |  |  | 源码实锤 |
| `mt:polar_to` | `(data)` | data |  | 按照极坐标系移动(point:polar_to{angle, distance} ) | 源码实锤 |
| `mt:polar_to_ex` | `(angle, distance)` | angle, distance |  |  | 源码实锤 |
| `mt:angle` | `(dest)` | dest |  | 求方向(向量self和向量dest的夹角) | 源码实锤 |
| `mt:distance` | `(dest)` | dest |  |  | 源码实锤 |
| `mt:to_coordinate` | `(point, facing)` | point, facing |  | 将self映射到坐标系(point, facing)后, self在该坐标系里的位置 | 源码实锤 |
| `mt:get_unit` | `()` |  |  |  | 源码实锤 |
| `mt:get_owner` | `()` |  |  |  | 源码实锤 |
| `mt:get_facing` | `()` |  |  |  | 源码实锤 |
| `mt:get_team_id` | `()` |  |  |  | 源码实锤 |
| `mt:angle_to` | `(dest)` | dest | number?, boolean? |  | 源码实锤 |
| `mt:get_snapshot` | `()` |  |  |  | 源码实锤 |
| `mt:create_effect` | `(model)` | model |  | TODO: 需要特别指定一个中立玩家： | 源码实锤 |
| `mt:is_visible_to` | `(dest)` | dest | boolean?, boolean? |  | 源码实锤 |
| `mt.has_restriction` | `(_,_)` | _,_ |  |  | 源码实锤 |
| `mt.has_label` | `(_,_)` | _,_ |  |  | 源码实锤 |
| `mt.get_attackable_radius` | `(_)` | _ |  |  | 源码实锤 |
| `mt:get_collision_flags` | `(bol)` | bol |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `ScenePoint` | TSTL 类（3 键） |  | dump 实锤 |
| `ScenePoint.prototype` | table（49 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `ScenePoint` | extends `Target` | `is_neutral_to`、`get_owner`、`get_attackable_radius`、`set_height`、`get_unit`、`__tostring`、`get_team_id`、`is_enemy`、`polar_to_ex`、`to_3d`、`get_facing`、`get_snapshot`、`__unm`、`to_vector`、`__sub`、`__mul`、`is_valid`、`has_label`、`angle`、`get_xy`、`is_visible_to`、`get_z`、`__add`、`create_effect`、`polar_to`、`get_scene`、`get_scene_name`、`copy_to_scene_point`、`is_block_ex`、`has_restriction`、`__div`、`add`、`get_collision_flags`、`to_coordinate`、`is_ally`、`is_neutral`、`distance`、`get_scene_point`、`get_x`、`get_y`、`copy`、`angle_to`、`get_point` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`ScenePoint.prototype.is_neutral_to`、`ScenePoint.prototype.set_height`、`ScenePoint.prototype.is_enemy`、`ScenePoint.prototype.to_3d`、`ScenePoint.prototype.is_valid`、`ScenePoint.prototype.is_block_ex`、`ScenePoint.prototype.is_ally`、`ScenePoint.prototype.is_neutral`

### `@common/base/selector`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.selector'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

选择器（命名语义推测；gui/selector.lua 是 UI 侧同名不同物）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/shop`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.shop'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

商店（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/skill`

- 来源：script 包（common 库）（`script\199\common\base\skill.lua`）
- 加载：`require 'base.skill'`（init.lua:112，非 app 平台）；`require 'base.skill'`
- 状态：✅ 有源码
- dump 值：table（顶层 2 键，函数 192，类 3）

技能 `Skill` 类 + `Cast`（槽位/冷却/属性 key 映射）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `get_skill_name_by_hash` | `(hash)` | hash |  |  | 源码实锤 |
| `active_cd` | `(self, cd, total)` | self, cd, total |  |  | 源码实锤 |
| `finish_cd` | `(self)` | self |  |  | 源码实锤 |
| `active_charge_cd` | `(self, cd, total)` | self, cd, total |  |  | 源码实锤 |
| `finish_charge_cd` | `(self)` | self |  |  | 源码实锤 |
| `is_removed` | `(self)` | self |  |  | 源码实锤 |
| `api:client_remove` | `()` |  |  |  | 源码实锤 |
| `remove` | `(self)` | self |  |  | 源码实锤 |
| `can_request` | `(self)` | self |  |  | 源码实锤 |
| `ac_skill` | `(id, hash, owner, is_silent)` | id, hash, owner, is_silent |  |  | 源码实锤 |
| `set` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `set_user_attribute` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `update_attribute` | `(self, attr, events)` | self, attr, events |  |  | 源码实锤 |
| `update_attribute_without_event` | `(self, attr)` | self, attr |  |  | 源码实锤 |
| `base.event.on_spell_attributes_changed` | `(key_values)` | key_values |  |  | 源码实锤 |
| `base.event.on_remove_spell` | `(removed_spells)` | removed_spells |  |  | 源码实锤 |
| `base.event.on_spell_cd_changed` | `(id, cd, total, type)` | id, cd, total, type |  |  | 源码实锤 |
| `base.event.on_spell_cd_finished` | `(id, type)` | id, type |  |  | 源码实锤 |
| `base.event.on_spell_cast_approach_ex` | `(unit_id, hash)` | unit_id, hash |  |  | 源码实锤 |
| `base.event.on_spell_cast_start_ex` | `(unit_id, hash, time, total)` | unit_id, hash, time, total |  |  | 源码实锤 |
| `base.event.on_spell_cast_notify_ex` | `(unit_id, hash, time, total)` | unit_id, hash, time, total |  |  | 源码实锤 |
| `base.event.on_spell_cast_shot_ex` | `(unit_id, hash, time, total)` | unit_id, hash, time, total |  |  | 源码实锤 |
| `base.event.on_spell_cast_end_ex` | `(unit_id, hash, time, total)` | unit_id, hash, time, total |  |  | 源码实锤 |
| `base.event.on_spell_cast_stop_ex` | `(unit_id, hash, time, total)` | unit_id, hash, time, total |  |  | 源码实锤 |
| `base.event.on_spell_cast_break_ex` | `(unit_id, hash)` | unit_id, hash |  |  | 源码实锤 |
| `base.event.on_spell_cast_failed_ex` | `(unit_id, hash)` | unit_id, hash |  |  | 源码实锤 |
| `base.proto.cancel_ignore_joy_stick` | `(msg)` | msg |  |  | 源码实锤 |
| `base.proto.skill_group_set_unit` | `(msg)` | msg |  |  | 源码实锤 |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:__index` | `(key)` | key |  |  | 源码实锤 |
| `api:level_data` | `(data, fallbackValue, level)` | data, fallbackValue, level | number |  | 源码实锤 |
| `api:get_name` | `()` |  |  |  | 源码实锤 |
| `api:get_owner` | `()` |  |  |  | 源码实锤 |
| `api:get_tip` | `()` |  |  |  | 源码实锤 |
| `api:get_stack` | `()` |  |  |  | 源码实锤 |
| `api:get_level` | `()` |  |  |  | 源码实锤 |
| `api:get_slot_id` | `()` |  |  |  | 源码实锤 |
| `api:is_enable` | `()` |  |  |  | 源码实锤 |
| `api:is_charge_skill` | `()` |  |  |  | 源码实锤 |
| `api:get_type` | `()` |  |  |  | 源码实锤 |
| `api:can_upgrade` | `()` |  |  | deprecated，新的用can_learn | 源码实锤 |
| `api:can_learn` | `()` |  |  |  | 源码实锤 |
| `api:event_notify` | `(name, ...)` | name, ... |  |  | 源码实锤 |
| `api:event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `api:get_cd` | `()` |  |  |  | 源码实锤 |
| `api:get_charge_cd` | `()` |  |  |  | 源码实锤 |
| `api:pause` | `()` |  |  |  | 源码实锤 |
| `api:resume` | `()` |  |  |  | 源码实锤 |
| `api:update_paused` | `()` |  |  |  | 源码实锤 |
| `api:cast` | `(smart)` | smart |  | deprecated | 源码实锤 |
| `api:client_channel_finish` | `()` |  |  |  | 源码实锤 |
| `get_target_indicator_cache` | `(link)` | link |  |  | 源码实锤 |
| `api:show_range` | `(follow, assistName)` | follow, assistName |  |  | 源码实锤 |
| `api:hide_range` | `()` |  |  |  | 源码实锤 |
| `api:move` | `(slot)` | slot |  |  | 源码实锤 |
| `api:upgrade` | `()` |  |  |  | 源码实锤 |
| `api:has_category` | `(category)` | category |  |  | 源码实锤 |
| `api:hotkey` | `(smart)` | smart |  |  | 源码实锤 |
| `api:create_actor` | `(link)` | link |  |  | 源码实锤 |
| `api:create_actors` | `(event)` | event |  |  | 源码实锤 |
| `api:destroy_actors` | `(event)` | event |  |  | 源码实锤 |
| `api:is_attack` | `()` |  |  |  | 源码实锤 |
| `api:is_attack_modifier` | `()` |  |  |  | 源码实锤 |
| `api:get_user_attribute` | `(key)` | key |  |  | 源码实锤 |
| `api:is_toggled_on` | `()` |  |  |  | 源码实锤 |
| `api:get_phase` | `()` |  |  |  | 源码实锤 |
| `api:get_current_show_cd` | `()` |  |  |  | 源码实锤 |
| `api:get_max_show_cd` | `()` |  |  |  | 源码实锤 |
| `api:get_currrent_charge_show_cd` | `()` |  |  |  | 源码实锤 |
| `api:get_max_charge_show_cd` | `()` |  |  |  | 源码实锤 |
| `try_load_show_methods` | `()` |  |  |  | 源码实锤 |
| `api:get_show_name` | `()` |  |  |  | 源码实锤 |
| `api:get_icon` | `()` |  |  |  | 源码实锤 |
| `api:get_tips` | `()` |  |  |  | 源码实锤 |
| `api:get_current_cd` | `()` |  |  |  | 源码实锤 |
| `api:get_cd_max` | `()` |  |  |  | 源码实锤 |
| `api:get_current_charge_cd` | `()` |  |  |  | 源码实锤 |
| `api:get_charge_cd_max` | `()` |  |  |  | 源码实锤 |
| `api:get_cooldown_key` | `()` |  |  |  | 源码实锤 |
| `base.skill_info` | `()` |  |  |  | 源码实锤 |
| `base.proto.sync_skill` | `(msg)` | msg |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（2 键） |  | dump 实锤 |
| `Cast` | TSTL 类（4 键） |  | dump 实锤 |
| `Cast.prototype` | table（4 键） |  | dump 实锤 |
| `Skill` | TSTL 类（2 键） |  | dump 实锤 |
| `Skill.prototype` | table（97 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Cast` | extends `Skill` | （仅构造/元方法） |
| `Skill` | 无 | `destroy_actors_passive`、`disable_hidden`、`can_learn`、`execute_cast_event`、`get_modifier_missile_asset`、`mark_autocast_skill`、`_on_spell_fish_channel`、`_on_change_slot`、`_on_remove`、`create_actor_passive`、`_on_cast_stop`、`get_skill_max_cd`、`_on_cooldown`、`set_phase`、`_on_enable`、`active_custom_cd`、`update_attribute_change`、`get_last_target`、`filter_categories`、`is_hidden`、`data_driven_target`、`get_name`、`create_actors`、`start_effect`、`_on_cast_shot`、`learn`、`_get_cool`、`get_multi_phase_max_cd`、`get_num`、`toggle`、`can_acquire_cast`、`get_cooldown_key`、`is_toggled_on`、`is_init_on`、`finish_channel`、`_get_range`、`get_phase`、`is_channeler_active`、`_get_charge_cool`、`execute_self_effect`、`get_last_target_angle`、`get_last_target_unit`、`bail`、`_on_cast_break`、`_on_can_cast`、`_on_cast_start`、`is_channeling`、`is_attack`、`apply_attribute_change`、`create_actor`、`_on_periodic`、`add_modifier`、`get_modifier_impact_actor_links`、`can_add_modifier`、`get_effect_link`、`set_num`、`finish_shot`、`_on_can_break`、`_get_formula_flag`、`_on_upgrade`、…（共 91 个，全量见 `parsed/fields` 对应 JSON） |

⚠️ **此处被截断，字段不全**：4 处（`<max depth exceeded>`），全部为 `prototype._descriptors.*` 属性访问器（get/set/enumerable/configurable），不损失 API 信息。全量截断路径见 keys_index.json / 对应 fields JSON。

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Cast.____super.prototype.destroy_actors_passive`、`Cast.____super.prototype.disable_hidden`、`Cast.____super.prototype.execute_cast_event`、`Cast.____super.prototype.get_modifier_missile_asset`、`Cast.____super.prototype.mark_autocast_skill`、`Cast.____super.prototype._on_spell_fish_channel`、`Cast.____super.prototype._on_change_slot`、`Cast.____super.prototype._on_remove`、`Cast.____super.prototype.create_actor_passive`、`Cast.____super.prototype._on_cast_stop`、`Cast.____super.prototype.get_skill_max_cd`、`Cast.____super.prototype._on_cooldown`、`Cast.____super.prototype.set_phase`、`Cast.____super.prototype._on_enable`、`Cast.____super.prototype.active_custom_cd`、`Cast.____super.prototype.update_attribute_change`、`Cast.____super.prototype.get_last_target`、`Cast.____super.prototype.filter_categories`、`Cast.____super.prototype.is_hidden`、`Cast.____super.prototype.data_driven_target`、`Cast.____super.prototype.start_effect`、`Cast.____super.prototype._on_cast_shot`、`Cast.____super.prototype.learn`、`Cast.____super.prototype._get_cool`、`Cast.____super.prototype.get_multi_phase_max_cd`、`Cast.____super.prototype.get_num`、`Cast.____super.prototype.toggle`、`Cast.____super.prototype.can_acquire_cast`、`Cast.____super.prototype.is_init_on`、`Cast.____super.prototype.finish_channel`、`Cast.____super.prototype._get_range`、`Cast.____super.prototype.is_channeler_active`、`Cast.____super.prototype._get_charge_cool`、`Cast.____super.prototype.execute_self_effect`、`Cast.____super.prototype.get_last_target_angle`、`Cast.____super.prototype.get_last_target_unit`、`Cast.____super.prototype.bail`、`Cast.____super.prototype._on_cast_break`、`Cast.____super.prototype._on_can_cast`、`Cast.____super.prototype._on_cast_start`、…（共 162 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/snapshot`

- 来源：script 包（common 库）（`script\199\common\base\snapshot.lua`）
- 加载：`require 'base.snapshot'`（init.lua:107，非 app 平台）；`require 'base.snapshot'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 13，类 1）

目标快照 `Snapshot` 类（Target 静态副本，技能参数固化用）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:new` | `()` |  |  |  | 源码实锤 |
| `mt:get_snapshot` | `()` |  | Snapshot |  | 源码实锤 |
| `mt:get_point` | `()` |  | Point |  | 源码实锤 |
| `mt.get_unit` | `(_)` | _ | Unit? |  | 源码实锤 |
| `mt:get_name` | `()` |  | string |  | 源码实锤 |
| `mt:get_owner` | `()` |  | Player |  | 源码实锤 |
| `mt:get_facing` | `()` |  | integer |  | 源码实锤 |
| `mt:is_ally` | `(dest)` | dest | boolean |  | 源码实锤 |
| `mt:is_visible_to` | `(dest)` | dest | boolean |  | 源码实锤 |
| `mt:get_team_id` | `()` |  | integer |  | 源码实锤 |
| `mt:has_restriction` | `(restriction)` | restriction |  |  | 源码实锤 |
| `mt:has_label` | `(label)` | label |  |  | 源码实锤 |
| `mt:get_attackable_radius` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `SnapShot` | TSTL 类（2 键） |  | dump 实锤 |
| `SnapShot.prototype` | table（16 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Snapshot` | 无 | `is_visible_to`、`has_restriction`、`has_label`、`get_owner`、`get_team_id`、`new`、`get_snapshot`、`get_attackable_radius`、`get_name`、`is_ally`、`get_facing`、`get_unit`、`get_point` |

### `@common/base/state_machine`

- 来源：script 包（common 库）（`script\199\common\base\state_machine.lua`）
- 加载：被 unit.lua:1 `require`；`require 'base.state_machine'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

自定义状态机（包装 C++ SCE.StateMachine，仅 StateGame 生效）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `CustomStateMachine:ctor` | `(name, priority, layer)` | name, priority, layer |  |  | 源码实锤 |
| `CustomStateMachine:add_state` | `(name, id)` | name, id |  |  | 源码实锤 |
| `base.state_machine` | `(name, priority, layer)` | name, priority, layer |  |  | 源码实锤 |
| `State:ctor` | `(name, id)` | name, id |  |  | 源码实锤 |
| `base.state_machine_state` | `(name, id)` | name, id |  |  | 源码实锤 |

### `@common/base/table`

- 来源：script 包（common 库）（`script\199\common\base\table.lua`）
- 加载：`include 'base.table'`（init.lua:102）+ `require 'base.table'`（init.lua:119，非 app 平台）；`require 'base.table'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

数编表访问层：`base.table` 懒加载元表 + `base.skill_table/unit_table/...`。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `SDBMHash` | `(str)` | str |  |  | 源码实锤 |
| `__index` | `(self, name)` | self, name |  |  | 源码实锤 |
| `base.skill_table` | `(name, level, key)` | name, level, key |  |  | 源码实锤 |
| `base.unit_table` | `(name, key)` | name, key |  |  | 源码实锤 |
| `base.buff_table` | `(name, key)` | name, key |  |  | 源码实锤 |
| `base.attack_table` | `(name, key)` | name, key |  |  | 源码实锤 |
| `base.item_table` | `(name, key)` | name, key |  |  | 源码实锤 |
| `base.spell_table` | `(name, key)` | name, key |  |  | 源码实锤 |

### `@common/base/table_attr`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.table_attr'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

表属性相关（命名语义推测）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/target_filter`

- 来源：script 包（common 库）（`script\199\common\base\target_filter.lua`）
- 加载：`require 'base.target_filter'`（init.lua:123，非 app 平台）；`require 'base.target_filter'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 9，类 1）

目标过滤器 `TargetFilters`（「需要;排除」字符串解析与校验）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:new` | `(filter_string)` | filter_string | TargetFilters |  | 源码实锤 |
| `mt:from_string` | `(filter_string)` | filter_string |  |  | 源码实锤 |
| `mt:from_data_field` | `(filter_string)` | filter_string |  |  | 源码实锤 |
| `mt.make_cmd_result` | `(filter, is_required)` | filter, is_required |  |  | 源码实锤 |
| `mt:validate` | `(caster,target)` | caster,target | string? ErrorText |  | 源码实锤 |
| `mt.filter_player` | `(caster, target, filter)` | caster, target, filter | boolean |  | 源码实锤 |
| `mt.filter_state` | `(target,filter)` | target,filter | boolean |  | 源码实锤 |
| `mt.filter_label` | `(target,label)` | target,label | boolean |  | 源码实锤 |
| `is_custom_restruction` | `(att)` | att |  |  | 源码实锤 |
| `mt.has_filter` | `(caster,target,filter)` | caster,target,filter |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `TargetFilters` | TSTL 类（2 键） |  | dump 实锤 |
| `TargetFilters.prototype` | table（13 键） |  | dump 实锤 |
| `TargetFilters.prototype.filters` | table（19 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `TargetFilters` | 无 | `validate`、`new`、`filter_player`、`filter_label`、`filter_state`、`make_cmd_result`、`has_filter`、`from_data_field`、`from_string` |

### `@common/base/tds_score`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.tds_score'`
- 状态：⚠️ 无源码
- dump 值：table（顶层 36 键，函数 116，类 41）

云变量/排行榜服务的 TS 参数类族 + `ScoreCommitter` 提交器（tds_score 族最有料的键）。

**说明（dump 实锤 + lua_plus 对照）**

本键是 tds_score 族中唯一有表导出的键（`@tds_score*` 五键值均为 `true`，实现引擎侧）。值树顶层 36 键 = 35 个 `Score*Param`/`Score*Data` TS 参数/数据类 + `ScoreCommitter` 提交器类。所有 Param 类的属性走 `_descriptors` 访问器（`_key`/`_user_id`/`_timetype`/`_limit`），descriptor 的 get/set 函数全部落在截断点（109 处截断全部是 `_descriptors.*` 访问器，不损失真实 API 信息）。

`ScoreCommitter` 方法（dump 实锤，签名未知；实参形态由 lua_plus 包装器 `server_lua_plus\14\base\base_lua_plus\tds_score.lua` 反查【源码实锤】）：

| 方法 | 实参形态（lua_plus 调用点） | 说明 | 置信 |
| --- | --- | --- | --- |
| `commit` | `c.commit() -> error_code, data, err_msg` | 向云端提交变更请求 | dump 实锤 + 反查推测 |
| `set` | `c.set{user_id=..., key=..., i_value=/s_value=/value=...}` | 设置云变量（数值/字符串/表格三态） | dump 实锤 + 反查推测 |
| `add` | （lua_plus 中调用写 `c.addi{user_id, key, value}`） | 数值增加；⚠️ 包装器方法名 `addi` 与 dump 方法名 `add` 不一致，疑版本漂移或动态别名 | dump 实锤 |
| `money_set` / `money_add` / `money_cost` / `money_add_ex` | `c.money_set{user_id, key, value}` 等 | 货币设置/增加/消耗 | dump 实锤 + 反查推测 |
| `rank_add` / `rank_set` | — | 排行榜增加/设置 | dump 实锤 |
| `list_add` / `list_modify` / `list_delete` | — | 列表型云变量操作 | dump 实锤 |
| `withlimit_add` | — | 限次增加（配 ScoreAddWithHour/Day/Week/Month/YearLimitParam，均 extends ScoreAddWithLimitParam） | dump 实锤 |
| `name_new` | — | 命名类云变量新建（配 ScoreNameNewParam） | dump 实锤 |
| `clear` | — | 清空（配 ScoreClearParam） | dump 实锤 |
| `add_finish_callback` | — | 注册完成回调 | dump 实锤 |

读取侧不在本模块：lua_plus 包装器用全局 `score.get/money_get/get_commit`（`_G.score` 28 函数，见 FINDINGS F5）；`base.score_commit_init(game_name)` → `score.get_commit()` 创建 committer 并记入 `base.last_created_score_committer`，提交结果落 `base.last_commit_success/last_commit_error_code/last_commit_error_msg`【源码实锤：lua_plus tds_score.lua】。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `ScoreCommitter.prototype.list_modify` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.commit` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.rank_add` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.rank_set` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.money_cost` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.list_add` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.list_delete` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.add` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.clear` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.add_finish_callback` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.withlimit_add` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.money_add` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.name_new` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.money_add_ex` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.money_set` | `(?)` |  |  |  | dump 实锤 |
| `ScoreCommitter.prototype.set` | `(?)` |  |  |  | dump 实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（36 键） |  | dump 实锤 |
| `ScoreRankAddParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreRankAddParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreListAddParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreListAddParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreGetParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreGetParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreAddParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreAddParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreSetParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreSetParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreNameNewParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreNameNewParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreListGetParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreListGetParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreMsgData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreMsgData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreRankSetParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreRankSetParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreMoneyData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreMoneyData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreMsgQueryParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreMsgQueryParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreNameData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreNameData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreGetUserRankParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreGetUserRankParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreListDeleteParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreListDeleteParam.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreAddWithLimitParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreAddWithLimitParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreObjData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreObjData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreGetUserRankData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreGetUserRankData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreListData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreListData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreAddWithHourLimitParam` | TSTL 类（3 键） |  | dump 实锤 |
| `ScoreAddWithHourLimitParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreGetRankTotalData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreGetRankTotalData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreAddWithMonthLimitParam` | TSTL 类（3 键） |  | dump 实锤 |
| `ScoreAddWithMonthLimitParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreListModifyParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreListModifyParam.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreNameSearchParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreNameSearchParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreRankListData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreRankListData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreListQueryByUuidParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreListQueryByUuidParam.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreGetRankTotalParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreGetRankTotalParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreGetRankListParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreGetRankListParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreCommitter` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreCommitter.prototype` | table（19 键） |  | dump 实锤 |
| `ScoreMoneyParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreMoneyParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreClearParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreClearParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreChannelMsgData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreChannelMsgData.prototype` | table（3 键） |  | dump 实锤 |
| `ScoreMsgSendParam` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreMsgSendParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreAddWithYearLimitParam` | TSTL 类（3 键） |  | dump 实锤 |
| `ScoreAddWithYearLimitParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreAddWithWeekLimitParam` | TSTL 类（3 键） |  | dump 实锤 |
| `ScoreAddWithWeekLimitParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreAddWithDayLimitParam` | TSTL 类（3 键） |  | dump 实锤 |
| `ScoreAddWithDayLimitParam.prototype` | table（5 键） |  | dump 实锤 |
| `ScoreData` | TSTL 类（1 键） |  | dump 实锤 |
| `ScoreData.prototype` | table（3 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `ScoreRankAddParam` | 无 | （仅构造/元方法） |
| `ScoreListAddParam` | 无 | （仅构造/元方法） |
| `ScoreGetParam` | 无 | （仅构造/元方法） |
| `ScoreAddParam` | 无 | （仅构造/元方法） |
| `ScoreSetParam` | 无 | （仅构造/元方法） |
| `ScoreNameNewParam` | 无 | （仅构造/元方法） |
| `ScoreListGetParam` | 无 | （仅构造/元方法） |
| `ScoreMsgData` | 无 | （仅构造/元方法） |
| `ScoreRankSetParam` | 无 | （仅构造/元方法） |
| `ScoreMoneyData` | 无 | （仅构造/元方法） |
| `ScoreMsgQueryParam` | 无 | （仅构造/元方法） |
| `ScoreNameData` | 无 | （仅构造/元方法） |
| `ScoreGetUserRankParam` | 无 | （仅构造/元方法） |
| `ScoreListDeleteParam` | 无 | （仅构造/元方法） |
| `ScoreAddWithLimitParam` | 无 | （仅构造/元方法） |
| `ScoreObjData` | 无 | （仅构造/元方法） |
| `ScoreGetUserRankData` | 无 | （仅构造/元方法） |
| `ScoreListData` | 无 | （仅构造/元方法） |
| `ScoreAddWithHourLimitParam` | 有 ____super（名未知） | （仅构造/元方法） |
| `ScoreGetRankTotalData` | 无 | （仅构造/元方法） |
| `ScoreAddWithMonthLimitParam` | 有 ____super（名未知） | （仅构造/元方法） |
| `ScoreListModifyParam` | 无 | （仅构造/元方法） |
| `ScoreNameSearchParam` | 无 | （仅构造/元方法） |
| `ScoreRankListData` | 无 | （仅构造/元方法） |
| `ScoreListQueryByUuidParam` | 无 | （仅构造/元方法） |
| `ScoreGetRankTotalParam` | 无 | （仅构造/元方法） |
| `ScoreGetRankListParam` | 无 | （仅构造/元方法） |
| `ScoreCommitter` | 无 | `list_modify`、`commit`、`rank_add`、`rank_set`、`money_cost`、`list_add`、`list_delete`、`add`、`clear`、`add_finish_callback`、`withlimit_add`、`money_add`、`name_new`、`money_add_ex`、`money_set`、`set` |
| `ScoreMoneyParam` | 无 | （仅构造/元方法） |
| `ScoreClearParam` | 无 | （仅构造/元方法） |
| `ScoreChannelMsgData` | 无 | （仅构造/元方法） |
| `ScoreMsgSendParam` | 无 | （仅构造/元方法） |
| `ScoreAddWithYearLimitParam` | 有 ____super（名未知） | （仅构造/元方法） |
| `ScoreAddWithWeekLimitParam` | 有 ____super（名未知） | （仅构造/元方法） |
| `ScoreAddWithDayLimitParam` | 有 ____super（名未知） | （仅构造/元方法） |
| `ScoreData` | 无 | （仅构造/元方法） |

⚠️ **此处被截断，字段不全**：109 处（`<max depth exceeded>`），全部为 `prototype._descriptors.*` 属性访问器（get/set/enumerable/configurable），不损失 API 信息。全量截断路径见 keys_index.json / 对应 fields JSON。

### `@common/base/team`

- 来源：script 包（common 库）（`script\199\common\base\team.lua`）
- 加载：`require 'base.team'`（init.lua:114，非 app 平台）；`require 'base.team'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 4，类 1）

队伍 `Team` 类 + `base.team(id)`。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:get_id` | `()` |  |  |  | 源码实锤 |
| `mt:each_player` | `()` |  |  |  | 源码实锤 |
| `next` | `()` |  |  |  | 源码实锤 |
| `init` | `()` |  |  |  | 源码实锤 |
| `base.team` | `(id)` | id |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Team` | TSTL 类（2 键） |  | dump 实锤 |
| `Team.prototype` | table（7 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Team` | 无 | `each_player`、`get_id`、`set_winner`、`playe_sound` |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Team.prototype.set_winner`、`Team.prototype.playe_sound`

### `@common/base/thirdordermatrix`

- 来源：script 包（common 库）（`script\199\common\base\thirdordermatrix.lua`）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.thirdordermatrix'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 5，类 1）

三阶矩阵类（加减/矩阵乘/向量乘/行列式）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `create_tom` | `(TOMArray)` | TOMArray |  | tom = ThirdOrderMatrix | 源码实锤 |
| `mt:tom_addition` | `(MartixB)` | MartixB |  | 矩阵加法 | 源码实锤 |
| `mt:tom_subtraction` | `(MartixB)` | MartixB |  | 矩阵减法 | 源码实锤 |
| `mt:tom_multiplication_with_tom` | `(MartixB)` | MartixB |  | 与矩阵相乘 | 源码实锤 |
| `mt:tom_multiplication_with_vector` | `(Vector)` | Vector |  | 与向量相乘 | 源码实锤 |
| `mt:tom_determinant` | `()` |  |  | 矩阵的行列式 determinant | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `ThirdOrderMatrix` | TSTL 类（2 键） |  | dump 实锤 |
| `ThirdOrderMatrix.prototype` | table（8 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `ThirdOrderMatrix` | 无 | `tom_determinant`、`tom_multiplication_with_vector`、`tom_subtraction`、`tom_multiplication_with_tom`、`tom_addition` |

### `@common/base/timer`

- 来源：script 包（common 库）（`script\199\common\base\timer.lua`）
- 加载：`require 'base.timer'`（init.lua:75）；`require 'base.timer'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 12，类 1）

帧计时器系统：`base.wait/loop/timer/next` 全家桶 + `Timer` 类。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `update_next` | `()` |  |  |  | 源码实锤 |
| `alloc_queue` | `()` |  |  |  | 源码实锤 |
| `m_timeout` | `(self, timeout, check_frame)` | self, timeout, check_frame |  |  | 源码实锤 |
| `m_timeout_forSetTime` | `(self, timeout)` | self, timeout |  | 不重置时间偏移量，使得用户通过api读取的时间保持正确 | 源码实锤 |
| `m_wakeup` | `(self)` | self |  |  | 源码实锤 |
| `get_remaining` | `(self)` | self |  |  | 源码实锤 |
| `on_tick` | `()` |  |  |  | 源码实锤 |
| `base.clock` | `()` |  |  |  | 源码实锤 |
| `base.event.on_tick` | `(delta)` | delta |  |  | 源码实锤 |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:remove` | `()` |  |  |  | 源码实锤 |
| `mt:pause` | `()` |  |  |  | 源码实锤 |
| `mt:resume` | `()` |  |  |  | 源码实锤 |
| `mt:restart` | `()` |  |  |  | 源码实锤 |
| `mt:get_current` | `()` |  |  |  | 源码实锤 |
| `mt:get_current_time` | `()` |  |  |  | 源码实锤 |
| `mt:set_current_time` | `(NewTime)` | NewTime |  |  | 源码实锤 |
| `mt:get_remaining_time` | `()` |  |  |  | 源码实锤 |
| `mt:get_remaining_time_new` | `()` |  |  |  | 源码实锤 |
| `mt:set_remaining_time` | `(NewTime)` | NewTime |  |  | 源码实锤 |
| `base.wait` | `(timeout, on_timer, timer)` | timeout, on_timer, timer |  |  | 源码实锤 |
| `base.loop` | `(timeout, on_timer)` | timeout, on_timer | Timer |  | 源码实锤 |
| `base.wait` | `(timeout, on_timer)` | timeout, on_timer |  |  | 源码实锤 |
| `base.loop` | `(timeout, on_timer)` | timeout, on_timer | Timer |  | 源码实锤 |
| `base.loop_lazy` | `(timeout, on_timer)` | timeout, on_timer |  |  | 源码实锤 |
| `base.next` | `(cb)` | cb |  |  | 源码实锤 |
| `base.timer` | `(timeout, count, on_timer)` | timeout, count, on_timer |  |  | 源码实锤 |
| `utimer_initialize` | `(u)` | u |  |  | 源码实锤 |
| `base.uwait` | `(u, timeout, on_timer)` | u, timeout, on_timer |  |  | 源码实锤 |
| `base.uloop` | `(u, timeout, on_timer)` | u, timeout, on_timer |  |  | 源码实锤 |
| `base.utimer` | `(u, timeout, count, on_timer)` | u, timeout, count, on_timer |  |  | 源码实锤 |
| `base.set_timer_warning` | `(w)` | w |  |  | 源码实锤 |
| `on_update` | `(delta)` | delta |  |  | 源码实锤 |
| `base.event.on_update` | `(delta)` | delta |  |  | 源码实锤 |
| `base.event.on_post_update` | `(delta)` | delta |  |  | 源码实锤 |
| `base.event.on_prerender_update` | `(delta)` | delta |  |  | 源码实锤 |
| `base.event.on_server_clock` | `(clock)` | clock |  |  | 源码实锤 |
| `base.timer_info` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Timer` | TSTL 类（2 键） |  | dump 实锤 |
| `Timer.prototype` | table（15 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Timer` | 无 | `resume`、`get_current`、`restart`、`get_current_time`、`remove`、`set_remaining_time`、`set_current_time`、`__tostring`、`get_remaining_time_new`、`pause`、`get_remaining_time` |

### `@common/base/trigger`

- 来源：script 包（common 库）（`script\199\common\base\trigger.lua`）
- 加载：`require 'base.trigger'`（init.lua:74）；`require 'base.trigger'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 17，类 1）

触发器 `Trigger` 体系：事件订阅/场景化事件/事件参数构造器总表。

**dump 截断说明**：197 处截断集中在 `Trigger.prototype.event.evt_args.*`（事件参数构造器表）。该表内容已由源码侧 `args.event_*` 函数清单完整覆盖（见上方函数表），截断不影响完整性【源码实锤补位】。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__tostring` | `()` |  |  | if base.test then | 源码实锤 |
| `mt:disable` | `()` |  |  | 禁用触发器 | 源码实锤 |
| `mt:enable` | `()` |  |  |  | 源码实锤 |
| `mt:is_enable` | `()` |  |  |  | 源码实锤 |
| `mt:__call` | `(...)` | ... |  | 运行触发器 | 源码实锤 |
| `mt:remove` | `()` |  |  | 摧毁触发器(移除全部事件) | 源码实锤 |
| `base.trigger_size` | `()` |  |  |  | 源码实锤 |
| `base.each_trigger` | `()` |  |  |  | 源码实锤 |
| `base.trigger` | `(event, callback)` | event, callback |  | 创建触发器 | 源码实锤 |
| `base.trig:new` | `(action, combine_args, scene, sync)` | action, combine_args, scene, sync | Trigger | 通过函数创建一个新的触发器 | 源码实锤 |
| `mt:add_event_common` | `(event)` | event |  |  | 源码实锤 |
| `mt:remove_event_common` | `(event)` | event |  |  | 源码实锤 |
| `mt:replicate` | `(include_event)` | include_event |  | 复制触发器 | 源码实锤 |
| `base.trigger_new_from_function` | `(func)` | func |  | 从函数创建回调 | 源码实锤 |
| `mt:add_event` | `(obj, name, custom_event, time, periodic)` | obj, name, custom_event, time, periodic |  |  | 源码实锤 |
| `mt:_add_scene_event` | `(obj, name, custom_event, time, periodic)` | obj, name, custom_event, time, periodic |  |  | 源码实锤 |
| `mt:_add_event` | `(obj, name, custom_event, time, periodic)` | obj, name, custom_event, time, periodic |  |  | 源码实锤 |
| `mt:_remove_event` | `(obj, name)` | obj, name |  |  | 源码实锤 |
| `mt:add_event_game_time` | `(time, periodic)` | time, periodic |  | base.game:event('单位-属性变化', function(trigger, unit, key, value) | 源码实锤 |
| `mt:add_event_game_time_internal` | `(time, periodic)` | time, periodic |  |  | 源码实锤 |
| `mt:set_action` | `(action)` | action |  |  | 源码实锤 |
| `args.event` | `(obj, evt_name)` | obj, evt_name | EventArgs |  | 源码实锤 |
| `args.event_unit` | `(obj, evt_name, unit)` | obj, evt_name, unit | UnitEventArgs |  | 源码实锤 |
| `args.event_unit_property_change` | `(obj, evt_name, unit, property, value)` | obj, evt_name, unit, property, value | UnitPropertyChangeEventArgs |  | 源码实锤 |
| `args.event_skill` | `(obj, evt_name, skill)` | obj, evt_name, skill | UnitSkillEventArgs |  | 源码实锤 |
| `args.event_skill_property_change` | `(obj, evt_name, skill, property, value)` | obj, evt_name, skill, property, value | UnitSkillPropertyChangeEventArgs |  | 源码实锤 |
| `args.event_skill_level_change` | `(obj, evt_name, skill, level)` | obj, evt_name, skill, level | UnitSkillLevelChangeEventArgs |  | 源码实锤 |
| `args.event_skill_stack_change` | `(obj, evt_name, skill, stack)` | obj, evt_name, skill, stack | UnitSkillStackChangeEventArgs |  | 源码实锤 |
| `args.event_skill_cooldown` | `(obj, evt_name, skill, time_remaining_ms, time_total_ms)` | obj, evt_name, skill, time_remaining_ms, time_total_ms | UnitSkillCooldownEventArgs |  | 源码实锤 |
| `args.event_unit_die` | `(obj, evt_name, unit, killer, type)` | obj, evt_name, unit, killer, type | UnitDieEventArgs |  | 源码实锤 |
| `args.event_unit_damage_dealt` | `(obj, evt_name, damage)` | obj, evt_name, damage |  | 伤害事件 | 源码实锤 |
| `args.event_unit_damage_taken` | `(obj, evt_name, damage)` | obj, evt_name, damage |  | 伤害事件 | 源码实锤 |
| `args.event_unit_buff` | `(obj, evt_name, unit, buff)` | obj, evt_name, unit, buff |  |  | 源码实锤 |
| `args.event_buff` | `(obj, evt_name, buff)` | obj, evt_name, buff |  |  | 源码实锤 |
| `args.event_buff_stack_change` | `(obj, evt_name, buff, stack, unit)` | obj, evt_name, buff, stack, unit | UnitSkillStackChangeEventArgs |  | 源码实锤 |
| `args.event_unit_purchase_item` | `(obj, evt_name, unit, item_name)` | obj, evt_name, unit, item_name |  |  | 源码实锤 |
| `args.event_unit_inventory` | `(obj, evt_name, unit, slot)` | obj, evt_name, unit, slot |  |  | 源码实锤 |
| `args.event_unit_inventory_target` | `(obj, evt_name, unit, slot, target)` | obj, evt_name, unit, slot, target |  |  | 源码实锤 |
| `args.event_unit_item` | `(obj, evt_name, unit, item, drop_mode)` | obj, evt_name, unit, item, drop_mode |  |  | 源码实锤 |
| `args.event_unit_cmd_request` | `(obj, evt_name, unit, command, target, key_modifier)` | obj, evt_name, unit, command, target, key_modifier |  |  | 源码实锤 |
| `args.event_unit_moved` | `(obj, evt_name, unit, pos_old, pos_new)` | obj, evt_name, unit, pos_old, pos_new |  |  | 源码实锤 |
| `args.event_unit_laned` | `(obj, evt_name, unit, vector_z)` | obj, evt_name, unit, vector_z |  |  | 源码实锤 |
| `args.event_unit_skill` | `(obj, evt_name, unit, skill)` | obj, evt_name, unit, skill |  |  | 源码实锤 |
| `args.event_unit_skill_stage` | `(obj, evt_name, unit, skill_id, time_elapsed_ms, time_total_ms)` | obj, evt_name, unit, skill_id, time_elapsed_ms, time_total_ms | UnitSkillCastEventArgs |  | 源码实锤 |
| `args.event_unit_skill_result` | `(obj, evt_name, unit, skill, result_code)` | obj, evt_name, unit, skill, result_code |  |  | 源码实锤 |
| `args.event_unit_xp` | `(obj, evt_name, xp_data)` | obj, evt_name, xp_data |  |  | 源码实锤 |
| `args.event_unit_mover` | `(obj, evt_name, unit, mover)` | obj, evt_name, unit, mover |  |  | 源码实锤 |
| `args.event_unit_scene` | `(obj, evt_name, unit, scene_name)` | obj, evt_name, unit, scene_name |  |  | 源码实锤 |
| `args.event_area` | `(obj, evt_name, area, unit)` | obj, evt_name, area, unit |  |  | 源码实锤 |
| `args.event_player` | `(obj, evt_name, player)` | obj, evt_name, player |  |  | 源码实锤 |
| `args.event_player_unit` | `(obj, evt_name, player, unit)` | obj, evt_name, player, unit |  |  | 源码实锤 |
| `args.event_player_team` | `(obj, evt_name, player, team)` | obj, evt_name, player, team |  |  | 源码实锤 |
| `args.event_player_property_change` | `(obj, evt_name, player, property, value)` | obj, evt_name, player, property, value | PlayerPropertyChangeEventArgs |  | 源码实锤 |
| `args.event_player_connect` | `(obj, evt_name, player, is_reconnect)` | obj, evt_name, player, is_reconnect |  |  | 源码实锤 |
| `args.event_player_chat` | `(obj, evt_name, player, msg)` | obj, evt_name, player, msg |  |  | 源码实锤 |
| `args.event_player_pick_hero` | `(obj, evt_name, player, hero_name)` | obj, evt_name, player, hero_name |  |  | 源码实锤 |
| `args.event_player_scene` | `(obj, evt_name, player, scene_name)` | obj, evt_name, player, scene_name |  |  | 源码实锤 |
| `args.event_player_config` | `(obj, evt_name, player, config)` | obj, evt_name, player, config |  |  | 源码实锤 |
| `args.event_player_ping` | `(obj, evt_name, player, ping)` | obj, evt_name, player, ping |  |  | 源码实锤 |
| `args.event_player_key_down` | `(obj, evt_name, player, key)` | obj, evt_name, player, key |  |  | 源码实锤 |
| `args.event_player_key_up` | `(obj, evt_name, player, key)` | obj, evt_name, player, key |  |  | 源码实锤 |
| `args.event_player_mouse_down` | `(obj, evt_name, player, key)` | obj, evt_name, player, key |  |  | 源码实锤 |
| `args.event_player_mouse_up` | `(obj, evt_name, player, key)` | obj, evt_name, player, key |  |  | 源码实锤 |
| `args.event_player_wheel_move` | `(obj, evt_name, player, delta_wheel)` | obj, evt_name, player, delta_wheel |  |  | 源码实锤 |
| `args.event_update` | `(obj, evt_name, delta)` | obj, evt_name, delta |  |  | 源码实锤 |
| `args.event_click` | `(obj, evt_name, screen_pos, actors_ID, button)` | obj, evt_name, screen_pos, actors_ID, button |  |  | 源码实锤 |
| `args.event_enter_foreground` | `(obj, evt_name, module_key)` | obj, evt_name, module_key |  |  | 源码实锤 |
| `args.event_property_change` | `(obj, evt_name, property, value)` | obj, evt_name, property, value |  |  | 源码实锤 |
| `args.event_message` | `(obj, evt_name, msg)` | obj, evt_name, msg |  |  | 源码实锤 |
| `args.event_message_timed` | `(obj, evt_name, msg, duration)` | obj, evt_name, msg, duration |  |  | 源码实锤 |
| `args.event_message_chat` | `(obj, evt_name, player_slot_id, type, msg, time)` | obj, evt_name, player_slot_id, type, msg, time |  |  | 源码实锤 |
| `args.event_resolution` | `(obj, evt_name, width, height)` | obj, evt_name, width, height |  |  | 源码实锤 |
| `args.event_scale` | `(obj, evt_name, scale)` | obj, evt_name, scale |  |  | 源码实锤 |
| `args.event_key` | `(obj, evt_name, key)` | obj, evt_name, key |  |  | 源码实锤 |
| `args.event_key_down` | `(obj, evt_name, key)` | obj, evt_name, key |  |  | 源码实锤 |
| `args.event_key_up` | `(obj, evt_name, key)` | obj, evt_name, key |  |  | 源码实锤 |
| `args.event_mouse_down` | `(obj, evt_name, key)` | obj, evt_name, key |  |  | 源码实锤 |
| `args.event_mouse_up` | `(obj, evt_name, key)` | obj, evt_name, key |  |  | 源码实锤 |
| `args.event_actor` | `(obj, evt_name, id)` | obj, evt_name, id |  |  | 源码实锤 |
| `args.event_actor_anim_message` | `(obj, evt_name, id, msg, anim)` | obj, evt_name, id, msg, anim |  |  | 源码实锤 |
| `args.event_actor_sound_message` | `(obj, evt_name, id, msg)` | obj, evt_name, id, msg |  |  | 源码实锤 |
| `args.event_scene` | `(obj, evt_name, scene_name)` | obj, evt_name, scene_name |  |  | 源码实锤 |
| `args.event_game_scene` | `(obj, evt_name, game, scene_name)` | obj, evt_name, game, scene_name |  |  | 源码实锤 |
| `args.event_eff_param` | `(obj, evt_name, ref_param)` | obj, evt_name, ref_param |  |  | 源码实锤 |
| `args.event_eff_param_impact_unit` | `(obj, evt_name, ref_param, impacted_unit)` | obj, evt_name, ref_param, impacted_unit |  |  | 源码实锤 |
| `args.event_custom_event` | `(obj, evt_name, custom_args)` | obj, evt_name, custom_args |  |  | 源码实锤 |
| `args.game_string_attribute_change` | `(obj, evt_name, game, key, value)` | obj, evt_name, game, key, value |  |  | 源码实锤 |
| `args.player_number_attribute_change` | `(obj, evt_name, player, key, value, value_change)` | obj, evt_name, player, key, value, value_change |  |  | 源码实锤 |
| `args.player_string_attribute_change` | `(obj, evt_name, player, key, value)` | obj, evt_name, player, key, value |  |  | 源码实锤 |
| `args.unit_number_attribute_change` | `(obj, evt_name, unit, key, value, value_change)` | obj, evt_name, unit, key, value, value_change |  |  | 源码实锤 |
| `args.unit_string_attribute_change` | `(obj, evt_name, unit, key, value)` | obj, evt_name, unit, key, value |  |  | 源码实锤 |
| `args.event_conversation` | `(obj, evt_name, speaker, listener, ref_param, conversation_link)` | obj, evt_name, speaker, listener, ref_param, conversation_link |  |  | 源码实锤 |
| `args.event_conversation_choose` | `(obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link)` | obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link |  |  | 源码实锤 |
| `args.event_inventory_item_tooltip` | `(obj, evt_name, item, item_tooltip_panel, slot_panel, inventory_panel)` | obj, evt_name, item, item_tooltip_panel, slot_panel, inventory_panel |  |  | 源码实锤 |
| `args.event_server_change_scene` | `(obj, evt_name, old_scene, new_scene)` | obj, evt_name, old_scene, new_scene |  |  | 源码实锤 |
| `args.event_scene_combind_area_notify` | `(obj, evt_name, from_scene, from_area, to_scene, to_area)` | obj, evt_name, from_scene, from_area, to_scene, to_area |  |  | 源码实锤 |
| `args.event_scene_combind_area_notifyB` | `(obj, evt_name, scene, area, target_scene)` | obj, evt_name, scene, area, target_scene |  |  | 源码实锤 |
| `args.event_spellbuild_preview` | `(obj, evt_name, owner, skill, spellbuild_unit_actor)` | obj, evt_name, owner, skill, spellbuild_unit_actor |  |  | 源码实锤 |
| `args.event_toast_show` | `(obj, evt_name, toast, text, source)` | obj, evt_name, toast, text, source |  |  | 源码实锤 |
| `args.event_menu_button` | `(obj, evt_name, Key)` | obj, evt_name, Key |  |  | 源码实锤 |
| `args.event_friend_list_init` | `(obj, evt_name, friend_data_list)` | obj, evt_name, friend_data_list |  |  | 源码实锤 |
| `args.event_friend_apply_list_init` | `(obj, evt_name, friend_apply_data_list)` | obj, evt_name, friend_apply_data_list |  |  | 源码实锤 |
| `args.event_friend_apply_list_state_change` | `(obj, evt_name, friend_apply_data)` | obj, evt_name, friend_apply_data |  |  | 源码实锤 |
| `evt:new` | `(obj, name)` | obj, name |  |  | 源码实锤 |
| `evt:remove` | `()` |  |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Trigger` | TSTL 类（2 键） |  | dump 实锤 |
| `Trigger.prototype` | table（23 键） |  | dump 实锤 |
| `Trigger.prototype.event` | table（3 键） |  | dump 实锤 |
| `Trigger.prototype.event.evt_args` | table（64 键） |  | dump 实锤 |
| `Trigger.prototype.event.dispatch_events` | table（12 键） |  | dump 实锤 |
| `Trigger.prototype.event.event_list` | table（121 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Trigger` | 无 | `is_enable`、`__call`、`_add_event`、`remove_event_common`、`new`、`disable`、`remove`、`__tostring`、`add_event`、`_add_scene_event`、`add_event_common`、`enable`、`add_event_game_time_internal`、`set_action`、`_remove_event`、`add_event_game_time`、`replicate` |

⚠️ **此处被截断，字段不全**：197 处（`<max depth exceeded>`），非 _descriptors 截断样本：`Trigger.prototype.event.evt_args.event_player_nickname` 等 197 处。全量截断路径见 keys_index.json / 对应 fields JSON。

### `@common/base/trigger_editor_v2`

- 来源：script 包（common 库）（`script\199\common\base\trigger_editor_v2\init.lua`）
- 加载：`require 'base.trigger_editor_v2'`（init.lua:162，仅 StateGame）；`require 'base.trigger_editor_v2'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

触编 V2 运行时入口（仅 StateGame 加载）：`__TS__Class2`/`base.force_as`/`base.instance_of`/`base.ArrayIterator`。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.ArrayIterator` | `(array)` | array |  |  | 源码实锤 |
| `__TS__Class2` | `(name)` | name |  |  | 源码实锤 |
| `c.prototype.____constructor` | `(self)` | self |  |  | 源码实锤 |
| `base.force_as` | `(classTbl, obj)` | classTbl, obj | any |  | 源码实锤 |
| `base.instance_of` | `(classTbl, obj)` | classTbl, obj |  |  | 源码实锤 |

### `@common/base/trigger_editor_v2/array`

- 来源：script 包（common 库）（`script\199\common\base\trigger_editor_v2\array.lua`）
- 加载：被 trigger_editor_v2/init.lua 装配；`require 'base.trigger_editor_v2.array'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 1，类 1）

触编 V2 的 TS `Array` 类（构造收可变参数填入数组）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `Array.prototype.____constructor` | `(self, T, ...)` | self, T, ... |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Array` | TSTL 类（2 键） |  | dump 实锤 |
| `Array.prototype` | table（3 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Array` | 无 | （仅构造/元方法） |

### `@common/base/try`

- 来源：script 包（common 库）（`script\199\common\base\try.lua`）
- 加载：`require 'base.try'`（init.lua:90）；`require 'base.try'`
- 状态：🔀 转发桩（`return require '@base.base.try'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 3 键，函数 2，类 0）

异常捕获工具（桩 → client_base；dump 揭示 try/try_wrap + FINALLY_RETURN 哨兵）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `try_wrap` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `try` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**lua_plus 扁平封装**（`server_lua_plus\14\base\base_lua_plus\` 对应模块，带触编 @ui 注解）【源码实锤】：

- `base.try_drop_item(item, callback)`
- `base.try_drop_item(item:item, callback:function<boolean>)`

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（3 键） |  | dump 实锤 |

### `@common/base/turn`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.turn'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

回合制相关（命名语义推测；lualib_bundle CLASSES 有 Turn* 类族佐证存在回合体系）。

语料中未反查到直接调用点；API 形态按命名语义推测【语义推测】。

### `@common/base/unit`

- 来源：script 包（common 库）（`script\199\common\base\unit.lua`）
- 加载：`require 'base.unit'`（init.lua:106，非 app 平台）；`require 'base.unit'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 377，类 2）

单位 `Unit` 类（最大玩法类，继承 Target；属性/技能/物品/状态机封装）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `mt:__tostring` | `()` |  |  |  | 源码实锤 |
| `mt:get_team_id` | `()` |  |  |  | 源码实锤 |
| `mt:is_visible` | `()` |  |  |  | 源码实锤 |
| `mt:get_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_string` | `(prop)` | prop |  |  | 源码实锤 |
| `mt:get_scene` | `()` |  |  |  | 源码实锤 |
| `mt:on_response` | `()` |  |  |  | 源码实锤 |
| `mt:get_scene_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_owner` | `()` |  |  |  | 源码实锤 |
| `mt:get_data` | `()` |  |  |  | 源码实锤 |
| `mt:set` | `(key, value)` | key, value |  |  | 源码实锤 |
| `mt:get` | `(key)` | key |  |  | 源码实锤 |
| `mt:is_alive` | `()` |  |  |  | 源码实锤 |
| `mt:get_level` | `()` |  |  |  | 源码实锤 |
| `mt:get_asset` | `()` |  |  |  | 源码实锤 |
| `mt:get_model_path` | `()` |  |  |  | 源码实锤 |
| `mt:get_skill_points` | `()` |  |  |  | 源码实锤 |
| `mt:get_snapshot` | `()` |  |  |  | 源码实锤 |
| `mt:each_skill` | `(type)` | type |  |  | 源码实锤 |
| `mt:each_skill_all` | `()` |  |  |  | 源码实锤 |
| `mt:has_label` | `(label)` | label |  |  | 源码实锤 |
| `mt:set_point` | `(scene_point)` | scene_point |  |  | 源码实锤 |
| `mt:destroy` | `()` |  |  |  | 源码实锤 |
| `mt:set_position` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:set_rotation` | `(x, y ,z)` | x, y ,z |  |  | 源码实锤 |
| `mt:set_scale_xyz` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:set_scale` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:has_restriction` | `(restriction)` | restriction |  |  | 源码实锤 |
| `mt:find_skill` | `(name, tp)` | name, tp |  |  | 源码实锤 |
| `mt:find_skill_by_slot` | `(slot)` | slot |  |  | 源码实锤 |
| `mt:get_attack` | `()` |  |  |  | 源码实锤 |
| `mt:find_buff` | `(name)` | name |  |  | 源码实锤 |
| `mt:each_buff` | `(target)` | target |  |  | 源码实锤 |
| `mt:each_buff_all` | `(target)` | target |  |  | 源码实锤 |
| `mt:get_class` | `()` |  |  |  | 源码实锤 |
| `mt:get_tag` | `()` |  |  |  | 源码实锤 |
| `mt:get_xy` | `()` |  |  |  | 源码实锤 |
| `mt:get_point` | `()` |  |  | 优先返回宿主坐标 | 源码实锤 |
| `mt:get_global_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_global_scene_point` | `()` |  |  |  | 源码实锤 |
| `mt:get_socket_point` | `(socket)` | socket |  |  | 源码实锤 |
| `mt:get_socket_position` | `(socket)` | socket |  |  | 源码实锤 |
| `mt:get_socket_rotation` | `(socket)` | socket |  |  | 源码实锤 |
| `mt:play_anim_ex` | `(anim_name, anim_param)` | anim_name, anim_param | AnimHandle? |  | 源码实锤 |
| `mt:get_anims` | `()` |  | table<ICustomAnimParams> |  | 源码实锤 |
| `mt:play_anim_bracket` | `()` |  |  |  | 源码实锤 |
| `mt:attach_to` | `(target, socket)` | target, socket |  |  | 源码实锤 |
| `mt:detach` | `()` |  |  |  | 源码实锤 |
| `mt:get_height` | `()` |  |  |  | 源码实锤 |
| `mt:get_facing` | `()` |  |  |  | 源码实锤 |
| `mt:get_highlight` | `()` |  |  |  | 源码实锤 |
| `mt:set_highlight` | `(on, ...)` | on, ... |  |  | 源码实锤 |
| `mt:get_outstroke` | `()` |  |  |  | 源码实锤 |
| `mt:set_outstroke` | `(enable, color, thickness)` | enable, color, thickness |  |  | 源码实锤 |
| `mt:set_shadow` | `(enable)` | enable |  |  | 源码实锤 |
| `mt:get_xray_enable` | `()` |  |  |  | 源码实锤 |
| `mt:set_xray_enable` | `(enable)` | enable |  |  | 源码实锤 |
| `mt:get_unit_random_model_index` | `()` |  |  |  | 源码实锤 |
| `mt:set_fow` | `(enable, radius)` | enable, radius |  |  | 源码实锤 |
| `mt:set_sight` | `(typ, param)` | typ, param |  |  | 源码实锤 |
| `mt:set_sight_skill_fan` | `(x, y, z, radius, angle)` | x, y, z, radius, angle |  |  | 源码实锤 |
| `mt:set_eye_height` | `(h)` | h |  |  | 源码实锤 |
| `mt:setup_occluding_camera_group` | `(...)` | ... |  |  | 源码实锤 |
| `mt:set_tint_enabled` | `(flag)` | flag |  |  | 源码实锤 |
| `mt:set_tint_color` | `(idx, clr)` | idx, clr |  | idx: 1/2/3 clr:{r, g, b, a} | 源码实锤 |
| `mt:set_tick_disabled` | `(on_or_off)` | on_or_off |  |  | 源码实锤 |
| `mt:is_item` | `()` |  |  |  | 源码实锤 |
| `mt:event_notify` | `(name, ...)` | name, ... |  |  | 源码实锤 |
| `mt:event` | `(name, f)` | name, f |  |  | 源码实锤 |
| `mt:cast` | `(skill, target, data)` | skill, target, data |  |  | 源码实锤 |
| `mt:move_to_direction` | `(x, y)` | x, y |  |  | 源码实锤 |
| `mt:stop_move_to_direction` | `(x, y)` | x, y |  |  | 源码实锤 |
| `mt:anim_play` | `(anim_name, params)` | anim_name, params |  |  | 源码实锤 |
| `sort_bracket` | `(bracket1,bracket2)` | bracket1,bracket2 |  |  | 源码实锤 |
| `add_bracket_to_table` | `(self, bracket_anim)` | self, bracket_anim |  | 添加bracket动画 | 源码实锤 |
| `mt:anim_play_bracket` | `(anim_birth, anim_stand, anim_death, params)` | anim_birth, anim_stand, anim_death, params |  | 手动构建BSD动画，然后play动画 | 源码实锤 |
| `mt:set_time_scale_global` | `(scale)` | scale |  |  | 源码实锤 |
| `mt:anim_set_paused_all` | `(paused)` | paused |  |  | 源码实锤 |
| `mt:unit_anim_operation` | `(value)` | value |  |  | 源码实锤 |
| `mt:learn_skill` | `(skill)` | skill |  |  | 源码实锤 |
| `mt:set_bone_chain_facing` | `(CHAIN_ID, angle, time)` | CHAIN_ID, angle, time |  |  | 源码实锤 |
| `mt:set_bone_chain_facing_v1` | `(angle, time)` | angle, time |  |  | 源码实锤 |
| `mt:reset_bone_chain_facing` | `(CHAIN_ID, time)` | CHAIN_ID, time |  |  | 源码实锤 |
| `mt:reset_bone_chain_facing_v1` | `(time)` | time |  |  | 源码实锤 |
| `get_destory_time` | `()` |  |  |  | 源码实锤 |
| `base.unit` | `(id)` | id | boolean? new |  | 源码实锤 |
| `alloc_unit_queue` | `()` |  |  | 定期清楚单位的逻辑 | 源码实锤 |
| `free_queue` | `(q)` | q |  |  | 源码实锤 |
| `add_destory_unit` | `(unit)` | unit |  |  | 源码实锤 |
| `remove_destory_unit` | `(unit)` | unit |  |  | 源码实锤 |
| `base.remove_unit` | `(id)` | id |  |  | 源码实锤 |
| `base.get_default_unit` | `(node_mark)` | node_mark |  |  | 源码实锤 |
| `base.get_default_item` | `(node_mark)` | node_mark |  |  | 源码实锤 |
| `set` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `modify_table` | `(ori_tbl, modify_tbl)` | ori_tbl, modify_tbl |  |  | 源码实锤 |
| `delete_table` | `(ori_tbl, modify_tbl)` | ori_tbl, modify_tbl |  |  | 源码实锤 |
| `set_by_sync` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `base.add_attribute_key` | `(name, id)` | name, id |  |  | 源码实锤 |
| `init_attribute` | `()` |  |  |  | 源码实锤 |
| `on_attr_anim_func` | `(self, key, value)` | self, key, value |  |  | 源码实锤 |
| `update_attribute` | `(self, attr)` | self, attr |  |  | 源码实锤 |
| `update_attribute_by_array` | `(attr)` | attr |  |  | 源码实锤 |
| `update_table_attribute` | `(self, attr)` | self, attr |  |  | 源码实锤 |
| `update_table_attribute_by_array` | `(attr)` | attr |  |  | 源码实锤 |
| `update_attribute_without_event` | `(self, attr)` | self, attr |  |  | 源码实锤 |
| `mt:attach_model` | `(path, hand_point, hold_point)` | path, hand_point, hold_point |  |  | 源码实锤 |
| `mt:detach_model` | `(path)` | path |  |  | 源码实锤 |
| `mt:change_model` | `(path)` | path |  |  | 源码实锤 |
| `mt:create_actor` | `(link, ignore_unit_list)` | link, ignore_unit_list |  |  | 源码实锤 |
| `mt:create_actors` | `(msg)` | msg |  |  | 源码实锤 |
| `mt:get_node_mark` | `()` |  |  |  | 源码实锤 |
| `on_unit_created` | `(id, attr)` | id, attr |  | 原GameUnit创建处理 | 源码实锤 |
| `on_light_unit_created` | `(unit_id, attr_map, is_actor)` | unit_id, attr_map, is_actor |  | 轻量单位创建处理 （包含GameUnit和同步Actor） | 源码实锤 |
| `base.event.on_controlled_sync_unit_created` | `(id, scene_name, unit_type_id, unit_slot)` | id, scene_name, unit_type_id, unit_slot |  |  | 源码实锤 |
| `mt:destroy_actors` | `(msg)` | msg |  |  | 源码实锤 |
| `base.event.on_unit_attributes_changed` | `(data, new)` | data, new |  |  | 源码实锤 |
| `base.event.on_unit_table_attributes_changed` | `(data, new)` | data, new |  |  | 源码实锤 |
| `base.event.on_unit_model_changed` | `(id, path)` | id, path |  |  | 源码实锤 |
| `on_unit_destory` | `(id)` | id |  | 原GameUnit销毁处理 | 源码实锤 |
| `on_light_unit_destroy` | `(unit_id)` | unit_id |  | 轻量单位销毁处理 （包含GameUnit和同步Actor） | 源码实锤 |
| `base.event.on_unit_attach_changed` | `(unit_id, attach_id)` | unit_id, attach_id |  | 轻量单位附着事件处理 （c++不处理具体附着逻辑，由脚本统一处理，其中包含GameUnit和同步Actor） | 源码实锤 |
| `base.event.on_unit_hovered` | `(id)` | id |  |  | 源码实锤 |
| `mt:set_blood_bar_visible` | `(visible)` | visible |  |  | 源码实锤 |
| `mt:set_status_bar_visibility` | `(visible)` | visible |  | 设置血条是否显示（暴露到触发的api） | 源码实锤 |
| `sync_unit_actor` | `(unit, key, value)` | unit, key, value |  |  | 源码实锤 |
| `mt:set_blood_bar_template` | `(template_name)` | template_name |  |  | 源码实锤 |
| `mt:set_blood_bar_widget` | `(key, value)` | key, value |  |  | 源码实锤 |
| `base.event.on_unit_blood_bar_created` | `(unit_id)` | unit_id |  |  | 源码实锤 |
| `mt:set_minimap_icon_visible` | `(visible)` | visible |  |  | 源码实锤 |
| `base.unit_info` | `()` |  |  |  | 源码实锤 |
| `mt:create_riseletter` | `(position ,text, type, color, fontsize)` | position ,text, type, color, fontsize |  |  | 源码实锤 |
| `mt:create_riseletter_by_link` | `(position ,text, link, color, fontsize)` | position ,text, link, color, fontsize |  |  | 源码实锤 |
| `mt:create_riseletter_by_templatename` | `(position ,text, template_name, color, fontsize)` | position ,text, template_name, color, fontsize |  |  | 源码实锤 |
| `mt:remove_riseletter` | `(riseletter)` | riseletter |  |  | 源码实锤 |
| `mt:set_riseletter_position` | `(riseletter, position)` | riseletter, position |  |  | 源码实锤 |
| `mt:create_riseletter_without_color_size` | `(location,text,text_type)` | location,text,text_type |  |  | 源码实锤 |
| `mt:create_riseletter_with_color_size` | `(location,text,text_type,color,size)` | location,text,text_type,color,size |  |  | 源码实锤 |
| `mt:try_pick_item` | `(item, callback)` | item, callback |  |  | 源码实锤 |
| `mt:get_or_create_state_machine` | `(name, priority, layer)` | name, priority, layer |  |  | 源码实锤 |
| `mt:remove_state_machine` | `(sm_name)` | sm_name |  |  | 源码实锤 |
| `base.event.on_unit_state_machine_changed` | `(unit_id, state_machines)` | unit_id, state_machines |  |  | 源码实锤 |
| `base.event.on_unit_state_machine_transit` | `(unit_id, sm_name, event_id)` | unit_id, sm_name, event_id |  |  | 源码实锤 |
| `mt:is_valid` | `()` |  |  |  | 源码实锤 |
| `mt:execute_on` | `(target,link, cache_override)` | target,link, cache_override | CmdResult |  | 源码实锤 |
| `mt:execute_on_point` | `(target,link, cache_override)` | target,link, cache_override | CmdResult |  | 源码实锤 |
| `mt:get_unit` | `()` |  |  |  | 源码实锤 |
| `mt:set_rotation` | `(x, y, z)` | x, y, z |  |  | 源码实锤 |
| `mt:get_all_items` | `()` |  | Item[] |  | 源码实锤 |
| `mt:get_display_name` | `()` |  |  |  | 源码实锤 |
| `mt:set_display_name` | `(name)` | name |  |  | 源码实锤 |
| `mt:get_inventory_items` | `(inv_idx)` | inv_idx |  |  | 源码实锤 |
| `base.get_units_from_screen_xy` | `(xy, is_accurate)` | xy, is_accurate |  |  | 源码实锤 |
| `try_load_show_methods` | `()` |  |  |  | 源码实锤 |
| `mt:get_show_name` | `()` |  |  |  | 源码实锤 |
| `mt:get_icon` | `()` |  |  |  | 源码实锤 |
| `mt:get_tips` | `()` |  |  |  | 源码实锤 |
| `mt:get_current_cd` | `()` |  |  |  | 源码实锤 |
| `mt:get_cd_max` | `()` |  |  |  | 源码实锤 |
| `mt:set_disappear_destory_time` | `(time)` | time |  | 设置单位离开视野的销毁时间 | 源码实锤 |
| `mt:get_cooldown` | `(cooldown_key)` | cooldown_key |  |  | 源码实锤 |
| `mt:get_cooldown_max` | `(cooldown_key)` | cooldown_key |  |  | 源码实锤 |
| `mt:insert_into_cooldown_map` | `(cooldown_key, skill)` | cooldown_key, skill |  |  | 源码实锤 |
| `mt:is_cooldown_map_empty` | `()` |  |  |  | 源码实锤 |
| `mt:remove_from_cooldown_map` | `(cooldown_key, skill)` | cooldown_key, skill |  |  | 源码实锤 |
| `mt:register_bone_chain` | `(CHAIN_ID, bone_chain_data)` | CHAIN_ID, bone_chain_data |  | 参考 https://xindong.atlassian.net/wiki/spaces/Editor/pages/1060713486 | 源码实锤 |
| `mt:register_model_bone_chain` | `(bol)` | bol |  | 开放给触发用户用的，应用模型配的数据 | 源码实锤 |
| `mt:test_build_box` | `(min, max, test_type)` | min, max, test_type |  | test_type: 0-粗糙（允许些微的高低不平） 1-严格 2-浮空 | 源码实锤 |
| `base.event.on_unit_cool_down` | `(unit_id, cooldown_key)` | unit_id, cooldown_key |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Unit` | TSTL 类（3 键） |  | dump 实锤 |
| `Unit.prototype` | table（386 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Unit` | extends `Target` | `is_neutral_to`、`set_random_model_index`、`insert_into_cooldown_map`、`change_type`、`add_height`、`get_attack`、`set_height`、`clear_scale`、`find_quest`、`add_provide_sight`、`set_attribute_min`、`create_unit`、`get_attach_unit`、`ai_damage`、`update_unit_buffs`、`get_attribute_max_inarray`、`get_attribute_min`、`set_ignore_rocker`、`get_creation_param`、`learn_skill`、`kill`、`get_or_create_state_machine`、`is_walking`、`get_scene_name`、`is_visible_to`、`add_state_machine`、`get_quest_conditions`、`add`、`add_level`、`replace_skill_internal`、`find_item`、`each_item`、`anim_play_time`、`get_walking_target`、`set_status_bar_visibility`、`get_z_speed`、`get_owner`、`set_location_async`、`get_attackable_radius`、`get_string`、`sync_state_machines`、`get_table_attr`、`set_cooldown`、`get_restriction_internal`、`remove_mark`、`set_custom`、`event_dispatch`、`ride_on`、`can_attack`、`get_effect_target`、`has_component`、`create_response`、`blink`、`set_owner`、`get_ride_unit`、`try_event_subscribe`、`on_response`、`create_inventorys`、`use_item_on_angle`、`add_exp`、…（共 376 个，全量见 `parsed/fields` 对应 JSON） |

**运行时存在但源码未见**（动态注册 / 继承自 ____super 的方法，按末段名启发式匹配）：`Unit.prototype.is_neutral_to`、`Unit.prototype.set_random_model_index`、`Unit.prototype.change_type`、`Unit.prototype.add_height`、`Unit.prototype.set_height`、`Unit.prototype.clear_scale`、`Unit.prototype.find_quest`、`Unit.prototype.add_provide_sight`、`Unit.prototype.set_attribute_min`、`Unit.prototype.create_unit`、`Unit.prototype.get_attach_unit`、`Unit.prototype.ai_damage`、`Unit.prototype.update_unit_buffs`、`Unit.prototype.get_attribute_max_inarray`、`Unit.prototype.get_attribute_min`、`Unit.prototype.set_ignore_rocker`、`Unit.prototype.get_creation_param`、`Unit.prototype.kill`、`Unit.prototype.is_walking`、`Unit.prototype.is_visible_to`、`Unit.prototype.add_state_machine`、`Unit.prototype.get_quest_conditions`、`Unit.prototype.add`、`Unit.prototype.add_level`、`Unit.prototype.replace_skill_internal`、`Unit.prototype.find_item`、`Unit.prototype.each_item`、`Unit.prototype.anim_play_time`、`Unit.prototype.get_walking_target`、`Unit.prototype.get_z_speed`、`Unit.prototype.set_location_async`、`Unit.prototype.get_attackable_radius`、`Unit.prototype.sync_state_machines`、`Unit.prototype.get_table_attr`、`Unit.prototype.set_cooldown`、`Unit.prototype.get_restriction_internal`、`Unit.prototype.remove_mark`、`Unit.prototype.set_custom`、`Unit.prototype.event_dispatch`、`Unit.prototype.ride_on`、…（共 321 个，全量见 `parsed/fields` 对应 JSON）

### `@common/base/utility`

- 来源：script 包（common 库）（`script\199\common\base\utility.lua`）
- 加载：`include 'base.utility'`（init.lua:69）；`require 'base.utility'`
- 状态：✅ 有源码
- dump 值：table（顶层 3 键，函数 0，类 3）

工具函数壳：转发 client_base utility + 本地 `base.hash`/枚举查询。

**dump 对照差异**：源码 `return require '@base.base.utility'` 的结果 + 本地新增 `base.hash`/`base.get_appendable_enum`/`base.get_appendable_keys`（全局副作用）【源码实锤】；dump 值树显示 client_base utility 的运行时返回 = `{ Mover, Region, Target }` 三个 TS 基类【dump 实锤】——`Target` 即 scene_point/unit/snapshot 的共同基类（`_G.Target = utility.Target`，utility.lua:2）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `base.hash` | `(str)` | str |  |  | 源码实锤 |
| `base.get_appendable_enum` | `(key)` | key |  |  | 源码实锤 |
| `base.get_appendable_keys` | `(key)` | key |  |  | 源码实锤 |
| `io.load` | `(filename, mode)` | filename, mode |  |  | 源码实锤 |
| `base.split` | `(str, p)` | str, p |  |  | 源码实锤 |
| `base.string_format` | `(context, params)` | context, params | string |  | 源码实锤 |
| `base.utf8_sub` | `(s, i, j)` | s, i, j |  |  | 源码实锤 |
| `base.to_type` | `(value, expect_type)` | value, expect_type |  |  | 源码实锤 |
| `base.get_unit_name` | `(type_id)` | type_id |  |  | 源码实锤 |
| `base.image_path` | `(path)` | path |  |  | 源码实锤 |
| `base.load_string` | `(str, skill)` | str, skill |  |  | 源码实锤 |
| `base.get_x` | `(obj)` | obj |  |  | 源码实锤 |
| `base.get_y` | `(obj)` | obj |  |  | 源码实锤 |
| `base.remove` | `(obj)` | obj |  |  | 源码实锤 |
| `base.default` | `(v, default)` | v, default |  |  | 源码实锤 |
| `gc_mt:__shl` | `(obj)` | obj |  |  | 源码实锤 |
| `gc_mt:flush` | `()` |  |  |  | 源码实锤 |
| `base.gc` | `()` |  |  |  | 源码实锤 |
| `base.calc_http_server_address` | `(server_name, default_port)` | server_name, default_port |  |  | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（3 键） |  | dump 实锤 |
| `Mover` | TSTL 类（2 键） |  | dump 实锤 |
| `Mover.prototype` | table（2 键） |  | dump 实锤 |
| `Region` | TSTL 类（2 键） |  | dump 实锤 |
| `Region.prototype` | table（2 键） |  | dump 实锤 |
| `Target` | TSTL 类（2 键） |  | dump 实锤 |
| `Target.prototype` | table（2 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Mover` | 无 | （仅构造/元方法） |
| `Region` | 无 | （仅构造/元方法） |
| `Target` | 无 | （仅构造/元方法） |

### `@common/base/validator`

- 来源：script 包（common 库）（**无源码**：api-13 全语料无此文件，引擎内嵌 TS 模块或未分发）
- 加载：未见于 `common/base/init.lua` 装配链【语义推测：引擎预注册 / 内嵌 TS 模块加载】；`require 'base.validator'`
- 状态：⚠️ 无源码
- dump 值：`true`（已加载无表导出）

触发器数据校验器；运行时另有 lua_plus `base.validator_*` 扁平封装。

**lua_plus 扁平封装**（`server_lua_plus\14\base\base_lua_plus\` 对应模块，带触编 @ui 注解）【源码实锤】：

- `base.validator_and(code1, code2)`
- `base.validator_and(code1:验证器代码, code2:验证器代码)`
- `base.validator_condition(condition)`
- `base.validator_condition(condition:boolean)`
- `base.validator_not(code1)`
- `base.validator_not(code1:验证器代码)`
- `base.validator_or(code1, code2)`
- `base.validator_or(code1:验证器代码, code2:验证器代码)`
- `base.validator_unit_filter(eff_param, unit, filters)`
- `base.validator_unit_filter(eff_param:eff_param, unit:效果节点单位位置, filters:string)`
- `base.validator_unit_filter_new(eff_param, unit, filters)`
- `base.validator_unit_filter_new(eff_param:eff_param, unit:效果节点单位位置, filters:target_filter)`
- `base.validator_unit_has_buff(eff_param, unit, buff_id_name)`
- `base.validator_unit_has_buff(eff_param:eff_param, unit:效果节点单位位置, buff_id_name:buff_id)`

### `@common/base/vector`

- 来源：script 包（common 库）（`script\199\common\base\vector.lua`）
- 加载：`require 'base.vector'`（init.lua:71）；`require 'base.vector'`
- 状态：✅ 有源码
- dump 值：table（顶层 1 键，函数 5，类 1）

三维向量 `Vector` 类。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `create_vector` | `(X, Y, Z)` | X, Y, Z |  |  | 源码实锤 |
| `mt:vector_addition` | `(VectorB)` | VectorB |  | 向量加法 | 源码实锤 |
| `mt:vector_subtraction` | `(VectorB)` | VectorB |  | 向量减法 | 源码实锤 |
| `mt:vector_multiplication` | `(VectorB)` | VectorB |  | 向量乘法(点乘) | 源码实锤 |
| `mt:get_vector_length` | `()` |  |  | 获取向量长度 | 源码实锤 |
| `mt:get_unit_vector` | `()` |  |  | 获取单位向量 | 源码实锤 |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（1 键） |  | dump 实锤 |
| `Vector` | TSTL 类（2 键） |  | dump 实锤 |
| `Vector.prototype` | table（8 键） |  | dump 实锤 |

**类**

| 类 | 继承 | 方法（dump 实锤） |
| --- | --- | --- |
| `Vector` | 无 | `vector_multiplication`、`vector_addition`、`get_unit_vector`、`vector_subtraction`、`get_vector_length` |

### `@common/base/voice`

- 来源：script 包（common 库）（`script\199\common\base\voice.lua`）
- 加载：`require 'base.voice'`（init.lua:137）；`require 'base.voice'`
- 状态：✅ 有源码
- dump 值：`true`（已加载无表导出）

语音房间封装（rpc.join_voice_room/voice_black_list + 语音事件）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `get_auth` | `(appid,room, user_id)` | appid,room, user_id |  |  | 源码实锤 |
| `rpc.join_voice_room` | `(room, team, range, cb)` | room, team, range, cb |  |  | 源码实锤 |
| `rpc.voice_black_list` | `(p, mute)` | p, mute |  |  | 源码实锤 |

### `@common/preload/lni_loader`

- 来源：script 包（common 库）（`script\199\common\preload\lni_loader.lua`）
- 加载：preload 机制（引擎在库加载前预执行 preload/ 目录）；`require 'preload.lni_loader'`
- 状态：🔀 转发桩（`return require '@base.preload.lni_loader'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 6 键，函数 6，类 0）

lni 加载器（桩 → client_base；dump 揭示 loader/packager/format 等 6 函数）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `initialize_computed` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `format` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `set_marco` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `loader` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `packager` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `normalize` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（6 键） |  | dump 实锤 |

### `@common/preload/reload`

- 来源：script 包（common 库）（`script\199\common\preload\reload.lua`）
- 加载：preload 机制（引擎在库加载前预执行 preload/ 目录）；`require 'preload.reload'`
- 状态：🔀 转发桩（`return require '@base.preload.reload'`，实现在 client_base 库，不在本包）
- dump 值：table（顶层 3 键，函数 3，类 0）

热重载（桩 → client_base；dump 揭示 reload/reload_event/raw_include）。

**函数**

| 函数 | 签名 | 参数 | 返回 | 说明 | 置信 |
| --- | --- | --- | --- | --- | --- |
| `raw_include` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `reload_event` | `(?)` |  |  |  | dump 实锤（实现 client_base） |
| `reload` | `(?)` |  |  |  | dump 实锤（实现 client_base） |

**字段/子表**（dump 值树实锤）

| 路径 | 类型 | 说明 | 置信 |
| --- | --- | --- | --- |
| `(模块顶层)` | table（3 键） |  | dump 实锤 |

---

## 覆盖报告

- 本组 93/93 键全覆盖（keys_index.json group=common-base 过滤）。
- 有源码 60 / 无源码 33；转发桩 8；含截断 8 键（均已在模块节标注）。
- 值形态：true=已加载无表导出；table=已展开值树；function=模块返回函数（lni_writer）。
