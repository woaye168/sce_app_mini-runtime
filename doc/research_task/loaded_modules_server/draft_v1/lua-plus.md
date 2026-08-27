# server_lua_plus 包（lua_plus 触发器 API 层）+ tds_score

模块数：47。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）。

源码覆盖：42/47；其余标注 ⚠️ 无源码并附调用点反查/语义推测。

---

### `@lua_plus/base`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lua_plus/base/base_lua_plus/actor`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\actor.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_last_created_actor` | `()` |  |
| `base.create_actor_at` | `(name, point, use_terrain)` |  |
| `base.actor_set_grount_height` | `(actor, height)` |  |
| `base.actor_set_position` | `(actor, point)` |  |
| `base.actor_set_facting` | `(actor, angle)` |  |
| `base.actor_attach_to_unit` | `(actor, host, socket)` |  |
| `base.create_actor_on_buff` | `(name, host)` |  |
| `base.buff_get_actor` | `(host, name)` |  |
| `base.unit_get_actor` | `(host, name)` |  |
| `base.eff_param_get_actor` | `(host, name)` |  |
| `base.actor_attach_to_actor` | `(actor, host, socket)` |  |
| `base.actor_destroy` | `(actor, flag)` |  |
| `base.actor_set_asset_model` | `(actor, asset)` |  |
| `base.actor_set_asset_sound` | `(actor, asset)` |  |
| `base.actor_set_owner` | `(actor, owner)` |  |
| `base.actor_set_shadow` | `(actor, enable)` |  |
| `base.actor_set_scale` | `(actor, scale)` |  |
| `base.actor_play` | `(actor)` |  |
| `base.actor_stop` | `(actor)` |  |
| `base.actor_pause` | `(actor)` |  |
| `base.actor_resume` | `(actor)` |  |
| `base.actor_set_volume` | `(actor, volume)` |  |
| `base.actor_set_grid_size` | `(actor, size_x, size_y)` |  |
| `base.actor_set_grid_range` | `(actor, start_x, start_y, range_x, range_y)` |  |
| `base.actor_set_grid_state` | `(actor, id_x, id_y, state)` |  |
| `base.actor_anim_play` | `(actor, anim, time, time_type, start_offset, blend_time, priority)` |  |
| `base.actor_anim_set_paused_all` | `(actor, paused)` |  |
| `base.actor_set_time_scale_global` | `(actor, time_scale)` |  |
| `base.actor_anim_play_bracket` | `(actor, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority, sync)` |  |

### `@lua_plus/base/base_lua_plus/advertise`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\advertise.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.adplay_status` | `(player)` | 开启广告返回状态 |
| `base.adplay_recall` | `(player, cb)` | 开启广告 |
| `base.advertiseReturnParams` | `()` | 触发器最后播放广告是否完成观看 |
| `base.advertiseReturnErrCode` | `()` | 触发器最后播放广告的错误码 |
| `base.advertiseReturnErrMsg` | `()` | 触发器最后播放广告出现的相应信息 |

### `@lua_plus/base/base_lua_plus/ai`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\ai.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_ai` | `(name)` |  |
| `base.unit_remove_ai` | `(unit)` |  |
| `base.unit_enable_ai` | `(unit)` |  |
| `base.unit_disable_ai` | `(unit)` |  |
| `base.unit_execute_ai` | `(unit)` |  |
| `base.unit_ai_attack_move_to` | `(unit, line, cycle)` |  |
| `base.unit_ai_move_to` | `(unit, line, cycle)` |  |
| `line_with_offset` | `(line, offset_x, offset_y)` | comment → Point[] |
| `base.unit_group_ai_attack_move_to` | `(unit_group, line, cycle)` |  |
| `base.unit_group_ai_move_to` | `(unit_group, line, cycle)` |  |

### `@lua_plus/base/base_lua_plus/ai_attack`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\ai_attack.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.ai_attack_add_team_threat` | `(ai_attack, team, threat)` |  |
| `base.ai_attack_add_unit_threat` | `(ai_attack, unit, threat)` |  |
| `base.ai_attack_add_type_threat` | `(ai_attack, unit_tag, threat)` |  |
| `base.ai_attack_remove` | `(ai_attack)` |  |

### `@lua_plus/base/base_lua_plus/area`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\area.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_scene_circle` | `(scene, area_name, present)` |  |
| `base.get_scene_rect` | `(scene, area_name, present)` |  |
| `base.get_scene_area` | `(scene, area_type, area_name, present)` |  |
| `base.circle_get_point` | `(circle)` |  |
| `base.circle_get_range` | `(circle)` |  |
| `base.circle_random_point` | `(circle)` |  |
| `base.rect_get_point` | `(rect)` |  |
| `base.rect_get_width` | `(rect)` |  |
| `base.rect_get_height` | `(rect)` |  |
| `base.rect_random_point` | `(rect)` |  |
| `base.get_random_point` | `(area)` |  |
| `base.get_area_point` | `(area)` |  |
| `base.get_scene_scale_area` | `(scene_name)` |  |
| `base.get_circle_area_unit` | `(circle)` |  |
| `base.get_circle_area_unit_v2` | `(circle)` |  |
| `base.get_rect_area_unit` | `(rect)` |  |
| `base.get_rect_area_unit_v2` | `(rect)` |  |
| `base.get_area_unit` | `(area)` |  |
| `base.get_area_unit_v2` | `(area)` |  |
| `base.get_area_unit_group` | `(area, 过滤条件)` |  |
| `base.get_area_type_unit` | `(area, unit_id_name)` |  |
| `base.get_area_type_unit_group` | `(area, unit_id_name, 过滤条件)` |  |
| `base.get_area_player_type_unit` | `(area, player, unit_id_name)` |  |
| `base.get_area_player_type_unit_group` | `(area, player, unit_id_name, 过滤条件)` |  |
| `base.is_point_in_circle` | `(point, circle)` |  |
| `base.is_point_in_rect` | `(point, rect)` |  |
| `base.is_point_in_area` | `(point, area)` |  |
| `base.is_unit_in_area` | `(unit, area)` |  |

