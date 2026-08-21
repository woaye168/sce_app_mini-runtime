# 凭证、userid 与登录态：来源与获取

> 最后验证：2026-08-21（凭证文件精读 + kid 解码实证 + 登录链 spawn 实证 + 在线端点探测）
> 结论先行：**凭证文件里没有 userid**；userid/user_name 只在「登录成功后的内存态」里有，需经一次真实登录拿到。

## 1. 凭证文件（user_info-\<env\>.json）字段——没有 userid

实证（`D:\sce_online\User\user_info-editor-pd.spark.xd.com.json` 全字段）：

| 字段 | 含义 | 备注 |
| --- | --- | --- |
| `access_token` | TapTap OAuth access_token | 很长 |
| `guest_id` | 游客 id（uuid） | 登录后仍保留 |
| `login` | **登录状态标志（0/1），不是 userid** | ⚠️ 极易误判——它只是「是否已登录」 |
| `login_token` / `login_token_secret` | **HTTP 签名对**（内部 API 用） | 长期有效，注释「不会序列化」但实证有落盘 |
| `login_type` | 授权方式（public=授权了用户信息，空=静默登录） | |
| `token` | `mac_key$kid`（TapTap token） | kid 段见下 |
| `token_type` | 11=编辑器TapTap / 13=手机TapTap / 14=安卓容器 / 999=游客 | `token_valid()` = token 非空且 type∈[11,14] |
| `version` | 凭证格式版本（=1） | |

## 2. kid 是 opaque 随机字节，不是 protobuf（0.2.1 探针实证）

`token = mac_key + "$" + kid`，kid 是 base64url。解码后是 ~30 字节随机字节（实测 `d7 f5 3d 20 ...`），**不含可解析的 protobuf 结构，更不含 userid**。早期「kid 是 protobuf、userid 在 f1」的假设已用 `examples/probe_userid.rs`（已删）证伪。**不要从 kid 推 userid**。

## 3. userid/user_name 的真实来源：entrance 登录响应

lua 侧契约（client_base-78 `base/account.lua`）：

```
lobby.request_token_login(token_type, token)   # native 长连接（WSS）登录
→ lobby_events.on_login_response(error_code, ...)  # native 回调
→ dispatch('登录', ...) → account.on_login_result(
     error_code, user_id, login_id, user_name, login_way, tk, tk2, login_token, login_token_secret, error_desc)
```

- **user_id 是登录响应的第 2 个位置参数**（string）。成功时存进内存 `account.latest_login_info = { user_id, login_id, user_name, login_way, hash_token, hash_secret }`。
- **它不落盘**——凭证文件不含 userid，重启后需重新登录才有。
- 编辑器界面显示「userid + TapTap 昵称」就是这个内存态（`lobby.get_login_account_info()` / `lobby.get_user_id()`）。

## 4. 脱机获取 userid 的正确方式（不依赖编辑器）

**用我们自己的 scegame 跑一次真实登录，登录成功后从 lua 层读出 userid 写结果文件**：

1. spawn 脱机 scegame（win 包引擎即可，登录能力与编辑器一致），带 `-env=game -server=<env> -use_local_res -no_update`，凭证预置到 `runtime/User/user_info-<env>.json`。
2. 引擎启动会走 account.login → entrance 登录（tester 的 startup/entrance 链）。
3. 登录回调里 `account.latest_login_info.user_id` 就是真值——**已实证**：我们的 spawn 日志出现 `GamePlayOnline request login, userid: 38672742`（即登录成功并拿到 userid）。
4. 在 lua 侧（或读它的登录日志/内存）把 user_id/user_name 落到一个结果文件，host 进程读回即可。

> 这是「与编辑器同源码的脱机版本」的正解——全程不碰编辑器，用脱机 scegame 自己的登录链。

## 5. WSS 明文抓取（为什么之前抓不到 + 怎么抓）

- entrance 登录走 **WSS**（`entrance-new-pd.tapsce.cn:443`，TLS 加密）。ws2_32 层（`frida_capture`）只能抓到密文。
- 要拿明文必须 hook TLS 边界 `SSL_read`/`SSL_write`：
  - **tester（win 包 scegame）**：TLS 走 `gmesdk.dll` → 导入 `libgmessl-1_1-x64.dll` 的 `SSL_read/write` → hook libgmessl 导出即可（`examples/entrance_login_capture.rs` 的挂法）。
  - **编辑器（sceengine.dll）**：TLS **静态链接**（有 `SSL_CTX_new`/`wss://` 字符串，但 3772 个导出里无 `SSL_*`），hook 导出表命中不了——需 hook 内部函数（find_xref 定位 `SSL_read` 字符串引用点反推函数地址）。
- **假阴性教训**：曾 hook tester 的 libgmessl 去抓「编辑器链路」，装上 hook 却零命中——根因是两套运行时混淆（见 runtimes.md §5）。

## 6. 探测过的死路（别再试）

- 在线 user-info HTTP 端点：`updater/publisher/login` 服务的 `/api/v1/user-info|me|account|check-token|verify-token` 等全部 404——**官方没有暴露「凭 token 查 userid」的 HTTP API**。
- 从凭证 `login` 字段读 userid：那是登录状态 0/1，不是 userid。
- 从 kid 解 protobuf：kid 是 opaque。

## 7. 工具

- `examples/decode_kid.rs`：解 kid 段（验证 opaque 用）。
- `examples/entrance_login_capture.rs`：WSS 明文截获（libgmessl SSL_read/write hook）。
- `auth login`（CLI）：扫码自登录，落盘完整凭证（含 login_token/secret 签名对）。
