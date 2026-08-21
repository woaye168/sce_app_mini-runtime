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
