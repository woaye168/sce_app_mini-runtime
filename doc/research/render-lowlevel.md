# 渲染底层通道全图：星火 1.0（api13/lua）渲染 API、免数编攻坚判死记录与星火 2.0（WasiCore）渲染面

> 主题：游戏 lua（StateGame）可用的全部渲染通道的能力边界、正确用法与 native 机制；「免数编注册」主线的完整攻防记录；星火 2.0（WasiCore）渲染 API 面与采用路径。
> 来源：合并自 `doc/research_task/lowlevel/render-01~22` + `wasicore-01/02/04`（25 份碎片文档）。
> 最后验证日期：2026-08-25
> 实证方式：PIE 实测探针（test_res002 RenderProbe/GameWorldProbe，截图存证）/ 二进制逆向（sceengine.dll api13 + managed dll）/ Frida hook（注册表/set_asset 动态验证）/ 真 tester 局线上实证（p_55a3 发布真局）。
> 探针现场：test_res002 `.bgd/src/client/RenderProbe.lua`（U 系列）、`.bgd/src/client/GameWorldProbe.lua`（G 系列，含 ENABLE_Gx 隔离开关）；手写页面 `ui/script/gui/page/GWProbePage/`。

## 1. 总览：渲染各通道能力矩阵

| 通道 | 用法一句话 | 编辑器 PIE | 线上（tester） | 出处/状态 |
| --- | --- | --- | --- | --- |
| 图像 | `ui.set_control_prop(id,'image',path)`；来源：相对路径/@图名/http(image_cache)/绝对路径 | ✅ | ✅ | 官方通道 |
| 图集子图（UV/源矩形） | **lua 无任何 UV 通道（已证伪）**；生产用 webview canvas2d > sprites 定格 > clip | — | — | §14 |
| UI 内 3D 世界/模型 | 手写 GUI 页面 + `gameui.UIScene` + `defaultui.UIWorld:Create` + `BindToUIScene` | ✅ | ✅（用户 E2E 实证） | §8 |
| scene 控件 model/particle | `base.ui.scene` + `base.ui.create` + `independent=true`；name=单位/特效表节点名；**light 游戏态判死 → 无生产价值** | ⚠️ 出图但恒黑 | 未验（不追） | §8.5 |
| 世界内模型换模 | `game.unit_change_model(真实单位正id, 'characters/_user/<名>/model.prefab')` | ✅ | 未单独复验（资产可达性已通） | §5 |
| actor 附着挂点 | 数编 ActorModel 条目 → `base.create_actor_at(link, point)` → `actor:attach_to(单位id, 'socket名')` | ✅ | 未单独复验 | §6 |
| `unit_attach_model` | **判死**（客户端全矩阵无渲染，frida 确认调用不到内层） | ❌ | — | §6.4 |
| UI 内特效 | particle 控件 `effect='<.effect 路径>'`（pak 感知）+ 数值类型属性 | ✅ | 同 StateGame 常规控件，风险低 | §4 |
| 特效进 UIWorld | 数编 ActorEffect 条目 → `UIWorld:CreateActor(link)`（particle link 直传被拒） | ✅ | 风险低 | §10.1 |
| spine | spine 控件 `resource='<.skel 无扩展名路径>'`（引擎自补 .skel） | ✅ | 风险低 | §4 |
| video | **imgui 每帧直驱** + `video_url='https://...mp4'`（自带 muted autoplay）；file:// 被拦 | ✅ | ✅（https mp4 真局实证） | §3 |
| webview / 任意 2D | **imgui 每帧直驱**（miniblink 离屏合成）；canvas2d = 自由 2D 绘制表面 | ✅ | ✅（canvas2d 真局实证） | §3 |
| base.ui 声明式建 webview/video | StateGame 不渲染（渲染驱动阀门；线上大厅管线驱动才活） | ❌ | ✅（大厅/base.ui+set_enabled_in_game 用户实证） | §3.5 |
| 世界内即时绘制 | `game.create_debug_draw_actor` + `debug_draw_line/point/circle/sector/rectangle/text` | ✅ | 官方调试通道 | §2 |
| 屏幕后处理/光照天气 | `game.set_screen_effect` / `use_light_group` / `set_scene_timer_postprocess` / `set_light_weather_attribute_animation` | ✅ | 官方通道 | §2 |
| 自定义渲染管线 | UIScene `RenderPath` 吃**项目包内路径**（`res/renderpaths/xxx.xml`） | ✅ | 未验（同 pak 资源，风险低） | §10.2 |
| 手建 actor（免 UIWorld 包装） | `SCE.ModelActor.new(有效数编link)` + `innerWorld:add_game_actor` + `show(true)` | ✅ | 风险低 | §10.3 |
| 换资产 `set_asset` | **参数必须也是已注册数编 link**（裸路径判死，模型/特效双侧） | ✅（数编 link） | 风险低 | §10.4 |
| 材质 | `set_mesh_asset_material` 内部走 ResourceCache 按名/路径取 Material，**不依赖数编** | ✅（逆向实证） | — | §11.6 |
| 纹理 userdata | `game.GetTexture` 返回 string（'浅草'），非纹理 userdata——自定义纹理通道未打通 | ❌ | — | §2.3 |
| 数编免注册创建/注入 | 全路线判死（merge_cache 虚拟数编 / 假 link / load_map 注入 / 裸路径 set_asset / 纯路径改写）；唯一剩余主线 = frida 运行时注入注册表 | ❌ | — | §11 |

**四大用户痛点终版答案**：① 特效本地文件直用 → particle 控件 .effect 直路径（属性必须原生 number/table 类型）；② tiled 图集 → webview canvas2d（线上已实证）；③ spine → .skel 直路径完全自由；④ 本地模型 → 世界内 `unit_change_model`（真实单位）/ UI 内 UIWorld+UIScene 通道（数编脚本化条目）；视频 → imgui video + http(s) mp4。

## 2. native 注册块与官方 API 边界

行号 = sceengine-strings.txt（api13 sceengine.dll）行号；tester 对应 scegame-tester-strings.txt。

### 2.1 游戏运行时 lua 注册块（StateGame 可用）

- **LuaGame `game.*`（442696-443310）——世界内渲染主力**：send_ui_message / get_game_table / get_player_list / cast_spell 系 / 坐标变换（world_to_screen/screen_to_xy/yz/xz/Raycast）/ **create_actor / remove_actor / set_actor_owner / set_actor_asset** / set_actor_shadow / socket 系 / **attach_actor_to_socket / attach_actor_to_anchor / detach_actor** / actor_play / actor_play_anim / actor_pause/resume/stop / actor_set_layer_mask 系 / actor_enable_raycast / 音效系 / set_actor_scale / grid actor 系 / launch/impact site 系 / set_actor_text / **set_actor_material_parameters** / actor_play_anim_bracket / 播放速度·时长·百分比·时间 / anim_handle_* 全套（442940-443011）/ **create_debug_draw_actor + debug_draw_line/point/circle/sector/rectangle/text + clear_debug_draws**（443014-443029）/ 战争迷雾与视野系 / CreateUnit / unit 位置·朝向·缩放 / unit_add_buff/remove_buff / riseletter 系 / 血条·小地图图标 / highlight/tint/xray/outstroke / circle·line·sector_selector / **shake_camera / set_camera / GetCamera / CameraFocus / lock_camera / camera_rotate_around_point / set_camera_attribute / switch_camera**（443181-443206）/ **set_screen_effect** / **use_light_group** / use_scene_timer / set_scene_timer_param / flow_rate / **set_light_weather_attribute_animation** / set_global_roughness_enable / **set_scene_timer_postprocess** / euler_to_quaternion / lerp_quat_align / set_background_anim / **unit_attach_model / unit_detach_model / unit_change_model**（443240-443245）/ set_static_mesh_predraw_enable / **GetTexture**（443250）/ set_max_particle_memory_limited / unit_register_bone_chain 系 / occluding_camera_group 系 / set_focus_clip_param / set_max_draw_distance / set_max_shadow_distance / 异步加载块系 / set_cipr。
- **LuaUI `ui.*`（443417-443571）**：含 set_control_prop / imgui_* / canvas_* / canvas_texture_*（canvas_texture 线上 PCBox 硬崩，弃）。
- **LuaUISound `ui_sound.*`（443582-443596）**：play_ui_sound/play_ui_sound_ex/play_sound/stop_sound/get_sound_position/is_playing/stop_all_sound。
- **LuaIO `io.*`（443603-443723）**：read/write/walk_dir/serialize/read_pak_entries/extract_pak/DownloadFile/UploadFile/unzip/add_resource_path 等（StateGame 多数被 isolation 置 nil）。
- LuaLog 443725-443817 / LuaLoader 443818-443832 / LuaCommon 443833-443879 / **LuaLobby 443880-444619**（含 `common.set_particle_lod_level`、`common.set_background_texture_path/uv`）/ LuaIM / LuaGlobalChat / LuaUpdater / LuaLogin / LuaJS / LuaTimer / **LuaScore `sce.s.*` 444851-444974** / **LuaMapPublisher `map_publisher.*`** / **LuaHttp `sce.httplib.*` 445042-445110**（request 全参数 + create/create_stream）/ LuaShortcut / LuaRandom。

### 2.2 类绑定块（LuaBinding/LuaExtend 区 445235-445340）

- `LUA_METATABLE_TEXTURE` + `set_data`（445236-445238）+ `game.GetTexture`（443250）：疑似自定义纹理 userdata 通道——**实测 GetTexture 任何入参返回 string '浅草'**（行为像「按路径查纹理名，未命中给默认名」），userdata 获取路径未找到，通道未打通。
- **EffectActor / AdditionModelActor / BeamActor**：set_attribute/get_attribute/set_asset/play/attach_to/detach/get_mesh_asset/set_mesh_asset_material（445308-445318）。
- **GameWorld**：set_map_dir/create_scene/purge/setup_viewport/destroy_viewport/add_game_unit/remove_game_unit/add_game_actor/remove_game_actor/set_render_target_link/set_camera_info（445320-445335）。
- MiniMapProxy/MiniMapIconProxy：is_valid/add_container_icon。

### 2.3 SCE native API 目录（`ImportSCEContext(nil)` dump 实证）

StateGame 全局环境：`defaultui`（table，含 UIWorld）✅、`ImportSCEContext`（function）✅、`SCE` 全局 = nil（只能经 ImportSCEContext 取）。

顶层：`AdditionModelActor, Animatable, BeamActor, ClassMap, EffectActor, GameActor, GameUnit, GameWorld, GetGameWorldInfos, MaterialActor, MiniMapIconProxy, MiniMapProxy, ModelActor, ReleaseRenderTargetLink, RuntimeLuaLifetimeObject`。

| 类 | 方法（__index） |
| --- | --- |
| GameWorld | create_scene / **load_map** / **set_map_dir** / setup_viewport / destroy_viewport / set_render_path / set_render_target_link / set_camera_info / **use_light_group** / add_game_actor / remove_game_actor / add_game_unit / remove_game_unit / purge / __create / __release |
| ModelActor | **set_asset** / **get_mesh_asset** / **set_mesh_asset_material** / play / pause / resume / stop / is_playing / attach_to / detach / set_position / set_rotation / set_scale / show / __create / __release |
| GameUnit | 同 ModelActor（含 set_asset/get_mesh_asset/set_mesh_asset_material） |
| EffectActor | **set_asset** / play / pause / resume / stop / attach_to / detach / set_position / set_rotation / set_scale / show（无 get_mesh_asset） |
| BeamActor / MaterialActor / AdditionModelActor | play/pause/resume/stop/attach_to/detach/set_position/set_rotation/set_scale/show（Beam/Material 无 set_asset） |

UIWorld 实例字段：innerWorld / renderLink（'RT:table:...'）/ position / rotation / UIScene / IsValid；方法：BindToUIScene / UnBindFromUIScene / CreateActor / CreateUnit / SetCamera(Position/Rotation) / SetViewSize / UseLightGroup / Destroy。innerWorld 实例字段：`__cinstance:userdata, __cname, render_type:number`。`GetGameWorldInfos` 返回空表（待查）。

### 2.4 编辑器侧（Editor\src\LuaExport，仅 StateEditor；游戏 lua 不可用）

EditorLuaSystem / luaex_PluginsManager / luaex_Material（**set_uv_tiling/set_uv_speed/set_tex_rotation 437789-437794，仅编辑器，tester 无此串**）/ luaex_Common / luaex_DebugManager / luaex_MapInfo / luaex_TaskPipline / luaex_CSharpCommon / luaex_Prefab / luaex_Spine / CSharpLua / LuaDockTitle。另有 get/set_required_subimages_horizontal/vertical、get/set_subuv_subimage_index（粒子编辑器 subuv，tester 无 → 弃）。

### 2.5 控件类型全集与属性名注册块

控件类型（424818-424846）：image data panel button label input progress canvas **nvg_canvas** scene sprites spine virtual_joystick(+_slider/_listener) particle dock_area window **viewport** color_packer color_panel lite_code minimap_canvas **webview video** scroll_rect spline_curve spline_bg bezier_curve。

