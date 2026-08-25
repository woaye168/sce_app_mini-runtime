# 云变量协议完整逆向：Entrance 帧格式 + ScoreArchive 消息结构（已双向实证）

> 研究日期：2026-08-23 | 状态：双向帧全部捕获解码；自建客户端 PoC 进行中
> 前置：cloudvar-01（Lua 链）/ cloudvar-02（静态补遗）/ cloudvar-03（消息 ID + 抓包环境）
> 数据件：test/temp/ssl_sniff_probe9.jsonl（全帧）、decode_entrance.py（解码器）

## 0. 一句话结论

云变量 = Entrance WSS（`wss://entrance-new-pd.tapsce.cn:443`，libhv WebSocketClient，URL 仅 `wss://%s` 无路径）上的 **Entrance 信封 proto（f1=msgid, f2=body）**；云变量消息 msgid=**0xA000**，body = `CEProto::ScoreArchive::Msg`（f1=发送者名, f2=target_map, f3=子消息类型, f4=子消息体, f5=空, f6=0/请求序号）。帧内容为**手写 protobuf wire、无加密无压缩**（TLS 由传输层负责），已双向完整捕获解码——**自建客户端直读直写在协议层面完全可行**（登录帧含 token，可重放）。

## 1. hook 点（scegame BuildPCBox v152，RVA 基于 image base 0x140000000）

| 点 | RVA | 说明 |
| --- | --- | --- |
| Entrance 发送函数入口 | 0x1e87be0 | 签名 `send(rcx=conn, edx=msgid, r8=frame_ptr, r9d=frame_len)`——frame 即 ScoreArchive::Msg 序列化字节（不含 msgid） |
| "Send message to entrance" 日志 xref | VA 0x141e87ecf | 函数内日志点（定位用） |
| "Receive entrance message" 日志点 | RVA 0x1e85f59 | 此处 msgid=[rsp+0x50]；**完整接收帧指针=[rbp-0x18]**（含信封 f1=msgid echo + f2=body，自描述 varint 长度） |

定位方法：find_xref 找日志格式串 → 反汇编回溯函数入口/栈帧。响应帧指针位置的发现法：在日志点 dump rsp/rbp 窗口内所有疑似指针，跟随读内存搜已知内容（我们写入的 key 字符串）。

## 2. 传输层

- libhv `WebSocketClient`（wss://%s，无路径 → 默认 `/`）；TLS 在 libhv 静态 OpenSSL 内（libgmessl hook 抓不到；ws2_32 层是密文）。
- 编辑器 BuildPC（sceengine.dll）同族代码同协议（LuaScore/Entrance 字符串一致），hook 点 RVA 不同需重定位。
- **原生消息 ID 免逆向渠道：`logs/lobby/lobby-*.log` 逐条打印收发 msgid 与 buffer size**。

## 3. Entrance 信封与消息 ID

wire 消息（WS binary frame）= proto：`f1 varint msgid` + `f2 bytes body`（接收帧实证 `08 80c002 12 2f...` = f1 0xA000）。

| msgid | 方向 | 含义 | body 结构（已解码） |
| --- | --- | --- | --- |
| 0x0001 | → | 登录请求 | {f1 str 'default', f2 空, f4 varint 0x1000040, f6 varint token_type(11), f7 str login_token(385B), f19 varint 1, f20 空} |
| 0x0002 | ← | 登录响应 | （lobby 日志实证存在） |
| 0x0010 | ← | server stop notify | 连接建立即收 |
| 0x0011 | → | 进局通知（28B） | |
| 0x3040/0x3060/0x3062/0x3063/0x3068/0x3069/0x3083 | ⇄ | 大厅局会话管理 | 部分解码：0x3040 body `08914e...`（f1 varint 序列号?） |
| 0x6001 | → | 统计上报（~30s 批量） | {f1 str 类别(user_behavior/user_hardware/...), 重复 f3 {f2 k, f3 v}}——纯遥测，自建客户端可不发 |
| 0x7001 | ← | 登录后配置下发 | |
| **0xA000** | ⇄ | **ScoreArchive 云变量** | 见 §4/§5 |

