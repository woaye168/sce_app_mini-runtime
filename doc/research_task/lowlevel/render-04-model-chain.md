# 本地模型通道完整溯源：数编注册链 + actor/set_asset 实测

> 研究日期：2026-08-24 | 状态：链路全通；自定义模型可行路径锁定（数编条目 = 可脚本化的 ini）
> 前置：render-01/02/03

## 0. 一句话结论

模型的唯一渲染通道是**世界内 actor**（`base.actor(link)` / `game.create_actor` / `set_asset`），而 actor/model 的 link 必须存在于 **native 数编注册表**——该注册表只在装载时建立，运行时的任何 lua 侧注入（eff.merge_cache / get_game_table / require 的 obj 模块表 / base.table）都**不会被 native 采纳**（全部实测证伪）。数编条目本体 = 项目 `editor/table/entry_data/<类型>/<$$link>/entry_data.ini`（纯文本 ini，可脚本生成）→ 编辑器编译 → `script/obj/*.lua` → 装载进 native。model 条目的 `Asset` 字段**直接写 prefab 相对路径**（如 `characters/_user/jilulu_19ec/model.prefab`），本地模型文件就这么进——"数据编辑器创建"可以脚本化绕过。

## 1. 实测序列（test_res002 PIE，截图存证）

| 实验 | 结果 |
| --- | --- |
| `base.actor('$$default_units_ts.actor.脚底假阴影.root')` + set_position/scale | ✅ actor 生成并渲染（灰色渐变圆=假阴影平面，capture_1787504386） |
| `actor:set_asset('$$spark_core.actor.GeneralBuildGrid.Model')`（既有数编 link） | ✅ 换装成红色网格平面（capture_1787504563/4770）——set_asset 通道本身可用 |
| set_asset 直接传 prefab 路径（`characters/_user/jilulu_19ec/model.prefab`，已修正为真实可解析路径） | ❌ 保持原样——**不认裸路径** |
| 运行时 forge 条目进 eff.merge_cache | ❌ create_actor 返回 nil |
| 注入 `game.get_game_table('ActorModelData'/'ActorData')` | ❌（且 ActorModelData 返回**空表**——native 注册表不经此暴露） |
| 注入 `require('@@.obj.model.model')` / `obj.actor.actor` 返回表 + base.table 副本 | ❌ |
| 写入项目 `script/obj/model/model.lua`/`actor.lua`（文件级） | ❌ 装载时**被编辑器 trigger 管线重新生成覆盖**（暂存目录里我们的条目消失）——obj/*.lua 是产物不是源 |
| `game.GetTexture` | 任何入参返回 string '浅草'（非纹理 userdata；默认纹理名？），自定义纹理通道未通 |
| scene 控件（base.ui/imgui/independent 全试） | ❌ StateGame 不渲染 |
| `base.local_player():get_hero()` | 恒 nil（本项目玩家非引擎单位；unit:change_model/attach_model 无法在本项目测） |

**相机**：`game.GetCamera()` = `{position={x,y,z}, rotation={roll,pitch,yaw}, camera_node_position, focus_distance}`；`game.set_camera{position=, rotation=, focus_distance=, time=ms}`（camera.lua:122）。本项目相机 position=[3325,3325,10] rotation=[-70,-7.16,...]，注视点≈(3329,3325,0)。3D 视口无地形（纯天空盒），actor 放注视点即可见。

## 2. 数编注册链（每一环都有实证）

```
编辑器内存数编
  ⇅ 持久化：editor/table/entry_data/<类型>/<$$命名空间.link>/entry_data.ini + ui_data.ini [+ i18n/]
  ↓ 编辑器 trigger 管线（tstl 编译，调试启动时 generate_lua_only）
script/obj/<类型>.lua（entry_datas['$$link']= {...} return 表）  ← 产物！手改会被覆盖
  ↓ 装载（require '@@.obj.*' / init_cache）
lua 侧：base.eff cache（caches.dict）、base.table.*（get_game_table + obj 模块合并）
native 侧：内部注册表（create_actor/set_actor_asset 只查这里；get_game_table 部分表为空=不经此暴露）
```

- 既有可用 link 来源：项目引用的**预编译库**（default_units_ts/spark_core 等，数千条目：雷神/脚底假阴影/GeneralBuildGrid…）。test_res002 自身数编只有 camera/map_config/gui 模板。
- `eff.merge_cache` 的官方用途是合并**lua 侧**逻辑数据（服务端效果树执行用），对 native 渲染注册无效。

## 3. 本地模型落地指南（当前可行）

1. 资产准备：`<名>/model.prefab`（JSON：skeletalMesh 指向 `<名>/model/m.mdl`、animationSet、materials、sockets…）+ `model/m.mdl` + `anim/*.ani` + `materials/*.material`，放项目 `res/characters/_user/<名>/`（或任何引擎资源根可解析位置，参照基座资产 `D:\sce_online\Res\characters\_user\jilulu_19ec\`）。fbx/gltf→m.mdl 转换是编辑器导入管线做的，**离线转换器 = 独立后续课题**。
2. 数编条目（脚本化）：在 `editor/table/entry_data/model/$$<命名空间>.<名>.Model/` 写 `entry_data.ini`（格式=现存任意 model 条目 ini 的字段集；本项目无样例，可在数据编辑器手工建一个后读取其 ini 作模板——一次性），actor 条目同理（Model 字段指过去）。
3. 重载项目（编辑器重开/重载）使 trigger 编译进 obj/*.lua。
4. 运行时使用：
   - 生成：`local a = base.actor('$$<ns>.<名>.root')` → set_position/set_scale/play
   - 换装：`actor:set_asset('$$<ns>.<名>.Model')`
   - 动画：actor_play_anim/actor_play 等（game.* 442696-443310 全套）
5. ⚠️ 线上（pak 投递）资源必须进包：res/ 下资产随地图 pak 走（bgd 构建 res 同步只覆盖五类 UI 资源，characters 不在其中——**自行发布前需验证 characters 目录是否进 pak**；编辑器发布管线的资源收集规则待查）。

## 4. 遗留问题（下一步）

- entry_data.ini 的 model/actor 条目字段模板（在数据编辑器手工建一条后读 ini 固化模板；或逆向 GameData dll 的序列化）。
- unit:change_model/attach_model 实测（需含引擎单位的对局环境；test_res002 玩家非引擎单位）。
- characters 等 3D 资产的发布打包规则（publish 是否收进 pak）。
- GameWorld+viewport（任意 3D 场景→UI 区域，小地图同款）未试。
- 服务端 actor（sync=true 的服务端表现）链路未涉及。

## 5. 坑沉淀

- restart_last_debug/full debug 都会用编辑器内存数编**重新生成** `script/obj/*.lua`——手改 obj 文件无效且会被静默还原（git 也看不出改动）。
- PIE 客户端 native 日志不落本地；资源加载失败静默——只能靠截图 + 变体对照。
- `game.get_game_table('ActorModelData')` 返回空表不代表无数编——native 注册表不经此暴露。