属性名注册块（Key.K_*）：公共 452112-452146（STATIC/DISABLED/SWALLOW_EVENT(S)/ENABLE_DRAG/DROP/Z_INDEX/ENABLE/SHOW/COLOR/GRAY/ROUND_CORNER_RADIUS/IMAGE/MASK_IMAGE/BORDER/OPACITY/LAYOUT/TRANSITION/ROTATE/SCALE/LOW_LEVEL/CLIP/RENDER_GROUP/FLIP_X/FLIP_Y/CUSTOM/FIX_SCALE/FIX_BORDER/TEXT/FONT/LOOP/SPEED/META_INFO_STR/BLUR_IMAGE）；scene 452524-452556（K_CAMERA_INFO/K_FOV/K_ORTHOGRAPHIC/K_MODEL/K_ANIM/K_PARTICLE/K_BUFF/K_LIGHT/K_AVATAR_PATH/K_ICON_PATH…）；particle 452595-452604（K_EFFECT/K_PLAY/K_STOP/K_PARTICLE_SIZE/K_DIRECT_SCALE/K_PARTICLE_ENDFLY/K_OFFSET_PERCENT/K_AUTO_SCALE/K_PARTICLE_SCALE/K_VIEW_MODE）；webview 452622-452629（K_URL/K_HTML/K_RUN_JS/K_WEB_MESSAGE…）；video 452632-452640。**全集确认：无 UV/源矩形/texcoord/subimage 属性**（texcoord 仅出现在 shader 源码串；subimage 仅 atlas 内部错误消息）。script-199 `common/base/ui/ui.lua:475-504` 的 ui_default watch 表（clip/image/flip_x/flip_y/scale/rotate/gray/opacity/...）与之一致。

## 3. imgui 立即模式直驱通道（webview/video 激活）

### 3.1 核心结论

`ui.imgui_begin_view/begin_ui/props/end_ui/end_view`（每帧立即模式驱动）是比 base.ui 声明式创建更底层、且能激活更多控件类型的通道：**webview（真加载网页）与 video（HTML5 播放器）在 StateGame 内经 imgui 直驱渲染成功，base.ui 创建的同类型控件完全不渲染**。imgui_* 不是另一个 UI 库，它就是 native ui.* 的立即模式入口，与 base.ui 共用控件系统。imgui 无独立 draw list、无自定义纹理/UV 采样（画图像仍走 image 属性）。

### 3.2 实测矩阵（PIE StateGame）

| 控件 | base.ui.create | imgui 每帧直驱 | 备注 |
| --- | --- | --- | --- |
| panel/image/sprites/spine/particle | ✅ | （未试，预计 ✅） | base.ui 足够 |
| webview | ❌（含延迟 set url 补救：创建后 60/300 帧两次 set_control_prop url 仍不渲染——死亡与时序无关） | ✅ 真渲染网页（baidu 导航栏可见） | |
| video | ❌ | ✅ 播放器 UI 渲染（file:// 被拦 0:00；https mp4 完整播放） | |
| scene | ❌ | ❌（含 independent=true） | scene 的正确用法是 base.ui.create 流，见 §8.5 |

- webview 内容出现在游戏视口 WGC 截图里 → webview 是**引擎内嵌渲染**（离屏合成进游戏 UI 纹理），不是独立 HWND overlay。

### 3.3 imgui 直驱用法（StateGame 实测可用）

```lua
base.event_register(base.game, '游戏-更新', function()
    ui.imgui_begin_view('main', 'my_view')           -- root='main'（StateGame UI 根）
    if ui.imgui_begin_ui('webview', 'my_web') then   -- begin 返回 false 时本帧跳过
        ui.imgui_props(true, 'https://www.baidu.com', function(show, url)
            return { show = show, url = url,
                     layout = { width = 320, height = 200,
                                position_type = 'absolute', position = { 1560, 260 } } }
        end)
        ui.imgui_end_ui('webview', 'my_web')
    end
    ui.imgui_end_view('main', 'probe_view')
end)
```

- imgui_* 全集 12 个（443431-443452）：begin/end_view、begin/end_ui、begin/end_wrapper、props/props2、data/state/view_data/view_state。
- props 语义：位置参数 = 上次返回值回灌，最后一个参数 = 产出属性表的函数（参考 xdeditor trigger_select.lua:1286 注释掉的官方用法）。
- begin_ui 的 type 参数直接透传 native 控件类型（'webview'/'video'/'scene'/'panel'/'label'/'input' 均可）。
- **必须每帧驱动**（立即模式：不驱动即消失）。官方封装参考 appui-50 ui/script/imgui/imgui.lua（含 rmgui 模板嵌 imgui 机制 begin_template/end_template）。
- 每帧 pcall 保护：签名错误会在每帧抛错，首次 log 后静默。
- **imgui 控件层级 id**：`imgui_state().id` 在 begin 块内返回如 `main[wv_probe]>webview0`，可直接用于 `ui.set_control_prop`（run_js 实证）。

### 3.4 video 控件细节

- **源属性 = `video_url`**（不是 webview 的 `url`）！出处：sceengine-strings 452631-452633（`GUIVideo` / `Key.K_SRC` / `"video_url"`）。属性名错了不报错、黑盒无请求。
- GUIVideo = webview + HTML5 `<video>` 标签；内嵌 HTML 模板（452635-452640）：`<video ... controls ... muted autoplay src="...">`——自带 muted+autoplay，http(s) 源给出即自动播，无需 run_js play()。
- file:// 被 CEF/miniblink 安全策略拦（播放器框出、0:00 不加载）；`http://127.0.0.1:8899/sample.mp4` PIE 实证播放器 + 画面渲染、进度走完。

### 3.5 base.ui webview 死亡机制（编辑器 vs 线上）

环境检测 `ui.check_webview_environment()` 编辑器恒 true（检测不是阀门）、`ui.set_enabled_in_game('webview', true)` 可用、控件能创建——**死亡点在渲染驱动**：webview 控件状态机需要被每帧驱动（imgui 循环干了这件事），base.ui 的 StateGame 渲染管线不驱动 webview 类型控件（页面不加载→不渲染→JS 桥自然无消息）。线上大厅（lobby state / StateApplication）的 UI 管线驱动它，故线上 base.ui + set_enabled_in_game 有效（用户 webview-demo.lua 线上实证）。

**生产建议**：编辑器内调试 webview 用 imgui 通道；正式发布走 base.ui + set_enabled_in_game；两者用同一份 html 内容。

### 3.6 线上（tester PCBox）实证（2026-08-24 真局，pak v97）

| 控件 | 驱动方式 | 线上结果 |
| --- | --- | --- |
| video（https mp4，oss.laf.run） | imgui 每帧 props `{show, video_url, layout}` | ✅ 播放器 UI + 进度 2:18/2:18 播完 |
| webview（内嵌 html canvas2d 棋盘格） | imgui 每帧 props `{show, html, layout}` | ✅ 完整上屏（带滚动条，overflow 问题见 §3.7） |

与 canvas_texture 的「编辑器可用、线上 PCBox 硬崩」形成对照——imgui 驱动的 video/webview 走 miniblink 离屏合成，线上构建支持完整。旧线索「bench 注释称 video 线上会崩」**已证伪**（至少 imgui 驱动 + https 源不崩）。

### 3.7 webview 深度：miniblink 内核 + lua↔JS 桥

- 实现：`Client\src\Game\WebEnv\Impl\WebEnvMiniblink.cpp` + `WebViewportClientMiniblink.cpp`（miniblink = 国产轻量嵌入式浏览器内核，UA=Chrome/69，非 CEF/WebView2 主路径）；字符串里另有 WebView2 初始化错误处理（双实现，miniblink 为游戏内嵌主路径）。
- 引擎向每个页面注入 polyfill：

```js
if (!window.chrome.webview) {
    window.chrome.webview = {};
    window.chrome.webview.postMessage = function(message){ chrome_webview_postMessage(message); };
    window.scelua = { send_string: function(msg){ chrome.webview.postMessage(msg) } ...
```

- JS→lua：`scelua.send_string(str)` → lua 侧 `on_web_message` 事件（官方 base.ui 用法：bind event on_web_message，client_base lobby.lua:541 / xdeditor resource_store_web_ui.lua:33-49，消息为 JSON 字符串）。**StateGame imgui 路径下消息未到达 lua（未通）**——假设：① web_message 事件路由依赖 base.ui 声明式创建时的 subscribe_now；② StateGame webview 不注入 scelua 桥；③ on_web_message 分发需控件名精确匹配。鉴别法：页面里 `typeof scelua` 用 run_js 读回上屏。
- lua→JS：控件属性 `run_js`（实证可用，页面执行上屏）。相关属性：url/html/run_js/web_message/web_type/web_dev_tools/web_import_script/isolated；`web_import_script` = 注入额外 JS。
- **canvas2d 自定义渲染通道（tiled/任意 2D 的生产级替代）**：lua 把图集 PNG 经 base64 内嵌进 html（require 数据模块，pak 感知）→ 页面 canvas2d 按 tile 映射 drawImage（浏览器级 GPU 加速，一张图集一次解码）→ webview 显示；lua→JS 数据通道 = run_js（可每帧推状态）。绕开 clip/sprites 性能问题。
- 消滚动条：html 加 `overflow:hidden` + 精确尺寸（body 默认 margin/滚动区会出滚动条）。
- 性能未知项：miniblink 离屏合成每帧拷贝成本、多实例上限（需压力实测）。

## 4. 特效 / sprites / spine 直路径

### 4.1 ★ UI particle 控件 .effect 直路径（特效免数编）

```lua
base.ui.particle {
    layout = { width = 300, height = 300, ... },
    effect = 'libs/res/particle/demo/p_12sc_effect_new_6o1_dl47/particle.effect',  -- 带不带 .effect 扩展名都行
    play = true,
    speed = 1,                -- ❗必须 number；字符串 '1' 会导致整个特效不渲染
    direct_scale = { 1, 1 },  -- ❗必须 table；字符串 '1,1' 同罪
}
```

- 根因破解：用户反馈「特效直接用项目中的特效文件不显示」——**根因不是路径，是属性类型**。`speed='1'`/`direct_scale='1,1'`（字符串）→ 控件创建成功但完全不渲染且无报错；换成 number/table 立即显示。
- 路径为 pak 感知资源路径（bgd 构建 `src/res/particle/...` → `res/effect/bgd_game_client/...` 改写后形态即可）。
- 待测项：世界特效（非 UI 向 .effect）在 UI particle 控件的表现差异（view_mode 属性）、粒子在 2D UI 视角的朝向问题。
- 世界内特效（EffectActor）必须数编注册（eff.cache 数据驱动）。

### 4.2 sprites 定格（官方图集通道）

`base.ui.sprites { image=图集, sprite_size={w,h}, row_frame_count=N, frame_count=1, start_frame=K, end_frame=K, playing=false }`——**全部数值类型**（字符串静默不渲），显示第 K 帧图元，帧切换改 start_frame/end_frame。性能画像：每控件一个 draw call。image 路径用运行时形态 `@<ProjectName>/image/sprites/...`（@前缀 = map_settings.json 的 ProjectName 而非项目目录名，如 @p_55a3）。

### 4.3 spine 完全自由

spine 控件 `resource='<.skel 无扩展名路径>'`（引擎自补 `.skel`，452577），动画名直接播（Run_Battle_00 实证）。

## 5. 本地模型世界内通道：unit_change_model 破解

### 5.1 一句话用法

```lua
-- id 必须是真实单位（正 id；玩家英雄 id 见下）。客户端 game.create_unit 造出的负 id 单位无效！
game.unit_change_model(1, 'characters/_user/jilulu_19ec/model.prefab')
game.unit_play_animation(1, 'Run_Battle_00')
-- 验证：
game.get_unit_model_path(1)  --> 'characters/_user/jilulu_19ec/model.prefab'
game.get_unit_asset(1)       --> '$$p_55a3.unit.主控.Model'（数编 link 不变）
```

### 5.2 实测矩阵（test_res002 PIE，api13）

| 对象 | id | change_model 效果 | get_unit_model_path |
| --- | --- | --- | --- |
| `game.create_unit('雷神')` preview 单位 | -20/-21（负） | **完全无效**（native 层 lookup 失败直接跳过，无报错） | nil（前后都 nil） |
| 玩家英雄（真实单位） | 1（正） | **生效**：库存 prefab / 本地 prefab / 坏路径全部写入并视觉切换 | 即时更新为新值 |

- 参数形态 = **prefab 相对路径**（英雄原始 model_path = `characters/general/sk_basic2/model.prefab` 证实）。此前「所有形态均无效」的根因是**作用对象错了**（客户端 preview 单位负 id），不是参数形态错了。
- 坏路径也静默写入（native 无校验日志），视觉表现 = 模型消失。
- 玩家英雄正 id 来源：client 日志 `player.lua:422 sync player hero id: 1.0`（`base.local_player():get_hero()` 在 test_res002 恒 nil，但 id 已同步；可直接用数值 id 调 game.* 原生函数）。
- 换模后动画走新模型的 anim 集（`unit_play_animation` 用新模型自带动画名）。
- 未验证：服务端侧 change_model 同步行为；双人/线上环境。

### 5.3 native 定位链（version-13 sceengine.dll，50,869,176 B）

| 环节 | 位置 |
| --- | --- |
| 字符串 `unit_change_model` | VA 0x1826bcec0（file off 0x26bbcc0），无 lea xref |
| lua 注册表项（luaL_Reg {name,func}） | .rdata file off 0x26b85f0：name 指针 + func=RVA **0x12a7f90**；邻项 unit_detach_model/UnitDetachModel 共用 RVA 0x12a7fc0，unit_set_meta_human_part_value=0x12a81d0 |
| wrapper 0x12a7f90 | 取游戏上下文（magic edx=0xfff0b9d7）→ tail jmp 实现 0x12a23b0 |
| 实现 0x12a23b0 | arg1=id（int），0x1816fbc50(manager,&out,id) 按 id 找单位，**找不到直接 return 0（无任何日志）**；arg2=lua string → 自定义串{u32 len,u32 pad,char* ptr} → 调本地应用 0x1785350(unit,&str) → 组消息广播到其他端 |
| 本地应用 0x1785350 | `strcmp(unit->0x558, 新串)` 相同则跳过；不同则虚函数 `[组件+0xc8]` 换模 + 拷入 unit+0x550 → tail 刷新 0x178ab50 |

