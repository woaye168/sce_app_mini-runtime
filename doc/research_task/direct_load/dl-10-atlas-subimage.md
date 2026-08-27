# dl-10 图集子图直载专项（静态部分）

> 日期：2026-08-28 | 状态：✅ 静态完成（无编辑器运行）
> 目标：在星火编辑器 1.0 lua 项目的 UI 上渲染图集子图（源矩形/UV 裁剪），不走整图、不走预切散图。
> 前置：[render-08-atlas-uv.md](../lowlevel/render-08-atlas-uv.md)（UV 矩阵全证伪）、render-15（'@gameui' 包组件源已解密在 `test/temp/gameui-48-script/`）、[dl-06-pascalcase-pie.md](dl-06-pascalcase-pie.md)（ui 表 82 函数 PIE 实锤）。

## 0. 通道状态板（图集子图直载全景）

| # | 通道 | 状态 | 依据 |
| --- | --- | --- | --- |
| 1 | UI 控件 bind UV/源矩形属性（uv/texture_rect/src_rect… 12 候选） | ❌ 判死 | render-08 染色归因实证 + ui_default watch 表源码无 UV【实测+源码实锤】；本轮补证：bind 全部 `set_xxx` 最终 funnel 到 native `ui.set_control_prop(id,key,...)`（script-199 `common/base/ui/ui.lua:68-77`），未知 key 由 native 静默吞掉——直调 `set_control_prop` 也是同一通道，无绕过价值【源码实锤】 |
| 2 | `base.ui.sprites` 帧索引定格（图集=精灵表） | ✅ 可用（官方） | script-199 `common/base/template/sprites.lua` watch 全参；defaultui/lib_lobby/appui 多处官方用法【源码实锤】；仅支持等分帧格，不支持任意源矩形 |
| 3 | clip 双层 panel（外 clip + 内 image 负 margin） | ✅ 可用（兜底） | render-08 |
| 4 | webview canvas2d drawImage 子矩形 | ✅ 可用（推荐） | webview-bridge + render-12 线上实证 |
| 5 | **native canvas 控件 + draw_image 负偏移裁剪** | 🟡 新线索，待 PIE 实测 | ui 表 canvas API 双端 dump 实锤，见 §3 |
| 6 | **game_hud 飘字/血条原生 HUD 的图集源矩形**（Texture.picture+Rect / bitmap font CharSet） | 🟡 新线索（引擎原生支持，限 HUD 场景） | 发布 pak 模板 json 实锤，见 §2 |
| 7 | C# Sprite2D.TextureRect/DrawRect | ❌ lua 触达路径未发现 | render-07 |
| 8 | LuaRefMaterial set_uv_tiling 等 | ❌ 编辑器 luaex 专属，tester 无串 | render-08 |
| 9 | `common.set_background_texture_uv` | ❌ 局内不可见 | render-08 实测 |
| 10 | canvas_texture_* 过程纹理当「子图加载器」 | ❌ 无「从图片文件取像素」入口，拼不成裁剪链 | §3 分析 |

## 1. '@gameui' 组件源通读（test/temp/gameui-48-script/，62 文件）

全部 62 个组件源已扫（uv/rect/clip/source/frame/atlas/sprite/texture 关键词全量 + 图片相关组件逐读）。【源码实锤】

- **无任何 UV/源矩形/裁剪 bind 转发**。所有图片类组件（`simple_ui/picture.lua`、`icon_square_frame.lua`、`icon_circle_frame.lua`、`icon_num.lua`、`btn_icon.lua`、`progress.lua`、buff 系、rect 面板系）图片通道只有 `image = bind.xxx`（整图路径字符串）+ 布局尺寸。
- 与裁剪沾边的仅两处，均非源矩形：
  - `icon_circle_frame.lua`：`mask_image` 属性——alpha 蒙版抠圆（遮罩图），不是矩形裁剪；
  - `active_button.lua`：`clip = true`——控件矩形裁剪（render-08 通道 3 同族）。
- `progress.lua` 用 `base.ui.progress` 的 `progress_type`（含九宫格），是进度条专属裁切，不可泛化。
- `uiscene.lua`：`RenderTarget = bind 属性转发 → image`（render-15 已记录）——这是「把 viewport 渲染结果整张贴到 panel.image」的通道，不含子矩形。
- 结论：'@gameui' 包里**没有**未被注意的源矩形转发通道，render-08 的判死面无需修正。