### `@lua_plus/base/base_lua_plus/attack`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\attack.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.attack_add_damage` | `(attack:attack, source:unit, target:unit, damage:number)` |  |
| `base.attack_get_cd` | `(attack:attack)` |  |
| `base.attack_get_name` | `(attack:attack)` |  |
| `base.attack_is_common_attack` | `(attack:attack)` |  |
| `base.attack_is_skill` | `(attack:attack)` |  |
| `base.attack_set_cd` | `(attack:attack, cd:number)` |  |
| `base.attack_stop` | `(attack:attack)` |  |

### `@lua_plus/base/base_lua_plus/buff`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\buff.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_last_created_buff` | `()` |  |
| `base.unit_add_buff` | `(unit, buff_id_name, stack)` |  |
| `base.buff_set_stack` | `(buff, count)` |  |
| `base.buff_get_pulse` | `(buff)` |  |
| `base.buff_get_remaining` | `(buff)` |  |
| `base.buff_get_stack_all` | `(unit, link)` |  |
| `base.buff_get_stack` | `(buff)` |  |
| `base.buff_remove` | `(buff)` |  |
| `base.buff_set_pulse` | `(buff, pulse)` |  |
| `base.buff_set_remaining` | `(buff, remaining)` |  |
| `base.unit_each_buff` | `(unit, id)` |  |
| `base.unit_find_buff` | `(unit, id)` |  |
| `base.unit_has_buff` | `(unit, id)` |  |
| `base.buff_get_id` | `(buff)` |  |
| `base.buff_get_level` | `(buff)` |  |
| `base.buff_set_level` | `(buff, level)` |  |
| `base.buff_get_tracked_units` | `(buff)` |  |
| `base.buff_get_tracked_units_v2` | `(buff)` |  |
| `base.get_all_buffs_id` | `()` |  |
| `base.unit_all_buffs` | `(unit)` |  |
| `base.buff_get_stack_param` | `(buff)` |  |

### `@lua_plus/base/base_lua_plus/camera`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\camera.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.player_lock_camera` | `(player)` |  |
| `base.player_set_camera` | `(player, camera_id_name, time)` |  |
| `base.player_shake_camera` | `(player, type, frequency, amplitude, time)` |  |
| `base.player_unlock_camera` | `(player)` |  |
| `base.player_camera_focus` | `(player, unit)` |  |

### `@lua_plus/base/base_lua_plus/capturer`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\capturer.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.capturer_remove` | `(capturer)` |  |
| `base.unit_capturer` | `(unit, radius)` |  |

### `@lua_plus/base/base_lua_plus/cheat`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\cheat.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.gm_god_is_enable` | `(player)` |  |
| `base.gm_cooldown_is_enable` | `(player)` |  |
| `base.gm_energy_is_enable` | `(player)` |  |

### `@lua_plus/base/base_lua_plus/common`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\common.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `table.pop_front` | `(t)` | function table.pop_back(t:table) unknown |
| `table.getn` | `(t, index)` |  |

### `@lua_plus/base/base_lua_plus/damage`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\damage.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.damage_get_damage` | `(damage)` |  |
| `base.damage_get_current_damage` | `(damage)` |  |
| `base.damage_get_type` | `(damage)` |  |
| `base.damage_set_current_damage` | `(damage, amount)` |  |
| `base.do_trigger_damage` | `(source, target, amount, damage_type)` |  |

### `@lua_plus/base/base_lua_plus/eff_param`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\eff_param.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.eff_param_origin_point` | `(eff_param)` |  |
| `base.eff_param_get_link` | `(eff_param)` |  |
| `base.unit_execute_effect_on_unit` | `(unit, target, link)` |  |
| `base.unit_execute_effect_on_point` | `(unit, target, link)` |  |
| `base.eff_param_missle_detach` | `(eff_param)` |  |
| `base.eff_param_missle_get` | `(eff_param)` |  |
| `base.eff_param_missle_range` | `(eff_param)` |  |
| `base.eff_param_set_damage_modifiers` | `(eff_param, unit)` |  |
| `base.eff_param_source_item` | `(eff_param)` |  |
| `base.eff_param_responsing_param` | `(eff_param)` |  |
| `base.eff_param_responsing_skill` | `(eff_param)` |  |
| `base.eff_param_responsing_damage` | `(eff_param)` |  |
| `base.eff_param_caster` | `(eff_param)` |  |
| `base.eff_param_main_target_point` | `(eff_param)` |  |
| `base.eff_param_main_target_unit` | `(eff_param)` |  |
| `base.eff_param_target_point` | `(eff_param)` |  |
| `base.eff_param_target_unit` | `(eff_param)` |  |
| `base.eff_param_has_target` | `(eff_param)` |  |
| `base.eff_param_get_root` | `(eff_param)` |  |
| `base.eff_param_get_parent` | `(eff_param)` |  |
| `base.eff_param_get_by_name` | `(eff_param, effect_id_name)` |  |
| `base.eff_param_get_level` | `(eff_param)` |  |
| `base.eff_param_get_skill` | `(eff_param)` |  |
| `base.eff_param_get_cast` | `(eff_param)` |  |
| `base.eff_param_get_var_unit` | `(eff_param, key)` |  |
| `base.eff_param_get_var_point` | `(eff_param, key)` |  |
| `base.eff_param_set_var_unit` | `(eff_param, key, value)` |  |
| `base.eff_param_set_var_point` | `(eff_param, key, value)` |  |
| `base.eff_param_get_userdata` | `(eff_param, key)` |  |
| `base.eff_param_get_cache` | `(eff_param)` |  |
| `base.eff_param_get_node_in_module` | `(eff_param, name)` |  |
| `base.validator_unit_filter` | `(eff_param, unit, filters)` |  |
| `base.validator_unit_filter_new` | `(eff_param, unit, filters)` |  |
| `base.validator_condition` | `(condition)` |  |
| `base.validator_and` | `(code1, code2)` |  |
| `base.validator_or` | `(code1, code2)` |  |
| `base.validator_not` | `(code1)` |  |
| `base.validator_unit_has_buff` | `(eff_param, unit, buff_id_name)` |  |