**防重复坑**：native 层 strcmp 短路，连续两次同路径调用第二次是 no-op；测试轮换时交替两个不同路径才每次都有动作。

### 5.4 unit_attach_model 实测与判死

- native 签名（impl RVA 0x12a20d0 → 核心调用 0x18176e8b0）：`unit_attach_model(id, path[, hand_point[, hold_point[, bool]]])`——arg2 path 必填，arg3/arg4 挂点串可省，arg5 布尔可省；同样要求正 id。
- PIE 全参数矩阵（挂点三形态/双挂点/arg5=true/官方武器 sm_jian，U22~U25）：**全部 ok=true 但无可见效果**。
- frida hook wrapper impl 0x12a20d0 / 核心 0x18176e8b0 / 内层 0x1817af940 → 客户端调用时**零命中**（pcall ok=true）——该注册在客户端 VM 可能根本没绑到 game 表，或为死 API。**unit_attach_model 判死（至少客户端侧不可用）**，官方活通道 = actor attach（§6）。
- `UnitAttachModel`（PascalCase）与 `unit_attach_model` 同一 wrapper（0x12a7f30）；`attach_actor_to_socket`=0x12a54b0、`attach_actor_to_anchor`=0x12a5480、`unit_detach_model`=0x12a7fc0、`unit_change_model`=0x12a7f90→impl 0x12a23b0。
- attach 核心链备查：wrapper 要求 `[unit+0x1f0]` 组件非空（否则静默退出）；核心 0x18176e8b0 按 path 哈希查 `[组件+0xe8]` 附着表（**同 path 重复 attach 会先拆除旧的**，toggle/替换语义）；`[组件+0x80]` 非空时加载资源（失败静默退出）→ 内层 0x1817af940(node_mgr, 组件, path, 资源, bool, hand_point, hold_point)。

## 6. actor 附着通道（官方活通道，视觉实证）

### 6.1 一句话用法（test_res002 实证）

```lua
-- 数编条目 $$p_55a3.actor.bgd_jilulu_attach（ActorModel，Asset=jilulu prefab）已注册
local a = base.create_actor_at('$$p_55a3.actor.bgd_jilulu_attach.root', base.point(3325, 3325, 0))
a:attach_to(1, 'socket_overhead')   -- 英雄 id=1；挂点名=socketName（§6.2 表）
-- 换点：a:attach_to(1, 'socket_hand_r')；卸除：a:detach()（actor.lua:406）
```

- 底层 native：`game.attach_actor_to_socket(actor_id, target_id, socket)`（wrapper RVA 0x12a54b0）/ `game.attach_actor_to_anchor(actor_id, anchor)`（0x12a5480）——与死掉的 unit_attach_model 是不同 API。
- script-199 封装链：`common/base/actor.lua:329 mt:attach_to(target, socket)`（target 可为 unit 或 actor）；`base_lua_plus/actor.lua:18 base.create_actor_at(name, point)`；服务端侧 `common/base/server.lua:96/156 register_func('actor'/'unit','attach_to',...)`（服务端也可发起 attach，联机同步未实测）。
- attach_to 内有 `AttachForwardOnce` 分支：只定位不附着（用 `game.get_socket_position(target, socket or 'Socket_Root')`）；正常分支 self.hosted=true + native attach。
- 实证截图：吉鲁鲁模型附着在英雄头顶 socket_overhead（capture_1787523735.png）。

### 6.2 挂点（socket）命名规范

prefab（TNND 加密，XOR CREATEEASY 解出 JSON）内含 `sockets` 数组：`{type:"socketMesh", boneName, socketName, position/rotation/scale, tag}`。**挂点名 = socketName**（2.0 官方注释实证：`GetSocketPosition("socket_weapon_r")`，`wasicoresdk\18\api\client\gamecore_actorsystem.cs:1172-1195`）。

**sk_basic2（test_res002 英雄模型）socket 表**：

| boneName | socketName |
| --- | --- |
| Root | socket_blood_bar / socket_root_bar / socket_laser |
| Bip001 | socket_center |
| Bip001 Head | socket_head / socket_overhead / socket_mask |
| Bip001 Spine / Spine2 | socket_waist_l / socket_waist_r / socket_chest / socket_hit / socket_back |
| Bip001 L/R Hand | socket_hand_l / socket_hand_r / socket_magic_weapon(L) / socket_gun_weapon_l(L) / socket_sword_weapon(R) |
| Bip001 L/R Foot | socket_foot_l / socket_foot_r |

**new_sk_basic2 差异**：`p_weapon_1/p_weapon_2`（数字不是 l/r，且 `sockket_weapon_l` 有官方拼写错误）+ `p_weapon_r/p_weapon_l` → socket_weapon_r/l；Root → `Socket_Root`（大小写不同！）。

**jilulu_19ec socket 表**：p_weapon_l/r → socket_weapon_l/r；Bip001 Spine2 → socket_hit/socket_back/socket_chest/socket_wing；Head → socket_overhead/socket_head/socket_mask；Root → socket_root_bar/socket_blood_bar/socket_root；L/R Hand → socket_hand_l/r；L/R Foot → socket_foot_l/r；Bip001 → socket_center；Spine → socket_waist_l/r。另有 rootBoneName 系列（BN_QL_QB_01 等 = 布料/头发物理骨）。

### 6.3 官方附加模型数编路径（ActorAdditionModel，对照实证）

其他游戏 pak 镜像（game_p_2xgc / game_p_1ax1 的 `ui/script/obj/actor/actor.lua`）存在大量数编节点 **ActorAdditionModel（「附属模型表现节点（自动挂载骨骼）」）**：

```lua
entry_datas['$$.unit.test_inventory_user.p_2xgc_ActorAdditionModel'] = {
    ['Model'] = '$$.unit.test_inventory_user.p_2xgc_Model',  -- 数编 Model link
    ['SocketName'] = '',            -- 可空（空=自动按骨骼蒙皮对齐）
    ['Offset']/['Rotation'] = {...},
    ['EventCreation'] = 'on_cast_start',  ['EventDestruction'] = 'on_cast_stop',
    ['EventCreationModel'] = '',          ['EventDestructionModel'] = 'Death',
    ['FollowRotation'] = 1, ['TrimInsideTriangles'] = false, ['sync'] = true,
    ['NodeType'] = 'ActorAdditionModel', ['Template'] = 'ActorAdditionModel',
}
```

与 2.0 `GameDataActorAdditionModel` 同一机制：**对齐由美术资源蒙皮决定（骨骼名与宿主完全一致），Socket 字段填了也不生效**；挂错资源的表现 = 显示在脚下/原点。适合「穿在身上的部件」场景。

## 7. 数编注册链与脚本化

### 7.1 数编注册链（每一环都有实证）

```
编辑器内存数编
  ⇅ 持久化：editor/table/entry_data/<类型>/<$$命名空间.link>/entry_data.ini + ui_data.ini [+ i18n/]
  ↓ 编辑器 trigger 管线（tstl 编译，调试启动时 generate_lua_only）
script/obj/<类型>.lua（entry_datas['$$link']= {...} return 表）  ← 产物！手改会被覆盖
  ↓ 装载（require '@@.obj.*' / init_cache）
lua 侧：base.eff cache（caches.dict）、base.table.*（get_game_table + obj 模块合并）
native 侧：内部注册表（create_actor/set_actor_asset 只查这里；get_game_table 部分表为空=不经此暴露）
```

- 既有可用 link 来源：项目引用的**预编译库**（default_units_ts/spark_core 等，数千条目）。
- `eff.merge_cache` 的官方用途是合并 **lua 侧**逻辑数据，对 native 渲染注册无效（§11）。
- 运行时 lua 侧注入全部证伪：eff.merge_cache / get_game_table 注入 / require 的 obj 模块表注入 / base.table 副本 / 手改 `script/obj/*.lua`（装载时被编辑器 trigger 管线重新生成覆盖）。

### 7.2 entry_data.ini 条目模板（脚本化）

**ActorModel 内嵌 Model**（`editor/table/entry_data/actor/<条目名>/entry_data.ini`，仿 default_units「通用模型表现」）：

```lua
[#CONFIG]
'Version' = 14                       -- 与项目既有条目一致（test_res002 主控=14，default_units=13）

['Model']                            -- 内嵌 Model 子节点（link = $$<ns>.actor.<名>.Model）
'Version' = 1
'NodeType' = '$$.model.Model'
'Inherit' = '$$.template@model.Model.root'
'Data' = {
    'Editor' = {},
    'Game' = {
        'Asset' = 'characters/_user/jilulu_19ec/model.prefab',   -- ← 本地模型 prefab 相对路径
    },
}
'UIConfig' = {}

['root']                             -- 主节点（link = $$<ns>.actor.<名>.root）
'Version' = 1
'NodeType' = '$$.actor.ActorModel'
'Inherit' = '$$.template@actor.ActorModel.root'
'Data' = {
    'Editor' = { 'CollectRes' = true },
    'Game' = {
        'Name' = '显示名',
        'Model' = 'Model',           -- 引用内嵌子节点（写子节点 ID）
    },
}
'UIConfig' = {}
```

**ActorEffect 内嵌 Particle**（特效进 UIWorld 用）：

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
'NodeType' = '$$.actor.ActorEffect'  -- ★ NodeType=ActorEffect → UIWorld GetActorFactory 路由到 SCE.EffectActor
'Inherit' = '$$.template@actor.ActorEffect.root'
'Data' = {
    'Editor' = { 'CollectRes' = true },
    'Game' = {
        'Name' = 'bgd演示特效',
        'Effect' = 'Particle_1',     -- 引用内嵌子节点（写子节点 ID）
        'KillOnFinish' = 0,          -- 持续型特效不自杀
        'KillOnDeactivate' = 0,
    },
}
'UIConfig' = {}
```

- 其他模板出处：ActorAdditionModel 字段模板 = `single_simple_ts_template_8/ui/script/plugin/obj_editor_v2/config/entry_data/template@actor/#actor#addition#model/entry_data.ini`；独立 Model 条目模板 = default_units 的 `entry_data/model/defaultmodelwithgenericfootstep/entry_data.ini`；ActorEffect 字段全集参照 game_p_1ax1 编译产物 `obj/effect/actor/data.lua`（Effect/SocketName/Offset/Rotation/FollowRotation/EventCreation/EventDestruction/AnimTrail/PlaySpeed/Scale/AutoScale/ShowShadow/CreationFilter...）。
- link 规则：`$$<项目ns>.<类型>.<条目目录名>[.子节点]`；目录名即条目 id。
- 数编 Asset 字段用 pak 感知路径（`res/effect/...` / `characters/_user/...`）。

### 7.3 生效流程（外部写入 → 编译进 obj）★★

1. 编辑器**关闭状态**下写入 ini（开着项目写会被保存清理逻辑当失效文件删除）。
2. **删 `script/obj/save_info.json` 和 `ui/script/obj/save_info.json`**（关键！三文件时间戳一致时 `need_save=false`，obj 永不重生成）。
3. 顺手把 `editor/table/save_info.json` 的 timestamp 改大（保险）。
4. 重开编辑器（`EVENT.load_map_done` 全量扫描 entry_data 目录发现新条目 + 时间戳不一致置 `force_complete_save_next_time`）。
5. full 调试（保存管线重生成 `script/obj` 与 `ui/script/obj`）→ 条目进 `ui/script/obj/actor/actor.lua`、`model/model.lua`、`effect/actor/{data,dict}.lua`。
6. 之后 restart_last_debug 即可正常使用。

### 7.4 数编发现/编译机制（xdeditor obj_editor_v2 源码实证）

- **发现**：无清单/索引；`EVENT.load_map_done` → `type_config_loader.load_entry_data`（:430）对每包扫 `editor/table/entry_data/<类型>/<条目>/entry_data.ini` 两层目录。运行期无目录监听，外部新增必须重开项目。
- **编译**：`script/obj`（服务端）与 `ui/script/obj`（客户端）由保存管线从内存数编生成（`type_config_saver.lua`），`clear_dir_files` 清掉不在内存集合的旧产物。
- **时间戳闸门**：`editor/table/save_info.json` vs `script/obj/save_info.json`、`ui/script/obj/save_info.json` 包级比对；一致则 `need_save=false` 跳过生成。
- **陷阱**：编辑器开着项目时外部写入新条目再保存 → 保存清理逻辑（map_info.lua:654-689）把「磁盘有内存无」的 ini 当失效文件**删除**。必须先重开项目再保存。
- restart_last/full debug 都会用编辑器内存数编重新生成 `script/obj/*.lua`——手改 obj 文件无效且会被静默还原（git 也看不出改动）。

### 7.5 手写 GUI 页面脚本化（免编辑器建页面）★

GUI 页面本质 = `ui/script/gui/page/<名>/` 下两个 lua 文件 + `page/init.lua` 注册一行，手写即生效（**restart_last_debug 可拾取，无需 full 调试/重开编辑器**）：

- `page/<名>/template.lua`：`gui_pkg.page_template { flatten_template = { ctrl_wrapper.panel{...},0, gameui.UIScene{ RenderPath='EngineRes/RenderPaths/GameSnapshot.xml', layout={...position={x,y},width,height}, name='gw_scene', show=true },1, } }`（flatten_template 每项 = 控件模板 + 父索引，0=根）。
- `page/<名>/component.lua`：`component '<名>' { pkg.require_template(lib_env,'<名>'), event={}, prop={}, method={}, state={} }`。
- `page/init.lua`：`kPageNames` 数组加 `'<名>'`。
- 控件绝对定位写法：`layout = { col_self='start', row_self='start', position={x,y}, width=.., height=.. }`（仿生产模板，勿用 position_type——那是 base.ui 直建流的字段）。
- 文件头标 AUTO-GENERATED 只是约定，运行期直接加载磁盘文件；**GUI 编辑器保存页面时会覆盖/清理手写页面**（研究用无碍，生产用须走编辑器或接受被清）。

