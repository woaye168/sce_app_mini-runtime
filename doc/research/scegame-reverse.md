# scegame 逆向：自托管运行时构建（0.1.0 调试脱机）

> 研究日期：2026-08-21（进行中）
> 目标：自托管「最小官方运行时」——官方 scegame/dll + 官方 lua 包 + 官方 pak，不依赖编辑器/tester 壳。
> 研究对象：`D:\sce_pc_tester\tester_1089\Win\scegame`（BuildPCBox，47.9MB）+ tester 运行时日志
> 方法：strings_dump（examples/）+ PE 导出（pe_exports.rs）+ 日志反推 + client_base/startup 源码

## 1. 运行时启动序列（tester game log 实证，逐行）

```
0. 日志打开 + Git Info（Client/U/Common 构建哈希）
1. Set UpdateIp to e.production.spark.xd.com          ← 环境变量（UpdateIp）
2. Engine initialize begin.
3. enable MSAA
4. Created 15 worker threads
5. resourcePrefixPath: D:/sce_pc_tester/tester_1089/Win/, resource: ResCache
   Added resource path <根>/ResCache/
6. resourcePrefixPath: <根>/, resource: Res
   Added resource path <根>/Res/
7. PackageFile::Open <根>/Update/<env>/Res/script/script.pak         ← script 包
8. 同  client_base/client_base.pak
9. 同  startup/startup.pak
10. 同 appui/appui.pak
11. 同 fonts/fonts.pak
12. (refconfig.pak not found 警告，可缺)
13. Found shadercache pak: shadercache_windows_game_dxbc(.pak)      ← UI shader
14. Found shadercache overlay: shadercache_windows_game_dxbc_extra
15. ========> 初始化完毕 <==========
16. （进游戏时）added package _m/script/<ver>/script with prefix[_m/script/190/script]
    added package _m/appui/<ver>/appui with prefix[_m/appui/28/appui]
17. ==> StateGame Open Lua
18. Host Network: Create KCP Network（useStreamMode: 1）
19. Create game instance. uuid(1) mapType(1) mapName(app_box)
20. StartGame - 1 → Connect to game host server ip:port
21. NetManager::OnConnected → GamePlayOnline connect success
22. GamePlayOnline request login, userid/username: 38672742（SetGameUserId）
23. send login request to server → Game host login result[0]
24. gamePath_: maps/app_box → SetGameInfo → Not a wasm project
25. （进具体地图）added package maps/<map>/<map>.pak + _m/maps/<lib>/<ver>/<lib>.pak
```

**关键事实：**
- 运行时包挂载点 = `<根>/Update/<env>/Res/`（测试/正式两环境各一棵）+ `<根>/ResCache/` + `<根>/Res/`。**自托管 = 把 tester 的 Win/Update 目录结构复制一份即得全部包**。
- 游戏局 login 用的是 **userid/username 数字**（38672742），不是 token——大厅（StateApplication）先登录拿到 userid，再 StartGame 传给游戏局。
- 地图按 `maps/<map>/<map>.pak` 挂载；依赖库按 `_m/maps/<lib>/<ver>/<lib>.pak` 带 `_m/...` 前缀挂载。

## 2. 引擎子系统（scegame 字符串实证）

Urho3D Engine.cpp 注册序（:513255-513278）：Console/Engine/Settings/Resource/Image/FWindow/XMLFile/Network/BaseResourceCache → "a couple of Subsystems registered finish." → Engine construct finish → initialize begin → MainThread/EngineWorker/Created N worker threads/VideoWorker → 渲染后端（gles/dx11/dx12/metal/vulkan/shadercache）→ "Initialized graphics & audio output." → "Initialized engine"。

渲染路径 argv（Engine.cpp 字符串 :513295-513318）：headless / nolimit / flushgpu / landscape / nosound / noip / mono / prepass / deferred / renderpath / noshadows / lqshadows / nothreads / borderless / lowdpi / renderertype / renderdoc.dll。
**有 `headless` argv**——自托管可无窗口跑。

wasmtime 子系统在场（wasm_*_Import / Localization_GetSubsystem_Import / Audio_IsInitialized_Import——wasm 模块调 native 的桥）；PaymentBridge 依赖 Lobby/Lua 子系统。

## 3. 包挂载机制

- 引擎的库注册表：`script;client_base;startup;xdeditor;xdeditor_startup;refconfig;appui;engineres;ui;fonts`（sceengine/scegame 字符串共有）。
- 挂载方式：`AddPackageFile '<pak>' to cache` + `added package[<路径>] with prefix[_m/<包>/<ver>/<包>]`——`@base` 跨库解析走「前缀 + 注册表」。
- 入口：`xdeditor_startup.main`（编辑器）/ startup 的 entrance/main.lua（tester 大厅）。
- 资源目录：URHO3D_PREFIX_PATH 环境变量 + resourcePrefixPath argv。

## 4. 壳↔引擎接口层（SCE_*_wasm_export，字符串实证）

scegame 里有完整一组 `SCE_*_wasm_export` 符号（:524910-524961+）——这是 **native 引擎暴露给 wasm/壳的回调注册面**：

- 帧循环：`SCE_game_tick` / `SCE_on_post_update` / `SCE_set_post_update_enabled` / `SCE_on_prerender_update` / `SCE_set_prerender_update_enabled`
- 场景/单位：`SCE_on_load_scene(_over)` / `SCE_on_unit_appear/disappear` / `SCE_on_unit_attributes_changed` / `SCE_on_player_attributes_changed` / `SCE_on_game_attribute_changed`
- 大厅/登录：`SCE_on_entrance_connected/disconnected/connect_error_catched` / `SCE_on_login_response` / `SCE_on_sdk_login_result` / `SCE_on_login_at_other_place_notify` / `SCE_on_notify_service_stopped`
- 对局：`SCE_on_start_game_notify` / `SCE_on_team_start_game_response` / `SCE_on_start_loading/finish_loading` / `SCE_on_host_disconnected`
- 队伍：`SCE_on_create_team_response` / `SCE_on_join_team_response/notify` / `SCE_on_leave_team_response/notify` / `SCE_on_start_match_response` / `SCE_on_cancel_match_response`

**推论：tester 的「游戏逻辑宿主」其实是 wasm**（AppBundle/dotnet.wasm + node.mjs）——游戏局由 wasm 层的 TypeScript（触编 v2 产物）驱动，native 引擎通过这些 export 回调 wasm。自托管宿主 = **native 引擎（scegame）+ wasm 运行时（wasmtime）+ 游戏 wasm 包**。

游戏局启动：lobby 的 `lobby_to_start_game_name` + native `StartGame[mapName, mapKind]`（GamePlayOnline.cpp:539039）+ `start_game`（lua 层 :537436）。

## 5. GameApplication 与 argv 全集（GameApplication.cpp 字符串考古，:512680-512729 + :525152-525244）

### 5.1 子系统/组件注册（:525152-525192）

`GameApplication` 注册的组件清单：GUILogger / GamePlayInfoManager / **Wasmtime** / CEParticleSystem / IGameContextGetter / **Lobby** / GameStatistics / **GameInstance** / FileRemover / GlobalChat / UploadSysInfo / **Updater / Downloader / Uploader / Ziper / Login** / PathSearcherData。

### 5.2 默认环境/资源（:525206-525227）

