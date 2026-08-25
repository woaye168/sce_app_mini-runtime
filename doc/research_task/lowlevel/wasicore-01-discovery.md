# wasicore-01 — 星火 2.0（WasiCore / C# / WASM）官方全代码栈发现

> 2026-08-24 | 起点：render-07 遗留线索「触发图是否暴露 Sprite2D/TextureRect」→ 触发图（xdeditor V1/V2 触发编辑器）**没有** Sprite2D 节点（证伪）；但沿引擎字符串挖出**官方第二开发栈**——星火 2.0 = WasiCore（C# 编译 WASM 跑在引擎内嵌 wasmtime 上），本机编辑器已随包携带。
> **本发现改变整个研究版图**：用户要的「渲染底层 API」与「更强云数据」在 2.0 栈里全部有官方答案；但 1.0 lua 项目（test_res002, api 13）**不能直接用**，两个栈是完全独立的项目类型。

## 1. 发现链（证据）

1. scegame-tester 字符串（`bgd_glzy/.editor_src_mirror/scegame-tester-strings.txt`）：
   - `SCEImportStaticSprite2D` + `StaticSprite2D_Get/SetTextureRect_Import` 等（531298-531307+）——Urho2D 全套以 **wasm import** 形式暴露给 C#。
   - `==> StateGame Open Wasmtime`（525769）、`WasmtimeState::OpenWasmtimeState - appbundlePath = [%s]`（525727-525729）、`WasmtimeStateGame::GetAppbundlePath - useGameBundle %d`（525853）——**进游戏态就打开 wasmtime 子系统**，游戏可带自己的 AppBundle。
   - `D:\BuildPCBox\NE_pd\Client\src\Game\Wasmtime\RuntimeMcpBridge.cpp`（525625）——客户端官方 Runtime MCP 桥（见 wasicore-04 §7）。
   - `SCE_on_*_export` / `SCE_GUI_on_*_export` / `SCE_PHYSICS2D/3D_*_export`（525786-525836）——引擎期望游戏 bundle 实现的完整 ABI（生命周期/输入/UI 事件/物理回调/nanovg canvas 渲染）。
2. 磁盘上的 AppBundle 实物：
   - `res/_m/maps/gamesparkcore/10/.../ui/AppBundle/managed/GameSparkCore.dll`（308KB）
   - `res/_m/maps/gamesystemui/14/.../ui/AppBundle/managed/GameSystemUI.dll`（537KB，含 zh-cn 资源 dll）
   - `res/_m/maps/official_client_deps_dll_package/23/.../managed/` = GameCore/GameGraph/GameUI/GameData/TriggerEncapsulation/Events（render-07 已逆向）
   - `res/_m/map_templates/64/map_templates/code_sample/` = **官方 C# 示例项目模板**（约 30 个示例 + GameEntry.sln）
   - `D:\sce_online\version-2000\appbundle`（大厅侧）
3. **WasiCoreSDK v18** 完整随包：`res/_m/wasm/wasicoresdk/18/wasicoresdk/{api,docs,lib,resources,schemas,templates,tools}`——含客户端/服务端/共享 API 声明（.cs）、完整中文文档（guides/systems/best-practices/ai skills）、SDK 程序集。
4. `api_pak_version.json` 顶层键只有 `"12"`、`"13"`、`"2000"`：`"2000"` 注册表含 `wasicoresdk:18`、`map_templates:64`、`wineditor:143`、`gamesparkcore:10`、`gamesystemui:14`——**当前编辑器安装即具备 2.0 能力**。

## 2. 两个栈的关系（关键结论）

| | 1.0（现有项目） | 2.0（WasiCore） |
| --- | --- | --- |
| api_version | 12 / 13 | **2000**（show_name "wasm"） |
| 语言 | TypeScript → ts2lua（script 库） | C#（.NET 9/10, WASI）双端单源 `#if SERVER/CLIENT` |
| 触发编辑器 | V1/V2（生成 lua） | **V3**（生成 C#，`src/TriggerGenerated/*.cs`） |
| 运行时 | lua54 VM | 引擎内嵌 wasmtime 跑 `AppBundle/managed/GameEntry.dll` |
| 工程结构 | `script/` + `ui/script/` | `src/`（C#）+ `editor/data/`（JSON）+ `scene/`，无 script/ 目录 |

