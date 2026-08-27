# 线上 pak 内容提取：原生 io 双名通道（PascalCase 漏网）与视频绝对路径方案

> 最后验证：2026-08-26（编辑器 PIE + 星火对战平台 tester PC 线上 E2E + Android 实测通过；iOS 白屏待排查）
> 需求起源：游戏内视频控件需要**真实磁盘绝对路径**，而线上地图内容在单文件 pak 里，磁盘无散文件。
> 结论一句话：**引擎对每个 io 函数同时注册小写与 PascalCase 两个名字，isolation 只阉割小写**——`io.ExtractPakFile` 等原生版线上直接可用，可把 pak 条目解到磁盘拿绝对路径。

## 1. 机制背景（为什么需要这套东西）

- 游戏发布后 = 单文件 TNND-UPAK 地图 pak（如 `p_55a3.pak`），**下载后不解散**，内容只以 pak 条目存在（机制详见 payload-packages.md）。
- `game.GetMapPath()` 在线上返回**资源路径**（`maps/p_55a3`），不是磁盘路径——PIE 下才返回项目目录（实测日志对比：`maps/p_55a3` vs `C:/Users/.../test_res002`）。
- StateGame 的 `io.read`/`io.copy` 被 isolation 包装重定向到 `<root>/User/maps/<地图>/` 裸文件系统，**读不到 pak 内容**（tester 实测 `io.read failed, error_code[1]`，见 editor-patch 仓 pc-tester-runtime-reverse.md §6）。
- pak 读取 API（read_pak_entries/extract_pak/extract_pak_file/read_cache）被 isolation 置 nil——**但只置了小写名**。

## 2. DLL 逆向证据（sceengine.dll，编辑器引擎 BuildPC）

字符串区存在完整 LuaIO 注册块（源文件标记 `D:\BuildPC\NE_pd\Client\src\Game\Lua\SubModule\LuaIO.cpp`）。注册表是 luaL_Reg 风格数组（文件偏移 `0x26bf060` 起，每条目 = 8 字节名字指针 + 8 字节函数指针），**每个函数注册两种拼写**：

| 条目偏移 | lua 名 / C++ 名 | 函数 VA |
| --- | --- | --- |
| 0x26bf0c0 / 0x26bf0d0 | `read_cache` / `ReadCache` | 0x1812c5730 |
| 0x26bf0e0 / 0x26bf0f0 | `read_pak_entries` / `ReadPakEntries` | 0x1812c5760 |
| 0x26bf100 / 0x26bf110 | `extract_pak` / `ExtractPak` | 0x1812c5450 |
| 0x26bf120 / 0x26bf130 | `extract_pak_file` / `ExtractPakFile` | 0x1812c5480 |

- 每个名字串全 dll **仅被这一张表引用一次**（无第二张注册表），即两种拼写都会进入运行时 `io` 表。
- 同表还有 `List/ExistsFile/ExistDir/Write/Read/CreateDir...` 等全套 PascalCase 变体。
- tester 引擎 scegame（BuildPCBox）同样存在（字符串 535551-535557 + RTTI `FactoryImpl<LuaIO>` 实证）。
- 工具链：`find_xref` 找不到 lea 引用是正常的——名字走 .rdata 数据表指针；用「字符串 VA 的 8 字节小端值全文件扫描」定位注册表（本法首次使用，已验证有效）。

## 3. isolation 阉割机制与漏网（script-199 common/isolation.lua）

StateGame 初始化时执行，**纯 Lua 层**操作，native 实现不动：

1. **包装类**（保留功能但加路径沙箱）：`io.read/write/copy/rename/remove/copy_to_folder/create_dir/exist_* /walk_dir/list/attribute_type/file_time` + `dofile/loadfile/load`——先 `local 保存原生`，再替换为「相对路径 → `<root>/User/maps/<地图>/` 拼接；绝对路径在非 editor debug 下直接 error」的包装版。
2. **置 nil 类**（完全禁用）：`io.read_cache/read_pak_entries/extract_pak/extract_pak_file/copy_cache_file/download_file/serialize 系/walk_resource_dir/add_resource_path/popen/select_*/watch 系/get_package_path` + `os.execute/exit/remove/...` + `debug.getregistry/getupvalue/...` + `package.loadlib`。
3. **★ 漏网**：两类操作都只写小写名，`io.ExtractPakFile` 等 PascalCase 变体未被触碰，且不过 `full()` 路径检查——**原生版直接可调，绝对路径随便传**。
4. 恢复小写名的不可能性（线上玩家端）：置 nil 后无 Lua 可达引用，debug/package 后门同步被阉，游戏代码又在 isolation 之后才加载。编辑器/自托管可改 isolation.lua 本身，线上只能靠 PascalCase 通道。
5. 用户提出的备用思路（备忘）：hook `base.path` 元表 `is_absolute` 恒返回 false，可骗过包装版 `full()` 让绝对路径走「root / pp」拼出绝对路径本身。原生 PascalCase 可用时用不到，留作兜底。