### `@lua_plus/base/base_lua_plus/game`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\game.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.game_exit` | `(player, show_confirm)` |  |
| `base.player_jump_scene` | `(player, scene, keep_hero)` |  |
| `base.game_ui_message` | `(message_name, data)` |  |
| `base.player_win_game` | `(player)` |  |
| `base.player_fail_game` | `(player)` |  |
| `base.object_store_value` | `(object, key, value)` |  |
| `base.object_restore_value` | `(object, key)` |  |
| `base.pause_game` | `()` |  |
| `base.pause_game_time` | `(sec)` |  |
| `base.unpause_game` | `()` |  |
| `base.switch_fov_mode` | `(number, scene)` |  |
| `base.get_gamemode_key` | `()` | 游戏模式 |

### `@lua_plus/base/base_lua_plus/gamechat`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\gamechat.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.gamechat_send_message` | `(text, user)` |  |
| `base.ui.proto.gamechatclient_send_message` | `(_, msg)` |  |

### `@lua_plus/base/base_lua_plus/global_variable`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\global_variable.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.table_new` | `()` | pending_game_units = {} |

### `@lua_plus/base/base_lua_plus/hook`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\hook.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `string.find` | `(...)` |  |
| `string.find_end` | `()` |  |

### `@lua_plus/base/base_lua_plus/item`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\item.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.create_item_on_point` | `(id, target)` |  |
| `base.create_item_on_unit` | `(id, target)` |  |
| `base.unit_add_item` | `(unit, item)` |  |
| `base.unit_has_item` | `(unit, id)` |  |
| `base.unit_all_items` | `(unit)` |  |
| `base.item_add_extra_mod` | `(item, buff_id_name, IsEquip)` |  |
| `base.remove_extra_mod` | `(item, buff_id_name, IsEquip)` |  |
| `base.item_generate_rand_mod` | `(item)` |  |
| `base.get_last_created_item` | `()` |  |
| `base.item_rnd_value` | `(item, buff_id_name, prop_name)` |  |
| `base.item_set_stack` | `(item, stack)` |  |
| `base.item_stack` | `(item)` |  |
| `base.item_unit` | `(item)` |  |
| `base.item_unit_get_item` | `(unit)` |  |
| `base.item_blink` | `(item, target)` |  |
| `base.item_get_holder` | `(item)` |  |
| `base.item_get_name` | `(item)` |  |
| `base.item_grant_tag` | `(item)` |  |
| `base.item_get_owner` | `(item)` |  |
| `base.item_remove` | `(item)` |  |
| `base.drop_item` | `(item)` |  |
| `base.item_skill` | `(item)` |  |
| `base.item_get_equip_state` | `(item)` |  |
| `base.get_inventory_items` | `(unit, index)` |  |
| `base.give_item_to_inventory` | `(item, unit, index)` |  |
| `base.get_item_info` | `(item)` |  |
| `base.load_item_from_info` | `(info, unit)` |  |
| `base.get_obj_items` | `()` |  |
| `base.load_score_item_to_unit` | `(unit, success_callback, error_callback)` |  |
| `base.save_score_item_to_unit` | `(unit, success_callback, error_callback)` |  |
| `base.get_player_score_item_list` | `(player)` |  |
| `base.bind_items_to_user` | `(items, player, success_callback, error_callback)` |  |
| `base.unbind_items_to_user` | `(player, success_callback, error_callback)` |  |

### `@lua_plus/base/base_lua_plus/lightning`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\lightning.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.lightning_remove` | `(lightning)` |  |
| `base.player_create_lightning` | `(player, model, source, target)` | 被代码中的定义覆盖 |
| `base.unit_create_lightning` | `(unit, model, source, target)` | 被代码中的定义覆盖 |

### `@lua_plus/base/base_lua_plus/localization`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\localization.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `_G.get_text` | `(id)` | 服务端的get_text假处理 |

### `@lua_plus/base/base_lua_plus/loot_pool`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\loot_pool.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@lua_plus/base/base_lua_plus/minimap`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\minimap.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.create_icon` | `(player, name, point)` |  |
| `base.icon_set_sync` | `(icon, sync)` |  |
| `base.icon_hide` | `(icon)` |  |
| `base.icon_hide_team` | `(icon, team)` |  |
| `base.icon_show` | `(icon)` |  |
| `base.icon_set_time` | `(icon, time)` |  |
| `base.minimap_signal` | `(player, name, point)` |  |

### `@lua_plus/base/base_lua_plus/mover`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\mover.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_last_created_mover` | `()` |  |
| `base.skill_mover_line` | `(mover, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` |  |
| `follow_or_move_to` | `(moving_unit, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` |  |
| `base.skill_mover_target` | `(moving_unit, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` |  |
| `base.mover_batch_update` | `(mover)` |  |
| `base.mover_remove` | `(mover)` |  |
| `base.unit_each_mover` | `(unit)` |  |
| `base.unit_follow` | `(mover, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` |  |

### `@lua_plus/base/base_lua_plus/player`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\player.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.player_add_attribute` | `(player, state, value)` |  |
| `base.get_player_controller` | `(player)` |  |
| `base.player_event_dispatch` | `(player:player, name:单位事件, ...)` |  |
| `base.player_event_notify` | `(player:player, name:单位事件, ...)` |  |
| `base.player_game_state` | `(player)` |  |
| `base.player_get_attribute` | `(player, state)` |  |
| `base.player_get_hero` | `(player)` |  |
| `base.player_get_slot_id` | `(player)` |  |
| `base.player_get_team_id` | `(player)` |  |
| `base.get_player_input_rocker` | `(player)` |  |
| `base.is_player_abort` | `(player)` |  |
| `base.kick_player` | `(player, backend, frontend)` |  |
| `base.player_leave_reason` | `(player)` |  |
| `base.player_send_message` | `(player, text, type, time)` |  |
| `base.player_message_box` | `(player, text)` |  |
| `base.player_set_attribute_number` | `(player, state, value)` |  |
| `base.player_set_attribute_string` | `(player, state, value)` |  |
| `base.player_set_afk` | `(player)` |  |
| `base.player_set_hero` | `(player, hero)` |  |
| `base.player_set_team_id` | `(player, id)` |  |
| `base.get_player_user_agent` | `(player)` |  |
| `base.player_user_id` | `(player)` |  |
| `base.player_get_scene_name` | `(player)` |  |
| `base.player_get_user_nick` | `(player)` |  |
| `base.get_each_player` | `(type)` |  |
| `base.player_set_hero_skill_sync_type` | `(player, sync)` |  |

