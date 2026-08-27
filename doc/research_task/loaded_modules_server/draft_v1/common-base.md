# script 包 · common 库（base 基础层）

模块数：93。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）。

源码覆盖：60/93；其余标注 ⚠️ 无源码并附调用点反查/语义推测。

---

### `@common`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@common/base`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\init.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `safe_callback` | `(name)` |  |
| `__newindex` | `(self, k, v)` |  |
| `base.error` | `(err,...)` |  |
| `base.callback_info` | `()` |  |
| `base.game.fff` | `()` |  |

### `@common/base/actor`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\actor.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__tostring` | `()` |  |
| `base.set_actor_map` | `(actor)` |  |
| `base.set_actor_mode` | `(allow_ray_cast)` | comment |
| `base.set_unit_highlight_on` | `(unit,r,g,b,a,time)` |  |
| `base.set_unit_highlight_off` | `(unit)` |  |
| `base.actor` | `(name, sid, skip_birth, scene)` |  |
| `base.actor_from_id` | `(id)` | comment → Actor |
| `base.actor_from_sid` | `(id)` | comment → Actor |
| `mt:set_destroy_on_orphan` | `(destroy)` |  |
| `mt:is_destroy_on_orphan` | `()` | comment → boolean |
| `mt:release` | `()` |  |
| `mt:destroy` | `(force)` |  |
| `mt:set_owner` | `(owner_id)` |  |
| `mt:set_shadow` | `(enable)` |  |
| `mt:set_point` | `(scene_point)` |  |
| `mt:set_position` | `(x, y, z)` |  |
| `mt:get_world_position` | `()` |  |
| `mt:get_position` | `()` |  |
| `mt:set_ground_z` | `(z)` |  |
| `mt:set_position_from` | `(target, socket)` |  |
| `mt:set_rotation` | `(x, y, z)` |  |
| `mt:get_rotation` | `()` |  |
| `mt:set_facing` | `(angle)` |  |
| `mt:get_socket_position` | `(socket)` |  |
| `mt:get_socket_rotation` | `(socket)` |  |
| `mt:set_scale` | `(x, y, z)` |  |
| `mt:set_scale_xyz` | `(x, y, z)` |  |
| `mt:actor_set_scale` | `(x)` |  |
| `mt:set_asset` | `(asset)` |  |
| `mt:set_fow` | `(enable, radius)` |  |
| `mt:set_grid_size` | `(size)` |  |
| `mt:set_grid_range` | `(start_id, range)` |  |
| `mt:set_grid_state` | `(grid_id, state)` |  |
| `mt:set_grid_stick_to_ground` | `(enable)` |  |
| `mt:attach_to` | `(target, socket)` |  |
| `mt:attach_to_anchor` | `(anchor_name)` |  |
| `mt:set_bearings` | `(x, y, z, facing, use_ground_height)` | comment |
| `mt:finalize_bearings` | `()` |  |
| `mt:detach` | `()` |  |
| `mt:show` | `(status)` |  |
| `mt:play` | `()` |  |
| `mt:play_anim_ex` | `(anim_name, anim_param)` | comment |
| `mt:get_anims` | `()` | comment → table<ICustomAnimParams> |
| `mt:play_animation` | `(anim, params)` |  |
| `mt:stop` | `(fade)` |  |
| `mt:pause` | `()` |  |
| `mt:resume` | `()` |  |
| `mt:set_volume` | `(volume)` |  |
| `mt:get_highlight` | `()` |  |
| `mt:set_highlight` | `(on, ...)` |  |
| `mt:set_material_parameters` | `(...)` |  |
| `mt:set_launch_site` | `(unit, socket)` |  |
| `mt:set_impact_site` | `(unit, socket)` |  |
| `mt:set_launch_position` | `(x, y, z)` |  |
| `mt:set_launch_scene_point` | `(point)` |  |
| `mt:set_launch_ground_z` | `(z)` |  |
| `mt:set_text` | `(text)` |  |
| `pos_distance` | `(p1, p2)` |  |
| `sub_class_action.CameraShake` | `(self, cache)` |  |
| `mt:do_sub_class_action` | `()` | comment |
| `mt:create_actor` | `(link)` | comment |
| `mt:create_actors` | `(msg)` |  |
| `mt:destroy_actors` | `(msg)` |  |
| `base.actor_info` | `()` |  |
| `base.get_actor_from_id` | `(id)` |  |
| `base.get_actor_from_sid` | `(id)` |  |
| `mt:anim_play` | `(anim_name, params)` |  |
| `mt:set_time_scale_global` | `(scale)` | 设置全局播放速度，只影响新API播放的动画 |
| `sort_bracket` | `(bracket1,bracket2)` |  |
| `add_bracket_to_table` | `(self, bracket_anim)` | 添加bracket动画 |
| `mt:anim_play_bracket` | `(anim_birth, anim_stand, anim_death, params)` | 手动构建BSD动画，然后play动画 |
| `mt:anim_set_paused_all` | `(paused)` |  |
| `mt:anim_operation` | `(op, params, ...)` |  |
| `mt:register_bone_chain` | `(CHAIN_ID, bone_chain_data)` | 参考 https://xindong.atlassian.net/wiki/spaces/Editor/pages/1060713486 |
| `mt:register_model_bone_chain` | `(bol)` | 开放给触发用户用的，应用模型配的数据 |
| `mt:set_bone_chain_facing` | `(CHAIN_ID, angle, time)` |  |
| `mt:set_bone_chain_facing_v1` | `(angle, time)` |  |
| `mt:reset_bone_chain_facing` | `(CHAIN_ID, time)` |  |
| `mt:reset_bone_chain_facing_v1` | `(time)` |  |
| `base.get_actors_from_screen_xy` | `(xy)` |  |
| `base.play_sound_effect` | `(link)` | 创建并播放2D音效 |
| `base.create_beam_effect` | `(link, source, target)` | → Actor |
| `base.actor_enable_raycast` | `(actor, enable)` |  |

### `@common/base/ad`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\ad.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `show_reward_video_ad` | `(reward, reward_amount, extra, cb)` |  |
| `cb` | `(val)` |  |

### `@common/base/admin`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/ai`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.ai_attack_add_team_threat(ai_attack, team, threat)`
  - `base.ai_attack_add_team_threat(ai_attack:ai_attack, team:integer, threat:integer)`
  - `base.ai_attack_add_type_threat(ai_attack, unit_tag, threat)`
  - `base.ai_attack_add_type_threat(ai_attack:ai_attack, unit_tag:单位标签, threat:integer)`
  - `base.ai_attack_add_unit_threat(ai_attack, unit, threat)`
  - `base.ai_attack_add_unit_threat(ai_attack:ai_attack, unit:unit, threat:integer)`
  - `base.ai_attack_remove(ai_attack)`
  - `base.ai_attack_remove(ai_attack:ai_attack)`

### `@common/base/ai_searcher`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/anim_handlers`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\anim_handlers.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_anim_map` | `()` |  |
| `base.get_anim_bracket_map` | `()` |  |
| `base.anim` | `(anim_name, owner_type, owner_id, owner_name, params)` |  |
| `base.bracket_anim` | `(anim_birth, anim_stand, anim_death, params, owner_type, owner_id, owner_name)` |  |
| `mt:play` | `(anim, loop, speed, blend_time)` |  |
| `mt:get_unit_or_actor` | `()` |  |
| `mt:replay` | `()` |  |
| `mt:refresh_global_pause` | `(paused)` |  |
| `mt:pause` | `()` |  |
| `mt:resume` | `()` |  |
| `mt:set_time` | `(time, trigger_events)` |  |
| `mt:set_time_scale` | `(scale)` |  |
| `mt:set_time_scale_absolute` | `(scale)` |  |
| `mt:set_percentage` | `(percentage)` |  |
| `mt:set_duration` | `(duration)` |  |
| `mt:destroy` | `()` |  |
| `mt:bracket_stop` | `()` |  |
| `mt:check_valid` | `()` | 检查该句柄的有效性 |
| `mt:remove` | `()` |  |

### `@common/base/array`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\array.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__index` | `(pos)` |  |
| `mt:__newindex` | `(pos, value)` |  |
| `mt:__len` | `()` |  |
| `mt:__pairs` | `()` |  |
| `set_len` | `(self, len)` |  |
| `insert` | `(self, pos, value)` |  |
| `remove` | `(self, pos)` |  |
| `random` | `(self)` |  |
| `convert` | `(self, t)` |  |
| `base.array` | `(default, t)` |  |

### `@common/base/auxiliary`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `add_animation(unit, animation_name, scale, is_loop, part)`
  - `get_game_mode_args()`
  - `get_map_kind()`
  - `get_player_id(base.local_player()`
  - `get_player_id(operatorPlayer)`
  - `get_player_id(player)`
  - `get_player_id(playerObj)`
  - `get_system_time()`

### `@common/base/behavior`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\behavior.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.proto.unit_get_interaction_spell` | `(msg)` |  |
| `base.proto.unit_remove_interaction_spell` | `(msg)` |  |
| `base.refresh_interact_joystick` | `()` |  |
| `init` | `()` |  |

### `@common/base/buff`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\buff.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get_buff_name_by_hash` | `(hash)` |  |
| `mt:__tostring` | `()` |  |
| `mt:get_name` | `()` |  |
| `set_remaining` | `(self, remaining)` |  |
| `mt:get_remaining` | `()` |  |
| `set_time` | `(self, time)` |  |
| `mt:get_time` | `()` |  |
| `mt:pause` | `()` |  |
| `mt:resume` | `()` |  |
| `mt:update_paused` | `()` |  |
| `mt:get_owner` | `()` |  |
| `set_stack` | `(self, stack , send_event)` |  |
| `mt:get_stack` | `()` |  |
| `mt:event_notify` | `(name, ...)` |  |
| `mt:event` | `(name, f)` |  |
| `ac_buff` | `(unit_id, hash, index)` |  |
| `try_load_show_methods` | `()` |  |
| `mt:get_show_name` | `()` |  |
| `mt:get_icon` | `()` |  |
| `mt:get_tips` | `()` |  |
| `mt:get_current_cd` | `()` |  |
| `mt:get_cd_max` | `()` |  |
| `base.event.on_buff_attached` | `(unit_id, hash, index, time, remaining, stack)` |  |
| `base.event.on_buff_detached` | `(unit_id, hash, index)` |  |
| `base.event.on_buff_update` | `(unit_id, hash, index, time, remaining, stack)` |  |

### `@common/base/channeler`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/cheat`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\cheat.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `gm.showmovejoystick` | `(cmd)` |  |
| `set` | `(self, props)` |  |
| `base.proto.__gm_debug_unit` | `(msg)` |  |
| `base.proto.__gm_debug_player` | `(msg)` |  |
| `set` | `(self, all_trace_player_props)` |  |
| `set` | `(self, props)` |  |
| `base.proto.__gm_debug_game` | `(msg)` |  |
| `eff_destroy` | `(root_id, id, force)` |  |
| `eff_destroy_all` | `()` |  |
| `base.proto.__gm_debug_eff_destory_all` | `(msg)` |  |
| `base.proto.__gm_debug_eff_destory` | `(msg)` |  |
| `base.proto.__gm_debug_eff_info` | `(msg)` |  |
| `get_unit_point` | `(eff_data)` | 是单位则更新单位所处地点 |
| `draw_circle_area` | `(eff_data,actor, color)` |  |
| `draw_arc_area` | `(eff_data,actor)` |  |
| `draw_line_area` | `(eff_data,actor)` |  |
| `get_eff_method` | `(eff_data)` |  |
| `draw_line` | `(point, parent_point, actor, color)` |  |
| `base.cheat.VRP` | `(eff_data)` |  |
| `base.proto.__gm_debug_eff_keep` | `(msg)` |  |
| `base.cheat.VAO_cast` | `(source_id, target_id, info)` | 将data中的目标与来源通过红线连接，并在来源头上标注技能信息 |
| `base.cheat.VAO_approach` | `(source_id, target_id, info)` |  |
| `base.cheat.VAO_approach_destory` | `(source_id)` |  |
| `base.proto.__gm_debug_ai_order` | `(msg)` |  |

### `@common/base/circle`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\circle.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:get_point` | `()` |  |
| `mt:get_scene_point` | `()` |  |
| `mt:get_range` | `()` |  |
| `mt:random_point` | `()` |  |
| `mt:scene_random_point` | `()` |  |
| `mt:init_region` | `(filter)` |  |
| `region:on_enter` | `(unit)` |  |
| `region:on_leave` | `(unit)` |  |
| `mt:remove_region` | `()` |  |
| `base.circle` | `(point, range, scene_name)` |  |

### `@common/base/class`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\class.lua`）
- ⚠️ 本文件是**转发桩**：`return require '@base.base.class'`（实现不在本包，见下方推测）