## 4. 接口签名与实测行为（全部 StateGame 线上实测）

### 4.0 签名反汇编实证（2026-08-26，lua_api_dump 工具，双引擎一致）

注册函数是 thunk（头部 `jmp` 真实实现），实现体内经跳板 stub 调 lua54 取参。推断结果：

| 函数 | 编辑器 RVA / tester RVA | 推断签名 | 官方调用点复核 |
| --- | --- | --- | --- |
| `extract_pak_file` / `ExtractPakFile` | 0x12c5480 / 0x19a3950 | **(string pak路径, string 条目名, string 目标路径) -> 1**（0=成功） | extractor.lua:92 ✓ 完全一致 |
| `extract_pak` / `ExtractPak` | 0x12c5450 / 0x19a3920 | (string pak路径, string 目标目录) -> 1 | 未实测 |
| `read_pak_entries` / `ReadPakEntries` | 0x12c5760 / 0x19a3c60 | (string pak路径) -> 1（条目名数组） | extractor.lua:119 ✓ |
| `read_cache` / `ReadCache` | 0x12c5730 / 0x19a3c30 | (?) 实现未走标准取参（路径来源待考），实测不可靠勿用 | — |
| `read` / `Read` | 0x12c5700 / 0x19a3c00 | (string path, boolean?, ...) -> (err, content)；isolation 包装版 ret==0 成功 | — |
| `write` / `Write` | — / 0x19a4140 | (string path, ..., string content) -> 1（0=成功） | — |
| `copy` / `Copy` | — / 0x19a3710 | (string src, string dst) -> 1 | — |
| `list` / `List` | 0x12c5640 / 0x19a3b40 | (string path, integer mode, ...) -> (err, array)；mode 1=文件 2=目录 | local_version.lua ✓ |
| `get_package_path` | — / 0x19a3a70 | (string, string) -> 1 | — |

> 推断噪音说明：无参函数偶发 `(userdata)` 误报（实现内 lua_touserdata 另作他用）；可选参数不标注（实现按 lua_gettop 分支）。完整 io 表 dump：`target/io-editor.tsv` / `target/io-tester.tsv`（重新生成：`lua_api_dump <PE> read_pak_entries`）。


| 接口（PascalCase 原生版） | 签名 | 实测 |
| --- | --- | --- |
| `io.ReadPakEntries(pak路径)` | → 条目名数组 | ✅ TNND 加密地图 pak 直接读出 1301 条目（**透明解密**） |
| `io.ExtractPakFile(pak路径, 条目名, 目标绝对路径)` | → 0=成功 | ✅ 35MB mp4 解出，`head_check magic=ftyp` 合法明文 |
| `io.List(目录, 模式)` | 1=文件 2=目录 → (err, 数组) | ✅ 枚举出 `Update/e.production.spark.xd.com` 与 `..._test` |
| `io.ExistsFile(路径)` / `io.ExistDir` | → boolean | ✅ 绝对路径可用 |
| `io.Read(路径)` | → (err, 内容) | ✅ 读解出的 mp4 正常（36,945,699 字节） |
| `io.Write(路径, 内容)` | → 0=成功 | 原生版无沙箱限制（对比：小写 `io.write` 被包装，只能写 User/maps/<map>/ 下） |
| `io.ReadCache(路径)` | → (err, 内容) | ⚠️ **不可靠，勿用**：`maps/p_55a3/res/shenyi.mp4` 返回 err=0 但 len=0（假命中）；`maps/p_55a3/script/main.lua` 反而 err=1。语义疑似「下载缓存」而非 ResourceCache，未深挖 |
| `io.ExtractPak(pak, 目标目录)` | 整包解（推断） | 未实测 |

**extractor.lua（startup-364）官方用法样本**（手机端启动释放 Res.pak 流程）：
`self.pak = <app_dir>/Res.pak` → `io.extract_pak_file(self.pak, 条目, 'Update/.../目标')` → ret 0 成功；`io.read_pak_entries(self.pak)` 数组判 `#entries == 0`。注意它带 `if io.extract_pak_file == nil then '旧版本不支持'` 守卫——**部分构建可能没注册该函数，调用前必须判 nil**。

## 5. 线上路径实况（tester PC 实测）