## 8. GameWorld / UIWorld 组件栈（3D 世界渲染进 UI）

### 8.1 官方组件栈（defaultui_63 uiworldscript.lua 全解 + 生产用法 p_2xgc）

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
- UIWorld:GetActorFactory 路由表（uiworldscript.lua:277-305，读 `base.eff.cache(link).NodeType`）：ActorModel→SCE.ModelActor / ActorAdditionModel→SCE.AdditionModelActor / ActorBeam→SCE.BeamActor / **ActorEffect→SCE.EffectActor** / ActorMaterial→SCE.MaterialActor；**particle link 直传被拒**（报错「UI场景不支持该表现类型：Particle」）——特效必须包一层 ActorEffect 数编条目。
- GetActorFactory 对 lua 缓存缺项不防御：`CreateActor(完全未注册 link)` → `uiworldscript.lua:279 attempt to ind(ex)` lua 错误（pcall 可捕）；「lua 有缓存但 native 无 → 返回 nil」是另一种死亡形态。生产代码需先查 `base.eff.cache(link)` 非空。

### 8.2 ★ UIScene 通道完整配方（PIE + 线上双实证）

```lua
-- ① 手写页面文件（§7.5）后：
local page = base.gui_new('GWProbePage')               -- 实例化页面
local ctrl = base.gui_get_part(page, 'gw_scene')       -- 取 UIScene 组件实例（= page.part['gw_scene'][1]）

-- ② 建世界（第三参 = 场景名，载真实地图；可省=空世界）
local world = defaultui.UIWorld:Create(false, '$$.camera_property.camerapro.root', 'default')

-- ③ 相机对焦（数编镜头 cache 无 init_position 时 SetCamera 静默跳过，必须手动对焦）
local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 300)  -- 焦点(0,0,0) pitch-70 距离300
world:SetCameraPosition(pos[1], pos[2], pos[3])
world:SetCameraRotation(-70, 0, 0)

-- ④ 摆内容：数编 actor link（§7.2 脚本化注册的本地模型条目）
local a = world:CreateActor('$$p_55a3.actor.bgd_jilulu_attach.root')
a:set_position(0, 0, 0)
a:play('Idle')

-- ⑤ 绑上屏（等一帧让控件拿到真实尺寸后再绑，否则 BindToUIScene 内部 rect 检查失败）
base.wait(200, function() world:BindToUIScene(ctrl) end)
```

- **命名控件访问 = `base.gui_get_part(page, '控件名')`**（= `page.part[name][1]`；生产游戏实证，`page.gw_scene` 直取是 nil）。`base.gui_get_part_as(UIScene类型, page, name)` 为类型化变体。
- 实证：地图 terrain 进 UI → 吉鲁鲁模型清晰渲染在控件区域；**2026-08-24 用户确认编辑器与线上发布全部生效**。
- RT 画面按 SetViewSize 的世界视口渲染，**可溢出控件边界显示**（BindToUIScene 自动用控件 rect 对齐，生产注意尺寸匹配）。
- UIWorld 默认 renderLink = `'RT:' .. tostring(self)`。

### 8.3 BindToUIScene 内部（defaultui uiworldscript.lua）

读 ctrl.ui:rect()（0 尺寸报错返回 false，**必须等一帧**）→ RenderPath=='GameSnapshot.xml' 时 render_type=66 否则 75 → SetViewSize(w,h) → set_render_path → `uiScene.RenderTarget = renderLink`（组件实例属性写，转发到 native part）→ set_render_target_link。

- RenderPath 枚举仅 `EngineRes/RenderPaths/GameSnapshot.xml`（半透明混合，render_type=66）/ `EngineRes/RenderPaths/CEMap.xml`（不透明，render_type=75）。
- ViewpotFormat 枚举：RGBA=58 / RGBA16F=66 / MOBILE_HDR=75。

### 8.4 崩溃归因矩阵（GameWorldProbe 隔离实验，PIE StateGame）

| 组 | 内容 | 结果 |
| --- | --- | --- |
| G1-only | scene 控件 + resource=数编 actor link + independent | 存活，不渲染 |
| G2-only | 2 只 UIWorld（camera/actor/RT link 全套） | 存活，世界正常渲染到 RT（**GameWorld 可创建、可加载 actor、可渲染到 RT 纹理，不崩**） |
| G1+G2+G4 | + scene 控件 set resource/render_target/RenderTarget='RT:gw_probe' | 存活，不渲染 |
| G3 | image 控件 image='RT:gw_probe_img' | **崩溃源：image 吃 RT 链接硬崩（整编辑器+游戏无 dump 消失）** |

**新坑固化**：RT 命名纹理（'RT:xxx'）只能被 UIScene 类控件消费；**喂给普通 image 控件 = native 硬崩无 dump**（与 canvas_texture PCBox 崩同款静默）。裸 `base.ui.view{type='UIScene'}` 可直建 native 控件，但 `ui.set_control_prop(id, 'RenderTarget'/'render_target'/'image', 'RT:..')` 全部静默吞（set_control_prop 无属性校验，错名不报错）——裸控件缺组件层的 RenderTarget 转发逻辑，**不要走这条路**。

### 8.5 scene 控件（GUIScene）终版结论

- `base.ui.scene` = native GUIScene 控件的 Lua 模板封装。绑定层全部走属性通道：`base.ui.gui.set_*(self.id, {[k]=v})` 实际是 `ui.set_control_prop(id, '<去掉set_前缀>', ...)` 语法糖（model/particle/buff/light/camera_info 都是普通属性，无 Lua 侧校验/转换）。`template/scene.lua:82` 仅当 `__lua_state_name` 为 StateGame/StateApplication 时创建 native 'scene' 控件，否则退化为 'panel'。
- GUIScene 完整 native 属性键（sceengine 与 scegame-tester 完全一致）：

| 分组 | 键 |
| --- | --- |
| 场景 | `independent`, `camera_info`, `rotation`, `rotation_ue`, `rotation_qua`, `fov`, `zoom`, `orthographic` |
| model 子表 | `name`, `facing`, `pitch`, `pause`, `scale3D`, `anim`, `anim_fade_time`, `can_edit` |
| 内容 | `particle`, `buff` |
| 光照 | `light`, `directional`, `zone`, `ambient_color`, `lightgroup` |
| 捏人 | `knead_human`, `part_name`, `value`, `part_cloth`, `save`, `avatar_path`, `icon_path` |

- **`model.name` 解析的是【数编单位（unit）表条目的节点名】**（`$$p_55a3.unit.主控.root` → `'主控'`）；**`particle.name` = 数编特效表条目节点名**。证据链：引擎自测用例（'斧王'/'剑圣'/'鹿目圆香' 全部是单位条目名）；GUIScene.cpp 错误串 `"Try to set unit card camera, but ui scene is nullptr."`（model 语义 = unit card）；native 按名查单位表通道 `"[CreateUnit:%d] failed, GetUnitTableEntryByName failed."`；test_res002 反证（只有单位名出画面，actor 名/model 条目名均空）。节点名 vs 显示名无法 100% 区分（'主控'两者相同，悬置；生产按节点名写）。
- 用法要点：`base.ui.scene(props)` 只建模板，**必须 `base.ui.create(tpl, name)` 实例化**；**`independent=true` 是 StateGame 出图前置**。
- **light 游戏态判死**：G20 七变体（官方 light / 过曝 color={10,10,10} / ambient_color / zoom / 正负 name 对照）全部纯黑方块无任何差异——StateGame 下 GUIScene 光照链完全不工作。**结论：scene 控件 model/particle 通道在 StateGame 无生产价值（内容恒黑），模型/特效预览一律走 UIWorld 通道**（世界灯光正常供光）。
- 坑：`scale` 键表没有（只有 K_SCALE3D）——模型缩放写 `scale3D`；anim 字符串/数组两种形态都见于官方代码；scene 控件给 `color=rgba(...,0.15)` 时整个控件（含黑底）变半透明——「透明无内容」是假象。
- knead_human（捏人）语义：scene 开 knead_human → part_name/value 调形 → part_cloth 穿衣 → save+avatar_path/icon_path 出头像文件（配套 `ChangeClothSceneController` 与 `"Save head icon FAILED! path[%s]"`）。使用方代码在 UPAK 压缩包内未解。

### 8.6 GameWorld load_map / set_map_dir / create_scene / use_light_group 逆向

| 方法 | 参数（lua） | 语义 |
| --- | --- | --- |
| `set_map_dir(dir)` | 1 个字符串 | **纯赋值**：存进 GameWorld 成员 `+0x5e8`（m_map_dir），无加载/校验。构造默认从引擎上下文取游戏根路径。正式流程 map_dir 来自命令行 `-map_path` 绝对路径。接受任意字符串，绝对路径最稳。 |
| `load_map(name, flag?)` | 字符串 + 可选 bool | 两步：① 预载：读 `<m_map_dir>/scene/<name>/area_save.lua`（触发区域信息，以数据文件方式 open——**map_dir 必须指向真实文件系统目录**，pak 内读不到只打 failed to open 日志，疑似可降级继续）；② 在 GameWorld **地图注册表**（`+0x408` 哈希表 + `+0x450` 记录数组）按 name 的 sdbm/djb2 哈希查记录，查不到打 `Failed to load map, scene[%s] is invalid.` 失败返回；查到则拼 `scene/<记录目录名>/` 下 `map.acmap` + `ClientCollision.dat` + `HeightData.dat` + `Sight.dat` 四件交场景加载器（走引擎资源系统相对路径解析，松散文件/pak 均可）。**name 不是任意路径，是注册表里的地图名**。 |
| `create_scene(flag?)` | 可选 bool | 新建空 Scene 对象（0x428 字节）替换 `this+0x240`；flag 为假时对 `this+0x258`（相机类）做绑定调用。**不负责加载地图**。 |
| `use_light_group(path, time?)` | 字符串 + 可选 number(float) | 调 `GameWorld+0x28` 指向的 LightGroupTimer 子对象：日志 `Switch light group: %s.`，加载 path 指定 lightgroup 资源并切换，float 为切换时间/淡入。**`+0x28` 为 null 时立即空指针硬崩**（impl 0x18187e040 开头 `mov rax,[rcx]; call [rax+8]`——pcall 拦不住，进程直接崩）。我们传 `'Editor/Light/Engine/default.lightgroup'` 崩掉大概率是 `+0x28` 为 null（调用前提未满足），不是路径问题。 |

注册表（方法 → wrapper VA，GameWorld 类注册函数 0x181346aa0）：

| 方法 | wrapper VA | impl / 关键调用 |
| --- | --- | --- |
| set_map_dir | 0x181347790 | 0x18175f540（string 赋到 this+0x5e8） |
| create_scene | 0x181347070 | 0x18174b260 |
| load_map | 0x1813472f0 | 预载 0x18174a340 → 主载 0x181755220 → 场景加载 0x181755630 |
| purge | 0x181347430 | — |
| setup_viewport | 0x181347cf0 | — |
| destroy_viewport | 0x181347110 | 0x18174c620 |
| add_game_unit / remove_game_unit | 0x181346ce0 / 0x1813474f0 | — |
| add_game_actor / remove_game_actor | 0x181346c40 / 0x181347460 | — |
| set_render_path | 0x181347860 | — |
| set_render_target_link | 0x181347b00 | — |
| set_camera_info | 0x181347570 | — |
| use_light_group | 0x181347dc0 | 0x18187e040（LightGroupTimer） |
| ReleaseRenderTargetLink | 0x181347ff0 | — |
| GetGameWorldInfos | 0x181347eb0 | — |

- load_map 细节：对 name 算 djb2 哈希（`edx=0x1505; imul 0x21`）；预载拼串 `<m_map_dir>/scene/<name>/area_save.lua`；主载注册表查找 `this+0x408` 哈希表（桶 `+0x418`，sdbm 哈希 `imul 0x1003f`），记录布局 `+0x00` u32 地图 id / `+0x18` 目录名长度 / `+0x20` 目录名指针。
- **地图注册表填充点未定位**（疑似 SetGameInfo/地图列表初始化链）——决定「任意项目地图能否直接 load_map」。PIE 先拿当前已加载地图的 name 试（必然已注册）。
- **表加载与 load_map 无关**（§11.3 实证：load_map 永不触发数编表加载）。
- 日志观察点：`Failed to load map` / `Load combined scene trigger area info` / `Switch light group` 三串定位走到哪步。

### 8.7 模块级 LoadMainMap / reset（游戏侧运行时换图判死）

- xdeditor「文件/强制重新加载项目」= `EDITOR.unload_map()` + `EDITOR.load_map(map_path)`（menu_bar.lua:1378-1396）；「文件/打开」= `EDITOR.unload_map()` + `EDITOR.update_map_libs(path)` + `EDITOR.load_map(path, true, cb)`（:1144-1197）。这些 `EDITOR.*` 由 native 注入 **xdeditor lua 状态（编辑器壳进程）**，**游戏 lua 状态无此绑定**（G34a 四层 dump 实证全无）。
- sceengine.dll 模块级 luaL_Reg（.rdata file 0x26cd100，VA 0x1826ce300）：

| lua 名 | wrapper VA | 语义 |
| --- | --- | --- |
| load_map / LoadMainMap | 0x181337a40 | tail jmp 0x181337240：luaL_checkstring → strcmp(输入, [ctx+0xd0] 当前图绝对路径) → **相等=no-op 返 0**；不等=整图加载分支 |
| reset / Reset | 0x181337a70 | `[ctx+0xc0]→0x1809524d0` 拆卸 + 清 ctx+0xc8 串 + `[ctx+0xd8]→0x1806df840` |
| SaveJson | 0x181337ac0 | （未展开） |

