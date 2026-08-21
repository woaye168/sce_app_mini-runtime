# 两套官方运行时：结构与引擎归属

> 最后验证：2026-08-21（MD5 比对 + 导入/导出表实证）
> **本仓历史教训**：早期用 `update-info` 的 `win` 包当引擎，实测其 scegame 与对战平台 scegame MD5 完全一致——即我们一度用的是「对战平台引擎」而非「编辑器引擎」。这导致 entrance 登录的 TLS 栈判断错误（hook 错位置，WSS 抓不到明文的假阴性）。

## 1. 定位对比

| | 星火编辑器 | 星火对战平台（tester） |
| --- | --- | --- |
| 用途 | 开发/调试/发布游戏（开发者） | 玩家玩已发布的游戏 |
| 根目录 | `D:\sce_online` | `D:\sce_pc_tester\<实例>\Win`（如 tester_1089） |
| 宿主入口 | `星火编辑器.exe`（.NET/WinUI 宿主） | `scegame`（直接游戏引擎 exe，无独立宿主） |
| API 版本目录 | `version-<api>\`（如 version-13），**dll 在该目录** | dll **直接在 Win 根目录**（无 version 子目录） |
| 引擎核心 | `version-<api>\sceengine.dll`（**50MB**，游戏实际跑在它里面） | `scegame` 自身（48MB，引擎内嵌） |
| 壳 | `version-<api>\SCE`（739KB，游戏客户端壳） | —— |

**关键**：结构相近（都含 client_base/startup/字体/资源包），但**引擎二进制不同**。游戏逻辑（lua + 网络 + 渲染）跑在各自的引擎核心里：编辑器在 `sceengine.dll`，对战平台在 `scegame.exe` 内嵌。

## 2. 引擎二进制实证

```
editor:  D:\sce_online\version-13\SCE            739KB  壳（ProductVersion 1.0.0+44d0ebab...）
editor:  D:\sce_online\version-13\sceengine.dll  50MB   引擎（3772 个导出符号，无 SSL_* 导出）
tester:  D:\sce_pc_tester\tester_1089\Win\scegame  48MB  引擎+壳一体（ProductVersion 172）
我们 update-info 下载的 win 包 scegame = 与 tester scegame MD5 完全一致（5A8F5FDD...）
```

## 3. 各 dll/exe 作用（两运行时共用一套 gme sdk，但挂接位置不同）

| 文件 | 作用 | 备注 |
| --- | --- | --- |
| `sceengine.dll` | **编辑器引擎**（游戏逻辑/lua/网络/渲染） | 3772 导出符号（CppInterface 全家 C 入口）；TLS **静态链接**（含 `SSL_CTX_new`/`wss://` 字符串但无导出） |
| `SCE`（739KB） | 编辑器游戏客户端壳 | 拉起 sceengine |
| `sce.dll` / `sce.deps.json` | .NET 互操作（编辑器宿主与引擎的桥，C# P/Invoke） | `sce.deps.json` 登记 dll 部署（editor-patch 桥 dll 也登记在这） |
| `scegame` / `scegame.exe` | **对战平台引擎+壳一体** | 我们的 win 载荷引擎就是它 |
| `gmesdk.dll` | GME 语音/网络 SDK | **tester 侧它导入 libgmessl 的 SSL_read/write**（TLS 在这）；编辑器侧也存在但 WSS 不走它 |
| `libgmessl-1_1-x64.dll` | OpenSSL 变体（GMSSL），导出 SSL_read/write/SSL_CTX_new 全套 | 供 gmesdk 调用；编辑器引擎 sceengine.dll **不依赖它**（静态链接） |
| `libgmecrypto-1_1-x64.dll` | GMSSL crypto 部分 | 同系列 |
| `libgme*.dll`（faad2/fdkaac/lamemp3/ogg/soundtouch） | 音频编解码 | —— |
| `sdk.dll` | 登录 SDK 桥 | 导入 gmesdk.dll；含 SSL_CTX_new 字符串但无 SSL_read/write 导入 |
| `lite.dll` / `lua54.dll` | 轻量模块 / Lua 5.4 运行时 | —— |
| `themis_x64.dll` | 反作弊/环境检测（腾讯 Themida 系） | 编辑器 14MB / tester 18MB |
| `shaderc.dll` | shader 编译 | —— |
| `webview2loader.dll` | WebView2 加载器（编辑器 UI/登录扫码页用） | —— |
| `commandtool.exe` | 官方无 GUI 命令行工具（Pack/MapRef/纹理压缩等） | 编辑器 version-\<api\> 下；脱机发布可用它打图集 |

