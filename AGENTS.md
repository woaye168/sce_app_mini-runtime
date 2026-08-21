# AGENTS.md — sce_app_mini-runtime

> 本文件为 AI 助手提供项目上下文。修改代码前请先阅读。

## 项目定位

脱机运行时（sce_app_mini-runtime）：独立的 egui 桌面应用。核心能力是 **B 模式（本地工程调试）完全脱机运行**——不依赖星火编辑器/对战平台，自带载荷同步（官方 update-info 通道下载）、staging 生成、控制协议客户端（上传工程、起局、host 日志）、游戏进程拉起与截屏自验。通过宿主 [bgd_sce_tools](https://github.com/woaye168/bgd_sce_tools) 的「应用市场」安装分发，宿主启动时传 `--project-path <项目根>`。应用单实例；`--background` 静默驻留；`--quit` 优雅退出；窗口 X = 正常退出。

## 技术栈与规范

- Rust 2021；eframe/egui 0.29；CLI 由 bgd_appsdk 统一入口托管（`--project-path` / `--background` / `--quit` / `notify`），本仓库不引入 clap
- **bgd_appsdk**（crates.io 公开包 `bgd_appsdk = "0.2"`，仓库 [bgd_sce_appsdk](https://github.com/woaye168/bgd_sce_appsdk)）：单实例/看守线程/日志/应用配置/**通用窗口壳 AppShell** 等公共基建，禁止在本仓库重复实现（UI 经 `ShellApp` trait 注册标签页即可）
- **模块拆分**：单文件接近 500 行必须按职责拆分
- **工具语言统一 Rust**：examples/ 研究工具一律 Rust（cargo examples，专用依赖进 `[dev-dependencies]`），禁止混用 Python；一次性探针用完即删，结论归档 `doc/research/`

## 目录结构

```
src/main.rs            # 入口（bgd_appsdk 统一入口 + 业务 CLI 分发）+ 应用状态 + 壳实现
src/core/mod.rs        # 模块声明
src/core/auth.rs       # 凭证库（多账号 list/use/import）
src/core/login.rs      # 扫码登录
src/core/verify.rs     # 凭证校验
src/core/host.rs       # 调试 host 控制协议（EditorLogin/上传/起局/host 日志，手写 protobuf wire）
src/core/payload.rs    # 载荷同步：update-info + OSS 下载 + TNND/UPAK 落位 + 注册表合成 + 基座资产
src/core/staging.rs    # 调试 staging 生成（白名单拷贝 + ui/script/main.lua 包装）
src/core/debug.rs      # B 模式编排：assign_host → 上传 → 起局 → spawn（CreateProcessW 防管道继承）
src/core/capture.rs    # 游戏窗口截屏（WGC，自验用）
src/core/locate.rs     # 官方目录推导（项目 tsconfig typeRoots 链）
src/ui/{auth,debug,settings}.rs  # 标签页（impl App 分散定义）
examples/              # 研究/维护工具（全部 Rust + 两个 ps1；见下方「工具集」）
doc/requirements/      # 版本需求文档
doc/research/          # 逆向研究成果（scegame-reverse.md 为协议/载荷主文档）
app.json               # 应用市场静态元数据（不含版本；CI 合成 app-release.json）
.github/workflows/release.yml  # tag 触发构建发布（base_assets.7z 从前一 release 继承）
```

## 核心机制（改代码前必读）

- **控制协议**（详见 doc/research/scegame-reverse.md §8）：TCP 直连 debug host；帧 = u32 LE 总长 + 0x00 + envelope；0xF000 段消息。大文件上传 = **0xF004 空声明（无 f3）→ 0xF008 分块（101400B）→ 0xF00A**；上传路径**全小写**（host 是 Linux 大小写敏感）。
- **载荷 0 依赖**：`payload sync` 走官方 `update-info`（query-string POST 空 body 免签名）+ OSS 公共读下载；每次 sync 实时查询 = 自动跟随星火服务器侧版本更新；部分包 TNND 加密（XOR CREATEEASY）需识别解密；`_m` 注册表包需 UPAK 解散文件 + 合成 api_pak_version.json（items 全量登记）。
- **基座资产**（update-info 不分发：ui/font/regular 字体族、fonts、characters、effect）：本机编辑器兜底复制（tsconfig 推导）→ 否则下载 GitHub release 的 base_assets.7z。重新打包：`examples/pack_base_assets.ps1` 后 `gh release upload <tag> --clobber`。
- **spawn 防卡死**：游戏进程必须用裸 `CreateProcessW(bInheritHandles=FALSE)` 拉起——std Command 会让游戏继承调用方管道句柄，导致管道对端工具等 EOF 卡死到游戏关窗。
- **B 模式永远带 `-no_update`**：否则 scegame 会自更新并清掉组装好的载荷。

## 工具集（examples/，全部 cargo examples）

| 工具 | 用途 |
| --- | --- |
| `frida_capture` | Frida 抓 ws2_32 收发（注入脚本 ws_hook.js 内嵌）→ jsonl |
| `capture_parse` | jsonl 分析：`frames`（按消息逐帧 dump）/ `dump`（时间线递归 wire 解码）/ `blocks`（大文件分块序列） |
| `restore_game` | 加密包一键还原：TNND → 7z → UPAK → 伪 KTX 图片转 PNG（BC1/2/3/7） |
| `proto_extract` | 从 protobuf C++ 二进制提取内嵌 FileDescriptorProto |
| `find_xref` / `disasm_at` | PE 字符串 RIP-xref 查找 / 线性反汇编（PE 解析手写在 examples/util） |
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
