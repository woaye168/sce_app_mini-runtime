# wasicore-04 — 2.0 项目采用路径（创建/迁移/构建发布/限制/版本/Runtime MCP）

> 2026-08-24 | 来源：`SDK\docs\guides\{quickstart,projectstructure,editor2publishandaiworkflow,deploymentandruntimeenvironments,clientruntimecompatibility,runtimemcpdebugguide}.md`、`veteran-user-guide\{00,04}.md`、`ai\skills\{migrate-1to2,wasicore-game-scaffold}\`、模板工程 `map_templates\64\map_templates\code_sample\`

## 1. 创建 2.0 项目

- **正确起点 = 星火编辑器「新建项目」菜单**选模板：`game_entry`（最小可运行，预置移动控件+技能摇杆两条 GameSystemUI 触发器动作，可删）/ `data_sample` / `code_sample` / 专项模板。
- **创建后必须用编辑器打开一次**：编辑器刷新 `src/WasiCoreSDK.props`（SDK 路径，禁止手工固定）、`AGENTS.md`/`CLAUDE.md`、`docs/sdk|api|schemas`、`ai/tools/`、纯客户端调试配置。
- 备用路线：从 `map_templates` 复制模板（仅自动化场景），需改 `project/map_settings.json` 的 ProjectName/MapDisplayName，仍需编辑器打开一次。
- code_sample 模板关键文件：`project/map_settings.json` = `"api_version": {"api_version": 2000, "show_name": "wasm"}` + `"TriggerEditorMode": "V3"`；`libs.json` 登记 GameSystemUI 依赖；`setup-ai-context.ps1` 由编辑器自动跑（同步 SDK docs/api 进项目 + 生成 AI 上下文）。

## 2. 与 1.0 的关系：完全独立，无混合模式

- 2.0 工程 = `src/`（C# 双端单源 `#if SERVER/CLIENT`）+ `editor/data/`（JSON）+ `scene/`；**没有** `script/`、`ui/script/`。触发编辑器 V3 生成 C#（`src/TriggerGenerated/{client,common,server}.cs`）。
- HybridFlappyBird 的 "Hybrid" = 混合同步策略（Node transform replicated + PropertyHostComponent 声明式属性同步），纯 C#。
- 单个项目二选一；编辑器安装同时携带两套包（api 2000 注册表仍含 script:197、xdeditor:159 等 1.0 包）。

## 3. 1.0 → 2.0 迁移（migrate-1to2 skill，AI 作业手册）

- 官方免责：**AI 辅助迁移无法 100%**；**以 .ts 为迁移基准，别读 lua**（1.0 = ts2lua）。
- 7 步：侦察 1.0 工程（数编条目/触发器规模/场景数）→ 新建 `game_entry` 空项目 → 子系统选策略（A 配置直迁 ini→JSON / B 结果代码化 ts→C#；经验：逻辑走 B，纯数据 A/B 均可）→ 迁数据（unit→GameDataUnit、spell→GameDataAbility，类型名会变）→ 迁触发器（→ C# `Trigger<T>`/`Game.Subscribe`）→ 迁公式（→ C# 委托 `Value = ctx => ...`）→ 迁场景/资源/UI + 手补清单。
- 利好：**场景/地形资产同格式可直接复制**（已实测）；`res/`、`ui/image/`、`ui/spine/` 按原相对路径合并；坐标不要盲目换算（2.0 默认镜头转 90°，可配回 1.0 朝向）。
- 质量门槛：`source-mapped → skeleton → compiled → runtime-smoke-passed → accepted` 五态，双端编译 + 实机截图/日志证据。
- 现实成本：bgd_sce_framework 整套 lua 基建（协议层/事件总线/Buff 系统）在 2.0 无对应物，等于框架重做或放弃。

## 4. 构建链与发布