- **不存在 lua+C# 混合项目**：单个项目二选一。code_sample 里 `HybridFlappyBird` 的 "Hybrid" = 混合同步策略（Node transform 同步 + PropertyHost 属性同步），不是 lua+C#。
- 编辑器安装同时携带两套包（api 2000 注册表里仍有 script:197 等 1.0 包），但那是编辑器级共存。
- 1.0 → 2.0 = **项目级移植**（官方 migrate-1to2 skill，AI 作业手册），逻辑层本质是重写；场景/地形/res/ui 资源资产同格式可直接复制（见 wasicore-04 §3）。

## 3. 对本研究两大主线的意义

### 渲染（用户痛点 → 2.0 官方答案）
| 用户痛点（1.0） | 2.0 官方 API | 详见 |
| --- | --- | --- |
| tiled 图集子图只能 clip/sprites 迂回 | `StaticSprite2D.TextureRect`（源矩形）/ `Canvas.DrawImage(image, sourceRect, destRect)` | wasicore-02 §1/§2 |
| canvas_texture_* 线上硬崩 | **Canvas/CanvasAnimated**（NanoVG 封装，官方一等公民，线上可用） | wasicore-02 §2 |
| 特效必须数编关联 | `ParticleSystem.Load("effect/x.effect")` 直载本地文件 + `CreateRuntime()` 纯代码构建 | wasicore-02 §3 |
| 本地模型不会用 | `Prefab.Load(".../model.prefab").CreateInstance(parent)` + SceneGraph | wasicore-02 §4 |
| spine 局限 | `AnimationSet2D.Load("x.skel/.scml")` → `AnimatedSprite2D` | wasicore-02 §5 |
| 无离屏渲染/视口 | `Texture2D.CreateRenderTarget` + RenderSurface + 多 viewport（RTT 可贴 UI/材质/Canvas） | wasicore-02 §4 |

### 云数据（1.0 云变量 → 2.0 CloudData）
2.0 = 结构化多桶 KV + UUID 列表 + 唯一名称注册表 + **跨用户 ACID 事务** + 游标扫描 + 模糊名称搜索（wasicore-03）。传输层被 `IUserCloudDataProvider` 完全封装，docs 无协议细节——**新逆向线索：2.0 云数据 op 面（query/commit/scan/claim）大概率仍走 Entrance 通道 + MessagePack 族**（framework_overview.md 出现 MessagePack），若证实，我们的 entrance_client 直连思路可扩展到 2.0 富操作。

## 4. 战略判断（给用户的决策输入）

- **1.0 项目（test_res002）维持 lua**：本台账既有结论仍是生产答案（render-02~08 通道 + entrance_client 直连云变量）。
- **新项目/愿意重写的项目**：2.0 是官方未来栈，渲染/云数据/UI/调试（Editor MCP + Runtime MCP 官方 AI 闭环）全面超越 1.0；本机编辑器新建项目选 code_sample/game_entry 模板即可零成本起步。
- **迁移成本**：中大。逻辑从 ts2lua 触发器模型重写为 C# 事件模型；官方明示无法 100% 自动迁移；2.0 尚缺小地图/UI 场景/场景拼接/部分小卡片（04-limitations）。bgd_sce_framework 整套 lua 基建在 2.0 无对应物。
- **2.0 已上线**：编辑器已发布线上；TapTap App 内嵌客户端已上线；部署环境 alpha/production × 运行时 debug/test/formal 云数据隔离。

## 5. 资源定位速查

| 内容 | 路径 |
| --- | --- |
| SDK 根（v18） | `D:\sce_online\update\editor-pd.spark.xd.com\res\_m\wasm\wasicoresdk\18\wasicoresdk\` |
| API 声明（双端） | `SDK\api\{client,server,shared}\*.cs` |
| 官方文档 | `SDK\docs\{guides,systems,best-practices,veteran-user-guide,ai\skills}\` |
| 示例模板项目 | `D:\sce_online\update\editor-pd.spark.xd.com\res\_m\map_templates\64\map_templates\code_sample\` |
| 示例索引 | `code_sample\src\EXAMPLES_INDEX.md`；切换示例改 `src\GlobalConfig.cs` 的 `TestGameMode` |
| 框架 dll（TNND 解密件） | `sce_app_mini-runtime\test\temp\managed_dec\`（GameCore/GameGraph/GameUI 等，render-07） |
| 引擎 wasm ABI 字符串 | `bgd_glzy\.editor_src_mirror\scegame-tester-strings.txt` 525159-525853、526146-531307 |