### `@lua_plus/base/base_lua_plus/point`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\point.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.point_angle` | `(point, target)` |  |
| `base.point_copy` | `(point)` |  |
| `base.point_distance` | `(point, target)` |  |
| `base.point_get_x` | `(point)` |  |
| `base.point_get_y` | `(point)` |  |
| `base.point_is_block` | `(point, scene_name, prevent_bits, required_bits)` |  |
| `base.point_is_block2` | `(point, prevent_bits, required_bits)` |  |
| `base.point_is_block_all` | `(point, scene_name)` |  |
| `base.point_is_block_all2` | `(point)` |  |
| `base.point_is_visible_to_unit` | `(point, dest, scene_name)` |  |
| `base.point_is_visible_to_unit2` | `(point, dest)` |  |
| `base.point_is_visible_to_player` | `(point, dest, scene_name)` |  |
| `base.point_is_visible_to_player` | `(point, dest)` |  |
| `base.point_move` | `(point, angle, distance)` |  |
| `base.get_scene_point` | `(scene, area_name, present)` |  |
| `base.get_scene_line` | `(scene, area_name, present)` |  |
| `base.get_point_scene` | `(point)` |  |
| `base.line_get` | `(line, index)` |  |
| `base.pathing_way_points` | `(st, ed)` |  |

### `@lua_plus/base/base_lua_plus/quest`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\quest.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.unit_receive_quest` | `(unit, id)` |  |
| `base.unit_get_quests` | `(unit)` |  |
| `base.unit_get_quest` | `(unit, id)` |  |
| `base.unit_get_quest_conditions` | `(unit)` |  |
| `base.unit_get_quest_condition` | `(unit, id)` |  |
| `base.quest_get_link` | `(quest)` |  |
| `base.quest_get_id` | `(quest)` |  |
| `base.quest_get_owner` | `(quest)` |  |
| `base.quest_get_conditions` | `(quest)` |  |
| `base.quest_get_active` | `(quest)` |  |
| `base.quest_get_complete` | `(quest)` |  |
| `base.quest_get_progress` | `(quest)` |  |
| `base.quest_get_progress_total` | `(quest)` |  |
| `base.quest_get_can_submit` | `(quest)` |  |
| `base.quest_condition_get_link` | `(quest_condition)` |  |
| `base.quest_condition_get_id` | `(quest_condition)` |  |
| `base.quest_condition_get_quest` | `(quest_condition)` |  |
| `base.quest_condition_get_owner` | `(quest_condition)` |  |
| `base.quest_condition_get_active` | `(quest_condition)` |  |
| `base.quest_condition_get_complete` | `(quest_condition)` |  |
| `base.quest_condition_get_progress` | `(quest_condition)` |  |
| `base.quest_condition_get_progress_total` | `(quest_condition)` |  |
| `base.quest_condition_get_can_submit` | `(quest_condition)` |  |
| `base.quest_reset` | `(quest)` |  |
| `base.quest_activate` | `(quest)` |  |
| `base.quest_deactivate` | `(quest)` |  |
| `base.quest_submit` | `(quest)` |  |
| `base.quest_get_current_condition` | `(quest)` |  |
| `base.quest_condition_set_progress` | `(quest_condition, progress)` |  |
| `base.quest_condition_add_progress` | `(quest_condition, progress)` |  |
| `base.quest_condition_set_active_state` | `(quest_condition, state)` |  |
| `base.quest_condition_set_complete_state` | `(quest_condition, state)` |  |
| `base.quest_condition_submit` | `(quest_condition)` |  |
| `base.load_score_quest_to_unit` | `(unit, success_callback, error_callback)` |  |
| `base.save_score_quest_to_unit` | `(unit, success_callback, error_callback)` |  |
| `base.get_player_score_quest_list` | `(player)` |  |
| `base.bind_quests_to_user` | `(quests, player, success_callback, error_callback)` |  |
| `base.unbind_quests_to_user` | `(player, success_callback, error_callback)` |  |

### `@lua_plus/base/base_lua_plus/simple_ui`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\simple_ui.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_last_created_component` | `()` |  |
| `base.component_position` | `(x, y)` |  |
| `base.component_size` | `(width, height)` |  |
| `base.component_color` | `(r, g, b)` |  |
| `base.create_component_button` | `(position, size, text, visiblity, event_label)` | 创建------------------------------------- |
| `base.create_component_picture` | `(position, size, image, visiblity, event_label)` |  |
| `base.create_component_text` | `(position, size, text, font_size, visiblity, event_label)` |  |
| `base.destroy_component` | `(component_id)` | 移除------------------------------------- |
| `base.set_component_position` | `(player, component_id, position)` |  |
| `base.set_component_size` | `(player, component_id, size)` |  |
| `base.set_component_visiblity` | `(component_id, player, visiblity)` |  |
| `base.set_component_color` | `(player, component_id, color)` |  |
| `base.set_component_can_be_clicked` | `(player, component_id, can_be_clicked)` |  |
| `base.set_component_text` | `(player, component_id, text)` |  |
| `base.set_component_font_size` | `(player, component_id, font_size)` |  |
| `base.set_component_image` | `(player, component_id, image)` |  |
| `base.set_component_opacity` | `(player, component_id, opacity)` |  |
| `base.set_component_zoom_type` | `(player, component_id, zoom_type)` |  |
| `base.set_component_auto_line_feed` | `(player, component_id, auto_line_feed)` |  |
| `base.set_component_text_align` | `(player, component_id, align)` |  |
| `base.set_component_text_vertical_align` | `(player, component_id, vertical_align)` |  |
| `base.get_component_position` | `(player, component_id)` |  |
| `base.get_component_size` | `(player, component_id)` |  |
| `base.get_component_visiblity` | `(component_id, player)` |  |
| `base.get_component_color` | `(player, component_id)` |  |
| `base.get_component_can_be_clicked` | `(player, component_id)` |  |
| `base.get_component_text` | `(player, component_id)` |  |
| `base.get_component_font_size` | `(player, component_id)` |  |
| `base.get_component_image` | `(player, component_id)` |  |
| `base.get_component_opacity` | `(player, component_id)` |  |
| `base.get_component_zoom_type` | `(player, component_id)` |  |
| `base.get_component_auto_line_feed` | `(player, component_id)` |  |
| `base.get_component_text_align` | `(player, component_id)` |  |
| `base.get_component_text_vertical_align` | `(player, component_id)` |  |
| `base.ui.proto.component_event` | `(player, msg)` |  |

