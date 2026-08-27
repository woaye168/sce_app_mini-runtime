# webview 双向桥全解（lua ↔ JS，三通道实现）

> 最后验证：2026-08-26（编辑器 PIE 全事件流实测 + 三端视频 demo 上线实测）
> 本文讲清 webview 控件的 lua↔JS 双向通信机制、三种实现通道（cgui / 原生 imgui / base.ui 声明式）的接法与坑。
> 视频播放实战（pak 提取 + 内联播放 + 双轨音频）见 pak-io-native.md；本文聚焦**双向桥本身**。

## 1. 引擎层的双向通道（native 实证）

webview 控件（GUIWebView）暴露一组属性（sceengine 字符串 452621-452629 实证）：

```
url / html / run_js / web_message / web_type / web_dev_tools / web_import_script / isolated
```

**JS 侧桥是引擎自动注入的 polyfill**（render-lowlevel §3.7）：

```js
window.chrome.webview.postMessage = function(message){ chrome_webview_postMessage(message); };
window.scelua = { send_string: function(msg){ chrome.webview.postMessage(msg) } };
```

所以页面里 `window.scelua.send_string(json_str)` 就能发消息——**imgui 通道也照常注入**（探针 run_js 自证 `typeof scelua === 'object'`）。

## 2. JS→lua 派发链（核心机制，踩坑都集中在这）

```
页面 scelua.send_string(str)
  → 引擎 chrome_webview_postMessage
  → 引擎调 lua 全局 ui_events.on_web_message(控件id, message)
  → script-199 base/ui/event.lua：查 base.ui.map[控件id]，调 ui.event.on_web_message(回调)
```

**两个必要条件，缺一不可**（这就是"消息发不出来"的全部根因）：

1. **`base.ui.map[控件id]` 必须有 `event.on_web_message` 回调**——`ui_events` 里 `if not ui then return` 静默丢弃不在 map 里的控件
2. **`base.ui.gui.register_event(控件id, 'on_web_message')` 订阅**——不订阅引擎不派发

base.ui 声明式创建时这两步自动做（`event={on_web_message=...}` 触发 subscribe）。**imgui 建的控件两步都没人做**——这就是 imgui 通道"消息不通"的真相（曾误判为引擎不支持）。

### imgui/cgui 通道接通桥的手动步骤

```lua
-- 1. begin 块内读 state 拿真实控件 id（形如 main[view]>P0>webview0）
local st = imgui.state()  -- 或 cgui core.state()
local cid = st.id or st.name

-- 2. 登记 base.ui.map + 挂回调
base.ui.map[cid] = base.ui.map[cid] or {}
base.ui.map[cid].event = { on_web_message = function(msg) ... end }

-- 3. 订阅事件
base.ui.gui.register_event(cid, 'on_web_message')
```

cgui 的 `cg.webview(opts.on_web_message=fn)` 已内置这三步（leaf.lua），开箱即用。

## 3. lua→JS：run_js 下发

控件属性 `run_js` = 要在页面执行的 JS 字符串（props 内容变化触发执行一次）。派发自定义事件的标准形式：

```lua
-- lua 侧
local cmd = string.format(
    'window.dispatchEvent(new CustomEvent("GlobalEvent",{detail:{message:%s}}))',
    base.json.encode({ type = 'replay' }))
-- 设进 webview 的 run_js 属性
```

```js
// JS 侧
window.addEventListener('GlobalEvent', function(e){
    var m = e.detail.message;  // { type='replay', ... }
    if (m.type == 'replay') { ... }
});
```

**run_js vs web_import_script 的区别**（易混）：

| | run_js | web_import_script |
| --- | --- | --- |
| 时机 | 页面加载完后，lua 随时主动调 | 页面加载过程中引擎自动导入执行 |
| 用途 | 发指令/改状态（一次性动作） | 给页面预装能力/SDK |
| 谁触发 | lua 主动 | 引擎（页面加载时） |

`web_import_script` 的典型场景：url= 加载**外部网页**（不是你的 html）时要往里注入 JS——引擎在加载时把你给的 JS 导入执行。minigame 加载外部游戏页注入 sdk_js 就是这个用法（startup/mini_game/main_page.lua）。

## 4. 三种实现通道对照