- 默认环境硬编码：`e.production.spark.xd.com` / `e.intl.spark.xd.com`；资源目录 `ResCache;Res`；库注册表 `script;client_base;startup;refconfig;appui;engineres;ui;fonts`；UI 样式 `UIStyle/DefaultStyle.xml`；state 入口 `StateApplication`；引擎设置 `EngineRes/EngineSettings.xml`。
- 编辑器路径标记：`EDITOR_SERVER_DEBUG SetApiversion %s`。
- 崩溃上传：`User/user_info-` 读凭证取 guest_id 上报（CrashUpload.cpp:525302-525328，logs/dump/*.dmp.pending）。

### 5.3 ★ argv 全集（:512682-512729，调试/运行控制相关）

| argv | 用途 |
| --- | --- |
| `generate_and_debug_map` | 无头调试（同编辑器 argv，引擎层也认识！） |
| `mode_args` | 模式参数 |
| `lobby_to_start_game_name` / `to_download_list` | 大厅起图相关 |
| `project_file` | 项目文件 |
| `editor_server_debug` / `editor_lobby_debug` | 编辑器调试后门（isolation 解锁） |
| `tapcode_server_debug` | tapcode 环境 |
| `debug_via_remote` / `debug_local` / `no_debug_game_in_editor` | 调试模式（远端/本地/是否内嵌） |
| `client_only` | 仅客户端（不起 host/服务端？） |
| `runtime_mcp_port` | **运行时 MCP 端口**（RuntimeMcpBridge :525203——运行时自带 MCP 桥！） |
| `host_port` / `host_ip` | 调试 host 地址（不走 assign_host 直连！） |
| `lua_debug` / `lua_debug_application` | lua 调试 |
| `profiler_server` / `tex_size_factor` / `renderdoc_capture` / `renderer_type` / `debug_mobile_renderer_emulation` | 渲染/性能 |
| `sub_process` / `sub_project` | 子进程/子项目 |
| `map_kind` | 地图类型 |
| `close_log_limit` | 日志 |

**重大意义：**
- `host_ip`/`host_port` argv 存在 → **可以跳过 assign_host 直连指定调试 host**（或本地起 host）。
- `client_only` → 只跑客户端局的可能。
- `runtime_mcp_port` + `RuntimeMcpBridge` → **scegame 自带运行时 MCP 桥**（和编辑器 bgd_mcp_bridge 同族），自托管时可以直接复用它来控制游戏局，不用自己造控制通道！
- `generate_and_debug_map` 引擎层就认识 → 它不只是 xdeditor lua 的 argv，引擎 GameApplication 也处理。

### 5.4 自托管宿主设计（基于以上）

```
sce_app_mini-runtime（Rust/egui 宿主）
  ├── 内嵌：官方 scegame 二进制（或直接在子进程拉起 scegame）
  ├── 载荷目录：<app>/runtime/
  │     ├── Update/<env>/Res/            ← 从 tester/编辑器 update 复制（script/client_base/startup/appui/fonts/_m/...）
  │     ├── ResCache/ Res/
  │     └── maps/<项目pak>/
  ├── 启动：scegame -inner -client_only -host_ip=<assign结果> -host_port=<port>
  │         -generate_and_debug_map -project_file=<项目> -editor_api_version=<n>
  ├── 凭证注入：User/user_info-<env>.json（凭证库当前活动凭证）
  └── 控制：runtime_mcp_port 的 RuntimeMcpBridge（若可用）或日志观测
```

下一步：实测 scegame 直接命令行拉起（带 -inner + 载荷目录），观察它要什么、走到哪一步。

## 6. ★ 实测里程碑（2026-08-21，scegame 直接拉起成功）

### 6.1 载荷目录组装（runtime/，从 tester Win + 编辑器 update 复制）

```
runtime/
  scegame.exe（= scegame 改名，PowerShell 需 .exe）+ 全套 dll（lua54/themis/shaderc/gme/sdk/msvcp/vc/ucrt/webview2loader/d3dcompiler/lite）
  Res/、ResCache/、User/user_info-e.production.spark.xd.com.json（凭证注入）
  Update/e.production.spark.xd.com/
    VERSION.JSON（= api_pak_version.json）、map_pak_version.json（同）
    Res/
      script/ client_base/ startup/ gameui/   ← ★ 必须是散文件目录（_m 的 pak 不够，Open Lua 要散文件）
      _m/script/199/script/script.pak、_m/appui/48/appui/appui.pak（引擎优先挂 _m 高版本）
      shadercache_windows_game_dxbc(.pak)/shadercache_windows_game_dxbc_extra/
      maps/p_55a3/p_55a3.pak + libs.json
  embedded_packages/（7 个 .7z + embedded_package_version.json，自举会解）
```

### 6.2 启动命令（实测可用）

```powershell
cd runtime; .\scegame.exe -inner -headless -server=e.production.spark.xd.com
```

- `-server=` 设 _G.IP（否则默认 e.master.sce.xd.com 找不到包）。
- `-headless` 引擎认识（仍会初始化渲染子系统，但不弹主窗口）。
- 工作目录必须是 runtime/（resourcePrefixPath 取 exe 目录）。

### 6.3 实测跑通的链路（lua-application 日志实证）

1. 引擎初始化全过（GPU/渲染/音频/shadercache）。
2. 官方 lua 加载链执行：script/common/init → client_base account/lobby → startup/entrance/main.lua → startup/main.lua。
3. **account 读了我们注入的凭证文件**（token_type=11, login=1，login_token/secret 在）。
4. **真连上大厅 entrance**（lobby.lua:1280 `连接上 entrance` + `已连接！！！` + 全平台维护状态 0）。
5. **官方自更新在线工作**：`do_try_update` → `POST updater-pd.tapsce.cn/api/map/update-info`（200）→ 比对版本 → 列出 gameui/script/appui/lib_lobby/global_default/lib_game_options 等包的 OSS 下载 URL（`sce-maps-pd.oss-cn-shanghai.aliyuncs.com/<包>/master/Version/<ver>/windows_game.7z`）。
6. embedded_packages 7/7 检查通过，VERSION.JSON 加载成功。

### 6.4 剩余卡点（资源缺失，非结构性）

- gameui 的 cursor 图（`_m/gameui/23/gameui/ui/image/scegameboxcursor.png`）——Res 里 gameui 散包版本不对（要 23 不是 48，自更新会拉）。
- 字体 `Fonts/updatFont.otf`、appui 图集（startup/ui/atlas）、toast.png——appui/startup 的 ui 资源。
- 这些都会由官方自更新自动下载补齐（实测它已经在拉 update-info 比对）。

### 6.5 结论

**自托管运行时（scegame + 载荷目录 + 凭证注入）成立**：不碰编辑器、不装 tester 壳，官方 lua 全跑、大厅能连、官方自更新能补齐依赖。下一步：让它完成登录（token_valid → request_token_login）并起游戏局（app_box 或项目 pak），验证 debug 链路。

## 7. ★★ B 模式（编辑器本地调试）完整链路与自托管复现（2026-08-21 跑通）

> 这是 0.1.0 的核心目标链。本节全部来自对编辑器真实调试行为的实捕（Win32_Process 命令行 + 日志），非推测。

### 7.1 编辑器「调试」按钮的完整链路（实捕）

1. **外壳 launcher**：`星火编辑器.exe` 只是启动器。它读 `<项目>/project/map_settings.json` 定 api_version，然后执行真正的二进制：
   `D:\sce_online\version-13\SCE -server=editor-pd.spark.xd.com -use_local_res -launcher="星火编辑器.exe" -editor_api_version=13 -no_ask_editor_api_version -generate_and_debug_map -file_path=<项目>\project.sce`
   - 坑：`-file_path` 必须是**项目里的文件**（project.sce），`MainFrame:GetMapPath()` = dirname(file_path)。传目录会取到上一级。
2. **map_starter**（xdeditor-160/map_starter/init.lua，`-generate_and_debug_map` 分支加载）：
   - 登录（读 `User/user_info-editor-pd.spark.xd.com.json`）→ `query_assign_host`：POST `http://editor-pd.spark.xd.com:9007/api/v1/assign_host`（带 token 签名 + `api_version=13`）→ 返回 `{"host_info":{"ip":"106.14.95.227","port":"14416","token":"<uuid>"}}`。
   - **暂存目录** `D:/sce_online/User/debug/<项目名>`：清空后按 `project_manager.get_project_map_dirs/files` 复制项目（含 scene/script/ui/table/res/project/config.ini/libs.json/project.sce），并 `trigger_manager.generate_lua_only`（tstl 生成 `ui/script/`，触编 TS→lua）。
   - `DebugManager.update_host(ip,port,token)`（native，控制连接 `controlConnToHost_`）。
   - `DebugManager.debug_game{map_path=暂存目录, map_kind=0, game_in_editor=false, ...}`（native）。
3. **native DebugManager**（sceengine.dll，BuildPC Editor 构建）：
   - 控制连接 protobuf 协议（`MessageHeader` 信封）：`EditorLogin`/`EditorLoginResult`、`EditorPing`/`EditorPingRes`、`EditorAssignWorld`、`EditorStartGame`/`EditorDestroyGame`、`NotifyEditorLog`。
   - **UploadDelta**：把项目文件逐个 `SendWriteFile: p_55a3, p_55a3/<相对路径>` 上传到远端 host（p_55a3 = 项目的发布地图 id）；依赖库若在 Res 里已有版本则 "ignore upload"（host 侧已有）。
   - `debugViaRemote, afterUploadDelta_, will InformHostLaunchGame now: p_55a3` → host 起局。
   - **spawn 游戏客户端**（实捕命令行，模板在 sceengine strings :442243-442264）：

```
D:/sce_online/version-13/SCE -env=game -editor_server_debug -editor_api_version=13 -no_update -save_replay -kcp_stream -map_kind=0 -server=editor-pd.spark.xd.com -use_local_res -host_ip=106.14.95.227 -host_port=14416 -local_test -user=38672742 -map_path="D:/sce_online/User/debug/test_res002" -to_download_list="test_res002" -width=2340 -height=1080
```

   - 然后 launcher `os.exit(0)`（控制连接断开不影响 host 局）。
4. **客户端进程**：GamePlayLocal（CreateDebugGame）→ `-local_test` → lua「本地调试, 立即连接本地host」→ KCP 直连 host → login(userid 数字) → 加载 `-map_path` 本地资源 + 收 host 同步。**服务端逻辑跑在官方远端调试 host 上**（这就是 13 版云变量必须在线的原因）。

**架构结论：B 模式 = 本地客户端（读本地项目资源）+ 官方远端调试 host（跑服务端 lua）。「脱机」= 不依赖编辑器安装，但必须带凭证在线（assign_host + host 局）。**

### 7.2 自托管复现（scegame 顶替客户端，实测跑通）

载荷：`runtime/Update/editor-pd/Res/` 从 `D:\sce_online\update\editor-pd.spark.xd.com\res\` 复制：script/client_base/startup/appui/engineres/fonts/uistyle/lite/refconfig/shadercache_windows_ui + `_m/{script/199,appui/48,shadercache_editor_dxbc(_extra),maps/{global_default/60,defaultui/63,script_libs/{lib_control/46,lib_game_options/105,smallcard_*},ai_templates/lib_common_ai/43}}`（约 620MB，待裁剪）。

**目录/凭证命名勘误（2026-08-21 更正）**：引擎**不做**域名截断——`_G.IP = argv.get('server')` 原样使用，Update 子路径与 user_info 文件名跟随 `-server` 原值（client_base-78/common/base/ip.lua:38-46 实证）。本次实测出现短名 `editor-pd` 是因为 **PowerShell 把 `-server=editor-pd.spark.xd.com` 在 `.` 处拆成了两个参数**（同一次把 `-host_ip=106.14.95.227` 拆成 `106` → wsa:10049）。**规范做法：应用内 CreateProcess 数组传参用全名** `Update/editor-pd.spark.xd.com/Res` + `User/user_info-editor-pd.spark.xd.com.json`；手动 PowerShell 测试时 `-server=...`/`-host_ip=...` 必须整体加引号。

```powershell
.\scegame.exe -env=game -editor_server_debug -editor_api_version=13 -no_update -save_replay -kcp_stream -map_kind=0 -server=editor-pd.spark.xd.com -use_local_res "-host_ip=106.14.95.227" -host_port=20906 -local_test -user=38672742 -map_path="D:/sce_online/User/debug/test_res002" -to_download_list="test_res002" -width=1280 -height=720
```

**实测结果（对编辑器刚起的新局顶替客户端）**：官方 lua 全跑（`argv has -editor_server_debug/-local_test`、`connect_host_opportunity: 1`）→ 凭证加载 → KCP 连 host `OnConnected` → login userid 38672742 **成功** → `Game host notify loading finished` → `notify start game` → `sync player scene[default]` → 服务端数据全量同步（Sync_PlayerList/Stats/Scoreboard/BagData/ShopData，test_res002 的真实服务端数据）。窗口标题「星火对战平台」，持续运行。**scegame（tester BuildPCBox 构建）作为 B 模式调试客户端完全可行。**

- 连**已结束的旧局**会 `Game host login result[2]` → return_to_lobby（局随客户端断开而销毁，重连拒绝）。
- `服务器发送了没有处理者的消息`（Sync_PlayerList 等）：客户端 lua 侧 handler 注册差异，待查（可能是 bgd 项目 protocol 注册时机或框架消息桥）。

### 7.3 自托管还差的环节（按依赖排序）

1. **控制协议客户端**（唯一硬依赖编辑器二进制的环节）：EditorLogin→UploadDelta(SendWriteFile 逐文件)→InformHostLaunchGame(EditorStartGame)。proto 名已知，需从 sceengine.dll 提取 descriptor 或抓包逆向。做完这步 = 完全不碰编辑器。
2. **暂存目录生成**：纯文件复制可自己实现；`trigger_manager.generate_lua_only`（tstl）对触编 TS 项目需要 node_modules（`_m/typescript_to_lua_node_modules/35`）——bgd 框架项目直接写 lua，可能可跳过或简化。
3. **shadercache**：`shadercache_editor_dxbc/dx11/*.cs` 编译失败（我们复制的 _m 散目录不被认，引擎要 pak 形态 `Found shadercache pak`），SSAO/bloom 渲染错误。需从 editor 环境找正确的 shadercache pak。
4. assign_host 的 Rust 实现（verify.rs 已有签名能力，加 `:9007` 端点即可；注意 editor-pd 域走 http:9007 而不是 443）。

### 7.4 另一线索（用户提示）

`official_client_deps_dll_package/23`（GameCore/GameData/GameGraph/GameUI/TriggerEncapsulation/Events dll）= 游戏 **C# 逻辑运行时**（触编 v2 的 C# 侧，`ui/AppBundle/managed/` 挂载，GameInstance 找 `/ui/AppBundle/managed/GameEntry.dll`）；`official_dotnet_bcl_package/6` = 内嵌 .NET BCL。纯 lua 项目（bgd 框架）不需要，wasm/C# 项目才走这条。暂不入 0.1.0 主链。

## 8. ★★★ 控制协议完整破解（2026-08-21，Frida 抓包实证）

> 方法：`examples/frida_capture.py`（frida spawn version-13/SCE `-generate_and_debug_map` 流程 + hook ws2_32 send/recv/WSASend/WSARecv/connect，全程抓字节）→ `examples/protocol_parse.py` 切帧解析。抓包原始件 `runtime/control_capture.jsonl`、解析件 `runtime/protocol_frames.txt`。
> 注：官方 protobuf 是手写 wire 编码（全二进制无 descriptor，`.proto` 字符串 0 命中），字段号靠抓包反推。

### 8.1 传输与帧格式

- TCP 直连 assign_host 返回的 `host_ip:host_port`（libhv，plain `connect`/`send`）。
- 帧 = `u32 LE total_len（含自身 4 字节）` + `u8 0x00` + `Envelope`。
- Envelope = proto：`f1 (wt2) Header`；Header = `f1 varint msg_type` + `f2 (wt2) body`。
- **msg_type 全部在 0xF000 段**（0xF000|sub_id）。body 为各消息自己的 proto。

### 8.2 消息表（抓包实证）

| type | 方向 | 名称 | body 字段 |
| --- | --- | --- | --- |
| 0xF000 | → | EditorLogin | `f1 varint userid`（如 38672742）、`f2 string host_token`（assign_host 返回的 uuid） |
| 0xF001 | ← | EditorLoginResult | `f1 varint result`（0=成功）、`f2 varint ?` |
| 0xF004 | → | SendWriteFile（小文件整发） | `f1 string 路径（p_xxxx/相对路径）`、`f2 string 项目名（p_xxxx）`、`f3 bytes 内容`（可空=增量跳过，见 §8.4） |
| 0xF008 | → | SendFileBlock（大文件分块） | `f1 string 路径`、`f2 bytes 块内容`、`f3 string 项目名`；块长实测 101400 字节，同一路径按序多块。**关键：分块前必须先发一个无 f3 的 0xF004 声明（host 据此创建文件），否则块数据被丢弃、host 上无此文件**（0.2.0 服务端加载失败根因） |
| 0xF00A | → | FileEnd（文件结束） | `f1 string 路径`、`f2 string 项目名`；每个文件传完（整发或分块后）发一个 |
| 0xF010 | ← | SendWriteFileAck | `f1 varint 0`、`f2 msg{ f1 string 路径, f2 string 项目名 }` |
| 0xF012 | → | **EditorStartGame**（InformHostLaunchGame） | `f1 string 项目名`、`f2 string 项目名`、`f5 varint 0`、`f10 empty`、`f11 string api_version（"13"）`、`f12 repeated 依赖库{ f1 string 版本（"46"）, f2 string 库名 }`（只列 host 已有的依赖库版本） |
| 0xF018 | ← | EditorStartGameRes | `f1 varint 0=成功`、`f5 varint session_id`（如 7676313536999653377） |
| 0xF01A | ← | 上传进度/结果通知 | `f1 varint`（0 或 100=完成?）、`f2 varint 311`（文件总数）、`f3 varint 0` |
| 0xF011 | → | EditorPing | `f2 varint seq` |
| 0xF017 | ← | EditorPingRes | `f1 varint 0` |
| 0xF00C | ← | NotifyEditorLog（host 推服务端日志） | `f1 string 时间戳（"10:57:02_509"）`、`f2 varint level`、`f3 varint`、`f4 string 源码位置`、`f5 empty`、`f6 string 内容`、`f7 string 项目名`、`f8 varint 1` |

### 8.3 会话时序（实测）

```
→ EditorLogin(userid, host_token)
← EditorLoginResult(0)
→ EditorPing(1)                    （期间心跳）
→ SendWriteFile(map_settings.json, 内容)     ┐
→ SendWriteFile/Block... 每个文件            ├ 全部文件（本次 ~311 个）
→ FileEnd(每个文件结尾)                       ┘
← SendWriteFileAck × N（host 边收边确认，乱序分片到达）
← 0xF01A(0, 311) / 0xF01A(100, 311)（进度）
→ EditorStartGame(p_55a3, api=13, 依赖库版本表)
← 0xF018 EditorStartGameRes(0, session_id)   ← 局起！
← NotifyEditorLog × N（host 持续推服务端 lua 日志）
（editor 随后 spawn 客户端；控制连接可保持收日志）
```

### 8.4 增量语义（FileCacheForRemoteHost）

- 编辑器侧有文件缓存（lastRevision → currentRev delta）；**host 侧也缓存已收文件**：未变化的文件 SendWriteFile 不带 f3 内容（空）+ FileEnd，host 复用缓存。
- 首次/全量：f3 带完整内容总是合法。我们实现全量带内容即可（简单且幂等）。
- 大文件阈值：实测 >~85KB 走 0xF008 分块（块 101400B），≤ 该值走 0xF004 整发；都以 0xF00A 收尾。
- 项目命名：`p_xxxx`（项目发布地图 id，来自 project/map_settings.json 或 editor 分配）；依赖库版本表（EditorStartGame.f12）来自项目 libs.json。

### 8.5 自托管实现要点（Rust）

1. TCP connect → 发 EditorLogin → 等 0xF001 result=0。
2. 顺序发文件：≤100KB 用 0xF004，否则 0xF008 按 101400 切块，每文件结束发 0xF00A。ack（0xF010/0xF01A）异步读即可（我方作为上传方不等逐文件 ack，官方 host 容忍流水发送）。**⚠️ 方向注意（2026-09-02 补，editor-patch host_stub 实测）：编辑器作为上传方时在本地连接下逐文件等 0xF010 ack 才发下一个文件**（不回 ack 编辑器静默卡在 update_host）——我方做 host 服务端（0.5.0 R3）必须逐文件回 ack，详见 sce_app_editor-patch/doc/research/editor-debug-channels.md §2。
3. 发 EditorStartGame（f12 依赖表从项目 libs.json 取：name + version）。
4. 等 0xF018 result=0 拿 session_id → spawn scegame 客户端（§7 命令行）。
5. 保持连接可读 NotifyEditorLog = **白拿的服务端日志通道**（0.1.0 可映射到调试 UI 日志页）。

## 9. ★★★★ 全链路自托管里程碑（2026-08-21 11:28，完全不碰编辑器跑通）

**Rust 实现**（`src/core/host.rs` 控制协议 + `src/core/debug.rs` 编排 + CLI `debug start`）首次全链实测：

```
mini-runtime debug start --project test_res002 --staging <暂存> --user 38672742 --runtime runtime/
→ 项目 p_55a3（api_version=13）
→ assign_host ✓（自签名 HTTP，106.14.95.227:20979 token=2a486ecf...）
→ 凭证注入 runtime/User/user_info-editor-pd.spark.xd.com.json ✓
→ 控制连接 EditorLogin(userid) ✓（token 被接受）
→ 上传 1243 文件（全量带内容；编辑器增量只传 311，选择规则待对齐）✓
→ EditorStartGame → session_id=7676321541143527428 ✓（远端局起）
→ spawn scegame 客户端 ✓
   客户端日志实证：Connect to game host server 106.14.95.227:20979 → OnConnected
   → Game host login result[0] → gamePath_: <暂存目录>（本地资源）
   → notify loading finished → notify start game → sync player scene[default]
```

**进程树零编辑器组件**（无 星火编辑器.exe / version-*\SCE）。0.1.0 核心目标达成。

### 9.1 实测修正/补充的事实

- **EditorStartGame f12 完整依赖表 = libs.json 全量（9 库，版本取载荷 `_m/maps/<subpath>/<ver>` 目录名）+ 固定三本地库 `server_lua_plus/-1, server_common/-1, global_default/-1`**（-1 = 本地未发布库随图上传）。缺库/错版本 → host 返回 result=-3（varint 按 u64 读为 18446744073709551613）。
- **EditorStartGameRes 的 result 是有符号**（int32），负数按 10 字节 varint 编码。
- CLI 注意：exe 是 windows 子系统（无控制台），CLI 输出走 `AttachConsole(ATTACH_PARENT_PROCESS)`（同 bgd_sce_tools）；管道捕获下由调用方 pipe 直收。
- 上传文件集：我们全量传了整个暂存目录（1243 个，含 ui/spine 图等）；编辑器只传 311 个（UploadDelta 选择规则待逆向对齐——多传无害但每次全量慢，M6 优化）。

### 9.2 剩余工程（M5/M6）

- **M5 暂存自生成**：`ui/script/main.lua` = 官方模板头 + `---origin_main_file---` + **源 ui/src/main.lua 原文** + `---ts_module---` 尾（模板段固定可内嵌；libs 段按 libs.json 生成；i18n 行带 ProjectName）。服务端 `script/main.lua` 项目里已由 bgd build 生成好，直接复制。白名单目录见 0.1.0.md §2.4。
- **M6**：GUI 调试页实测、上传文件集对齐编辑器规则、shadercache pak、载荷裁剪（目标 <300MB）。

## 10. M5 完成与全链终验（2026-08-21 12:00）

**staging 自生成（core/staging.rs）+ 全链终验通过**——这次连暂存目录都是我们自己生成的（`--staging` 省略 → 自动 `<runtime>/User/debug/test_res002`，44.2MB/1241 文件）：

```
debug start --project test_res002 --user 38672742 --runtime runtime/
→ 暂存自动生成 ✓ → assign_host ✓ → EditorLogin ✓ → 上传 1241 文件 ✓
→ EditorStartGame → session_id=7676329903444852744 ✓ → 客户端 pid ✓
→ 客户端日志（lua-game）实证：
   [bgd-game] bgd-game-common(1.0.x)      ← .bgd/src/common/init.lua 跑在官方运行时
   [bgd-game] bgd-game-client(1.0.x)      ← .bgd/src/client/init.lua
   [bgd-game] 游戏客户端启动-game-entrance-client   ← p_55a3/main.lua:67（我们包装的入口！）
```

**至此 0.1.0 主链完全闭环且完全脱机**：暂存生成、assign_host、控制协议上传/起局、客户端拉起、bgd 游戏代码执行——全程无编辑器进程、原理上可无编辑器安装。

### 10.1 M5 实测修正的事实

- **载荷 Update/<env>/ 下必须有 api_pak_version.json / map_pak_version.json / VERSION.JSON**（从官方 update 目录复制）——缺了会导致 `@global_default.lua_declare` 等库模块解析不到（包版本定位失败）。
- **依赖库版本号的权威来源 = `api_pak_version.json[<api_version>][<库短名>]` 注册表**（编辑器同款：api13 → lib_common_sounds=16/lib_control=46/defaultui=63…）。取最新本地版本会踩雷（lib_common_sounds/17 挂载后 `@lib_common_sounds.main` not found——注册表锁 16）。assemble 脚本与 resolve_libs 均已按注册表对齐。
- **`@gameui` 库被 bgd 客户端入口引用**（ui/src/main.lua 的 `@gameui.simple_ui.init`）：载荷需 `_m/gameui/<ver>`，**版本由 api_pak_version.json 按 api_version 决定**（api 12→47，13→48，2000→52）。
- **载荷组装/升级已脚本化**：`examples/assemble_runtime.ps1`（393MB，从本机 tester+编辑器 update 目录复制，版本自动跟随注册表）。编辑器升级后重跑即完成载荷升级。脚本产物全链实测通过。
- **UI 无文字 = 缺字体族**：游戏字体在 `res/ui/font/<族名>/`（regular.otf/regularbold.otf…共 10 族 ~195MB），引擎按族名目录解析（`Can't find font family: ui/font/regular`）。载荷只需 `ui/font/regular`（12.5MB，默认 UI 字体）；notoemoji 无实体文件（emoji 回退，告警可忽略）。**不要**把 tester 的 ui/（大厅 UI）混进来。
- **CLI 截屏自验**：`mini-runtime capture [--title 星火对战平台] [--out x.png]`（WGC 截窗口，方案借自 editor-patch capture.rs；最小化先 SHOWNOACTIVATE 恢复）。已用其实测证明字体修复（verify.png 文字全渲染）。
- 触发器 stub 策略可行（test_res002 的触发器未编译为 stub，客户端主流程不受影响）。
- python subprocess.run 等待坑：子进程（scegame）继承管道导致 run() 等 EOF 不返回——测试用 Popen + 文件重定向。

## 11. 服务端未运行根因与修复（2026-08-21，0.2.0）

**现象**：局起、客户端进图正常，但服务端逻辑没跑（不刷怪、背包/装备/商店无效，Req_* 全部「无人处理」）。

**诊断**：用新增的 `debug start --hold <秒>`（保持控制连接收 0xF00C NotifyEditorLog）拿到 host 服务端日志，直接看到 `p_55a3/main.lua:9 init_cache 失败 → 首次加载lua失败`。

**两个上传协议 bug（均抓包对照实证）**：

1. **大文件（>101400B）分块前缺 0xF004 空声明**：官方序列 = `0xF004(f1 path, f2 project, 无 f3)` → `0xF008 × N` → `0xF00A`。host 收到 0xF004 才创建文件登记；缺声明则块数据无处落盘 → host 报 `no file '.../maps/p_55a3/script/obj/effect/actor/data.lua'`（1.38MB 大文件全灭）。
2. **上传路径必须全小写**：host 是 Linux（/work/bin/...）大小写敏感。官方把盘上 `ToSceClass.lua` 传成 `tosceclass.lua`（抓包 f1 实证）；我们原样传大写 → host require 小写模块名时 not found（bgd `api/init.lua` 聚合 `@.../tosceclass`）。

修复后 host 日志全绿：`首次加载lua完成` → bgd-libs/game-server 初始化 → 玩家上线 → **BOSS 刷出/技能书刷出/超时消失循环**——截屏实证（BOSS 巨蝎/远古巨兽在图、计分板、攻击力=7 服务端同步）。

**连带发现**：scegame 客户端运行时会动 `-map_path` 暂存目录的文件（疑似 FileRemover 类清理），staging 每次 start 重新生成即可，无实际影响。

**进程卡死坑（用户实测反馈）**：CLI spawn 游戏客户端若用 std Command（bInheritHandles=TRUE），游戏进程会继承调用方管道句柄 → python subprocess/终端工具等管道 EOF 一直阻塞到游戏窗口关闭。修复：`spawn_detached` 走裸 `CreateProcessW(bInheritHandles=FALSE)`（core/debug.rs），会话改存 pid + OpenProcess/GetExitCodeProcess 轮询。

## 12. ★ 载荷 0 依赖下载通道（2026-08-21 打通并全链验证）

> **载荷体系细节已抽出到 [payload-packages.md](payload-packages.md)**（update-info 契约/包格式/落位/注册表/基座资产/版本跟随），本节保留关键结论。
> **⚠️ 引擎归属修正（2026-08-21 补）**：本节说的 `win` 包经 MD5 实证 = **对战平台引擎**（与 tester 的 scegame 一致），不是编辑器引擎（version-\<api\>/sceengine.dll）。B 模式客户端用它可行（登录/跑游戏能力一致），但研究编辑器 native 行为时注意区分两套运行时——详见 [runtimes.md](runtimes.md)。

**官方自更新通道逆向结论**（tester/编辑器共用）：

1. **版本发现 = update-info**：`POST https://updater-pd.tapsce.cn/api/map/update-info?<参数全在 query string>`，**空 body、免签名**（body 放参数会 500！list 用 `;` 分隔）。响应行 3 = JSON：`items[] = {name, version, url, md5, size, original_size, path, variation, api_version}`，依赖自动展开（查项目库会带出 global_default/spark_core/lib_ui 等）。
2. **包下载**：`https://<item.url>`（OSS 公共读，直连/代理均可）。7z 内容 = 单个 `<Name>.pak`（+ 库的 libs.json）；**部分包 TNND 加密**（magic + XOR CREATEEASY，下载后先按 magic 解密——engineres 实测命中）。
3. **引擎二进制也在此通道**：包名 `win`（v152 = scegame + 全套 dll + embedded_packages 骨架，62MB）+ `winlauncher`（tester 壳，不需要）。**彻底摆脱 tester 安装依赖**。
4. **编辑器客户端包**：`wineditor`（v148, 193MB，含 version-2000 客户端 + 基础 res 包 fonts/lite/refconfig 等）。`variation=windows_editor` 可查编辑器侧包（xdeditor 等）。
5. **update-info 不覆盖的资产**：`ui/`（游戏字体族 ui/font/regular 等）、`characters/`、`effect/`、`anim/` 等编辑器基座资源——任何 list 组合都不返回。这些只能来自编辑器安装包 → **0.2.0 决策：打成 base_assets.7z 随我们自己的 release 分发**（稳定很少变）。**仓库为私有**：asset 直链匿名 404，下载必须走 GitHub API（`releases/latest` 定位 asset → `asset.url` + `Accept: application/octet-stream` + token，同 bgd_sce_tools net.rs 方案）；token 取 env `MINI_RUNTIME_GITHUB_TOKEN` → 凭据管理器 `bgd_sce_tools/github_token`（注意：该 fine-grained PAT 必须勾选 sce_app_mini-runtime 仓库，否则 API 404）；env `MINI_RUNTIME_BASE_ASSETS_URL` 可覆盖为任意公开直链。
6. **落位布局**（实测）：基础包 `Res/<name>/<name>.pak`（client_base/startup/fonts/engineres/uistyle/shadercache_windows_ui/lite/refconfig）；注册表版本包 `Res/_m/<name 或 maps/...>/<ver>/<name>/<name>.pak` + **散文件解包**（_m/maps 库与 script/appui/gameui 需要散文件，UPAK 自研解包器：头 UPAK+u32 数+u32 校验；条目 名字\0+u32 off+u32 size+**u32 条目校验**）；库 libs.json 随包。
7. **api_pak_version.json 合成**：`#package_path`（name→Res/_m/... 路径）+ `<api>` 表（name→version），**凡 _m 落位的包都要登记**（含依赖展开项），否则引擎回退 `Res/maps/<lib>` 报 cannot find package。
8. **版本跟随**：每次 sync 实时查 update-info = 自动跟随星火服务器侧更新（实测 startup 已从 364 升到 365）。

**终验（C:\mini_dl_test）**：100% 官方通道下载载荷 → debug start → host 服务端全绿（首次加载lua完成/刷怪）→ 客户端 bgd 入口执行 → 截屏文字/BOSS/技能书全渲染。

（待续）

## 13. ★★★ KCP 会话层破解（client↔host 游戏会话，2026-09-02 中继抓包 + VM oracle 实证）

> 抓包平台 = local_host.rs 中继（`host_capture-*.jsonl` 全流量），分析工具 = `examples/kcp_capture_parse.rs`。本节是 §8 控制面之外的**会话面**权威记录。

### 13.1 拓扑与端口规律

- 控制面 TCP 与会话面 UDP/KCP **同主机、端口相差 50（引擎硬编码）**：客户端 `-host_port=5003` 时实际 dial `127.0.0.1:5053`（Network 日志 `KCP will connect to` 实锤）；云端 13738→13788、20770→20820 同规律。
- KCP 建连是客户端 lua VM 启动的前置闸门：KCP 失败 → `kcp connect failed at 4.010000` → lua-game 日志 0 字节，卡死在进局前。

### 13.2 CE1 握手族（抓包 + 字符串实证）

```
c→h  CE1SYN    (13B，重发至应答)
h→c  CE1SYACK  (16B: magic + conv(4 LE 下发) + 4B)
c→h  CE1ACK    (6B)
h→c  CE1SYNACK (16B，重发至首个 KCP PUSH)
```

dll 字符串另有完整控制魔法族（KCPNetwork.cpp 段）：`CE1REP`（连接替换，`[kcp] try replace connection %d %f`）/ `CE1RES`（语义未实证，疑为 reset/resume 类）/ `CE1DISCONNECT` / `CE1TIMEOUT` / `CE1PAUSELOADING` / `CE1PAUSE`。相关 argv：`kcp_stream`（流模式）、`no_kcp`、`kcp_redundant`、`[kcp] %x fec switch %d`（FEC 运行时开关）。

### 13.3 KCP 段与流式分帧

标准 KCP 头（全 LE）：`conv(4) cmd(1) frg(1) wnd(2) ts(4) sn(4) una(4) len(4)` + payload；cmd 0x51=PUSH / 0x52=ACK；每客户端独立递增 conv（0x14/0x15…）。
`-kcp_stream` 流模式：PUSH payload 拼成字节流，**消息分帧 = 3 字节 LE 长度（含自身）+ 消息体**；流重组须按 sn 排序去重后拼接再分帧（逐段当整帧会错位）。

### 13.4 c2h（客户端→host）= 明文 protobuf

外壳 `f1{ f1 varint msg_type, f2 bytes body }`。已识别：1=登录{f1 userid, f2 "userid" 串, f6 "", f7=1, f8=1}；3=加载进度{f1 递增值 30..100}；5=状态；0x2000=周期心跳{f1 i32, f2 i32}；0x6011=UI 视图同步{f6="main[map_view]>P0>P0" UI 路径, f8 hash}；**0x7006=玩法协议{f2=cmsg_pack}**；0xF100=时钟同步{f1{f1=unix_ms, f2=conv}} + f2 double。

### 13.5 玩法消息体 = cmsg_pack（msgpack 变体）

- VM oracle 实证：`cmsg_pack.pack({type='Req_PlayerList', args={}})` 产出与抓包 c2h 0x7006 内层**逐字节一致**（`82 c4 04 type c4 0e Req_PlayerList c4 04 args 90`）。
- 格式 = MessagePack 变体：字符串用 bin 家族（0xc4 bin8 等）而非 fixstr/str8。
- 客户端 lua 收发界面（script-199 common/base/server.lua:184-234）：发送 `base.game:server(type)(args)` = `cmsg_pack.pack({type,args})` → native `game.send_ui_message`；接收 native 回调 `base.event.on_ui_message(str)`（旧）/ `on_ui_message_new(str, type_id, type_name)`（新，**type_id 数字映射表由首次下发的 type_name 建立**）→ `cmsg_pack.unpack(str)` → `proto[type](args)`。
- h2c 语义层旁路：VM 内 hook 上述两个回调即可拿到全部解码后消息（已实证）。

### 13.6 h2c（host→客户端）传输层 = ZCompress 自研 Huffman 压缩（无加密！）

初判"加密"系误判，证据链：熵 7.36（加密应≈8.0）；首 5 字节 68% 消息相同（每消息内嵌 Huffman 树头——后查明实为初始树编码的公共前缀）；成对 XOR 长零串（共享明文前缀）；deflate/zlib/lz4/zstd 全解不开；引擎字符串实锤 `ZCompress.cpp` / `ZCompressAdapter.cpp` / `when huffman encoding...` / `rebuild huffman tree finished` / `[Compress(%x)]`（源路径 `D:\BuildPC\NE_pd\Client\src\Game\Network\ZCompress\`）。
h2c 帧 = `[3B LE 帧长][ZCompress(消息)]`。**无密钥无密码学，只剩位流格式逆向**（两条路：dll 反汇编编解码函数 / 已知明文对照推断）。~~待验：压缩是否有"不压缩"标志位~~ → 已实锤，见 §13.8。

### 13.8 ★★★★ ZCompress 位流格式权威（2026-09-02，R1 完成：Rust 复刻 + 实机 oracle 全量互验）

> 实现 = `src/core/zcompress.rs`（纯算法零依赖）。逆向自 sceengine.dll（version-13 editor 构建）反汇编 + Frida oracle（hook 解码入口记录输入/输出对）双路互证。
> **验收数据**：① Frida oracle（`test/temp/zc_oracle.py` → `test/temp/zc_oracle.jsonl`）1016/1016 条消息 (输入,输出) 与自研解码逐字节一致（`test/temp/zc_check.rs`）；② 基准 capture `host_capture-1788302184.jsonl` 3781/3781 帧 100% 解码通过（`test/temp/zc_verify.rs`）；③ 新基准 capture `host_capture-1788336006.jsonl` 1026/1026 帧通过。

**帧格式**：`data[0]` 有符号 >= 0（0x00-0x7F）→ **原样模式**，载荷 = `data[1..]`（消息级"不压缩"标志，解压入口 VA 0x181ac0820 实锤：非运行时开关，是逐消息标志字节；无连接级开关）；否则压缩模式，**bit0 = 压缩标志 1**，从位偏移 1 起解码。编码端可直接用原样模式（合法旁路，host 侧 encode 采用）。

**位序**：MSB-first（读 1 位 = `(byte << bit_off) & 0x80`，0x181ae4030；读 n 位同理高位先出，0x180e43130）。越界容错：读 n 位时整字节段字节不足则整段放弃返回已有值；起始即越界返回 0。

**压缩模式 = LZ77 + 自适应 Huffman**（解码主体 0x181ac3d50，连接级有状态对象，**树/环窗状态跨消息持续**）：

- 字母表 291 = `0x123` 符号：`0x00-0xFF` 字面量；`0x100-0x121` 长度；`0x122` END（消息结束标记）。
- 长度与距离共享字母表：读完长度符号后，下一符号按距离解释。
  - 长度 = `extra(sym - 0x100) + 3`
  - 距离：sym < 0x100 → `sym + 9`；0x100 <= sym < 0x108 → `sym - 0xFF`（即距离 1-8）；>= 0x108 → `extra(sym - 0x100) + 0x100`
  - `extra(c)`（0x181ac4040）：c < 8 → c；否则 `e = (c>>1) - 3`，**流内剩余位 < e 时容错返回 c**，否则 `read_bits(e) + (1<<(e+1)) + ((c&1)<<e) + 4`。（值域：c=8,9 → 8-11；c=10,11 → 12-19；… c=33 → 24580-32774）
- LZ77 环窗 0x8102 = 33026 字节（内联在解码对象头部）。窗口绝对位置簿记：`f8110`（写入计数，封顶 0x8102）/`f8118`（环内最旧字节绝对位置）/`f8104`（环满时缓存写位置）；读位置 < 窗口基址时读 0（不报错），环索引越界置错误标志。精确状态机见 zcompress.rs `emit`/match-copy（逐指令镜像）。
- 树走读（0x181ac40c0）：节点 16B `{weight i32, sym u16, child[2] i32}`（叶子 child0 == -1），从根逐位下钻 child[bit]；位流耗尽置错误标志返回 0。
- **自适应策略（0x181ac4b40，每消息解码后执行）**：该消息全部符号（含 END）权重 +1（某符号权重达 0x7FFFFFFF 时本批整体回滚并永久停更）；然后若本消息输出字节数 > 100 且（重建计数 < 2000 或输出 > 1000）→ 全量重建树。
- **建树（0x181ac4660）**：291 叶子（weight = 权重数组值）入桶式优先队列（按权重升序，同权重 FIFO，合并节点进桶尾）；反复弹两个最小者合并为新内部节点（child0 = 先弹出者 = 位 0 侧），直至剩一根。初始权重全 1（对象构造时初始化 + 重建，0x181ac4180）。节点容量 0x247。错误路径：解码错误（位流耗尽/距离异常/环越界）→ 状态冻结（不做权重更新/重建）。

**KCP 数据报批包（重要，0.4.0 抓包解析器的坑）**：host 侧会把多个 KCP 段合并进一个 UDP 数据报（实证：95B 数据报 = ACK + ACK + PUSH）。解析器必须在一个数据报内按 `24B 头 + len` 逐段迭代，否则丢段。旧分析"首消息丢失/丢包"实为此解析缺陷，中继抓包本身无损。

**encode 侧**：host 引擎编码器不在客户端 dll（无从反汇编）；但解码格式已完全确定，合法编码 = 任选 LZ77 解析 + 相同自适应规则的 Huffman 发射，或直接用原样模式旁路。0.5.0 采用原样模式（`encode_frame_raw` = `0x00 + 明文`），引擎侧终验路径：自研 host 发包 → 真客户端解码 → hook `on_ui_message` 对账（R3 收口）。

**Frida oracle 手法（复用价值高）**：hook `SCEEngine.dll` RVA 0x1ac4ab0（解码入口），onEnter 读 reader（`[rdx]=base [rdx+8]=end`）取输入，onLeave 读输出向量（`[r8]=count [r8+8]=base`）取明文。脚本 `test/temp/zc_oracle.py`，对账器 `test/temp/zc_check.rs`。

### 13.9 ★★★ h2c 应用层消息表 + 登录→进局全消息序列（2026-09-02，R2 全解码产出）

> 工具：`examples/kcp_capture_parse.rs` 升级为全解码（新子命令 `decode`（时间线回放）/ `msgs`（聚合表）/ `dump <conv> <type> [n]`（按类型全量 dump）；**逐段解析合并数据报**；h2c 经 zcompress.rs 解码）。
> 验收：基准 capture 1788302184（0.4.0 验证局，conv 0x14+0x15）全量回放 0 解码失败；新基准 1788336006（conv 0x6）1704 行时间线 0 失败。Req_*/Sync_* 人眼可读。
> 信封结构（双向通用）：`外层 {f1: header_bytes}`，header = `{f1 varint msg_type, f2 bytes body}`。

