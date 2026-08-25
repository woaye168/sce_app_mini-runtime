# render-17 — ★★ 动态渲染深化：特效进 UIWorld + 项目包内自定义 renderpath + SCE native API 目录 + 免数编攻坚（G24~G30）

> 研究日期：2026-08-24~25 | 状态：✅ PIE 视觉实证（特效横幅 capture_1787573079；红底 renderpath capture_1787572683；手建 actor 双吉鲁鲁 capture_1787575007；set_asset 换哪吒 capture_1787579397；G30 特效种子横幅 capture_1787612431）
> 前置：render-14（UIScene/UIWorld 通道）、render-16（scene 控件判死）、用户灵感（G17 运行时构造 + Urho3D renderpath 项目包内 xml）
> 探针：test_res002 GameWorldProbe.lua G18~G30；数编条目 editor/table/entry_data/actor/bgd_demo_effect；自定义管线 res/renderpaths/bgd_snapshot_red.xml

## 0. 一句话结论

1. **特效进 UI 的官方通道打通**：数编 ActorEffect 条目（脚本化写 entry_data.ini，模板见 §1）→ `UIWorld:CreateActor('$$<ns>.actor.<名>.root')` → 特效在 UIScene 内渲染（紫色横幅实证）。
2. **★ 渲染管线动态化突破**：UIScene 的 `RenderPath` 属性**吃项目包内路径**（`res/renderpaths/xxx.xml`）——自定义 Urho3D renderpath XML（改 clear 颜色为实证）随项目分发、运行时 G17 页面直接引用生效。官方 45 个管线 XML（`C:\Users\woaye\Desktop\out\engineres\renderpaths\`，Urho3D 命令：clear/scenepass/forwardlights/planershadow/volumetriclight/renderui_back/front/sendevent/postprocess）全部可作为改写基底。
3. **SCE native API 面 dump**（`ImportSCEContext(nil)`）：`ModelActor.set_asset` / `EffectActor.set_asset`、`GameWorld.load_map` / `set_map_dir`、`use_light_group`——G24~G30 已实测（§6）。
4. **免数编攻坚三连实证（G27/G28/G29/G30）**：① 手建 actor 通道活（`SCE.ModelActor.new(有效link)` + `innerWorld:add_game_actor` + `show(true)` 渲染成功）；② **创建必须有 native 已注册数编 link**——假 link/无参/虚拟 merge_cache 条目全部 nil；③ **set_asset 换资产也必须数编表 ID**（模型侧用户复验修正；特效侧裸路径实测无效）——「一颗种子 + set_asset 换任意资产」路线同样卡在数编注册。

## 1. 特效进 UIWorld（G21，视觉实证）

### 数编 ActorEffect 条目模板（脚本化，render-10 同款流程）

`editor/table/entry_data/actor/<条目名>/entry_data.ini`：

```lua
[#CONFIG]
'Version' = 14

['Particle_1']                       -- 内嵌粒子子节点（link = $$<ns>.actor.<名>.Particle_1）
'Version' = 1
'NodeType' = '$$.particle.Particle'
'Inherit' = '$$.template@particle.Particle.root'
'Data' = {
    'Editor' = {},
    'Game' = {
        'Name' = '粒子节点_1',
        'Asset' = 'res/effect/bgd_libs_client/demo/p_12sc_effect_new_6o1_dl47/particle.effect',  -- pak 感知特效路径
    },
}
'UIConfig' = {}

['root']
'Version' = 1
'NodeType' = '$$.actor.ActorEffect'  -- ★ 关键：NodeType=ActorEffect → UIWorld GetActorFactory 路由到 SCE.EffectActor
'Inherit' = '$$.template@actor.ActorEffect.root'
'Data' = {
    'Editor' = { 'CollectRes' = true },
    'Game' = {
        'Name' = 'bgd演示特效',
        'Effect' = 'Particle_1',     -- 引用内嵌子节点
        'KillOnFinish' = 0,          -- 持续型特效不自杀
        'KillOnDeactivate' = 0,
    },
}
'UIConfig' = {}
```

- 生效流程同 render-10 §2（关编辑器写 ini → 删两个 obj save_info.json → bump editor/table 戳 → 重开编辑器 → full 调试）。
- 模板出处：xdeditor-169 `obj_editor_v2/config/entry_data/template@actor/#actor#effect/entry_data.ini`（TNND，已解密存 test/temp/tpl_actor_effect.ini）；字段全集参照 game_p_1ax1 编译产物 `obj/effect/actor/data.lua`（Effect/SocketName/Offset/Rotation/FollowRotation/EventCreation/EventDestruction/AnimTrail/PlaySpeed/Scale/AutoScale/ShowShadow/CreationFilter...）。
- **播放语义**：`CreateActor` 返回即自动播放（G21 未调 play 已显示；`ea:play('cast')` 亦不报错）。位置 `set_position(x,y,z)` 世界坐标。
- **UIWorld:GetActorFactory 路由表**（defaultui_63 uiworldscript.lua:277-305，读 `base.eff.cache(link).NodeType`）：ActorModel→SCE.ModelActor / ActorAdditionModel→SCE.AdditionModelActor / ActorBeam→SCE.BeamActor / **ActorEffect→SCE.EffectActor** / ActorMaterial→SCE.MaterialActor；**particle link 直传被拒**（G19 实证报错"UI场景不支持该表现类型：Particle"）——必须包一层 ActorEffect 数编条目。

## 2. ★ 自定义 renderpath（G22，视觉实证）

- UIScene 组件属性 `RenderPath` 是 **pak 感知路径**：官方值 `EngineRes/RenderPaths/GameSnapshot.xml`（半透明混合，render_type=66）/ `EngineRes/RenderPaths/CEMap.xml`（不透明，render_type=75，BindToUIScene 内部按是否 GameSnapshot 二选）。
- **项目包内路径实证可用**：`res/renderpaths/bgd_snapshot_red.xml`（gamesnapshot.xml 拷贝、clear 改 `1 0 0 1`）→ G17 运行时页面引用 → **红底 + 吉鲁鲁 + 地形正常渲染**。即：渲染管线 XML 可作为项目资产分发并在运行时动态应用。
- 官方管线库：`C:\Users\woaye\Desktop\out\engineres\renderpaths\` 45 个（cemap 系 12、editor 系 11、forward/prepass/deferred/pbrdeferred 系、autochess、gamesnapshot、gameplayui(_clear)、gameoverlay、modeltest、particleedit/test、rendertotexture(_nogamma)）。
- XML 结构（Urho3D renderpath）：`<renderpath finalCopy substitute zPrepass>` + command 序列（clear/staticmeshpredraw/scenepass(depth/base/light/postopaque/alphanoblend/xray/alpha/planershadow/postalpha/outsurface/innerstroke/outstroke)/forwardlights/volumetriclight/renderui_back/renderui_front/sendevent）+ `<postprocess name="EngineRes/PostProcess/...xml">`。`cemapunderui.xml` 揭示 UIScene 合成机制：`renderui_back` → `sendevent "UIScene"` → postprocess → `renderui_front`。
- 生产想象空间：自定义 clear/雾色、开关 XRay/描边 pass、换 postprocess 链（bloomHDR/tonemap）、PBR/deferred 管线进 UI、rendertotexture 离屏玩法。

## 3. SCE native API 目录（G23 dump，`ImportSCEContext(nil)`）★

顶层：`AdditionModelActor, Animatable, BeamActor, ClassMap, EffectActor, GameActor, GameUnit, GameWorld, GetGameWorldInfos, MaterialActor, MiniMapIconProxy, MiniMapProxy, ModelActor, ReleaseRenderTargetLink, RuntimeLuaLifetimeObject`

| 类 | 方法（__index） |
| --- | --- |
| GameWorld | create_scene / **load_map** / **set_map_dir** / setup_viewport / destroy_viewport / set_render_path / set_render_target_link / set_camera_info / **use_light_group** / add_game_actor / remove_game_actor / add_game_unit / remove_game_unit / purge / __create / __release |
| ModelActor | **set_asset** / **get_mesh_asset** / **set_mesh_asset_material** / play / pause / resume / stop / is_playing / attach_to / detach / set_position / set_rotation / set_scale / show / __create / __release |
| GameUnit | 同 ModelActor（含 set_asset/get_mesh_asset/set_mesh_asset_material） |
| EffectActor | **set_asset** / play / pause / resume / stop / attach_to / detach / set_position / set_rotation / set_scale / show（无 get_mesh_asset） |
| BeamActor / MaterialActor / AdditionModelActor | play/pause/resume/stop/attach_to/detach/set_position/set_rotation/set_scale/show（Beam/Material 无 set_asset） |

- UIWorld 实例字段：innerWorld / renderLink（'RT:table:...'）/ position / rotation / UIScene / IsValid；方法：BindToUIScene / UnBindFromUIScene / CreateActor / CreateUnit / SetCamera(Position/Rotation) / SetViewSize / UseLightGroup / Destroy。
- innerWorld 实例字段：`__cinstance:userdata, __cname, render_type:number`。

### 下轮假设（G24，实测结果见 §6）

1. `SCE.ModelActor.new(假link或无参)` + `set_asset('characters/_user/jilulu_19ec/model.prefab')` → **免数编直载本地模型**。
2. `SCE.EffectActor.new(...)` + `set_asset('res/effect/.../x.effect')` → **免数编直载特效**。
3. `innerWorld:load_map(...)` / `set_map_dir(...)` → 运行时动态换图/载自定义场景。
4. `use_light_group(...)` → UIScene 世界光照控制（scene 控件 light 判死后的光照答案）。

## 4. 附带实证（G18/G20，scene 控件收尾，详见 render-16 §6）

- scene 控件 `independent=true` 无 color 时**永远纯黑方块**（正/负 name 对照无差异；light 官方写法/过曝 {10,10,10}/ambient_color/zoom 全无效）→ **light 游戏态彻底判死，scene model 通道无生产价值**，模型预览走 UIWorld（光照由世界灯光正常提供）。
- G18f 动态 bind（创建后写 anim/scale3D）不报错但效果不可见（黑底无法判别）——动态 bind 有效性悬置，无生产意义不再追。

## 5. 坑与备注

- G19：UIWorld:CreateActor 直传 particle link 报 `UI场景不支持该表现类型：Particle $$lib_control.particle...`（defaultui/uiworldscript.lua:304 log.error，返回 nil）。
- ActorEffect 的 `Effect` 子节点引用写**子节点 ID**（'Particle_1'），不是全 link。
- 数编 Asset 字段用 pak 感知路径（`res/effect/...`），与 render-02 UI particle 控件同规则。
- 用户新坑 13 备忘：不要直接改项目 `script/`、`ui/script/`（构建管线覆盖）——探针一律写 `.bgd/src` + 运行时构造（G17 流）。

## 6. 免数编攻坚实测（G24~G30，2026-08-25）★★

主线问题：**能否完全不经过数编表直接加载资源/创建单位**（用户指令）。结论：**三条绕行路线全部卡在同一道墙——native 数编注册表运行时只读**。

### G24/G25：创建必须有 native 已注册 link（判死免数编创建）

- `SCE.ModelActor.new()` 无参 = nil；`new('')` / `new('$$p_55a3.actor.bgd_seed_void.root')`（伪造）/ `new('x')` 三种假种子**全部 nil**（G25c）。
- native strings 有 `[CreateActor:%d,%s] failed, GetActorTableEntry failed.`（:538686）——创建走 native `GetActorTableEntry` 查注册表，查不到即 nil。
- 注册表类族（strings :576211-576240）：ActorTableEntry / ActorModelTableEntry / ActorEffectTableEntry / UnitTableEntry 等；**无 RegisterEntry / LoadTable / ReloadTable 任何运行时注册 API**。

### G27：手建 actor 通道活（种子 link 必须有，但不必走 UIWorld:CreateActor 包装）

```lua
local SCE = ImportSCEContext(nil)
local ma = SCE.ModelActor.new('$$p_55a3.actor.bgd_jilulu_attach.root')  -- 有效数编 link
world3.innerWorld:add_game_actor(ma)
ma:set_position({ -150, 0, 0 })
ma:show(true)
ma:play('Idle')
```

→ 第二只吉鲁鲁渲染成功（截图 capture_1787575007）。G24/G25 手建不可见的真实原因是**位置出画**（±150 在相机 dist=300 视野外；G26 放 (0,0,120) 与原点吉鲁鲁投影重叠属设计失误）。

### G28：set_asset 换模型——必须数编表 ID（用户复验修正）

- `ma:set_asset('$$p_55a3.model.nezha.root')`（数编 model link）→ 吉鲁鲁种子真换哪吒（截图 capture_1787579397）。
- **裸 prefab 路径不渲染**（用户复验判定，已将探针改为数编 ID、原路径行注释保留）。
- 推论：set_asset 不是「直读文件」，仍走数编/资产注册解析。

### G29：运行时虚拟数编（merge_cache）——lua 层成功、native 层失败 ★

用户共享实现（doc/research/lowlevel/virtual_effect.lua + get_eff_cache.lua）模式：`base.eff.cache(模板link)` → deepcopy → 改 Name/Link → `base.eff.merge_cache({dict={[新link]=新条目}})`。

G29b 实测（深拷贝 jilulu_attach root+Model 子节点，注册 `$$p_55a3.actor.bgd_virtual_nazha.root`）：

- lua 侧 `base.eff.cache(VROOT)` 读回 ok（NodeType=ActorModel）；
- `world3:CreateActor(VROOT)` = **nil**。
- 根因（script-199 `common/base/eff.lua:155`）：merge_cache 只写 lua 层 `caches.dict[key]=value`，**native 注册表不同步**；native 注册表启动时建立、运行时只读。
- 即：虚拟数编只对「lua 层消费 base.eff.cache 的 API」有效；凡是 native 按 link 查表的入口（CreateActor / *.new / set_asset）一律不认。

### G30：EffectActor 特效 set_asset——裸路径无效（模型侧结论特效侧复证）

- 坑：手建 `SCE.EffectActor.new(link)` + `add_game_actor` + `show(true)` **不渲染**——特效需 `play('cast')` 触发（条目 EventCreation=on_cast_start）；改走 G21 路径（`world3:CreateActor` + `play('cast')`）种子横幅正常渲染（截图 capture_1787612431）。
- `ea:set_asset('res/effect/_user/uitexiao3_a4wc/particle.effect')` 与 `set_asset('effect/_user/uitexiao3_a4wc/particle.effect')`（去 res/ 前缀）两种形态 + 重 play('cast')：**均无报错、画面不变**（横幅依旧，capture_1787612499/1787612573）→ 特效侧裸路径 set_asset 无效。

### 机制结论与下一步方向

```
创建/换资产路径全图（已实证）：
  UIWorld:CreateActor(link) / SCE.*.new(link)  ──► native GetActorTableEntry(link)  ──► 注册表（启动时定，运行时只读）
  set_asset(x)                                  ──► x 必须也是已注册数编 link（裸路径无效）
  base.eff.merge_cache                          ──► 只到 lua caches.dict，穿透不到 native
```

下一步候选（按优先级）：

1. **frida 逆向 native 注册表**：找 GetActorTableEntry 背后的容器（TableEntry 类族），看是否有可写的运行时插入口或注册函数（哪怕未导出 lua）；也可 hook set_asset 看它如何解析参数（确认裸路径失败点）。
2. **GameWorld.load_map / set_map_dir 语义探明**（G24d 调用无报错但未验证效果）——若能在 G17 世界舞台上动态载图，地图侧可绕数编。
3. **大厅动态加载机制反查**：大厅能动态加载渲染其他游戏的模型/地图，其资源加载链必然有免数编入口（或有大厅专属数编下发机制）——从 lobby 源码/抓包入手。
4. `set_mesh_asset_material` / `get_mesh_asset` 语义（mesh 级直换资产的可能）。
5. `GetGameWorldInfos` 返回空表待查。