### `@lua_plus/base/base_lua_plus/skill`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\skill.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.get_last_created_skill` | `()` |  |
| `base.unit_add_skill` | `(unit, id, skill_type, slot)` |  |
| `base.add_skill_to_slot` | `(unit, id, slot)` |  |
| `base.add_skill` | `(unit, id, slot)` |  |
| `base.add_skill_simple` | `(unit, id)` |  |
| `base.skill_active_cd` | `(skill, max_cd, ignore_cooldown_reduce)` |  |
| `base.skill_active_custom_cd` | `(skill, max_cd, cd)` |  |
| `base.skill_add_level` | `(skill, level)` |  |
| `base.skill_add_stack` | `(skill, stack)` |  |
| `base.skill_get_attribute` | `(skill, attr)` |  |
| `base.skill_set_attribute` | `(skill, attr, val)` |  |
| `base.skill_get_stage` | `(skill)` |  |
| `base.skill_stage_finish` | `(skill)` |  |
| `base.skill_disable` | `(skill)` |  |
| `base.skill_enable` | `(skill)` |  |
| `base.skill_enable_hidden` | `(skill)` |  |
| `base.skill_disable_hidden` | `(skill)` |  |
| `base.skill_get_cd` | `(skill)` |  |
| `base.skill_get_level` | `(skill)` |  |
| `base.skill_get_name` | `(skill)` |  |
| `base.skill_get_slot_id` | `(skill)` |  |
| `base.skill_get_owner` | `(skill)` |  |
| `base.skill_get_last_target_unit` | `(skill)` |  |
| `base.skill_get_target_unit` | `(skill)` |  |
| `base.skill_get_target_point` | `(skill)` |  |
| `base.skill_get_target_angle` | `(skill)` |  |
| `base.skill_get_type` | `(skill)` |  |
| `base.skill_is_cast` | `(skill)` |  |
| `base.skill_is_enable` | `(skill)` |  |
| `base.skill_is_skill` | `(skill)` |  |
| `base.skill_notify_damage` | `(skill, damage)` |  |
| `base.skill_reload` | `(skill)` |  |
| `base.skill_remove` | `(skill)` |  |
| `base.skill_set` | `(skill, key, value)` |  |
| `base.skill_set_animation` | `(skill, animation)` |  |
| `base.skill_set_cd` | `(skill, cd, force)` |  |
| `base.skill_set_level` | `(skill, level)` |  |
| `base.skill_set_option` | `(skill, key, value)` |  |
| `dummy` | `()` |  |
| `base.skill_simple_cast` | `(skill)` |  |
| `base.skill_stop` | `(skill)` |  |
| `base.unit_blink` | `(unit, target)` |  |
| `base.unit_can_attack` | `(unit, target)` |  |
| `base.same_skill` | `(skill_a, skill_b)` |  |
| `base.unit_cast_smart` | `(unit, id)` |  |
| `base.unit_cast` | `(unit, id)` |  |
| `base.unit_cast_on_angel` | `(unit, id, target)` |  |
| `base.unit_cast_on_unit` | `(unit, id, target)` |  |
| `base.unit_cast_on_point` | `(unit, id, point)` |  |
| `base.unit_cast_skill` | `(unit, id)` |  |
| `base.unit_cast_skill_on_unit` | `(unit, id, target)` |  |
| `base.unit_cast_skill_on_point` | `(unit, id, point)` |  |
| `base.unit_clean_command` | `(unit)` |  |
| `base.unit_current_skill` | `(unit)` |  |
| `base.unit_each_skill` | `(unit, skill_type)` |  |
| `base.unit_find_skill_by_name` | `(unit, id, include_level_zero)` |  |
| `base.unit_find_skill_by_slot` | `(unit, slot)` |  |
| `base.get_all_skills_id` | `()` |  |
| `base.unit_all_skill` | `(unit)` |  |
| `base.skill_can_learn` | `(skill)` |  |
| `base.skill_learn` | `(skill)` |  |

### `@lua_plus/base/base_lua_plus/snapshot`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\snapshot.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.snapshot_get_point` | `(snapshot)` |  |
| `base.snapshot_get_name` | `(snapshot)` |  |
| `base.snapshot_get_owner` | `(snapshot)` |  |
| `base.snapshot_get_facing` | `(snapshot)` |  |

### `@lua_plus/base/base_lua_plus/sound`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\sound.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.player_play_music` | `(player, path)` |  |
| `base.player_play_sound` | `(player, name)` |  |
| `base.point_play_sound` | `(point, name, distance, scene_name)` |  |
| `base.point_play_sound2` | `(point, name, distance)` |  |

