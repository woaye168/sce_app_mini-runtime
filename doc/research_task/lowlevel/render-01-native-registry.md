# 渲染底层 API：native 注册块穷举与各通道结论

> 研究日期：2026-08-23 | 状态：静态分析完成，探针未跑
> 证据：sceengine-strings.txt（api13 sceengine.dll）、script-199、client_base、game_p_2xgc、test_res002/.bgd/libs
> 前置阅读：editor-patch 仓库 ui-render-atlas-canvas.md（渲染链/图集/canvas_texture 实测）

## 0. 一句话结论

游戏 lua 的 native 边界远不止 ui.*：`game.*`（LuaGame）才是世界内渲染主力，另有类绑定块（EffectActor/GameWorld/texture set_data）与 viewport/webview/video 控件通道。**特效 .effect 本地文件可直接喂 UI particle 控件**（框架实证）；**模型直载候选已锁定**（unit:change_model/attach_model + model.prefab 约定，待实测）；**图集子图确认无 native UV 通道**，新突破口 = `game.GetTexture`+texture:set_data 与 webview html。

## 1. native 注册块总清单

### 1.1 游戏运行时（Client\src\Game\Lua\SubModule，StateGame 可用）

行号 = sceengine-strings.txt 行号；块边界以 cpp 源路径标记切分。

- **LuaGame `game.*` 442696-443310**：send_ui_message/get_game_table/get_player_list/cast_spell 系/坐标变换(world_to_screen/screen_to_xy/yz/xz/Raycast)/**create_actor/remove_actor/set_actor_owner/set_actor_asset**/set_actor_shadow/socket 系/attach_actor_to_socket/attach_actor_to_anchor/detach_actor/actor_play/actor_play_anim/actor_pause/resume/stop/actor_set_layer_mask 系/actor_enable_raycast/音效系/set_actor_scale/grid actor 系/launch/impact site 系/set_actor_text/**set_actor_material_parameters**/actor_play_anim_bracket/播放速度·时长·百分比·时间/anim_handle_* 全套(442940-443011)/**create_debug_draw_actor + debug_draw_line/point/circle/sector/rectangle/text + clear_debug_draws**(443014-443029)/战争迷雾与视野系/CreateUnit/unit 位置·朝向·缩放/unit_add_buff/remove_buff/riseletter 系/血条·小地图图标/highlight/tint/xray/outstroke/circle·line·sector_selector/**shake_camera/set_camera/GetCamera/CameraFocus/lock_camera/camera_rotate_around_point/set_camera_attribute/switch_camera**(443181-443206)/**set_screen_effect**/use_light_group/use_scene_timer/set_scene_timer_param/flow_rate/**set_light_weather_attribute_animation**/set_global_roughness_enable/**set_scene_timer_postprocess**/euler_to_quaternion/lerp_quat_align/set_background_anim/**unit_attach_model/unit_detach_model/unit_change_model**(443240-443245)/set_static_mesh_predraw_enable/**GetTexture**(443250)/set_max_particle_memory_limited/unit_register_bone_chain 系/occluding_camera_group 系/set_focus_clip_param/set_max_draw_distance/set_max_shadow_distance/异步加载块系/set_cipr
- **LuaUI `ui.*` 443417-443571**（全集见 ui-render-atlas-canvas.md §3；含 set_control_prop/imgui_*/canvas_*/canvas_texture_*）
- **LuaUISound `ui_sound.*` 443582-443596**：play_ui_sound/play_ui_sound_ex/play_sound/stop_sound/get_sound_position/is_playing/stop_all_sound
- **LuaIO `io.*` 443603-443723**：read/write/create_dir/exist_file/walk_dir/serialize/deserialize/read_cache/read_pak_entries/extract_pak/extract_pak_file/DownloadFile/UploadFile/unzip 系/zip_file/add_resource_path/remove_resource_path/select_file 系/add_watch 系（StateGame 多数被 isolation 置 nil）
- LuaLog 443725-443817 / LuaLoader 443818-443832 / LuaCommon 443833-443879
- **LuaLobby 443880-444619**（lobby.*/common.* 等多表；含 `common.set_particle_lod_level` 444062、`set_particle_dynamic_batch_enabled` 444066）
- LuaIM 444620-444697 / LuaGlobalChat 444698-444733 / LuaUpdater 444734-444759 / LuaLogin 444760-444776 / LuaJS 444777-444784 / LuaTimer 444785-444845
- **LuaScore `sce.s.*` 444851-444974**（见 cloudvar-01/02）
- **LuaMapPublisher `map_publisher.*` 444979-445038**（含 connect_random_terrain_server 自定义 TCP 通道）
- **LuaHttp `sce.httplib.*` 445042-445110**：request(url/method=GET|POST/header/query/json/input=字符串|文件|流|函数/output=路径|流|函数/progress/max_recv_speed/max_send_speed) + create/create_stream
- LuaShortcut 445112-445132 / LuaRandom 445133-445219

