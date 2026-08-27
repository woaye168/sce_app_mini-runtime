# script 包 · common 库 base/game 子层（引擎实现，无源码）

模块数：15。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）。

源码覆盖：0/15；其余标注 ⚠️ 无源码并附调用点反查/语义推测。

## 组级调用点反查：`base.game.*` 全部已观测形态（推测）

`@common/base/game/*` 各子模块均引擎内嵌、运行时挂到 `base.game` 表上；从键名无法判定某个函数属于哪个子模块，故把全语料观测到的 `base.game.*` 调用形态整体列出：

- `base.game.cancel_time_stop()`
- `base.game.clear_debug_draws(actor)`
- `base.game.close_scene(old_scene)`
- `base.game.create_debug_draw_actor()`
- `base.game.debug_draw_circle(actor, high_point, 0, 0, 0, 10, Color.Red, true)`
- `base.game.debug_draw_circle(actor, high_point, 0, 0, 0, eff_data.radius, color, false)`
- `base.game.debug_draw_circle(actor, high_point, 0, 0, 0, root_cache.Range, Color.Aquam...)`
- `base.game.debug_draw_circle(actor, point, euler_alpha, euler_beta, euler_gamma, radiu...)`
- `base.game.debug_draw_line(actor, a, b, Color.Red)`
- `base.game.debug_draw_line(actor, a, d, Color.Red)`
- `base.game.debug_draw_line(actor, b, c, Color.Red)`
- `base.game.debug_draw_line(actor, c, d, Color.Red)`
- `base.game.debug_draw_line(actor, s_point, e_point, color)`
- `base.game.debug_draw_line(actor, s_point, t_point, Color.Yellow)`
- `base.game.debug_draw_line(actor, s_point, t_point, line_color)`
- `base.game.debug_draw_line(actor, this_high_point, parent_high_point, color)`
- `base.game.debug_draw_point(actor, point, color)`
- `base.game.debug_draw_rectangle(actor, v_point, w_point, h_point, color, solid)`
- `base.game.debug_draw_sector(actor, high_point, 0, 0, eff_data.angle - eff_data.Arc/2,...)`
- `base.game.debug_draw_sector(actor, point, euler_alpha, euler_beta, euler_gamma, radiu...)`
- `base.game.debug_draw_text(actor, high_point_text, base.i18n.get_text(root_name)`
- `base.game.debug_draw_text(actor, point, text, color, displayTop)`
- `base.game.debug_draw_text(actor, s_point, info, Color.Yellow, true)`
- `base.game.default_game_result(data)`
- `base.game.default_game_result({result = 'failed', player = player})`
- `base.game.default_game_result({result = 'win', player = player})`
- `base.game.ensure_one_lobby(player)`
- `base.game.get_all_scene_name()`
- `base.game.get_current_scene()`
- `base.game.get_default_unit('"..id.."')`
- `base.game.get_default_unit(node_mark)`
- `base.game.get_default_unit(value.node_mark)`
- `base.game.get_mode_key()`
- `base.game.get_model_anim_point_info(model_path, anim_name)`
- `base.game.get_scene_object_by_key(scene)`
- `base.game.get_scene_scale(scene_name)`
- `base.game.get_server_tag(user_id, data.server_key)`
- `base.game.get_session_id()`
- `base.game.is_camera_locked()`
- `base.game.load_combined_map(scene, direction)`
- `base.game.load_combined_map_deco(scene, direction)`
- `base.game.load_scene('draw')`
- `base.game.load_scene(scene)`
- `base.game.load_scene(scene_name)`
- `base.game.load_scene_cache_and_combined(scene, direction)`
- `base.game.lock_camera()`
- `base.game.object_restore_value(object, key)`
- `base.game.object_store_value(object, key, value)`
- `base.game.one_more_round()`
- `base.game.pathing_way_points(st, ed, 0, st:get_scene()`
- `base.game.purge_combined_map()`
- `base.game.purge_combined_map_deco()`
- `base.game.set('_lib_card_game_finish', table.concat(slots, ',')`
- `base.game.set('_lib_card_item_show', value)`
- `base.game.set('hp', current_level_hp)`
- `base.game.set('hp', value)`
- `base.game.set('max_hp', value)`
- `base.game.set('scene', '__main_panel__')`
- `base.game.set('scene', level_name)`
- `base.game.set('session_id', poi.session_id)`
- `base.game.set('skill_learn', value)`
- `base.game.set('skill_upgrade', value)`
- `base.game.set('smallcard_arpg_map_jump_scene',scene)`
- `base.game.set('smallcard_game_speed_pause', game_pause and '1' or '0')`
- `base.game.set('smallcard_game_speed_speed', tostring(speed)`
- `base.game.set('task.accept', id)`
- `base.game.set('task.update', task.id .. ',' .. index .. ',' .. value)`
- `base.game.set('总时间', total_time)`
- `base.game.set('提前金币', ahead_gold)`
- `base.game.set('雅典娜', poi.base:get_id()`
- `base.game.set(key, value)`
- `base.game.set_dynamic_point_light(val)`
- `base.game.set_game_speed(value)`
- `base.game.switch_fov_mode(number, scene)`
- `base.game.time_stop()`
- `base.game.time_stop(sec)`
- `base.game.unlock_camera()`

