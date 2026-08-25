# GameWorld+viewport（UIWorld）攻坚：全矩阵实证 + 崩溃归因

> 研究日期：2026-08-24 | 状态：通道组件逐个定性；显示侧剩唯一路径（数编 UIScene 模板）
> 前置：render-03（scene 控件死亡旧结论）/ render-11（pak 资源）
> 探针：test_res002 GameWorldProbe.lua（保留可复用，带 ENABLE_G1/G2/G3 隔离开关）

## 0. 一句话结论

**GameWorld（SCE.GameWorld / defaultui.UIWorld）在 StateGame 可创建、可加载 actor、可渲染到 RT 纹理（不崩）**；但「把 RT 显示到控件」这最后一环：image 控件吃 RT 链接**硬崩（整编辑器+游戏无 dump 消失）**，scene 控件吃任何变体**不渲染**——唯一剩余官方显示路径 = **数编 UIScene 模板控件 + defaultui.UIWorld:BindToUIScene**（生产游戏 p_2xgc 排行榜实证在用的路径）。

## 1. 官方组件栈（defaultui_63 uiworldscript.lua 全解 + 生产用法 p_2xgc）

```
defaultui.UIWorld:Create(useShadow, 数编camera_link, scene名?)
  = SCE.GameWorld:new() + create_scene + set_camera_info + (load_map)
world:SetCameraPosition/Rotation(x,y,z)     -- 免数编镜头（数编 cache 为空时 SetCamera 静默跳过）
world:SetViewSize(w,h)                       -- setup_viewport(w,h,render_type=75|66)
world:CreateActor(数编actor_link)            -- ActorModel/ActorAdditionModel/ActorBeam/ActorEffect/ActorMaterial
world:CreateUnit(数编unit_link)              -- SCE.GameUnit.new
world.innerWorld:set_render_target_link('RT:xxx')  -- 世界渲染到命名 RT 纹理
world:BindToUIScene(ui场景控件)               -- uiScene.RenderTarget='RT:xxx' + set_render_path
```

- 生产实证（game_p_2xgc 排行榜）：数编 UIScene 控件 + `UIWorld:Create(false, '$$p_2xgc.camera_property.排行榜镜头.root', 'new_scene_paihang')` + CreateActor 摆场景点。
- UIScene 模板定义（libs_components.lua）：`com_name='UIScene'`，模板字段 RenderPath='EngineRes/RenderPaths/GameSnapshot.xml'（枚举仅 GameSnapshot/CEMap）、UseShadow。
- **组件实现不在任何解密镜像里**（gameui pak 的 TS 组件框架内部）——RenderTarget setter 的 native 映射不可见，裸控件无法复刻。

## 2. 崩溃归因（隔离矩阵，PIE StateGame）

| 组 | 内容 | 结果 |
| --- | --- | --- |
| 全矩阵（G1+G2+G3） | 首轮 | **整编辑器+游戏进程无 dump 消失**（日志断在首帧渲染） |
| G1-only | scene 控件 + resource=数编 actor link + independent | 存活，**不渲染** |
| G2-only | 2 只 UIWorld（camera/actor/RT link 全套） | 存活，世界正常渲染到 RT |
| G1+G2+G4 | + scene 控件 set resource/render_target/RenderTarget='RT:gw_probe' | 存活，**不渲染** |
| G3 | image 控件 image='RT:gw_probe_img' | **崩溃源**（单独定性：image 吃 RT 链接硬崩） |

**新坑固化**：RT 命名纹理（'RT:xxx'）只能被 UIScene 类控件消费；**喂给普通 image 控件 = native 硬崩无 dump**（与 canvas_texture PCBox 崩同款静默）。生产慎用任何非官方路径消费 RT。

## 3. scene 控件属性表补全（sceengine-strings 452291+）

independent / camera_info / buff / rotation_ue / rotation_qua / zoom / scale3D / anim_fade_time / can_edit / **knead_human（捏人）** / part_name / part_cloth / avatar_path / icon_path / ambient_color / lightgroup / resource / skin_name / custom_bounds / bounds——scene 控件定位 = **角色/模型预览专用**（捏人系统），直驱全属性变体在 StateGame 均不渲染（死亡确认，render-03 结论加固）。

## 4. 环境变量（StateGame lua 全局）

- `defaultui`（table，含 UIWorld）✅ 游戏可用（依赖库加载即注册全局）。
- `ImportSCEContext`（function）✅ 存在；`SCE` 全局 = nil（SCE.GameWorld 只能经 defaultui.UIWorld 触达）。
- camera link 用 test_res002 自带 `$$.camera_property.camerapro.root`（cache 无 init_position → SetCamera 静默跳过，须手动 SetCameraPosition/Rotation）。

## 5. 下轮唯一路径（前置全齐）

**数编 UIScene 模板控件脚本化创建 + BindToUIScene**：
1. 用 render-10 的 entry_data.ini 脚本化方法，在 test_res002 数编 GUI（template@gui_ctrl / 界面文件）里加一个 UIScene 控件实例（模板 `$$gameui.template@gui_ctrl.UIScene`）。
2. lua：`local ctrl = <拿到控件>; local world = defaultui.UIWorld:Create(false, CAM); world:BindToUIScene(ctrl); world:CreateActor(...)`。
3. 验证 RT 画面（吉鲁鲁 actor）上屏。线上 tester 复验（PCBox 构建差异风险：RT 渲染管线）。
4. 若通 = **世界内 3D 渲染进 UI 的官方通道全通**（模型/特效/场景快照自由），补上 render 主线最后一块。
