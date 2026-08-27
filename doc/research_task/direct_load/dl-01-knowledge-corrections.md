# dl-01 知识修正：webview/video 过时结论批量修订

> 日期：2026-08-27 | 状态：✅ 完成（文档修订，无新增实测）
> 依据：[webview-bridge.md](../../research/webview-bridge.md)（2026-08-26 编辑器 PIE 全事件流实测 + 三端视频 demo 上线实测）、[pak-io-native.md](../../research/pak-io-native.md)（2026-08-26 PIE + tester PC 线上 E2E + Android/iOS 实测）

## 修正原则

- 不删改历史原文，以「~~划线~~ + 修正块/批注 + 链接」标注，保留研究史。
- 修正点全部指向 2026-08-26 的两份定稿文档。

## 修正清单（逐条）

### 一、render-05-webview-bridge.md（修正密度最高）

| # | 原表述 | 修正 | 置信级 |
| --- | --- | --- | --- |
| 1 | 头部状态「JS→lua（web_message）未通（路由待查）」 | JS→lua 桥已完整打通并三端上线实测。根因 = imgui 控件不在 `base.ui.map` 且未 `register_event`，`ui_events.on_web_message` 静默丢弃；解法 = 手动三步登记（cgui `cg.webview` 已内置） | 【实测】PIE + 三端线上 |
| 2 | §0 「JS→lua 桥…消息未到达 lua」 | 同上，已通 | 【实测】 |
| 3 | §2 实测矩阵「scelua.send_string → 一条都没到」 | 当时是缺登记所致，非引擎限制；已补修正批注 | 【实测】 |
| 4 | §2 「JS→lua 未通的假设 ①②③」 | 鉴别完毕：假设①命中（路由依赖 `base.ui.map` 登记）；假设②证伪（scelua polyfill imgui 通道照常注入） | 【实测】 |

### 二、render-03-imgui-channel.md

| # | 原表述 | 修正 | 置信级 |
| --- | --- | --- | --- |
| 5 | §1「离屏 WebView2 合成进游戏 UI 纹理」 | 内核实为 **miniblink**（render-05 §1 字符串实证）；CEF/WebView2 非游戏内主路径 | 【源码实锤】引擎字符串 |
| 6 | §1/§6「file:// 被 CEF 拦」 | 拦的主体是 miniblink 不是 CEF；且「被拦」仅针对 PC html 文本加载姿势——播放器 HTML 写成文件 + `url='file://...'` 加载即可播（三端实测） | 【实测】 |
| 7 | §4/§6「bench 注释称 video 线上会崩，生产慎用」 | render-12 已证伪（线上真局 https mp4 2:18 播完不崩）；完整生产方案见 pak-io-native.md（pak 提取 + file:// + 双轨音频 + 三端实测） | 【实测】线上 tester |
| 8 | §5 遗留「webview run_js/web_message 双向通信实测」 | 已完成（webview-bridge.md §2/§3） | 【实测】 |

### 三、render-12-online-imgui.md

| # | 原表述 | 修正 | 置信级 |
| --- | --- | --- | --- |
| 9 | §2「JS→lua（web_message）线上仍未通（render-05 §2 假设待鉴别）」 | 2026-08-26 已通：机制破解 + 三端上线实测（webview-bridge.md §2、pak-io-native.md §7.7） | 【实测】 |

### 四、lowlevel/README.md

| # | 原表述 | 修正 |
| --- | --- | --- |
| 10 | 文档索引 render-05 行「JS→lua 未通」 | 标注已通 + 链接 |
| 11 | 遗留清单「webview JS→lua 鉴别」条 | 补终版结论（双向桥全通，三端实测）；同时保留「StateGame 渲染管线不驱动 webview」结论的有效性说明（webview-bridge.md §6.1 确认其为真结论、与互通无关） |
| 12 | 速查「webview = CEF 合成进 UI 纹理」 | 修正为 miniblink；补双向桥已通 |
| 13 | 速查缺视频生产方案 | 新增一行：视频已完整突破（pak 提取 + file:// + 双轨音频，pak-io-native.md） |
| 14 | 速查「scene 控件 StateGame 死亡（模型 UI 通道未打通）」 | render-14 早已修正但速查未跟上：死亡的是用法不是控件，UIScene/UIWorld 通道编辑器+线上均生效 |
| 15 | 速查待补「webview run_js 双向通信」 | 划线标注已完成 |
| 16 | 进度日志 | 追加 2026-08-27 本条修正记录 |

## 仍成立、未被推翻的相关结论（防误改）

- **base.ui webview 在 StateGame 不渲染 = 真结论**（渲染驱动阀门，与 JS 互通无关）——游戏局内用 imgui/cgui 通道，大厅才用 base.ui（webview-bridge.md §6.1 末行明确确认）。
- render-05 §5 的「死亡点=渲染驱动」机制结论保留，仅补注 JS→lua 部分。
- imgui 每帧驱动要求、scene 控件的两阶段认知史（render-03 → render-14 修正）保持原样。

## 对本任务（direct_load）的启示

1. **cgui 是局内 UI 新基建**：`cg.webview` 开箱内置双向桥/console 转发/手势捕获，局内 webview 类实验直接用 cgui，不要再手写 imgui 三步登记（除非无 cgui 环境）。
2. **图集子图主线新增一条已实证逃生口**：webview canvas2d 通道（render-05 §3 / render-12）在双向桥打通后升级为**全双工自定义 2D 表面**——lua↔JS 都可驱动，图集子图 UV 裁剪在浏览器侧是平凡操作（drawImage 源矩形），可作为图集子图目标的保底生产通道（已线上实证）。线索三原生通道攻坚失败时的 fallback。
3. **PascalCase 漏网（线索二）已有线上实证先例**（io.*），本轮枚举 render/resource 相关函数时方法论直接复用 pak-io-native.md §2/§3。