## 4. 编辑器目录结构（D:\sce_online）

```
D:\sce_online\
  星火编辑器.exe           # .NET/WinUI 宿主（GUI 外壳）
  version-<api>\           # 引擎+sceengine.dll+SCE+sdk/gmesdk/themis 等 dll + commandtool.exe
  version-2000\            # 新版客户端（.NET，wwwroot/plugins，与游戏运行链不同）
  update\<env>\            # 在线更新缓存（editor-pd.spark.xd.com）
    api_pak_version.json   # ★ 版本注册表（包名→版本）
    res\                   # 散包（client_base/startup/fonts/engineres/uistyle/refconfig/_m/...）
  Res\                     # 基座资产（characters/effect/maps——update-info 不分发）
  ResCache\  User\         # 资源缓存 / 用户数据（user_info-<env>.json 凭证在这）
```

## 5. 易踩的坑（实证）

1. **「win 包 = 编辑器引擎」是错的**——win 是对战平台引擎。编辑器引擎在 version-\<api\>/sceengine.dll，**update-info 不直接给编辑器引擎**（编辑器走安装包/launcher_update）。脱机 B 模式的客户端我们用 win 包 scegame 是可行的（登录/跑游戏能力一样），但**研究编辑器行为时要分清**。
2. **TLS 栈两运行时不同**：tester 的 TLS 在 gmesdk→libgmessl（hook 导出可抓明文）；编辑器的 TLS 在 sceengine.dll **静态链接**（无导出可 hook）。混淆会得出「hook 装上但抓不到」的假阴性。
3. **version 目录名 = api_version**（version-13 对应 api_version=13），与 map_settings.json 的 api_version 对应。
4. **凭证文件位置**：`<运行根>/User/user_info-<env_domain>.json`（如 `D:\sce_online\User\user_info-editor-pd.spark.xd.com.json`）。
5. **B 模式永远带 `-no_update`**：否则 scegame 自更新会清掉手工组装的载荷。

## 6. ★ 引擎的在线分发机制（2026-08-21 逆向 client_base update 链 + api_pak_version.json 实证）

这是「运行时按 api_version 组装 + 编辑器同款下载机制」的核心，mini-runtime 的 runtimes 架子以此为准。

### 6.1 api_version 是组装的总钥匙

`api_pak_version.json`（`update/<env>/` 下）顶层键 = `#package_path` + 各 api 版本表（`-1`/`12`/`13`/`2000`）。
项目 `map_settings.json` 的 `api_version`（如 13）→ 决定两套东西：
- **引擎**：`<运行根>/version-<api_version>/`（version-13 对应 api13；version-2000 对应新客户端）
- **lua 运行时包**：`<api_version>` 表里的 `{包名: 版本}` → 经 `#package_path` 映射到 `Res/_m/...` 真实路径

### 6.2 引擎二进制 = update-info 的一个普通包（按 api 选版本）

编辑器引擎（SCE 壳 + sceengine.dll + 依赖 dll）**不是安装包独占**，而是经 update-info 以**二进制包**形式分发，版本号按 api_version 选：

| 二进制包名 | 含义 | 实证 |
| --- | --- | --- |
| `win` | **对战平台引擎**（tester 的 scegame 一体） | api13 表无此项（tester 用） |
| `wineditor` | **编辑器引擎**（含 version-\<api\> 的 sceengine.dll + SCE 壳 + 全套依赖 + 基座 res） | `api_pak_version.json` 的 `13.wineditor = 147` ←→ version-13\sceengine.dll PV=147；`2000.wineditor = 148` ←→ version-2000\sceengine.dll PV=148 |

