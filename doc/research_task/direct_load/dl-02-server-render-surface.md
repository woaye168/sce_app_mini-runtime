# dl-02 — 服务端渲染面盘点（StateGame package.loaded 候选清单）

> 日期：2026-08-27 ｜ 状态：✅ 盘点完成（静态筛选，未做新增实测）
> 研究任务：「模型/图集子图免数编直载」线索一——服务端 StateGame 为模型/特效/单位创建权威入口，本文档产出双端联动实测（服务端创建 + 客户端观察）的候选函数/类清单。
>
> **输入材料（全部只读）**：
> - [../loaded_modules_server/draft_v2/00-INDEX.md](../loaded_modules_server/draft_v2/00-INDEX.md) 及同目录 common-base.md / common-base-game.md / lua-plus.md / map-libs.md / smallcard-libs.md（216 键分组文档）
> - [../../test/loaded_modules_server/parsed/fields/_G.json](../../test/loaded_modules_server/parsed/fields/_G.json)（5121 函数路径 / 373 类）及 `parsed/fields/` 各模块 JSON
> - 既有结论（只读参考）：[../lowlevel/render-06-unit-change-model.md](../lowlevel/render-06-unit-change-model.md)（客户端 `game.unit_change_model` 须真实单位、吃 prefab 裸路径【实测】）、[../lowlevel/render-17-dynamic-uiworld.md](../lowlevel/render-17-dynamic-uiworld.md)（客户端免数编判死：创建必须有已注册数编 link）、[../lowlevel/render-21-setasset-entry-mutation.md](../lowlevel/render-21-setasset-entry-mutation.md)（set_asset 分派链：lua wrapper 零校验 → vtable+0xa0 分派 → manager 查 MODEL/EFFECT 表，miss 静默返回）
>
> **置信级标注**（沿用 00-INDEX 约定）：【实测】/【dump 实锤】/【源码实锤】/【反查推测】/【语义推测】。
> **特别标记**：🔗 = 接受「路径/link/名称」参数（免数编直载相关）；🅢 = 服务端独有面（客户端 dump/文档中不存在或客户端为另一套实现，双端联动实验重点）。

## 0. 口径与关键前提（筛选时必读）

1. **服务端权威面以 dump 为准**：`@common/base/game/*` 15 键为引擎内嵌（接管 script 同名文件）；draft_v1 反查的 `base.game.debug_draw_* / camera_focus / load_combined_map / get_model_anim_point_info` 等**不在服务端 dump 的 base.game 124 函数内**（客户端语料），已从候选剔除（见 common-base-game.md 组级结论 4）。
2. **Unit 类双端不对称（重大线索）**：script 源码 `common/base/unit.lua` 的 `mt:change_model(path)` / `mt:attach_model(path, hand_point, hold_point)` / `mt:detach_model(path)` / `mt:get_model_path()` / `mt:get_asset()` **未出现在服务端 dump 的 `Unit.prototype`（386 键）中**——这些形态对应客户端 native `game.unit_change_model`（render-06【实测】）。而服务端 dump 独有 `Unit.prototype.set_model / set_model_attribute / model_swap_push / model_swap_pop / set_asset / set_particle / update_actor` ——**服务端换模入口是另一套命名**，是双端联动的首要实测对象。
3. **Actor 类双端不对称**：服务端 dump `Actor.prototype` 44 方法，**有 `set_asset` 但无** `set_material_parameters / attach_to_anchor / get_socket_* / set_fow / get_anims / play_anim_ex / set_highlight`（后组为 script 源码实锤但 dump 未见，疑客户端向）。
4. **服务端当前渲染面实测 = 空白**：既有【实测】（2026-08-27 PIE 批次 1~4）只覆盖 score/backend/room/协程，未触渲染面；渲染侧【实测】均来自客户端研究（render-06/17/21），本文档直接引用其结论。
5. **lua_plus = server_lua_plus 包**（服务端专用触发器层），其 `base.*` 扁平函数天然是🅢服务端独有面，全部【源码实锤】+【dump 实锤】双证。

---

## A. 单位创建 / 模型换装（30 条）

