# lua-plus 分组：server_lua_plus 包（触发器 lua+ 服务端 API 层）+ tds_score 云变量族

> draft_v2：以 dump 值树（`test/loaded_modules_server/parsed/fields/_G.json` 的 `_G.base`/`_G.score` 运行时注册面）+ v14 解密源码双证据核验。
> 范围：47 键 = `@lua_plus/*`×42 + `@tds_score/*`×5；另附关联键 `@common/base/tds_score`（common-base 组，但属 tds_score 族核心证据，本节一并给出）。

## 组级结论（新增/修正 draft_v1）

1. **API 面形态**：42 个 `@lua_plus/*` 键值全为 `true`（已加载无表导出）。全部 API 以 `function base.<领域>_<动作>(...)` 扁平函数挂到共享 `_G.base` 表。源码提取有效 `base.*` 定义 **577 个**，与 dump `_G.base`（977 顶层字段、775 个一级函数）逐一比对：**577/577 全部在 dump 注册面中出现**（含 2 个嵌套协议处理器 `base.ui.proto.*`）——lua_plus 层源码与运行时完全对齐。
2. **draft_v1 错误修正**：`attack.lua` 全文、`unit.lua` 的 5 个 `unit_event_*`、`player.lua` 的 2 个 `player_event_*` 均在 `--[[ ]]` 块注释中，**从未注册**（dump 全文 0 次出现）。draft_v1 误列 14 个不可用函数，本版全部剔除并标注。
3. **模块 env 隔离（新发现）**：lua_plus 模块的**裸全局赋值不进真实 `_G`**——obj_check 注册的 32 个 `*_check` 全局检查器、test.lua 的 `debug_bp_confident` 在 `_G` dump 中全部 ABSENT；而对共享表的写入（`base.*`、`string.find_end`、`table.pop_front`）与显式 `_G.get_text =` 均可见。结论：lua_plus chunk 跑在独立 env，游戏侧代码不应依赖 `unit_check` 等裸全局（除非游戏 chunk 与库同 env——待实测）。
4. **stdlib 补丁实锤**：common.lua 的 `table.pop_front/table.getn`、hook.lua 的 `string.find` 包装 + `string.find_end` 均在 dump 的 `string`/`table` 标准库表中出现。
5. **云变量族分层**：`_G.score`（29 函数，引擎侧直接口）← `tds_score.lua`（20 个 `base.score_*` 触发器封装，源码实锤）← `@common/base/tds_score`（36 个 TSTL 参数/数据/ScoreCommitter 类，dump 实锤）。`@tds_score/*` 5 键值全为 `true`，实现不随包分发。
6. `loot_pool.lua` 为 0 字节空文件（draft_v1"未提取到函数"修正为"文件为空"）。

---

### `@lua_plus/base`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\init.lua`）

- 内容为两行：`require 'base.obj_check'` + `require_folder 'base.base_lua_plus'`——即 lua_plus 库服务端入口：先加载 obj_check（注册 32 个全局参数检查器），再整目录加载 base_lua_plus 下全部模块。
- dump 值为 `true`：已加载、无表导出；其 API 面全部经 `_G.base.*` 扁平函数暴露（见各子模块节）。

---

### `@lua_plus/base/obj_check`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/obj_check'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\obj_check.lua`）

- 从 `base.obj_check.*`（common 库 obj_check 模块提供，见 common-base 组）取出 32 个检查器并赋为**裸全局**：`unit_check / committer_check / item_check / skill_check / eff_param_check / player_check / mover_check / circle_check / rect_check / area_check / point_check / line_check / buff_check / unit_group_check / lightning_check / icon_check / trigger_check / snapshot_check / timer_check / any_unit_check / any_skill_check / any_eff_param_check / any_player_check / any_mover_check / id_check / cache_type / event_name_check / time_check / component_type_check / component_check / quest_check / quest_condition_check`。
- **dump 对照**：这 32 个全局在 `_G` dump 中**全部未见**（`unit_check`/`committer_check`/`player_check` 等均 ABSENT），而本包其他模块体内大量调用它们（如 `unit_check(unit)`）——佐证 lua_plus 各模块跑在**独立 chunk env**（裸全局写入不进真实 `_G`），但对共享表（`base`/`string`/`table`）的写入与 `_G.xxx =` 显式写入全局可见。详见 FINDINGS-lua-plus.md。

---

### `@lua_plus/base/base_lua_plus/actor`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/actor'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\actor.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_last_created_actor` | `()` | 触发器最后创建的表现 | 【源码实锤】+【dump 实锤】 |
| `base.create_actor_at` | `(name, point, use_terrain)` | 创建表现 | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_grount_height` | `(actor, height)` | 设置表现地面相对高度 | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_position` | `(actor, point)` | 移动表现 | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_facting` | `(actor, angle)` | 设置表现朝向 | 【源码实锤】+【dump 实锤】 |
| `base.actor_attach_to_unit` | `(actor, host, socket)` | 将表现附着到单位上 | 【源码实锤】+【dump 实锤】 |
| `base.create_actor_on_buff` | `(name, host)` | 为Buff创建表现 | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_actor` | `(host, name)` | Buff上附着的表现 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_actor` | `(host, name)` | 单位上附着的表现 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_actor` | `(host, name)` | 效果节点上附着的表现 | 【源码实锤】+【dump 实锤】 |
| `base.actor_attach_to_actor` | `(actor, host, socket)` | 将表现附着到表现上 | 【源码实锤】+【dump 实锤】 |
| `base.actor_destroy` | `(actor, flag)` | 摧毁表现 | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_asset_model` | `(actor, asset)` | 替换表现的模型资源（仅对模型和粒子表现有效） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_asset_sound` | `(actor, asset)` | 替换表现的音效资源（仅对音效表现有效） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_owner` | `(actor, owner)` | 设置表现所属玩家 | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_shadow` | `(actor, enable)` | 设置表现是否显示影子（仅限模型表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_scale` | `(actor, scale)` | 设置表现缩放（仅限模型和粒子表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_play` | `(actor)` | 播放表现（仅限音效、粒子和材质表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_stop` | `(actor)` | 停止播放表现（仅限音效和粒子表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_pause` | `(actor)` | 暂停表现（仅限音效表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_resume` | `(actor)` | 继续播放被暂停的表现（仅限音效表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_volume` | `(actor, volume)` | 设置表现音量（仅限音效表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_grid_size` | `(actor, size_x, size_y)` | 设置网格物体的网格大小（仅限网格表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_grid_range` | `(actor, start_x, start_y, range_x, range_y)` | 设置网格物体的原点偏移和网格范围（仅限网格表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_grid_state` | `(actor, id_x, id_y, state)` | 设置网格表现中子网格的状态（仅限网格表现） | 【源码实锤】+【dump 实锤】 |
| `base.actor_anim_play` | `(actor, anim, time, time_type, start_offset, blend_time, priority)` | 模型表现播放动画（新API） | 【源码实锤】+【dump 实锤】 |
| `base.actor_anim_set_paused_all` | `(actor, paused)` | 暂停/恢复模型表现的所有动画（新API） | 【源码实锤】+【dump 实锤】 |
| `base.actor_set_time_scale_global` | `(actor, time_scale)` | 设置模型表现相对播放时间倍数（只影响新API的动画） | 【源码实锤】+【dump 实锤】 |
| `base.actor_anim_play_bracket` | `(actor, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority, sync)` | 设置模型表现的bsd动画（新API） | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/advertise`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/advertise'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\advertise.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.adplay_status` | `(player)` | 播放广告 | 【源码实锤】+【dump 实锤】 |
| `base.adplay_recall` | `(player, cb)` | 播放广告 | 【源码实锤】+【dump 实锤】 |
| `base.advertiseReturnParams` | `()` | 广告的观看状态 | 【源码实锤】+【dump 实锤】 |
| `base.advertiseReturnErrCode` | `()` | 广告的响应码 | 【源码实锤】+【dump 实锤】 |
| `base.advertiseReturnErrMsg` | `()` | 广告的响应信息 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/ai`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/ai'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\ai.lua`）