- frida 直调三轮实证：同名 LoadMainMap = 设计内 no-op；异名 = 整分支跑但**进程死亡**（内存爆冲 12GB→32GB 后崩溃，无任何 map/table 加载日志）；reset = **拆卸即 AV**。→ **游戏客户端会话无法承受换图**，重载必须编辑器侧完整编排。
- 当前图标识 = 绝对路径（[ctx+0xd0] 引擎串）；部分 ctx 字段是带 vtable 的 String 对象（vtable 在 +0），不能按 {len,ptr} 直读。

## 9. '@gameui' 包：物理位置、require 前缀、UIScene 组件源

### 9.1 物理位置

```
D:\sce_online\update\editor-pd.spark.xd.com\res\_m\gameui\<版本>\gameui\ui\script\*.lua   ← 全部 TNND 加密
```

- 版本由 `api_pak_version.json` 决定（`#package_path.gameui = "Res/_m/gameui"`）：**api 12→47，api 13→48，api 2000→52**。
- test_res002（api 13）用 gameui 48（lua 齐全 62 个）。**悬案**：update 缓存中 gameui 52 只有 ui/image、没有 script 目录（47/48 均有），2.0 项目的 gameui lua 来源待查。
- 同源副本：`sce_app_mini-runtime/runtime/Update/editor-pd.spark.xd.com/Res/_m/gameui/48/...`（payload sync 下载）。
- 解密产物（明文）：`sce_app_mini-runtime/test/temp/gameui-48-script/`。复现：`sce_app_editor-patch/target/debug/examples/decrypt_mirror.exe "<update缓存>/res/_m/gameui/48/gameui/ui/script" "<输出目录>"`。

### 9.2 require('@gameui.xxx') 解析机制（native 侧）

- 实现在 sceengine.dll / commandtool 的 C++ `PathSearcher` / `PathSearcherClient` / `LuaLoader`（`Client\src\Game\PathSearcher\PathSearcherClient.cpp`）。
- 内置前缀→脚本根表（sceengine-strings 443836-443842 铁证）：

```
script/common          ← '@common.xxx'（script 包）
client_base/common
gameui → gameui/ui/script  (+ gameui/ui 资源根)   ← '@gameui.xxx'
appui  → appui/ui/script
```

- `require('@gameui.UIScene')` → 包根 `Res/_m/gameui/48/gameui/` + 脚本根 `ui/script/` + 点号转目录 → `ui/script/uiscene.lua`。
- 组件注册链：`@gameui.component`（component/init.lua）调 script-199 `common/base/gui/package.lua` 的 `load_component`，逐个 `require(require_url)` 后打 `package_name='gameui'`、`require_url`；`meta_info_str='@gameui/uiscene.lua 39'` 由此推导。`require(url, lib_env)` 双参数形式是引擎扩展。
- PathSearcher 类型映射（表加载紧随的 AddPath 序列实证）：`type[0]=地图根 / type[1]=ui/script / type[2]=table / type[3]=ui / type[4]=data`。

### 9.3 uiscene.lua（gameui 48，全 78 行）摘要

- `component 'UIScene'{...}`（`__ui_type='UIScene'`）。模板 = 根 panel(644×846) + 内层撑满 panel，**`image = bind.RenderTarget`——RenderTarget 是 bind 属性，直接转发到内层 panel 的 image 属性**；native UIScene 把控件渲染目标写进该 image。
- `metadata.editable_prop.RenderPath` = 编辑器下拉：GameSnapshot.xml（半透明混合）/ CEMap.xml（不透明）。
- EmmyLua 注解（实现在 native/TS 侧）：UIScene 字段 `CameraLink`、`RenderPath`、`UseShadow`、`World`(UIWorld)、`RenderTarget`；方法 `InitWorld(): UIWorld`、`DestroyWorld()`；UIWorld：`setup_viewport/destroy_viewport/create_scene/purge/__release/set_render_target_link(image)/set_camera_info/set_render_path`。
- **结论：组件层只是一层 bind 属性转发**——裸 `base.ui.view{type='UIScene'}` 缺的正是这层转发（set_control_prop 直写不进去的原因在 native 属性注册侧；生产走组件/页面通道即可）。

### 9.4 gameui 包组件清单（component/init.lua 注册）

**UIScene**、timershow、msgbox、msgbox_btn、progress、btn_icon、normal_btn、normal_rect、attachable_panel、active_button、simpleui_button/text/picture、Buff列表/Buff图标/Buff描述（prefab.buff.*）、transition_label、input_paste、number_input_paste，外加 sci/gf/xf 三系 btn/rect 别名（全部映射回 normal_btn/normal_rect）。script 目录其余文件（未注册为编辑器组件）：arrow、btn、btn_icon_test、corner、icon_circle_frame、icon_num、icon_square_frame、mouse_partical、timershowclient、triangle、component/virtual_list(+horizontal)/virtual_table、prefab/{btn,rect,buff}/* 共 62 个 lua。

## 10. 动态渲染深化

### 10.1 特效进 UIWorld（ActorEffect，视觉实证）

数编 ActorEffect 条目（模板 §7.2）→ `UIWorld:CreateActor('$$<ns>.actor.<名>.root')` → 特效在 UIScene 内渲染（紫色横幅实证）。

- **播放语义**：`CreateActor` 返回即自动播放（未调 play 已显示；`ea:play('cast')` 亦不报错）。位置 `set_position(x,y,z)` 世界坐标。
- 手建 `SCE.EffectActor.new(link)` + `add_game_actor` + `show(true)` **不渲染**——特效需 `play('cast')` 触发（条目 EventCreation=on_cast_start）；走 `world:CreateActor` + `play('cast')` 正常。
- ActorEffect 的 `Effect`/`Model` 子节点引用在 ini 里写**子节点 ID**（'Particle_1'）；编译产物 `script/obj/effect/actor/data.lua` 里是**子节点全 link**。

### 10.2 ★ 自定义 renderpath（渲染管线动态化，视觉实证）

- UIScene 组件属性 `RenderPath` 是 **pak 感知路径**：官方值 `EngineRes/RenderPaths/GameSnapshot.xml`（半透明混合，render_type=66）/ `EngineRes/RenderPaths/CEMap.xml`（不透明，render_type=75）。
- **项目包内路径实证可用**：`res/renderpaths/bgd_snapshot_red.xml`（gamesnapshot.xml 拷贝、clear 改 `1 0 0 1`）→ 运行时页面直接引用 → 红底 + 模型 + 地形正常渲染。即渲染管线 XML 可作为项目资产分发并运行时动态应用。
- 官方管线库 45 个（engineres/renderpaths/：cemap 系 12、editor 系 11、forward/prepass/deferred/pbrdeferred 系、autochess、gamesnapshot、gameplayui(_clear)、gameoverlay、modeltest、particleedit/test、rendertotexture(_nogamma)）全部可作为改写基底。
- XML 结构（Urho3D renderpath）：`<renderpath finalCopy substitute zPrepass>` + command 序列（clear/staticmeshpredraw/scenepass(depth/base/light/postopaque/alphanoblend/xray/alpha/planershadow/postalpha/outsurface/innerstroke/outstroke)/forwardlights/volumetriclight/renderui_back/renderui_front/sendevent）+ `<postprocess name="EngineRes/PostProcess/...xml">`。`cemapunderui.xml` 揭示 UIScene 合成机制：`renderui_back` → `sendevent "UIScene"` → postprocess → `renderui_front`。
- 生产想象空间：自定义 clear/雾色、开关 XRay/描边 pass、换 postprocess 链（bloomHDR/tonemap）、PBR/deferred 管线进 UI、rendertotexture 离屏玩法。

### 10.3 手建 actor 通道（免 UIWorld 包装，种子 link 必须有）

```lua
local SCE = ImportSCEContext(nil)
local ma = SCE.ModelActor.new('$$p_55a3.actor.bgd_jilulu_attach.root')  -- 有效数编 link
world3.innerWorld:add_game_actor(ma)
ma:set_position({ -150, 0, 0 })
ma:show(true)
ma:play('Idle')
```

→ 渲染成功（双吉鲁鲁实证）。早期「手建不可见」的真实原因是位置出画（±150 在相机 dist=300 视野外）。

### 10.4 set_asset 换资产——必须数编表 link（判死裸路径）

- `ma:set_asset('$$p_55a3.model.nezha.root')`（数编 model link）→ 吉鲁鲁种子真换哪吒（视觉实证）。
- **裸 prefab 路径不渲染**（模型侧用户复验）；特效侧 `ea:set_asset('res/effect/.../particle.effect')` 与去 res/ 前缀两形态 + 重 play：**均无报错、画面不变**（判死）。
- 推论：set_asset 不是「直读文件」，仍走数编/资产注册解析（native 机制见 §11.4）。

## 11. 免数编攻坚判死记录（native 注册表逆向，frida 主线资料库）

> 对象：`sceengine.dll` api13（PE32+ x64，50.8MB，imagebase 0x180000000）。VA/RVA 并记：RVA = VA - 0x180000000；.text 文件偏移 = RVA - 0xC00；.rdata = RVA - 0x1200；.data = RVA - 0x1A00。

### 11.1 攻防全图（终版）

```
已实证可用（免逐条数编）：
  主世界真实单位  game.unit_change_model(正id, prefab相对路径)   [§5]
  纹理/材质      ResourceCache 按路径（set_mesh_asset_material） [§11.6]
  UI 特效        particle 控件 .effect 直路径                    [§4.1]
  spine          .skel 直路径                                    [§4.3]
  渲染管线       UIScene RenderPath 吃项目包内 xml               [§10.2]
  数编条目       entry_data.ini 脚本化预生成                     [§7]

已实证判死：
  merge_cache 虚拟数编 → native 三入口（CreateActor/*.new/set_asset）  [§11.2]
  load_map / set_map_dir 注入数编（永不触发表加载）                    [§11.3]
  裸路径 set_asset（模型/特效双侧，查找 miss 静默返回）                [§11.4/§10.4]
  假 link 创建（SCE.*.new(假link/无参) 全 nil）                        [G24/G25]
  条目纯路径内存改写（Tier-1）→ 不一致条目空渲                         [§11.5]
  模块级 LoadMainMap 异名换图（进程死亡）/ reset（AV）                 [§8.7]

唯一剩余主线：frida 运行时注入 native 注册表（§11.7 hook 点已备）
  旁路：frida 直调 path 型加载器（unit_change_model apply 0x1785350）
       作用到 UIWorld actor/unit 的 native 对象上——免注册表换资产
```

### 11.2 虚拟数编（merge_cache）终审（G29 + G33 用户代码原样复验）

- 手法：`base.eff.cache(模板link)` → deepcopy → 改 Name/Link → `base.eff.merge_cache({dict={[新link]=新条目}})`（用户共享实现 virtual_effect.lua/get_eff_cache.lua）。
- 实证：lua 层读写完全正常（`base.eff.cache` 读回、NodeType/Effect/Model 字段正确、用户 `virtual_effect.new/set_value/get_link` API 全工作），但 native 三入口全部拒绝——`UIWorld:CreateActor(虚拟link)`=nil、`SCE.ModelActor.new(虚拟link)`=nil、`set_asset(虚拟model link)`=静默无效果（视觉对照实证）。
- 根因（script-199 `common/base/eff.lua:155`）：merge_cache 只写 lua 层 `caches.dict[key]=value`，**native 注册表不同步**；native 注册表启动时建立、运行时只读。
- **虚拟数编的有效域 = 仅 lua 层 `base.eff.cache` 消费者**（如逻辑层技能/buff 配置读取），渲染入口一律不通。
- 虚拟化必须整套（root + 子节点）注册并改引用（编译产物里 root 的 Effect/Model 字段 = 子节点全 link）。
- native 失败无任何日志进 lua log——死亡点静默，只能靠返回值/视觉判定。

### 11.3 表加载 = 会话一次（load_map 永不触发表加载）

12 个游戏会话的 native 日志事件序列实证：`==> Begin loading table [p_55a3]` 每会话仅启动时一次；任何运行时 `load_map`（含 UIWorld innerWorld 载场景、含 bogus 地图名）均不触发表加载（「Map table is loaded, skip it.」一次性闸）。

- **判死「伪造地图目录 + load_map 注入数编」路线**：load_map 只读场景四件 + area_save.lua，与表无关。
- 数编编译产物经 `SendWriteFile: p_55a3, p_55a3/script/obj/effect/<type>/data.lua` 同步进游戏；native 注册表真实数据源高度疑似 = `script/obj/effect/*/data.lua`（与 lua cache 同源）；`table/` 目录只有 constant.ini/mapinfo.ini/floatingtexttemplate.json。**精确文件清单待 frida hook LoadMapTable 读文件点确认。**
- restart_last 语义：已入库文件的内容变更能拾取；**新增文件需 full 调试**（服务端探针新增文件不生效实录）。

### 11.4 set_asset 分派链（终版，汇编 + frida 双证）

```
lua: actor:set_asset(x)
  └─ wrapper 0x181345a10：checkudata(L,1,"GameActor") + luaL_checkstring(L,2)，零校验零解析
       └─ call [actor.vtbl+0xa0]（0x181345aaf）——按类分派：
            ModelActor  → 0x1817837d0：[actor+0x28]→vfunc+0x60=manager；manager vfunc+0x60(串) 查 MODEL 表
                            mov rbx,[out]; test rbx,rbx; je ret   ← 裸路径死亡点（静默）
                            hit: entry 存 actor+0x1e8 → apply 0x18177d5a0/0x181769c30/0x181783e30
            EffectActor → 0x18179c0a0：同形，manager vfunc+0x70(串) 查 EFFECT 表
                            mov rcx,[out]; test rcx,rcx; je ret   ← 裸路径死亡点（静默）
                            hit: entry 存 actor+0x178 → apply 0x18179c1a0…
            GameUnit    → 未捕（同 wrapper，vtable+0xa0 自有一覆写）