### A1. 创建单位（lua_plus 扁平封装，🅢 全部服务端独有）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| A1 | `base.player_create_unit` 🔗🅢 | `@lua_plus/base/base_lua_plus/unit` | `(player, id, where, face)` | 【源码实锤】+【dump 实锤】 | id = 物编单位 link |
| A2 | `base.player_create_unit_ai` 🔗🅢 | 同上 | `(player, id, where, face, default_ai)` | 【源码实锤】+【dump 实锤】 | 带默认 AI |
| A3 | `base.player_create_unit_on_scene` 🔗🅢 | 同上 | `(player, id, where, face, scene)` | 【源码实锤】+【dump 实锤】 | 指定场景 |
| A4 | `base.player_create_unit_illusion` / `base.player_create_unit_illusion_on_scene` 🔗🅢 | 同上 | `(player, unit, where, face[, scene])` | 【源码实锤】+【dump 实锤】 | 镜像单位 |
| A5 | `base.unit_create_unit_illusion` / `base.create_unit_illusion` 🅢 | 同上 | `(unit, where, face)` / `(unit, dest, where, face)` | 【源码实锤】+【dump 实锤】 | 镜像单位 |
| A6 | `base.eff.EffectCreateUnit.validate/execute` 🅢 | `@common/base/eff`（`_G.base.eff`） | `(?)` | 【dump 实锤】 | 效果树「创建单位」节点；同族另有 `EffectCreateItem` |
| A7 | `base.eff.EffectLaunchMissile.validate/execute` 🅢 | 同上 | `(?)` | 【dump 实锤】 | 效果树「发射弹道」节点（弹道=模型/mover） |

### A2. TSTL 类方法（服务端 dump 权威面）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| A8 | `Player.prototype.create_unit` 🔗🅢 | `@common/base/player` | 签名未知 | 【dump 实锤】 | lua_plus A1 的底层【语义推测】 |
| A9 | `Player.prototype.create_illusion` / `create_light_unit` / `create_block_units` / `create_controlled_sync_unit` 🅢 | 同上 | 签名未知 | 【dump 实锤】 | 轻量单位/阻挡单位/受控同步单位 |
| A10 | `Unit.prototype.create_unit` 🔗🅢 | `@common/base/unit` | 签名未知 | 【dump 实锤】 | 单位造单位（召唤物语义） |
| A11 | `Unit.prototype.create_light_unit` / `create_block_units` 🅢 | 同上 | 签名未知 | 【dump 实锤】 | — |
| A12 | `Unit.prototype.create_illusion` 🅢 | 同上 | 签名未知 | 【dump 实锤】 | — |
| A13 | `Unit.prototype.change_type` 🔗🅢 | 同上 | 签名未知 | 【dump 实锤】 | 单位类型切换（疑=数编 link 重建，若接受运行时 link 则是直载捷径） |

### A3. 换模 / 模型属性（★ 核心候选区）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| A14 | `Unit.prototype.set_model` 🔗🅢 | `@common/base/unit` | 签名未知 | 【dump 实锤】 | **服务端换模第一候选**；对标客户端 `game.unit_change_model(id, prefab路径)`【实测：render-06，吃裸路径但须真实单位】 |
| A15 | `base.unit_set_model` 🔗🅢 | `@lua_plus/base/base_lua_plus/unit` | `(unit, model)` | 【源码实锤】+【dump 实锤】 | A14 的触发器封装；`model` 参数形态（link/路径）未实测 |
| A16 | `Unit.prototype.model_swap_push` / `model_swap_pop` 🔗🅢 | `@common/base/unit` | 签名未知 | 【dump 实锤】 | **换模栈**——push/pop 语义暗示临时换装可还原，服务端 dump 独有 |
| A17 | `Unit.prototype.set_model_attribute` 🔗🅢 | 同上 | 签名未知 | 【dump 实锤】 | 模型属性直改 |
| A18 | `Unit.prototype.set_random_model_index` 🅢 | 同上 | 签名未知 | 【dump 实锤】 | 随机模型索引（数编多模型变体） |
| A19 | `Unit.prototype.set_asset` 🔗🅢 | 同上 | 签名未知 | 【dump 实锤】 | 对标客户端 `actor:set_asset` 分派链（render-21：lua wrapper 零解析、manager 查表 miss 静默）；服务端 GameUnit 级 set_asset 客户端 render-21 未捕获 |
| A20 | `Unit.prototype.set_particle` 🔗🅢 | 同上 | 签名未知 | 【dump 实锤】 | 粒子直改 |
| A21 | `Unit.prototype.update_actor` / `refresh_unit_buff_actors` 🅢 | 同上 | 签名未知 | 【dump 实锤】 | 单位 actor 刷新（换模后同步？） |
| A22 | `base.unit_update_actor` 🅢 | 引擎注册（`_G.base`，lua_plus 源码未见） | 签名未知 | 【dump 实锤】 | dump-only base 全局函数 |
| A23 | `base.crop.__index.set_model_unit` / `set_unit_model` / `fresh_unit_model` / `create_crop_unit` 🔗🅢 | `@common/base/crop`（无源码；`_G.base.crop` 实锤） | 签名未知 | 【dump 实锤】 | 作物单位换模全套（采集玩法），证明「运行时换单位模型」在服务端有官方业务先例 |