## 2. game_hud 原生 HUD：引擎原生支持图集源矩形（★ 新发现）

发布 pak / 项目目录的 `game_hud/` 模板（实物：`test/temp/app_box/extracted/game_hud/`）：

- **`acriselettertemplate.json`（飘字模板）**：Layout 控件 `Type:"IMAGE"` 支持
  `"Texture": { "picture": "ACRiseLetterHUD.AllInOne", "Rect": {X,Y,Width,Height} }` —— **图集源矩形**，坐标任意（非等分帧格）【实锤 json】。另有 `Type:"IMAGEPPT"`（按 name 选 Scheme，每项一个 Rect）。
- **`riseletter.json`（位图字体定义）**：每字体 `AtlasOffset` + `CharSet[{Char, Rect{X,Y,W,H}, OffsetX, OffsetY}]`，字符=图集 `riseletter.png` 上的任意子矩形【实锤 json】。
- **`acbloodtemplate.json`（血条模板）**：IMAGE 控件用 `TexturePath` 整图 + `Rect`（布局矩形），INDICATOR/RULER 用 `Skin` 整图——**血条侧无源矩形**，源矩形能力是飘字/位图字体子系统专属。
- 编辑链：触编 riseletter 对象编辑器（xdeditor-169 `plugin/obj_editor_ui`，MODULE_TYPE.RISELETTER）→ 预览 `uicanvas:create_gameplay_ui_object(...)`；预览代码把 IMAGE 控件的 `IMAGE` 字段当 Font 名用（`node_preview.lua:3671-3674`）→ 即飘字 IMAGE 实为由位图字体驱动的图集字符，发布期合成 AllInOne 图集。
- 运行时 API（game 表，dl-06 dump 实锤）：`create_riseletter_by_templatename / set_riseletter_position / set_riseletter_world_position / set_riseletter_unit / set_riseletter_callback / remove_riseletter`；script-199 `common/base/base_lua_plus/unit.lua:74+` 有 `base.create_riseletter_by_templatename(position, text, templatename, color, fontsize)` 薄封装。
- **判定**：引擎在飘字 HUD 子系统里有完整图集子图渲染（任意源矩形 + 偏移）。但它渲染在**世界空间头顶 HUD**层，不是 base.ui 屏幕 UI；模板需触编/手写 json 定义在项目 `game_hud/`（项目可控，bgd 项目均有此目录）。适用场景 = 头顶图标/状态字/飘字类展示；想拿它当通用屏幕 UI 图集子图 = 场景不匹配，仅作备选记录。【json 实锤 + 源码实锤，PIE 未测】

## 3. ui 表 82 函数全量 dump（★ 新产物 + canvas 新线索）

产物（lua_api_dump，锚点 `imgui_begin_view` / `CanvasDrawImage`）：

- `test/direct_load/ui-editor.out`（D:\sce_online\version-13\sceengine.dll，164 条目含签名）
- `test/direct_load/ui-tester.out`（线上 tester SCEGame，164 条目含签名）

**编辑器与线上 tester 的 ui 表条目集合完全一致（均 164 = 82 函数双名）**——canvas 全族在线上也存在【dump 实锤】，与 dl-06 PIE pairs 枚举数吻合。

canvas 相关清单（签名 = dump 推断）：

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `draw_image / CanvasDrawImage` | `(canvas, image_path, x, y, w, h)` | **只有目标矩形，无源矩形参数** |
| `clear / CanvasClear` | `(canvas)` | |
| `draw_line/draw_circle/fill_circle/draw_polygon/fill_polygon/fill_triangle` | 各坐标参 | |
| `rotate / CanvasRotate` | `(canvas, x, y, angle)` | 旋转中心+角度 |
| `canvas_path_line_to/path_stroke/path_bezier_curve_to` | | 路径描边 |
| `canvas_texture_set_name/set_size/set_fill_color/fill_pixel/fill_rect/fill_circle/clear_circle/get_pixel_color/set_compressed_data/get_compressed_data/set_blur` | | **过程纹理**（canvas 自带像素缓冲逐像素读写） |

脚本层封装（script-199）：