- csproj：`net9.0`（.NET 9/10），5 配置 `Server-Debug/Server-Release/Client-Debug/Client-Release/Client-Resource`（DefineConstants 注入 SERVER/CLIENT/DEBUG/RESOURCE）；引用 `$(WasiCoreSDKPath)lib/{server,client,shared}/*.dll`；内嵌 CodeGenerator.dll（数编代码生成）+ IFileResourceAnalyzer.dll（Client-Resource 发布资源清单）；BannedApiAnalyzers 限制禁用 API。
- 编译：`dotnet build src/GameEntry.csproj -c Server-Debug` + `-c Client-Debug`（**两端都要编**，`#if` 可见性不同）。
- 产物 = `GameEntry.dll` → `AppBundle\managed\`（服务端）与 `ui\AppBundle\managed\`（客户端）（BuildAndCopy.bat）。注意：`dotnet build` **不会自动复制**，编辑器内点调试才会自动编译+复制。
- **日常开发可不装本机 .NET SDK**——编辑器用随包绿色 SDK 编译（调试按钮 = 保存→生成→编译→部署→Host 一条龙；另有「调试（不编译）」= MCP `debug_start_no_compile`）。
- 发布：发布测试游戏 → 创作者中心邀请码 → PC/Android 对战平台或 **TapTap App 内嵌客户端**（已上线）→ TapTap 审核上架。环境双轴：部署 `alpha`（外网公测，每周一/二更新）/`production`（线上，滞后约两周）× 运行时 `debug/test/formal`，3×3 组合**云数据互相隔离**。
- 客户端二进制兼容：`CapabilityVersion`（截至 2026-08-05 最新=1）+ `Runtime.Supports(feature)` 门禁。

## 5. 2.0 当前限制（04-limitations + migrate 手补清单）

- 缺部分 1.0 小卡片机制（商城已有）；捏人动画重定向需手动触发（或显式填 `GameDataModel.HumanoidSourceAnimations`）。
- 官方在规划：小地图（用 Canvas 自绘或 `MiniMapIcon` 过渡）、UI 场景。
- migrate 手补：场景拼接不支持；UI 编辑器覆盖不完整（代码建控件需主动 `AddToVisualTree()`，整页用 `UI.Page()` 根容器）；数编字段差异（技能动画改指独立 `GameDataAnimation`）；可附加枚举机制重设计。
- 定性：「与 1.0 的低代码体验、功能覆盖、稳定程度仍不完全等价」；2.0 = "代码 + 数据编辑器 + 生成代码 + AI 工具"组合开发模式，不是图形化触发器的简单替换。
- 2.0 独有新能力：Box2D 2D 物理、客户端 3D 物理、Canvas 绘制、类型安全数编、双端代码共享。

## 6. 版本线索

- api_version=2000（show_name "wasm"）就是 2.0；本机 `api_pak_version.json` 顶层键 `"12"/"13"/"2000"`，2000 注册表含 wasicoresdk:18、map_templates:64、wineditor:143、gamesparkcore:10、gamesystemui:14 → **当前编辑器已具备 2.0 能力**，新建项目菜单应可见模板。
- SDK 技术栈 .NET 9.0/10.0 + WASI；星火 2.0 编辑器已发布线上。

## 7. Runtime MCP（2.0 官方运行时调试）

- 架构：客户端进程监听本机 TCP bridge（**127.0.0.1:18765**）；编辑器 MCP 暴露固定工具 `runtime_call_tool` 转发到 bridge；脚本侧工具（`debug.ping`/`ui.snapshot`/`ui.find`/`ui.get_rect`/`ui.click`/`input.pointer`/`input.key`/`input.text`/`debug.capture_screenshot`/`scenegraph.materials`/`material.probe`）**不在** tools/list 里（预期行为）。
- 条件：必须经编辑器调试启动（普通/不编译/纯客户端），`Game.IsDebugTestMode == true`；**正式包/线上不开放**；只连本机客户端。
- 项目可注册自定义工具：`#if CLIENT` + `RuntimeDebugToolRegistry.Register("project.xxx", ...)`。
- fallback：编辑器 MCP 不可达时用项目内 `ai/tools/Invoke-SceRuntimeMcp.ps1` 直连 loopback TCP。
- TriggerMcpHost 与 Runtime MCP 无关。
- **对照我们的 bgd_mcp_bridge**：不同代际不同物——我们的桥是给 1.0 (api 13) 编辑器打的 C# 补丁（HttpListener + lua 事件总线）；Runtime MCP 是 2.0 官方内置。做 2.0 项目时 AI 调试闭环官方自带，不需要 patch。

## 8. 对用户的直接回答

现有 1.0 lua 项目（script-199, api 13）**无法直接用上 2.0 C# 能力**。要用 = 新建 2.0 项目（零成本试玩：建 code_sample 模板项目）或按 migrate-1to2 移植（中大成本，逻辑重写 + bgd 框架无对应物）。2.0 已上线（编辑器线上 + TapTap 内嵌客户端）。