### A4. 单位动画（换装后验证用）

| # | 函数路径 | 所属模块 | 签名 | 置信 |
| --- | --- | --- | --- | --- |
| A24 | `base.unit_play_animation` 🔗🅢 | `@lua_plus/.../unit` | `(unit, name, speed, loop, part)` | 【源码实锤】+【dump 实锤】 |
| A25 | `base.unit_anim_play` / `base.unit_anim_play_bracket` / `base.unit_anim_set_paused_all` / `base.unit_set_time_scale_global` 🅢 | 同上 | 见 lua-plus.md | 【源码实锤】+【dump 实锤】 |
| A26 | `base.unit_remove_animation` 🔗🅢 | 同上 | `(unit, animation_name)` | 【源码实锤】+【dump 实锤】 |
| A27 | `base.skill_set_animation` 🔗🅢 | `@lua_plus/.../skill` | `(skill, animation)` | 【源码实锤】+【dump 实锤】 |
| A28 | `Unit.prototype.anim_play` / `anim_play_once` / `anim_play_time` / `anim_play_speed` / `anim_play_start_time` / `anim_play_bracket` / `anim_play_bracket_once` / `anim_play_bracket_time` / `anim_operation` / `anim_stop` / `anim_set_paused_all` / `play_animation` / `add_animation` / `remove_animation` | `@common/base/unit` | 签名未知 | 【dump 实锤】 |
| A29 | `base.auxiliary.add_animation` 🔗 | `@common/base/auxiliary` | `(unit, animation_name, scale, is_loop, part)` | 【反查推测】 |
| A30 | `base.anim` / `base.bracket_anim` / `base.get_anim_map` / `base.get_anim_bracket_map` 🔗 | `@common/base/anim_handlers` | 见 common-base.md | 【源码实锤】 |

> **剔除记录（双端不对称实证）**：script 源码 `unit.lua` 的 `mt:change_model / attach_model / detach_model / get_model_path / get_asset` 不在服务端 dump `Unit.prototype` 内 → 非服务端候选；客户端对应 native `game.unit_change_model` / `game.unit_attach_model` 已有【实测】（render-06）。

---

## B. actor 创建与附着（24 条）

### B1. 创建/获取 actor

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| B1 | `base.create_actor_at` 🔗🅢 | `@lua_plus/base/base_lua_plus/actor` | `(name, point, use_terrain)` | 【源码实锤】+【dump 实锤】 | **触发器建表现入口**；`name` 疑为数编 actor link，是否吃裸路径未实测 |
| B2 | `base.actor` 🔗 | `@common/base/actor` | `(name, sid, skip_birth, scene)` | 【源码实锤】 | script Actor 类构造；B1 底层 |
| B3 | `base.get_last_created_actor` 🅢 | `@lua_plus/.../actor` | `()` | 【源码实锤】+【dump 实锤】 | 取刚建 actor（联动验证用） |
| B4 | `base.actor_from_id` / `base.actor_from_sid` / `base.get_actor_from_id` / `base.get_actor_from_sid` / `base.actor_info` | `@common/base/actor` | `(id)` | 【源码实锤】 | — |
| B5 | `base.play_actor` / `base.stop_actor` 🅢 | 引擎注册（`_G.base`，lua_plus 源码未见） | 签名未知 | 【dump 实锤】 | dump-only；疑为 native 直注册的表现播放控制 |
| B6 | `defaultui.NewActor` 类（prototype 33 方法：`attach_to`/`detach`/`anim_operation`/`anim_set_paused_all`/`get_visible_slots`/`sync_attribute` 等） 🅢 | `@defaultui/actor`（实现未分发） | — | 【dump 实锤】 | **服务端 dump 独有 OO 表现封装**；与 lua_plus `base.actor_*` 关系【语义推测：上层 OO 封装】 |
| B7 | `defaultui.create_NewActor_at` 🔗🅢 | `@defaultui/main`（值内联） | 签名未知 | 【dump 实锤】 | 在指定位置创建 NewActor |