### `@lua_plus/base/base_lua_plus/tds_score`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\tds_score.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.score_commit_init` | `(game_name)` |  |
| `base.get_last_created_score_committer` | `()` |  |
| `base.get_last_commit_success` | `()` |  |
| `base.get_last_commit_error_code` | `()` |  |
| `base.get_last_commit_error_msg` | `()` |  |
| `base.string_to_score_game` | `(game_name)` |  |
| `base.score_money_get` | `(player, key)` |  |
| `base.score_geti` | `(player, key)` |  |
| `base.score_exist` | `(player, key)` |  |
| `base.score_money_exist` | `(player, key)` |  |
| `base.score_gets` | `(player, key)` |  |
| `base.score_get` | `(player, key)` |  |
| `base.score_c_money_set` | `(c, player, key, value)` |  |
| `base.score_c_money_add` | `(c, player, key, value)` |  |
| `base.score_c_money_cost` | `(c, player, key, value)` |  |
| `base.score_c_seti` | `(c, player, key, value)` |  |
| `base.score_c_addi` | `(c, player, key, value)` |  |
| `base.score_c_sets` | `(c, player, key, value)` |  |
| `base.score_c_set` | `(c, player, key, value)` |  |
| `base.score_c_commit` | `(c)` |  |

### `@lua_plus/base/base_lua_plus/test`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\test.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `debug_bp_confident` | `(...)` |  |

### `@lua_plus/base/base_lua_plus/timer`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\timer.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.timer_clock` | `()` |  |
| `base.timer_remove` | `(timer)` |  |
| `base.timer_resume` | `(timer)` |  |
| `base.timer_pause` | `(timer)` |  |
| `base.timer_restart` | `(timer)` |  |
| `base.timer_sleep` | `(time)` |  |
| `base.timer_wait` | `(time, func)` |  |
| `base.timer_loop` | `(time, func)` |  |
| `base.timer_timer` | `(time, times, func)` |  |
| `base.remaining` | `(timer)` |  |

### `@lua_plus/base/base_lua_plus/timershow`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\timershow.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.create_timershow` | `(x, y, time)` |  |
| `base.remove_timershow` | `(name)` |  |
| `base.pause_timershow` | `(name)` |  |
| `base.resume_timershow` | `(name)` |  |
| `base.add_player_timershow_visible` | `(name, player)` |  |
| `base.del_player_timershow_visible` | `(name, player)` |  |
| `base.assign_timershow` | `(name, timer)` |  |

### `@lua_plus/base/base_lua_plus/trigger`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\trigger.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.trigger_disable` | `(trigger)` |  |
| `base.trigger_enable` | `(trigger)` |  |
| `base.trigger_is_enable` | `(trigger)` |  |
| `base.trigger_remove` | `(trigger)` |  |
| `base.trigger_new` | `(func, t, disable, scene, sync)` |  |
| `base.trigger_add_event` | `(trigger, trigger_event)` |  |
| `base.trigger_event_wrapper_unit` | `(unit, event_name)` | 把触发事件表包装成函数 |
| `base.trigger_event_wrapper_skill` | `(skill, event_name)` |  |
| `base.trigger_event_wrapper_eff_param` | `(eff_param, event_name)` |  |
| `base.trigger_event_wrapper_player` | `(player, event_name)` |  |
| `base.trigger_event_wrapper_game` | `(event_name)` |  |
| `base.trigger_event_wrapper_mover` | `(mover, event_name)` |  |
| `base.trigger_event_wrapper_timer_periodic` | `(time)` |  |
| `base.trigger_event_wrapper_timer_once` | `(time)` |  |
| `base.trigger_event_wrapper_area` | `(area, event_name)` |  |
| `base.trigger_custom_event_wrapper` | `(event_name)` |  |
| `base.trigger_call` | `(trigger, e, sync)` |  |