**c2h（明文）消息表**：

| msg_type | 语义 | body 布局 | 频次特征 |
| --- | --- | --- | --- |
| 1 | 登录 | `{f1 userid, f2 "userid"字符串, f6 "", f7=1, f8=1}` | 连接建立后首发 |
| 3 | 加载进度 | `{f1: 30/45/95/100}` 递增 | 客户端自驱，进图期 |
| 5 | 状态 | `{f1: 0}` | 进图末期 1 次 |
| 0x2000 | 心跳 | `{f1 i32, f2 i32}`（常为 0,0） | ~0.27s 周期 |
| 0x6011 | UI 视图同步 | `{f1..f5, f6=UI路径字符串, f8 hash}` | UI 变化时 |
| 0x7006 | **玩法上行** | `{f1: cmsg_pack {type:"Req_*", args}}` | 玩家操作驱动 |
| 0x1001 | 周期探测（疑似 RTT/保活） | `{f1: 递增值, f2: 11B 载荷}` | ~1s，与 h2c 0x1108 成对 |
| 0x7300 | 空 body | 进图后 1 次 |
| 0xF100 | 时钟同步请求 | `{f1 {f1 unix_ms, f2 conv}, f2 double}` | 周期，与 h2c 0xf101 成对 |

**h2c（ZCompress 压缩）消息表**：