### B2. 附着型创建（挂到单位/Buff/效果节点）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| B8 | `base.create_actor_on_buff` 🔗🅢 | `@lua_plus/.../actor` | `(name, host)` | 【源码实锤】+【dump 实锤】 | — |
| B9 | `base.unit_get_actor` / `base.buff_get_actor` / `base.eff_param_get_actor` 🔗🅢 | 同上 | `(host, name)` | 【源码实锤】+【dump 实锤】 | 附着查询（联动验证用） |
| B10 | `base.actor_attach_to_unit` / `base.actor_attach_to_actor` 🔗🅢 | 同上 | `(actor, host, socket)` | 【源码实锤】+【dump 实锤】 | socket=挂点名 |
| B11 | `Unit.prototype.create_actor` 🔗 | `@common/base/unit` | `(link, ignore_unit_list)` | 【源码实锤】+【dump 实锤】 | 数编 actor link |
| B12 | `Unit.prototype.create_actors` / `destroy_actors` | 同上 | `(msg)` | 【源码实锤】+【dump 实锤】 | — |
| B13 | `Skill.prototype.create_actor` 🔗 / `create_actors` / `destroy_actors` / `create_actor_passive` / `destroy_actors_passive` / `start_effect` | `@common/base/skill` | `(link)` 等 | 【源码实锤】+【dump 实锤】 | 技能挂表现 |
| B14 | `EffectParam.prototype.create_actor` 🔗 | `@common/base/eff_param` | `(link, position, force_no_sync)` | 【源码实锤】+【dump 实锤】 | **效果树内建 actor 创建；`force_no_sync` 参数值得联动实测** |
| B15 | `base.unit_buff.create_actor` / `create_actors` / `destroy_actors` 🔗 | `@common/base/buff`（`_G.base.unit_buff`） | 签名未知 | 【dump 实锤】 | Buff 挂表现 |
| B16 | `ScenePoint.prototype.create_effect` 🔗 | `@common/base/scene_point` | `(model)` | 【源码实锤】 | 点上建特效（TODO 注释：需指定中立玩家） |

### B3. actor 控制 / 资产替换

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| B17 | `Actor.prototype.set_asset` 🔗 | `@common/base/actor` | `(asset)` | 【源码实锤】+【dump 实锤】 | **render-21 分派链客户端判死裸路径**（manager 查 MODEL/EFFECT 表 miss 静默）；服务端未实测 |
| B18 | `base.actor_set_asset_model` 🔗🅢 / `base.actor_set_asset_sound` 🔗🅢 | `@lua_plus/.../actor` | `(actor, asset)` | 【源码实锤】+【dump 实锤】 | B17 触发器封装；注释「仅对模型和粒子表现有效」 |
| B19 | `base.actor_destroy` 🅢 | 同上 | `(actor, flag)` | 【源码实锤】+【dump 实锤】 | — |
| B20 | `base.actor_set_scale` / `set_shadow` / `set_position` / `set_facting`(原文拼写) / `set_grount_height`(原文拼写) / `set_owner` / `actor_play` / `actor_stop` / `actor_pause` / `actor_resume` / `set_volume` / `set_grid_size` / `set_grid_range` / `set_grid_state` 🅢 | 同上 | 见 lua-plus.md | 【源码实锤】+【dump 实锤】 | — |
| B21 | `base.actor_anim_play` 🔗🅢 / `actor_anim_play_bracket` / `actor_anim_set_paused_all` / `actor_set_time_scale_global` | 同上 | 见 lua-plus.md | 【源码实锤】+【dump 实锤】 | 模型表现动画新 API |
| B22 | `Actor.prototype.attach_to` / `detach` / `show` / `play` / `stop` / `set_scale` / `set_facing` / `set_position` / `set_bearings` / `anim_play` / `anim_play_bracket` / `set_time_scale_global` / `get_visible_slots`（44 方法全集见 `parsed/fields/common__base__actor.json`） | `@common/base/actor` | 见 common-base.md | 【dump 实锤】（多数兼【源码实锤】） | 服务端 dump 无 `attach_to_anchor` / `set_material_parameters` / `set_fow` / `get_socket_*`（疑客户端向） |
| B23 | `base.play_sound_effect` 🔗 | `@common/base/actor` | `(link)` | 【源码实锤】 | 音效 actor；`base.actor_set_asset_sound` 同理 |
| B24 | `base.set_actor_map` / `base.set_actor_mode` / `base.actor_enable_raycast` / `base.set_unit_highlight_on` / `set_unit_highlight_off` | `@common/base/actor` | 见 common-base.md | 【源码实锤】 | 全局 actor 注册/射线/高亮 |