| | cgui（cg.webview） | 原生 imgui | base.ui 声明式 |
| --- | --- | --- | --- |
| 视图生命周期 | `cg.mount` 每帧驱动 | 手写 on_post_update + begin/end_view | base.ui.create 创建即活 |
| props | `cg.webview(id, opts表)` | `imgui.props2(fn, sig)` 函数+签名 | 模板属性直写 |
| state 读取 | begin 块内 `core.state()` | begin 块内 `imgui.state()` | 不适用（事件回调式） |
| JS→lua 桥 | `opts.on_web_message` 自动登记 | **手写 map 登记+订阅**（§2） | `event={on_web_message=...}` 自动 |
| 手势事件 | `opts.on_web_event`（注入捕获脚本） | 同左（自己注入或借 cgui） | 无（自己写 JS） |
| 控制台转发 | `opts.web_console_log=true` | 自己注入转发脚本 | 自己写 |
| 渲染驱动 | StateGame imgui 循环驱动 | 同左 | **需大厅管线驱动**（StateGame base.ui webview 不渲染，render-lowlevel §3.5） |
| 适用 | 游戏局内 UI（推荐） | 底层/无 cgui 环境 | 大厅/启动页 |

参考实现：`examples/apidemo/cutscene_cgui.lua`（cgui 版全功能）/ `cutscene_imgui.lua`（原生 imgui 版）/ `webview_ctrl.lua`（base.ui 版，你的原始研究）。

## 5. cgui webview 完整能力（2026-08-26 定型）

```lua
cg.webview('my_wv', {
    url = 'https://...',          -- 或 html = '<html>...'（html 优先于 url）
    run_js = '...',               -- lua→JS 执行
    web_dev_tools = true,         -- F12 开发者工具（PC）
    web_console_log = true,       -- JS console.log/warn/error + uncaught → lua log.info
    isolated = false,             -- 独立窗口 webview（实测无效果）
    on_web_message = function(msg) end,   -- JS scelua.send_string → 此回调
    on_web_event = function(ev) end,      -- ★ 手势事件（见下）
    layout = { ... },
})
```

### on_web_event 手势事件（cgui 注入捕获脚本实现）

手势名全集：`press` / `release` / `click` / `double_click` / `double_press` / `long_press` / `move`

ev 字段（命名对齐 UI 手势体系）：

| 字段 | 含义 | 出现于 |
| --- | --- | --- |
| `x, y` | 当前触点坐标（逻辑像素 clientX/Y） | 全部 |
| `step_x, step_y` | 本次移动增量（相对上一帧 move） | move |
| `delta_x, delta_y` | 从按下点起的累计位移 | move/release/click |
| `duration_ms` | 按下时长（press 起算） | release/click/long_press |

实现要点（踩坑记录）：
- **pointer 事件统一鼠标/触摸**：`pointerdown/pointermove/pointerup` 单事件跨端不重复（别同时挂 mousedown+touchstart，移动端一次触摸触发两个 → 计数 +2）
- **双击判定按「两次 pointerdown 间隔<400ms」**，不依赖 window 全局时序变量（时序写错会让 click 都不发）
- 长按 = 按住 500ms（计时器，期间 pointerup/移动取消）
- **注入形态**：html= 拼 html 尾部；url= 走 web_import_script 注入外部页

## 6. 坑清单（本次实战全部踩过）

### 6.1 桥与 JS 工程坑

| 坑 | 根因 | 解法 |
| --- | --- | --- |
| JS→lua 消息发了 lua 收不到 | imgui 控件不在 base.ui.map + 没 register_event | §2 三步（cgui 已封装） |
| **JS 对象写成 `=` 语法错误** | Lua 写顺手 `{__wvev=1}`（JS 是 `:`） | DevTools 报 SyntaxError 直接暴露；写 JS 时警惕 |
| console 看不到 JS 报错 | 无转发通道 | cgui `web_console_log=true`（console+uncaught 全转 lua） |
| url= 外部页注入不了脚本 | html 拼尾部只适用自有页 | web_import_script 注入 |
| base.ui webview StateGame 不渲染 | 渲染驱动阀门，StateGame 管线不驱动 webview（**真结论，与互通无关**） | 游戏局内用 imgui/cgui 通道；大厅才用 base.ui |

### 6.2 三端兼容坑（PC / Android / iOS 实战全记录，2026-08-26）

按踩坑时间线排列，每条都经真机验证。视频专项细节另见 pak-io-native.md §7/§7.1。

**手势与触摸类**