| msg_type | 语义 | body 布局 | 备注 |
| --- | --- | --- | --- |
| 2 | 登录应答 | 340B：`{f1=0, f2={f1 session_id, f2=5, f3={f1 "p_55a3"}(地图id), f4×N=玩家席位表{f1 userid/序号/等级…}, f6=127}, f3={f1 "FluctuateThreshold", f3=150}, f4 f32, f5=本人 userid, f6=1}` | 登录后立即下发 |
| 0x15 | 登录后小状态 | `{f1=0, f2={repeated {f1 id, f2 value}}}`（2 组 (0,0),(1,0)） | 跟随登录应答 |
| 0x31007 | **服务器 tick 驱动** | `{f1: 递增计数}` | ~200ms 周期，最高频（748 次/局）——驱动客户端 `base.event.on_update` 族 |
| 0x1108 | 0x1001 的应答 | 50B | 成对 |
| 0xf101 | 时钟同步应答 | | 与 0xF100 成对 |
| 0x7008 | **玩法下发（UI 消息通道）** | `{f1: cmsg_pack args（map）, f2: seq 序号, f3: type_id, f4: type_name（仅该 type_id 首次出现携带）}` | = 客户端 `on_ui_message_new(str=f1, type_id=f3, type_name=f4)` 的线格式；Sync_*/S2C_* 全走这里 |
| 0x6 / 0x100 / 0x109 | 空 body 控制消息 | | 进图期各 1 次 |
| 0x102 | `{f1=0, f2={f1=userid, f2="", f3=0, f4=1}}` | ×2 | 进图期 |
| 0x112 / 0x1105 / 0x1120 / 0x103 | 进图大消息（1403B/1254B/811B/476B） | 场景/单位全量初始化 | |
| 0x5004 / 0x10d / 0x10e / 0x1129 | 进图中小消息 | 0x10d 含 "GameMode" 字符串 | |
| 0x1128 | `{f1=0, f2={f1: fixed64?}}` | ×123 | 周期 |