### `@common/base/cmd_result`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\cmd_result.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.cmd_result:new` | `()` | comment → CmdResult |
| `base.cmd_result:__eq` | `(other)` | comment → boolean |
| `base.cmd_result:__lt` | `(other)` | comment → boolean |
| `base.cmd_result:__le` | `(other)` | comment → boolean |
| `base.cmd_result:get_value` | `()` | comment → integer |
| `base.cmd_result:get_text` | `()` |  |

### `@common/base/co`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\co.lua`）
- ⚠️ 本文件是**转发桩**：`return require '@base.base.co'`（实现不在本包，见下方推测）


### `@common/base/collision_flags`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\collision_flags.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.collision_flags` | `(mask)` |  |
| `mt:contains` | `(flag)` | 是否包含某一类型碰撞 |
| `mt:each_collision` | `(callback)` | 遍历为真的碰撞 |

### `@common/base/crop`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/damage`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.damage_get_angle(damage:damage)`
  - `base.damage_get_current_damage(damage)`
  - `base.damage_get_current_damage(damage:damage)`
  - `base.damage_get_damage(damage)`
  - `base.damage_get_damage(damage:damage)`
  - `base.damage_get_type(damage)`
  - `base.damage_get_type(damage:damage)`
  - `base.damage_set_current_damage(damage, amount)`
  - `base.damage_set_current_damage(damage:damage, amount:number)`

### `@common/base/datetime`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/deque`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\deque.lua`）
- ⚠️ 本文件是**转发桩**：`return require '@base.base.deque'`（实现不在本包，见下方推测）


### `@common/base/detection`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `check_text(data.text,function(suggestion)`
  - `check_text(name, function(suggestion)`
  - `check_text(nick, function(suggestion)`
  - `check_text(text,function(suggestion,list)`

### `@common/base/eff`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\eff.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `eff.init_cache` | `()` |  |
| `eff.merge_cache` | `(in_cache)` |  |
| `eff.has_cache_init` | `()` |  |
| `eff.cache_init_finished` | `()` |  |
| `eff.caches` | `(node_type)` | comment |
| `eff.all_caches` | `(node_type)` |  |
| `eff.cache` | `(link)` | comment → table? |
| `eff:cache_ts` | `(link)` | comment → table? |
| `eff.get_node_type` | `(node_type)` |  |
| `eff.cache_as` | `(link, node_type)` |  |
| `eff.original_data` | `()` |  |
| `eff.get_namespace` | `(link)` | comment → table |
| `eff.find_sibling` | `(link, name)` | comment → table |
| `eff.validate` | `(ref_param, do_cache)` | comment → string? |
| `eff.execute_validators` | `(validators, ref_param, ...)` |  |
| `execute_internal` | `(ref_param)` | comment → CmdResult |
| `eff.execute` | `(ref_param)` | comment → CmdResult |

### `@common/base/eff_param`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\eff_param.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `ref_param:debuginfo` | `()` | comment → string |
| `ref_param:logfail` | `(result, info)` | comment → string? |
| `ref_param:new` | `(init_tree)` | comment → EffectParam |
| `ref_param:is_root` | `()` | comment → boolean |
| `ref_param:root` | `()` | comment → EffectParam |
| `ref_param:create_child` | `()` | comment → EffectParam |
| `ref_param:get_scene` | `()` | comment → string? |
| `ref_param:set_var_point` | `(key, point)` | comment |
| `ref_param:set_var_unit` | `(key, unit)` | comment |
| `ref_param:var_unit` | `(key)` | comment → Unit\|nil |
| `ref_param:var_point` | `(key)` | comment → Point\|nil |
| `ref_param:link_child` | `(child_param)` | comment |
| `ref_param:set` | `(in_ref_param)` | comment |
| `ref_param:init` | `(source,default_target)` | comment |
| `ref_param:set_source` | `(source)` | comment |
| `ref_param:calc_target` | `()` |  |
| `ref_param:set_target` | `(target)` | comment |
| `ref_param:set_launch` | `(launch)` | comment |
| `ref_param:get_level` | `()` |  |
| `ref_param:level_data` | `(data, fallbackValue, level)` | comment → boolean\|string\|number |
| `ref_param:set_cache` | `(link)` | comment |
| `ref_param:set_buff` | `(buff)` | comment |
| `ref_param:snap_shot_values` | `(table)` | comment → table |
| `ref_param:search` | `(link)` | comment → EffectParam |
| `ref_param:unit_sorts` | `(group, sorts)` | comment |
| `ref_param:missile_detach` | `()` | comment |
| `ref_param:is_missile_detached` | `()` | comment → boolean |
| `ref_param:set_channeler` | `(channeler)` | comment |
| `ref_param:get_channeler` | `()` | comment → Channeler |
| `ref_param:skill` | `()` | comment → Skill |
| `ref_param:cast` | `()` | comment → Cast |
| `ref_param:caster` | `()` | comment → Target |
| `ref_param:item` | `()` | comment → Target |
| `ref_param:user_data` | `()` | comment → table |
| `ref_param:item_random` | `(buff_link, prop_name,a, b, is_percentage, stack_index)` | comment → number |
| `ref_param:origin` | `()` | comment → Target |
| `ref_param:main_target` | `()` | comment → Target |
| `ref_param:set_caster` | `(caster)` | comment |
| `ref_param:set_origin` | `(origin_target)` | comment |
| `ref_param:set_creator` | `(creator_player)` | comment |
| `ref_param:creator_player` | `()` | comment |
| `ref_param:setup_caster` | `()` | comment |
| `ref_param:set_damage_modifiers` | `(unit,needreset)` | comment |
| `ref_param:get_site_target` | `(site, var)` | comment → Target |
| `ref_param:parse_loc` | `(loc_express, type)` | comment → Target? |
| `ref_param:parse_player` | `(player_express)` | comment → Player |
| `ref_param:parse_angle` | `(angle_express)` | comment → number |
| `ref_param:event` | `(name, f)` | comment |
| `ref_param:post_event` | `(event_subname)` | comment |
| `ref_param:post_new_target` | `(new_target)` | comment |
| `ref_param:init_child_on` | `(link, target)` | comment → EffectParam |
| `ref_param:execute` | `()` |  |
| `ref_param:execute_child_on` | `(link, target)` | comment → CmdResult |
| `get_exclude` | `(in_player, mask, scene)` | comment → integer[] |
| `ref_param:create_actor` | `(link, position, force_no_sync)` | comment → Actor? |
| `ref_param:next_child` | `()` |  |
| `ref_param:stop` | `()` |  |
| `ref_param:add_buff` | `(target, link, stack, params)` | comment → EffectParam |
| `ref_param:damage` | `(target, amount, type, params)` | comment → EffectParam |
| `Amount` | `()` |  |
| `ref_param:loop` | `(loop_data)` | comment |
| `tick` | `()` |  |
| `early_out` | `()` |  |
| `safe_tick` | `()` |  |
| `tick_start` | `()` |  |
| `ref_param:on_channeler_cleared` | `()` | comment |
| `ref_param:loop_clear_up` | `(complete)` | comment |
| `ref_param:get_node_in_module` | `(name)` | comment → any |
| `ref_shared:new` | `(root)` | comment → EffectParamShared |
| `ref_shared:close` | `()` |  |
| `ref_shared:is_closed` | `()` |  |
| `ref_shared:set_skill` | `(cast)` | comment |
| `ref_shared:set_level` | `(level)` | comment |
| `ref_shared:set_weapon` | `(weapon)` | comment |
| `ref_shared:set_item` | `(item)` | comment |

### `@common/base/effect`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/event`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\event.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.assign_event` | `(name, f)` |  |
| `base.forward_event_register` | `(name)` |  |
| `base.event_dispatch` | `(obj, name, ...)` |  |
| `is_ts_class_metatable` | `(c)` |  |
| `base.event_serialize` | `(t, depth, event_name)` |  |
| `base.event_deserialize` | `(t)` |  |
| `__client_event_to_server` | `(obj, name, ...)` |  |
| `base.event_notify` | `(obj, name, ...)` |  |
| `base.event_register` | `(obj, name, f)` |  |
| `base.game:event_dispatch` | `(name, ...)` |  |
| `base.game:event_notify` | `(name, ...)` |  |
| `base.game:event` | `(name, f)` |  |
| `base.game:broadcast` | `(name, f)` |  |
| `base.custom_event_notify` | `(event_name, event_param)` |  |
| `base.send_custom_event` | `(event)` | 触发V2用 |
| `TriggerEvent.prototype.____constructor` | `(self, obj, event_name, periodic, time)` |  |
| `base.单位进入视野.prototype.____constructor` | `(self, obj, evt_name, unit)` |  |
| `base.消息技能.prototype.____constructor` | `(self, obj, evt_name, msg)` |  |
| `base.场景加载完成.prototype.____constructor` | `(self, obj, evt_name, scene_name)` |  |
| `base.消息错误.prototype.____constructor` | `(self, obj, evt_name, msg, duration)` |  |
| `base.消息聊天.prototype.____constructor` | `(self, obj, evt_name, player, duration)` |  |
| `base.消息公告.prototype.____constructor` | `(self, obj, evt_name, msg, duration)` |  |
| `base.画面分辨率变化.prototype.____constructor` | `(self, obj, evt_name, width, height)` |  |
| `base.游戏阶段切换.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `base.游戏更新.prototype.____constructor` | `(self, obj, evt_name, delta)` |  |
| `base.玩家重连.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `base.游戏属性变化.prototype.____constructor` | `(self, obj, evt_name, property, value_s)` |  |
| `base.游戏开始.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `base.游戏结束.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `base.玩家断线.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `base.画面分辨率缩放变化.prototype.____constructor` | `(self, obj, evt_name, scale)` |  |
| `base.按键松开.prototype.____constructor` | `(self, obj, evt_name, key_keyboard)` |  |
| `base.对话选择.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link)` |  |
| `base.对话开始.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `base.鼠标点击物品栏中物品.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `base.鼠标长按物品栏中物品.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `base.鼠标长按物品栏中物品抬起.prototype.____constructor` | `(self, obj, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `base.对话跳过时.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `base.对话结束时.prototype.____constructor` | `(self, obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `base.按键按下.prototype.____constructor` | `(self, obj, evt_name, key_keyboard)` |  |
| `base.表现音效事件.prototype.____constructor` | `(self, obj, evt_name, msg, actor)` |  |
| `base.表现动画事件开始.prototype.____constructor` | `(self, obj, evt_name, actor, msg, anmi)` |  |
| `base.鼠标按下.prototype.____constructor` | `(self, obj, evt_name, key)` |  |
| `base.表现动画事件结束.prototype.____constructor` | `(self, obj, evt_name, anmi, msg, actor)` |  |
| `base.鼠标松开.prototype.____constructor` | `(self, obj, evt_name, key)` |  |
| `base.鼠标移动.prototype.____constructor` | `(self, obj, evt_name)` |  |
| `base.服务器请求切换场景.prototype.____constructor` | `(self, obj, old_scene, new_scene)` |  |
| `base.玩家属性变化.prototype.____constructor` | `(self, obj, evt_name, player, property, value_n, value_s)` |  |
| `base.玩家改变英雄.prototype.____constructor` | `(self, obj, evt_name, player, unit)` |  |
| `base.单位施法完成.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `base.单位施法出手.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `base.单位施法停止.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `base.单位失去状态.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `base.单位获得状态.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `base.单位状态层数变化.prototype.____constructor` | `(self, obj, evt_name, buff, stack, unit)` |  |
| `base.单位施法引导.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `base.单位属性变化.prototype.____constructor` | `(self, obj, evt_name, unit, property, value_n, value_s)` |  |
| `base.单位离开视野.prototype.____constructor` | `(self, obj, evt_name, unit)` |  |
| `base.单位施法开始.prototype.____constructor` | `(self, obj, evt_name, unit, skill_id, time_elapsed, time_total)` |  |
| `base.单位选中.prototype.____constructor` | `(self, obj, evt_name, player, unit)` |  |
| `base.单位取消选中.prototype.____constructor` | `(self, obj, evt_name, player, unit)` |  |
| `base.玩家改变队伍.prototype.____constructor` | `(self, obj, evt_name, player, team)` |  |
| `base.技能获得.prototype.____constructor` | `(self, obj, evt_name, unit, skill)` |  |
| `base.技能属性变化.prototype.____constructor` | `(self, obj, evt_name, skill, property, value_n)` |  |
| `base.技能充能激活.prototype.____constructor` | `(self, obj, evt_name, skill, time_remaining, time_total)` |  |
| `base.技能冷却激活.prototype.____constructor` | `(self, obj, evt_name, skill, time_remaining, time_total)` |  |
| `base.状态获得.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `base.状态层数变化.prototype.____constructor` | `(self, obj, evt_name, buff, stack, unit)` |  |
| `base.状态失去.prototype.____constructor` | `(self, obj, evt_name, unit, buff)` |  |
| `base.技能失去.prototype.____constructor` | `(self, obj, evt_name, unit, skill)` |  |
| `base.技能冷却完成.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `base.技能可用状态变化.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `base.技能等级变化.prototype.____constructor` | `(self, obj, evt_name, skill, level)` |  |
| `base.技能学习状态变化.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `base.技能层数变化.prototype.____constructor` | `(self, obj, evt_name, skill, stack)` |  |
| `base.技能槽位变化.prototype.____constructor` | `(self, obj, evt_name, skill)` |  |
| `base.玩家暂时离开.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `base.玩家回到游戏.prototype.____constructor` | `(self, obj, evt_name, player)` |  |
| `base.单位失去物品.prototype.____constructor` | `(self, obj, evt_name, player, item, drop_mode)` |  |
| `base.单位获得物品.prototype.____constructor` | `(self, obj, evt_name, player, item)` |  |
| `base.联合场景区域通知.prototype.____constructor` | `(self, obj, evt_name, from_scene, from_area, to_scene, to_area)` |  |
| `base.联合场景跨越区域.prototype.____constructor` | `(self, obj, evt_name, from_scene, from_area, to_scene, to_area)` |  |
| `base.联合场景进入区域.prototype.____constructor` | `(self, obj, evt_name, scene, area, target_scene)` |  |
| `base.联合场景离开区域.prototype.____constructor` | `(self, obj, evt_name, scene, area, target_scene)` |  |
| `base.建造预放置开始.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `base.建造预放置取消.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `base.建造预放置确认.prototype.____constructor` | `(self, obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `base.消息提示显示时.prototype.____constructor` | `(self, obj, evt_name, toast, text, source)` |  |
| `base.菜单栏按钮按下时.prototype.____constructor` | `(self, obj, evt_name, Key)` |  |
| `base.初始化好友列表.prototype.____constructor` | `(self, obj, evt_name, friend_data_list)` |  |
| `base.初始化好友申请列表.prototype.____constructor` | `(self, obj, evt_name, friend_apply_data_list)` |  |
| `base.申请列表状态变化.prototype.____constructor` | `(self, obj, evt_name, friend_apply_data)` |  |
| `base.join_middle_game` | `(middle_game_key)` |  |
| `base.send_add_friend` | `(user_id)` |  |
| `base.send_agree_add` | `(user_id)` |  |
| `base.send_refuse_add` | `(user_id)` |  |

### `@common/base/event_deque`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\event_deque.lua`）
- ⚠️ 本文件是**转发桩**：`return require '@base.base.event_deque'`（实现不在本包，见下方推测）