| 坑 | 现象 | 根因 | 解法 |
| --- | --- | --- | --- |
| **点一次计数 +2** | 移动端点一下，lua 收到两次事件 | 同时挂 `mousedown` + `touchstart`：移动端一次触摸两个都触发 | **只用 pointer 事件**（`pointerdown/move/up`），鼠标触摸统一，跨端不重复 |
| 移动端按钮 click 不触发/迟钝 | iOS/Android 点按钮偶尔没反应 | WKWebView click 有 300ms 延迟，且快速点击可能被吞 | 按钮补挂 `touchstart`（与 click 双保险） |
| **iOS 长按放大镜** | 长按视频/页面弹出系统放大镜与选中菜单 | iOS WKWebView 默认 touch-callout + 文本可选中 | 全元素 CSS：`-webkit-touch-callout:none; user-select:none; -webkit-user-select:none` |
| 页面可滚动/可捏合缩放 | 移动端双指能放大页面、边缘拉出滚动条 | 缺 viewport 声明，body 默认 margin | `<meta viewport user-scalable=no>` + `overflow:hidden` + `touch-action:none` + `body{margin:0}` |

**画面布局类**

| 坑 | 现象 | 根因 | 解法 |
| --- | --- | --- | --- |
| **iOS 右缘 1-2px 透出竖条** | 全屏视频最右边漏出一条背景光 | webview 物理像素与 CSS 逻辑像素非整数缩放的舍入缝（疑似引擎合成对齐问题） | 双保险：① 视频 `object-fit:cover` + `transform:scale(1.01)` 放大 1% 溢出裁切；② webview 下层垫**全屏纯黑底**（fullscreen 面板 `color='rgba(0,0,0,1)'`）挡住透出 |
| 画面四周留黑 | 移动端视频上下/左右大黑边 | `object-fit:contain` 等比缩放，屏幕比例≠16:9 | 移动端用 `cover` 填满（裁上下），PC 用 `contain`；demo 用 `VIDEO_FIT` 参数按端切换 |
| **视频被顶到下半屏** | 全屏播放时视频只占下半屏，上半黑 | cgui fullscreen 面板是 `direction=col` 布局，套 box 后 webview 与 box 成两个子元素纵向排列 | fullscreen 面板**直接放 webview 单子元素**，黑底由面板 color 承担，不套 box |
| Android 两侧留黑 | 视频未铺满宽度 | 同 contain 问题 | 同 `VIDEO_FIT='cover'` |
| 中文按钮乱码 | iOS 上重播/跳过按钮文字变问号方块 | `io.write` 写盘无 BOM，HTML 缺 charset 时浏览器按 latin-1 解码 | 播放器 HTML 必带 `<meta charset="UTF-8">` |

**移动端适配类**

| 坑 | 现象 | 根因 | 解法 |
| --- | --- | --- | --- |
| 按钮被刘海/圆角遮挡 | iOS 全屏时按钮贴近屏幕边缘难点击 | 安全区（safe-area） | 按钮整体偏移（demo 用 120px 避开刘海区） |
| 按钮热区太小 | 移动端点不中 | 按 PC 尺寸做的按钮 | 移动端按钮 88px 高 + 34 字号（demo `BTN_OPTS` 按端切换） |

**视频播放类**（详见 pak-io-native.md §7）

| 坑 | 解法 |
| --- | --- |
| iOS 裸绝对路径白屏 | 必须 `file://` 前缀（PC 宽容、iOS 严格） |
| html 文本加载时 file:// 视频被拦 | 播放器 HTML 写成文件放 mp4 旁，`url='file://...'` 加载，视频 src 同目录相对 |
| iOS 原生 video 控件强制全屏 | 弃原生控件，webview + 自控 HTML（`playsinline`） |
| 全平台无声（GUIVideo 模板写死 muted） | 双轨音频：ffmpeg 提 ogg 走 `ui_sound.play_sound`，触摸视频后 JS 取消静音 + 通知 lua 停独立音轨 |
| 双声音/回声 | 触摸切换时 lua 必须停独立音轨（依赖双向桥 video_touch 回报） |
| **安卓首帧前闪「灰底+大播放三角」占位图 ~0.5s** | 安卓 WebView 对无 `poster` 的 `<video>` 在首帧解码前画内置占位图（PC/iOS 画空白故无感）。解法：CSS 起手 `opacity:0`，JS 监听 `playing` 事件（首帧真正上屏，比 canplay 准）再 `opacity=1`；或给 video 塞 1×1 黑图 data-URI poster |

## 7. 调试方法论（iOS 无日志环境的组合拳）

1. **DevTools**：`web_dev_tools=true` 弹 F12（PC），console/语法错误直接看
2. **console 转发**：`web_console_log=true`，JS 侧 log/报错全进 lua log（跨端）
3. **上屏诊断**：cgui 面板渲染诊断文本 + `common.copy_to_clipboard` 导出（iOS 拉不到日志文件的标准解法）
4. **scelua 自检**：JS 首帧 `console.log('scelua=' + typeof scelua)` 确认桥注入（undefined = 引擎没注入，别往下查了）