```

- lua 绑定区（VA 0x181345570 起大注册函数）：绑定辅助 `0x181a81ac0`=注册类(rcx=binder, rdx=基类名, r8=类名)、`0x181a81fc0`=注册方法、`0x181a81890`=checkudata。类继承注册顺序：GameActor(基='')→注册 set_asset 等；ModelActor(基=GameActor)→自有 play/stop/is_playing/get_mesh_asset/set_mesh_asset_material（**不重复注册 set_asset**）；AdditionModelActor/EffectActor/BeamActor/MaterialActor 均不注册；GameUnit(基=ModelActor)。
- render-18 候选函数 **0x18129fb50 已排除**（非 EffectActor::SetAsset；其「字符串哈希 h=h*0x1003F+c + 桶链容器 this+0x110 + miss 回退 0x180777810 按路径直载」形态符合 **UI particle 控件 effect setter**——与 §4.1 特效直路径实证一致，G30 矛盾闭环）。
- 引擎串结构：`{u32 len, u32 cap, char* ptr}`（SSO 栈上短串优化）。
- `get_mesh_asset` wrapper（约 VA 0x181345f0f）返回 `{uuid=, type=, skeletalMesh=, staticMesh=, path=}` 表——mesh 条目内含文件路径，可作 link↔路径对照的调试手段。
- `set_mesh_asset_material` wrapper（VA 0x1813464b0）：内部 GetSubsystem('Material'/'BaseResourceCache'/'ResourceCache') → **材质按名/路径走 ResourceCache，不碰数编**。

### 11.5 注册表容器与哈希（终版）+ Tier-1 改写判死

**manager（GameDataManagerImp，源 `Client\src\Game\Table\GameDataMgrImp.cpp`）vtbl 查找族**：

| vfunc | 地址（RVA） | 用途 | 容器 |
| --- | --- | --- | --- |
| +0x50 | 0x18202d0 | link 字符串查 ACTOR 表（CreateActor 路） | 同 +0x58 |
| +0x58 | 0x18202f0 | **typeid(u32) 查 ACTOR 表** | `mgr+0x230` 桶链 |
| +0x60 | 0x181e1b0 | 字符串查 MODEL 表（SetAsset 路） | `mgr+0x308` 链式 strcmp 遍历 |
| +0x70 | （未解析） | 字符串查 EFFECT 表（EffectActor::SetAsset 路） | 待查 |

**ACTOR 表桶链（fn58 反汇编全解）**：

```
buckets = [mgr+0x230] {+0x4: bucketCount, +0x10: bucket[]}
bucket  = typeid & (bucketCount-1)
node    = {+0x0: next, +0x18: typeid(u32), +0x20: entry 内联}
sentinel= [mgr+0x228]（链尾判停）
```

- **typeid = djb2-32(link 全串)**（h=5381, h=h*0x21+c）：捕获 0x8df9751e ↔ `'$$p_55a3.actor.bgd_demo_effect.root'` 精确命中。**任意 link 的注册键可离线预算**（nezha model link=0x9c14972f、jilulu actor=0xe42efc36）。
- ACTOR 条目布局（ActorEffect 实例）：`+0x0 vtbl / +0x8 共享对象 / +0x10 typeid / +0x18 {len,cap} / +0x20 link ptr / +0x138 疑似 u64 哈希 / +0x140 {len,cap} / +0x148 Inherit 父 link ptr`。
- manager 本体 +0x90/+0x98 = 地图路径串；+0xa8 起为另一按小整数索引的容器（疑似单位 id 表）。

**MODEL 条目布局（nezha 实例 dump）**：

```
+0x00 vtbl
+0x10 {len=25, cap=26}  +0x18 → '$$p_55a3.model.nezha.root'
+0x20 {len=64, cap=65}  +0x28 → 'characters/_user/p_55a3_nazha_wuwuqi_xin1_85sc_w72l/model.prefab'  ← Asset 字段
+0x30 float 1.0（Scale）
+0x38 {len=4,cap=8} +0x40 → 短串（疑 anim 名）
+0x58/+0x60/+0x68/+0x70 子对象指针（疑预载 mesh/archetype 句柄）
+0xa8.. UTF-16 内联串 "…fault.png"（default.png）
+0xc8..+0x130 内联 ASCII 槽位名串
```

**CreateActor 调用链**：失败日志串 `"[CreateActor:%d,%s] failed, GetActorTableEntry failed."` VA 0x182725cb8，lea xref 唯一 VA 0x1816fa31f；所在函数 = CreateActor impl **VA 0x1816fa290**：`[this+0x330]` 非空检查 → `[rdx]`（actor id）≤0 时从 `[this+0x44c]` 取并自减（id 分配器）→ 两级虚调用（`call [rax+0x60]` 取 manager → `call [rax+0x50]` = GetActorTableEntry(key)，key 结构体 +8 = link C 字符串）→ entry null 打失败日志。兄弟日志串：`unit already exists.` / `[CreateUnit:%d] failed, GetUnitTableEntryByName failed.` 等。

**RTTI 表族（.data 0x2da5838~0x2da5e48）**：DecorationTableEntry / ActorModelTableEntry / ActorEffectTableEntry / ActorSoundTableEntry / ActorTableEntry / ModelActorEntry / EffectActorEntry / GridActorEntry / SoundActorEntry / TextActorEntry / MaterialActorEntry / AdditionModelActorEntry / ActionActorEntry / BeamActorEntry / UnitTableEntry / SpellTableEntry / SoundTableEntry / UnitSoundTableEntry / ClientSpellTableEntry / ClientBuffTableEntry / AnimationTableEntry / MapInfoTableEntry / ConstantTableEntry / ConfigTableEntry / LightningTableEntry / EventParticleTableEntry / CameraTableEntry / UnitModelTableEntry / AccessConfigTableEntry / SpellAssistTableEntry / **MapTableManager** / **GameDataManager / GameDataManagerImp**。lookup key = u64/u32 typeid（日志 `"ModelActor ==> Init, but cannot find typeid(%llu) in ActorTable."` / `"SetModelComponent, but cannot find typeid(%llu) in ActorTable."`）。

**写入路径（启动加载）**：加载日志 `"==> Begin loading table [%s]. "` VA 0x182726588，xref VA 0x1816fd2c9——所在大函数即 LoadMapTable 族（`[r15+0x3e8]`=表名、`[r15+0x3f0/0x3f8]`=路径串、`[r15+0x68]`=状态）。相关串：`'$MapPath$/table/;$MapPath$/'`、`'%s/tableH'`、`'4_load_table'`、`'-not_read_table'`（exe 启动参数，可整表不读）。**运行时注册口静态判否**：全 dll 无 `RegisterEntry`/`ReloadTable`/`EntryManager`/`AddTableEntry`/`InsertTableEntry`（0 命中）；lua 注册面也无表注册 API。**注册表写入只发生在 LoadMapTable 加载期；运行时插入只能 frida 定位加载期容器 insert 后直接调用**。

**Tier-1 条目改写（判死）**：钩 ModelActor::SetAsset 入口 → 挂 manager 接口返回钩 → 钩 manager vfunc+0x60（MODEL 查找 0x181e1b0）→ nezha 查找返回前就地改写 entry+0x20/+0x28 为吉鲁鲁 prefab 路径。内存改写成功（回读验证），但 set_asset 应用后**目标位置无模型渲染**——apply 链消费的不止路径（预载 mesh/archetype 句柄 +0x58 系子对象与路径不一致）→ **纯路径改写 ≠ 换资产，判死**。

**下轮候选**：① hook apply 链三函数看实际读 entry 哪些字段（真实消费面，头号目标）；② 改写路径同时置空预载句柄逼懒加载；③ 钩 vfunc+0x60 直接返回伪造条目指针（克隆 jilulu MODEL 条目内存 + 改 link/asset/typeid 键）；④ Tier-2 注入：djb2 可预算 + 桶链结构已解 → 克隆 ACTOR 节点挂入桶链 + lua merge_cache 双侧同步，造全新注册 link。

### 11.6 旁路资源加载（ResourceCache）

- Urho3D ResourceCache 完整内嵌（`Urho3D\Resource\ResourceCache.cpp`），内部 API 字符串族：`BaseResourceCache_GetResource / GetExistingResource / AddManualResource / AddPackageFile / ReloadResource / Exists / FileExist / ScanDir…`。
- lua 绑定面无 ResourceCache 直接暴露；最近通道 = `game.GetTexture`（语义未通，§2.2）与 `set_mesh_asset_material`（内部 GetResourceCache）。即 **纹理/材质可按路径加载，模型/特效资产不能**（模型走 SkeletalArchetypeInstance/prefab 链，绑在数编条目上）。
- `'%s/model.prefab'`、`'/model.prefab'` 有 xref（0x180fabe00 / 0x181195a6e / 0x181196f8a / 0x18119792a），属编辑器/预制体加载链（多在 Editor 侧），游戏态价值低。

### 11.7 frida hook 点建议（按优先级，下轮直接可用）

| # | 目标 | VA（RVA） | 目的 |
|---|---|---|---|
| 1 | CreateActor impl 入口 | 0x1816fa290（0x16fa290） | dump arg3 key 结构体（+8 link 串）；在 0x1816fa2f5 call 处读 `[rax+0x50]` 解析 GetActorTableEntry 实际地址（注意 3 字节 call 坑：钩 0x1816fa2f1） |
| 2 | GetActorTableEntry 实际函数（#1 解析） | 动态 | dump manager this（容器基址）、key、返回 entry；看 HashMap 布局 → **找配套 insert** |
| 3 | set_asset wrapper | 0x181345a10（0x1345a10） | dump 入参串；读 `[obj→vtable]+0xa0` 解析三类 SetAsset（ModelActor=0x17837d0 / EffectActor=0x179c0a0 已解，GameUnit 未捕） |
| 4 | apply 链三函数 | 0x18177d5a0 / 0x181769c30 / 0x181783e30 | 换资产的真实消费字段清单（Tier-1 判死后的头号目标） |
| 5 | EFFECT 表查找 | manager vfunc+0x70（未解析） | 同 #2 法可解；容器待查 |
| 6 | LoadMapTable 加载函数 | 0x1816fd2c9 所在函数（入口向前扫） | 启动期 dump manager this + 容器 insert 调用点 → 提取 insert 函数原型供运行时复用；同时确认表真实数据源文件清单 |
| 7 | MODEL 表 `mgr+0x308` 链节点布局 | — | strcmp 遍历，注入点与 ACTOR 桶链不同需另解 |

## 12. 发布 pak 资源规则（自定义模型线上可达性）

### 12.1 实测证据链

| 证据 | 内容 |
| --- | --- |
| test_res002 已发布 pak 清单 | 含 `characters/_user/p_55a3_jilulu_19ec_a8oz/model.prefab`（改名存根）+ `res/anim/sk_basic2/...ani` + `map_ref_res/effect/**.mdl`（数编引用收集）；**无** jilulu 的 m.mdl/材质/动画 |
| 项目 res 目录 | `res/characters/_user/p_55a3_jilulu_19ec_a8oz/` **只有 model.prefab**；改名后内部引用仍指向**原始路径** `characters/_user/jilulu_19ec/model/m.mdl` 等（TNND 解密实证） |
| `res/project_resources.json` | 项目导入资源清单：`p_55a3_jilulu_19ec_a8oz` = {good_path: `jilulu_19ec/model.prefab/`, show_name: 吉鲁鲁}——编辑器「导入资源」= 复制改名 prefab + 登记此 json |
| 对照游戏 game_p_2xgc 发布 pak | 同样模式：`res/characters/_user/p_2xgc_juchui.../model.prefab` 孤存根，引用原始路径不在包内 |
| 本机 `D:\sce_online\Res\characters\_user\jilulu_19ec\` | 全套资产（mdl/材质/动画）存在——来源 = 平台资源库下载（PIE 渲染的真相） |

### 12.2 规则与生产建议

1. **地图 pak 收什么**：`res/` 下已登记资源（project_resources.json 的存根 prefab）+ 数编/场景引用收集（`map_ref_res/`，含 effect 的 mdl 等依赖）+ res/anim 等已被引用收集的动画。
2. **自定义模型（characters/_user）的依赖件不进地图 pak**——两个游戏 pak 一致实证。
3. **线上可达性已 E2E 实证（2026-08-24）**：平台按 user_libs 通道分发资源库资产（mini-runtime 载荷体系 `_m/maps/user_libs/` 落位目录与此呼应）；render-14 UIScene/UIWorld 通道（jilulu 本地模型）线上发布生效。change_model/attach 线上虽未单独复验，资产可达性已通。
4. **生产建议**：引用模型一律用**原始资源库路径**（`characters/_user/<名>/model.prefab`），不要用改名存根路径（存根内部引用仍指原始路径，且存根路径未必存在于其他用户机器）。
5. 数编 `Editor.CollectRes=true` 与「工具→资源统计和动画重定向」流程可能影响收集范围——未逐项验证。

## 13. C# managed dll 逆向结论（1.0 客户端依赖包）

### 13.1 解密方法（TNND，逐文件 XOR）

这批 dll（`official_client_deps_dll_package/23` 下的 `ui/AppBundle/managed`）全部 TNND 加密（magic `TNND` + 逐字节 XOR `CREATEEASY`，跳过 4 字节头）：

```python
key = b'CREATEEASY'
body = bytearray(data[4:])
for i in range(len(body)): body[i] ^= key[i % len(key)]  # 解出 MZ 头 .NET 程序集
```

元数据用 pip 包 **dnfile** 读（TypeDef/MethodDef 表）。解密件存 `test/temp/managed_dec/`。

### 13.2 内容地图

| dll | 大小 | 内容 |
| --- | --- | --- |
| **GameCore.dll** | 1.2MB | 触发器/.NET 游戏逻辑核心：ActorSystem、AbilitySystem、MovementSystem、EntitySystem、Protocol（实体同步协议）、ResourceType（Model/Prefab/Texture/Particle=纯路径包装，Path 属性 + 隐式 string 转换）、UserCloudData |
| **GameGraph.dll** | 400KB | **C# 场景图/渲染资源层**（Urho3D 血统）：ResourceSystem（ResourceCache/Texture2D/MeshBuilder/**Sprite2D**/Prefab/ParticleSystem 全套 Builder/RenderSurface）、MaterialSystem、SpineSystem 全套、相机/灯光/Zone/DebugRenderer |
| GameUI.dll | 450KB | UI 控件层 C# 侧（Control/Brush/DesignSystem/CameraSystem/TriggerEvent） |
| GameData.dll | 21KB | 数编全局配置壳（15 类型） |
| TriggerEncapsulation.dll | 216KB | 触发器封装（Commands/Event/QuestSystem/UIProperty/Messaging） |
| Events.dll | 25KB | 事件定义 |
| dotnet_bcl_package/6 | — | 纯 BCL（System.*/Microsoft.*），无游戏逻辑 |