---

## C. 特效创建（8 条）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| C1 | `base.create_beam_effect` 🔗 | `@common/base/actor` | `(link, source, target)` | 【源码实锤】 | 光束特效（返回 Actor） |
| C2 | `base.player_create_lightning` 🔗🅢 | `@lua_plus/base/base_lua_plus/lightning` | `(player, model, source, target)` | 【源码实锤】+【dump 实锤】 | **`model` 参数 = 闪电模型 link/路径，免数编疑点**；触发器层覆盖引擎同名底层（lua-plus.md §lightning） |
| C3 | `base.unit_create_lightning` 🔗🅢 | 同上 | `(unit, model, source, target)` | 【源码实锤】+【dump 实锤】 | 同上 |
| C4 | `Unit.prototype.lightning` 🔗🅢 | `@common/base/unit` | 签名未知 | 【dump 实锤】 | C2/C3 底层【语义推测】 |
| C5 | `Unit.prototype.effect` 🔗🅢 / `Player.prototype.effect` 🔗🅢 | `@common/base/unit` / `@common/base/player` | 签名未知 | 【dump 实锤】 | 语义=执行效果节点【语义推测】；与 `base.unit_execute_effect_on_unit/point` 关系待考 |
| C6 | `base.unit_execute_effect_on_unit` 🔗🅢 / `base.unit_execute_effect_on_point` 🔗🅢 | `@lua_plus/.../unit`（eff_param 组） | `(unit, target, link)` | 【源码实锤】+【dump 实锤】 | 直接按 link 执行效果树 |
| C7 | `base.response.ResponseEffectImpact`（`exectue`（原文拼写）等） | `@common/base/response` | 签名未知 | 【dump 实锤】 | 效果命中响应（弹道命中挂表现？） |
| C8 | `base.eff.EffectLaunchMissile` / `EffectTeleport` / `EffectUnitModifyHeight` / `EffectUnitModifyFacing` 等 60+ 效果节点 `validate/execute` 🅢 | `@common/base/eff`（`_G.base.eff`） | `(?)` | 【dump 实锤】 | 效果树节点全集（`_G.json` 实锤）；EffectLaunchMissile 见 A7 |

---