**登录→进局全消息序列（conv 0x6 实测，ts 为相对 ms）**——host 侧每阶段职责即 R3 的行为基线：

```
c→h 1 登录(userid)                                    ← KCP 建连后客户端主动登录
h→c 2 登录应答(session_id/地图 p_55a3/席位表/FluctuateThreshold=150)
h→c 0x15 小状态
c→h 3 加载进度 30 → 45（客户端自驱，host 无需应答）
c→h 0x2000 心跳开始（~0.27s 周期，host 无需应答）
c→h 3 进度 95；c→h 0x7006 Req_PlayerList（客户端开始请求玩法数据）
c→h 3 进度 100；c→h 5 状态(0)；c→h 0x6011 UI 视图同步
h→c 0x6（空）→ 0x102 → 0x31007 tick 开始（此后 ~200ms 周期不断）→ 0x100（空）
h→c 0x7008 __sync_game_info(session_id)  ← 玩法通道首开
h→c 0x112(1403B) → 0x5004 → 0x1129 → 0x1105(1254B) → 0x1120(811B) → 0x10e → 0x103(476B) → 0x10d(GameMode) → 0x102 → 0x109（空）  ← 进图初始化消息群
h→c 0x7008 ×N：Sync_PlayerList/PlayerStats/Scoreboard/BagData/ShopData/SkillLevels/PlayerTeamMap（+ensure 重发一轮）→ S2C_async_nick
h→c 0x103(25B)；c→h 0x7300（空）→ c→h 0x6011；h→c 0x7008 ticket_info_update
此后稳态：c→h 0x2000 心跳 / 0x1001 ↔ h→c 0x1108 / 0xF100 ↔ 0xf101 / h→c 0x31007 tick / 0x1128 / 玩法 0x7006→0x7008
```

