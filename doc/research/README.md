# 研究知识库（doc/research）

> 本目录是 sce_app_mini-runtime 逆向/脱机研究的成体系知识库，按主题拆分。
> 每份文档开头标注「最后验证日期 + 实证方式」。改代码前按主题先读对应文档。

## 文档索引

| 文档 | 主题 | 何时读 |
| --- | --- | --- |
| [runtimes.md](runtimes.md) | **两套官方运行时**（星火编辑器 vs 星火对战平台）的结构、引擎归属、各 dll/exe 作用 | 涉及引擎二进制、载荷组装、登录/运行链路时 |
| [offline-auth-debug.md](offline-auth-debug.md) | 脱机登录体系：凭证文件格式、TapTap device flow、HTTP 签名、服务地址推导 | 涉及凭证/登录/内部 API 调用时 |
| [scegame-reverse.md](scegame-reverse.md) | 逆向主文档：引擎子系统、控制协议（0xF000 段）、B 模式自托管、载荷 0 依赖下载通道 | 涉及调试 host 控制协议、staging、载荷下载时 |
| [credential-userid.md](credential-userid.md) | **userid/user_name 的来源与获取**（凭证里没有、登录态才有、kid 是 opaque） | 涉及 userid 自动补全、凭证命名、多账号时 |
| [payload-packages.md](payload-packages.md) | 载荷包体系：update-info 契约、包格式（7z/TNND/UPAK）、落位布局、版本注册表合成、基座资产 | 涉及 payload sync、包解包、版本跟随、字体/资源缺失时 |

## 研究方法论（贯穿所有文档）

1. **实证优先**：一切结论以抓包/日志/二进制实证为准，lua 源码只作线索（源码里的端口/地址常是旧域名遗留，以实测为准）。
2. **区分两套运行时**：编辑器（version-\<api\>/SCE + sceengine.dll）与对战平台（Win/scegame）结构相近但引擎不同——混淆它们会导致「hook 错栈」「抓不到明文」这类假阴性。
3. **二进制考古**：sceengine.dll/scegame 的字符串（`strings`）+ 导入/导出表（`examples/pe_imports.rs`/`pe_exports.rs`）+ RIP-xref（`find_xref`）+ 反汇编（`disasm_at`）是定位 native 行为的四件套。
4. **抓包分层**：ws2_32（`frida_capture`）抓 socket 层（控制协议是明文 TCP 可直抓）；TLS 明文要在「SSL_read/write 边界」抓（见 payload-packages.md 与 credential-userid.md 的 WSS 章节）。

## 工具集对应（examples/，全 Rust）

| 工具 | 服务的主题 |
| --- | --- |
| `frida_capture` / `entrance_login_capture` | 抓包（socket 层 / SSL 明文层） |
| `capture_parse` | 抓包 jsonl 解析（控制协议切帧/wire 解码） |
| `restore_game` | 加密包还原（TNND/7z/UPAK/KTX） |
| `proto_extract` | protobuf descriptor 提取（结论：官方手写 wire 无 descriptor） |
| `pe_imports` / `pe_exports` | PE 导入/导出表（定位 TLS/网络栈归属） |
| `find_xref` / `disasm_at` | 字符串 xref / 反汇编（native 行为定位） |
| `decode_kid` | 凭证 token 的 kid 段解码（结论：opaque 随机字节） |
