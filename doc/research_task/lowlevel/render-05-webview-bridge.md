# webview 深度：miniblink 内核 + lua↔JS 桥 + canvas2d 自定义渲染通道

> 研究日期：2026-08-24 | 状态：canvas2d 渲染 + lua→JS（run_js）实证；JS→lua（web_message）未通（路由待查）
> 前置：render-03（imgui 通道）

## 0. 一句话结论

游戏内 webview 控件 = **miniblink 离屏浏览器**（`WebEnvMiniblink.cpp`，UA=Chrome/69，非 CEF/WebView2 主路径）渲染进 UI 纹理；经 imgui 直驱在 StateGame 可跑**任意 HTML/canvas2d**（自定义 2D 光栅渲染逃生口，实证棋盘格页面）；lua→JS 用 `run_js` 属性实证可用；JS→lua 桥（`scelua.send_string` / `chrome.webview.postMessage`）在 StateGame imgui 路径下消息未到达 lua（事件路由或桥注入条件待查）。

## 1. 内核与桥（scegame-tester-strings 542120-542165 实证）

- 实现：`Client\src\Game\WebEnv\Impl\WebEnvMiniblink.cpp` + `WebViewportClientMiniblink.cpp`（miniblink = 国产轻量嵌入式浏览器内核）；字符串里另有 WebView2 初始化错误处理（Failed to find WebView2-Runtime 等）——双实现，miniblink 为主（游戏内嵌），WebView2 可能用于别的环境。
- 引擎向每个页面注入 polyfill：

```js
if (!window.chrome.webview) {
    window.chrome.webview = {};
    window.chrome.webview.postMessage = function(message){ chrome_webview_postMessage(message); };
    window.scelua = { send_string: function(msg){ chrome.webview.postMessage(msg) } ...
```

- JS→lua：`scelua.send_string(str)` → lua 侧 `on_web_message` 事件（官方 base.ui 用法：bind event on_web_message，client_base lobby.lua:541 / xdeditor resource_store_web_ui.lua:33-49 实证，消息为 JSON 字符串）。
- lua→JS：控件属性 `run_js`（"Execute java script => error: %d" / "AsyncExecuteJS failed" 日志串）。
- 相关属性：url/html/run_js/web_message/web_type/web_dev_tools/web_import_script/isolated（452622-452629）；`web_import_script` = 注入额外 JS（"ImportScript success"）。

## 2. StateGame 实测（test_res002 PIE，imgui 直驱）

| 实验 | 结果 | 证据 |
| --- | --- | --- |
| imgui webview + html 原始串（canvas2d 棋盘格 + 周期重绘） | ✅ 渲染 | capture_1787506928.png |
| `ui.set_control_prop('main[wv_probe]>webview0', 'run_js', js)` | ✅ 页面执行（LUA2JS-OK 黄字上屏） | capture_1787507160.png |
| JS `scelua.send_string('tick n')` → `ui_events.on_web_message` | ❌ 一条都没到 | 日志无记录 |
| `ui.register_event('probe_wv2', 'on_web_message')` / `ui.RegisterEvent(层级id, ...)` | 调用不报错但无效 | 同上 |

- **imgui 控件的层级 id**：`imgui_state().id` 在 begin 块内返回如 `main[wv_probe]>webview0`——可直接用于 `set_control_prop`（run_js 实证）。
- JS→lua 未通的假设：① web_message 事件路由依赖 base.ui 声明式创建时的 subscribe_now（imgui 路径没走）；② StateGame 的 webview 不注入 scelua 桥（桥注入条件可能与 state/通道相关）；③ on_web_message 的 ui_events 分发需要控件名精确匹配（层级 id vs 短名）。下轮鉴别：在页面里把 `typeof scelua` / `typeof chrome.webview` 用 run_js 读回（经画面文字显示），确认桥是否注入。

## 3. canvas2d 自定义渲染通道（tiled/任意 2D 的生产级替代）

**链路已通**：lua 把图集 PNG 经 base64 内嵌进 html（require 数据模块，pak 感知）→ 页面 canvas2d 按 tile 映射 drawImage（浏览器级速度，GPU 加速）→ webview 显示。lua→JS 数据通道 = run_js（可每帧推状态）。这套东西等价于一个自由 2D 绘制表面，绕开 clip/sprites 的性能问题。

- 注意 mini地图/滚条：页面尺寸超出控件会出现滚动条（本轮棋盘 400x300 控件 400x300 仍出滚动条——body 默认 margin/滚动区所致，html 里 `overflow:hidden` + 精确尺寸可消）。
- 性能未知项：miniblink 离屏合成每帧拷贝成本、多实例上限——需压力实测。
- 线上（tester PCBox）可用性**未验证**（canvas_texture 的前车之鉴：编辑器能跑线上崩——webview 是生产大厅在用的通道，理论上线上支持更好，但必须实测）。

## 4. 坑沉淀

- 大段内嵌 HTML 用独立数据模块（`return [[...]]`）存放——内联进探针文件容易把文件搞坏（编辑器工具对长 bracket 串处理翻车实录）。
- `base.wait` 在事件回调中不可靠（render-03 §6 已记）；帧计数器模式（游戏-更新 + frames==N）是可靠的延迟手段。
- imgui 每帧驱动的 webview 会持续重放 html（每次 props 给同样值时 native 侧应判同不 reload——本轮未见明显闪烁，但未严格验证）。

## 5. W2 补测：base.ui webview 在编辑器 PIE 的完整行为（2026-08-24）

用户 demo（webview-demo.lua，线上已验证可用）揭示前置开关 `ui.set_enabled_in_game('webview', true)` 与环境检测 `ui.check_webview_environment()`。在编辑器 PIE（StateGame）实测：

| 调用 | 结果 |
| --- | --- |
| `ui.check_webview_environment()`（开关前/后） | **均返回 true**——编辑器环境检测通过，检测不是阀门 |
| `ui.set_enabled_in_game('webview', true)` | ok 无报错 |
| `base.ui.view{type='webview', name='wv_raw'}` | 创建成功（控件 id `ui-1206-wv_raw`），html/event 属性赋值均不报错 |
| 截图验证 | **不渲染**（capture_1787513366：无任何 webview 痕迹） |
| JS `scelua.send_string` → lua `on_web_message` | 一条未到（页面根本没跑起来） |

**机制结论**（回答"为什么 base.ui webview 编辑器里死、imgui 里活"）：环境检测与开关在编辑器都为 true/可用，控件也能创建——**死亡点在渲染驱动**：webview 控件的状态机需要被每帧驱动（imgui 通道的 begin_view/begin_ui/props/end_ui/end_view 循环干了这件事），base.ui 的 StateGame 渲染管线不驱动 webview 类型控件（页面不加载→不渲染→JS 桥自然无消息）。线上大厅有效是因为大厅（lobby state / StateApplication）的 UI 管线驱动它。用户 demo 注释「需要在经过线上的 lobby 才能看到 webview 效果」与此一致。

**生产建议**：编辑器内调试 webview 用 imgui 通道（render-03）；正式发布走 base.ui + set_enabled_in_game（用户 demo 线上已验证）。两者用同一份 html 内容即可。