## 4. ScoreArchive::Msg 请求结构（f4 子消息体逐类型解码）

顶层：`f1 str source`（lobby 态='startup'，游戏态预计=地图名）+ `f2 str target_map`（'ClientReadonlyMap'/'ClientReadWriteMap'/空）+ `f3 varint 子类型` + `f4 bytes 子消息体` + `f5 bytes 空` + `f6 varint 0`。

| f3 | 子消息 | f4 body 字段 |
| --- | --- | --- |
| 2 | **Commit** | repeated f1 = 操作项 {f1 str key, f2 varint op(3=score_seti, 7=score_sets), f3 varint user_id, f5 varint ivalue / f6 str svalue}；f2 str = commit 描述 |
| 4 | **ScoreInit**（读） | f1 = user_id（lua 传 nil/number 时为 varint 原始字节 e6b2b812=38672742；官方 lobby 某调用为 str '-'，f2 为 user 字符串 '38672742'——两种形态都见过，待统一）；f2 str = key（可 repeated 多 key） |
| 10 | **MoneyInit** | {f1 varint user_id} |
| 20 | **NameSearch** | {f1 str key, f2 str name_substr} |
| 48 | **QueryList**（list_query） | {f1 varint user_id, f2 str key, f3 varint limit} |

待补（官方测试脚本 xdeditor test/scorearchive.lua 里存在但探针未触发）：get_rank_list/get_user_rank/get_rank_total/message_*/query_item/item_add/item_use/name_new/client_score_init 等子类型号。

## 5. ScoreArchive 响应结构

信封 f1=0xA000，f2 body = 响应 Msg：`f3 varint 响应子类型` + `f4 bytes body` + `f6 varint 请求序号`（0x6a8b0ac9c19c7d8x 单调递增，即 __scorearchive_message_key）+ `f7 bytes 空`。

| f3 | 含义 | body |
| --- | --- | --- |
| 100 | **Result（错误/通用确认）** | {f1 varint error_code, f2 str reason}——13=Nopermission、1146=TableNotFound（MySQL 错误码直通）、0=commit 成功 |
| 5 | **ScoreInitRes**（读响应） | f1 repeated 条目 {f1 varint user_id, f2 bytes {f2 bytes {f1 str key, f2 varint ivalue}}, f6 varint 时间戳/id}——嵌套较深，详见 decode 输出 |

## 6. 直读直写自建客户端设计（PoC 路线）

```
Python websockets → wss://entrance-new-pd.tapsce.cn/
  → 收 0x0010（server stop notify，忽略）
  → 发 0x0001 登录：body = f1'default' + f4 0x1000040 + f6 11 + f7 <login_token> + f19 1
     （token 取自 User/user_info-<env>.json 的 login_token 字段）
  → 收 0x0002 登录响应（验证成功）
  → 发 0xA000：body = Msg{f1 'poc', f2 'ClientReadWriteMap', f3 4, f4 ScoreInit{user_id,key}}
  → 收 0xA000 响应（f3=5 数据 / f3=100 错误）
  → 写：f3=2 Commit{ops, desc}
```

已知风险/待验证：① 是否需先发 0x6001 遥测（大概率不用）；② token 有效期与绑定（token_type 11 = 编辑器凭证；tester 凭证 type 可能不同）；③ 读写次数限制在服务端按什么维度计（若按局/连接计，自建连接可能绕开——待实测）；④ 游戏态 source 名与 target_map 的合法值集合。

## 7. 本次新增的坑（知识沉淀）

- python 长输出被工具宿主截断时，jsonl 文件是权威数据源（print 仅观察用）。
- frida spawn 的进程在脚本退出后不会自动退出——抓完必须手动 Stop-Process scegame（防多实例重复登录）。
- WSARecv 的 onLeave 里 arg3 指针可读实际字节数；但 IOCP overlapped 场景 onLeave 时机不可靠——native 函数 hook（协议层）比 socket 层 hook 更干净。
- mini-runtime runtime 载荷是研究注入的理想沙盒：TNND 散文件解密→改→明文写回即可跑；但必须 -no_update（自更新会整包重装冲掉改动，且不保留旁路备份文件）。
