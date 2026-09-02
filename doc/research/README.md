# 研究知识库（doc/research）

> 本目录是 sce_app_mini-runtime 逆向/脱机研究的成体系知识库，按主题拆分。
> 每份文档开头标注「最后验证日期 + 实证方式」。改代码前按主题先读对应文档。

## 文档索引

| 文档 | 主题 | 何时读 |
| --- | --- | --- |
| [runtimes.md](runtimes.md) | **两套官方运行时**（星火编辑器 vs 星火对战平台）的结构、引擎归属、各 dll/exe 作用 | 涉及引擎二进制、载荷组装、登录/运行链路时 |
| [offline-auth-debug.md](offline-auth-debug.md) | 脱机登录体系：凭证文件格式、TapTap device flow、HTTP 签名、服务地址推导 | 涉及凭证/登录/内部 API 调用时 |
| [scegame-reverse.md](scegame-reverse.md) | 逆向主文档：引擎子系统、控制协议（0xF000 段）、B 模式自托管、载荷 0 依赖下载通道、**KCP 会话层破解（§13：CE1 握手/端口+50 规律/c2h 明文/cmsg_pack/h2c ZCompress）** | 涉及调试 host 控制协议、游戏会话（KCP）、staging、载荷下载时 |
| [self-host.md](self-host.md) | **自建 host 可行性**：三层组成现状、中继已交付验证（§9）、KCP 会话破解（§10）、server VM oracle（§11）、server 运行时不可得五条证据、架构 A/B/B+/C | 涉及自建/本地 host、调试后端选型时 |
| [credential-userid.md](credential-userid.md) | **userid/user_name 的来源与获取**（凭证里没有、登录态才有、kid 是 opaque） | 涉及 userid 自动补全、凭证命名、多账号时 |
| [payload-packages.md](payload-packages.md) | 载荷包体系：update-info 契约、包格式（7z/TNND/UPAK）、落位布局、版本注册表合成、基座资产 | 涉及 payload sync、包解包、版本跟随、字体/资源缺失时 |
| [map-distribution-chain.md](map-distribution-chain.md) | **地图分发链路**：编辑器上传（调试上传链/发布）→ 双通道构建（server.zip 私有 vs windows_game.7z 公共）→ 客户端下载加载；客户端发布 pak 条目级实证「物理不含服务端逻辑」 | 涉及服务端逻辑来源、发布/下载链路、变体通道时 |
| [cloudvar-lowlevel.md](cloudvar-lowlevel.md) | **云变量底层全解**：sce.s/ScoreArchive 本质、Entrance 0xA000 协议解码、op 码/MessagePack 值编码/签名终表、权限模型、直连客户端与限流、2.0 云数据 API | 涉及云变量读写、entrance_client 直连、权限/限流问题、云数据能力调研时 |
| [render-lowlevel.md](render-lowlevel.md) | **渲染底层全图**：各渲染通道能力矩阵与正确用法（imgui/UIScene/换模/附着/特效直路径）、数编注册链与脚本化、免数编攻坚判死记录（native 注册表逆向）、2.0 WasiCore 渲染面 | 涉及 UI/模型/特效/图集/webview/video 渲染、数编注册、动态资源加载时 |
| [pak-io-native.md](pak-io-native.md) | **线上 pak 内容提取**：LuaIO 双名注册（PascalCase 漏网 isolation）逆向实证、ReadPakEntries/ExtractPakFile 签名与实测、三环境视频绝对路径策略、iOS 白屏排查计划 | 涉及线上读 pak 内容、要真实磁盘绝对路径（视频/外部工具）、io 沙箱绕过时 |
| [common-table.md](common-table.md) | **引擎 common 表全集**（运行时枚举 + 注册表逆向双实证，双名机制/不对称坑/分类清单）；逐函数签名见 [common-table-editor.md](common-table-editor.md) / [common-table-tester.md](common-table-tester.md)（双引擎注册名一致） | 写代码找系统级 API（平台/剪贴板/窗口/性能/渲染开关）时 |
| [webview-bridge.md](webview-bridge.md) | **webview lua↔JS 双向桥全解**：派发链（ui_events→base.ui.map）、三通道（cgui/imgui/base.ui）接法、run_js vs web_import_script、手势事件/console 转发、坑清单（含三端兼容全记录） | 写 webview 交互、JS 双向通信、调试 JS 报错、移动端适配时 |

## 研究方法论（贯穿所有文档）

1. **实证优先**：一切结论以抓包/日志/二进制实证为准，lua 源码只作线索（源码里的端口/地址常是旧域名遗留，以实测为准）。
2. **区分两套运行时**：编辑器（version-\<api\>/SCE + sceengine.dll）与对战平台（Win/scegame）结构相近但引擎不同——混淆它们会导致「hook 错栈」「抓不到明文」这类假阴性。
3. **二进制考古**：sceengine.dll/scegame 的字符串（`strings`）+ 导入/导出表（`examples/pe_imports.rs`/`pe_exports.rs`）+ RIP-xref（`find_xref`）+ 反汇编（`disasm_at`）是定位 native 行为的四件套。
4. **抓包分层**：ws2_32（`frida_capture`）抓 socket 层（控制协议是明文 TCP 可直抓）；TLS 明文要在「SSL_read/write 边界」抓（见 payload-packages.md 与 credential-userid.md 的 WSS 章节）；**KCP 游戏会话用 local_host 中继抓全流量（host_capture-*.jsonl）**；VM 内 hook（on_ui_message 等）= 语义层取证旁路。
5. **VM oracle**：引擎 lua VM 自带 cmsg_pack 等 native 序列化器——已知明文生成（pack）与抓包字节对照，是推二进制格式的低成本路径。

## 工具集对应（examples/，全 Rust）

| 工具 | 服务的主题 |
| --- | --- |
| `frida_capture` / `entrance_login_capture` | 抓包（socket 层 / SSL 明文层） |
| `capture_parse` | 抓包 jsonl 解析（控制协议切帧/wire 解码） |
| `kcp_capture_parse` | KCP 会话抓包全解码（host_capture-*.jsonl，stats/flow/decode/msgs/dump，见 scegame-reverse.md §13） |
| `restore_game` | 加密包还原（TNND/7z/UPAK/KTX） |
| `proto_extract` | protobuf descriptor 提取（结论：官方手写 wire 无 descriptor） |
| `pe_imports` / `pe_exports` | PE 导入/导出表（定位 TLS/网络栈归属） |
| `find_xref` / `disasm_at` | 字符串 xref / 反汇编（native 行为定位） |
| `entrance_sniff` / `entrance_client` | Entrance 帧明文 dump / 云变量直连读写 CLI（见 cloudvar-lowlevel.md） |
| `probes/`（RenderProbe/GameWorldProbe/CloudProbe 等） | 游戏内 lua 探针套件（见 render-lowlevel.md） |
| `decode_kid` | 凭证 token 的 kid 段解码（结论：opaque 随机字节） |