- `line_with_offset` 为文件内 local 辅助（路线偏移），非 API。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_ai` | `(name)` | 获取或创建AI表 | 【源码实锤】+【dump 实锤】 |
| `base.unit_remove_ai` | `(unit)` | 移除单位的所有AI | 【源码实锤】+【dump 实锤】 |
| `base.unit_enable_ai` | `(unit)` | 启用单位的AI | 【源码实锤】+【dump 实锤】 |
| `base.unit_disable_ai` | `(unit)` | 禁用单位的AI | 【源码实锤】+【dump 实锤】 |
| `base.unit_execute_ai` | `(unit)` | 执行单位的AI | 【源码实锤】+【dump 实锤】 |
| `base.unit_ai_attack_move_to` | `(unit, line, cycle)` | 令单位沿指定路线进攻 | 【源码实锤】+【dump 实锤】 |
| `base.unit_ai_move_to` | `(unit, line, cycle)` | 令单位沿指定路线行动 | 【源码实锤】+【dump 实锤】 |
| `line_with_offset` | `(line, offset_x, offset_y)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `base.unit_group_ai_attack_move_to` | `(unit_group, line, cycle)` | 令单位组沿指定路线进攻（保持队形） | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_ai_move_to` | `(unit_group, line, cycle)` | 令单位组沿指定路线行动 （保持队形） | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/ai_attack`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/ai_attack'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\ai_attack.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.ai_attack_add_team_threat` | `(ai_attack, team, threat)` | 为搜敌器添加玩家队伍仇恨值 | 【源码实锤】+【dump 实锤】 |
| `base.ai_attack_add_unit_threat` | `(ai_attack, unit, threat)` | 为搜敌器添加单位仇恨值 | 【源码实锤】+【dump 实锤】 |
| `base.ai_attack_add_type_threat` | `(ai_attack, unit_tag, threat)` | 为搜敌器添加对某种标签的单位仇恨值 | 【源码实锤】+【dump 实锤】 |
| `base.ai_attack_remove` | `(ai_attack)` | 移除搜敌器 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/area`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/area'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\area.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_scene_circle` | `(scene, area_name, present)` | 获取地编中的圆形区域 | 【源码实锤】+【dump 实锤】 |
| `base.get_scene_rect` | `(scene, area_name, present)` | 获取地编中的矩形区域 | 【源码实锤】+【dump 实锤】 |
| `base.get_scene_area` | `(scene, area_type, area_name, present)` | 获取地编区域 | 【源码实锤】+【dump 实锤】 |
| `base.circle_get_point` | `(circle)` | 圆心 | 【源码实锤】+【dump 实锤】 |
| `base.circle_get_range` | `(circle)` | 圆的半径 | 【源码实锤】+【dump 实锤】 |
| `base.circle_random_point` | `(circle)` | 圆内的随机点 | 【源码实锤】+【dump 实锤】 |
| `base.rect_get_point` | `(rect)` | 矩形的中心点 | 【源码实锤】+【dump 实锤】 |
| `base.rect_get_width` | `(rect)` | 矩形的宽度（X轴） | 【源码实锤】+【dump 实锤】 |
| `base.rect_get_height` | `(rect)` | 矩形的高度（Y轴） | 【源码实锤】+【dump 实锤】 |
| `base.rect_random_point` | `(rect)` | 矩形内的随机点 | 【源码实锤】+【dump 实锤】 |
| `base.get_random_point` | `(area)` | 区域内的随机点 | 【源码实锤】+【dump 实锤】 |
| `base.get_area_point` | `(area)` | 区域的中心点 | 【源码实锤】+【dump 实锤】 |
| `base.get_scene_scale_area` | `(scene_name)` | 场景的整个区域 | 【源码实锤】+【dump 实锤】 |
| `base.get_circle_area_unit` | `(circle)` | 圆形区域内的所有单位 | 【源码实锤】+【dump 实锤】 |
| `base.get_circle_area_unit_v2` | `(circle)` |  | 【源码实锤】+【dump 实锤】 |
| `base.get_rect_area_unit` | `(rect)` | 矩形区域内的所有单位 | 【源码实锤】+【dump 实锤】 |
| `base.get_rect_area_unit_v2` | `(rect)` |  | 【源码实锤】+【dump 实锤】 |
| `base.get_area_unit` | `(area)` | 区域内的所有单位 | 【源码实锤】+【dump 实锤】 |
| `base.get_area_unit_v2` | `(area)` |  | 【源码实锤】+【dump 实锤】 |
| `base.get_area_unit_group` | `(area, 过滤条件)` | 区域内的所有单位组成的单位组 | 【源码实锤】+【dump 实锤】 |
| `base.get_area_type_unit` | `(area, unit_id_name)` | 区域内指定Id的单位 | 【源码实锤】+【dump 实锤】 |
| `base.get_area_type_unit_group` | `(area, unit_id_name, 过滤条件)` | 区域内指定Id的单位组成的单位组 | 【源码实锤】+【dump 实锤】 |
| `base.get_area_player_type_unit` | `(area, player, unit_id_name)` | 区域内属于某个玩家的指定Id的单位 | 【源码实锤】+【dump 实锤】 |
| `base.get_area_player_type_unit_group` | `(area, player, unit_id_name, 过滤条件)` | 区域内属于某个玩家的指定Id的单位组成的单位组 | 【源码实锤】+【dump 实锤】 |
| `base.is_point_in_circle` | `(point, circle)` | 点是否在圆形区域内 | 【源码实锤】+【dump 实锤】 |
| `base.is_point_in_rect` | `(point, rect)` | 点是否在矩形区域内 | 【源码实锤】+【dump 实锤】 |
| `base.is_point_in_area` | `(point, area)` | 点是否在区域内 | 【源码实锤】+【dump 实锤】 |
| `base.is_unit_in_area` | `(unit, area)` | 单位是否在区域内 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/attack`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/attack'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\attack.lua`）

- **重要修正（相对 draft_v1）**：本文件全文处于 `--[[ ... ]]` 块注释中（v14 源码第 1 行 `--[[function base.attack_active_cd...` 至末尾 `]]`），7 个 `base.attack_*` 函数**从未注册**。dump 对照实锤：`_G.base` 中 `attack_add_damage` 等 7 名全部 ABSENT（且 dump 全文检索 0 次）。draft_v1 误列为可用 API。
- 注释块内函数（仅存档，运行时不可用）：`attack_active_cd / attack_add_damage / attack_get_cd / attack_get_name / attack_is_common_attack / attack_is_skill / attack_set_cd / attack_stop`。

---

### `@lua_plus/base/base_lua_plus/buff`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/buff'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\buff.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_last_created_buff` | `()` | 触发器最后创建的Buff | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_buff` | `(unit, buff_id_name, stack)` | 为单位添加Buff | 【源码实锤】+【dump 实锤】 |
| `base.buff_set_stack` | `(buff, count)` | 设置Buff层数 | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_pulse` | `(buff)` | Buff周期 | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_remaining` | `(buff)` | Buff剩余时间 | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_stack_all` | `(unit, link)` | 指定Id的buff的总层数（计算所有实例） | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_stack` | `(buff)` | Buff层数 | 【源码实锤】+【dump 实锤】 |
| `base.buff_remove` | `(buff)` | 移除Buff | 【源码实锤】+【dump 实锤】 |
| `base.buff_set_pulse` | `(buff, pulse)` | 设置Buff周期 | 【源码实锤】+【dump 实锤】 |
| `base.buff_set_remaining` | `(buff, remaining)` | 设置Buff剩余时间 | 【源码实锤】+【dump 实锤】 |
| `base.unit_each_buff` | `(unit, id)` | 单位身上所有指定Id的Buff | 【源码实锤】+【dump 实锤】 |
| `base.unit_find_buff` | `(unit, id)` | 单位身上一个指定Id的Buff | 【源码实锤】+【dump 实锤】 |
| `base.unit_has_buff` | `(unit, id)` | 单位是否拥有指定Id的Buff | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_id` | `(buff)` | Buff的Id | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_level` | `(buff)` | Buff的等级 | 【源码实锤】+【dump 实锤】 |
| `base.buff_set_level` | `(buff, level)` | 设置Buff的等级 | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_tracked_units` | `(buff)` | 获取Buff追踪的所有单位 | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_tracked_units_v2` | `(buff)` |  | 【源码实锤】+【dump 实锤】 |
| `base.get_all_buffs_id` | `()` | 获取所有Buff表ID | 【源码实锤】+【dump 实锤】 |
| `base.unit_all_buffs` | `(unit)` | 单位身上所有Buff | 【源码实锤】+【dump 实锤】 |
| `base.buff_get_stack_param` | `(buff)` | Buff的效果节点 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/camera`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/camera'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\camera.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.player_lock_camera` | `(player)` | 锁定镜头 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_camera` | `(player, camera_id_name, time)` | 设置玩家镜头 | 【源码实锤】+【dump 实锤】 |
| `base.player_shake_camera` | `(player, type, frequency, amplitude, time)` | 震动镜头 | 【源码实锤】+【dump 实锤】 |
| `base.player_unlock_camera` | `(player)` | 解锁镜头 | 【源码实锤】+【dump 实锤】 |
| `base.player_camera_focus` | `(player, unit)` | 使镜头跟随单位 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/capturer`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/capturer'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\capturer.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.capturer_remove` | `(capturer)` | 移除弹道捕获器 | 【源码实锤】+【dump 实锤】 |
| `base.unit_capturer` | `(unit, radius)` | 创建弹道捕获器 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/cheat`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/cheat'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\cheat.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.gm_god_is_enable` | `(player)` | 玩家是否开启god作弊模式 | 【源码实锤】+【dump 实锤】 |
| `base.gm_cooldown_is_enable` | `(player)` | 玩家是否开启cooldown作弊模式 | 【源码实锤】+【dump 实锤】 |
| `base.gm_energy_is_enable` | `(player)` | 玩家是否开启energy作弊模式 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/common`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/common'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\common.lua`）

- 对标准库 `table` 打补丁：`table.pop_front(t)`（移除首元素）、`table.getn(t, index)`（取 `t[index]`，语义非标准 getn）。文件头还有被行注释掉的 `table.push_back/pop_back`。
- dump 对照：`table.pop_front`、`table.getn` 均在 `_G.table` 中出现【dump 实锤】（同一共享表被成功补丁）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `table.pop_front` | `(t)` |  | 【源码实锤】+【dump 实锤】 |
| `table.getn` | `(t, index)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/damage`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/damage'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\damage.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.damage_get_damage` | `(damage)` | 伤害实例的原始伤害值 | 【源码实锤】+【dump 实锤】 |
| `base.damage_get_current_damage` | `(damage)` | 伤害实例的当前伤害值 | 【源码实锤】+【dump 实锤】 |
| `base.damage_get_type` | `(damage)` | 伤害实例的伤害类型 | 【源码实锤】+【dump 实锤】 |
| `base.damage_set_current_damage` | `(damage, amount)` | 修改伤害实例的当前伤害值 | 【源码实锤】+【dump 实锤】 |
| `base.do_trigger_damage` | `(source, target, amount, damage_type)` | 令单位对单位造成伤害 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/eff_param`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/eff_param'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\eff_param.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.eff_param_origin_point` | `(eff_param)` | 效果节点的原始施法点 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_link` | `(eff_param)` | 效果节点的Id | 【源码实锤】+【dump 实锤】 |
| `base.unit_execute_effect_on_unit` | `(unit, target, link)` | 执行效果(对单位) | 【源码实锤】+【dump 实锤】 |
| `base.unit_execute_effect_on_point` | `(unit, target, link)` | 执行效果(对点) | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_missle_detach` | `(eff_param)` | 解绑效果节点的弹道 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_missle_get` | `(eff_param)` | 效果节点挂载的弹道单位 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_missle_range` | `(eff_param)` | 效果节点挂载的弹道的射程 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_set_damage_modifiers` | `(eff_param, unit)` | 设置效果节点的施法加成属性来源 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_source_item` | `(eff_param)` | 效果节点的引发物品 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_responsing_param` | `(eff_param)` | 效果节点的引发响应的效果节点 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_responsing_skill` | `(eff_param)` | 效果节点的引发响应的技能 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_responsing_damage` | `(eff_param)` | 效果节点的引发响应的伤害实例 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_caster` | `(eff_param)` | 效果节点的施法者 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_main_target_point` | `(eff_param)` | 效果节点的效果树主目标（点） | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_main_target_unit` | `(eff_param)` | 效果节点的效果树主目标（单位） | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_target_point` | `(eff_param)` | 效果节点的目标点 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_target_unit` | `(eff_param)` | 效果节点的目标单位 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_has_target` | `(eff_param)` | 效果节点是否拥有目标单位 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_root` | `(eff_param)` | 效果节点的效果树根节点 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_parent` | `(eff_param)` | 效果节点的效果树父节点 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_by_name` | `(eff_param, effect_id_name)` | 效果节点的指定类型祖先节点 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_level` | `(eff_param)` | 效果节点的技能等级快照 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_skill` | `(eff_param)` | 效果节点的引发技能 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_cast` | `(eff_param)` | 效果节点的施法实例 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_var_unit` | `(eff_param, key)` | 效果节点保存的单位变量 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_var_point` | `(eff_param, key)` | 效果节点保存的点变量 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_set_var_unit` | `(eff_param, key, value)` | 设置效果节点的单位变量 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_set_var_point` | `(eff_param, key, value)` | 设置效果节点的点变量 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_userdata` | `(eff_param, key)` | 效果节点的效果树自定义值 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_cache` | `(eff_param)` | 效果节点的类型数据 | 【源码实锤】+【dump 实锤】 |
| `base.eff_param_get_node_in_module` | `(eff_param, name)` | 效果节点的兄弟数据 | 【源码实锤】+【dump 实锤】 |
| `base.validator_unit_filter` | `(eff_param, unit, filters)` | 单位过滤 | 【源码实锤】+【dump 实锤】 |
| `base.validator_unit_filter_new` | `(eff_param, unit, filters)` | 单位过滤 | 【源码实锤】+【dump 实锤】 |
| `base.validator_condition` | `(condition)` | 满足触发器条件 | 【源码实锤】+【dump 实锤】 |
| `base.validator_and` | `(code1, code2)` | 验证器“与”操作 | 【源码实锤】+【dump 实锤】 |
| `base.validator_or` | `(code1, code2)` | 验证器“或”操作 | 【源码实锤】+【dump 实锤】 |
| `base.validator_not` | `(code1)` | 验证器“非”操作 | 【源码实锤】+【dump 实锤】 |
| `base.validator_unit_has_buff` | `(eff_param, unit, buff_id_name)` | 效果节点的目标单位是否拥有Buff | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/game`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/game'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\game.lua`）