## D. 场景 / 世界管理（18 条）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| D1 | `base.game.load_scene` / `load_scene_internal` / `close_scene` / `close_scene_internal` / `has_scene` 🅢 | `@common/base/game/game` | 签名未知 | 【dump 实锤】 | 客户端语料未见，🅢 |
| D2 | `base.game.get_all_scene_name` / `get_all_template_scene_name` / `get_default_scene_name` / `get_scene_scale` / `is_world_scene` 🅢 | 同上 | — | 【dump 实锤】 | — |
| D3 | `base.game.create_scene_copy` / `create_scene_copy_internal` / `load_create_scene_copy` 🅢 | 同上 | — | 【dump 实锤】 | 场景副本 |
| D4 | `base.game.get_scene_object` / `get_scene_object_by_key` / `get_scene_object_list` / `close_scene_object` 🔗🅢 | 同上 | — | 【dump 实锤】 | 场景对象（场景内预置 actor 集合） |
| D5 | `base.game.set_surrounding` / `set_surrounding_scene` / `set_surrounding_scene_internal` 🅢 | 同上 | — | 【dump 实锤】 | 无缝地图周边场景 |
| D6 | `base.game.init_units` / `load_controlled_sync_units` 🅢 | 同上 | — | 【dump 实锤】 | **单位初始化/受控同步单位加载——服务端→客户端同步链入口，双端联动必查** |
| D7 | `base.game.get_default_unit` / `get_default_units` / `get_default_item` 🔗🅢 | 同上 | — | 【dump 实锤】 | 物编默认表读取 |
| D8 | `base.game._scene_object_map.default.scene_actors`（运行时表，截断） | 同上 | — | 【dump 实锤】 | 场景对象内挂 `scene_actors` 集合——场景 actor 注册表的直接证据 |
| D9 | scene_manager 7 函数：`set_scene_activated` / `set_scene_not_activated` / `is_scene_activated` / `get_activated_scenes` / `get_obj_scene_events` / `init_region` / `check_event_scene_region_remove` | `@common/base/game/scene` | 见 common-base-game.md | 【dump 实锤】+【源码实锤】（前 5 个 script 变体签名） | 本组唯一表导出键 |
| D10 | `base.scene_object` 🔗🅢 | 引擎注册（`_G.base`，分组文档未收录） | 签名未知 | 【dump 实锤】 | dump-only；疑为场景对象构造/查询 |
| D11 | `base.player_jump_scene_object` 🔗🅢 | 同上 | 签名未知 | 【dump 实锤】 | dump-only |
| D12 | `base.player_jump_scene` 🅢 | `@lua_plus/.../player` | `(player, scene, keep_hero)` | 【源码实锤】+【dump 实锤】 | — |
| D13 | `base.unit_jump_scene` / `base.unit_jump_scene2` 🅢 | `@lua_plus/.../unit` | `(unit, scene_name, position)` / `(unit, position)` | 【源码实锤】+【dump 实锤】 | — |
| D14 | `Unit.prototype.jump_scene` / `Player.prototype.jump_scene` 🅢 | `@common/base/unit` / `player` | 签名未知 | 【dump 实锤】 | — |
| D15 | `base.get_scene_point` / `get_scene_line` / `get_scene_rect` / `get_scene_circle` / `get_scene_area` / `get_scene_scale_area` 🔗🅢 | `@lua_plus/.../point` 等 | 见 lua-plus.md | 【源码实锤】+【dump 实锤】 | 地编元素读取 |
| D16 | `base.get_scene_name_by_hash` / `get_scene_hash_by_name` 🔗 | `@common/base/game`（script） | `(hash)` / `(name)` | 【源码实锤】 | 场景名↔哈希 |
| D17 | `Player.prototype.set_camera` / `shake_camera` / `unlock_camera` 🅢 | `@common/base/player` | 签名未知 | 【dump 实锤】 | 服务端驱动镜头（渲染间接面） |
| D18 | `base.game.ray_cast` / `set_sight_block` / `get_placement_point` / FOW 系列（`switch_fow_mode` 等）🅢 | `@common/base/game/game` | — | 【dump 实锤】 | 世界查询/视野（弱渲染相关，备查） |

> **剔除记录**：`base.game.debug_draw_*` / `create_debug_draw_actor` / `camera_focus` / `lock_camera` / `load_combined_map` / `load_combined_map_deco` / `load_scene_cache_and_combined` / `get_model_anim_point_info` / `set_dynamic_point_light` 为 script 源码实锤但**不在服务端 dump base.game 124 函数内**（common-base-game.md 组级结论 4），非服务端候选。

---

