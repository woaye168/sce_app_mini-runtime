# render-14 — ★★ UIScene 通道完整打通：3D 世界/本地模型渲染进 UI（页面脚本化 + BindToUIScene 视觉实证）

> 研究日期：2026-08-24 | 状态：✅ 完整打通并视觉实证（吉鲁鲁模型在 UI 控件内渲染，截图 capture_1787566974）；**2026-08-24 用户确认：编辑器与线上发布全部生效（UIWorld/UIScene 通道线上复验通过，含 characters/_user 本地模型线上可达）**
> 前置：render-13（GameWorld 渲 RT 不崩、显示侧卡死）；本轮闭合「显示侧唯一路径」
> 探针：test_res002 GameWorldProbe.lua G7~G16（全部保留可复跑）；手写页面 ui/script/gui/page/GWProbePage/

## 0. 一句话结论

**「3D 世界渲染进 UI」的官方通道完整走通**：手写 GUI 页面文件（免编辑器）内嵌 `gameui.UIScene` 组件 → `base.gui_new` 实例化 → `base.gui_get_part` 取控件 → `defaultui.UIWorld:Create(false, 镜头link, '场景名')` 载真实地图 → `CreateActor(数编actor link)` 摆本地模型 → `BindToUIScene(控件)` 上屏。**本地模型（characters/_user prefab 包装的 ActorModel 数编条目）在 UI 内清晰渲染**。
同时**修正 render-03/13 的"scene 控件 StateGame 死亡"结论**：scene 控件没死，`base.ui.scene` + `independent=true` 就能出画面（之前是没用对创建流程/缺关键属性）。

## 1. 完整可用配方（PIE 实证，逐行可复制）

```lua
-- ① 手写页面文件（免开编辑器，见 §2）后：
local page = base.gui_new('GWProbePage')               -- 实例化页面
local ctrl = base.gui_get_part(page, 'gw_scene')       -- 取 UIScene 组件实例（= page.part['gw_scene'][1]）

-- ② 建世界（第三参 = 场景名，载真实地图；可省=空世界）
local world = defaultui.UIWorld:Create(false, '$$.camera_property.camerapro.root', 'default')

-- ③ 相机对焦（数编镜头 cache 无 init_position 时 SetCamera 静默跳过，必须手动对焦）
local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 300)  -- 焦点(0,0,0) pitch-70 距离300
world:SetCameraPosition(pos[1], pos[2], pos[3])
world:SetCameraRotation(-70, 0, 0)

-- ④ 摆内容：数编 actor link（render-10 脚本化注册的本地模型条目）
local a = world:CreateActor('$$p_55a3.actor.bgd_jilulu_attach.root')
a:set_position(0, 0, 0)
a:play('Idle')

-- ⑤ 绑上屏（等一帧让控件拿到真实尺寸后再绑，否则 BindToUIScene 内部 rect 检查失败）
base.wait(200, function() world:BindToUIScene(ctrl) end)
```

实证截图：capture_1787566438（地图 terrain 进 UI）→ capture_1787566974（吉鲁鲁模型清晰渲染在控件区域）。

## 2. 手写 GUI 页面脚本化（免编辑器建页面，本轮新通道）★

比 render-10 的 entry_data.ini 数编脚本化**更直接**：GUI 页面本质就是 `ui/script/gui/page/<名>/` 下的两个 lua 文件 + `page/init.lua` 注册一行，手写即生效（restart_last_debug 可拾取，无需 full 调试/重开编辑器）：

- `page/GWProbePage/template.lua`：`gui_pkg.page_template { flatten_template = { ctrl_wrapper.panel{...},0, gameui.UIScene{ RenderPath='EngineRes/RenderPaths/GameSnapshot.xml', layout={...position={x,y},width,height}, name='gw_scene', show=true },1, } }`（flatten_template 每项 = 控件模板 + 父索引，0=根）。
- `page/GWProbePage/component.lua`：`component 'GWProbePage' { pkg.require_template(lib_env,'GWProbePage'), event={}, prop={}, method={}, state={} }`。
- `page/init.lua`：`kPageNames` 数组加 `'GWProbePage'`。
- 控件绝对定位写法：`layout = { col_self='start', row_self='start', position={x,y}, width=.., height=.. }`（仿生产模板，勿用 position_type——那是 base.ui 直建流的字段）。
- 文件头标 AUTO-GENERATED 只是约定，运行期直接加载磁盘文件；**GUI 编辑器保存页面时会覆盖/清理手写页面**（研究用无碍，生产用须走编辑器或接受被清）。

## 3. 关键机制破解记录

### 3.1 base.ui.scene 官方流（修正旧结论）★
- 官方用法在 **script-199 `common/test/scene.lua` / `particle.lua` / `test_ac.lua` / `scenes.lua`**（引擎组自测文件）：
  ```lua
  local tpl = base.ui.scene { show=true, layout={...},
      model    = { name='斧王', facing=0, position={0,0,0}, pause=true, anim='Idle', anim_time=0.1, scale=0.5 },
      particle = { name='剑刃风暴' }, }   -- 特效进 scene 控件
  local ui, bind = base.ui.create(tpl, '页面名')
  ```
