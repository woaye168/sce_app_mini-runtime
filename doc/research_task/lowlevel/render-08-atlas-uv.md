# render-08 — tiled 图集/UV 通道专题（UI 控件 UV 矩阵证伪 + 最终通道结论）

> 2026-08-24 | 问题：tiled 导入的图片集如何高性能显示图元（用户痛点：clip 双层 panel 与 sprites 定格都有性能问题）
> 结论：**lua UI 控件无任何可用 UV/源矩形属性**（四候选全证伪）；生产可用通道 = ① sprites 定格 ② clip 法 ③ webview canvas2d（推荐，见下）；引擎 C# 层有 Sprite2D.TextureRect 能力但未暴露给 lua（render-07）。

## 1. UV 属性矩阵（U18~U21，test_res002 PIE 实证）

对 panel/image 控件（`base.ui.panel { image = 图集, <key> = {0,0,0.5,0.5} }`）逐一尝试：

| 候选 key | 结果 |
| --- | --- |
| `uv` / `uv_rect` / `texture_rect` / `src_rect` / `source_rect` / `rect` / `clip_rect` / `tex_rect` / `image_rect` / `draw_rect` / `uv0` / `sprite_rect` | **全部无效**（赋值不报错——bind 层静默吞掉未知 key；截图显示完整图集） |

- **染色归因法**（U21，排坑利器）：给每个候选控件不同 `color` 染色（红/绿/蓝/黄），一眼定位哪个控件产生效果——本轮用它识破了 U19b 的假阳性（缩放盒 = sprites 定格控件与面板行叠加，不是任何 uv key 生效）。
- script-199 `common/base/ui/ui.lua:475-504` 的 ui_default watch 表 = 官方全控件通用属性全集：`clip/image/flip_x/flip_y/scale/rotate/gray/opacity/...` **无 UV**——与实证一致（render-01 字符串穷举结论成立，未被推翻）。

## 2. 其他 UV 线索排查

| 线索 | 出处 | 判定 |
| --- | --- | --- |
| `set_uv_tiling`/`set_uv_speed`/`set_tex_rotation`（LuaRefMaterial，luaex_Material.cpp） | sceengine 437789-437794 | **仅编辑器 luaex**（D:\BuildPC\NE_pd\Editor\src），scegame-tester 无此串 → 线上不可用，弃 |
| `get/set_subuv_subimage_index` | sceengine 438810 | 粒子编辑器 subuv 帧索引（xdeditor particle_editor 用），tester 无 → 弃 |
| `common.set_background_texture_path(path)` + `common.set_background_texture_uv(us,vs,ue,ve)` | sceengine 444056-444059，**tester 536139-536141 也有（线上可用）** | U19b 实测调用 ok 但局内画面无任何变化——背景贴图被地形/场景覆盖或仅作用于大厅/加载场景，**局内不可见，弃** |
| C# `GameGraph.ResourceSystem.Sprite2D.TextureRect/DrawRect` | render-07（client_deps GameGraph.dll） | 能力存在但 lua 触达路径未发现（疑似触发图/节点图系统专用）→ 留作线索 |

## 3. tiled 图集最终通道排序（生产建议）

1. **webview canvas2d（推荐新项目验证）**：图集 PNG base64 内嵌 html → canvas2d drawImage 子矩形（浏览器级 GPU 加速，一张图集一次解码）→ render-05 已实证编辑器内可用（imgui 通道）；线上 base.ui webview 用户 demo 已验证可用（webview-demo.lua），canvas2d 是其子集能力，风险低。**待办：线上 tester 实测一次**。
2. **sprites 定格**（官方通道）：`base.ui.sprites { image=图集, sprite_size={w,h}, row_frame_count=N, frame_count=1, start_frame=K, end_frame=K, playing=false }`（全部数值类型！字符串静默不渲）→ 显示第 K 帧图元。U19b 实证渲染正常。性能画像：每控件一个 draw call，帧切换改 start_frame/end_frame 即可。
3. **clip 双层 panel**（兜底）：外 panel clip + 内 panel image 负 margin 定位。

## 4. 坑沉淀

- **假阳性教训**：多控件同屏探针时，一个控件的渲染结果可能叠加在另一组控件区域上造成误判——归因必须染色/隔离/单态对照（U19b→U20→U21 三轮才定案）。
- `base.ui.image` 在 bgd/script-199 运行时**不存在**（image 是全控件通用属性，不是控件类型）——显示图片用 `base.ui.panel { image = ... }`。
- `base.ui.view{type=...}` 裸构造能建控件（webview/image 都建成功）但缺少模板层 watch 布局处理，渲染行为不可靠——探针一律用 `base.ui.create(base.ui.panel{...})` 正式 DSL。
- 探针文件里写图集路径用运行时形态 `@<ProjectName>/image/sprites/bgd_game_client/<file>`（@p_55a3 来自 map_settings.json，非项目目录名）。