## E. 资源加载 / 数编缓存 / 路径解析（10 条）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| E1 | `base.eff.cache` 🔗 | `@common/base/eff` | `(link)` | 【源码实锤】 | **数编效果缓存统一入口**——一切 eff link 解析走这里；render-20 虚拟 merge_cache 条目客户端判死，服务端侧是否同样校验未实测 |
| E2 | `base.eff.cache_ts` / `cache_as` / `caches` / `all_caches` / `original_data` / `get_namespace` / `find_sibling` 🔗 | 同上 | 见 common-base.md | 【源码实锤】 | 缓存族 |
| E3 | `EffectParam.prototype.search` 🔗 / `set_cache` / `init_child_on(link, target)` / `execute_child_on(link, target)` | `@common/base/eff_param` | 见 common-base.md | 【源码实锤】 | 效果树 link 解析/执行 |
| E4 | `base.game.load_json_file` / `load_lua_file` 🔗🅢 | `@common/base/game/game` | 签名未知 | 【dump 实锤】 | **服务端运行时文件加载**——路径解析范围（pak 内/外）未实测 |
| E5 | `base.game.lni` / `json_encode` / `json_decode` / `do_json_table` 🅢 | 同上 | — | 【dump 实锤】 | 序列化/lni（`@common/preload/lni_loader` 转发桩） |
| E6 | `base.get_default_unit` / `base.get_default_item` 🔗 | `@common/base/unit` | `(node_mark)` | 【源码实锤】 | 物编默认表按 mark 读取（源码注释：客户端从服务器获取默认地编单位） |
| E7 | `Unit.prototype.get_creation_param` / `EffectParam.prototype.get_creation_param` 🅢 | `@common/base/unit` / `eff_param` | 签名未知 | 【dump 实锤】 | 创建参数回看（联动验证用） |
| E8 | `base.game.statistics_create_actor` 🅢 | `@common/base/game/game` | 签名未知 | 【dump 实锤】 | 命名可疑（统计侧建 actor？）；`base.any_*.statistics_create_actor` 同族均为共享表内联 |
| E9 | `io.open` / `io.read` / `io.lines`（stdlib 裁剪子集）🅢 | `io`（stdlib） | 标准 | 【dump 实锤】 | 服务端文件 IO 存在——pak 外直读可能性探针（配合 E4） |
| E10 | `package.searchpath`（stdlib 裁剪仅存此函数）🅢 | `package` | 标准 | 【dump 实锤】 | 模块路径解析残留面 |

---

## F. 其他可疑渲染入口（8 条）

| # | 函数路径 | 所属模块 | 签名 | 置信 | 备注 |
| --- | --- | --- | --- | --- | --- |
| F1 | `Unit.prototype.texttag` / `texttag_by_location` / `texttag_by_location_color_size` 🅢 | `@common/base/unit` | 签名未知 | 【dump 实锤】 | 飘字=服务端驱动的渲染面；script 源码另有 `mt:create_riseletter*(...)` 系列（服务端 dump Unit.prototype 未见，疑客户端向——待对照） |
| F2 | `Unit.prototype.set_highlight` / `set_outstroke` / `set_tint_color` / `set_tint_enabled` / `set_xray_enable` / `set_shadow` / `set_visible` / `set_blood_bar_visible` / `set_status_bar_visibility` | `@common/base/unit`（script） | 见 common-base.md | 【源码实锤】 | 渲染状态修改（联动观察辅助） |
| F3 | `Item.prototype.create_to_point` 🔗 / `create_to_unit` 🔗 | `@common/base/item` | `(link/id, point, scene)` / `(id, target)`【反查推测】 | 【dump 实锤】（名） | 物品=带 `sys_item_*` 属性的单位，地面上可见——物编 link 创建 |
| F4 | `base.create_item_on_point` 🔗🅢 / `base.create_item_on_unit` 🔗🅢 | `@lua_plus/.../item` | `(id, target)` | 【源码实锤】+【dump 实锤】 | F3 触发器封装 |
| F5 | `Unit.prototype.ride_on` / `ride_off` / `get_ride_unit` / `get_rider_unit` 🅢 | `@common/base/unit` | 签名未知 | 【dump 实锤】 | 骑乘=模型组合渲染 |
| F6 | `base.eff.EffectUnitApplyMover` / `EffectUnitAddForce` / `EffectUnitRemoveForce` 等 🅢 | `@common/base/eff` | `(?)` | 【dump 实锤】 | 运动器（mover 视觉表现间接） |
| F7 | `base.game.mover_function` / `mover_line` / `mover_target` 🅢 | `@common/base/game/game` | — | 【dump 实锤】 | 运动器注册 |
| F8 | `base.event.on_unit_model_changed(id, path)` 🔗 | `@common/base/unit`（script） | `(id, path)` | 【源码实锤】 | **模型变更事件——双端联动的现成观测点**（服务端换模是否广播此事件，是判定同步链的关键证据） |

