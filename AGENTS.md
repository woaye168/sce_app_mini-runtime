# AGENTS.md — sce_app_mini-runtime

> 本文件为 AI 助手提供项目上下文。修改代码前请先阅读。

## 项目定位

脱机运行时（sce_app_mini-runtime）：独立的 egui 桌面应用。核心能力是 **B 模式（本地工程调试）完全脱机运行**——不依赖星火编辑器/对战平台，自带载荷同步（官方 update-info 通道下载）、staging 生成、控制协议客户端（上传工程、起局、host 日志）、游戏进程拉起与截屏自验。**运行时按项目 api_version 可切换**（core/runtimes.rs）：默认「星火编辑器-api\<N\> 运行时」（引擎 = wineditor@api 包解出的 version-\<N\>/SCE 壳 + sceengine.dll），另保留「星火对战平台 测试/正式环境运行时」（scegame 一体引擎，0.2.0 既有链路）。通过宿主 [bgd_sce_tools](https://github.com/woaye168/bgd_sce_tools) 的「应用市场」安装分发，宿主启动时传 `--project-path <项目根>`。应用单实例；`--background` 静默驻留；`--quit` 优雅退出；窗口 X = 正常退出。

## 技术栈与规范

- Rust 2021；eframe/egui 0.29；CLI 由 bgd_appsdk 统一入口托管（`--project-path` / `--background` / `--quit` / `notify`），本仓库不引入 clap
- **bgd_appsdk**（crates.io 公开包 `bgd_appsdk = "0.2"`，仓库 [bgd_sce_appsdk](https://github.com/woaye168/bgd_sce_appsdk)）：单实例/看守线程/日志/应用配置/**通用窗口壳 AppShell** 等公共基建，禁止在本仓库重复实现（UI 经 `ShellApp` trait 注册标签页即可）
- **模块拆分**：单文件接近 500 行必须按职责拆分
- **工具语言统一 Rust**：examples/ 研究工具一律 Rust（cargo examples，专用依赖进 `[dev-dependencies]`），禁止混用 Python；一次性探针用完即删，结论归档 `doc/research/`

## 目录结构

```
src/main.rs            # 入口（bgd_appsdk 统一入口 + 业务 CLI 分发）+ 应用状态 + 壳实现
src/core/mod.rs        # 模块声明
src/core/auth.rs       # 凭证库（多账号 list/use/import；make_label 命名 {userid}_{时间}_{env}_{type}）
src/core/login.rs      # 扫码登录
src/core/login_state.rs # 登录态获取（起脱机客户端真实登录，从 logs/game 抓 userid；注入凭证强制 login=1）
src/core/verify.rs     # 凭证校验
src/core/runtimes.rs   # 运行时切换架子（RuntimeKind：编辑器-api/对战平台测试/正式；引擎包/spawn 目标/env 域）
src/core/host.rs       # 调试 host 控制协议（EditorLogin/上传/起局/host 日志，手写 protobuf wire）
src/core/local_host.rs # 自建 host（中继模式）：TCP 控制面中继 + UDP KCP NAT（会话端口=控制端口+50）+ 全流量 capture
src/core/kcp_server.rs  # KCP 会话面服务端（CE1 握手 + KCP 服务端 + 3B 流分帧，单会话，§13.1-13.3）
src/core/host_server.rs # 自研 host 控制面 TCP 服务端（0xF000 段：登录/逐文件 0xF010 ack/起局/0xF00C 日志/0xF01B teardown）
src/core/game_host.rs   # 真本地 host 编排（R3 壳：登录应答+初始化消息群+tick/时钟/探测应答；R4：lua 编排脑接线——起局建脑/0x7006 路由/0x7008 出站/帧泵/广播挂起补发）
src/core/lua_host.rs    # lua 宿主（0.5.0 R4）：mlua lua54 vendored 内嵌 VM + shim 面 + 自研 require 加载链（sanitize_lua ≥0x80 标识符清洗）
src/core/cmsg_pack.rs   # cmsg_pack（msgpack 变体）pack/unpack + lua 值互转（零依赖）
src/core/host_templates.rs # AUTO-GENERATED：官方 h2c 消息序列模板（kcp_capture_parse `export` 从基准 capture 提取）
src/core/zcompress.rs  # ZCompress 复刻（h2c 传输层压缩，纯算法零依赖；格式权威 = doc/research/scegame-reverse.md §13.8）
src/core/payload.rs    # 载荷同步：update-info + OSS 下载 + TNND/UPAK 落位 + 注册表合成 + 基座资产
src/core/staging.rs    # 调试 staging 生成（白名单拷贝 + ui/script/main.lua 包装）
src/core/debug.rs      # B 模式编排：assign_host → 上传 → 起局 → spawn（CreateProcessW 防管道继承）
src/core/capture.rs    # 游戏窗口截屏（WGC，自验用）
src/core/locate.rs     # 官方目录推导（项目 tsconfig typeRoots 链）
src/ui/{auth,debug,settings}.rs  # 标签页（impl App 分散定义）
examples/              # 研究/维护工具（全部 Rust + 两个 ps1；见下方「工具集」）
test/temp/             # 原始 dump / 一次性独立小 crate（引用文档须注明路径，期末清理无引用价值的堆积）
doc/requirements/      # 版本需求文档
doc/research/          # 逆向研究成果（scegame-reverse.md 为协议/载荷主文档）
app.json               # 应用市场静态元数据（不含版本；CI 合成 app-release.json）
.github/workflows/release.yml  # tag 触发构建发布（base_assets.7z 从前一 release 继承）
```

## 核心机制（改代码前必读）

- **运行时切换（runtimes.rs）**：`RuntimeKind = EditorApi(api) | TesterTest | TesterProd`，决定引擎包（wineditor/win）、update-info variation（windows_editor/client）、env 域、spawn 目标（`version-<api>/SCE` / `scegame.exe`）。默认 = 项目 map_settings.json 的 api_version 对应的编辑器运行时；GUI 调试页下拉/CLI `--kind editor-<api>|tester_test|tester_prod` 可切。两套运行时结构相同（控制协议/staging/上传链不变，仅引擎与 spawn 目标不同）。详见 doc/research/runtimes.md。
- **编辑器引擎自举**：wineditor 是普通 update-info 包，版本号取 `api_pak_version.json[<api>].wineditor`（api13→147）；下载 OSS `windows_game.7z` 解出 `version-<api>/`（SCE 壳 + sceengine.dll + 一套 dll）+ 基座 res。**不要再从本机编辑器安装目录复制引擎**。
- **依赖库版本权威 = api_pak_version.json[\<api\>] 注册表**（不是 update-info 返回的最新版），落位 `_m/maps/`、`_m/maps/script_libs/`、`_m/maps/user_libs/`（与编辑器一致）。
- **启动调试自动补载荷**：GUI/工作线程在 spawn 前检查 `engine_ready`，未就绪自动 `payload sync`（进度上状态栏）。
- **userid 自动获取**：凭证文件无 userid（login 字段是 0/1 登录状态）；`auth refresh <凭证名>` / 凭证页「刷新登录态」起脱机客户端真实登录，从 `logs/game/` 抓 `GamePlayOnline request login, userid:` 行。**注入凭证必须强制 login=1**（大厅 after_update 的自动登录闸门），否则僵在登录按钮超时。详见 doc/research/credential-userid.md §4.1。
- **控制协议**（详见 doc/research/scegame-reverse.md §8）：TCP 直连 debug host；帧 = u32 LE 总长 + 0x00 + envelope；0xF000 段消息。大文件上传 = **0xF004 空声明（无 f3）→ 0xF008 分块（101400B）→ 0xF00A**；上传路径**全小写**（host 是 Linux 大小写敏感）。
- **载荷 0 依赖**：`payload sync` 走官方 `update-info`（query-string POST 空 body 免签名）+ OSS 公共读下载；每次 sync 实时查询 = 自动跟随星火服务器侧版本更新；部分包 TNND 加密（XOR CREATEEASY）需识别解密；`_m` 注册表包需 UPAK 解散文件 + 合成 api_pak_version.json（items 全量登记）。
- **基座资产**（update-info 不分发：ui/font/regular 字体族、fonts、characters、effect）：本机编辑器兜底复制（tsconfig 推导）→ 否则下载本仓库 release 的 base_assets.7z（**仓库私有，走 GitHub API + token**：env `MINI_RUNTIME_GITHUB_TOKEN` → 凭据管理器 `bgd_sce_tools/github_token`；env `MINI_RUNTIME_BASE_ASSETS_URL` 可覆盖为公开直链）。重新打包：`examples/pack_base_assets.ps1` 后 `gh release upload <tag> --clobber`。
- **spawn 防卡死**：游戏进程必须用裸 `CreateProcessW(bInheritHandles=FALSE)` 拉起——std Command 会让游戏继承调用方管道句柄，导致管道对端工具等 EOF 卡死到游戏关窗。
- **B 模式永远带 `-no_update`**：否则 scegame 会自更新并清掉组装好的载荷。
- **自建 host（中继模式，local_host.rs）**：`debug start --host cloud|local`（CLI）与调试页「host 模式」下拉（GUI）可选，默认云端直连。local = 127.0.0.1:5003 中继（编辑器「调试(本地服务器)」同口接入）：TCP 控制面 EditorLogin 拦截换真 token 后帧级透传到 assign_host 云端；UDP KCP NAT 转发。**KCP 会话端口 = 控制端口 + 50（引擎硬编码，5003→5053 / 20770→20820）**，UDP 必须双端口监听，否则客户端 KCP 建连失败、lua VM 不起。全流量落 `<runtime>/User/host_capture-*.jsonl`（KCP 会话抓包平台：c2h 明文 protobuf/cmsg_pack 可直读，h2c=ZCompress 压缩无加密，见 doc/research/scegame-reverse.md §13）。assign/云连接失败必须回 0xF001 result≠0（防编辑器 update_host co.call 悬挂）。详见 doc/research/self-host.md。
- **真本地 host（0.5.0 R3+R4，game_host.rs + lua_host.rs）**：`debug start --host shell` / `host start --shell`（PIE 同口）。R3 壳 = 登录/初始化消息群模板 + tick/时钟/探测应答；R4 = mlua lua54 内嵌 VM 跑项目服务端 lua（决策：内嵌 + 磁盘现读零内嵌），0x7006→`base.ui.proto[type]` 路由、`game:ui`/`player:ui`→0x7008 出站、事件泵（游戏-帧 50ms/玩家-连入/断线/按键内建通道 __client_key_down/up）、广播无就绪会话挂起补发。shim 面/踩坑全集 = self-host.md §9.6（**base.clock() 是毫秒**；mlua `Vec<Value>` 传参吞前导 nil 必须 `Variadic::from_iter`；引擎 lua 放行 ≥0x80 标识符需 sanitize_lua 清洗）。

## 工具集（examples/，全部 cargo examples）

| 工具 | 用途 |
| --- | --- |
| `frida_capture` | Frida 抓 ws2_32 收发（注入脚本 ws_hook.js 内嵌）→ jsonl |
| `entrance_sniff` | Entrance 协议帧明文 dump（hook 发送函数+接收日志点，云变量 0xA000 全双向；RVA 可用环境变量覆盖） |
| `entrance_client` | Entrance 直连客户端：绕过引擎直读直写云变量（read/seti/sets/list；协议见 doc/research/lowlevel/cloudvar-04~06） |
| `capture_parse` | jsonl 分析：`frames`（按消息逐帧 dump）/ `dump`（时间线递归 wire 解码）/ `blocks`（大文件分块序列） |
| `kcp_capture_parse` | 中继 KCP 抓包全解码（host_capture-*.jsonl）：`stats` / `flow` / `decode`（双向时间线回放，h2c 经 zcompress 解码 + cmsg_pack 直出 Req_*/Sync_*）/ `msgs`（聚合表）/ `dump <conv> <type> [n]`（按类型全量 dump）。逐段解析合并数据报。零外部依赖，缺 libclang 时可 rustc 直编 |
| `restore_game` | 加密包一键还原：TNND → 7z → UPAK → 伪 KTX 图片转 PNG（BC1/2/3/7） |
| `proto_extract` | 从 protobuf C++ 二进制提取内嵌 FileDescriptorProto |
| `find_xref` / `disasm_at` | PE 字符串 RIP-xref 查找 / 线性反汇编（PE 解析手写在 examples/util） |
| `lua_api_dump` | luaL_Reg 注册表导出 + capstone 签名推断（锚点字符串→qword 引用→走表；thunk 跟进 + 跳板 stub→IAT→lua54 取参分析）。用法 `lua_api_dump <PE> <锚点>`，如 common 表锚 `get_platform`、io 表锚 `read_pak_entries` |
| `pe_imports` / `pe_exports` | PE 导入/导出表 dump（定位 TLS 栈归属等） |
| `entrance_login_capture` | WSS 明文截获（spawn 挂起 → hook libgmessl SSL_read/write + ws2_32 connect 对照）→ jsonl |
| `probe_wineditor` / `probe_libs` | wineditor 可下载性验证 / 依赖库三 variation 对比 |
| `decode_kid` | 解码凭证 token 的 kid 段 |
| `assemble_runtime.ps1` | 本机官方目录组装载荷（payload sync 的本机兜底） |
| `pack_base_assets.ps1` | 从本机编辑器目录重打 base_assets.7z |

运行：`cargo run --example <名> -- <参数>`（各工具有用法注释）。

## 使用方约定（改代码前必读）

- 应用只需实现 `ShellApp` 并调 `bgd_appsdk::app::run`——公共逻辑（CLI 分发、单实例、看守线程、项目解析、窗口壳）全托管，禁止自己再写一套。
- 新增标签页 = `src/ui/` 加页面文件 + `ui/mod.rs` 加 mod 声明 + main.rs 的 `TABS` / `ui_tab` 分发各加一行。
- 宿主协议：`--background` 静默驻留、`--quit` 优雅退出、`notify key=value` 解耦通知。
- **命名契约**：宿主按 `<id>.exe` 落盘，单实例/信号前缀一律由 appsdk 按 exe 名推导（`app::default_si_prefix`），应用方禁止硬编码（`AppOptions.si_prefix` 保持 `None`）。
- **关键结论**：egui 窗口隐藏时事件循环休眠，任何信号处理不能放 UI update，也不能依赖 ViewportCommand——这类需求一律提到 bgd_appsdk 看守线程里实现。

## 构建与发布

```bash
cargo check && cargo check --examples   # 主程序 + 工具集
cargo build --release
git tag v0.x.0 && git push origin v0.x.0   # CI 注入版本号 → 构建 → 上传 exe + app-release.json + base_assets.7z
```

- 版本号唯一来源是 git tag（Cargo.toml 固定 `0.0.0-dev`，CI 构建时注入）。
- **base_assets.7z 不入 git**（19MB 二进制）：首次发版手动 `gh release upload <tag> examples/base_assets.7z`，之后 CI 自动从前一 release 继承；内容变化时用 `pack_base_assets.ps1` 重打后 `--clobber` 覆盖上传。
- **本应用无自我更新**：版本更新统一由宿主 bgd_sce_tools 应用市场负责（registry 在 bgd_sce_appsdk，元数据来自本仓库 CI 合成的 app-release.json）。

## 修改守则

- 公共基建（单实例/看守线程/日志/配置/窗口壳）禁止在本仓库重复实现；缺能力先改 bgd_appsdk 并升版本。
- 协议/载荷相关改动必须同步更新 `doc/research/scegame-reverse.md`；用户可见行为改动同步本文件。
- 单文件接近 500 行必须按职责拆分（页面进 `src/ui/`，非 UI 逻辑进 `src/core/`）。
- 提交规范：Conventional Commits（`feat: / fix: / docs: / ci: / refactor: / chore:` 前缀，Release notes 依赖）。