### 13.3 价值判定

- **云变量：无突破点**。C# 侧只有 `GameCore.UserCloudData.UserData<T>`（纯客户端缓存容器），协议与存储全在 native（LuaScore→Entrance）。
- **渲染：Sprite2D 有 `DrawRect`/`TextureRect`/`FlipX/Y`/`UseTextureRect` 属性**（GameGraph.ResourceSystem.Sprite2D）——**图集源矩形（UV 裁剪）能力在引擎 C# 层存在**，只是 lua ui 控件属性没暴露。这解答了「tiled 图集为什么只能 clip/sprites 迂回」：能力在，接口没开到 lua。后证实该 API 是 **2.0（WasiCore）官方公开 API**，1.0 lua/触发图无任何触达路径（触发图 V1/V2 节点穷举无 Sprite2D，已证伪）。
- MeshBuilder/CreateColorMaterial/AddPrimitiveShape（程序化网格）、RenderSurface（离屏渲染目标包装 = scene 控件/视口底层）、SpineSystem 完整状态机包装均在 C# 层。**lua→C# 直调通道未发现**（大概率只能经触发图/数编系统间接驱动）。
- GameUI.CameraSystem 有完整相机包装，可与 lua `game.GetCamera/set_camera` 对照补相机参数语义。

## 14. tiled 图集专题（UI 控件 UV 全证伪 + 生产通道排序）

### 14.1 UV 属性矩阵证伪（U18~U21，PIE 实证）

对 panel/image 控件逐一尝试：`uv` / `uv_rect` / `texture_rect` / `src_rect` / `source_rect` / `rect` / `clip_rect` / `tex_rect` / `image_rect` / `draw_rect` / `uv0` / `sprite_rect` —— **全部无效**（赋值不报错——bind 层静默吞掉未知 key；截图显示完整图集）。

- **染色归因法**（排坑利器）：给每个候选控件不同 `color` 染色（红/绿/蓝/黄），一眼定位哪个控件产生效果——用它识破了假阳性（缩放盒 = sprites 定格控件与面板行叠加，不是任何 uv key 生效）。
- script-199 ui.lua:475-504 的 ui_default watch 表 = 官方全控件通用属性全集，**无 UV**——与字符串穷举一致。

### 14.2 其他 UV 线索排查

| 线索 | 出处 | 判定 |
| --- | --- | --- |
| `set_uv_tiling`/`set_uv_speed`/`set_tex_rotation`（LuaRefMaterial） | sceengine 437789-437794 | **仅编辑器 luaex**，tester 无此串 → 线上不可用，弃 |
| `get/set_subuv_subimage_index` | sceengine 438810 | 粒子编辑器 subuv 帧索引，tester 无 → 弃 |
| `common.set_background_texture_path(path)` + `set_background_texture_uv(us,vs,ue,ve)` | sceengine 444056-444059，tester 536139-536141 也有（线上可用） | 实测调用 ok 但局内画面无任何变化——背景贴图被地形/场景覆盖或仅作用于大厅/加载场景，**局内不可见，弃** |
| C# `Sprite2D.TextureRect/DrawRect` | §13（GameGraph.dll） | 能力存在但 1.0 lua 触达路径未发现（2.0 官方 API）→ 留作线索 |

### 14.3 tiled 图集最终通道排序（生产建议）

1. **webview canvas2d（推荐，线上已实证）**：图集 PNG base64 内嵌 html → canvas2d drawImage 子矩形（浏览器级 GPU 加速，一张图集一次解码）；编辑器内 imgui 通道，线上 base.ui webview 或 imgui 均可（§3）。
2. **sprites 定格（官方通道）**：§4.2（全部数值类型！字符串静默不渲）。每控件一个 draw call。
3. **clip 双层 panel（兜底）**：外 panel clip + 内 panel image 负 margin 定位。

## 15. 星火 2.0（WasiCore）专题

### 15.1 发现证据链

1. scegame-tester 字符串：`SCEImportStaticSprite2D` + `StaticSprite2D_Get/SetTextureRect_Import` 等（531298+）——Urho2D 全套以 **wasm import** 形式暴露给 C#；`==> StateGame Open Wasmtime`、`WasmtimeState::OpenWasmtimeState - appbundlePath = [%s]`——进游戏态就打开 wasmtime 子系统，游戏可带自己的 AppBundle；`SCE_on_*_export` / `SCE_GUI_on_*_export` / `SCE_PHYSICS2D/3D_*_export`——引擎期望游戏 bundle 实现的完整 ABI（生命周期/输入/UI 事件/物理回调/nanovg canvas 渲染）；`RuntimeMcpBridge.cpp`（客户端官方 Runtime MCP 桥）。
2. 磁盘 AppBundle 实物：`res/_m/maps/gamesparkcore/10/.../ui/AppBundle/managed/GameSparkCore.dll`（308KB）、`gamesystemui/14/.../GameSystemUI.dll`（537KB）、`official_client_deps_dll_package/23/.../managed/`（§13）、`res/_m/map_templates/64/map_templates/code_sample/`（**官方 C# 示例项目模板**，约 30 个示例 + GameEntry.sln）、`D:\sce_online\version-2000\appbundle`（大厅侧）。
3. **WasiCoreSDK v18** 完整随包：`res/_m/wasm/wasicoresdk/18/wasicoresdk/{api,docs,lib,resources,schemas,templates,tools}`——含客户端/服务端/共享 API 声明（.cs）、完整中文文档、SDK 程序集。
4. `api_pak_version.json` 顶层键只有 `"12"`、`"13"`、`"2000"`：2000 注册表含 wasicoresdk:18、map_templates:64、wineditor:143、gamesparkcore:10、gamesystemui:14——**当前编辑器安装即具备 2.0 能力**。

### 15.2 两个栈的关系（关键结论）

| | 1.0（现有项目） | 2.0（WasiCore） |
| --- | --- | --- |
| api_version | 12 / 13 | **2000**（show_name "wasm"） |
| 语言 | TypeScript → ts2lua（script 库） | C#（.NET 9/10, WASI）双端单源 `#if SERVER/CLIENT` |
| 触发编辑器 | V1/V2（生成 lua） | **V3**（生成 C#，`src/TriggerGenerated/*.cs`） |
| 运行时 | lua54 VM | 引擎内嵌 wasmtime 跑 `AppBundle/managed/GameEntry.dll` |
| 工程结构 | `script/` + `ui/script/` | `src/`（C#）+ `editor/data/`（JSON）+ `scene/`，无 script/ 目录 |

- **不存在 lua+C# 混合项目**：单个项目二选一（code_sample 里 HybridFlappyBird 的 "Hybrid" = 混合同步策略，不是 lua+C#）。
- 1.0 → 2.0 = **项目级移植**（官方 migrate-1to2 skill），逻辑层本质是重写；场景/地形/res/ui 资源资产同格式可直接复制。

### 15.3 2.0 渲染 API 面（对 1.0 痛点的官方答案）

| 1.0 痛点 | 2.0 官方 API |
| --- | --- |
| tiled 图集子图只能 clip/sprites 迂回 | `StaticSprite2D.TextureRect`（源矩形）/ `Canvas.DrawImage(image, sourceRect, destRect)` |
| canvas_texture_* 线上硬崩 | **Canvas/CanvasAnimated**（NanoVG 封装，官方一等公民） |
| 特效必须数编关联 | `ParticleSystem.Load("effect/x.effect")` 直载本地文件 + `CreateRuntime()` 纯代码构建 |
| 本地模型不会用 | `Prefab.Load(".../model.prefab").CreateInstance(parent)` + SceneGraph |
| spine 局限 | `AnimationSet2D.Load("x.skel/.scml")` → `AnimatedSprite2D` |
| 无离屏渲染/视口 | `Texture2D.CreateRenderTarget` + RenderSurface + 多 viewport（RTT 可贴 UI/材质/Canvas） |

关键签名（`wasicoresdk\18\api\client\*.cs`）：

```csharp
// StaticSprite2D（gamegraph_nodesystem_component_graphics.cs:1350-1434）
public RectangleF DrawRect { get; set; }        // UseDrawRect=true 时生效（本地绘制矩形）
public RectangleF TextureRect { get; set; }     // UseTextureRect=true 时生效（源纹理/UV 裁剪矩形）
public bool UseDrawRect / UseTextureRect { get; set; }
public BlendMode BlendMode / bool FlipX / FlipY / SwapXY / Color Color / float Alpha / UseHotSpot...

// Sprite2D/Texture2D 资源（gamegraph_resourcesystem.cs:1052-1090）
public static Sprite2D? Load(string path);   // res/icon/coin.png → "icon/coin.png"
public static Texture2D? CreateRenderTarget(int width, int height, bool hdr = true, int ms = 1, bool autoResolve = true);
// 通用：ResourceCache.GetResource<T>(string path)

// Canvas.DrawImage 四重载（gameui_control_primitive.cs:427-490）
void DrawImage(Image image, float x, float y);
void DrawImage(Image image, float x, float y, float width, float height);
void DrawImage(Image image, float sx, float sy, float sw, float sh, float x, float y, float w, float h);
void DrawImage(Image image, RectangleF sourceRect, RectangleF destRect);
// 另有 DrawTexture(Texture2D, ...) 同源矩形重载，可直接画 RenderTarget

// 运行时粒子（免数编，仅客户端）
var effect = ParticleSystem.Load("effect/SampleSpells/WarStomp/WarStomp/particle.effect");
effect.AddToNode(node);                       // 挂载即播放
var particle = ParticleSystem.CreateRuntime();  // 纯代码构建
var sparks = particle.AddSpriteEmitter().Timing(duration: 0.9f, loops: 1, localSpace: true);
sparks.Spawn.Burst(64); sparks.Lifetime.Range(0.35f, 0.8f); ...
// 四种发射器：AddSpriteEmitter() / AddMeshEmitter(modelPath) / AddBeamEmitter(src,dst) / AddRibbonEmitter()
particle.ExportEffect("RuntimeParticles/MyEffect", overwrite: true);  // 仅 Debug

// 模型加载
public sealed class Prefab : Resource {
    public static Prefab? Load(GameCore.ResourceType.Prefab prefabPath);
    public Node CreateInstance(Node parent);          // 必须传 parent（生命周期契约）
}
// 坑：返回 PrefabInstance 根，组件在子节点 → GetComponentInChildren<AnimatedModelComponent>()
// 或 Mesh + 组件：node.CreateComponent<StaticMeshComponent>().Mesh = Mesh.Load(".../m.mdl");

// Spine / Spriter
var spine = AnimationSet2D.Load("AnimatedSprite2DSample/spineboy.skel");  // 路径必须带扩展名（与 UI Spine 控件相反）
sprite.SetAnimation("walk", LoopMode2D.ForceLooped); sprite.SetSkin("default");
```

- Canvas 图元：线/矩形/圆角矩形/圆/椭圆/三角形/PathF（贝塞尔/弧）+ 渐变 Paint + 变换栈 + ClipRect；文本 DrawText/DrawTextBox/MeasureText/CreateFont；动画图 DrawAnimatedImage。仅客户端（#if CLIENT），订阅 OnRender 驱动；CanvasAnimated 须 `StartTiming()`；控件不 `Destroy()` 会导致移动端 FBO/纹理泄漏。
- 路径规则：场景侧走 `res/` 根，UI 侧走 `ui/` 根（`ui/image/abc.png` → `new Image("image/abc.png")`，禁止 ui/ 前缀、禁止放 user_files）；云端/资源库资源直接写 `characters/.../model.prefab`。
- 离屏渲染/多视口：`Renderer.SetupMainViewport/GetNextFreeViewportIndex/CreateViewport/SetViewport`；每视口可 `SetRenderPath("EngineRes/RenderPaths/CEMapSSAO.xml")`；RTT 结果可贴 PBR 材质、Panel（`.Image(rt,"名")` + RuntimeTexture.Register）、Canvas（DrawTexture）。程序化网格：`new MeshBuilder().SetPositions(...).SetIndices(...).SetTexCoords(...).Build()`。
- **客户端 vs 服务端**：渲染类 API 几乎全仅客户端（服务端只有 SceneGraph/物理/导航 + Mesh.Load/Shader/Texture2D.Load/ResourceCache 四个壳，配合服务端预烘焙模型碰撞）。运行时支持 Spine 4.2/4.1 及 3.8 `.skel`，旧 `.json` 不兼容。
- 序列帧选型：UI 装饰序列帧用 `Sprites` 控件 `.SpriteSheet(frameW, frameH, frameCount, framesPerRow)`；游戏对象用 `AnimatedImageSource + CanvasAnimated`（AnimatedImageSource 必须数编 GameDataAnimatedImage 定义）。