### 1.2 类绑定块（LuaBinding/LuaExtend 区 445235-445340）——最关键新发现

- **`LUA_METATABLE_TEXTURE` + `set_data`（445236-445238）+ `game.GetTexture`（443250）**：疑似自定义纹理 userdata 通道，未验证，头号探针目标（canvas_texture 的线上安全替代候选）。
- **EffectActor / AdditionModelActor / BeamActor**：set_attribute/get_attribute/set_asset/play/attach_to/detach/get_mesh_asset/set_mesh_asset_material（445308-445318）
- **GameWorld**：set_map_dir/create_scene/purge/setup_viewport/destroy_viewport/add_game_unit/remove_game_unit/add_game_actor/remove_game_actor/set_render_target_link/set_camera_info（445320-445335）——配合 viewport 控件 = 小地图同款「世界画面→UI」通道
- MiniMapProxy/MiniMapIconProxy：is_valid/add_container_icon（445337-445340）

### 1.3 编辑器侧（Editor\src\LuaExport，仅 StateEditor；游戏 lua 不可用）

EditorLuaSystem 436708 / luaex_PluginsManager 437077 / luaex_Material 437755 / luaex_Common 438158 / luaex_DebugManager 438252 / luaex_MapInfo 439245 / luaex_TaskPipline 439423 / luaex_CSharpCommon 439464 / luaex_Prefab 439553 / luaex_Spine 439879 / CSharpLua 440221 / LuaDockTitle 441867。
渲染相关（编辑器上下文专用）：set_uv_tiling/set_uv_speed(437789-437791)、set_model(438453)、set_model_materiallist(437658)、set_particle(437676)、set_particle_position/euler_angle/scale+parent 版(438524-438529)、set_model_name(438990)、set_model_material(439140)、**get/set_required_subimages_horizontal/vertical(438655-438689)、get/set_subuv_subimage_index(438810-438811)**、set_light_* 系列。

### 1.4 控件类型全集（424818-424846）

image data panel button label input progress canvas **nvg_canvas** scene sprites spine virtual_joystick(+_slider/_listener) particle dock_area window **viewport** color_packer color_panel lite_code minimap_canvas **webview video** scroll_rect spline_curve spline_bg bezier_curve

### 1.5 属性名注册块（Key.K_*）

公共 452112-452146（STATIC/DISABLED/SWALLOW_EVENT(S)/ENABLE_DRAG/DROP/Z_INDEX/ENABLE/SHOW/COLOR/GRAY/ROUND_CORNER_RADIUS/IMAGE/MASK_IMAGE/BORDER/OPACITY/LAYOUT/TRANSITION/ROTATE/SCALE/LOW_LEVEL/CLIP/RENDER_GROUP/FLIP_X/FLIP_Y/CUSTOM/FIX_SCALE/FIX_BORDER/TEXT/FONT/LOOP/SPEED/META_INFO_STR/BLUR_IMAGE）；dock 452180-452194；layout/label 452419-452455；transition 452463-452475；scroll 452477-452497；button 452499-452500；input 452502-452508；progress 452514-452519；**scene 452524-452556**（K_CAMERA_INFO/K_FOV/K_ORTHOGRAPHIC/K_MODEL/K_ANIM/K_PARTICLE/K_BUFF/K_LIGHT/K_AVATAR_PATH/K_ICON_PATH…）；sprites 452560-452568；spine 452572-452577（旁有 `.skel`）；vj 452582-452592；**particle 452595-452604**（K_EFFECT/K_PLAY/K_STOP/K_PARTICLE_SIZE/K_DIRECT_SCALE/K_PARTICLE_ENDFLY/K_OFFSET_PERCENT/K_AUTO_SCALE/K_PARTICLE_SCALE/K_VIEW_MODE）；viewport 452606-452607；color_packer/panel 452610-452619；webview 452622-452629（K_URL/K_HTML/K_RUN_JS/K_WEB_MESSAGE…）；video 452632-452640；scroll_rect 452643-452651；spline/bezier 452656-452668。
**全集确认：无 UV/源矩形/texcoord/subimage 属性**（texcoord 仅出现在 shader 源码串；subimage 仅 atlas 内部错误消息 428600）。

## 2. 各渲染通道最底层 API