- **`base.ui.scene(props)` 只建模板，`base.ui.create(tpl, name)` 才实例化**——render-13 及本轮 G6 只建模板没 create，是"scene 不渲染"的根因之一。
- **`independent = true` 是出画面关键**（G8a 实证：无 independent 完全不渲染，有则出视口画面）。
- scene 控件属性表：model/particle/camera_info/buff/light/independent/zoom/scale3D/rotation_ue/...（script-199 `common/base/template/scene.lua` = GUI 绑定层，native 接口是 `base.ui.gui.set_model/set_camera_info/set_particle/set_buff/set_light(id, {k=v})`）。
- 遗留：model.name 的解析规则未完全摸清（'主控'unit 名 / 'bgd吉鲁鲁附着模型' actor 名 / '默认动画预览模型' model 条目名均未显示模型——画面出但内容黑/空，光照 light 属性无效）。**预览单个模型更可靠的路径是 §1 的 UIWorld 通道**，scene 控件 model 通道留给下轮（可从捏人系统 knead_human 反查）。

### 3.2 UIScene 组件结构
- `@gameui.component`.UIScene 类：`__ui_type='UIScene'`（native 控件类型名），`require_url='@gameui.UIScene'`，meta_info_str='@gameui/uiscene.lua 39'，`__part_count=2`（根 panel + native UIScene part），组件源码物理位置未在 gameui 包散文件中找到（包内只有 png；**gameui 包的 lua 来源待查**——可能在 pak 或别包内）。
- **命名控件访问 = `base.gui_get_part(page, '控件名')`**（= `page.part[name][1]`；生产游戏 p_2xgc 排行榜实证，`page.gw_scene` 直取是 nil）。
- `base.ui.view { type='UIScene', name=... }` 可直建 native UIScene 控件（tostring 前缀 {UIScene|...}），但 `ui.set_control_prop(id, 'RenderTarget'/'render_target'/'image', 'RT:..')` **全部静默吞**（set_control_prop 无属性校验，错名不报错）+ 画面无变化——裸控件缺组件层的 RenderTarget 转发逻辑，**不要走这条路**。
- UIWorld 默认 renderLink = `'RT:' .. tostring(self)`（形如 'RT:table: 000001FE...'）。
- BindToUIScene 内部（defaultui uiworldscript.lua）：读 ctrl.ui:rect()（0 尺寸报错返回 false，**必须等一帧**）→ RenderPath=='GameSnapshot.xml' 时 render_type=66 否则 75 → SetViewSize(w,h) → set_render_path → `uiScene.RenderTarget = renderLink`（组件实例属性写，转发到 native part）→ set_render_target_link。
- RT 画面按 SetViewSize 的世界视口渲染，**可溢出控件边界显示**（实证：模型头部超出控件上缘）——生产注意 SetViewSize 与控件尺寸匹配（BindToUIScene 自动用控件 rect，已对齐）。

### 3.3 生产对照（game_p_2xgc 排行榜，镜像实证）
- 页面模板 `gameui.UIScene { RenderPath='EngineRes/RenderPaths/GameSnapshot.xml', ... name='排行榜UI场景' }`。
- 触发代码：`base.gui_get_part_as(UIScene类型, base.gui_get_main_page(), '排行榜UI场景')` → `defaultui.UIWorld:Create(false, '$$p_2xgc.camera_property.排行榜镜头.root', 'new_scene_paihang')` → actor set_position/set_rotation/play → `BindToUIScene`。
- RenderPath 枚举仅 GameSnapshot.xml / CEMap.xml（libs_components 模板定义）。

## 4. 本轮排障时间线（方法论）

1. G6 base.ui.scene 直建 → 无画面（只建模板没 create）。
2. 读 script-199 官方 test 文件 → G7 base.ui.create 流 → 仍无画面（缺 independent）。
3. G8 变体矩阵 → **independent=true 出暗画面**（缺光照/内容）；camera_info 字符串/表、zoom、ambient_color、lightgroup 全无效。
4. G10 cls dump → native 类型名='UIScene'；G11 手写页面 + gui_get_part → 组件实例拿到，Bind=true 无画面（世界没内容+相机没对焦）。
5. G12 裸 view{type='UIScene'} + 属性矩阵 → 全静默吞。
6. G14 **Create 第三参载真实场景 'default'** → 地图 terrain 进 UI（首个画面）。
7. G15 actor 放 (3325,3325) 出画（相机焦点在原点）→ G16 相机对焦原点 + actor 放原点 → **吉鲁鲁上屏**。
8. 坑：探针分组开关改 if 块时弄丢收尾 end（两次）——改开关矩阵代码后先 build 再跑（build 不过 lua 语法，但运行会黑盒）。

## 5. 遗留

- scene 控件 model/particle 的 name 解析规则 + 光照生效条件（捏人系统反查，knead_human）。
- '@gameui' 包的 lua 物理位置（uiscene.lua 组件源）——拿到可解 RenderTarget 转发细节 + 其他组件能力目录。
- ~~UIWorld 通道线上 tester 复验~~ → ✅ 用户实证（2026-08-24：编辑器与线上发布均生效）。
- ActorEffect 数编条目进 UIWorld（特效进 UI 的另一条路，模板 GetActorFactory 支持 EffectActor）。
- 手写页面通道的 GUI 编辑器兼容性（编辑器保存会清手写页；可考虑 bgd 工具化生成）。