### `@lua_plus/base/base_lua_plus/unit`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\unit.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.unit_set_loot` | `(unit, link)` |  |
| `base.get_last_created_unit` | `()` |  |
| `base.get_all_units_id` | `()` |  |
| `base.player_create_unit` | `(player, id, where, face)` |  |
| `base.player_create_unit_ai` | `(player, id, where, face, default_ai)` |  |
| `base.player_create_unit_on_scene` | `(player, id, where, face, scene)` |  |
| `base.player_create_unit_illusion` | `(player, unit, where, face)` |  |
| `base.player_create_unit_illusion_on_scene` | `(player, unit, where, face, scene)` |  |
| `base.unit_create_unit_illusion` | `(unit, dest, where, face)` |  |
| `base.create_unit_illusion` | `(unit, where, face)` |  |
| `base.unit_get_id` | `(unit)` |  |
| `base.unit_set_scale` | `(unit, scale)` |  |
| `base.unit_add_attribute` | `(unit, state, value)` |  |
| `base.unit_add_attribute_ex` | `(unit, state, value, value_type)` |  |
| `base.unit_add_ai` | `(unit, name, data)` |  |
| `base.unit_play_animation` | `(unit, name, speed, loop, part)` | cover play_animation |
| `base.unit_add_height` | `(unit, height)` |  |
| `base.unit_add_provide_sight` | `(unit, team)` |  |
| `base.unit_add_resource` | `(unit, energy_type, value)` |  |
| `base.unit_add_mark` | `(unit, unit_type)` | cover add_restriction |
| `base.unit_add_sight` | `(unit, sight)` |  |
| `base.unit_event_notify` | `(unit:unit, name:string, ...)` |  |
| `base.unit_event_has` | `(unit:unit, name:string)` |  |
| `base.unit_event_subscribe` | `(unit:unit, name:string)` |  |
| `base.unit_event_unsubscribe` | `(unit:unit, name:string)` |  |
| `base.unit_get_attribute` | `(unit, state)` | TODO 多返回值 |
| `base.unit_get_attribute_ex` | `(unit, state, value_type)` |  |
| `base.unit_get_attribute_max` | `(unit, state)` |  |
| `base.unit_get_attribute_min` | `(unit, state)` |  |
| `base.unit_get_class` | `(unit)` |  |
| `base.unit_get_facing` | `(unit)` |  |
| `base.unit_get_height` | `(unit)` |  |
| `base.unit_get_creation_param` | `(unit)` |  |
| `base.unit_get_name` | `(unit)` |  |
| `base.unit_get_player` | `(unit)` |  |
| `base.unit_set_player` | `(unit, player)` |  |
| `base.unit_get_point` | `(unit)` |  |
| `base.unit_get_resource` | `(unit, resource_type)` |  |
| `base.unit_get_mark` | `(unit, unit_mark)` | cover get_restriction |
| `base.unit_get_attackable_radius` | `(unit)` |  |
| `base.unit_get_team_id` | `(unit)` |  |
| `base.unit_get_tag` | `(unit)` |  |
| `base.unit_get_walk_command_a` | `(unit)` |  |
| `base.unit_get_walk_command_b_point` | `(unit)` |  |
| `base.unit_get_walk_command_b_unit` | `(unit)` |  |
| `base.unit_walk` | `(unit, target)` |  |
| `base.unit_has_mark` | `(unit, unit_mark)` | cover has_restriction |
| `base.unit_is_alive` | `(unit)` |  |
| `base.unit_is_ally_of_unit` | `(unit, dest)` |  |
| `base.unit_is_ally_of_player` | `(unit, dest)` |  |
| `base.unit_is_enemy_of_unit` | `(unit, dest)` |  |
| `base.unit_is_enemy_of_player` | `(unit, dest)` |  |
| `base.unit_is_illusion` | `(unit)` |  |
| `base.unit_is_in_range_of_unit` | `(unit, target, radius)` |  |
| `base.unit_is_in_range_of_point` | `(unit, target, radius)` |  |
| `base.unit_is_visible_to_unit` | `(unit, target)` |  |
| `base.unit_is_visible_to_player` | `(unit, target)` |  |
| `base.unit_is_walking` | `(unit)` |  |
| `base.unit_add_z_speed` | `(unit, speed)` |  |
| `base.unit_set_z_speed` | `(unit, speed)` |  |
| `base.unit_get_z_speed` | `(unit)` |  |
| `base.unit_kill` | `(unit, killer)` |  |
| `base.unit_learn_skill` | `(unit, skill)` |  |
| `base.unit_reborn` | `(unit, where)` |  |
| `base.unit_remove` | `(unit)` |  |
| `base.unit_remove_animation` | `(unit, animation_name)` |  |
| `base.unit_remove_buff` | `(unit, buff_name)` |  |
| `base.unit_remove_privide_sight` | `(unit, team_id)` |  |
| `base.unit_remove_mark` | `(unit, unit_mark)` | cover:remove_restriction |
| `base.unit_replace_skill` | `(unit, skill_id_old, skill_id_new)` |  |
| `base.unit_set` | `(unit, state, value)` | 仅限内置属性 |
| `base.unit_set_ex` | `(unit, state, value, value_type)` |  |
| `base.unit_set_str` | `(unit, state, value)` |  |
| `base.unit_set_attribute_max` | `(unit, state, value)` |  |
| `base.unit_set_attribute_min` | `(unit, state, value)` |  |
| `base.unit_set_attribute_sync` | `(unit, state, sync)` |  |
| `base.unit_set_facing` | `(unit, facing)` |  |
| `base.unit_set_height` | `(unit, height)` |  |
| `base.unit_set_model` | `(unit, model)` |  |
| `base.unit_set_resource` | `(unit, energy_type, value)` |  |
| `base.unit_set_attackable_radius` | `(unit, radius)` |  |
| `base.unit_texttag` | `(unit, target, text, text_type, sync, r, g, b, size)` |  |
| `base.unit_get_scene_name` | `(unit)` |  |
| `base.unit_jump_scene` | `(unit, scene_name, position)` |  |
| `base.unit_jump_scene2` | `(unit, position)` |  |
| `base.get_all_units` | `()` |  |
| `base.node_mark` | `(node_mark, unit_name)` |  |
| `base.set_location_async` | `(unit, position)` |  |
| `base.set_facing_async` | `(unit, facing)` |  |
| `base.unit_anim_play` | `(unit, anim, time, time_type, start_offset, blend_time, priority)` |  |
| `base.unit_anim_set_paused_all` | `(unit, paused)` |  |
| `base.unit_set_time_scale_global` | `(unit, time_scale)` |  |
| `base.unit_anim_play_bracket` | `(unit, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority, sync)` |  |
| `base.unit_get_exp` | `(unit)` |  |
| `base.unit_add_exp` | `(unit, exp, ignore_fraction)` |  |
| `base.unit_set_exp` | `(unit, exp)` |  |
| `base.unit_get_level` | `(unit)` |  |
| `base.unit_add_level` | `(unit, level)` |  |
| `base.unit_set_level` | `(unit, level)` |  |
| `base.unit_get_max_level` | `(unit)` |  |
| `base.unit_set_max_level` | `(unit, max_level)` |  |
| `base.unit_get_single_level_exp` | `(unit, level)` |  |
| `base.unit_get_cumu_level_exp` | `(unit, level)` |  |
| `base.unit_get_exp_fraction` | `(unit)` |  |
| `base.unit_set_exp_fraction` | `(unit, fraction)` |  |
| `base.unit_set_prohibit_exp_distribute` | `(unit, value)` |  |
| `base.unit_set_level_profile` | `(unit, profile_id)` |  |
| `base.unit_grant_loot` | `(unit, target, link)` |  |
| `base.get_unit_from_id` | `(id)` |  |