---

---

### `@common/base/game/error_info`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/game`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `__tostring()`
  - `add_damage(...)`
  - `broadcast('process_message', function(title, message)`
  - `broadcast('progress_update_content', function(content_text, progres...)`
  - `broadcast('progress_update_error_text', function(error_text)`
  - `broadcast('progress_update_tips', function(...)`
  - `broadcast('progress_update_title', function(title_text)`
  - `broadcast('send_logs', function()`
  - `broadcast('set_font_family', function(family)`
  - `broadcast('show_errors', function(title)`
  - `broadcast('update_map_finish', function(seq, success, error_msg)`
  - `broadcast('upload_applog_finish', function()`
  - `broadcast(name, f)`
  - `camera_focus(hero)`
  - `camera_focus(local_hero)`
  - `camera_focus(unit)`
  - `cancel_keep_alive()`
  - `cancel_time_stop()`
  - `chat('全体', '测试聊天-全体')`
  - `chat('队伍', '测试聊天-队伍')`
  - `chat(target, msg)`
  - `chat(type, msg)`
  - `circle_selector(base_point, range)`
  - `circle_selector(base_point, range, '', false)`
  - `circle_selector(circle:get_scene_point()`
  - `circle_selector(point, 150, '物品')`
  - `circle_selector(point, 50)`
  - `circle_selector(point, range, '', false)`
  - `circle_selector(point, range, '物品')`
  - `circle_selector(pos, radius, tag, ignore_center_pos)`
  - `circle_selector(position, radius, Tag)`
  - `circle_selector(target, max_collision_radius)`
  - `circle_selector(unit:get_point()`
  - `clear_debug_draws(actor)`
  - `close_scene(old_scene)`
  - `close_scene(scene_name)`
  - `create_debug_draw_actor()`
  - `debug_draw_circle(actor, high_point, 0, 0, 0, 10, Color.Red, true)`
  - `debug_draw_circle(actor, high_point, 0, 0, 0, eff_data.radius, color, false)`
  - `debug_draw_circle(actor, high_point, 0, 0, 0, root_cache.Range, Color.Aquam...)`
  - `debug_draw_circle(actor, point, euler_alpha, euler_beta, euler_gamma, radiu...)`
  - `debug_draw_line(actor, a, b, Color.Red)`
  - `debug_draw_line(actor, a, d, Color.Red)`
  - `debug_draw_line(actor, b, c, Color.Red)`
  - `debug_draw_line(actor, c, d, Color.Red)`
  - `debug_draw_line(actor, s_point, e_point, color)`
  - `debug_draw_line(actor, s_point, t_point, Color.Yellow)`
  - `debug_draw_line(actor, s_point, t_point, line_color)`
  - `debug_draw_line(actor, this_high_point, parent_high_point, color)`
  - `debug_draw_point(actor, point, color)`
  - `debug_draw_rectangle(actor, v_point, w_point, h_point, color, solid)`
  - `debug_draw_sector(actor, high_point, 0, 0, eff_data.angle - eff_data.Arc/2,...)`
  - `debug_draw_sector(actor, point, euler_alpha, euler_beta, euler_gamma, radiu...)`
  - `debug_draw_text(actor, high_point_text, base.i18n.get_text(root_name)`
  - `debug_draw_text(actor, point, text, color, displayTop)`
  - `debug_draw_text(actor, s_point, info, Color.Yellow, true)`
  - `default_game_result(data)`
  - `default_game_result({result = 'failed', player = player})`
  - `default_game_result({result = 'win', player = player})`
  - `disable_ai()`
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.game_exit(player, show_confirm)`
  - `base.game_exit(player:player, show_confirm:boolean)`
  - `base.game_exit(show_confirm)`
  - `base.game_exit(show_confirm:boolean)`
  - `base.game_get_mouse_pos_x(touch_id)`
  - `base.game_get_mouse_pos_x(touch_id:integer)`
  - `base.game_get_mouse_pos_y(touch_id)`
  - `base.game_get_mouse_pos_y(touch_id:integer)`
  - `base.game_get_orientation()`
  - `base.game_get_resolution_height()`
  - `base.game_get_resolution_width()`
  - `base.game_screen_to_world(screen_x, screen_y)`
  - `base.game_screen_to_world(screen_x:integer, screen_y:integer)`
  - `base.game_set_resolution(width, height)`
  - `base.game_set_resolution(width:integer, height:integer)`
  - `base.game_shortcut()`
  - `base.game_ui_message(message_name, data)`
  - `base.game_ui_message(message_name:string, data:table)`
  - `base.game_world_to_screen_x(point)`
  - `base.game_world_to_screen_x(point:point)`
  - `base.game_world_to_screen_y(point)`
  - `base.game_world_to_screen_y(point:point)`

### `@common/base/game/game_message`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/game_result`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/item`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `bind_items_to_user(items, player, success_callback, error_callback)`
  - `create_to_point(cache.ItemType, base.point(0,0)`
  - `create_to_point(id, target, target:get_scene()`
  - `create_to_point(link, base.point(0,0)`
  - `create_to_point(link, hero:get_point()`
  - `create_to_point(link, unit:get_point()`
  - `create_to_point(name, point, rect.scene)`
  - `create_to_point(v.ItemType, point, unit:get_scene_name()`
  - `create_to_unit(id, target)`
  - `get_player_score_item_list(player)`
  - `load_item_from_info(info, unit)`
  - `load_item_from_info(info,unit)`
  - `load_score_to_unit(unit, success_callback, error_callback)`
  - `save_score_to_unit(unit, success_callback, error_callback)`
  - `unbind_items_to_user(player, success_callback, error_callback)`
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.item_add_extra_mod(item, buff_id_name, IsEquip)`
  - `base.item_add_extra_mod(item:item, buff_id_name:buff_id, IsEquip:IsEquip)`
  - `base.item_blink(item, target)`
  - `base.item_blink(item:item, target:point)`
  - `base.item_generate_rand_mod(item)`
  - `base.item_generate_rand_mod(item:item)`
  - `base.item_get_equip_state(item)`
  - `base.item_get_equip_state(item:item)`
  - `base.item_get_holder(item)`
  - `base.item_get_holder(item:item)`
  - `base.item_get_inventory(item)`
  - `base.item_get_inventory(item:item)`
  - `base.item_get_name(item)`
  - `base.item_get_name(item:item)`
  - `base.item_get_owner(item)`
  - `base.item_get_owner(item:item)`
  - `base.item_get_stack(item)`
  - `base.item_get_stack(item:item)`
  - `base.item_grant_tag(item)`
  - `base.item_grant_tag(item:item)`
  - `base.item_remove(item)`
  - `base.item_remove(item:item)`
  - `base.item_rnd_value(item, buff_id_name, prop_name)`
  - `base.item_rnd_value(item:item, buff_id:buff_id, prop_name:单位属性)`
  - `base.item_rnd_value(item:item, buff_id_name:buff_id, prop_name:单位属性)`
  - `base.item_set_stack(item, stack)`
  - `base.item_set_stack(item:item, stack:number)`
  - `base.item_skill(item)`
  - `base.item_skill(item:item)`
  - `base.item_stack(item)`
  - `base.item_stack(item:item)`
  - `base.item_table(name, key)`
  - `base.item_unit(item)`
  - `base.item_unit(item:item)`
  - `base.item_unit_get_item(unit)`
  - `base.item_unit_get_item(unit:unit)`

### `@common/base/game/learn_skill`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/level_up`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/player`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `create(ai_info[i].user_id, ai_info[i].type, ai_info[i].behavior_...)`
  - `create(ai_info[j].user_id, ai_info[j].type, ai_info[j].behavior_...)`
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.player_add_attribute(player, state, value)`
  - `base.player_add_attribute(player:player, state:玩家属性, value:number)`
  - `base.player_camera_focus(player, unit)`
  - `base.player_camera_focus(player:player, unit:unit)`
  - `base.player_create_lightning(player, model, source, target)`
  - `base.player_create_lightning(player:player, model:string, source, target)`
  - `base.player_create_unit(player, id, where, face)`
  - `base.player_create_unit(player:player, id:unit_id, where:point, face:angle)`
  - `base.player_create_unit_ai(player, id, where, face, default_ai)`
  - `base.player_create_unit_ai(player:player, id:unit_id, where:point, face:angle, default_ai:是否)`
  - `base.player_create_unit_illusion(player, unit, where, face)`
  - `base.player_create_unit_illusion(player:player, unit:unit, where:point, face:angle)`
  - `base.player_create_unit_illusion_on_scene(player, unit, where, face, scene)`
  - `base.player_create_unit_illusion_on_scene(player:player, unit:unit, where:point, face:angle, scene:场景)`
  - `base.player_create_unit_on_scene(player, id, where, face, scene)`
  - `base.player_create_unit_on_scene(player:player, id:unit_id, where:point, face:angle, scene:场景)`
  - `base.player_event(player:player, name:string, callback:function<trigger>)`
  - `base.player_event_cheat(player:player, name:'玩家事件_输入作弊码', callback:function<trigger,player,string>)`
  - `base.player_event_dispatch(player:player, name:单位事件, ...)`
  - `base.player_event_notify(player:player, name:单位事件, ...)`
  - `base.player_fail_game(player)`
  - `base.player_fail_game(player:player)`
  - `base.player_game_state(player)`
  - `base.player_game_state(player:player)`
  - `base.player_get_attribute(player, state)`
  - `base.player_get_attribute(player:player, state:玩家属性)`
  - `base.player_get_hero(player)`
  - `base.player_get_hero(player:player)`
  - `base.player_get_scene_name(player)`
  - `base.player_get_scene_name(player:player)`
  - `base.player_get_slot_id(player)`
  - `base.player_get_slot_id(player:player)`
  - `base.player_get_team_id(player)`
  - `base.player_get_team_id(player:player)`
  - `base.player_get_user_nick(player)`
  - `base.player_get_user_nick(player:player)`
  - `base.player_jump_scene(player, scene, keep_hero)`
  - `base.player_jump_scene(player:player, scene:场景, keep_hero:是否)`
  - `base.player_leave_reason(player)`
  - `base.player_leave_reason(player:player)`
  - `base.player_local()`
  - `base.player_lock_camera(player)`
  - `base.player_lock_camera(player:player)`
  - `base.player_message_box(player, text)`
  - `base.player_message_box(player:player, text:string)`
  - `base.player_play_music(player, path)`
  - `base.player_play_music(player:player, path:string)`
  - `base.player_play_sound(player, name)`
  - `base.player_play_sound(player:player, name:string)`
  - `base.player_send_message(player, text, type, time)`
  - `base.player_send_message(player:player, text:string,type:消息类型,time:number)`
  - `base.player_set_afk(player)`
  - `base.player_set_afk(player:player)`
  - `base.player_set_attribute_number(player, state, value)`
  - `base.player_set_attribute_number(player:player, state:玩家属性, value:number)`
  - `base.player_set_attribute_string(player, state, value)`
  - `base.player_set_attribute_string(player:player, state:玩家属性, value:string)`
  - `base.player_set_camera(player, camera_id_name, time)`
  - `base.player_set_camera(player:player, camera_id_name:unknown, time:number)`
  - `base.player_set_hero(player, hero)`

### `@common/base/game/reborn`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/scene`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/select_hero`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `click_hero(name)`
  - `click_random_hero()`
  - `hero_list()`
  - `select_hero(name)`
  - `show_hero(name, distance, offset, height)`
  - `show_random()`
  - `show_timer()`

### `@common/base/game/shop`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/template_buff`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 未在语料中反查到直接调用点；按命名语义推测用途。

### `@common/base/game/ui`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- 从调用点反查到的 API 形态（参数以实际调用为准，**推测**）：

  - `add_wait_to_create_ctrl(ui_ctrl)`
  - `bind(player, name)`
  - `bind_unit(poi.hero, bind, 0, 50)`
  - `bind_unit(unit, bind, x, y)`
  - `button(props)`
  - `check_create_new()`
  - `component('_test_bution')`
  - `component('activity_panel')`
  - `component('ai_avatar_component')`
  - `component('ai_avatar_component',appui.ui.basic)`
  - `component('ai_avatar_item',appui.ui.basic)`
  - `component('anim_controller_ui')`
  - `component('anim_info', appui.ui.basic)`
  - `component('anim_info_ui')`
  - `component('anim_libs', appui.ui.basic)`
  - `component('anim_operation')`
  - `component('anim_period_advanced_operation_v2')`
  - `component('anim_period_operation_component')`
  - `component('anim_preview_component', appui.ui.focus)`
  - `component('anim_preview_item', appui.ui.basic)`
  - `component('anim_scene_ui')`
  - `component('anim_title_bar')`
  - `component('anim_track_panel')`
  - `component('anim_tree')`
  - `component('anim_tree_item')`
  - `component('anim_view_component', preview_component)`
  - `component('animation_view_component')`
  - `component('app_title_bar')`
  - `component('appui_basic')`
  - `component('appui_basic_border')`
  - `component('appui_basic_focus', basic)`
  - `component('appui_basic_icon')`
  - `component('appui_basic_loading_icon')`
  - `component('appui_basic_round_corner')`
  - `component('appui_button', basic)`
  - `component('appui_check_options', basic)`
  - `component('appui_collapse', basic)`
  - `component('appui_common_dialog', basic)`
  - `component('appui_form', basic)`
  - `component('appui_icon_slider', basic)`
  - `component('appui_icon_tree', tree)`
  - `component('appui_icon_tree_content', basic)`
  - `component('appui_input', basic)`
  - `component('appui_input_tree', tree)`
  - `component('appui_input_tree_content', basic)`
  - `component('appui_label_group', basic)`
  - `component('appui_label_slider', basic)`
  - `component('appui_menu', basic)`
  - `component('appui_menu_button', basic)`
  - `component('appui_menu_item', basic)`
  - `component('appui_menu_panel', basic)`
  - `component('appui_menu_width')`
  - `component('appui_multi_select', basic)`
  - `component('appui_multi_select_multi_layers', basic)`
  - `component('appui_new_tree', basic)`
  - `component('appui_new_tree_node_component', basic)`
  - `component('appui_new_tree_node_right_icon_component', basic)`
  - `component('appui_number_input', basic)`
  - `component('appui_scene_button', basic)`
  - `component('appui_select', basic)`
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.ui_info()`

### `@common/base/game/unit`

- 归属：script 包（common 库）
- 研判：引擎内嵌模块，运行时扩展 `base.game` 全局表（子模块归属无法从键名精确判定，组级 API 见文档头部）。
- 状态：⚠️ **无源码**（引擎实现 / 未随包分发，以下为推测）
- 引用方：0 个文件
- lua_plus 扁平封装定义（**有源码**，完整签名与 @ui 注解见 lua-plus.md 对应模块）：

  - `base.unit_add_ai(unit, name, data)`
  - `base.unit_add_ai(unit:unit, name:string, data:table)`
  - `base.unit_add_attribute(unit, state, value)`
  - `base.unit_add_attribute(unit:unit, state:单位属性, value:number)`
  - `base.unit_add_attribute_ex(unit, state, value, value_type)`
  - `base.unit_add_attribute_ex(unit:unit, state:单位属性, value:number, value_type:单位数值属性类型)`
  - `base.unit_add_buff(unit, buff_id_name, stack)`
  - `base.unit_add_buff(unit:unit, buff_id_name:buff_id, stack:integer)`
  - `base.unit_add_exp(unit, exp, ignore_fraction)`
  - `base.unit_add_exp(unit:unit, exp:number, ignore_fraction:boolean)`
  - `base.unit_add_height(unit, height)`
  - `base.unit_add_height(unit:unit, height:number)`
  - `base.unit_add_item(unit, item)`
  - `base.unit_add_item(unit:unit, item:item)`
  - `base.unit_add_level(unit, level)`
  - `base.unit_add_level(unit:unit, level:integer)`
  - `base.unit_add_mark(unit, unit_type)`
  - `base.unit_add_mark(unit:unit, unit_type:单位标记)`
  - `base.unit_add_provide_sight(unit, team)`
  - `base.unit_add_provide_sight(unit:unit, team:integer)`
  - `base.unit_add_resource(unit, energy_type, value)`
  - `base.unit_add_resource(unit:unit, energy_type:能量类型, value:number)`
  - `base.unit_add_sight(unit, sight)`
  - `base.unit_add_sight(unit:unit, sight:sight)`
  - `base.unit_add_skill(unit, id, skill_type, slot)`
  - `base.unit_add_skill(unit:unit, id:skill_id, skill_type:技能存在形式, slot:integer)`
  - `base.unit_add_z_speed(unit, speed)`
  - `base.unit_add_z_speed(unit:unit, speed:number)`
  - `base.unit_ai_attack_move_to(unit, line, cycle)`
  - `base.unit_ai_attack_move_to(unit:unit, line:line, cycle:是否)`
  - `base.unit_ai_move_to(unit, line, cycle)`
  - `base.unit_ai_move_to(unit:unit, line:line, cycle:是否)`
  - `base.unit_all_buffs(unit)`
  - `base.unit_all_buffs(unit:unit)`
  - `base.unit_all_items(unit)`
  - `base.unit_all_items(unit:unit)`
  - `base.unit_all_skill(unit)`
  - `base.unit_all_skill(unit:unit)`
  - `base.unit_anim_play(unit, anim, time, time_type, start_offset, blend_time, priority)`
  - `base.unit_anim_play(unit:unit, anim:string, time:number,time_type:integer, start_offset:number, blend_time:integer, priority:integer)`
  - `base.unit_anim_play_bracket(unit, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority, sync)`
  - `base.unit_anim_play_bracket(unit:unit, anim_birth:string, anim_stand:string, anim_death:string, force_one_shot:boolean, kill_on_finish:boolean, priority:integer, sync:boolean)`
  - `base.unit_anim_set_paused_all(unit, paused)`
  - `base.unit_anim_set_paused_all(unit:unit, paused:boolean)`
  - `base.unit_blink(unit, target)`
  - `base.unit_blink(unit:unit, target:point)`
  - `base.unit_can_attack(unit, target)`
  - `base.unit_can_attack(unit:unit, target:unit)`
  - `base.unit_capturer(unit, radius)`
  - `base.unit_capturer(unit:unit, radius:number)`
  - `base.unit_cast(unit, id)`
  - `base.unit_cast(unit:unit, id:skill_id)`
  - `base.unit_cast_on_angel(unit, id, target)`
  - `base.unit_cast_on_angel(unit:unit, id:skill_id, target:number)`
  - `base.unit_cast_on_point(unit, id, point)`
  - `base.unit_cast_on_point(unit:unit, id:skill_id, point:point)`
  - `base.unit_cast_on_unit(unit, id, target)`
  - `base.unit_cast_on_unit(unit:unit, id:skill_id, target:unit)`
  - `base.unit_cast_skill(unit, id)`
  - `base.unit_cast_skill(unit:unit, id:skill)`