### 15.4 采用路径与限制

- **创建**：星火编辑器「新建项目」选模板（game_entry 最小可运行 / data_sample / code_sample / 专项模板）；**创建后必须用编辑器打开一次**（刷新 `src/WasiCoreSDK.props`、AGENTS.md、docs/sdk|api|schemas、ai/tools/）。备用路线：从 map_templates 复制模板（改 map_settings.json 的 ProjectName/MapDisplayName，仍需编辑器打开一次）。
- **构建**：csproj `net9.0`，5 配置（Server/Client × Debug/Release + Client-Resource）；`dotnet build src/GameEntry.csproj -c Server-Debug` + `-c Client-Debug`（**两端都要编**）；产物 GameEntry.dll → `AppBundle\managed\`（服务端）与 `ui\AppBundle\managed\`（客户端）（dotnet build 不自动复制，编辑器内点调试才自动编译+复制）。日常开发可不装本机 .NET SDK（编辑器用随包绿色 SDK）。
- **发布**：发布测试游戏 → 创作者中心邀请码 → PC/Android 对战平台或 **TapTap App 内嵌客户端**（已上线）。环境双轴：部署 alpha/production × 运行时 debug/test/formal，3×3 组合**云数据互相隔离**。客户端二进制兼容：CapabilityVersion + `Runtime.Supports(feature)` 门禁。
- **迁移（migrate-1to2）**：官方明示 AI 辅助迁移无法 100%；**以 .ts 为迁移基准，别读 lua**；7 步（侦察 → 新建 game_entry → 子系统选策略（A 配置直迁 ini→JSON / B 结果代码化 ts→C#）→ 迁数据 → 迁触发器 → 迁公式 → 迁场景/资源/UI + 手补清单）。场景/地形资产同格式可直接复制（已实测）；坐标不要盲目换算（2.0 默认镜头转 90°，可配回）。现实成本：bgd_sce_framework 整套 lua 基建在 2.0 无对应物，等于框架重做或放弃。
- **当前限制**：缺部分 1.0 小卡片机制（商城已有）；小地图（用 Canvas 自绘或 MiniMapIcon 过渡）、UI 场景官方在规划；场景拼接不支持；UI 编辑器覆盖不完整（代码建控件需主动 `AddToVisualTree()`，整页用 `UI.Page()` 根容器）；捏人动画重定向需手动触发。定性：「与 1.0 的低代码体验、功能覆盖、稳定程度仍不完全等价」。
- **Runtime MCP（2.0 官方运行时调试）**：客户端进程监听本机 TCP bridge（**127.0.0.1:18765**）；编辑器 MCP 暴露固定工具 `runtime_call_tool` 转发；脚本侧工具（debug.ping/ui.snapshot/ui.find/ui.click/input.*/debug.capture_screenshot/scenegraph.materials/material.probe）不在 tools/list 里（预期行为）。条件：必须经编辑器调试启动（`Game.IsDebugTestMode == true`），**正式包/线上不开放**。项目可注册自定义工具（`RuntimeDebugToolRegistry.Register`）。与我们的 bgd_mcp_bridge 不同代际不同物（我们的桥是给 1.0 api13 编辑器打的 C# 补丁）；做 2.0 项目时 AI 调试闭环官方自带。
- **云数据**：2.0 = 结构化多桶 KV + UUID 列表 + 唯一名称注册表 + 跨用户 ACID 事务 + 游标扫描 + 模糊名称搜索；传输层被 `IUserCloudDataProvider` 完全封装。新逆向线索：2.0 云数据 op 面大概率仍走 Entrance 通道 + MessagePack 族。
- **战略判断**：1.0 项目维持 lua（本台账既有结论是生产答案）；新项目/愿意重写的项目上 2.0（渲染/云数据/UI/调试全面超越 1.0，code_sample 模板零成本起步）。2.0 已上线（编辑器线上 + TapTap 内嵌客户端）。

## 16. 坑与教训沉淀

### 16.1 探针方法论

1. **控件属性类型敏感**：particle/sprites 等控件的数值属性必须 number/table 原生类型——字符串数字/逗号串让控件**静默不渲染（无日志）**。
2. **base.wait 并发注册语义**：同一 tick 内连续 `base.wait(5000,A); base.wait(10000,B); base.wait(15000,C)`，三个定时器**全部相对注册时刻**触发（t0+5/+10/+15），不是顺序串联！要"状态驻留 N 秒再进下一态"必须嵌套回调或单态一局一跑。且 base.wait 在事件回调 for 循环里不真正按序延迟（同帧全执行），还会引发框架 timer "on_timer函数为空" 刷屏——探针时序分隔用「游戏-更新 + 帧计数」模式（实测可靠）。
3. **MCP capture_game（WGC 离屏恢复路径）单张耗时可达 ~30s**——9s 短窗口全部落空，45s 长窗口才稳定命中。判别渲染与否用 capture_game（游戏视口裁剪）+ CopyFromScreen（真屏）双通道对照。
4. **假阳性教训**：多控件同屏探针时，一个控件的渲染结果可能叠加在另一组控件区域上造成误判——归因必须染色/隔离/单态对照（染色归因法，§14.1）。
5. **PIE 客户端 native 日志不落本地**（logs\game 无新文件）——native 层资源加载失败在 PIE 没有日志出口，只能靠截图目视 + 变体矩阵 + lua 侧 getter 复读。
6. **先查 getter 再上 frida**：script-199 `unit.lua:105-111` 早有 `mt:get_asset()`/`mt:get_model_path()`——lua 侧 getter 复读就能判定 native 是否生效，比 frida 内存 dump 省事得多。
7. **restart_last 拾取规则**：已入库文件的内容变更能拾取；**新增文件需 full 调试**（服务端探针新文件不生效实录——探针直接写既有 init.lua 顶层并加注册时日志）。
8. **星火 lua 注释前缀 `|` 整卷判死**：行首 `|--`/`|---`（Emmy 变体）→ `unexpected symbol near '|'`，文件整卷 require 失败。接收外部 lua 文件先 grep `^\|--` / `^\|---` 清洗。悬案（无害）：构建管线对已入库旧文件首行 `|--` 有剥离行为，机制未明。
9. **探针一律用 `base.ui.create(base.ui.panel{...})` 正式 DSL**：`base.ui.view{type=...}` 裸构造能建控件但缺少模板层 watch 布局处理，渲染行为不可靠；`base.ui.image` 不存在（image 是全控件通用属性，显示图片用 `base.ui.panel { image = ... }`）。
10. **大段内嵌 HTML 用独立数据模块**（`return [[...]]`）存放——内联进探针文件容易把文件搞坏（编辑器工具对长 bracket 串处理翻车实录）。
11. **不要直接改项目 `script/`、`ui/script/`**（构建管线覆盖）——探针一律写 `.bgd/src` + 运行时构造。obj/*.lua 是产物不是源（手改会被 trigger 管线静默还原，git 也看不出）。
12. 探针分组开关改 if 块时弄丢收尾 end——改开关矩阵代码后先 build 再跑。
13. `game.get_game_table('ActorModelData')` 返回空表不代表无数编——native 注册表不经此暴露。
14. 日志方括号数字（`[9244]`）不是进程 pid，是线程/通道 id；PIE 游戏态进程 = exe 名为 `SCE` 的进程（Get-Process SCE；模块含 SCEEngine.dll）。MemoryStat 日志行（约每 30s 一条）可用作进程存活心跳。
15. 相机 API：`game.GetCamera()` = `{position={x,y,z}, rotation={roll,pitch,yaw}, camera_node_position, focus_distance}`；`game.set_camera{position=, rotation=, focus_distance=, time=ms}`。test_res002 相机 position=[3325,3325,10] rotation=[-70,-7.16,...]，注视点≈(3329,3325,0)；3D 视口无地形（纯天空盒），actor 放注视点即可见。

### 16.2 frida / 逆向工具坑

1. **frida 17 python API 变化**：`Module.getBaseAddress` 已删除 → `Process.getModuleByName('SCEEngine.dll').base`；`session.enumerate_modules()` 也没有 → JS 侧 `Process.enumerateModules()`。
2. **模块名大小写**：磁盘是 `sceengine.dll`，进程内模块名是 **`SCEEngine.dll`**（getModuleByName 大小写敏感）。
3. **editor PIE 进程布局**：3 个 `sce` 进程（exe 无扩展名，D:\sce_online\version-13\SCE）：含 SCEModule.dll/SCECustomControl.dll（276 模块）= 编辑器壳；另两个（142 模块）= 游戏态进程。ctx getter 对编辑器壳 lua 状态也返回非空 ctx（但 [ctx+0x10] 无效）——**按 [ctx+0x10] 非空鉴别真游戏进程**。
4. **3 字节 `call [rax+0x50]`（FF 50 50）** frida 无法 intercept（`unable to intercept function at ...`）——钩点前移 4 字节到上一条指令起点（rax 已是 vtable）；6 字节 `call [rax+0xa0]` 可直接钩。
5. **`readUtf8String(N)` 带长度**遇 NUL 抛 `can't decode byte 0x00`——未知长度 C 串用 `readCString()` 或不带参 `readUtf8String()`；带长度只用于引擎 String{len,ptr} 精确读。
6. **后台探针残留**：python 进程存活致旧钩子刷屏/冲突——每轮先 `Get-Process python` 清点；探针 DUR 不宜过长。
7. **frida Rust 绑定**：ScriptHandler 回调线程内**实例状态不可靠**（File 句柄报 os error 6、String 字段读出 NUL）——跨线程状态走全局 OnceLock + 逐条 append 打开文件才稳。
8. **Win64 参数序**：frida NativeFunction 首参进 rcx，wrapper 的 magic（0xfff0b9d7）在 rdx；ctx getter（thunk 0x181cbcb01）签名 = `(lua_State* L, 0xfff0b9d7 magic) → 游戏上下文`。frida 捕获的 native AV 以 JS error 抛回不杀进程。getter 调用频次极高（每帧多次），called 守卫每进程独立。
9. **lua54.dll 165 导出可用**（lua_replace 是宏无导出 → settop(0)+pushstring 造参）。
10. **luaL_Reg 定位法**（无 lea xref 时）：字符串 VA → 全二进制搜 8 字节小尾 VA → 命中 .rdata 的 luaL_Reg 表项 → +8 即函数指针。对 game.*/ui.*/score.* 全部适用（工具化：`test/temp/find_luareg.py`）。
11. 静态定位 vtable 不可行（本 dll vtable 不含 COL RVA 反向引用，RTTI 链断裂）——frida 从实例 `[obj]+偏移` 直接读。
12. 工具链：examples/find_xref.rs / disasm_at.rs 用 rustc 直编绕过 frida-sys 的 libclang 依赖（`rustc --edition 2021 -O examples/find_xref.rs -o test/temp/find_xref.exe`；disasm_at 加 `--extern capstone=...`）。

## 17. 遗留问题 / 下轮入口

1. **frida 运行时注入注册表主线**（免数编唯一活路）：hook 点清单 §11.7 已备——#1→#2 解析 GetActorTableEntry 容器与 insert（编辑器 PIE 可 attach，先核对进程 Path 防挂错）；若 insert 可运行时安全调用（参数 = 构造的 TableEntry），则「虚拟数编」真正打通（配合 lua 层 merge_cache 双侧同步）。
2. **apply 链消费面逆向**（0x18177d5a0/0x181769c30/0x181783e30 实际读 entry 哪些字段）——Tier-1 判死后的头号目标；候选：改写路径同时置空预载句柄逼懒加载 / 查找钩返回伪造条目指针（克隆+改字段）/ 桶链注入全新注册条目（djb2+桶链已解）。
3. **GameUnit 的 vtable+0xa0 覆写地址未捕**；EFFECT 表查找（vfunc+0x70）地址与容器未解析；MODEL 表 `mgr+0x308` 链节点精确布局未解。
4. **若注册表插入判死的退路**：ResourceCache 通道（材质/纹理自由）+ UI particle 控件 .effect 直路径 + 主世界 unit_change_model + 数编条目脚本化预生成（§7 流程）。
5. **大厅/mini-runtime 通道**：平台级「加载其他游戏」的正解（官方换游戏流程，线上可达）——mini-runtime 组装大厅盒子流程 + dll 补丁触发。大厅动态加载机制反查：大厅能动态加载渲染其他游戏的模型/地图，其资源加载链必然有免数编入口（或大厅专属数编下发机制）——从 lobby 源码/抓包入手。
6. **编辑器侧自动化**：bgd_mcp_bridge 在 xdeditor 态直调 EDITOR.load_map/unload_map（编辑器工作流自动化；非线上）。
7. **JS→lua（web_message）未通**：鉴别法 = 页面里 `typeof scelua` / `typeof chrome.webview` 用 run_js 读回上屏，确认桥是否注入（§3.7 三假设）。
8. **数编脚本化 bgd 工具化**：bgd_sce_tools 集成「写 ini + 删 obj save_info + bump 戳」一键流程；手写 GUI 页面通道的编辑器兼容性（编辑器保存会清手写页）。
9. **fbx/gltf→m.mdl 离线转换**（编辑器导入管线做的转换，独立课题）。
10. 其他小项：native 表真实数据源精确文件清单（hook LoadMapTable 读文件点）；`set_mesh_asset_material`/`get_mesh_asset` 语义深化；`GetGameWorldInfos` 返回空表待查；knead_human 完整时序（解 defaultui_63 UPAK）；miniblink 离屏合成性能压测；数编 CollectRes / objref.txt / full.ref 资源收集机制；gameui 52（api 2000）无 script 目录悬案；服务端 actor attach + 联机同步表现（sync 字段语义）；ActorAdditionModel 实建（自动骨骼挂载，socket 可空）；2.0 云数据 op 面 Entrance+MessagePack 线索。
