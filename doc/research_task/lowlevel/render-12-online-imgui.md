# imgui 通道线上（tester PCBox）实证：video https mp4 + webview canvas2d

> 研究日期：2026-08-24 | 状态：✅ 线上真局实证（截图存证 test/temp/tester_p55a3_game.png）
> ⚠️ **修正（2026-08-27，dl-01）**：本文「JS→lua（web_message）线上仍未通」已被推翻——2026-08-26 双向桥完整打通并三端（PC/Android/iOS）上线实测：根因 = imgui 控件不在 `base.ui.map` 被 `ui_events` 静默丢弃，解法 = 手动登记 + `register_event`（cgui `cg.webview` 内置）。见 [webview-bridge.md](../research/webview-bridge.md) §2、[pak-io-native.md](../research/pak-io-native.md) §7.7。
> 前置：render-03（imgui 通道 PIE 实证）/ render-05（webview canvas2d + 线上未验证遗留）
> 探针：test_res002 RenderProbe U28/U29（随 pak v97 发布测试环境，真局 imgui 每帧直驱）

## 0. 一句话结论

**imgui 立即模式直驱的 video 与 webview 控件在线上（星火对战平台 tester PCBox 构建）真实渲染、不崩溃**：https mp4 播放器完整加载播完（2:18/2:18 进度条走满），webview canvas2d 棋盘格页面完整上屏。canvas2d 自定义渲染通道（tiled 图集/任意 2D 的生产级替代）线上可用正式坐实。

## 1. 实证矩阵（tester test 环境 p_55a3 真局，2026-08-24 14:57）

| 控件 | 驱动方式 | 线上结果 | 证据 |
| --- | --- | --- | --- |
| video（https mp4，oss.laf.run） | imgui 每帧 props `{show, video_url, layout}` | ✅ 播放器 UI + 进度 2:18/2:18 播完（末帧黑屏属正常停帧） | 截图 tester_p55a3_game.png 左上 |
| webview（内嵌 html canvas2d 棋盘格） | imgui 每帧 props `{show, html, layout}` | ✅ 橙蓝棋盘格渲染（带滚动条，render-05 §3 已知 overflow 问题） | 同截图中部 |

- **未崩溃**：与 canvas_texture 的「编辑器可用、线上 PCBox 硬崩」形成对照——imgui 驱动的 video/webview 走 miniblink 离屏合成，线上构建支持完整。
- 旧线索「bench 注释称 video 线上会崩」**证伪**（至少 imgui 驱动 + https 源不崩）。

## 2. 生产意义（对四大痛点的最终答案更新）

1. **tiled 图集** → **webview canvas2d 通道（线上已实证）**：图集 PNG base64 经 pak 内嵌数据模块 → 页面 drawImage 按 tile 映射 → lua 经 run_js 每帧推状态。绕开 clip/sprites 性能问题，浏览器级 GPU 加速。消滚动条：html 加 `overflow:hidden` + 精确尺寸（render-05 §3）。
2. **视频** → imgui video + https mp4（线上已实证）；file:// 被拦（render-03），必须 http(s)/data:。
3. 模型/特效/spine → render-02/06/10 既有结论不变（编辑器 PIE 通道；线上未逐一复验，但同属 StateGame 常规控件，风险低）。
4. ~~JS→lua（web_message）线上仍未通（render-05 §2 假设待鉴别；本次未复测）~~ → **2026-08-26 已通**（[webview-bridge.md](../research/webview-bridge.md) §2 机制破解 + 三端上线实测）；run_js（lua→JS）单向可用已够 canvas2d 通道用。

## 3. 验证环境记录

- 进游戏自动化：cloudvar-09 §1（`-game=p_55a3 -tag=test -ai_test=1`）。
- 截图：mini-runtime capture CLI（WGC，对 iconic 窗口离屏截图有效）。
- RenderProbe 保留在 test_res002（U28 video + U29 webview，场景-加载完成即驱动，无需交互）。