### `@common/base/exception`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\exception.lua`）
- ⚠️ 本文件是**转发桩**：`return require '@base.base.exception'`（实现不在本包，见下方推测）


### `@common/base/fish`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/force`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\force.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:insert` | `(player)` |  |
| `mt:remove` | `(player)` |  |
| `mt:has` | `(player)` |  |
| `mt:len` | `()` |  |
| `mt:random` | `()` |  |
| `mt:ipairs` | `()` |  |
| `mt:clear` | `()` |  |
| `base.force:__call` | `(list)` |  |
| `init` | `()` |  |

### `@common/base/force_movement`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/friend`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\friend.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.friend.send_add_friend` | `(user_id)` |  |
| `base.friend.send_agree_add` | `(user_id)` | 同意好友申请 |
| `base.friend.send_refuse_add` | `(user_id)` | 拒绝好友申请 |
| `base.proto.InGame_S2C_init_friend_list` | `(data)` | 好友列表 |
| `base.proto.InGame_S2C_init_friend_apply_list` | `(data)` | 好友申请列表 |
| `base.proto.InGame_S2C_notice_friend_state` | `(data)` | 申请列表状态变化 |
| `base.proto.InGame_S2C_friend_apply_fail` | `(data)` | 添加好友失败 |

### `@common/base/game`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\game.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.game:__tostring` | `()` |  |
| `init_scene_name_map` | `()` |  |
| `base.get_scene_name_by_hash` | `(hash)` | → string |
| `base.get_scene_hash_by_name` | `(name)` | → integer |
| `base.game:hotkey` | `()` | 方法 ------------------------ |
| `base.game:key_state` | `(key)` |  |
| `base.game:selected_unit` | `()` |  |
| `base.game:chat` | `(type, msg)` |  |
| `base.game:show_timer` | `()` |  |
| `base.game:set_game_scene` | `(...)` |  |
| `base.game:get_current_scene` | `()` |  |
| `base.game:lock_camera` | `()` |  |
| `base.game:unlock_camera` | `()` |  |
| `base.game:set_camera_attribute` | `(key, value, time)` |  |
| `base.game:input_mouse` | `()` |  |
| `base.game:loading_left` | `()` |  |
| `base.game:select_unit` | `(unit)` |  |
| `base.game:circle_selector` | `(pos, radius, tag, ignore_center_pos)` |  |
| `base.game:line_selector` | `(pos, length, width, face, tag)` |  |
| `base.game:sector_selector` | `(pos, radius, degree, face, tag)` |  |
| `base.game:get_winner` | `()` |  |
| `base.game:get_winner_team` | `()` |  |
| `base.game:send_broadcast` | `(...)` |  |
| `base.game:camera_focus` | `(unit)` |  |
| `base.game.get_default_unit` | `(node_mark)` | 客户端从服务器获取默认地编单位 |
| `base.game.object_store_value` | `(object, key, value)` | comment |
| `base.game.object_restore_value` | `(object, key)` |  |
| `base.event.on_spell_cast_result` | `(msg)` |  |
| `base.event.on_error_tip` | `(msg, time)` |  |
| `base.event.on_system_message` | `(msg, type, time)` |  |
| `base.event.on_notify_chat_message` | `(player_slot_id, type, msg, time)` |  |
| `base.event.on_unit_clicked` | `(id)` |  |
| `base.event.on_control_spell_assist` | `(control, spell_id, type, shape, range, width, plane_range, id)` |  |
| `base.event.on_move_spell_assist` | `()` | [[ |
| `base.event.on_spell_assist_update` | `(spell_id , time, id)` |  |
| `base.event.on_game_will_enter_foreground` | `()` |  |
| `base.event.on_game_enter_foreground` | `()` |  |
| `base.event.on_game_enter_background` | `()` |  |
| `base.event.on_click` | `(screen_pos, actorsID, button)` |  |
| `key_down` | `(key)` |  |
| `key_up` | `(key)` |  |
| `update_key_state` | `(key, count)` |  |
| `base.event.on_key_down` | `(unkey)` |  |
| `base.event.on_key_up` | `(unkey)` |  |
| `base.event.on_mouse_down` | `(button_type)` |  |
| `base.event.on_mouse_up` | `(button_type)` |  |
| `base.event.on_mouse_move` | `()` |  |
| `base.event.on_wheel_move` | `(delta_wheel)` |  |
| `base.event.on_joystick_button_down` | `(button_name)` |  |
| `base.event.on_joystick_button_up` | `(button_name)` |  |
| `base.event.on_joystick_axis_move` | `(axis_name, position)` |  |
| `base.event.on_joystick_hat_move` | `(state)` |  |
| `base.event.on_start_loading` | `(time)` |  |
| `base.event.on_enter_game` | `()` |  |
| `base.event.on_replay_stopped` | `()` |  |
| `base.event.on_game_result` | `(json)` |  |
| `base.event.on_load_scene` | `(scene_name)` |  |
| `base.event.on_load_scene_over` | `(scene_name)` |  |
| `base.event.on_combined_scene_area_notify` | `(...)` |  |
| `base.event.on_game_setting_changed` | `()` |  |
| `base.event.on_create_riseletter_failed` | `(riselettertype,templatename)` |  |
| `base.event.on_game_start` | `(...)` | function param: map_name, map_kind, session_id, background_loading |
| `base.event.on_game_loading` | `(content, percent)` |  |
| `base.event.on_game_started` | `(...)` | function param: map_name, map_kind, session_id, background_loading |
| `base.event.on_game_exit` | `(map_name, map_kind, session_id, ...)` | function param: map_name, map_kind, session_id, background_loading |
| `base.event.on_game_kick` | `(...)` |  |
| `base.event.on_game_reconnected` | `(...)` |  |
| `base.event.on_url_launch` | `(map_name)` |  |
| `base.event.on_file_changed` | `(file_path, file_name, change_list)` | 监测文件夹是否变化 |
| `base.event.on_broadcast` | `(args)` |  |
| `base.event.on_sync_custom_game_attribute` | `(key, value)` |  |
| `base.get_game_attribute` | `(key)` |  |
| `base.event.on_actor_event` | `(actor_id, msg, anim, start)` |  |
| `base.event.on_game_time_pause` | `()` |  |
| `base.event.on_game_time_resume` | `()` |  |
| `base.event.on_actor_destroy` | `(actor_id)` |  |
| `base.event.on_debug_cheat` | `(cheat_codes)` |  |
| `base.event.on_actor_finish_animation` | `(actor_id, anim, operation)` |  |
| `base.event.on_unit_finish_animation` | `(unit_id, anim, operation)` |  |
| `base.event.on_game_sync_unit_attribute_config` | `(attribute_config)` |  |
| `base.game:on_kick` | `(msg)` |  |
| `base.game.create_debug_draw_actor` | `()` |  |
| `base.game.debug_draw_point` | `(actor, point, color)` |  |
| `base.game.debug_draw_circle` | `(actor, point, euler_alpha, euler_beta, euler_gamma, radius, color, solid)` |  |
| `base.game.debug_draw_line` | `(actor, s_point, e_point, color)` |  |
| `base.game.debug_draw_sector` | `(actor, point, euler_alpha, euler_beta, euler_gamma, radius, angle, color, solid)` |  |
| `base.game.debug_draw_text` | `(actor, point, text, color, displayTop)` |  |
| `base.game.debug_draw_rectangle` | `(actor, v_point, w_point, h_point, color, solid)` |  |
| `base.game.clear_debug_draws` | `(actor)` |  |
| `base.get_current_fps` | `()` |  |
| `base.get_current_ping` | `()` |  |
| `base.set_use_right_click_move` | `(use)` |  |
| `base.get_use_right_click_move` | `()` |  |
| `base.raycast_unit_at_screen_xy` | `(x, y)` |  |
| `base.get_units_from_rect` | `(point, width, height, face)` | 获取矩形区域内的所有单位  返回单位数组 |
| `base.get_units_from_sector` | `(point,radius,arc,face)` | 获取扇形区域内的所有单位  返回单位数组 |
| `base.game.load_combined_map` | `(scene, direction)` | 显示拼接场景 |
| `base.game.purge_combined_map` | `()` | 释放拼接场景 |
| `base.game.load_combined_map_deco` | `(scene, direction)` | 创建拼接场景通行模型 |
| `base.game.purge_combined_map_deco` | `()` | 释放拼接场景通行模型 |
| `base.game.load_scene_cache_and_combined` | `(scene, direction)` |  |
| `AnimPointInfo.prototype.____constructor` | `(self, tbl)` |  |
| `base.game.get_model_anim_point_info` | `(model_path, anim_name)` | 给触发用的api，用ts类包了一层 |
| `base.get_obj_items` | `()` | → table |
| `base.get_all_skills_id` | `()` | → table |
| `base.get_all_buffs_id` | `()` | → table |
| `base.get_all_units_id` | `()` | → table |
| `base.game_shortcut` | `()` | 创建游戏快捷方式 |
| `base.shallow_copy` | `(tbl)` | → table |
| `base.set_cursor_shape` | `(path)` |  |
| `base.use_system_cursor` | `()` |  |
| `base.get_ground_z` | `(x, y, bool)` |  |
| `base.get_ground_z_from_point` | `(point, bool)` |  |
| `init_gameplay` | `()` |  |
| `base.get_platform` | `()` |  |
| `base.get_platform_is_app` | `()` |  |
| `base.start_game` | `(map_name, is_to_test)` |  |
| `base.game.set_dynamic_point_light` | `(val)` |  |

### `@common/base/gameplay`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/group`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\group.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:insert` | `(obj)` |  |
| `mt:remove` | `(obj)` |  |
| `mt:has` | `(obj)` |  |
| `mt:len` | `()` |  |
| `mt:random` | `()` |  |
| `mt:ipairs` | `()` |  |
| `mt:clear` | `()` |  |
| `base.group` | `(list)` |  |

### `@common/base/hashtable`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\hashtable.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `new_wk` | `()` |  |
| `mt:save` | `(k1, k2, tp, value)` | tp为编辑器中的值类型，在编译时生成 |
| `mt:load` | `(k1, k2, tp, def)` | tp为编辑器中的值类型，def为该类型的默认值，在编译时生成 |
| `mt:flush` | `()` |  |
| `mt:flush_parent` | `(k1)` |  |
| `mt:flush_child` | `(k1, k2)` |  |
| `base.hashtable` | `()` |  |

### `@common/base/heal`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/inventory`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `new('$$lib_promotion1_inventory.item_container.升变2装备栏.root', ...)`
  - `new('$$lib_promotion2_inventory.item_container.升变2装备栏.root', ...)`

### `@common/base/isolation`

- 归属：script 包（common 库）
- 研判：StateGame 沙箱阉割表（禁用 io/os/debug 等）；common 根有 isolation.lua，base 侧版本未随包分发。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/item`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\item.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__index` | `(key)` |  |
| `mt:__tostring` | `()` |  |
| `base.item` | `(id, silence)` |  |
| `mt:get_owner` | `()` |  |
| `mt:is_in_unit_cooldown` | `()` |  |
| `mt:try_drop` | `(callback)` |  |
| `mt:get_attr_need` | `()` |  |
| `mt:foreach_attr_need` | `(func)` |  |
| `mt:get_all_extra_mod` | `(is_equip)` |  |
| `mt:get_rand_mod` | `(buff_link, buff_idx, key, percentage)` |  |
| `mt:get_name` | `()` |  |
| `try_load_show_methods` | `()` |  |
| `mt:get_show_name` | `()` |  |
| `mt:get_icon` | `()` |  |
| `mt:get_tips` | `()` |  |
| `mt:get_current_cd` | `()` |  |
| `mt:get_cd_max` | `()` |  |
| `mt:get_stack` | `()` |  |

### `@common/base/json_decode`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/line`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\line.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:get` | `(i)` |  |
| `mt:get_length` | `()` |  |
| `base.line` | `(points)` |  |
| `base.get_scene_line` | `(scene, area_name, present)` | 获取地编线 |

### `@common/base/lni`

- 归属：script 包（common 库）
- 研判：`base.game.lni = require 'lni_loader'`（C++ 实现）的 Lua 侧封装；lni = 引擎配置数据格式。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/lni_writer`

- 归属：script 包（common 库）
- 研判：`base.game.lni = require 'lni_loader'`（C++ 实现）的 Lua 侧封装；lni = 引擎配置数据格式。
- 状态：✅ 有源码（`script\199\common\base\lni_writer.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `format_key` | `(name)` |  |
| `format_value` | `(value)` |  |
| `convert_table` | `(tbl)` |  |
| `convert_root` | `(root)` |  |

### `@common/base/load_done`

- 归属：script 包（common 库）
- 研判：资源/场景加载完成回调登记（命名语义推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/log`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\log.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `fmt` | `(f, ...)` |  |
| `log.debugf` | `(fmt, ...)` |  |
| `log.infof` | `(fmt, ...)` |  |
| `log.warnf` | `(fmt, ...)` |  |
| `log.errorf` | `(fmt, ...)` |  |
| `log.error` | `(...)` |  |
| `log.alertf` | `(fmt, ...)` |  |
| `log.fail` | `(info)` |  |
| `log.failf` | `(fmt, ...)` |  |
| `_G.printf` | `(fmt, ...)` |  |
| `log.traceback_debug_bp` | `(...)` |  |

### `@common/base/loot`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/loot_pool`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/lualib_bundle`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\lualib_bundle.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `__TS__ArrayIsArray` | `(value)` |  |
| `__TS__ArrayClone` | `(self)` |  |
| `__TS__ArrayConcat` | `(self, ...)` |  |
| `__TS__Symbol` | `(description)` |  |
| `__TS__ArrayEntries` | `(array)` |  |
| `next` | `(self)` |  |
| `__TS__ArrayEvery` | `(self, callbackfn, thisArg)` |  |
| `__TS__ArrayFilter` | `(self, callbackfn, thisArg)` |  |
| `__TS__ArrayForEach` | `(self, callbackFn, thisArg)` |  |
| `__TS__ArrayForEachEx` | `(self, callbackFn, thisArg)` |  |
| `__TS__ArrayRandom` | `(self)` |  |
| `__TS__ArrayRandoms` | `(self, number, duplicate)` |  |
| `__TS__ArrayFind` | `(self, predicate, thisArg)` |  |
| `__TS__ArrayFindIndex` | `(self, callbackFn, thisArg)` |  |
| `iteratorGeneratorStep` | `(self)` |  |
| `iteratorIteratorStep` | `(self)` |  |
| `iteratorStringStep` | `(self, index)` |  |
| `__TS__Iterator` | `(iterable)` |  |
| `iteratorGeneratorStep` | `(self)` |  |
| `iteratorIteratorStep` | `(self)` |  |
| `iteratorStringStep` | `(self, index)` |  |
| `iteratorLuaTable` | `(self, key)` |  |
| `__TS__IteratorMap` | `(iterable)` |  |
| `arrayLikeStep` | `(self, index)` |  |
| `arrayLikeIterator` | `(arr)` |  |
| `__TS__ArrayFrom` | `(arrayLike, mapFn, thisArg)` |  |
| `__TS__ArrayIncludes` | `(self, searchElement, fromIndex)` |  |
| `__TS__ArrayIndexOf` | `(self, searchElement, fromIndex)` |  |
| `__TS__ArrayJoin` | `(self, separator)` |  |
| `__TS__ArrayMap` | `(self, callbackfn, thisArg)` |  |
| `__TS__ArrayPush` | `(self, ...)` |  |
| `__TS__ArrayPushArray` | `(self, items)` |  |
| `__TS__CountVarargs` | `(...)` |  |
| `__TS__ArrayReduce` | `(self, callbackFn, ...)` |  |
| `__TS__ArrayReduceRight` | `(self, callbackFn, ...)` |  |
| `__TS__ArrayReverse` | `(self)` |  |
| `__TS__ArrayUnshift` | `(self, ...)` |  |
| `__TS__ArraySort` | `(self, compareFn)` |  |
| `__TS__ArraySlice` | `(self, first, last)` |  |
| `__TS__ArraySome` | `(self, callbackfn, thisArg)` |  |
| `__TS__ArraySplice` | `(self, ...)` |  |
| `__TS__ArrayToObject` | `(self)` |  |
| `__TS__ArrayFlat` | `(self, depth)` |  |
| `__TS__ArrayFlatMap` | `(self, callback, thisArg)` |  |
| `__TS__ArraySetLength` | `(self, length)` |  |
| `__TS__TypeReference` | `(typeName, typeArguments)` |  |
| `__TS__Keyword` | `(keyword)` |  |
| `__TS__UnionType` | `(types)` |  |
| `__TS__Type.__TS__TypeArgumentCheck` | `(currentType, targetType)` |  |
| `__TS__Type.__TS__TypeArgumentListCheck` | `(currentArguments, targetArguments)` |  |
| `__TS__InstanceOf` | `(obj, classTbl)` |  |
| `__TS__ForceAs` | `(obj, targetTypeReference)` |  |
| `TypeArgumentsFuncWrapper` | `(superTypeArguments, superTargetFunc)` |  |
| `__TS__SuperTypeArgumentsFuncWrapper` | `(classTable, currentTypeArguemnts, superTargetFunc)` |  |
| `__TS__New` | `(target, typeArguments, ...)` |  |
| `__TS__Class` | `(self)` |  |
| `__TS__FunctionBind` | `(fn, ...)` |  |
| `promiseDeferred` | `(self)` |  |
| `isPromiseLike` | `(self, thing)` |  |
| `__TS__Promise.prototype.____constructor` | `(self, executor)` |  |
| `____catch` | `(e)` |  |
| `__TS__Promise.resolve` | `(data)` |  |
| `__TS__Promise.reject` | `(reason)` |  |
| `__TS__Promise.prototype.catch` | `(self, onRejected)` |  |
| `__TS__Promise.prototype.finally` | `(self, onFinally)` |  |
| `__TS__Promise.prototype.resolve` | `(self, data)` |  |
| `__TS__Promise.prototype.reject` | `(self, reason)` |  |
| `__TS__Promise.prototype.createPromiseResolvingCallback` | `(self, f, resolve, reject)` |  |
| `____catch` | `(e)` |  |
| `__TS__Promise.prototype.handleCallbackData` | `(self, data, resolve, reject)` |  |
| `__TS__AsyncAwaiter` | `(generator)` |  |
| `adopt` | `(self, value)` |  |
| `fulfilled` | `(self, value)` |  |
| `step` | `(self, result)` |  |
| `__TS__Await` | `(thing)` |  |
| `__TS__ClassExtends` | `(target, base, superTypeArgumentsFunc)` |  |
| `__TS__CloneDescriptor` | `(____bindingPattern0)` |  |
| `__TS__ObjectAssign` | `(target, ...)` |  |
| `__TS__ObjectGetOwnPropertyDescriptor` | `(object, key)` |  |
| `descriptorIndex` | `(self, key)` |  |
| `descriptorNewIndex` | `(self, key, value)` |  |
| `__TS__SetDescriptor` | `(target, key, desc, isPrototype)` |  |
| `__TS__ClassIndex` | `(target, isPrototype)` |  |
| `__TS__Decorate` | `(decorators, target, key, desc)` |  |
| `__TS__DecorateParam` | `(paramIndex, decorator)` |  |
| `__TS__StringIncludes` | `(self, searchString, position)` |  |
| `getErrorStack` | `(self, constructor)` |  |
| `wrapErrorToString` | `(self, getDescription)` |  |
| `initErrorClass` | `(self, Type, name)` |  |
| `____class_0.prototype.____constructor` | `(self, message)` |  |
| `____class_0.prototype.__tostring` | `(self)` |  |
| `createErrorClass` | `(self, name)` |  |
| `____class_3.prototype.____constructor` | `(self, ...)` |  |
| `__TS__ObjectGetOwnPropertyDescriptors` | `(object)` |  |
| `__TS__Delete` | `(target, key)` |  |
| `__TS__StringAccess` | `(self, index)` |  |
| `__TS__DelegatedYield` | `(iterable)` |  |
| `generatorIterator` | `(self)` |  |
| `generatorNext` | `(self, ...)` |  |
| `__TS__Generator` | `(fn)` |  |
| `__TS__InstanceOfObject` | `(value)` |  |
| `__TS__LuaIteratorSpread` | `(self, state, firstKey)` |  |
| `Map.prototype.____constructor` | `(self, entries)` |  |
| `Map.prototype.clear` | `(self)` |  |
| `Map.prototype.delete` | `(self, key)` |  |
| `Map.prototype.forEach` | `(self, callback)` |  |
| `Map.prototype.forEachEx` | `(self, callback)` |  |
| `Map.prototype.get` | `(self, key)` |  |
| `Map.prototype.has` | `(self, key)` |  |
| `Map.prototype.set` | `(self, key, value)` |  |
| `Map.prototype.entries` | `(self)` |  |
| `next` | `(self)` |  |
| `Map.prototype.keys` | `(self)` |  |
| `next` | `(self)` |  |
| `Map.prototype.values` | `(self)` |  |
| `next` | `(self)` |  |
| `__TS__MapGet` | `(self, key)` |  |
| `__TS__MapSet` | `(self, key, value)` |  |
| `__TS__MapDelete` | `(self, key)` |  |
| `__TS__MapClear` | `(self)` |  |
| `__TS__MapForEach` | `(self, callback)` |  |
| `__TS__MapForEachEx` | `(self, callback)` |  |
| `__TS__MapSize` | `(self)` |  |
| `__TS__MathSign` | `(val)` |  |
| `__TS__Modulo50` | `(a, b)` |  |
| `__TS__Number` | `(value)` |  |
| `__TS__NumberIsFinite` | `(value)` |  |
| `__TS__NumberIsNaN` | `(value)` |  |
| `__TS__NumberToString` | `(self, radix)` |  |
| `__TS__ObjectDefineProperty` | `(target, key, desc)` |  |
| `__TS__ObjectEntries` | `(obj)` |  |
| `__TS__ObjectFromEntries` | `(entries)` |  |
| `__TS__ObjectKeys` | `(obj)` |  |
| `__TS__ObjectRest` | `(target, usedProperties)` |  |
| `__TS__ObjectValues` | `(obj)` |  |
| `__TS__ParseFloat` | `(numberString)` |  |
| `__TS__StringSubstr` | `(self, from, length)` |  |
| `__TS__StringSubstring` | `(self, start, ____end)` |  |
| `__TS__ParseInt` | `(numberString, base)` |  |
| `__TS__PromiseAll` | `(iterable)` |  |
| `__TS__PromiseAllSettled` | `(iterable)` |  |
| `__TS__PromiseAny` | `(iterable)` |  |
| `__TS__PromiseRace` | `(iterable)` |  |
| `Set.prototype.____constructor` | `(self, values)` |  |
| `Set.prototype.add` | `(self, value)` |  |
| `Set.prototype.clear` | `(self)` |  |
| `Set.prototype.delete` | `(self, value)` |  |
| `Set.prototype.forEach` | `(self, callback)` |  |
| `Set.prototype.forEachEx` | `(self, callback)` |  |
| `Set.prototype.randomValues` | `(self)` |  |
| `Set.prototype.random` | `(self)` |  |
| `Set.prototype.randoms` | `(self, number, duplicate)` |  |
| `Set.prototype.has` | `(self, value)` |  |
| `Set.prototype.entries` | `(self)` |  |
| `next` | `(self)` |  |
| `Set.prototype.keys` | `(self)` |  |
| `next` | `(self)` |  |
| `Set.prototype.values` | `(self)` |  |
| `next` | `(self)` |  |
| `__TS__SparseArrayNew` | `(...)` |  |
| `__TS__SparseArrayPush` | `(sparseArray, ...)` |  |
| `__TS__SparseArraySpread` | `(sparseArray)` |  |
| `WeakMap.prototype.____constructor` | `(self, entries)` |  |
| `WeakMap.prototype.delete` | `(self, key)` |  |
| `WeakMap.prototype.get` | `(self, key)` |  |
| `WeakMap.prototype.has` | `(self, key)` |  |
| `WeakMap.prototype.set` | `(self, key, value)` |  |
| `WeakSet.prototype.____constructor` | `(self, values)` |  |
| `WeakSet.prototype.add` | `(self, value)` |  |
| `WeakSet.prototype.delete` | `(self, value)` |  |
| `WeakSet.prototype.has` | `(self, value)` |  |
| `__TS__SourceMapTraceBack` | `(fileName, sourceMap)` |  |
| `debug.traceback` | `(thread, message, level)` |  |
| `replacer` | `(____, file, srcFile, line)` |  |
| `stringReplacer` | `(____, file, line)` |  |
| `__TS__Spread` | `(iterable)` |  |
| `__TS__StringCharAt` | `(self, pos)` |  |
| `__TS__StringCharCodeAt` | `(self, index)` |  |
| `__TS__StringEndsWith` | `(self, searchString, endPosition)` |  |
| `__TS__StringPadEnd` | `(self, maxLength, fillString)` |  |
| `__TS__StringPadStart` | `(self, maxLength, fillString)` |  |
| `__TS__StringReplace` | `(source, searchValue, replaceValue)` |  |
| `__TS__StringSplit` | `(source, separator, limit)` |  |
| `__TS__StringReplaceAll` | `(source, searchValue, replaceValue)` |  |
| `__TS__StringSlice` | `(self, start, ____end)` |  |
| `__TS__StringStartsWith` | `(self, searchString, position)` |  |
| `__TS__StringTrim` | `(self)` |  |
| `__TS__StringTrimEnd` | `(self)` |  |
| `__TS__StringTrimStart` | `(self)` |  |
| `__TS__SymbolRegistryFor` | `(key)` |  |
| `__TS__SymbolRegistryKeyFor` | `(sym)` |  |
| `__TS__TypeOf` | `(value)` |  |

### `@common/base/margin`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\margin.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.margin` | `(...)` | 逻辑全在服务端，客户端只需要空函数 |

### `@common/base/match_info`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/math`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\math.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.math.sin` | `(r)` |  |
| `base.math.cos` | `(r)` |  |
| `base.math.tan` | `(r)` |  |
| `base.math.asin` | `(v)` |  |
| `base.math.acos` | `(v)` |  |
| `base.math.atan` | `(v1, v2)` |  |
| `base.math.ceil` | `(v)` |  |
| `base.math.floor` | `(v)` |  |
| `base.math.float_eq` | `(a, b)` | 浮点数比较 |
| `base.math.float_ueq` | `(a, b)` |  |
| `base.math.float_lt` | `(a, b)` |  |
| `base.math.float_le` | `(a, b)` |  |
| `base.math.float_gt` | `(a, b)` |  |
| `base.math.float_ge` | `(a, b)` |  |
| `base.math.random_float` | `(a, b)` | 随机浮点数 |
| `is_int` | `(n)` | comment |
| `base.math.is_int` | `(n)` |  |
| `base.math.random_int` | `(a, b)` | 随机整数 |
| `base.math.float_modf` | `(n)` | 浮点数小数部分（编辑器用） |
| `base.math.included_angle` | `(r1, r2)` | 计算2个角度之间的夹角 |
| `base.math.lerp` | `(from, to, t)` | 插值运算 |
| `base.math.clamp` | `(value, left, right)` |  |
| `base.math.max` | `(...)` |  |
| `base.math.min` | `(...)` |  |
| `base.math.vector_add` | `(vector1, vector2)` |  |
| `base.math.vector_sub` | `(vector1, vector2)` |  |
| `base.math.vector_mul` | `(vector, mul)` |  |
| `base.math.dot_product` | `(vector1, vector2)` |  |
| `base.math.cross_product` | `(vector1, vector2)` |  |
| `base.math.sqrt` | `(x)` | 平方根 |
| `base.math.log` | `(...)` | 对数 |
| `base.math.pow` | `(x, y)` | 次幂 |
| `base.math.square` | `(x)` | 平方 |
| `base.math.exp` | `(x)` | 自然指数 |
| `base.math.abs` | `(x)` | 绝对值 |

### `@common/base/mover_line`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/mover_target`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/obj_check`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\obj_check.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `unit_check` | `(unit, disable_error)` |  |
| `item_check` | `(item, disable_error)` |  |
| `skill_check` | `(skill, disable_error)` |  |
| `player_check` | `(player, disable_error)` |  |
| `circle_check` | `(obj, disable_error)` |  |
| `rect_check` | `(obj, disable_error)` |  |
| `area_check` | `(obj, disable_error)` |  |
| `point_check` | `(point, disable_error)` |  |
| `line_check` | `(line, disable_error)` |  |
| `buff_check` | `(buff, disable_error)` |  |
| `trigger_check` | `(trigger, disable_error)` |  |
| `timer_check` | `(timer, disable_error)` |  |
| `any_unit_check` | `(unit, disable_error)` |  |
| `any_skill_check` | `(skill, disable_error)` |  |
| `any_player_check` | `(player, disable_error)` |  |
| `id_check` | `(obj_id, disable_error)` |  |
| `event_name_check` | `(event_name, disable_error)` |  |
| `time_check` | `(time, disable_error)` |  |
| `component_check` | `(cmpt, disable_error)` |  |
| `base.gui_check` | `(cmpt)` |  |
| `base.gui_get_part_as` | `(ts_type, cmpt, part_name)` |  |
| `base.gui_get_parts_ts` | `(ts_type, cmpt, part_name)` |  |
| `base.gui_get_array_child` | `(ts_type, cmpt)` |  |
| `base.gui_get_child_ui_by_name_as` | `(ts_type, cmpt, child_name)` |  |
| `base.gui_get_children` | `(ctrl)` |  |
| `base.gui_get_rect` | `(ctrl)` |  |
| `base.gui_get_parent` | `(ctrl)` |  |
| `base.fade_in_out` | `(fade_type, fade_time, is_wait, color, opacity, curve_type, z_index)` |  |
| `base.fade_in` | `(fade_time, is_wait, color, opacity, curve_type, z_index)` |  |
| `init` | `(self)` |  |
| `fade_in` | `(self)` |  |
| `fade_out` | `(self)` |  |
| `base.fade_out` | `(fade_time, is_wait, color, opacity, curve_type, z_index)` |  |

### `@common/base/old_junk`

- 归属：script 包（common 库）
- 研判：历史遗留兼容层（命名语义推测）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/player`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\player.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__tostring` | `()` |  |
| `init_one_player` | `(id, ptype, team)` |  |
| `init_players` | `()` |  |
| `set_team_id` | `(self, team)` |  |
| `mt:get_team_id` | `()` |  |
| `mt:get_team` | `()` |  |
| `set_hero` | `(self, unit)` |  |
| `mt:get_hero` | `()` |  |
| `mt:is_ally` | `(other)` |  |
| `mt:is_enemy` | `(other)` |  |
| `mt:is_neutral_to` | `(other)` | comment → boolean |
| `mt:is_neutral` | `()` | comment → boolean |
| `mt:is_online` | `()` | comment → boolean |
| `mt:set_hero_upper_body_facing` | `(facing, sync_to_server)` |  |
| `mt:cancel_hero_upper_body_facing` | `(time)` |  |
| `set_hero_name` | `(self, name)` |  |
| `mt:get_hero_name` | `()` |  |
| `mt:get_hero_reborn` | `()` |  |
| `mt:user_name` | `()` |  |
| `mt:user_title` | `()` |  |
| `mt:user_icon` | `()` |  |
| `mt:user_border` | `()` |  |
| `mt:get` | `(key)` |  |
| `mt:get_slot_id` | `()` |  |
| `mt:controller` | `()` |  |
| `mt:game_state` | `()` |  |
| `mt:loading_progress` | `()` |  |
| `modify_table` | `(ori_tbl, modify_tbl)` |  |
| `delete_table` | `(ori_tbl, modify_tbl)` |  |
| `set_by_sync` | `(self, key, value)` |  |
| `set` | `(self, key, value)` |  |
| `mt:event_notify` | `(name, ...)` |  |
| `mt:event` | `(name, f)` |  |
| `base.local_player` | `()` |  |
| `base.player` | `(id)` |  |
| `base.each_player` | `(type)` |  |
| `next` | `()` |  |
| `sort_pairs` | `(t)` |  |
| `base.event.on_player_table_attributes_changed` | `(key_values)` |  |
| `base.event.on_player_attributes_changed` | `(key_values)` |  |
| `base.event.on_loading_progress_notify` | `(slot_id, progress)` |  |
| `mt:get_nick_name` | `()` |  |
| `mt:get_num` | `(name, ...)` |  |

### `@common/base/point`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\point.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `create_point` | `(x, y, z, scene)` | 创建一个点 → Point |
| `table_to_point` | `(table)` | comment → Point? |
| `mt:__tostring` | `()` |  |
| `mt:get_xy` | `()` | 获取坐标 |
| `mt:get_x` | `()` |  |
| `mt:get_y` | `()` |  |
| `mt:get_height` | `()` |  |
| `mt:set_scene` | `(scene)` | comment |
| `mt:copy` | `()` | 复制点 |
| `mt:copy_to_scene_point` | `(scene)` |  |
| `mt:get_point` | `()` | 返回点 |
| `mt:get_scene_point` | `()` |  |
| `mt:get_scene` | `()` |  |
| `mt:get_position` | `()` | 返回位置 |
| `mt:__add` | `(data)` |  |
| `mt:__sub` | `(data)` |  |
| `mt:__mul` | `(dest)` | 求距离(point * point) |
| `mt:__div` | `(dest)` |  |
| `mt:__unm` | `()` |  |
| `mt:add` | `(data)` |  |
| `mt:polar_to_ex` | `(angle, distance)` |  |
| `mt:polar_to` | `(data)` | 按照极坐标系移动(point:polar_to{angle, distance} ) |
| `mt:angle` | `(dest)` | 求方向(向量self和向量dest的夹角) |
| `mt:distance` | `(dest)` |  |
| `mt:to_coordinate` | `(point, facing)` | 将self映射到坐标系(point, facing)后, self在该坐标系里的位置 |
| `mt:set_height` | `(value)` |  |
| `mt:is_block` | `()` |  |
| `mt.has_restriction` | `(_,_)` |  |
| `mt.has_label` | `(_,_)` | comment |
| `mt.get_attackable_radius` | `(_)` | comment |
| `mt:get_unit` | `()` |  |
| `mt:get_team_id` | `()` |  |
| `base.get_scene_point` | `(scene, area_name, present)` |  |

### `@common/base/position`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\position.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__tostring` | `()` |  |
| `mt:get_xy` | `()` |  |
| `mt:get_x` | `()` |  |
| `mt:get_y` | `()` |  |
| `mt:get_ui_x` | `()` |  |
| `mt:get_ui_y` | `()` |  |
| `mt:get_point` | `()` |  |
| `base.mouse_screen_pos` | `()` |  |
| `base.position` | `(x, y)` |  |
| `base.screen_pos` | `(x, y)` | 用下面这个不容易误解 |

### `@common/base/promise`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\promise.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get` | `(self, timeout, callback)` |  |
| `co_result` | `(self, timeout)` |  |
| `co_error` | `(self, timeout)` |  |
| `co_get` | `(self, timeout)` |  |
| `set` | `(self, value, err)` |  |
| `try_set` | `(self, value, err)` |  |
| `set_result` | `(self, v)` |  |
| `try_set_result` | `(self, v)` |  |
| `set_error` | `(self, err)` |  |
| `try_set_error` | `(self, err)` |  |
| `ready` | `(self)` |  |
| `promise:__call` | `()` | → promise |
| `get` | `(self, timeout, callback)` |  |
| `co_get` | `(self, timeout)` |  |
| `_start` | `(self, promise_list, timeout)` |  |
| `ready` | `(self)` |  |
| `multi_promise:__call` | `(promise_list, join_type, timeout)` | → multi_promise |

### `@common/base/quest`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\quest.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.print_table` | `(t)` |  |
| `quest_condition:new` | `(tbl)` |  |
| `quest_condition:update_remaining_time` | `(remaining_time)` |  |
| `quest_condition:get_remaining_time` | `()` |  |
| `quest_condition:update` | `(tbl)` |  |
| `quest_condition:remove` | `()` |  |
| `quest_condition:submit` | `()` |  |
| `quest_condition:get_description` | `()` |  |
| `quest:new` | `(tbl)` |  |
| `quest:update` | `(tbl)` |  |
| `quest:remove` | `()` |  |
| `quest.update_quests` | `(unit, tbl, change_table)` |  |
| `quest:__tostring` | `()` | if base.test then |
| `quest_condition:__tostring` | `()` |  |

### `@common/base/rect`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\rect.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:get_scene` | `()` |  |
| `mt:get_point` | `()` |  |
| `mt:get_scene_point` | `()` |  |
| `mt:get_start_point` | `(pos,width)` |  |
| `mt:get_start_scene_point` | `(pos,width)` |  |
| `mt:get_width` | `()` |  |
| `mt:get_height` | `()` |  |
| `mt:random_point` | `()` |  |
| `mt:scene_random_point` | `()` |  |
| `mt:init_region` | `()` |  |
| `region:on_enter` | `(unit)` |  |
| `region:on_leave` | `(unit)` |  |
| `mt:remove_region` | `()` |  |
| `base.rect` | `(...)` |  |

### `@common/base/response`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\response.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.response:new` | `(link)` | comment → Response? |
| `base.response:set_cache` | `(link)` | comment |
| `base.response:execute` | `(in_param, ...)` | comment |
| `base.response:response` | `(...)` |  |
| `response_compare` | `(a, b)` | comment → boolean |
| `base.response:add` | `(unit, ref_param)` | comment |
| `base.response:remove` | `()` |  |
| `base.response:enabled` | `()` |  |
| `base.response:disabled` | `()` |  |
| `base.response.ResponseDamage:validate` | `(in_param, damage)` |  |
| `base.response.ResponseDamage:exectue` | `(in_param, damage)` |  |
| `base.response.ResponseMissileImpact:exectue` | `(in_param)` |  |
| `base.response.ResponseEffectImpact:exectue` | `(in_param)` |  |
| `base.response.ResponseSpell:exectue` | `(in_param, event, skill)` |  |
| `has_category` | `(cache, category)` | comment |
| `filter_categories` | `(cache, category_filters)` | comment → boolean |
| `base.response.ResponseBuff:exectue` | `(in_param, data)` |  |
| `base.response.ResponseUnit:exectue` | `(in_param, event)` |  |
| `base.response:start_cooldown` | `()` |  |

### `@common/base/room`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `find_room({ game_name = data.game_name, room_mode = data.room_mode,...)`
  - `sync_room_info({ room_code = data.room_code, room_cur_number = 0 })`

### `@common/base/rpc`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\rpc.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `rpc_call` | `(k, ...)` |  |
| `__index` | `(t, k)` |  |
| `__newindex` | `(t, k, v)` |  |
| `make_args` | `(owner, ...)` |  |
| `rpc_accept` | `(owner, k, ...)` |  |
| `rpc.callback` | `(id, ...)` |  |
| `proto.__simple_rpc__` | `(call)` |  |

### `@common/base/scene_object`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/scene_point`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\scene_point.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `create_scene_point` | `(x, y, z, scene, error_mark)` | comment → ScenePoint |
| `create_scene_point_by_hash` | `(x, y, z, scene_hash, error_mark)` | comment → ScenePoint |
| `mt:__tostring` | `()` |  |
| `mt:get_xy` | `()` | 获取坐标 |
| `mt:get_x` | `()` |  |
| `mt:get_y` | `()` |  |
| `mt:get_z` | `()` | 获取z |
| `mt:get_height` | `()` |  |
| `mt:get_scene` | `()` |  |
| `mt:get_scene_name` | `()` |  |
| `mt:get_scene_point` | `()` |  |
| `mt:copy` | `()` | 复制点 |
| `mt:copy_to_scene_point` | `(scene)` |  |
| `mt:get_point` | `()` | 返回点 |
| `mt:to_vector` | `(height)` | 转换为矢量 |
| `mt:__add` | `(data)` |  |
| `mt:__sub` | `(data)` | 按照极坐标系移动(point:polar_to({angle, distance})) |
| `mt:__mul` | `(dest)` | 求距离(point * point) |
| `mt:__div` | `(dest)` |  |
| `mt:__unm` | `()` |  |
| `mt:add` | `(data)` |  |
| `mt:polar_to` | `(data)` | 按照极坐标系移动(point:polar_to{angle, distance} ) |
| `mt:polar_to_ex` | `(angle, distance)` |  |
| `mt:angle` | `(dest)` | 求方向(向量self和向量dest的夹角) |
| `mt:distance` | `(dest)` |  |
| `mt:to_coordinate` | `(point, facing)` | 将self映射到坐标系(point, facing)后, self在该坐标系里的位置 |
| `mt:get_unit` | `()` |  |
| `mt:get_owner` | `()` |  |
| `mt:get_facing` | `()` |  |
| `mt:get_team_id` | `()` |  |
| `mt:angle_to` | `(dest)` | comment → number?, boolean? |
| `mt:get_snapshot` | `()` |  |
| `mt:create_effect` | `(model)` | TODO: 需要特别指定一个中立玩家： |
| `mt:is_visible_to` | `(dest)` | comment → boolean?, boolean? |
| `mt.has_restriction` | `(_,_)` |  |
| `mt.has_label` | `(_,_)` | comment |
| `mt.get_attackable_radius` | `(_)` | comment |
| `mt:get_collision_flags` | `(bol)` |  |

### `@common/base/selector`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/shop`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/skill`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\skill.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get_skill_name_by_hash` | `(hash)` |  |
| `active_cd` | `(self, cd, total)` |  |
| `finish_cd` | `(self)` |  |
| `active_charge_cd` | `(self, cd, total)` |  |
| `finish_charge_cd` | `(self)` |  |
| `is_removed` | `(self)` |  |
| `api:client_remove` | `()` |  |
| `remove` | `(self)` |  |
| `can_request` | `(self)` |  |
| `ac_skill` | `(id, hash, owner, is_silent)` |  |
| `set` | `(self, key, value)` |  |
| `set_user_attribute` | `(self, key, value)` |  |
| `update_attribute` | `(self, attr, events)` |  |
| `update_attribute_without_event` | `(self, attr)` |  |
| `base.event.on_spell_attributes_changed` | `(key_values)` |  |
| `base.event.on_remove_spell` | `(removed_spells)` |  |
| `base.event.on_spell_cd_changed` | `(id, cd, total, type)` |  |
| `base.event.on_spell_cd_finished` | `(id, type)` |  |
| `base.event.on_spell_cast_approach_ex` | `(unit_id, hash)` |  |
| `base.event.on_spell_cast_start_ex` | `(unit_id, hash, time, total)` |  |
| `base.event.on_spell_cast_notify_ex` | `(unit_id, hash, time, total)` |  |
| `base.event.on_spell_cast_shot_ex` | `(unit_id, hash, time, total)` |  |
| `base.event.on_spell_cast_end_ex` | `(unit_id, hash, time, total)` |  |
| `base.event.on_spell_cast_stop_ex` | `(unit_id, hash, time, total)` |  |
| `base.event.on_spell_cast_break_ex` | `(unit_id, hash)` |  |
| `base.event.on_spell_cast_failed_ex` | `(unit_id, hash)` |  |
| `base.proto.cancel_ignore_joy_stick` | `(msg)` |  |
| `base.proto.skill_group_set_unit` | `(msg)` |  |
| `mt:__tostring` | `()` |  |
| `mt:__index` | `(key)` |  |
| `api:level_data` | `(data, fallbackValue, level)` | comment → number |
| `api:get_name` | `()` |  |
| `api:get_owner` | `()` |  |
| `api:get_tip` | `()` |  |
| `api:get_stack` | `()` |  |
| `api:get_level` | `()` |  |
| `api:get_slot_id` | `()` |  |
| `api:is_enable` | `()` |  |
| `api:is_charge_skill` | `()` |  |
| `api:get_type` | `()` |  |
| `api:can_upgrade` | `()` | deprecated，新的用can_learn |
| `api:can_learn` | `()` |  |
| `api:event_notify` | `(name, ...)` |  |
| `api:event` | `(name, f)` |  |
| `api:get_cd` | `()` |  |
| `api:get_charge_cd` | `()` |  |
| `api:pause` | `()` |  |
| `api:resume` | `()` |  |
| `api:update_paused` | `()` |  |
| `api:cast` | `(smart)` | deprecated |
| `api:client_channel_finish` | `()` |  |
| `get_target_indicator_cache` | `(link)` | comment |
| `api:show_range` | `(follow, assistName)` |  |
| `api:hide_range` | `()` |  |
| `api:move` | `(slot)` |  |
| `api:upgrade` | `()` |  |
| `api:has_category` | `(category)` |  |
| `api:hotkey` | `(smart)` |  |
| `api:create_actor` | `(link)` |  |
| `api:create_actors` | `(event)` | comment |
| `api:destroy_actors` | `(event)` | comment |
| `api:is_attack` | `()` |  |
| `api:is_attack_modifier` | `()` |  |
| `api:get_user_attribute` | `(key)` | comment |
| `api:is_toggled_on` | `()` |  |
| `api:get_phase` | `()` |  |
| `api:get_current_show_cd` | `()` |  |
| `api:get_max_show_cd` | `()` |  |
| `api:get_currrent_charge_show_cd` | `()` |  |
| `api:get_max_charge_show_cd` | `()` |  |
| `try_load_show_methods` | `()` |  |
| `api:get_show_name` | `()` |  |
| `api:get_icon` | `()` |  |
| `api:get_tips` | `()` |  |
| `api:get_current_cd` | `()` |  |
| `api:get_cd_max` | `()` |  |
| `api:get_current_charge_cd` | `()` |  |
| `api:get_charge_cd_max` | `()` |  |
| `api:get_cooldown_key` | `()` |  |
| `base.skill_info` | `()` |  |
| `base.proto.sync_skill` | `(msg)` |  |

### `@common/base/snapshot`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\snapshot.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:new` | `()` |  |
| `mt:get_snapshot` | `()` | comment → Snapshot |
| `mt:get_point` | `()` | comment → Point |
| `mt.get_unit` | `(_)` | comment → Unit? |
| `mt:get_name` | `()` | comment → string |
| `mt:get_owner` | `()` | comment → Player |
| `mt:get_facing` | `()` | comment → integer |
| `mt:is_ally` | `(dest)` | comment → boolean |
| `mt:is_visible_to` | `(dest)` | comment → boolean |
| `mt:get_team_id` | `()` | comment → integer |
| `mt:has_restriction` | `(restriction)` | comment |
| `mt:has_label` | `(label)` | comment |
| `mt:get_attackable_radius` | `()` | comment |

### `@common/base/state_machine`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\state_machine.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `CustomStateMachine:ctor` | `(name, priority, layer)` |  |
| `CustomStateMachine:add_state` | `(name, id)` |  |
| `base.state_machine` | `(name, priority, layer)` |  |
| `State:ctor` | `(name, id)` |  |
| `base.state_machine_state` | `(name, id)` |  |

### `@common/base/table`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\table.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `SDBMHash` | `(str)` |  |
| `__index` | `(self, name)` |  |
| `base.skill_table` | `(name, level, key)` |  |
| `base.unit_table` | `(name, key)` |  |
| `base.buff_table` | `(name, key)` |  |
| `base.attack_table` | `(name, key)` |  |
| `base.item_table` | `(name, key)` |  |
| `base.spell_table` | `(name, key)` |  |

### `@common/base/table_attr`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/target_filter`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\target_filter.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:new` | `(filter_string)` | comment → TargetFilters |
| `mt:from_string` | `(filter_string)` |  |
| `mt:from_data_field` | `(filter_string)` |  |
| `mt.make_cmd_result` | `(filter, is_required)` |  |
| `mt:validate` | `(caster,target)` | comment → string? ErrorText |
| `mt.filter_player` | `(caster, target, filter)` | comment → boolean |
| `mt.filter_state` | `(target,filter)` | comment → boolean |
| `mt.filter_label` | `(target,label)` | comment → boolean |
| `is_custom_restruction` | `(att)` |  |
| `mt.has_filter` | `(caster,target,filter)` |  |

### `@common/base/tds_score`

- 归属：script 包（common 库）
- 研判：排行榜/积分服务的 base 侧封装；参照 server_lua_plus 的 base_lua_plus/tds_score.lua 包装器。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/team`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\team.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:get_id` | `()` |  |
| `mt:each_player` | `()` |  |
| `next` | `()` |  |
| `init` | `()` |  |
| `base.team` | `(id)` |  |

### `@common/base/thirdordermatrix`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\thirdordermatrix.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `create_tom` | `(TOMArray)` | tom = ThirdOrderMatrix |
| `mt:tom_addition` | `(MartixB)` | 矩阵加法 |
| `mt:tom_subtraction` | `(MartixB)` | 矩阵减法 |
| `mt:tom_multiplication_with_tom` | `(MartixB)` | 与矩阵相乘 |
| `mt:tom_multiplication_with_vector` | `(Vector)` | 与向量相乘 |
| `mt:tom_determinant` | `()` | 矩阵的行列式 determinant |

### `@common/base/timer`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\timer.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `update_next` | `()` |  |
| `alloc_queue` | `()` |  |
| `m_timeout` | `(self, timeout, check_frame)` |  |
| `m_timeout_forSetTime` | `(self, timeout)` | 不重置时间偏移量，使得用户通过api读取的时间保持正确 |
| `m_wakeup` | `(self)` |  |
| `get_remaining` | `(self)` |  |
| `on_tick` | `()` |  |
| `base.clock` | `()` |  |
| `base.event.on_tick` | `(delta)` |  |
| `mt:__tostring` | `()` |  |
| `mt:remove` | `()` |  |
| `mt:pause` | `()` |  |
| `mt:resume` | `()` |  |
| `mt:restart` | `()` |  |
| `mt:get_current` | `()` |  |
| `mt:get_current_time` | `()` |  |
| `mt:set_current_time` | `(NewTime)` |  |
| `mt:get_remaining_time` | `()` |  |
| `mt:get_remaining_time_new` | `()` |  |
| `mt:set_remaining_time` | `(NewTime)` |  |
| `base.wait` | `(timeout, on_timer, timer)` |  |
| `base.loop` | `(timeout, on_timer)` | → Timer |
| `base.wait` | `(timeout, on_timer)` |  |
| `base.loop` | `(timeout, on_timer)` | → Timer |
| `base.loop_lazy` | `(timeout, on_timer)` |  |
| `base.next` | `(cb)` |  |
| `base.timer` | `(timeout, count, on_timer)` |  |
| `utimer_initialize` | `(u)` |  |
| `base.uwait` | `(u, timeout, on_timer)` |  |
| `base.uloop` | `(u, timeout, on_timer)` |  |
| `base.utimer` | `(u, timeout, count, on_timer)` |  |
| `base.set_timer_warning` | `(w)` |  |
| `on_update` | `(delta)` |  |
| `base.event.on_update` | `(delta)` |  |
| `base.event.on_post_update` | `(delta)` |  |
| `base.event.on_prerender_update` | `(delta)` |  |
| `base.event.on_server_clock` | `(clock)` |  |
| `base.timer_info` | `()` |  |

### `@common/base/trigger`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\trigger.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__tostring` | `()` | if base.test then |
| `mt:disable` | `()` | 禁用触发器 |
| `mt:enable` | `()` |  |
| `mt:is_enable` | `()` |  |
| `mt:__call` | `(...)` | 运行触发器 |
| `mt:remove` | `()` | 摧毁触发器(移除全部事件) |
| `base.trigger_size` | `()` |  |
| `base.each_trigger` | `()` |  |
| `base.trigger` | `(event, callback)` | 创建触发器 |
| `base.trig:new` | `(action, combine_args, scene, sync)` | 通过函数创建一个新的触发器 → Trigger |
| `mt:add_event_common` | `(event)` |  |
| `mt:remove_event_common` | `(event)` |  |
| `mt:replicate` | `(include_event)` | 复制触发器 |
| `base.trigger_new_from_function` | `(func)` | 从函数创建回调 |
| `mt:add_event` | `(obj, name, custom_event, time, periodic)` | comment |
| `mt:_add_scene_event` | `(obj, name, custom_event, time, periodic)` |  |
| `mt:_add_event` | `(obj, name, custom_event, time, periodic)` |  |
| `mt:_remove_event` | `(obj, name)` |  |
| `mt:add_event_game_time` | `(time, periodic)` | base.game:event('单位-属性变化', function(trigger, unit, key, value) |
| `mt:add_event_game_time_internal` | `(time, periodic)` | comment |
| `mt:set_action` | `(action)` | comment |
| `args.event` | `(obj, evt_name)` | comment → EventArgs |
| `args.event_unit` | `(obj, evt_name, unit)` | comment → UnitEventArgs |
| `args.event_unit_property_change` | `(obj, evt_name, unit, property, value)` | comment → UnitPropertyChangeEventArgs |
| `args.event_skill` | `(obj, evt_name, skill)` | comment → UnitSkillEventArgs |
| `args.event_skill_property_change` | `(obj, evt_name, skill, property, value)` | comment → UnitSkillPropertyChangeEventArgs |
| `args.event_skill_level_change` | `(obj, evt_name, skill, level)` | comment → UnitSkillLevelChangeEventArgs |
| `args.event_skill_stack_change` | `(obj, evt_name, skill, stack)` | comment → UnitSkillStackChangeEventArgs |
| `args.event_skill_cooldown` | `(obj, evt_name, skill, time_remaining_ms, time_total_ms)` | comment → UnitSkillCooldownEventArgs |
| `args.event_unit_die` | `(obj, evt_name, unit, killer, type)` | comment → UnitDieEventArgs |
| `args.event_unit_damage_dealt` | `(obj, evt_name, damage)` | 伤害事件 |
| `args.event_unit_damage_taken` | `(obj, evt_name, damage)` | 伤害事件 |
| `args.event_unit_buff` | `(obj, evt_name, unit, buff)` |  |
| `args.event_buff` | `(obj, evt_name, buff)` |  |
| `args.event_buff_stack_change` | `(obj, evt_name, buff, stack, unit)` | comment → UnitSkillStackChangeEventArgs |
| `args.event_unit_purchase_item` | `(obj, evt_name, unit, item_name)` |  |
| `args.event_unit_inventory` | `(obj, evt_name, unit, slot)` |  |
| `args.event_unit_inventory_target` | `(obj, evt_name, unit, slot, target)` |  |
| `args.event_unit_item` | `(obj, evt_name, unit, item, drop_mode)` |  |
| `args.event_unit_cmd_request` | `(obj, evt_name, unit, command, target, key_modifier)` |  |
| `args.event_unit_moved` | `(obj, evt_name, unit, pos_old, pos_new)` |  |
| `args.event_unit_laned` | `(obj, evt_name, unit, vector_z)` |  |
| `args.event_unit_skill` | `(obj, evt_name, unit, skill)` |  |
| `args.event_unit_skill_stage` | `(obj, evt_name, unit, skill_id, time_elapsed_ms, time_total_ms)` | comment → UnitSkillCastEventArgs |
| `args.event_unit_skill_result` | `(obj, evt_name, unit, skill, result_code)` |  |
| `args.event_unit_xp` | `(obj, evt_name, xp_data)` |  |
| `args.event_unit_mover` | `(obj, evt_name, unit, mover)` |  |
| `args.event_unit_scene` | `(obj, evt_name, unit, scene_name)` |  |
| `args.event_area` | `(obj, evt_name, area, unit)` |  |
| `args.event_player` | `(obj, evt_name, player)` |  |
| `args.event_player_unit` | `(obj, evt_name, player, unit)` |  |
| `args.event_player_team` | `(obj, evt_name, player, team)` |  |
| `args.event_player_property_change` | `(obj, evt_name, player, property, value)` | comment → PlayerPropertyChangeEventArgs |
| `args.event_player_connect` | `(obj, evt_name, player, is_reconnect)` |  |
| `args.event_player_chat` | `(obj, evt_name, player, msg)` |  |
| `args.event_player_pick_hero` | `(obj, evt_name, player, hero_name)` |  |
| `args.event_player_scene` | `(obj, evt_name, player, scene_name)` |  |
| `args.event_player_config` | `(obj, evt_name, player, config)` |  |
| `args.event_player_ping` | `(obj, evt_name, player, ping)` |  |
| `args.event_player_key_down` | `(obj, evt_name, player, key)` |  |
| `args.event_player_key_up` | `(obj, evt_name, player, key)` |  |
| `args.event_player_mouse_down` | `(obj, evt_name, player, key)` |  |
| `args.event_player_mouse_up` | `(obj, evt_name, player, key)` |  |
| `args.event_player_wheel_move` | `(obj, evt_name, player, delta_wheel)` |  |
| `args.event_update` | `(obj, evt_name, delta)` |  |
| `args.event_click` | `(obj, evt_name, screen_pos, actors_ID, button)` |  |
| `args.event_enter_foreground` | `(obj, evt_name, module_key)` |  |
| `args.event_property_change` | `(obj, evt_name, property, value)` |  |
| `args.event_message` | `(obj, evt_name, msg)` |  |
| `args.event_message_timed` | `(obj, evt_name, msg, duration)` |  |
| `args.event_message_chat` | `(obj, evt_name, player_slot_id, type, msg, time)` |  |
| `args.event_resolution` | `(obj, evt_name, width, height)` |  |
| `args.event_scale` | `(obj, evt_name, scale)` |  |
| `args.event_key` | `(obj, evt_name, key)` |  |
| `args.event_key_down` | `(obj, evt_name, key)` |  |
| `args.event_key_up` | `(obj, evt_name, key)` |  |
| `args.event_mouse_down` | `(obj, evt_name, key)` |  |
| `args.event_mouse_up` | `(obj, evt_name, key)` |  |
| `args.event_actor` | `(obj, evt_name, id)` |  |
| `args.event_actor_anim_message` | `(obj, evt_name, id, msg, anim)` |  |
| `args.event_actor_sound_message` | `(obj, evt_name, id, msg)` |  |
| `args.event_scene` | `(obj, evt_name, scene_name)` |  |
| `args.event_game_scene` | `(obj, evt_name, game, scene_name)` |  |
| `args.event_eff_param` | `(obj, evt_name, ref_param)` |  |
| `args.event_eff_param_impact_unit` | `(obj, evt_name, ref_param, impacted_unit)` |  |
| `args.event_custom_event` | `(obj, evt_name, custom_args)` |  |
| `args.game_string_attribute_change` | `(obj, evt_name, game, key, value)` |  |
| `args.player_number_attribute_change` | `(obj, evt_name, player, key, value, value_change)` |  |
| `args.player_string_attribute_change` | `(obj, evt_name, player, key, value)` |  |
| `args.unit_number_attribute_change` | `(obj, evt_name, unit, key, value, value_change)` |  |
| `args.unit_string_attribute_change` | `(obj, evt_name, unit, key, value)` |  |
| `args.event_conversation` | `(obj, evt_name, speaker, listener, ref_param, conversation_link)` |  |
| `args.event_conversation_choose` | `(obj, evt_name, speaker, listener, ref_param, conversation_link, conversation_choice_item_link)` |  |
| `args.event_inventory_item_tooltip` | `(obj, evt_name, item, item_tooltip_panel, slot_panel, inventory_panel)` |  |
| `args.event_server_change_scene` | `(obj, evt_name, old_scene, new_scene)` |  |
| `args.event_scene_combind_area_notify` | `(obj, evt_name, from_scene, from_area, to_scene, to_area)` |  |
| `args.event_scene_combind_area_notifyB` | `(obj, evt_name, scene, area, target_scene)` |  |
| `args.event_spellbuild_preview` | `(obj, evt_name, owner, skill, spellbuild_unit_actor)` |  |
| `args.event_toast_show` | `(obj, evt_name, toast, text, source)` |  |
| `args.event_menu_button` | `(obj, evt_name, Key)` |  |
| `args.event_friend_list_init` | `(obj, evt_name, friend_data_list)` |  |
| `args.event_friend_apply_list_init` | `(obj, evt_name, friend_apply_data_list)` |  |
| `args.event_friend_apply_list_state_change` | `(obj, evt_name, friend_apply_data)` |  |
| `evt:new` | `(obj, name)` |  |
| `evt:remove` | `()` |  |

### `@common/base/trigger_editor_v2`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\trigger_editor_v2\init.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.ArrayIterator` | `(array)` |  |
| `__TS__Class2` | `(name)` |  |
| `c.prototype.____constructor` | `(self)` |  |
| `base.force_as` | `(classTbl, obj)` | comment → any |
| `base.instance_of` | `(classTbl, obj)` |  |

### `@common/base/trigger_editor_v2/array`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\trigger_editor_v2\array.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `Array.prototype.____constructor` | `(self, T, ...)` |  |

### `@common/base/try`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\try.lua`）
- ⚠️ 本文件是**转发桩**：`return require '@base.base.try'`（实现不在本包，见下方推测）
- lua_plus 扁平封装（**有源码**，见 lua-plus.md 对应模块）：

  - `base.try_drop_item(item, callback)`
  - `base.try_drop_item(item:item, callback:function<boolean>)`


### `@common/base/turn`

- 归属：script 包（common 库）
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/unit`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\unit.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `mt:__tostring` | `()` |  |
| `mt:get_team_id` | `()` |  |
| `mt:is_visible` | `()` |  |
| `mt:get_name` | `()` |  |
| `mt:get_string` | `(prop)` | comment |
| `mt:get_scene` | `()` |  |
| `mt:on_response` | `()` |  |
| `mt:get_scene_name` | `()` |  |
| `mt:get_owner` | `()` |  |
| `mt:get_data` | `()` |  |
| `mt:set` | `(key, value)` |  |
| `mt:get` | `(key)` |  |
| `mt:is_alive` | `()` |  |
| `mt:get_level` | `()` |  |
| `mt:get_asset` | `()` |  |
| `mt:get_model_path` | `()` |  |
| `mt:get_skill_points` | `()` |  |
| `mt:get_snapshot` | `()` |  |
| `mt:each_skill` | `(type)` |  |
| `mt:each_skill_all` | `()` |  |
| `mt:has_label` | `(label)` | comment |
| `mt:set_point` | `(scene_point)` |  |
| `mt:destroy` | `()` |  |
| `mt:set_position` | `(x, y, z)` |  |
| `mt:set_rotation` | `(x, y ,z)` |  |
| `mt:set_scale_xyz` | `(x, y, z)` |  |
| `mt:set_scale` | `(x, y, z)` |  |
| `mt:has_restriction` | `(restriction)` |  |
| `mt:find_skill` | `(name, tp)` |  |
| `mt:find_skill_by_slot` | `(slot)` |  |
| `mt:get_attack` | `()` |  |
| `mt:find_buff` | `(name)` |  |
| `mt:each_buff` | `(target)` |  |
| `mt:each_buff_all` | `(target)` |  |
| `mt:get_class` | `()` |  |
| `mt:get_tag` | `()` |  |
| `mt:get_xy` | `()` |  |
| `mt:get_point` | `()` | 优先返回宿主坐标 |
| `mt:get_global_point` | `()` |  |
| `mt:get_global_scene_point` | `()` |  |
| `mt:get_socket_point` | `(socket)` |  |
| `mt:get_socket_position` | `(socket)` |  |
| `mt:get_socket_rotation` | `(socket)` |  |
| `mt:play_anim_ex` | `(anim_name, anim_param)` | comment → AnimHandle? |
| `mt:get_anims` | `()` | comment → table<ICustomAnimParams> |
| `mt:play_anim_bracket` | `()` |  |
| `mt:attach_to` | `(target, socket)` |  |
| `mt:detach` | `()` |  |
| `mt:get_height` | `()` |  |
| `mt:get_facing` | `()` |  |
| `mt:get_highlight` | `()` |  |
| `mt:set_highlight` | `(on, ...)` |  |
| `mt:get_outstroke` | `()` |  |
| `mt:set_outstroke` | `(enable, color, thickness)` |  |
| `mt:set_shadow` | `(enable)` |  |
| `mt:get_xray_enable` | `()` |  |
| `mt:set_xray_enable` | `(enable)` |  |
| `mt:get_unit_random_model_index` | `()` |  |
| `mt:set_fow` | `(enable, radius)` |  |
| `mt:set_sight` | `(typ, param)` |  |
| `mt:set_sight_skill_fan` | `(x, y, z, radius, angle)` |  |
| `mt:set_eye_height` | `(h)` |  |
| `mt:setup_occluding_camera_group` | `(...)` |  |
| `mt:set_tint_enabled` | `(flag)` |  |
| `mt:set_tint_color` | `(idx, clr)` | idx: 1/2/3 clr:{r, g, b, a} |
| `mt:set_tick_disabled` | `(on_or_off)` |  |
| `mt:is_item` | `()` |  |
| `mt:event_notify` | `(name, ...)` |  |
| `mt:event` | `(name, f)` |  |
| `mt:cast` | `(skill, target, data)` |  |
| `mt:move_to_direction` | `(x, y)` |  |
| `mt:stop_move_to_direction` | `(x, y)` |  |
| `mt:anim_play` | `(anim_name, params)` |  |
| `sort_bracket` | `(bracket1,bracket2)` |  |
| `add_bracket_to_table` | `(self, bracket_anim)` | 添加bracket动画 |
| `mt:anim_play_bracket` | `(anim_birth, anim_stand, anim_death, params)` | 手动构建BSD动画，然后play动画 |
| `mt:set_time_scale_global` | `(scale)` |  |
| `mt:anim_set_paused_all` | `(paused)` |  |
| `mt:unit_anim_operation` | `(value)` |  |
| `mt:learn_skill` | `(skill)` |  |
| `mt:set_bone_chain_facing` | `(CHAIN_ID, angle, time)` |  |
| `mt:set_bone_chain_facing_v1` | `(angle, time)` |  |
| `mt:reset_bone_chain_facing` | `(CHAIN_ID, time)` |  |
| `mt:reset_bone_chain_facing_v1` | `(time)` |  |
| `get_destory_time` | `()` |  |
| `base.unit` | `(id)` | comment → boolean? new |
| `alloc_unit_queue` | `()` | 定期清楚单位的逻辑 |
| `free_queue` | `(q)` |  |
| `add_destory_unit` | `(unit)` |  |
| `remove_destory_unit` | `(unit)` |  |
| `base.remove_unit` | `(id)` |  |
| `base.get_default_unit` | `(node_mark)` |  |
| `base.get_default_item` | `(node_mark)` |  |
| `set` | `(self, key, value)` |  |
| `modify_table` | `(ori_tbl, modify_tbl)` |  |
| `delete_table` | `(ori_tbl, modify_tbl)` |  |
| `set_by_sync` | `(self, key, value)` |  |
| `base.add_attribute_key` | `(name, id)` |  |
| `init_attribute` | `()` |  |
| `on_attr_anim_func` | `(self, key, value)` |  |
| `update_attribute` | `(self, attr)` |  |
| `update_attribute_by_array` | `(attr)` |  |
| `update_table_attribute` | `(self, attr)` |  |
| `update_table_attribute_by_array` | `(attr)` |  |
| `update_attribute_without_event` | `(self, attr)` |  |
| `mt:attach_model` | `(path, hand_point, hold_point)` |  |
| `mt:detach_model` | `(path)` |  |
| `mt:change_model` | `(path)` |  |
| `mt:create_actor` | `(link, ignore_unit_list)` | comment |
| `mt:create_actors` | `(msg)` |  |
| `mt:get_node_mark` | `()` |  |
| `on_unit_created` | `(id, attr)` | 原GameUnit创建处理 |
| `on_light_unit_created` | `(unit_id, attr_map, is_actor)` | 轻量单位创建处理 （包含GameUnit和同步Actor） |
| `base.event.on_controlled_sync_unit_created` | `(id, scene_name, unit_type_id, unit_slot)` |  |
| `mt:destroy_actors` | `(msg)` |  |
| `base.event.on_unit_attributes_changed` | `(data, new)` | comment |
| `base.event.on_unit_table_attributes_changed` | `(data, new)` |  |
| `base.event.on_unit_model_changed` | `(id, path)` |  |
| `on_unit_destory` | `(id)` | 原GameUnit销毁处理 |
| `on_light_unit_destroy` | `(unit_id)` | 轻量单位销毁处理 （包含GameUnit和同步Actor） |
| `base.event.on_unit_attach_changed` | `(unit_id, attach_id)` | 轻量单位附着事件处理 （c++不处理具体附着逻辑，由脚本统一处理，其中包含GameUnit和同步Actor） |
| `base.event.on_unit_hovered` | `(id)` |  |
| `mt:set_blood_bar_visible` | `(visible)` |  |
| `mt:set_status_bar_visibility` | `(visible)` | 设置血条是否显示（暴露到触发的api） |
| `sync_unit_actor` | `(unit, key, value)` |  |
| `mt:set_blood_bar_template` | `(template_name)` |  |
| `mt:set_blood_bar_widget` | `(key, value)` |  |
| `base.event.on_unit_blood_bar_created` | `(unit_id)` |  |
| `mt:set_minimap_icon_visible` | `(visible)` |  |
| `base.unit_info` | `()` |  |
| `mt:create_riseletter` | `(position ,text, type, color, fontsize)` |  |
| `mt:create_riseletter_by_link` | `(position ,text, link, color, fontsize)` |  |
| `mt:create_riseletter_by_templatename` | `(position ,text, template_name, color, fontsize)` |  |
| `mt:remove_riseletter` | `(riseletter)` |  |
| `mt:set_riseletter_position` | `(riseletter, position)` |  |
| `mt:create_riseletter_without_color_size` | `(location,text,text_type)` |  |
| `mt:create_riseletter_with_color_size` | `(location,text,text_type,color,size)` |  |
| `mt:try_pick_item` | `(item, callback)` |  |
| `mt:get_or_create_state_machine` | `(name, priority, layer)` |  |
| `mt:remove_state_machine` | `(sm_name)` |  |
| `base.event.on_unit_state_machine_changed` | `(unit_id, state_machines)` |  |
| `base.event.on_unit_state_machine_transit` | `(unit_id, sm_name, event_id)` |  |
| `mt:is_valid` | `()` |  |
| `mt:execute_on` | `(target,link, cache_override)` | comment → CmdResult |
| `mt:execute_on_point` | `(target,link, cache_override)` | comment → CmdResult |
| `mt:get_unit` | `()` |  |
| `mt:set_rotation` | `(x, y, z)` |  |
| `mt:get_all_items` | `()` | comment → Item[] |
| `mt:get_display_name` | `()` |  |
| `mt:set_display_name` | `(name)` |  |
| `mt:get_inventory_items` | `(inv_idx)` |  |
| `base.get_units_from_screen_xy` | `(xy, is_accurate)` |  |
| `try_load_show_methods` | `()` |  |
| `mt:get_show_name` | `()` |  |
| `mt:get_icon` | `()` |  |
| `mt:get_tips` | `()` |  |
| `mt:get_current_cd` | `()` |  |
| `mt:get_cd_max` | `()` |  |
| `mt:set_disappear_destory_time` | `(time)` | 设置单位离开视野的销毁时间 |
| `mt:get_cooldown` | `(cooldown_key)` |  |
| `mt:get_cooldown_max` | `(cooldown_key)` |  |
| `mt:insert_into_cooldown_map` | `(cooldown_key, skill)` |  |
| `mt:is_cooldown_map_empty` | `()` |  |
| `mt:remove_from_cooldown_map` | `(cooldown_key, skill)` |  |
| `mt:register_bone_chain` | `(CHAIN_ID, bone_chain_data)` | 参考 https://xindong.atlassian.net/wiki/spaces/Editor/pages/1060713486 |
| `mt:register_model_bone_chain` | `(bol)` | 开放给触发用户用的，应用模型配的数据 |
| `mt:test_build_box` | `(min, max, test_type)` | test_type: 0-粗糙（允许些微的高低不平） 1-严格 2-浮空 |
| `base.event.on_unit_cool_down` | `(unit_id, cooldown_key)` |  |

### `@common/base/utility`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\utility.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.hash` | `(str)` |  |
| `base.get_appendable_enum` | `(key)` |  |
| `base.get_appendable_keys` | `(key)` |  |
| `io.load` | `(filename, mode)` |  |
| `base.split` | `(str, p)` |  |
| `base.string_format` | `(context, params)` | comment → string |
| `base.utf8_sub` | `(s, i, j)` |  |
| `base.to_type` | `(value, expect_type)` |  |
| `base.get_unit_name` | `(type_id)` |  |
| `base.image_path` | `(path)` |  |
| `base.load_string` | `(str, skill)` |  |
| `base.get_x` | `(obj)` |  |
| `base.get_y` | `(obj)` |  |
| `base.remove` | `(obj)` |  |
| `base.default` | `(v, default)` |  |
| `gc_mt:__shl` | `(obj)` |  |
| `gc_mt:flush` | `()` |  |
| `base.gc` | `()` |  |
| `base.calc_http_server_address` | `(server_name, default_port)` |  |

### `@common/base/validator`

- 归属：script 包（common 库）
- 研判：触发器数据校验器（与 @p_55a3/trigger_validator 同源机制；项目侧有同名 TS 源码 trigger_validator.ts）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

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

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\vector.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `create_vector` | `(X, Y, Z)` |  |
| `mt:vector_addition` | `(VectorB)` | 向量加法 |
| `mt:vector_subtraction` | `(VectorB)` | 向量减法 |
| `mt:vector_multiplication` | `(VectorB)` | 向量乘法(点乘) |
| `mt:get_vector_length` | `()` | 获取向量长度 |
| `mt:get_unit_vector` | `()` | 获取单位向量 |

### `@common/base/voice`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\base\voice.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get_auth` | `(appid,room, user_id)` |  |
| `rpc.join_voice_room` | `(room, team, range, cb)` |  |
| `rpc.voice_black_list` | `(p, mute)` |  |

### `@common/preload/lni_loader`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\preload\lni_loader.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@common/preload/reload`

- 归属：script 包（common 库）
- 状态：✅ 有源码（`script\199\common\preload\reload.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）