- `common.get_app_dir()` = `D:/sce_pc_tester/tester_1089/Win/`（tester）/ 编辑器安装根（PIE）
- 地图 pak 落点：`<app_dir>/Update/<env>/Res/maps/<map>/<map>.pak`
  - env 目录实测两棵：`e.production.spark.xd.com`（正式）/ `e.production.spark.xd.com_test`（测试）——**测试环境 pak 在 `_test` 后缀树**
  - 目录名大小写：磁盘上是 `update` 小写，Windows 不敏感；**iOS/Android 文件系统敏感，枚举 `Update` 失败时试小写**
- 正确的 pak 定位法 = `io.List(app_dir .. 'Update', 2)` 动态枚举 env 目录再拼 `Res/maps/<map>/<map>.pak`，不要猜死路径
- 地图 pak 磁盘上是 **TNND 加密**（magic `TNND` 实证），ReadPakEntries/ExtractPakFile 走引擎 PackageFile **透明解密**——这是相对自己解密的决定性优势

## 6. 三环境路径策略（视频绝对路径实战）

```lua
local platform = require '@base.base.platform'  -- client_base/base/platform.lua（script 库也有同名 base.platform）

-- 环境判断 API（全部 cache 过的纯函数）：
--   is_win()      = get_platform()=='Windows' 且无 android/ios argv
--   is_android()  = has_arg('android') or get_platform()=='Android'
--   is_ios()      = has_arg('ios')     or get_platform()=='iOS'
--   is_mobile()   = is_ios() or is_android()
--   is_formal()   = 正式服（win 看 entrance argv；android/ios 恒 true）
--   common.is_game_play_in_editor() = PIE 编辑器调试

-- 策略分流：
-- 1) 编辑器 PIE：地图是项目散文件，直接
--      game.GetMapPath() .. '/res/leishen.mp4'        -- 真实磁盘路径
-- 2) 对战平台 PC / Android：pak 提取法（本文 §4 链路）
--      io.List 枚举 Update/<env> → io.ExtractPakFile(pak, 'res/leishen.mp4', dest)
--      dest = app_dir .. 'User/maps/<map>/leishen.mp4'  → 真实绝对路径
--      （PC 与 Android 均已实测通过）
-- 3) iOS：白屏待排查，见 §7
```

生产化注意：

- **判 nil**：`if io.ExtractPakFile == nil then 回退 http URL`（旧构建可能没有）
- **缓存判重**：每次进游戏解 35MB 浪费——用 `io.ExistsFile(dest)` + 尺寸或 pak 版本号（`common.get_map_pak_version`）做跳过
- **官方堵洞风险**：这是 isolation 漏网通道，官方任何版本可能补上 PascalCase 阉割——必须有 http 兜底
- iOS 沙盒可写目录假设待验证（§7）

## 7. iOS 终版结论（2026-08-26 全链实测收敛）

白屏根因与解法全部实证：

1. **提取链路 iOS 正常**：`get_app_dir()` = `Documents/`（可写），`Update/`（大写）枚举正常，pak 在 `Update/e.production.spark.xd.com_test/Res/maps/...`，ExtractPakFile 正常解出合法 mp4。
2. **白屏根因 = 播放器要 file:// scheme**：裸绝对路径白屏，加 `file://` 前缀即播（iOS webview 语义，PC 宽容、iOS 严格）。
3. **原生 video 控件（GUIVideo）iOS 强制全屏**：引擎内嵌模板其实已带 `webkit-playsinline playsinline`（sceengine 字符串 452635-452640），但 iOS 仍调起系统全屏播放器 → WKWebView 原生配置未开 `allowsInlineMediaPlayback`，Lua 层改不了。**结论：iOS 内联播放不能走原生 video 控件**。
4. **★ iOS 内联正解 = webview 控件 + 自控 HTML**。模板要点：
   - `<meta name="viewport" content="...user-scalable=no">` + `overflow:hidden` + `touch-action:none` —— 禁滚动条/捏合缩放
   - `object-fit:contain` —— 防右下裁剪
   - `muted autoplay` 起手（iOS 自动播放强制静音），页面内首次 touch 手势取消静音
