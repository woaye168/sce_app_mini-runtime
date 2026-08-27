# dl-05 特效/actor 创建双端联动（服务端创建 → 客户端观察）

> 日期：2026-08-28 | 状态：✅ 完成（PIE；结果为阴性 + 一个重要环境发现）
> 探针：`test_res002/.bgd/src/server/test/dl05_server_effect.lua`

## 结论

| 调用（服务端） | 返回 | 判定 |
| --- | --- | --- |
| `base.unit_get_point(hero)` | `{point|...|(1054.9, 1105.8, 0, default)}` | ✅ 取点正确姿势（`hero:get_position()` 不存在） |
| `base.create_actor_at('$$p_55a3.actor.bgd_demo_effect.root', point, true)` | nil | ❌ |
| `base.create_actor_at('$$p_55a3.actor.bgd_jilulu_attach.root', point, true)` | nil | ❌ |
| `base.create_actor_at(jilulu 裸路径, point, true)` | nil（走到 create_actor_at 内部才失败） | ❌ |
| `base.actor('$$p_55a3.actor.bgd_demo_effect.root')` | nil | ❌ |

## 根因（重要环境发现）

**render-10 时代手工数编的 `bgd_demo_effect` / `bgd_jilulu_attach` actor 条目已不在 obj 注册表中**——`script/obj/actor/actor.lua` 与 `ui/script/obj/actor/actor.lua` 均无 `bgd_` 前缀条目（render-10 已知「编辑器开着写条目再保存会被清理逻辑删除」，本次实测确认已被清理）。所以本轮 nil 是**条目不存在**，不是服务端通道本身判死。

- 服务端 `create_actor_at` 源码链（server_lua_plus actor.lua:9-27）：`base.actor(name, nil, nil, point:get_scene_name())` → script-199 actor.lua:57 `base.actor`：`base.eff.cache(name)` miss 即 nil → native `game.create_actor`。
- **特效双端联动的有效复测需要先有已注册特效 actor 条目**——两条路：① 重新走 render-10 数编脚本化流程注册条目（注意编辑器关闭态写入）；② 直接用现有注册条目（如 `$$default_units_ts.actor.*` 音效/特效族，或 lib_control 粒子）。
- 既有事实不受影响：render-10 客户端 `create_actor_at + attach_to` 通道当时视觉实证为真（条目当时存在）。

## 备注

- `hero:get_position()` 服务端不存在（第一批报错），正确取点 = `base.unit_get_point(hero)`（lua_plus）。
- 本项目玩家英雄 = 场景放置的 `$$default_units_ts.unit.星火战士.root`（id=1），相机/视野正常。