- `common/base/template/canvas.lua`：`base.ui.canvas` DSL 控件（`template/init.lua` ui_types 白名单含 'canvas'）+ `ui:get_brush()` 返回 texture_brush（过程纹理操作）。
- `common/base/ui/brush.lua`：`base.ui.brush:create(canvas_id)` → `draw_image(path,x,y,w,h)` 等画笔封装（同样无源矩形）。
- `common/test/canvas.lua`：官方用法示例——`ui.add_child('', json{type='canvas',...})` + `ui.draw_image(handle, 'image/xxx.png', 180, 180, 40, 40)` + rotate 组合。
- 真实用例：`common/base/p_ui/line_chart.lua` 折线图（canvas+brush）。

### 3.1 ★ 待测通道：canvas 负偏移裁剪

`draw_image(canvas, 图集, -cropX, -cropY, atlasW, atlasH)`：把控件尺寸设为子图尺寸 (cropW×cropH)，图集按负偏移画入——若 canvas 把内容裁剪到自身纹理/控件边界（canvas 是固定尺寸纹理缓冲，超界像素天然无处可去），即等效源矩形裁剪：

- 每控件一次 draw_image 调用、native 渲染（比 webview 轻、比 sprites 灵活——任意矩形非等分帧格）；
- 未知点：① canvas 控件类型在 1.0 lua 项目 game 侧是否可正常创建渲染（脚本白名单有，引擎双端有函数，但**无官方游戏内用例**，line_chart 是否上线过未知）；② 超界像素是否真的不画出控件外（若画出去了就得外套 clip panel，退回通道 3）；③ 图集路径用 `@<ProjectName>/...` 运行时形态是否被 draw_image 接受；④ 每帧 redraw 还是一次性（test/canvas.lua 的 on_update 每帧 clear+redraw 模式暗示内容可能需维护）。

### 3.2 canvas_texture_* 当裁剪链？——拼不上

`set_name(id, 名字)` 语义是命名/绑定纹理名，不是加载图片文件（无证据）；`get_pixel_color` 只能读 canvas 自身缓冲；`set/get_compressed_data` 是缓冲压缩串往返。**没有「从磁盘图集读像素」入口**，组不成「读图集→取子矩形→写新纹理」链。❌ 排除（除非 PIE 实证 set_name 可吃图片路径，概率低）。

## 4. api-13 包源码搜索补记

- 官方 lua 侧「图集」一词的唯一所指 = **sprites 控件**（lib_control `skill_group.lua:497-502` editable_prop 选项 `{text='图集', value=false}` → sprites 参数组；defaultui `skill.lua:395-414`「图集专属」注释）。即官方语境里图集子图 = 等分帧格索引，任意源矩形在 lua UI 层不存在官方通道——与 render-08 结论互证。【源码实锤】
- lite/2（编辑器内嵌 lite 文本编辑器）有 `renderer.draw_rect/set_clip_rect`，宿主是 lite 自有 C API，与游戏 base.ui 无关，排除。
- app_box/client_base 等包无其他 image 源矩形用法。

## 5. 下一步 PIE 实测清单

1. **canvas 负偏移裁剪**（§3.1 四个未知点，test_res002 一个探针文件可覆盖：建 4 个 canvas 控件分别画同一张图集的不同负偏移区域 + 染色对照，截图判定）。优先级最高——若成立即为继 webview canvas2d 之后的第二条任意源矩形生产通道，且纯 native。
2. 顺带验证 `ui.canvas_texture_set_name(canvas_id, 图集路径)` 是否加载图片（若成立，3.2 重开）。
3. （低优先）飘字通道可行性摸底：项目 `game_hud/acriselettertemplate.json` 手写一个 IMAGE 控件模板 + 自定义 riseletter.json 字体，`base.create_riseletter_by_templatename` 拉起，确认自定义图集路径/矩形是否生效（HUD 场景专用）。

## 6. 关联

- [render-08-atlas-uv.md](../lowlevel/render-08-atlas-uv.md)（UV 判死面，本文未推翻任何一条）
- [dl-06-pascalcase-pie.md](dl-06-pascalcase-pie.md)（ui 表 82 函数 PIE 枚举）
- render-15（'@gameui' 包定位解密）、render-07（C# Sprite2D.TextureRect）
- 工具：`examples/lua_api_dump.rs`（锚点 `imgui_begin_view` 导出全表）