**下载机制（编辑器同款，client_base-78 `update/download_manager.lua` + `update/init.lua` 实证）：**
1. `update-info?list=wineditor;...&api_version=<api>&variation=windows_editor` → 得 wineditor 的 `{version, url, md5, size, original_size, path, ...}`。
2. **版本选哪个由 api_pak_version.json[\<api\>].wineditor 定**（147），不是盲目下最新——这就是「按 api 组装」。
3. 下载 url（OSS）→ 7z 解出 → **整棵 version-\<api\>/ 目录 + launcher_update/ + update/\<env\>/res 基座包**。wineditor 是**自包含的引擎运行时**（含 version-\<api\> 全部 dll + res 散包 + embedded_packages）。
4. 安装到 `<运行根>/version-<api>`；`platform.binary()` 返回当前二进制名（编辑器=`wineditor`），`download_manager` 据此判断要不要更新（编辑器允许更新成低版本）。

### 6.3 version-\<api\> 目录内容（version-13 实测）

```
version-13\
  SCE                  # 739KB 游戏客户端壳（-env=game ... 跑游戏）
  sceengine.dll        # 50MB 引擎（PV=147 = api13.wineditor）
  sce.dll / sce.deps.json / scemodule.dll / scecustomcontrol.dll   # .NET 互操作桥
  gmesdk.dll / sdk.dll / themis_x64.dll / lua54.dll / lite.dll / shaderc.dll / embree4.dll / ...
  commandtool.exe      # 无 GUI 命令行工具（Pack/MapRef/纹理压缩——脱机发布打图集用）
  embedded_packages\   # 内嵌包（client_base-78.7z / script-199.7z / startup-364.7z / appui / xdeditor_startup）
  SCE.WebView2/  assets/  microsoft.ui.xaml/  ...   # 编辑器 UI 壳资源（跑游戏不需要）
```

- 跑游戏只用 **SCE + sceengine.dll + 游戏依赖 dll**（gmesdk/sdk/themis/lua54/lite/shaderc/embree4/tbb12 等）+ **lua 运行时包**（来自 update/\<env\>/res）。
- `星火编辑器.exe`（宿主）只是 GUI 外壳，**跑游戏不需要它**——它按 api_version spawn `version-<api>/SCE`。
- version-13 无 `res/` 子目录——lua 包在 `update/<env>/res/`（全局共享，不按 version 隔离）。

### 6.4 对 mini-runtime 的直接推论

- **「星火编辑器-13 运行时」自举 = 下载 wineditor@147 + 取其中 version-13 的游戏必需子集**（SCE+sceengine.dll+游戏dll+embedded_packages），lua 包走现有 update-info 通道。
- 现有 B 模式的「控制协议上传/起局」与引擎解耦（spawn 的是客户端 exe）；换引擎 = 只换 spawn 目标从「tester scegame.exe」→「编辑器 version-13\SCE」。
- **基座资产其实随 wineditor 自带**（wineditor 含 update/\<env\>/res 基座包）——之前的 base_assets.7z 通道可降级为兜底。

### 6.5 wineditor@147 实测（2026-08-21，地基验证）

```
update-info: list=wineditor & api_version=13 & variation=windows_editor
→ name=wineditor version=147 size=144MB
  url=sce-maps-pd.oss-cn-shanghai.aliyuncs.com/wineditor/master/Version/147/windows_game.7z
下载解出顶层 = launcher_update/ + update/<env>/res/ + version-13/ + variation.json
version-13/ 实测：SCE(739KB) + sceengine.dll(50MB, PV=147) + gmesdk/sdk/themis/lua54/lite/shaderc/embree4/commandtool.exe + embedded_packages/(client_base-78/script-199/startup-364/appui/xdeditor_startup)
update/<env>/res/ = client_base/engineres/fonts/lite/refconfig/shadercache_windows_ui/startup/uistyle/xdeditor/xdeditor_startup
```

**结论：编辑器-13 运行时可完全脱机自举**（wineditor@147 一个包给齐引擎+基座 res），无需本机装编辑器。