### `@lua_plus/base/base_lua_plus/单位组`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\单位组.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get_items_table_mt` | `(items_table_name, item_check, tables_list)` |  |
| `mt:check_items_table` | `(newtable)` |  |
| `mt:__add` | `(newtable)` |  |
| `mt:__sub` | `(newtable)` |  |
| `mt:__eq` | `(newtable)` |  |
| `mt:__tostring` | `()` |  |
| `mt:add_item` | `(item)` |  |
| `mt:add_items` | `(items)` |  |
| `mt:_remove_item` | `(item)` |  |
| `mt:refresh` | `()` |  |
| `mt:remove_item` | `(item)` |  |
| `mt:remove_items` | `(items)` |  |
| `mt:copy` | `()` |  |
| `mt:contains` | `(item)` |  |
| `mt:union` | `(newtable)` |  |
| `mt:sub` | `(newtable)` |  |
| `mt:intersect` | `(newtable)` |  |
| `mt:get_length` | `()` |  |
| `mt.new` | `()` |  |
| `mt:get_items_map` | `()` |  |
| `mt:add` | `(item)` | 兼容v2 |
| `mt:has` | `(item)` |  |
| `mt:delete` | `(item)` |  |
| `mt:clear` | `(item)` |  |
| `mt:forEachEx` | `(callbackfn)` |  |
| `mt:random` | `()` |  |
| `mt:randoms` | `(number, duplicate)` |  |
| `mt:values` | `()` |  |
| `next` | `(self)` |  |
| `base.单位组` | `(单位数组)` |  |
| `base.create_unit_group` | `(units)` |  |
| `base.unit_group_random_unit` | `(ug)` |  |
| `base.unit_group_random_units` | `(ug, cnt)` |  |
| `base.unit_group_forEachEx` | `(ug, callbackfn)` |  |

### `@lua_plus/base/base_lua_plus/单位组_玩家组api`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\单位组_玩家组api.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.unit_group_add_item` | `(单位组, 单位)` |  |
| `base.unit_group_add_items` | `(单位组, 目标单位组)` |  |
| `base.unit_group_contains` | `(单位组, 单位)` |  |
| `base.unit_group_copy` | `(单位组)` |  |
| `base.unit_group_remove_item` | `(单位组, 单位)` |  |
| `base.unit_group_remove_items` | `(单位组, 目标单位组)` |  |
| `base.unit_group_union` | `(单位组, 目标单位组)` |  |
| `base.unit_group_sub` | `(单位组, 目标单位组)` |  |
| `base.unit_group_intersect` | `(单位组, 目标单位组)` |  |
| `base.unit_group_count` | `(单位组)` |  |
| `base.unit_group_get_items_map` | `(单位组)` |  |

### `@lua_plus/base/base_lua_plus/单位过滤器`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\单位过滤器.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `check_target_filter` | `(target_filter)` |  |
| `base.target_filter_validate_on_unit` | `(...)` |  |
| `base.target_filter_validate` | `(过滤, 过滤单位, 基准单位)` |  |
| `base.unit_group_filter_group_on_unit` | `(...)` |  |
| `base.unit_group_filter_group` | `(单位组, 过滤, 基准单位)` |  |

### `@lua_plus/base/base_lua_plus/附着点`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\base_lua_plus\附着点.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.附着点` | `(unit, socket)` | TODO pay/backend/debugger/score |

### `@lua_plus/base/obj_check`

- 归属：server_lua_plus 包（lua_plus 库）
- 状态：✅ 有源码（`server_lua_plus\14\base\obj_check.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@tds_score`

- 归属：tds_score 库（无独立包，引擎侧）
- 研判：TDS（TapTap 开发者服务）排行榜/积分库；`use_mysql` 键名表明可走 MySQL 存储。无独立分发包，引擎侧实现；参照 server_lua_plus base_lua_plus/tds_score.lua 包装器。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：4 个文件（如 `const.lua` 等）
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@tds_score/new_base_score`

- 归属：tds_score 库（无独立包，引擎侧）
- 研判：TDS（TapTap 开发者服务）排行榜/积分库；`use_mysql` 键名表明可走 MySQL 存储。无独立分发包，引擎侧实现；参照 server_lua_plus base_lua_plus/tds_score.lua 包装器。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@tds_score/score`

- 归属：tds_score 库（无独立包，引擎侧）
- 研判：TDS（TapTap 开发者服务）排行榜/积分库；`use_mysql` 键名表明可走 MySQL 存储。无独立分发包，引擎侧实现；参照 server_lua_plus base_lua_plus/tds_score.lua 包装器。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.score_c_addi(c, player, key, value)`
  - `base.score_c_addi(c:score_committer, player:player, key:string, value:number)`
  - `base.score_c_commit(c)`
  - `base.score_c_commit(c:score_committer)`
  - `base.score_c_money_add(c, player, key, value)`
  - `base.score_c_money_add(c:score_committer, player:player, key:string, value:number)`
  - `base.score_c_money_cost(c, player, key, value)`
  - `base.score_c_money_cost(c:score_committer, player:player, key:string, value:number)`
  - `base.score_c_money_set(c, player, key, value)`
  - `base.score_c_money_set(c:score_committer, player:player, key:string, value:number)`
  - `base.score_c_set(c, player, key, value)`
  - `base.score_c_set(c:score_committer, player:player, key:string, value:unknown)`
  - `base.score_c_seti(c, player, key, value)`
  - `base.score_c_seti(c:score_committer, player:player, key:string, value:number)`
  - `base.score_c_sets(c, player, key, value)`
  - `base.score_c_sets(c:score_committer, player:player, key:string, value:string)`
  - `base.score_commit_init(game_name)`
  - `base.score_commit_init(game_name:score_game)`
  - `base.score_exist(player, key)`
  - `base.score_exist(player:player, key:string)`
  - `base.score_get(player, key)`
  - `base.score_get(player:player, key:string)`
  - `base.score_geti(player, key)`
  - `base.score_geti(player:player, key:string)`
  - `base.score_gets(player, key)`
  - `base.score_gets(player:player, key:string)`
  - `base.score_money_exist(player, key)`
  - `base.score_money_exist(player:player, key:string)`
  - `base.score_money_get(player, key)`
  - `base.score_money_get(player:player, key:string)`

### `@tds_score/tds_score`

- 归属：tds_score 库（无独立包，引擎侧）
- 研判：TDS（TapTap 开发者服务）排行榜/积分库；`use_mysql` 键名表明可走 MySQL 存储。无独立分发包，引擎侧实现；参照 server_lua_plus base_lua_plus/tds_score.lua 包装器。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@tds_score/use_mysql`

- 归属：tds_score 库（无独立包，引擎侧）
- 研判：TDS（TapTap 开发者服务）排行榜/积分库；`use_mysql` 键名表明可走 MySQL 存储。无独立分发包，引擎侧实现；参照 server_lua_plus base_lua_plus/tds_score.lua 包装器。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。