**对 R3 的硬性结论**：① 客户端 0x6011 与 msg 5 收到即弃（host 不处理不断连）；② 登录必须应答 type 2 + 0x15（否则客户端卡在登录）；③ 0x31007 tick 必须持续下发（客户端逻辑帧由它驱动，200ms 周期）；④ 玩法请求 0x7006 的应答走 0x7008（f3 type_id 可按到达顺序递增分配，f4 首次携带名字）。

**R4/R5 补充实证**：⑤ 0x7006 内建通道 `__client_key_down/up`（{player_id, key}，script-199 game.lua:517-525）不走 base.ui.proto，host 原生转 玩家-按键按下/松开 事件；`move_to_direction`/`stop_move_to_direction`/`__client_mouse_*` 同为内建通道（官方由 host 原生世界模拟消化，B+ 路线弃）。⑥ type 2 登录应答的本人 userid（varint）出现 2 处、0x102 各含 1 处——多账号局必须按实际登录 userid 原位补丁（varint 等长约束：本地账号 userid 分配在 4 字节 varint 区间 90000001 起）。⑦ 广播（base.game:ui）在零就绪会话期产生需挂起补发（官方语义 = 后进玩家拿世界状态）。

### 13.10 分析工具与 oracle 手法

| 工具/手法 | 用途 |
| --- | --- |
| `examples/kcp_capture_parse.rs` | host_capture jsonl 全解码：stats / flow / **decode（时间线回放）/ msgs（聚合表）/ dump（按 msg_type 全量 dump）**；逐段解析合并数据报；h2c 经 zcompress.rs 解码；零外部依赖可 rustc 直编 |
| server 探针（临时 `src/server/api_probe.lua`，已删，结论见 self-host.md §11） | 云端 server VM 内 dump 全局/package.loaded/base.* → log.info → 0xF00C 回读（用后还原） |
| VM hook `on_ui_message(_new)` | h2c 语义流取证（绕过压缩层） |
| `cmsg_pack.pack` 已知明文生成 | 与抓包字节对照，推 ZCompress/分帧格式 |