5. **★ 本地文件的加载姿势（关键坑）**：webview `html` 文本加载（loadHTMLString）的页面源不是 file，**其中 file:// 视频会被 WKWebView 拦**——解法：把播放器 HTML **写成文件放在 mp4 旁边**（沙箱版 `io.write('player.html', html)` 恰好落在 User/maps/<map>/），webview 用 `url = 'file://' .. html路径` 加载，视频 src 写纯文件名（同目录相对，同为 file 源即可读）。
6. **声音双轨方案（全平台）**：引擎 GUIVideo 模板写死 `muted autoplay` → 原生 video 控件**所有平台都无声**；统一由引擎音效系统 `ui_sound.play_sound` 播独立音轨（ffmpeg 从视频离线提取 ogg：`-vn -c:a libvorbis -b:a 128k`，秒级直通；**音效走资源路径 pak 感知，不用解 pak**）；iOS 上触摸视频 → JS 取消视频静音 + `scelua.send_string` 通知 lua 停独立音轨。
7. **★ imgui/cgui 通道 JS→lua 桥打通（2026-08-26 探针实证）**：`on_web_message` 在 imgui state 里不存在（`imgui_state()` 只回 `id,name`），一度误判"imgui 通道不支持"。真实机制链：`scelua.send_string` → 引擎调 lua 全局 `ui_events.on_web_message(id, msg)` → `base/ui/event.lua` 查 **`base.ui.map[id].event.on_web_message`** 回调。imgui 建的控件不在 `base.ui.map`（那是 base.ui 声明式才登记的），所以静默丢弃。**解法：把 imgui 控件 id 塞进 `base.ui.map[id] = { event = { on_web_message = fn } }` + `base.ui.gui.register_event(id, 'on_web_message')`**，消息即通（BRIDGE_MSG 实证）。cgui `M.webview` 的 `opts.on_web_message` 已内置此登记（leaf.lua）。
8. **lua→JS**：控件属性 `run_js` 下发 JS 串执行（实证可用）；派发自定义事件用 `window.dispatchEvent(new CustomEvent('GlobalEvent', {detail:{message=...}}))`，JS 侧 `window.addEventListener('GlobalEvent', ...)` 接收（范本 examples/apidemo/webview_ctrl.lua）。
9. 参考实现：test_res002 `.bgd/src/client/test/aye.lua`——**完整全屏过场动画组件**：pak 提取 + 播放器文件 + 双轨音频 + 双向桥（触摸切原声/播完跳过回报驱动 lua 按钮态/run_js 下发重播跳过指令）+ 多通道对照（原生 video/https/桥探针/诊断面板）。

### 7.1 上线实测收尾细节（2026-08-26 三端实测）

- **播放器 HTML 必带 `<meta charset="UTF-8">`**：文件经 `io.write` 写盘无 BOM，缺 charset 时浏览器按 latin-1 解码 → 中文按钮乱码（iOS 实测）。
- **画面留黑**：`object-fit:contain` 等比缩放，移动端屏幕比例与 16:9 视频不符时留黑明显；移动端用 `cover` 填满裁上下视觉更好（demo 用 `VIDEO_FIT` 全局参数控制，移动端 cover / PC contain）。
- **iOS 右缘 1-2px 透出**：contain 黑边 + webview 物理像素与 CSS 逻辑像素非整数缩放的舍入缝 → `cover` 溢出裁切 + `video{transform:scale(1.01)}` 放大 1% 消除。
- **触摸/点击事件在 webview 内容区正常到达 JS**（mousedown/touchstart 都收得到），之前"点了没反应"是旧探针 HTML 结构问题，非引擎限制。
- **iOS 长按放大镜**：`-webkit-touch-callout:none` 全元素 + user-select 禁选择可消除。
- **iOS 右缘漏光竖条**：webview 物理像素对齐舍入缝，黑底垫层（fullscreen 面板 `color=rgba(0,0,0,1)`）挡住透出即可。
- **安卓首帧前闪占位图（灰底+大播放三角，~0.5s）**：安卓 WebView 对无 `poster` 的 `<video>` 在首帧解码前画内置占位图（PC/iOS 画空白故无感）。解法：CSS 起手 `opacity:0` + JS 监听 `playing` 事件再 `opacity=1`（比 canplay 准；替代方案 = 1×1 黑图 data-URI poster）。
- **双向桥派发生态**：详见 [webview-bridge.md](webview-bridge.md)（on_web_event 手势全集/console 转发/三通道接法/坑清单）。

## 8. 验证记录

| 日期 | 环境 | 结果 |
| --- | --- | --- |
| 2026-08-26 | 编辑器 PIE（StateGame） | 全链路通：ListUpdate→pak_found(tester pak)→entries=1301→Extract ret=0→magic=ftyp |
| 2026-08-26 | tester PC 线上（发布后真包） | 全链路通，解出 `Win/User/maps/p_55a3/shenyi.mp4` 36,945,699 字节，视频播放正常 |
| 2026-08-26 | 对战平台 Android | 正常 |
| 2026-08-26 | 对战平台 iOS | 提取链路全通；白屏根因=file:// scheme 缺失；原生 video 控件强制全屏（判死）；webview 自控 HTML 内联播放通过（§7 终版） |

## 9. 关联文档

- pak 格式/落位/版本注册表：payload-packages.md
- 包挂载机制/启动序列：scegame-reverse.md §1/§3
- StateGame io 边界与 tester 差异矩阵：editor-patch 仓 doc/research/pc-tester-runtime-reverse.md §6/§11
- isolation 阉割全表：editor-patch 仓 .trae/skills/sce-lib-script-199/api.md