- `base.game_ui_message(message_name, data)` 是服务端→客户端自定义消息通道（配 client 侧 `base.game:event(...)` 监听）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.game_exit` | `(player, show_confirm)` | 令指定玩家退出游戏 | 【源码实锤】+【dump 实锤】 |
| `base.player_jump_scene` | `(player, scene, keep_hero)` | 跳转玩家场景 | 【源码实锤】+【dump 实锤】 |
| `base.game_ui_message` | `(message_name, data)` | 向客户端发消息 | 【源码实锤】+【dump 实锤】 |
| `base.player_win_game` | `(player)` | 玩家赢得游戏 | 【源码实锤】+【dump 实锤】 |
| `base.player_fail_game` | `(player)` | 玩家游戏失败 | 【源码实锤】+【dump 实锤】 |
| `base.object_store_value` | `(object, key, value)` | 在对象上保存任意值 | 【源码实锤】+【dump 实锤】 |
| `base.object_restore_value` | `(object, key)` | 对象上保存的任意值 | 【源码实锤】+【dump 实锤】 |
| `base.pause_game` | `()` | 暂停游戏 | 【源码实锤】+【dump 实锤】 |
| `base.pause_game_time` | `(sec)` | 暂停游戏一段时间 | 【源码实锤】+【dump 实锤】 |
| `base.unpause_game` | `()` | 取消暂停游戏 | 【源码实锤】+【dump 实锤】 |
| `base.switch_fov_mode` | `(number, scene)` | 切换迷雾模式 | 【源码实锤】+【dump 实锤】 |
| `base.get_gamemode_key` | `()` | 获取游戏模式 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/gamechat`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/gamechat'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\gamechat.lua`）

- 局内聊天走**云变量频道消息**：`base.gamechat_send_message` 内部调 `score.publish_message('@lib_gamechat_'..session_id, {src_user_name, text, time})`——这是 `_G.score` 频道消息 API 的官方用途实锤（跨端聊天 = 云频道 pub/sub）。
- 同时注册协议处理器 `base.ui.proto.gamechatclient_send_message`（嵌套路径，dump 中 IN）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.gamechat_send_message` | `(text, user)` | 输出信息到聊天窗口 | 【源码实锤】+【dump 实锤】 |
| `base.ui.proto.gamechatclient_send_message` | `(_, msg)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/global_variable`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/global_variable'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\global_variable.lua`）

- 设置触发器"任意对象"哨兵：`base.any_unit / any_unit_id / any_player / any_skill / any_eff_param / any_mover / any_item` 全部 = `base.game`（引擎 game 对象兼任通配符）；`base.ai_searcher_default_range = -1`。仅 `base.table_new` 一个函数（返回 `{}`）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.table_new` | `()` | 空表 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/hook`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/hook'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\hook.lua`）

- 猴子补丁 `string.find`：包装原实现，把匹配结束位置记入 `string.find_end_pos`；新增 `string.find_end()` 读取该值。
- dump 对照：`string.find`（已替换）与 `string.find_end` 均在 `_G.string` 中出现【dump 实锤】。注意 `string.find_end_pos` 是裸字段，未单独验证。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `string.find` | `(...)` |  | 【源码实锤】+【dump 实锤】 |
| `string.find_end` | `()` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/item`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/item'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\item.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.create_item_on_point` | `(id, target)` | 创建物品 | 【源码实锤】+【dump 实锤】 |
| `base.create_item_on_unit` | `(id, target)` | 为单位创建物品 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_item` | `(unit, item)` | 将物品添加给单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_has_item` | `(unit, id)` | 单位是否持有指定ID的物品 | 【源码实锤】+【dump 实锤】 |
| `base.unit_all_items` | `(unit)` | 获取单位身上所有物品 | 【源码实锤】+【dump 实锤】 |
| `base.item_add_extra_mod` | `(item, buff_id_name, IsEquip)` | 为物品添加额外增益词条 | 【源码实锤】+【dump 实锤】 |
| `base.remove_extra_mod` | `(item, buff_id_name, IsEquip)` | 为物品移除额外增益词条 | 【源码实锤】+【dump 实锤】 |
| `base.item_generate_rand_mod` | `(item)` | 为物品的词条生成随机结果（每个词条只会生成一次） | 【源码实锤】+【dump 实锤】 |
| `base.get_last_created_item` | `()` | 触发器最后创建的物品 | 【源码实锤】+【dump 实锤】 |
| `base.item_rnd_value` | `(item, buff_id_name, prop_name)` | 物品的词条随机结果 | 【源码实锤】+【dump 实锤】 |
| `base.item_set_stack` | `(item, stack)` | 设置物品堆叠层数 | 【源码实锤】+【dump 实锤】 |
| `base.item_stack` | `(item)` | 物品的堆叠层数 | 【源码实锤】+【dump 实锤】 |
| `base.item_unit` | `(item)` | 物品在地上时的单位 | 【源码实锤】+【dump 实锤】 |
| `base.item_unit_get_item` | `(unit)` | 物品单位对应的物品对象 | 【源码实锤】+【dump 实锤】 |
| `base.item_blink` | `(item, target)` | 移动物品 | 【源码实锤】+【dump 实锤】 |
| `base.item_get_holder` | `(item)` | 物品的持有者单位 | 【源码实锤】+【dump 实锤】 |
| `base.item_get_name` | `(item)` | 物品的Id | 【源码实锤】+【dump 实锤】 |
| `base.item_grant_tag` | `(item)` | 物品被赋予的标签 | 【源码实锤】+【dump 实锤】 |
| `base.item_get_owner` | `(item)` | 物品的持有者玩家 | 【源码实锤】+【dump 实锤】 |
| `base.item_remove` | `(item)` | 移除物品 | 【源码实锤】+【dump 实锤】 |
| `base.drop_item` | `(item)` | 卸下物品 | 【源码实锤】+【dump 实锤】 |
| `base.item_skill` | `(item)` | 物品附加的技能 | 【源码实锤】+【dump 实锤】 |
| `base.item_get_equip_state` | `(item)` | 物品的装备状态 | 【源码实锤】+【dump 实锤】 |
| `base.get_inventory_items` | `(unit, index)` | 获取指定编号物品栏的全部物品 | 【源码实锤】+【dump 实锤】 |
| `base.give_item_to_inventory` | `(item, unit, index)` | 将物品添加到指定单位的指定物品栏 | 【源码实锤】+【dump 实锤】 |
| `base.get_item_info` | `(item)` | 将物品实例转化为数据信息 | 【源码实锤】+【dump 实锤】 |
| `base.load_item_from_info` | `(info, unit)` | 将数据信息转化为物品实例 | 【源码实锤】+【dump 实锤】 |
| `base.get_obj_items` | `()` | 数编所有物品 | 【源码实锤】+【dump 实锤】 |
| `base.load_score_item_to_unit` | `(unit, success_callback, error_callback)` | 为单位还原云变量物品 | 【源码实锤】+【dump 实锤】 |
| `base.save_score_item_to_unit` | `(unit, success_callback, error_callback)` | 为单位保存全部云变量物品 | 【源码实锤】+【dump 实锤】 |
| `base.get_player_score_item_list` | `(player)` | 获取玩家场上所有的云变量物品 | 【源码实锤】+【dump 实锤】 |
| `base.bind_items_to_user` | `(items, player, success_callback, error_callback)` | 将一组云变量物品绑定给玩家 | 【源码实锤】+【dump 实锤】 |
| `base.unbind_items_to_user` | `(player, success_callback, error_callback)` | 将一组云变量物品解除绑定 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/lightning`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/lightning'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\lightning.lua`）

- 源码注释"被代码中的定义覆盖"：`player_create_lightning`/`unit_create_lightning` 是触发器层对引擎同名底层能力的覆盖实现（创建后写 `base.last_created_lightning`）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.lightning_remove` | `(lightning)` | 移除闪电 | 【源码实锤】+【dump 实锤】 |
| `base.player_create_lightning` | `(player, model, source, target)` | 创建闪电给单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_create_lightning` | `(unit, model, source, target)` | 创建闪电给玩家 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/localization`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/localization'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\localization.lua`）

- 全文 3 行：`_G.get_text = function(id) return '@'..id..'@' end`——服务端 get_text 假实现（文本本地化只客户端有效）。
- dump 对照：`get_text` 在 `_G` 顶层函数中出现【dump 实锤】（显式 `_G.` 写入不受 env 隔离影响）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `_G.get_text` | `(id)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/loot_pool`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/loot_pool'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\loot_pool.lua`）

- **空文件（0 字节）**——模块键存在仅因 require_folder 扫到该文件并执行（无任何语句）。draft_v1 的"未提取到函数"应修正为"文件为空"。

（空文件，无函数）

---

### `@lua_plus/base/base_lua_plus/minimap`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/minimap'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\minimap.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.create_icon` | `(player, name, point)` | 创建小地图图标 | 【源码实锤】+【dump 实锤】 |
| `base.icon_set_sync` | `(icon, sync)` | 设置小地图图标同步方式 | 【源码实锤】+【dump 实锤】 |
| `base.icon_hide` | `(icon)` | 隐藏小地图图标 | 【源码实锤】+【dump 实锤】 |
| `base.icon_hide_team` | `(icon, team)` | 隐藏小地图图标（立即对一个队伍不可见） | 【源码实锤】+【dump 实锤】 |
| `base.icon_show` | `(icon)` | 显示小地图图标 | 【源码实锤】+【dump 实锤】 |
| `base.icon_set_time` | `(icon, time)` | 设置小地图图标持续时间 | 【源码实锤】+【dump 实锤】 |
| `base.minimap_signal` | `(player, name, point)` | 发送小地图信号 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/mover`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/mover'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\mover.lua`）