---

## 汇总

| 组 | 条目数 |
| --- | --- |
| A. 单位创建/模型换装 | 30 |
| B. actor 创建与附着 | 24 |
| C. 特效创建 | 8 |
| D. 场景/世界管理 | 18 |
| E. 资源加载/路径解析 | 10 |
| F. 其他可疑渲染入口 | 8 |
| **合计** | **98** |

- 🔗 接受路径/link/名称参数的条目：A1~A8、A10、A13~A17、A19、A20、A23、A24、A26、A27、A29、A30、B1、B2、B7~B11、B13~B18、B23、C1~C6、D4、D7、D10、D11、D15、D16、E1~E4、E6、F3、F4、F8。
- 🅢 服务端独有面（双端联动实验重点）：全部 lua_plus `base.*` 扁平函数（A、B、C、D 各组标注者）+ `base.game` 124 函数中的场景/单位初始化族（D1~D8）+ dump-only 引擎注册函数（`base.stop_actor`/`play_actor`/`unit_update_actor`/`scene_object`/`player_jump_scene_object`）+ `defaultui.NewActor` 体系 + `Unit.prototype.set_model/model_swap_*/set_asset/set_particle` 换模族（A14~A23）。
- 已有【实测】可直接引用的结论：仅客户端侧（render-06：真实单位 + prefab 裸路径换模可行；render-17：客户端创建必须有已注册数编 link；render-21：set_asset 裸路径判死、静默 miss）。**服务端渲染面当前零实测**，以上候选全部待双端联动验证。

## 优先实测顺序（Top 10，按「语义上最可能接受裸路径/运行时新资源」排序）

1. **A14 `Unit:set_model(model)`** —— 对标 render-06 客户端 `unit_change_model` 已实测吃 prefab 裸路径；服务端同名语义入口若能吃裸路径且广播，直载即通。
2. **A16 `Unit:model_swap_push(model)` / `model_swap_pop()`** —— 换模栈语义=官方运行时换装通道，参数形态最可能为路径/link。
3. **A19 `Unit:set_asset(asset)`** —— 对标 render-21 分派链（wrapper 零校验）；客户端 GameUnit 覆写未捕获，服务端实测可补齐。
4. **B1 `base.create_actor_at(name, point, use_terrain)`** —— 触发器层建表现正门；`name` 吃裸路径/虚拟 link 直接决定免数编可行性。
5. **B17/B18 `Actor:set_asset(asset)` / `base.actor_set_asset_model`** —— 已建 actor 换资产；配合 B1 先建「种子 actor」再换。
6. **B14 `EffectParam:create_actor(link, position, force_no_sync)`** —— 效果树内建 actor；`force_no_sync` 暗示可控同步行为。
7. **C2/C3 `base.player_create_lightning` / `base.unit_create_lightning`(model, ...)** —— `model` 参数是闪电模型 link/路径，免数编疑点最高。
8. **B2 `base.actor(name, sid, skip_birth, scene)`** —— script 层构造入口，绕过触发器封装直探 `name` 解析。
9. **A23 `base.crop.__index.set_model_unit` / `fresh_unit_model`** —— 服务端官方业务里现成的「运行时换单位模型」实现，可读其行为反推参数形态。
10. **A15 `base.unit_set_model(unit, model)`** —— A14 的 lua_plus 触发器封装（签名双证最齐），与 A14 交叉验证用。

> 实验编排建议：先在服务端 PIE 用真实玩家英雄（正 id 单位）逐候选调用 + 客户端同步观察（截图/日志/`on_unit_model_changed` 事件 F8），每个候选用「坏路径 / 数编 link / pak 内裸路径 / pak 外项目路径」四档参数矩阵——复用 render-06 的实测矩阵范式与 render-21 的静默 miss 教训（无报错≠生效，必须视觉或 getter 复读确认）。