| 通道 | 最底层 API | 形态/证据 |
|---|---|---|
| 图像 | ui.set_control_prop(id,'image',path) | 代理机制 ui.lua:69-94；来源：相对路径/@图名/http(image_cache)/**绝对路径** |
| 图集子图 | 无 native UV 通道 | sprites 网格帧/clip 视窗/draw_image 仅目标矩形(brush.lua:50)；canvas_texture 线上崩 |
| UI 内 3D 模型 | scene 控件 model 属性 | `set_control_prop(id,'model',{name='斧王',facing,position,pause,anim,anim_time,scale})`（test/scene.lua:9-16）；name=数据编辑器单位名 |
| 世界内模型/特效 | game.create_actor(name,...) + set_actor_asset / unit_attach_model/unit_change_model(path) | actor.lua:81、unit.lua:1232-1241；name/asset 为数编注册项 |
| UI 内特效 | particle 控件 effect 属性 | **直接吃 .effect 文件路径**（框架 bench 实证 'libs/res/particle/demo/p_12sc_effect_new_6o1_dl47/particle.effect'，test_res002 .bgd/libs/client/cgui/bench/page_builtin.lua:111） |
| spine | spine 控件 resource 属性 | 无扩展名路径，引擎自补 `.skel`（452577；test/spine.lua:36） |
| 视频 | video 控件 src/video_url 属性 | **GUIVideo = webview + HTML5 `<video>` 标签**（452635-452640 内嵌 HTML 模板）；官方用法 https URL（template/video.lua:19-36） |
| 任意 2D/WebGL | webview 控件 url/html/run_js | template/webview.lua；html 属性可直接给整页 HTML——被低估的逃生口 |
| 世界内即时绘制 | game.create_debug_draw_actor + debug_draw_line/point/circle/sector/rectangle/text | 443014-443029 |
| 屏幕后处理 | game.set_screen_effect / set_scene_timer_postprocess / use_light_group / set_light_weather_attribute_animation | 443211-443228 |
| 世界→UI 投射 | GameWorld.create_scene/setup_viewport/set_render_target_link + viewport 控件 | 445320-445335 + 452606-452607（MiniMapManager 441085-441092） |

## 3. 模型/特效本地文件加载结论

- **特效 (.effect)**：**UI 通道不走数据编辑器**——particle 控件 effect 属性直接接受 pak 感知 .effect 路径（框架 bench 实证；bgd 工具链 res 同步规则 particle→`res/effect/<prefix>/x.effect` 印证）。用户遇到的「不显示」更可能是 view_mode/play/auto_scale 属性或 .effect 内容与 UI 粒子视口（2D 视角）不匹配——列入探针复现。**世界内**（EffectActor）必须数编注册（eff.cache 数据驱动，actor.lua:68-81；443337-443338）。
- **模型**：资产形态 = 目录约定 `<name>/model.prefab`（JSON，`skeletalMesh: "<dir>/model/m.mdl"`，game_p_2xgc/res 实证）+ m.mdl（Urho3D 二进制）+ 动画。数据编辑器的「模型」本质就是引用 prefab 路径（data.lua:105 `Asset='eqpt/weapon/sk_jk_wp1/model.prefab'`）。**lua 直载候选三条**（均待实测）：`unit:change_model(path)` / `unit:attach_model(path,...)`（参数名即 path）、scene 控件 `model.name` 传 prefab 路径。外来模型（fbx/gltf）需先转 m.mdl——编辑器导入管线做的转换，离线转换器是另一课题。
- **spine**：完全自由，`.skel` 直路径。
- **video 本地文件**：src 是 URL；`file:///` 绝对路径或 `data:` URI 待实测。

## 4. imgui_* 全清单与评估（443431-443452，共 12 个）

imgui_begin_view/end_view/begin_ui/end_ui/begin_wrapper/end_wrapper/props/props2/data/state/view_data/view_state。全部声明式（props/data 传属性表，底层与 RMGUI 同一属性系统），**无独立 draw list、无自定义纹理/UV 采样**——画图像仍走 image 属性。结论：imgui 不能解决图集子图/自定义纹理问题，价值在即时模式高性能调试 UI。

## 5. 探针清单（按优先级）

1. `game.GetTexture` 签名 + texture userdata `set_data` 数据格式（RGBA raw？LZ4？）——canvas_texture 线上替代候选
2. particle 控件 `.effect` 直路径「不显示」复现（view_mode/play/auto_scale 矩阵 + 日志定位）
3. `unit:change_model/attach_model` 传 res 内 model.prefab / m.mdl 路径；scene 控件 model.name 传 prefab 路径
4. video `src` 喂 `file:///D:/...mp4` 与 `data:video/mp4;base64,...`
5. webview `html` 属性 data:URI + canvas2d/WebGL 性能实测
6. GameWorld + viewport 复刻小地图链路（任意 3D 场景→UI 区域）
7. nvg_canvas 的 CanvasDrawImage 是否有多参数重载（试探传源矩形参数）