- `follow_or_move_to` 为文件内 local 辅助（直线/跟随复用逻辑），非 API。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_last_created_mover` | `()` | 触发器最后创建的移动器 | 【源码实锤】+【dump 实锤】 |
| `base.skill_mover_line` | `(mover, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` | 使单位向目标点进行直线运动 | 【源码实锤】+【dump 实锤】 |
| `follow_or_move_to` | `(moving_unit, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `base.skill_mover_target` | `(moving_unit, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` | 使单位向目标单位进行追踪运动 | 【源码实锤】+【dump 实锤】 |
| `base.mover_batch_update` | `(mover)` | 批量更新移动器 | 【源码实锤】+【dump 实锤】 |
| `base.mover_remove` | `(mover)` | 移除移动器 | 【源码实锤】+【dump 实锤】 |
| `base.unit_each_mover` | `(unit)` | 单位身上的所有移动器 | 【源码实锤】+【dump 实锤】 |
| `base.unit_follow` | `(mover, target, speed, mover_id_name, on_block, on_finish, on_hit, on_remove)` | 使单位跟随单位（忽视寻路） | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/player`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/player'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\player.lua`）

- **修正（相对 draft_v1）**：`base.player_event_dispatch / player_event_notify` 在源码中处于 `--[[ ... ]]` 块注释内（"TODO:等 jj 设计"），dump 中 ABSENT，draft_v1 误列。下表为 24 个有效注册函数。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.player_add_attribute` | `(player, state, value)` | 玩家增加属性 | 【源码实锤】+【dump 实锤】 |
| `base.get_player_controller` | `(player)` | 玩家的控制者类型 | 【源码实锤】+【dump 实锤】 |
| `base.player_game_state` | `(player)` | 玩家的游戏状态 | 【源码实锤】+【dump 实锤】 |
| `base.player_get_attribute` | `(player, state)` | 玩家的属性 | 【源码实锤】+【dump 实锤】 |
| `base.player_get_hero` | `(player)` | 玩家的主控单位 | 【源码实锤】+【dump 实锤】 |
| `base.player_get_slot_id` | `(player)` | 玩家的槽位Id | 【源码实锤】+【dump 实锤】 |
| `base.player_get_team_id` | `(player)` | 玩家的队伍Id | 【源码实锤】+【dump 实锤】 |
| `base.get_player_input_rocker` | `(player)` | 玩家的摇杆方向 | 【源码实锤】+【dump 实锤】 |
| `base.is_player_abort` | `(player)` | 玩家是否已放弃游戏 | 【源码实锤】+【dump 实锤】 |
| `base.kick_player` | `(player, backend, frontend)` | 将玩家踢出游戏 | 【源码实锤】+【dump 实锤】 |
| `base.player_leave_reason` | `(player)` | 玩家的退出记录 | 【源码实锤】+【dump 实锤】 |
| `base.player_send_message` | `(player, text, type, time)` | 向玩家显示消息 | 【源码实锤】+【dump 实锤】 |
| `base.player_message_box` | `(player, text)` | 向玩家显示弹框消息 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_attribute_number` | `(player, state, value)` | 设置玩家数值型属性 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_attribute_string` | `(player, state, value)` | 设置玩家字符型属性 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_afk` | `(player)` | 将玩家设置为挂机状态 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_hero` | `(player, hero)` | 设置玩家主控单位 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_team_id` | `(player, id)` | 设置玩家队伍Id | 【源码实锤】+【dump 实锤】 |
| `base.get_player_user_agent` | `(player)` | 玩家的用户客户端 | 【源码实锤】+【dump 实锤】 |
| `base.player_user_id` | `(player)` | 玩家的虚拟用户Id | 【源码实锤】+【dump 实锤】 |
| `base.player_get_scene_name` | `(player)` | 玩家所在场景的名称 | 【源码实锤】+【dump 实锤】 |
| `base.player_get_user_nick` | `(player)` | 玩家的昵称 | 【源码实锤】+【dump 实锤】 |
| `base.get_each_player` | `(type)` | 获取指定玩家类型所有玩家 | 【源码实锤】+【dump 实锤】 |
| `base.player_set_hero_skill_sync_type` | `(player, sync)` | 设置玩家主控单位技能同步方式 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/point`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/point'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\point.lua`）

- `base.point_is_visible_to_player` 重复定义两次：`(point, dest, scene_name)` 与 `(point, dest)`——后定义覆盖前者，运行时生效的是 2 参版。`point_is_visible_to_unit`/`point_is_block` 同理存在带 `scene_name` 的旧版与 `2` 后缀的新版并存。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.point_angle` | `(point, target)` | 两点连线的角度 | 【源码实锤】+【dump 实锤】 |
| `base.point_copy` | `(point)` | 复制点 | 【源码实锤】+【dump 实锤】 |
| `base.point_distance` | `(point, target)` | 两点间的距离 | 【源码实锤】+【dump 实锤】 |
| `base.point_get_x` | `(point)` | 点的X坐标 | 【源码实锤】+【dump 实锤】 |
| `base.point_get_y` | `(point)` | 点的Y坐标 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_block` | `(point, scene_name, prevent_bits, required_bits)` | 点的某类碰撞类型检测 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_block2` | `(point, prevent_bits, required_bits)` | 点的某类碰撞类型检测 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_block_all` | `(point, scene_name)` | 点的碰撞类型检测 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_block_all2` | `(point)` | 点的碰撞类型检测 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_visible_to_unit` | `(point, dest, scene_name)` | 点对单位的可见性 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_visible_to_unit2` | `(point, dest)` | 点对单位的可见性 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_visible_to_player` | `(point, dest, scene_name)` | 点对玩家的可见性 | 【源码实锤】+【dump 实锤】 |
| `base.point_is_visible_to_player` | `(point, dest)` | 点对玩家的可见性 | 【源码实锤】+【dump 实锤】 |
| `base.point_move` | `(point, angle, distance)` | 点的极坐标偏移 | 【源码实锤】+【dump 实锤】 |
| `base.get_scene_point` | `(scene, area_name, present)` | 获取地编点 | 【源码实锤】+【dump 实锤】 |
| `base.get_scene_line` | `(scene, area_name, present)` | 获取地编线 | 【源码实锤】+【dump 实锤】 |
| `base.get_point_scene` | `(point)` | 点的所属场景 | 【源码实锤】+【dump 实锤】 |
| `base.line_get` | `(line, index)` | 线上的点 | 【源码实锤】+【dump 实锤】 |
| `base.pathing_way_points` | `(st, ed)` | 两点间的通行路径 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/quest`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/quest'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\quest.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.unit_receive_quest` | `(unit, id)` | 为单位创建任务 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_quests` | `(unit)` | 单位的任务列表 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_quest` | `(unit, id)` | 单位的指定编号的任务 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_quest_conditions` | `(unit)` | 单位的任务目标列表 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_quest_condition` | `(unit, id)` | 单位的指定编号的任务目标 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_link` | `(quest)` | 任务的Id | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_id` | `(quest)` | 任务的编号 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_owner` | `(quest)` | 任务的所属单位 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_conditions` | `(quest)` | 任务的任务目标列表 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_active` | `(quest)` | 任务的激活状态 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_complete` | `(quest)` | 任务的完成状态 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_progress` | `(quest)` | 任务的进度 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_progress_total` | `(quest)` | 任务的总进度 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_can_submit` | `(quest)` | 任务是否可提交 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_link` | `(quest_condition)` | 任务目标的Id | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_id` | `(quest_condition)` | 任务目标的编号 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_quest` | `(quest_condition)` | 任务目标的所属任务 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_owner` | `(quest_condition)` | 任务目标的所属单位 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_active` | `(quest_condition)` | 任务目标的激活状态 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_complete` | `(quest_condition)` | 任务目标的完成状态 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_progress` | `(quest_condition)` | 任务目标的进度 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_progress_total` | `(quest_condition)` | 任务目标的总进度 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_get_can_submit` | `(quest_condition)` | 任务目标是否可提交 | 【源码实锤】+【dump 实锤】 |
| `base.quest_reset` | `(quest)` | 重置任务 | 【源码实锤】+【dump 实锤】 |
| `base.quest_activate` | `(quest)` | 激活任务 | 【源码实锤】+【dump 实锤】 |
| `base.quest_deactivate` | `(quest)` | 取消激活任务 | 【源码实锤】+【dump 实锤】 |
| `base.quest_submit` | `(quest)` | 提交任务 | 【源码实锤】+【dump 实锤】 |
| `base.quest_get_current_condition` | `(quest)` | 任务的当前任务目标 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_set_progress` | `(quest_condition, progress)` | 设置任务目标进度 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_add_progress` | `(quest_condition, progress)` | 增加任务目标进度 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_set_active_state` | `(quest_condition, state)` | 设置任务目标的激活状态 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_set_complete_state` | `(quest_condition, state)` | 设置任务目标的完成状态 | 【源码实锤】+【dump 实锤】 |
| `base.quest_condition_submit` | `(quest_condition)` | 提交任务目标 | 【源码实锤】+【dump 实锤】 |
| `base.load_score_quest_to_unit` | `(unit, success_callback, error_callback)` | 为单位还原云变量物品 | 【源码实锤】+【dump 实锤】 |
| `base.save_score_quest_to_unit` | `(unit, success_callback, error_callback)` | 为单位保存全部云变量物品 | 【源码实锤】+【dump 实锤】 |
| `base.get_player_score_quest_list` | `(player)` | 获取玩家场上所有的云变量物品 | 【源码实锤】+【dump 实锤】 |
| `base.bind_quests_to_user` | `(quests, player, success_callback, error_callback)` | 将一组云变量物品绑定给玩家 | 【源码实锤】+【dump 实锤】 |
| `base.unbind_quests_to_user` | `(player, success_callback, error_callback)` | 将一组云变量物品解除绑定 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/simple_ui`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/simple_ui'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\simple_ui.lua`）

- 服务端简易 UI 组件（按钮/图片/文本）协议封装：组件按 id 管理，操作经 proto 消息下发客户端。`base.ui.proto.component_event` 为客户端事件上行处理器（嵌套路径，dump 中 IN）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_last_created_component` | `()` | 触发器最后创建的简易控件 | 【源码实锤】+【dump 实锤】 |
| `base.component_position` | `(x, y)` | 简易控件位置 | 【源码实锤】+【dump 实锤】 |
| `base.component_size` | `(width, height)` | 简易控件尺寸 | 【源码实锤】+【dump 实锤】 |
| `base.component_color` | `(r, g, b)` | 简易控件染色 | 【源码实锤】+【dump 实锤】 |
| `base.create_component_button` | `(position, size, text, visiblity, event_label)` | 创建按钮简易控件 | 【源码实锤】+【dump 实锤】 |
| `base.create_component_picture` | `(position, size, image, visiblity, event_label)` | 创建图片简易控件 | 【源码实锤】+【dump 实锤】 |
| `base.create_component_text` | `(position, size, text, font_size, visiblity, event_label)` | 创建文本简易控件 | 【源码实锤】+【dump 实锤】 |
| `base.destroy_component` | `(component_id)` | 移除简易控件 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_position` | `(player, component_id, position)` | 设置简易控件位置 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_size` | `(player, component_id, size)` | 设置简易控件尺寸 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_visiblity` | `(component_id, player, visiblity)` | 设置简易控件可见性 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_color` | `(player, component_id, color)` | 设置简易控件染色 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_can_be_clicked` | `(player, component_id, can_be_clicked)` | 设置简易控件是否可被点击 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_text` | `(player, component_id, text)` | 设置简易控件文本 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_font_size` | `(player, component_id, font_size)` | 设置简易控件字号 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_image` | `(player, component_id, image)` | 设置简易控件图片 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_opacity` | `(player, component_id, opacity)` | 设置简易控件图片不透明度 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_zoom_type` | `(player, component_id, zoom_type)` | 设置简易控件图片缩放方式 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_auto_line_feed` | `(player, component_id, auto_line_feed)` | 设置简易控件文本自动换行 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_text_align` | `(player, component_id, align)` | 设置简易控件文本横向对齐方式 | 【源码实锤】+【dump 实锤】 |
| `base.set_component_text_vertical_align` | `(player, component_id, vertical_align)` | 设置简易控件文本纵向对齐方式 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_position` | `(player, component_id)` | 获得简易控件位置 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_size` | `(player, component_id)` | 获得简易控件尺寸 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_visiblity` | `(component_id, player)` | 简易控件是否可见 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_color` | `(player, component_id)` | 获得简易控件染色参数 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_can_be_clicked` | `(player, component_id)` | 获得简易控件是否可被点击 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_text` | `(player, component_id)` | 获得简易控件文本 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_font_size` | `(player, component_id)` | 获得简易控件字号 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_image` | `(player, component_id)` | 获得简易控件图片 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_opacity` | `(player, component_id)` | 获得简易控件不透明度 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_zoom_type` | `(player, component_id)` | 获得简易控件图片缩放方式 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_auto_line_feed` | `(player, component_id)` | 获得简易控件文本是否自动换行 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_text_align` | `(player, component_id)` | 获得简易控件文本横向对齐方式 | 【源码实锤】+【dump 实锤】 |
| `base.get_component_text_vertical_align` | `(player, component_id)` | 获得简易控件文本纵向对齐方式 | 【源码实锤】+【dump 实锤】 |
| `base.ui.proto.component_event` | `(player, msg)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/skill`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/skill'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\skill.lua`）

- `dummy` 为空占位 local 函数，非 API。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.get_last_created_skill` | `()` | 触发器最后添加的技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_skill` | `(unit, id, skill_type, slot)` | 为单位添加技能 | 【源码实锤】+【dump 实锤】 |
| `base.add_skill_to_slot` | `(unit, id, slot)` | 为单位添加技能（并指定位置） | 【源码实锤】+【dump 实锤】 |
| `base.add_skill` | `(unit, id, slot)` | 为单位添加技能（并指定位置） | 【源码实锤】+【dump 实锤】 |
| `base.add_skill_simple` | `(unit, id)` | 为单位添加技能（不指定位置） | 【源码实锤】+【dump 实锤】 |
| `base.skill_active_cd` | `(skill, max_cd, ignore_cooldown_reduce)` | 激活技能冷却 | 【源码实锤】+【dump 实锤】 |
| `base.skill_active_custom_cd` | `(skill, max_cd, cd)` | 激活技能自定义冷却 | 【源码实锤】+【dump 实锤】 |
| `base.skill_add_level` | `(skill, level)` | 增加技能等级 | 【源码实锤】+【dump 实锤】 |
| `base.skill_add_stack` | `(skill, stack)` | 增加技能层数 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_attribute` | `(skill, attr)` | 技能的自定义属性值 | 【源码实锤】+【dump 实锤】 |
| `base.skill_set_attribute` | `(skill, attr, val)` | 设置技能的自定义属性 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_stage` | `(skill)` | 技能的当前阶段 | 【源码实锤】+【dump 实锤】 |
| `base.skill_stage_finish` | `(skill)` | 完成技能当前阶段 | 【源码实锤】+【dump 实锤】 |
| `base.skill_disable` | `(skill)` | 禁用技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_enable` | `(skill)` | 启用技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_enable_hidden` | `(skill)` | 隐藏技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_disable_hidden` | `(skill)` | 取消隐藏技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_cd` | `(skill)` | 技能的冷却时间 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_level` | `(skill)` | 技能的等级 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_name` | `(skill)` | 技能的Id | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_slot_id` | `(skill)` | 技能的槽位编号 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_owner` | `(skill)` | 技能的拥有者 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_last_target_unit` | `(skill)` | 技能的上次施法的目标单位 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_target_unit` | `(skill)` | 技能的目标单位 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_target_point` | `(skill)` | 技能的目标点 | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_target_angle` | `(skill)` | 技能的目标角度（向量技能） | 【源码实锤】+【dump 实锤】 |
| `base.skill_get_type` | `(skill)` | 技能的存在形式 | 【源码实锤】+【dump 实锤】 |
| `base.skill_is_cast` | `(skill)` | 技能是否为施法实例 | 【源码实锤】+【dump 实锤】 |
| `base.skill_is_enable` | `(skill)` | 技能是否被启用 | 【源码实锤】+【dump 实锤】 |
| `base.skill_is_skill` | `(skill)` | 技能是否是非普攻技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_notify_damage` | `(skill, damage)` | 通知伤害 | 【源码实锤】+【dump 实锤】 |
| `base.skill_reload` | `(skill)` | 重新加载技能脚本 | 【源码实锤】+【dump 实锤】 |
| `base.skill_remove` | `(skill)` | 移除技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_set` | `(skill, key, value)` | 设置技能自定义数据 | 【源码实锤】+【dump 实锤】 |
| `base.skill_set_animation` | `(skill, animation)` | 设置施法动画 | 【源码实锤】+【dump 实锤】 |
| `base.skill_set_cd` | `(skill, cd, force)` | 设置技能当前剩余冷却 | 【源码实锤】+【dump 实锤】 |
| `base.skill_set_level` | `(skill, level)` | 设置技能等级 | 【源码实锤】+【dump 实锤】 |
| `base.skill_set_option` | `(skill, key, value)` | 设置技能属性 | 【源码实锤】+【dump 实锤】 |
| `dummy` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `base.skill_simple_cast` | `(skill)` | 施放技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_stop` | `(skill)` | 停止施法 | 【源码实锤】+【dump 实锤】 |
| `base.unit_blink` | `(unit, target)` | 瞬移单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_can_attack` | `(unit, target)` | 单位能否攻击目标 | 【源码实锤】+【dump 实锤】 |
| `base.same_skill` | `(skill_a, skill_b)` | 判断两个施法实例是否同源 | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_smart` | `(unit, id)` | 命令单位尝试智能施法一个技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast` | `(unit, id)` | 命令单位施放立即技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_on_angel` | `(unit, id, target)` | 命令单位施放向量技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_on_unit` | `(unit, id, target)` | 命令单位对单位施放技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_on_point` | `(unit, id, point)` | 命令单位对点施放技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_skill` | `(unit, id)` | 命令单位施放立即技能（指定技能） | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_skill_on_unit` | `(unit, id, target)` | 命令单位对单位施放技能（指定技能） | 【源码实锤】+【dump 实锤】 |
| `base.unit_cast_skill_on_point` | `(unit, id, point)` | 命令单位对点施放技能（指定技能） | 【源码实锤】+【dump 实锤】 |
| `base.unit_clean_command` | `(unit)` | 清空单位命令队列 | 【源码实锤】+【dump 实锤】 |
| `base.unit_current_skill` | `(unit)` | 单位正在施放的技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_each_skill` | `(unit, skill_type)` | 单位身上所有指定存在形式的技能数组 | 【源码实锤】+【dump 实锤】 |
| `base.unit_find_skill_by_name` | `(unit, id, include_level_zero)` | 单位身上一个指定Id的技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_find_skill_by_slot` | `(unit, slot)` | 单位身上指定槽位的技能 | 【源码实锤】+【dump 实锤】 |
| `base.get_all_skills_id` | `()` | 获取所有技能ID | 【源码实锤】+【dump 实锤】 |
| `base.unit_all_skill` | `(unit)` | 单位的所有技能 | 【源码实锤】+【dump 实锤】 |
| `base.skill_can_learn` | `(skill)` | 技能是否可供学习 | 【源码实锤】+【dump 实锤】 |
| `base.skill_learn` | `(skill)` | 学习技能 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/snapshot`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/snapshot'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\snapshot.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.snapshot_get_point` | `(snapshot)` | 快照的坐标 | 【源码实锤】+【dump 实锤】 |
| `base.snapshot_get_name` | `(snapshot)` | 快照的单位Id | 【源码实锤】+【dump 实锤】 |
| `base.snapshot_get_owner` | `(snapshot)` | 快照的所属玩家 | 【源码实锤】+【dump 实锤】 |
| `base.snapshot_get_facing` | `(snapshot)` | 快照的朝向 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/sound`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/sound'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\sound.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.player_play_music` | `(player, path)` | 播放音乐 | 【源码实锤】+【dump 实锤】 |
| `base.player_play_sound` | `(player, name)` | 播放音效 | 【源码实锤】+【dump 实锤】 |
| `base.point_play_sound` | `(point, name, distance, scene_name)` | 在指定点播放音效 | 【源码实锤】+【dump 实锤】 |
| `base.point_play_sound2` | `(point, name, distance)` | 在指定点播放音效 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/tds_score`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/tds_score'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\tds_score.lua`）

- lua_plus 云变量（TDS score）触发器封装层。读路径直调 `_G.score.get/money_get{user_id=..., key=...}`；写路径走**事务式 committer**：`score_commit_init` → `score.get_commit(game_name)` 建变更请求（`'__MAIN_MAP__'` 特判为 nil=本地图）→ `score_c_*` 系列向请求追加操作 → `score_c_commit` 提交。结果状态写 `base.last_commit_success / last_commit_error_code / last_commit_error_msg`（动态字段，dump 时无调用故未见）。
- 全部写操作先过 `committer_check(c)`（obj_check 注册的全局检查器）。
- **疑点**：`base.score_c_addi` 内部调 `c.addi{...}`，但 dump 的 `ScoreCommitter.prototype`（@common/base/tds_score）17 个方法中**无 `addi`**（有 `add`）——疑似死路径或经 `__index` 动态分发，列入待实测清单。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.score_commit_init` | `(game_name)` | 新建一个云变量变更请求 | 【源码实锤】+【dump 实锤】 |
| `base.get_last_created_score_committer` | `()` | 触发器最后创建的云变量变更请求 | 【源码实锤】+【dump 实锤】 |
| `base.get_last_commit_success` | `()` | 触发器最后提交的云变量变更请求是否成功 | 【源码实锤】+【dump 实锤】 |
| `base.get_last_commit_error_code` | `()` | 触发器最后提交的云变量变更请求错误代码 | 【源码实锤】+【dump 实锤】 |
| `base.get_last_commit_error_msg` | `()` | 触发器最后提交的云变量变更请求错误消息 | 【源码实锤】+【dump 实锤】 |
| `base.string_to_score_game` | `(game_name)` | 转换字符串地图名为地图 | 【源码实锤】+【dump 实锤】 |
| `base.score_money_get` | `(player, key)` | 获得玩家的云变量货币值 | 【源码实锤】+【dump 实锤】 |
| `base.score_geti` | `(player, key)` | 获得玩家的数值型云变量值 | 【源码实锤】+【dump 实锤】 |
| `base.score_exist` | `(player, key)` | 查询玩家的云变量值是否存在 | 【源码实锤】+【dump 实锤】 |
| `base.score_money_exist` | `(player, key)` | 查询玩家的云变量货币值是否存在 | 【源码实锤】+【dump 实锤】 |
| `base.score_gets` | `(player, key)` | 获得玩家的字符串云变量值 | 【源码实锤】+【dump 实锤】 |
| `base.score_get` | `(player, key)` | 获得玩家的任意表格类型云变量值 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_money_set` | `(c, player, key, value)` | 添加云变量请求操作：设置玩家货币 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_money_add` | `(c, player, key, value)` | 添加云变量请求操作：修改玩家货币 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_money_cost` | `(c, player, key, value)` | 添加云变量请求操作：消耗玩家货币 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_seti` | `(c, player, key, value)` | 添加云变量请求操作：设置数值型云变量 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_addi` | `(c, player, key, value)` | 添加云变量请求操作：修改数值型云变量 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_sets` | `(c, player, key, value)` | 添加云变量请求操作：设置字符串型云变量 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_set` | `(c, player, key, value)` | 添加云变量请求操作：设置任意表格类型云变量 | 【源码实锤】+【dump 实锤】 |
| `base.score_c_commit` | `(c)` | 提交云变量变更请求 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/test`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/test'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\test.lua`）

- 全文 1 个函数：`debug_bp_confident(...)`（`debug_bp` 存在时透传调用，调试断点用）。
- dump 对照：`debug_bp_confident` 在 `_G` 中 **ABSENT**——裸全局函数定义落在模块 env，不进真实 `_G`（与 obj_check 节结论互证）。模块键值 `true` = 文件已执行。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `debug_bp_confident` | `(...)` |  | 【源码实锤】（裸全局，落在模块 env，dump 未见，见组注） |

---

### `@lua_plus/base/base_lua_plus/timer`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/timer'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\timer.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.timer_clock` | `()` | 当前游戏时间 | 【源码实锤】+【dump 实锤】 |
| `base.timer_remove` | `(timer)` | 移除计时器 | 【源码实锤】+【dump 实锤】 |
| `base.timer_resume` | `(timer)` | 恢复计时器 | 【源码实锤】+【dump 实锤】 |
| `base.timer_pause` | `(timer)` | 暂停计时器 | 【源码实锤】+【dump 实锤】 |
| `base.timer_restart` | `(timer)` | 重启计时器 | 【源码实锤】+【dump 实锤】 |
| `base.timer_sleep` | `(time)` | 等待一段时间 | 【源码实锤】+【dump 实锤】 |
| `base.timer_wait` | `(time, func)` | 等待一段时间后执行动作 | 【源码实锤】+【dump 实锤】 |
| `base.timer_loop` | `(time, func)` | 每隔一段时间循环执行动作 | 【源码实锤】+【dump 实锤】 |
| `base.timer_timer` | `(time, times, func)` | 每隔一段时间循环执行动作(限定次数) | 【源码实锤】+【dump 实锤】 |
| `base.remaining` | `(timer)` | 计时器剩余的秒数 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/timershow`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/timershow'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\timershow.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.create_timershow` | `(x, y, time)` | 在指定位置创建计时器控件 | 【源码实锤】+【dump 实锤】 |
| `base.remove_timershow` | `(name)` | 移除指定计时器控件 | 【源码实锤】+【dump 实锤】 |
| `base.pause_timershow` | `(name)` | 暂停指定计时器控件 | 【源码实锤】+【dump 实锤】 |
| `base.resume_timershow` | `(name)` | 恢复指定计时器控件 | 【源码实锤】+【dump 实锤】 |
| `base.add_player_timershow_visible` | `(name, player)` | 设置计时器控件对玩家显示 | 【源码实锤】+【dump 实锤】 |
| `base.del_player_timershow_visible` | `(name, player)` | 设置计时器控件对玩家隐藏 | 【源码实锤】+【dump 实锤】 |
| `base.assign_timershow` | `(name, timer)` | 将指定计时器设置给计时器控件 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/trigger`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/trigger'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\trigger.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.trigger_disable` | `(trigger)` | 关闭触发器 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_enable` | `(trigger)` | 开启触发器 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_is_enable` | `(trigger)` | 触发器是否开启 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_remove` | `(trigger)` | 移除触发器 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_new` | `(func, t, disable, scene, sync)` |  | 【源码实锤】+【dump 实锤】 |
| `base.trigger_add_event` | `(trigger, trigger_event)` | 为触发器添加事件 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_unit` | `(unit, event_name)` | 单位事件 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_skill` | `(skill, event_name)` |  | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_eff_param` | `(eff_param, event_name)` |  | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_player` | `(player, event_name)` |  | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_game` | `(event_name)` |  | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_mover` | `(mover, event_name)` |  | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_timer_periodic` | `(time)` | 循环游戏时间事件 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_timer_once` | `(time)` | 单次游戏时间事件 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_event_wrapper_area` | `(area, event_name)` | 区域事件 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_custom_event_wrapper` | `(event_name)` | 自定义事件 | 【源码实锤】+【dump 实锤】 |
| `base.trigger_call` | `(trigger, e, sync)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/unit`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/unit'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\unit.lua`）

- **修正（相对 draft_v1）**：`base.unit_event_dispatch / unit_event_notify / unit_event_has / unit_event_subscribe / unit_event_unsubscribe` 5 个函数在源码中处于 `--[[ ... ]]` 块注释内（"TODO：等触发器设计"），dump 中全部 ABSENT，draft_v1 误列。下表为 105 个有效注册函数。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.unit_set_loot` | `(unit, link)` | 设置单位击杀奖励列表 | 【源码实锤】+【dump 实锤】 |
| `base.get_last_created_unit` | `()` | 触发器最后创建的单位 | 【源码实锤】+【dump 实锤】 |
| `base.get_all_units_id` | `()` | 获取所有单位ID | 【源码实锤】+【dump 实锤】 |
| `base.player_create_unit` | `(player, id, where, face)` | 创建单位 | 【源码实锤】+【dump 实锤】 |
| `base.player_create_unit_ai` | `(player, id, where, face, default_ai)` | 创建单位 | 【源码实锤】+【dump 实锤】 |
| `base.player_create_unit_on_scene` | `(player, id, where, face, scene)` | 创建单位（指定所属玩家和场景） | 【源码实锤】+【dump 实锤】 |
| `base.player_create_unit_illusion` | `(player, unit, where, face)` | 创建镜像单位（指定所属玩家） | 【源码实锤】+【dump 实锤】 |
| `base.player_create_unit_illusion_on_scene` | `(player, unit, where, face, scene)` | 创建镜像单位（指定所属玩家和场景） | 【源码实锤】+【dump 实锤】 |
| `base.unit_create_unit_illusion` | `(unit, dest, where, face)` | 创建镜像单位（指定所属单位） | 【源码实锤】+【dump 实锤】 |
| `base.create_unit_illusion` | `(unit, where, face)` | 创建镜像单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_id` | `(unit)` | 单位的编号 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_scale` | `(unit, scale)` | 设置单位缩放 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_attribute` | `(unit, state, value)` | 修改单位属性的基础值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_attribute_ex` | `(unit, state, value, value_type)` | 修改单位属性 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_ai` | `(unit, name, data)` | 添加单位AI | 【源码实锤】+【dump 实锤】 |
| `base.unit_play_animation` | `(unit, name, speed, loop, part)` | 播放单位动画 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_height` | `(unit, height)` | 增加单位高度 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_provide_sight` | `(unit, team)` | 共享单位视野 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_resource` | `(unit, energy_type, value)` | 增加单位能量 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_mark` | `(unit, unit_type)` | 为单位添加标记 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_sight` | `(unit, sight)` | 为单位添加可见形状 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_attribute` | `(unit, state)` | 单位的属性最终值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_attribute_ex` | `(unit, state, value_type)` | 单位的属性 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_attribute_max` | `(unit, state)` | 单位的属性值上限 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_attribute_min` | `(unit, state)` | 单位的属性值下限 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_class` | `(unit)` | 单位的类别 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_facing` | `(unit)` | 单位的朝向 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_height` | `(unit)` | 单位的离地高度 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_creation_param` | `(unit)` | 创建了指定单位的效果节点 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_name` | `(unit)` | 单位的Id | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_player` | `(unit)` | 单位的所属玩家 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_player` | `(unit, player)` | 设置单位的所属玩家 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_point` | `(unit)` | 单位的坐标 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_resource` | `(unit, resource_type)` | 单位的某种能量的数值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_mark` | `(unit, unit_mark)` | 单位的行为标记计数 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_attackable_radius` | `(unit)` | 单位的选取半径 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_team_id` | `(unit)` | 单位的队伍Id | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_tag` | `(unit)` | 单位的标签 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_walk_command_a` | `(unit)` | 单位的移动理由 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_walk_command_b_point` | `(unit)` | 单位的移动目标点 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_walk_command_b_unit` | `(unit)` | 单位的移动目标单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_walk` | `(unit, target)` | 令单位向点移动 | 【源码实锤】+【dump 实锤】 |
| `base.unit_has_mark` | `(unit, unit_mark)` | 单位是否存在标记 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_alive` | `(unit)` | 单位是否存活 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_ally_of_unit` | `(unit, dest)` | 单位与单位的盟友关系 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_ally_of_player` | `(unit, dest)` | 单位与玩家的盟友关系 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_enemy_of_unit` | `(unit, dest)` | 单位与单位的敌对关系 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_enemy_of_player` | `(unit, dest)` | 单位与玩家的敌对关系 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_illusion` | `(unit)` | 单位是否是镜像 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_in_range_of_unit` | `(unit, target, radius)` | 单位是否在另一单位的指定距离内 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_in_range_of_point` | `(unit, target, radius)` | 单位是否在点的指定距离内 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_visible_to_unit` | `(unit, target)` | 单位对单位是否可见 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_visible_to_player` | `(unit, target)` | 单位对玩家是否可见 | 【源码实锤】+【dump 实锤】 |
| `base.unit_is_walking` | `(unit)` | 单位是否在移动 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_z_speed` | `(unit, speed)` | 增加单位的Z轴速度值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_z_speed` | `(unit, speed)` | 设置单位的Z轴速度值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_z_speed` | `(unit)` | 单位的Z轴速度值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_kill` | `(unit, killer)` | 杀死单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_learn_skill` | `(unit, skill)` | 令单位学习技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_reborn` | `(unit, where)` | 复活单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_remove` | `(unit)` | 移除单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_remove_animation` | `(unit, animation_name)` | 移除单位的动画 | 【源码实锤】+【dump 实锤】 |
| `base.unit_remove_buff` | `(unit, buff_name)` | 移除单位指定类型的Buff | 【源码实锤】+【dump 实锤】 |
| `base.unit_remove_privide_sight` | `(unit, team_id)` | 移除单位对队伍的视野共享 | 【源码实锤】+【dump 实锤】 |
| `base.unit_remove_mark` | `(unit, unit_mark)` | 移除单位的行为标记 | 【源码实锤】+【dump 实锤】 |
| `base.unit_replace_skill` | `(unit, skill_id_old, skill_id_new)` | 替换单位的技能 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set` | `(unit, state, value)` | 设置单位属性基础值并清空百分比（数值属性） | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_ex` | `(unit, state, value, value_type)` | 设置单位属性（数值属性） | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_str` | `(unit, state, value)` | 设置单位属性（字符串属性） | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_attribute_max` | `(unit, state, value)` | 设置单位属性上限 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_attribute_min` | `(unit, state, value)` | 设置单位属性下限 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_attribute_sync` | `(unit, state, sync)` | 设置单位属性同步方式 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_facing` | `(unit, facing)` | 设置单位的朝向 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_height` | `(unit, height)` | 设置单位高度 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_model` | `(unit, model)` | 设置单位模型 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_resource` | `(unit, energy_type, value)` | 设置单位能量 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_attackable_radius` | `(unit, radius)` | 设置单位选取半径 | 【源码实锤】+【dump 实锤】 |
| `base.unit_texttag` | `(unit, target, text, text_type, sync, r, g, b, size)` | 创建漂浮文字 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_scene_name` | `(unit)` | 单位所在场景的名称 | 【源码实锤】+【dump 实锤】 |
| `base.unit_jump_scene` | `(unit, scene_name, position)` | 转移单位场景 | 【源码实锤】+【dump 实锤】 |
| `base.unit_jump_scene2` | `(unit, position)` | 转移单位场景 | 【源码实锤】+【dump 实锤】 |
| `base.get_all_units` | `()` | 所有单位 | 【源码实锤】+【dump 实锤】 |
| `base.node_mark` | `(node_mark, unit_name)` |  | 【源码实锤】+【dump 实锤】 |
| `base.set_location_async` | `(unit, position)` | 设置单位位置（异步） | 【源码实锤】+【dump 实锤】 |
| `base.set_facing_async` | `(unit, facing)` | 设置单位朝向（异步） | 【源码实锤】+【dump 实锤】 |
| `base.unit_anim_play` | `(unit, anim, time, time_type, start_offset, blend_time, priority)` | 模型单位播放一次动画 | 【源码实锤】+【dump 实锤】 |
| `base.unit_anim_set_paused_all` | `(unit, paused)` | 暂停/恢复单位模型动画（新API） | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_time_scale_global` | `(unit, time_scale)` | 设置单位模型动画相对播放时间倍数（只影响新API的动画） | 【源码实锤】+【dump 实锤】 |
| `base.unit_anim_play_bracket` | `(unit, anim_birth, anim_stand, anim_death, force_one_shot, kill_on_finish, priority, sync)` | 设置模型表现的bsd动画（新API） | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_exp` | `(unit)` | 单位的经验值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_exp` | `(unit, exp, ignore_fraction)` | 增加单位经验值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_exp` | `(unit, exp)` | 设置单位经验值 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_level` | `(unit)` | 单位的等级 | 【源码实锤】+【dump 实锤】 |
| `base.unit_add_level` | `(unit, level)` | 提高单位等级 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_level` | `(unit, level)` | 设置单位等级 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_max_level` | `(unit)` | 单位的等级上限 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_max_level` | `(unit, max_level)` | 设置单位等级上限 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_single_level_exp` | `(unit, level)` | 计算单位某一级所需经验 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_cumu_level_exp` | `(unit, level)` | 计算单位升到某一级所需的总经验 | 【源码实锤】+【dump 实锤】 |
| `base.unit_get_exp_fraction` | `(unit)` | 单位的经验倍率 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_exp_fraction` | `(unit, fraction)` | 设置单位经验倍率 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_prohibit_exp_distribute` | `(unit, value)` | 设置单位是否参与经验值分配 | 【源码实锤】+【dump 实锤】 |
| `base.unit_set_level_profile` | `(unit, profile_id)` | 设置单位升级配置 | 【源码实锤】+【dump 实锤】 |
| `base.unit_grant_loot` | `(unit, target, link)` | 直接给予单位奖励 | 【源码实锤】+【dump 实锤】 |
| `base.get_unit_from_id` | `(id)` | 从单位编号获取单位 | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/单位组`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/单位组'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\单位组.lua`）

- 实现**元表式单位组容器**：`base.单位组(单位数组)` 构造（支持 `+`/`-`/`==`/tostring 运算符重载与迭代），`get_items_table_mt` 为通用元表工厂（单位组/玩家组共用）。`mt:*` 方法即单位组实例方法面。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `get_items_table_mt` | `(items_table_name, item_check, tables_list)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:check_items_table` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:__add` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:__sub` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:__eq` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:__tostring` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:add_item` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:add_items` | `(items)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:_remove_item` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:refresh` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:remove_item` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:remove_items` | `(items)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:copy` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:contains` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:union` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:sub` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:intersect` | `(newtable)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:get_length` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt.new` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:get_items_map` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:add` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:has` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:delete` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:clear` | `(item)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:forEachEx` | `(callbackfn)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:random` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:randoms` | `(number, duplicate)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `mt:values` | `()` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `next` | `(self)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `base.单位组` | `(单位数组)` |  | 【源码实锤】+【dump 实锤】 |
| `base.create_unit_group` | `(units)` |  | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_random_unit` | `(ug)` |  | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_random_units` | `(ug, cnt)` |  | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_forEachEx` | `(ug, callbackfn)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/单位组_玩家组api`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/单位组_玩家组api'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\单位组_玩家组api.lua`）


**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.unit_group_add_item` | `(单位组, 单位)` | 向单位组添加单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_add_items` | `(单位组, 目标单位组)` | 向单位组添加单位组 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_contains` | `(单位组, 单位)` | 单位组是否包含单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_copy` | `(单位组)` | 单位组的复制 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_remove_item` | `(单位组, 单位)` | 从单位组移除单位 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_remove_items` | `(单位组, 目标单位组)` | 从单位组移除单位组 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_union` | `(单位组, 目标单位组)` | 单位组的并集 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_sub` | `(单位组, 目标单位组)` | 单位组的差集 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_intersect` | `(单位组, 目标单位组)` | 单位组的交集 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_count` | `(单位组)` | 单位组的单位数量 | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_get_items_map` | `(单位组)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/单位过滤器`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/单位过滤器'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\单位过滤器.lua`）

- `check_target_filter` 为文件内 local 辅助；4 个 `base.*` 为触发器"单位过滤"入口（v2 带 `_on_unit` 后缀为带基准单位版）。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `check_target_filter` | `(target_filter)` |  | 【源码实锤】（文件内 local 辅助/内部实现，非 API 面） |
| `base.target_filter_validate_on_unit` | `(...)` |  | 【源码实锤】+【dump 实锤】 |
| `base.target_filter_validate` | `(过滤, 过滤单位, 基准单位)` |  | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_filter_group_on_unit` | `(...)` |  | 【源码实锤】+【dump 实锤】 |
| `base.unit_group_filter_group` | `(单位组, 过滤, 基准单位)` |  | 【源码实锤】+【dump 实锤】 |

---

### `@lua_plus/base/base_lua_plus/附着点`

- 来源：server_lua_plus 包 v14（`server_lua_plus\14\base\`）
- 加载：`require 'lua_plus/base/base_lua_plus/附着点'`（由 `@lua_plus/base` 入口 `require_folder` 整目录加载）；dump 值 = `true`（已加载无表导出），API 经 `_G.base.*` 暴露
- 状态：✅ 有源码（`base\base_lua_plus\附着点.lua`）

- 全文 1 函数：`base.附着点(unit, socket)` 返回 `{unit, socket}` 二元表（绑点描述符，供 actor 挂接 API 使用）。文件头注释 `TODO pay/backend/debugger/score` 为遗留待办标记。

**函数**

| 函数 | 签名 | 说明 | 置信 |
| --- | --- | --- | --- |
| `base.附着点` | `(unit, socket)` | 单位的绑点 | 【源码实锤】+【dump 实锤】 |

---

## tds_score 云变量族（`@tds_score/*` + 关联键 `@common/base/tds_score`）

### `@tds_score`、`@tds_score/new_base_score`、`@tds_score/score`、`@tds_score/tds_score`、`@tds_score/use_mysql`

- 来源：tds_score 库（无独立分发包，引擎侧实现；require 根 `tds_score/`）
- 加载：5 键 dump 值全为 `true`——**已加载、无表导出**
- 状态：⚠️ 无源码
- dump 证据：模块体表不导出，但运行时面可从三处互证——`_G.score` 29 函数（下表）、`@common/base/tds_score` 36 类（下节）、lua_plus 包装器 `tds_score.lua` 的调用点（`score.get / score.money_get / score.get_commit / score.publish_message`，【源码实锤】调用形态）。
- `use_mysql` 键名提示存在 MySQL 存储后端变体；`_G.score.database_type` 函数或可查询当前后端【语义推测】，待实测。

**`_G.score` 运行时函数面（29 个，全部【dump 实锤】；语义为推测，签名以 tds_score.lua 调用点为准处另标）**

| 函数 | 说明 | 置信 |
| --- | --- | --- |
| `score.get{user_id, key}` | 读玩家云变量（数值/字符串/表格通用，返回 `error_code, data[], err_msg`；data 元素含 `i_value/s_value/value`） | 【dump 实锤】+【源码实锤】（tds_score.lua 调用点） |
| `score.money_get{user_id, key}` | 读玩家云变量货币（返回结构同上，`data[].value`） | 【dump 实锤】+【源码实锤】（tds_score.lua 调用点） |
| `score.get_commit(game_name)` | 创建云变量变更请求（ScoreCommitter），`game_name=nil` 表示本地图 | 【dump 实锤】+【源码实锤】（tds_score.lua 调用点） |
| `score.get_rank_list` | 排行榜查询 | 【dump 实锤】+【语义推测】 |
| `score.get_total_rank` | 排行榜总数查询 | 【dump 实锤】+【语义推测】 |
| `score.get_user_rank` | 查指定用户排名 | 【dump 实锤】+【语义推测】 |
| `score.list_query` / `score.list_query_by_uuid` | 云列表查询（按 key / 按 uuid） | 【dump 实锤】+【语义推测】 |
| `score.world_list_query` / `score.world_list_query_by_uuid` | 全服（world 域）云列表查询 | 【dump 实锤】+【语义推测】 |
| `score.world_data_get` | 全服共享数据读取 | 【dump 实锤】+【语义推测】 |
| `score.withlimit_query` | 带限购（周期限额）查询，对应 ScoreAddWith*LimitParam 族 | 【dump 实锤】+【语义推测】 |
| `score.name_exist` / `score.name_search` | 云变量名存在性 / 搜索 | 【dump 实锤】+【语义推测】 |
| `score.publish_message(channel, value)` | 向云频道发布消息（gamechat 局内聊天即用它，频道名 `@lib_gamechat_<session_id>`） | 【dump 实锤】+【源码实锤】（gamechat.lua 调用点） |
| `score.subscribe_channel` / `score.unsubscribe_channel` / `score.subscribe_message` | 云频道订阅/退订/消息回调 | 【dump 实锤】+【语义推测】 |
| `score.message_send` / `message_query` / `message_modify_read` / `message_delete` | 云消息（站内信式）发/查/已读/删 | 【dump 实锤】+【语义推测】 |
| `score.is_old_player` | 是否回流/老玩家判定 | 【dump 实锤】+【语义推测】 |
| `score.check_permission` | 云变量权限检查 | 【dump 实锤】+【语义推测】 |
| `score.database_type` | 查询存储后端类型（对照 `use_mysql`） | 【dump 实锤】+【语义推测】 |
| `score.test_cloud_value` | 云变量连通性测试 | 【dump 实锤】+【语义推测】 |
| `score.from_json` / `score.to_json` | 云变量值 JSON 编解码 | 【dump 实锤】+【语义推测】 |

**实测结论**（2026-08-27 编辑器 PIE 批次 2~4，探针 probe_server_apis）【实测】：

| 项 | 实测结果 |
| --- | --- |
| **写-读回环** | `score.get_commit()` → `c.add{user_id, key, value=1}` → `c.commit()` 返回 `(0, {})`；`score.get{user_id, key}` 回读得 `(0, [{user_id, key, value=1, raw_i_value=1}])`——**服务端云变量直写实锤可用** |
| 协程约束 | `score.get` 主线程直调报 `co.lua:56: cannot wrap coroutine by main thread!!!`——**必须协程内**（Q11） |
| `database_type()` | 返回 `"mysql"`——默认即 MySQL 后端（Q9） |
| `check_permission()` | 返回 `("p_55a3", "old")` 二值（图名 + 权限档） |
| `get_commit(game_name)` | 无参=本地图 OK；`get_commit(__MAIN_MAP__)`（**全局变量，值=主图名**，不带引号）与 `get_commit('p_55a3')`（显式真实图名）均实测 OK；**错误用法**：传 `'__MAIN_MAP__'` 字面量字符串（引擎小写化为 `__main_map__`，非真实图名）或不存在图名 → `no permission to operator game score[...]`（tds_score/score.lua:112）——跨图写有权限闸门（Q4）。注：lua_plus 包装器中 `game_name == '__MAIN_MAP__'` 的字符串比较是触发器 UI 层哨兵约定（转 nil），与 Lua 层全局变量 `__MAIN_MAP__` 是两层语境 |
| `commit()` 返回 | 成功 = `(0, {})`，第二返回是数据表**不是错误码**；`base.last_commit_success/error_code/error_msg` 由 lua_plus 包装器设置，裸 commit 后仍为 nil（Q3） |
| **死路径实锤** | committer 实测方法面 `add/set/commit/name_new/money_add` 在；**`addi`=nil、`rank_add`=nil**——`base.score_c_addi` 及 rank_add 族包装器调用即 nil 错误（Q2，lua_plus v14 与引擎版本漂移） |
| 排行读接口缺失 | `get_rank_list/get_user_rank/get_total_rank` 调用均报 `tds_score/use_mysql.lua:530/566/581: field nil`——**当前 MySQL 后端未实现排行榜查询三接口** |
| `is_old_player{user_id}` | OK 返回 `(0, true)`（参数须为表，Q10） |
| `test_cloud_value{user_id, key}` | OK 返回 `(0, 1)`（连通性测试，Q10） |
| `list_query{key=}` | OK 返回 `(0, {})`；位置参/缺 key 报错（"key参数不是字符串"） |
| `name_exist` | `{name_substr=}` 与位置字符串均报"name_substr参数不是字符串"——参数形态未走通（疑多位置参） |
| `world_data_get / world_list_query` | 需 `worldId` 合法整数；`{worldId=1}` 仍报"不是合法整数"（疑需真实世界 ID 或键名不同，Q5 未完全走通） |
| **频道消息回环**（Q7） | `subscribe_message(chan, {ok=, error=, timeout=})` 返回 `true`；`publish_message(chan, {text=...})` 后 ok 回调收到 `{message={text=...}}`——**服务端云频道 pub/sub 走通**（callback 必须是表不是函数） |
| `message_send{key=}` | 需 `target_user_id` 整数；`message_query{key=}` OK 返回 `(0, {})`（Q8 部分） |
| 底层实现 | 引擎内嵌 Lua：`tds_score/score.lua`、`tds_score/use_mysql.lua`（traceback 实锤，api-13 无此文件，pak-extract 可挖） |
| 读写计次/限频 | 编辑器调试环境未观测到计次限制（正式环境限次规则见 cloudvar 客户端研究，服务端侧待正式环境验证） |

### `@common/base/tds_score`（关联键，common-base 组；tds_score 族的 TSTL 类注册表）

- 来源：script 包（common 库），⚠️ 无源码；加载 `require '@common/base/tds_score'`
- dump 值：table，**36 个顶层键全部为美 TSTL 类**（`prototype`/`____constructor`/`_descriptors` 结构），41 个类节点（含 5 个 `____super` 父类），116 个函数叶子（多为 `__index/__newindex/____constructor` 机械三联）。**109 处截断**（全部为各参数类 `_descriptors.<prop>.set/get/enumerable/configurable`——属性描述符的访问器函数被截断，字段不全，但属性名本身完整可读）。

**类清单（36 个顶层类，按语义分组，全部【dump 实锤】）**

| 分组 | 类 | 可确认属性（`_descriptors` 键名） |
| --- | --- | --- |
| 单值读写参数 | `ScoreGetParam` | `_key`, `_user_id` |
| | `ScoreSetParam` / `ScoreAddParam` / `ScoreClearParam` / `ScoreMoneyParam` | `_key` |
| 排行榜参数 | `ScoreRankAddParam` / `ScoreRankSetParam` / `ScoreGetRankListParam` / `ScoreGetRankTotalParam` / `ScoreGetUserRankParam` | `_key` |
| 云列表参数 | `ScoreListAddParam` / `ScoreListGetParam`（`_key`,`_user_id`,`_timetype`,`_limit`） / `ScoreListDeleteParam` / `ScoreListModifyParam` / `ScoreListQueryByUuidParam` | 见左 |
| 周期限购加值 | `ScoreAddWithLimitParam`（基类）→ `ScoreAddWithHourLimitParam` / `ScoreAddWithDayLimitParam` / `ScoreAddWithWeekLimitParam` / `ScoreAddWithMonthLimitParam` / `ScoreAddWithYearLimitParam`（5 子类均 `____super` 指向基类） | `_key` |
| 云变量名/消息参数 | `ScoreNameNewParam` / `ScoreNameSearchParam` / `ScoreMsgQueryParam` / `ScoreMsgSendParam` | `_key` |
| 数据载体（prototype 仅 3 键，纯数据类） | `ScoreData` / `ScoreMoneyData` / `ScoreNameData` / `ScoreMsgData` / `ScoreObjData` / `ScoreListData` / `ScoreRankListData` / `ScoreGetUserRankData` / `ScoreGetRankTotalData` / `ScoreChannelMsgData` | （无描述符，字段被内联共享省略） |
| 变更请求 | `ScoreCommitter`（prototype 19 键） | 见下 |

**ScoreCommitter 方法面（17 个，【dump 实锤】；签名未知，参数形态可对照 tds_score.lua 调用点）**

`commit / set / add / clear / money_set / money_add / money_add_ex / money_cost / rank_add / rank_set / list_add / list_delete / list_modify / withlimit_add / name_new / add_finish_callback / ____constructor`

- 与 lua_plus 包装器对照：`base.score_c_commit→c.commit`、`score_c_set(i/s)→c.set{i_value|s_value|value}`、`score_c_money_set/add/cost→c.money_*` 全部对上【源码实锤】；**例外：`score_c_addi→c.addi` 实测为死路径**——committer 无 `addi`/`rank_add` 方法（2026-08-27 PIE 实测 `type(c.addi)==nil`），lua_plus v14 包装器与当前引擎版本漂移【实测】。
- 数据载体类的 prototype 在 dump 中显示为"跨键共享内联"（同一 prototype 对象同时出现在 `@common/base/lualib_bundle`、`@common/base/tds_score`、`_G` 等 6 处）——这些类是全场单例，lualib_bundle 的 CLASSES 注册表同源。

---
