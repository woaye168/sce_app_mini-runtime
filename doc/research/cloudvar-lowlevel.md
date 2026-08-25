# 云变量（sce.s / ScoreArchive）底层协议全解：Entrance 通道、直连与权限模型

> **主题一句话**：星火 1.0 云变量（`sce.s`/`score`）的本质 = Entrance 长连接上 msgid **0xA000** 的 `CEProto::ScoreArchive::Msg` 手写 protobuf wire 协议，可完全脱离引擎自建客户端直读直写；写操作全环境放行，查询类操作按地图授权。
> **来源**：合并自 `doc/research_task/lowlevel/cloudvar-01~10` + `wasicore-03-clouddata-api.md`。
> **最后验证日期**：2026-08-25。
> **实证方式**：Frida 抓包（native 发送/接收 hook）+ 二进制逆向（字符串/xref/反汇编/PE 导入导出）+ 直连 PoC（Rust/Python 自建 Entrance 客户端）+ 真 tester 局实测（线上 test 环境 p_55a3）。

---

## 1. 总览：sce.s 是什么，数据流走哪里

- `sce.s` **不是 Lua 封装**，是引擎 C++（`LuaScore.cpp`）直接注册的 native 全局表（注册名 `__LUA_SCORE__`），全库 grep 无 `sce.s = ` 赋值点。
- 所有读写经**平台 Entrance 长连接**（`EntranceHeader` + 数字消息 ID + protobuf `CEProto::ScoreArchive::Msg`）发往内部称 "ScoreServer" 的服务。**既不走游戏局 KCP，也不走 HTTP**。HTTP 仅用于管理面（编辑器开通/查询云变量空间 `https://publisher-<env>.spark.xd.com:9000/api/map/set-package-score-name|get-package-score-name`，错误码 3="SQL failed"）。
- 存储后端 = **MySQL 实锤**：物品过期字段用 MySQL 语法拼接（`"9999-12-31 23:59:59"`、`NOW() + interval second`）；`name_search` 返回 error **1146 TableNotFound**（MySQL 原码直通）。
- 游戏进程（含局内 host）内置相同链路（scegame-tester strings 有同一套 LuaScore.cpp / Entrance 代码）——服务端 lua 的 `score.*` 也走本进程 Entrance 连接，不经客户端转发。
- **Lua 层没有暴露"向 Entrance 发自定义消息"的通用函数**；复用该连接须 native hook 或自建连接仿协议。
- 双端 API 面（官方文档镜像 = `test_res002/.bgd/libs/types/class_cloud.d.lua`）：
  - **客户端 `sce.s.*`**（异步 {ok,error,timeout}）：get_commit/score_init/money_init/name_search/get_rank_list/get_user_rank/get_rank_total/message_query/message_send/message_modify_read/message_delete/list_query/query_item/client_score_init + readonly_map/readwrite_map。
  - **服务端 `score.*`**（协程 RPC，sync 返回 error_code/data/err_msg）：get/money_get/list_query/list_query_by_uuid/get_commit(committer: set/add/addi/money_add/money_cost/list_add/list_modify/list_delete/name_new/world_data_set/world_list_add)/message_*/name_search/subscribe_channel/unsubscribe_channel/subscribe_message/publish_message/world_data_get/test_cloud_value。
  - 官方限额（文档口径）：每局每分钟 300 读 + 300 写；单次 ≤32M、每分钟写 ≤48M；客户端序列化 ≤64K；写次数 = commit+message_send+modify_read+delete。
- 请求-响应关联：lobby 事件 `on_scorearchive_message_event`，payload 键 `__scorearchive_message_type/__body/__key`（key = 服务端单调序号，见 §3）；官方超时约 3000ms。

**游戏 lua 可用网络通道盘点**：① 局内 base.proto（KCP，限局内双端）；② `sce.httplib`（native LuaHttp.cpp，`request{url,method,json,header,input,output}`，任意 URL，全功能 HTTP）；③ `lobby.send_luastate_broadcast`（大厅↔游戏 lua 广播）；④ sce.s 专用 Entrance 通道（无通用自定义发送函数）。`subscribe_message/publish_message` = 官方更底层频道 pub/sub（ac 层支付/实名在用，schema 自由，服务端 API，直接用无需逆向）。

## 2. Lua→native→Entrance 调用链溯源

```
sce.s.<fn>(...)                                ← 纯 native 注册（__LUA_SCORE__）
  ↓ LuaScore.cpp（sceengine-strings :444850 源路径硬编码）
  ↓   sce.s 函数名表 :444933-444948（见 §1 客户端 API）
  ↓   commit 对象方法表 :444881-444894：commit/score_set/score_seti/score_addi/score_sets/
  ↓     money_add/money_cost/list_add/list_modify/list_delete/item_add/item_use/
  ↓     name_new/client_score_set（元表 __COMMIT_METATABLE_REF__ :444880）
  ↓ protobuf 子消息（编码失败日志暴露 proto 名 :444857-444925）：
  ↓   Commit/MoneyInit/NameSearch/QueryRankList/QueryRank/QueryRankTotal/QueryMessage/
  ↓   SetMessage/DeleteMessage/QueryList/QueryItem/ClientScoreInit
  ↓   字段名可见：target_map/listId/item_name/count/expire_type/expire_time/
  ↓     target_user_id/src_user_id/message_id/name_substr
  ↓ 顶层封装 CEProto::ScoreArchive::Msg（:444970）
  ↓ ★ 发送点 :444852 "Send Scorearchive msg to Sntrance failed."（Sntrance=Entrance 笔误）
Entrance 长连接（Entrance.cpp :456347；帧头 EntranceHeader :439973/439983；
  发送日志 :456425 "Send message to entrance, message : 0x%X"；
  连接参数 :456183 "Connecting to entrance, ip:%s port:%d protocol:%s"，TCP/KCP 可变；
  域名 :436614-436615 e.production.spark.xd.com / e.intl.spark.xd.com）
回包：Entrance 分发 → lobby 事件 on_scorearchive_message_event（:436414）
  → LuaScore 解析响应 proto（:444950-444967：Result/MoneyInitRes/NameSearchRes/
    QueryMessageRes/SetMessageRes/QueryItemRes/DeleteMessageRes/QueryListRes；
    :440251-440256：MultiScoreInitRes/ClientScoreInitRes）→ Lua 回调
```

native 侧关键符号/字符串（sceengine-strings.txt 行号）：

| 行号 | 字符串 | 含义 |
| --- | --- | --- |
| 444850 | `...\LuaScore.cpp` | Lua 绑定源文件 |
| 444949 | `__LUA_SCORE__` | 注册名 |
| 444852 | `Send Scorearchive msg to Sntrance failed.` | ★ 发送点（xref 即达发送函数） |
| 444970 | `CEProto::ScoreArchive::Msg` | 顶层 proto |
| 444953/444969 | `UnexpectedMessageFromScoreServer[` / `UnknownMessageFromScoreServer[` | 收包分发；服务端称 ScoreServer |
| 444873-444874 | `"9999-12-31 23:59:59"` / `NOW() + interval second` | 物品过期 MySQL 语法拼接 → 存储后端 MySQL |
| 436414-436418 | `on_scorearchive_message_event`/`ScoreArchiveMessage` | lobby 事件表项 |
| 440280/444846 | `ClientReadonlyMap`/`ClientReadWriteMap` | 地图作用域枚举 |
| 440224-440256 | `CSharpScore` + `CSharpScore.cpp` | 编辑器 C# 侧同协议实现 |
| 511880-511886 | `CSharpScoreDataManager_Create/Destroy/QueryCategory/QueryLabel`、`CSharpScore_Create/Destroy/InitScore` | 导出符号（P/Invoke 入口） |
| 456347/456425/439973 | Entrance.cpp / 发送日志 / EntranceHeader | 承载通道 |

**已证伪**：cloudvar-01/02 推测"标准 protobuf 日志（`protobuf Encoding failed: %s, proto name: Commit`）→ 二进制内嵌 FileDescriptorProto → proto_extract 可提取 descriptor"——**二进制无 .proto 字符串，全部手写 wire**。正确路线 = 动态帧捕获（§3）。

## 3. Entrance 帧格式与 0xA000 ScoreArchive 协议完整解码

### 3.1 传输层与 hook 点

- 传输 = libhv `WebSocketClient`，`wss://entrance-new-pd.tapsce.cn:443`（URL 仅 `wss://%s` 无路径；正式/测试/editor-pd 环境同域）。TLS 在 libhv 静态 OpenSSL 内——**libgmessl SSL hook 抓不到；ws2_32 层只能看到密文**。
- **原生消息 ID 免逆向渠道：`logs/lobby/lobby-*.log` 逐条打印收发 msgid 与 buffer size**（"Send message to entrance, message : 0x%X, buffer size : %d" / "Receive entrance message : 0x%04X"）。
- 帧内容为**手写 protobuf wire、无加密无压缩无签名无反重放**（TLS 由传输层负责），已双向完整捕获解码。
- hook 点（scegame BuildPCBox v152，RVA 基于 image base 0x140000000）：

| 点 | RVA | 说明 |
| --- | --- | --- |
| Entrance 发送函数入口 | 0x1e87be0 | 签名 `send(rcx=conn, edx=msgid, r8=frame_ptr, r9d=frame_len)`——frame 即 ScoreArchive::Msg 序列化字节（不含 msgid） |
| "Send message to entrance" 日志 xref | VA 0x141e87ecf | 函数内日志点（定位用） |
| "Receive entrance message" 日志点 | RVA 0x1e85f59 | 此处 msgid=[rsp+0x50]；**完整接收帧指针=[rbp-0x18]**（含信封 f1=msgid echo + f2=body，自描述 varint 长度） |

- wineditor v147（mini-runtime B 模式引擎）hook 点：**send RVA 0x1aa3770（同签名，已验证）/ recv log RVA 0x1aa1b19（msgid=[rsp+0x50]）**，经环境变量 ENT_SEND_RVA/ENT_RECV_RVA 传入 entrance_sniff。编辑器 BuildPC（sceengine.dll）同族代码同协议，RVA 不同需重定位。
- 定位方法：find_xref 找日志格式串 → 反汇编回溯函数入口/栈帧。响应帧指针位置发现法：日志点 dump rsp/rbp 窗口内所有疑似指针，跟随读内存搜已知内容（自己写入的 key 字符串）。

### 3.2 Entrance 信封与消息 ID 表

wire 消息（WS binary frame）= proto：**`f1 varint msgid` + `f2 bytes body`**（接收帧实证 `08 80c002 12 2f...` = f1 0xA000）。

| msgid | 方向 | 含义 | body 结构 |
| --- | --- | --- | --- |
| 0x0001 | → | 登录请求 | {f1 str 'default', f2 空, f4 varint 0x1000040, f6 varint token_type(11), f7 str login_token(385B), f19 varint 1, f20 空} |
| 0x0002 | ← | 登录响应 | f1 varint 0=成功；body{f1 user_id, f2 nick, ...}（559B） |
| 0x0010 | ← | server stop notify | 连接建立即收，忽略 |
| 0x0011 | → | 进局相关通知（28B） | notify start game 时点 |
| 0x3040 | ⇄ | 大厅局会话 | 17B 心跳式往复；body `08914e...`（f1 varint 序列号?） |
| 0x3060/0x3062/0x3063/0x3068/0x3069/0x3083 | ⇄ | 大厅局状态序列 | 进大厅局前后 |
| 0x6001 | → | 统计上报（~30s 批量） | {f1 str 类别(user_behavior/user_hardware/...), 重复 f3 {f2 k, f3 v}}——纯遥测，**自建客户端可不发**（实测不发也登录成功） |
| 0x7001 | ← | 登录后配置下发 | 登录响应后 |
| **0xA000** | ⇄ | **★ ScoreArchive 云变量（CEProto::ScoreArchive::Msg）** | 见 §3.3/§3.4 |

### 3.3 ScoreArchive::Msg 请求结构

顶层：`f1 str source`（lobby 态='startup'，直连任意值如 'poc' 也接受，服务端不校验；游戏态预计=地图名）+ `f2 str target_map`（'ClientReadonlyMap'/'ClientReadWriteMap'/空）+ `f3 varint 子类型` + `f4 bytes 子消息体` + `f5 bytes 空` + `f6 varint 0`。

**f3 子消息类型号（实测）**：

| f3 | 子消息 | f4 body 字段 |
| --- | --- | --- |
| 2 | **Commit** | repeated f1 = 操作项 {f1 str key, f2 varint op, f3 varint user_id, f4/f5/f6/... 按 op 不同}；f2 str = commit 描述 |
| 4 | **ScoreInit**（读） | f1 = user_id（lua 传 nil/number 时为 varint 原始字节 e6b2b812=38672742；官方 lobby 某调用为 str '-'、f2 为 user 字符串 '38672742'——两种形态都见过）；f2 str = key（repeated 多 key） |
| 10 | **MoneyInit** | {f1 varint user_id} |
| 20 | **NameSearch** | {f1 str key, f2 str name_substr} |
| 36 | **QueryRankList**（get_rank_list） | {f1 key, f3 起始名次, f4 结束名次, f5 str 'iscore'} |
| 38 | **QueryRank**（get_user_rank） | {f1 key, f2 str 'iscore'} |
| 48 | **QueryList**（list_query） | {f1 varint user_id, f2 str key, f3 varint limit} |
| 70 | **QueryItem** | {f1 varint user_id, f2 str key} |

未取到子类型号（调用签名未试通/权限不足未发送/服务端独有）：QueryRankTotal、QueryMessage、SetMessage、DeleteMessage、ClientScoreInit、MoneyCost、list_modify/list_delete、item_use、world_data_*、subscribe/publish。

### 3.4 ScoreArchive 响应结构

信封 f1=0xA000，f2 body = 响应 Msg：`f3 varint 响应子类型` + `f4 bytes body` + `f6 varint 请求序号`（单调递增，即 `__scorearchive_message_key`）+ `f7 bytes 空`。实测不需要客户端带 key（f5 空/f6 0 即可），串行请求-等待即可对齐。

| f3 | 含义 | body |
| --- | --- | --- |
| 5 | **ScoreInitRes**（读响应） | f1 repeated 条目，条目内：f1 varint user_id；f2 bytes 积分值组（内 f2 bytes = 数字积分条目 {f1 str key, f2 varint ivalue} repeated，f3 bytes = 字符串积分条目 {f1 str key, f2 str svalue} repeated；普通 table 积分为另一种序列化形态，未取样）；f6 varint 时间戳/内部 id |
| 49 | **QueryListRes** | 含原值 + 服务端时间戳（如 `2026-08-24 04:46:09`，list 项带落库时间） |
| 100 | **Result（错误/通用确认）** | {f1 varint error_code, f2 str reason} |

**已知错误码**：0=成功（commit 确认）、13=Nopermission、25=task_queue_limit_exceeded（限流，自定义码）、1146=TableNotFound（MySQL 原码直通）。

## 4. 全操作矩阵：op 码表 + MessagePack 值编码 + API 签名终表

### 4.1 Commit op 码表（f2 值，实测）

Commit 的每个操作 = `{f1 key, f2 op码, f3 user_id, ...}`，值按 op 放 f4/f5/f6/f7/f9/f10。

| op | 方法 | 附加字段 |
| --- | --- | --- |
| 0 | score_set（任意值） | f4 bytes = **MessagePack** 序列化值 |
| 3 | score_seti | f5 varint ivalue |
| 4 | score_addi | f5 varint 增量 |
| 7 | score_sets | f6 str svalue |
| 13 | money_add | f5 varint 金额 |
| 15 | list_add | f4 bytes = MessagePack 值 |
| 18 | item_add | f4=MessagePack 额外信息, f5=count, f7=item_name, f9=expire_type, f10=str `'"9999-12-31 23:59:59"'`（**带引号的日期串**，MySQL 直拼实锤）, f11=? |
| 20 | name_new | f4=str name, f7=str key（f1 空；value 字段未确认） |

未取到 op 码：money_cost/list_modify/list_delete/item_use/client_score_set（调用签名未试通/权限不足未发送）。

**MessagePack 实证**（score_set `{a=1,b='x',nested={1,2}}`）：

```
83 c401 61 01 c406 6e6573746564 92 01 02 c401 62 c401 78
= fixmap(3) → bin8'a':1, bin8'nested':fixarray(2)[1,2], bin8'b':bin8'x'
```

字符串用 **bin8/bin16 家族（0xc4/0xc5）**，不是 fixstr——**弃用 rmp-serialize**（其 str 走 fixstr 与观测不符）；entrance_client `mp_encode` 手写 JSON→MessagePack 编码器复刻观测编码，线上验证字节级一致（`set bgd_probe_mp '{"a":1,"b":"x","nested":[1,2]}'` → code=0，ScoreInitRes 复读 `83c40161...920102`）。客户端序列化 64K 上限（文档）即指此编码。

### 4.2 客户端 API 签名终表（wrapper 反汇编 + 探针实证，全闭合）

注册不在静态 luaL_Reg 表，而是运行时 `lua_pushcclosure` 注册块（VA 0x181319d1a 起，函数名字符串与 wrapper 函数指针交替 lea；committer 注册块 0x18131de06-0x18131df32）。引擎 lua = **lua54.dll（Lua 5.4）**。

| API | 签名（客户端） | 依据 |
| --- | --- | --- |
| sce.s.message_send | **(player\|uid\|nil src, key:string, target_user_id:int, value:any, events?)** | arg3 报错串='target_user_id参数不是合法整数'（0x1826cba70）；arg1='player参数不是合法的类型（player/integer/nil)'；arg2='key参数不是字符串'；arg4 序列化器 0x181325170；arg5 events |
| sce.s.message_query | (player\|uid\|nil, key:string, events, arg4?) | **events 必填 #3**（"#3 table expected" 来源） |
| sce.s.message_modify_read / message_delete | (player, message_id:int, [read:bool,] events?)（推断） | 错误串池 message_id/read 布尔 |
| committer.item_add | **(player, key:string, item_name:string, count:number, extra:any, expire_type:int, expire_time?:string)** | 定版（三次迭代 + 反汇编 + PIE 线上实证 ok）：arg3='item_name参数不是字符串'、arg4='count参数不是数字'、arg5=序列化器（extra 任意值 MessagePack）、arg6=isnumber（expire_type 0/1/2）、arg7=isstring 可选（'9999-12-31 23:59:59' 格式）。迭代坑：曾误以为 arg5=expire_type（报 'expire_type参数不是数字'），实际 arg6 才是 |
| sce.s.get_rank_list | ([map_name,] key, start:int, number:int, other_key?, events?) | 双 string 前缀判 map_name；isnumber×2 必填 |
| sce.s.get_rank_total | ([map_name,] key, events, ...?) | **events 必填 #3**（'iscore' 放 #3 报错）；服务端 Nopermission 但客户端校验通过 |
| sce.s.get_user_rank | (player, key, other_key?, events?) | 头部 player helper + 双 isstring |
| sce.s.score_init | (map_scope, uid\|nil, events, key...) | 官方样例见 xdeditor test/scorearchive.lua |
| committer 通用 | commit(desc, events)；操作函数参数1=目标玩家(player/uid/nil=自己) | 官方样例同上 |

wrapper VA 清单（SCEEngine.dll version-13）：get_commit 0x18131dde0 / score_init 0x181323990 / money_init 0x181321e40 / name_search 0x1813225c0 / get_rank_list 0x18131e150 / get_user_rank 0x18131ed10 / get_rank_total 0x18131e7f0 / message_query 0x1813215b0 / message_send 0x181321990 / message_modify_read 0x1813212a0 / message_delete 0x181320f60 / list_query 0x1813208d0；committer：score_set 0x181324170 / score_seti 0x181324560 / score_addi 0x1813235a0 / score_sets 0x181324570 / money_add 0x181321e20 / money_cost 0x181321e30 / list_add 0x18131fcd0 / list_modify 0x181320470 / list_delete 0x1813200f0 / item_add 0x18131f110 / item_use 0x18131f9f0 / name_new 0x1813221e0 / client_score_set 0x18131d1d0。

**uid 获取**：bgd `base.local_player()` 是 lua 包装表，native API 不认（报"player参数不是合法的类型（player/integer/nil)"），需传整数 uid 或 nil。PIE 本机 uid 从日志提取（`common/base/player.lua:330 local user：38672742`；Player repr `{player|1-user-1|""|38672742}`）。

错误串池（.rdata 0x26ca300-0x26ca980，中文 UTF-8）：`key不是字符串` `value不是合法的整数` `target_map不是字符串` `value暂时只支持字符串` `listId参数不是合法整数[字符串]` `value参数不是合法table` `item_name参数不是字符串` `count参数不是数字/整数` `expire_type参数不是数字` `expire_time参数不是字符串` `expire_type参数错误，不是0 1 2中的一个` `item_id参数不是整数` `key参数不是string` `name_substr参数不是字符串` `start不是整数` `number不是整数` `other_key参数不是string` `player参数不是合法的类型（player/integer)` `is readed参数不是bool` `target_user_id参数不是合法整数` `message_id参数不是合法整数` `read参数不是合法布尔值` `messageId参数不是合法整数`（camelCase 变体疑似 subscribe/push 回调路径）。

## 5. 权限模型：环境矩阵、地图级授权与修正记录

### 5.1 三环境权限矩阵（终版，含 cloudvar-10 修正）

| API | lobby 态（editor PIE / tester 大厅 / 直连） | 真局（tester test 环境 p_55a3） |
| --- | --- | --- |
| score_init 读（readonly/readwrite） | ✅ | ✅ |
| commit score_set/seti/sets/addi | ✅ | ✅ |
| list_query / list_add | ✅ | ✅ |
| client_score_set | ✅（推断：commit 类统一放行） | ✅ |
| name_new / item_add（commit） | ✅ | ✅ |
| **money_add（commit，op13）** | **✅ ok（修正，见 §5.3）** | **✅ ok** |
| money_init（读，f3=10） | ❌ Nopermission(13) | ❌ Nopermission(13) |
| get_rank_list / get_user_rank / get_rank_total | ❌ Nopermission(13) | ❌ Nopermission(13) |
| query_item | ❌ Nopermission(13) | ❌ Nopermission(13) |
| message_query / message_send | ❌ Nopermission(13)（签名已试通，请求到达服务端被鉴权拒绝） | ❌ Nopermission(13) |
| name_search | ❌ TableNotFound(1146)（该 key 的名字表未建） | ❌ TableNotFound(1146) |

### 5.2 授权结论（终版）

- **commit 类写操作全环境放行**（score_set/seti/sets/addi、list_add、item_add、money_add、client_score_set、name_new）；**查询类操作按地图授权**（money_init/rank/message/query_item/name_search——需要地图在创作者中心开通对应功能）。
- **货币「只写不读」是服务端设计**：money_add 写放行、money_init 读 Nopermission（写=commit 类统一放行；读=查询类按地图授权）。
- **entrance_client 直连（lobby 上下文）能力边界 = 全部写 + score/list 读**，已封顶。
- **target_map 语义**：'ClientReadWriteMap' = 按当前上下文地图解析的逻辑名；lobby 态也有默认地图上下文（直连能读到游戏侧写入的值，两态同空间，按用户+默认图解析）。token 不绑定地图。
- 官方限额（每局每分钟 300 读+300 写）按「局」维度统计；直连无局上下文，**只有队列限流（§6.2），未观测到分钟级硬计数**。

### 5.3 修正记录与被证伪的假设

| 原结论 | 终版结论 | 说明 |
| --- | --- | --- |
| 「money_add lobby 态 Nopermission」（cloudvar-06 §3）、「真局才解锁 money_add」（cloudvar-09 §0/§2） | **money_add 在 lobby/直连/真局全环境放行** | 旧观测误判（原因未考，可能 op 编码或会话状态问题）；2026-08-24 三环境复测全 ✅ |
| 「凭证/登录 flags 是 ScoreArchive 授权开关」假设 | **已证伪**：授权与凭证/flags 无关 | 实验矩阵：editor 凭证 money → 13；**tester 凭证**（`tester_1089/Win/User/user_info-e.production.spark.xd.com.json`，token_type=11 同 editor）money → 13；登录帧 f3=1/f5=1 变体 → 同拒；f4 flags 变体 0x1000041/42/48 → 同拒。tester lobby `isGameFlag : true` vs editor lobby `game_flag is: false` 也不是开关。tester 与 editor lobby 连同一 Entrance（`wss://entrance-new-pd.tapsce.cn:443`，无独立 tester 端点） |
| 「proto descriptor 可提取」（cloudvar-01/02） | **已证伪** | 二进制无 .proto 字符串，全部手写 wire |
| 「entrance 登录走 WSS，ws2_32 只能抓密文，拿不到明文」（credential-userid.md） | socket 层成立，但 **native 协议层 hook（发送函数入口/接收日志点）直接拿明文帧**，比 SSL hook 更干净 | 方法学可推广到任何 libhv 静态 TLS 的通道 |
| 「云变量必须经游戏局/官方 API」 | **已证伪** | Entrance 连接 + token 即可，局外可用（lobby 态/任意自建进程均可） |
| 「直连需先发 0x6001 遥测」 | 不需要 | 实测不发遥测登录直接成功 |

## 6. 直连客户端（entrance_client）：能力清单与限流实证

### 6.1 连接与登录（PoC 流程，已线上验证）

```
Python websockets / Rust → wss://entrance-new-pd.tapsce.cn/（WS 根路径）
  → 收 0x0010（server stop notify，建连即收，忽略）
  → 发 0x0001 登录：body = f1'default' + f4 0x1000040（固定 flags，直接重放）
                    + f6 11（token_type，编辑器凭证）+ f7 <token> + f19 1 + f20 ''
     （token = `User/user_info-<env>.json` 的 **token 字段**——385 字符，BBAXRA... 开头；
       不是 login_token/access_token）
  → 收 0x0002 登录响应（f1=0 成功；body 含 user_id/nick）
  → 收 0x7001（登录后配置下发，skip）
  → 收发 0xA000：读 Msg{f1 source, f2 'ClientReadWriteMap', f3 4, f4 ScoreInit{user_id,key}}
                写 Msg{f1 source, f2 'ClientReadWriteMap', f3 2, f4 Commit{repeated op, f2 desc}}
  → 响应：f3=5 ScoreInitRes / f3=49 QueryListRes / f3=100 Result{code,reason}
```

PoC 实测（editor-pd，uid 38672742）：读游戏侧写入值 ✓（cloudprobe_key1=12345、cloudprobe_skey1='hello_probe'）；Commit 直写 code=0 ✓；复读直写值 ✓（cloudprobe_poc_i=777、cloudprobe_poc_s='direct_write'）。直连绕开 sce.s/score 的类型分桶与签名限制——可构造任意 op 组合、批量 key、自定义 source 名。tester 凭证 userid 同为 38672742（登录响应 `user id : 38672742@140729326959375`，@ 后缀 = 设备/实例标识？）。连接保活：官方 ~30s 一批 0x6001；长连接需观察服务端踢人策略。

### 6.2 限流实证（entrance_client burst，editor-pd lobby 直连）

| 实验 | 结果 |
| --- | --- |
| burst 350 读（7ms 发完） | 350/350 响应（796ms，≈440 ops/s 应答速率）：**57 ok + 293 × code=25 task_queue_limit_exceeded** |
| 10s 后单读 | ✅ 恢复 ok（无限流后遗症，秒级恢复） |
| 再次 burst 100 读 | **精确复现 57 ok + 43 限流** |

结论：限流器 = **在途请求并发窗口（容量 ~57-64）**，超窗即拒 code=25（自定义码，非 MySQL/HTTP 家族）。直连读写吞吐天花板 = 窗口 × 单 op 延迟 ≈ **130 ops/s 稳态**（突发 ~440 ops/s 应答）。关联查询/批量读设计：单连接流水 + 窗口内并发（≤50 在途）即可跑满；无需担心分钟级配额。

### 6.3 entrance_client 子命令清单（examples/entrance_client.rs）

| 子命令 | 协议 | 说明 |
| --- | --- | --- |
| `read <key...>` | f3=4 ScoreInit | 读 score（默认能力） |
| `set <key> <json>` | op0 score_set | 写任意值（JSON→MessagePack `mp_encode`） |
| `ladd <key> <json>` | op15 list_add | 列表追加 |
| `madd <currency> <amount>` | op13 money_add | 写货币 |
| `iadd <key> <item_name> [count] [extra_json]` | op18 item_add | f4=msgpack extra, f5=count, f7=item_name, f9=expire_type=0, f10='"9999-12-31 23:59:59"' |
| `money` / `rank` / `urank` / `qitem` / `names` | f3=10/36/38/70/20 | 查询类（地图未授权时返回 13/1146） |
| `burst <key> <n>` | 连发 f3=4 | 限流压测：响应码分布/吞吐统计 |

实验变量：`ENT_F4`（登录 flags）、`ENT_LOGIN_EXTRA`（登录帧追加 hex 字段）。PowerShell 调参注意：json 参数用单引号包裹（`'{...}'`），双引号需 `""` 转义会被 PS 吞掉（两次踩坑）。

## 7. B 模式游戏态定性：sce.s 黑洞

- **B 模式（mini-runtime debug 局）客户端完全不连 Entrance**：游戏流量 = **UDP sendto/recvfrom ↔ debug host**（如 106.14.95.227:20400，与 assign_host 同 IP 不同端口）。进程内无任何 Entrance TCP 连接、无 ent_send 帧。
- 实证（CloudProbe S1-S14 + MessageProbe，场景-加载完成 +20s 触发）：**全部 sce.s 调用无任何回调**（score_init/commit/money_init/rank/list_query/name_search 全黑洞）；仅 message_query 触发本地 5s timeout。客户端参数校验照常工作（签名错误本地即报）。native 日志 `Send stat[entrance_net_stat] failed: send message error` 每 30s 一条 = Entrance 发送层从未就绪。
- **结论：B 模式不能做云变量游戏态实验**——ScoreArchive 无承载通道。游戏态帧补抓/权限验证必须真 tester 局。
- 推论价值：B 模式 UDP 协议（KCP 嫌疑，帧头 `4C .. .. .. 51/52` = 'L..Q/R'）逆向 = host 侧云变量（subscribe/publish/world_data）的唯一可见路径（debug host 流量本机可见），工程量大，暂挂。
- 附 lobby 态 PIE 客户端 TCP 通道样本（误挂收获）：Entrance 长连接在 123.56.153.41:19100（TCP）；帧型 26B 心跳（u32 总长 0x1a + seq u16 递增）、0x40 数据帧、0x41 回包、0x49/0x02/0x01 控制帧；**0x40 载荷无明文标记（固定会话头 + 流加密特征）**；ent_send hook 只见 0x6001 遥测明文——0xA000 走 WSS(TLS)，ws2_32 层只有密文。

## 8. 自动化进真 tester 局通道（已实证）

```
tester_1089.exe -game=<project_id> -tag=test [-ai_test=1]
```

- **参数必须 `-key=value` 形式**：`-game p_55a3`（空格形）会被解析为「flag 存在但值为空」→ TO_START_GAME_MAP 停在 app_android 壳死等（踩坑实录）。
- 启动链：startup 读 argv('game') → 下载更新该图 → quick_start 大厅壳（无 wx_lobby 时 = **app_android**；无 game 参数时 = **app_box** 盒子大厅）→ app_android 内 lib_lobby 模式页 → **`-ai_test=1` 触发免交互开局**（proxy_guide.check_immediately_start: argv.has('ai_test') → match_game:start_single → 秒匹配进图）。大厅进真游戏两段式：大厅（app_android/app_box 也是「地图」）→ 匹配 → 真游戏地图。
- 登录：凭证持久化自动登录（token_type=11，`Login success, user id : 38672742@<实例id>`）。
- 相关机制（源码实证）：`start-game://<project_id>&tag=test` URL scheme（本机未注册协议）；`lobby.register_event('reload_app')` / `app.reload_with_command_line(cmdline)`（native 触发的换参重启）；第二实例转发在本构建未生效。
- scegame 直启完整参数（launcher 转发后形态）：`-server=e.production.spark.xd.com -wx_lobby=app_android -launcher="../tester_1089.exe" -game=p_55a3 -tag=test -ai_test=1`。
- **tester 大厅（lobby 态）也在收发 0xA000**（app_box score_info 读地图评分/最近游玩）——lobby 态读权限两环境一致。

## 9. 星火 2.0（WasiCore）云数据 API 面

> 来源：wasicore-03（SDK docs + UserCloudDataTest 示例，约 1600 行文档研读）。**注意：这是 2.0（C#/WasiCore）项目的 API，1.0 lua 项目不可用。**

**结论：2.0 云数据 = 结构化多桶 KV + UUID 列表 + 唯一名称注册表 + 跨用户 ACID 事务 + 游标扫描 + 模糊名称搜索**，全面超越 1.0 云变量；但仍「按 userId+key 点查」，无任意 WHERE。**仅服务端可用**（`#if SERVER`；客户端无缓存 API，官方路径 = 服务端读完经快照/消息/同步属性推客户端）。传输层被 provider 完全封装。

### 9.1 数据类型（六桶，`GameCore.UserCloudData`，双入口 `CloudDataApi`/`CloudData` 别名）

| 类型 | 语义 |
| --- | --- |
| BigInt | 64 位整数 |
| VarChar255 | ≤255 字符串 |
| Blob | 二进制（raw byte[] / `Utf8BlobData` / `DoubleBlobData` 子类型标记） |
| Currency | 货币（不足自动 `InsufficientCurrency`） |
| CappedData | 上限 + 定时重置（`UserDataResetOption.Daily()/Weekly()/Monthly()/Never`），存已消耗量，含 Cap/LastUpdateTime/NextResetTime |
| ListItem | 全局唯一雪花 UUID 列表项 |

### 9.2 查询（批量，多 userId × 多 key）

```csharp
await CloudData.QueryUserDataAsync(long[] userIds, VarChar180[] keys);   // 通用
await CloudData.QueryPlayersDataAsync(players, keys);
await CloudData.QueryCurrencyAsync(userIds, keys);
await CloudData.QueryCappedDataAsync(userIds, keys);
await CloudData.QueryUserListItemsAsync(userId, key, maxCount);                  // 最新 N 条
await CloudData.ScanUserListItemsAsync(userId, key, batchSize, beforeItemUuid);  // 游标分批（排他游标，UUID 倒序）
await CloudData.FindListItemByIdAsync(long itemId);                              // 全局 ID 反查（跨列表）
await CloudData.CheckNameClaimedStatusAsync(collectionKey, name);                // 名称占用
await CloudData.SearchClaimedNamesAsync(collectionKey, namePart);                // 模糊搜索 ← 最接近关联查询
await CloudData.QueryUserNameBatchAsync(long[] userIds);                         // 平台用户名反查
```

### 9.3 写入 = 流式事务（TransactionBuilder）+ 跨用户原子提交

```csharp
await CloudData.ForUser(userId)                    // 或 ForPlayer / ForUser(User)
    .SetData(key, value)                           // int/long/string/byte[]/bool 自动路由桶
    .AddToData(key, delta)
    .DeleteData(key, CloudDataType.Blob)           // 删除需显式带类型
    .AddCurrency(key, n) / .CostCurrency(key, n)
    .ModifyCappedData(key, delta, cap, UserDataResetOption.Daily())
    .ResetCappedData(key)
    .PrepareListItem(key, byte[]) → ListItemReference   // 事务前即可拿 .Id
    .PrepareUtf8ListItem(key, json) / .PrepareDoubleListItem(key, d)
    .AddListItem(itemRef) / .AddListItems(refs)
    .UpdateListItem(itemId, bytes) / .UpdateListItemUtf8 / .UpdateListItemDouble
    .DeleteListItem(itemId) / .DeleteListItems(refs)
    .MoveListItem(itemId, "warehouse")             // 跨列表移动（背包→仓库）
    .ClaimName(collection, name, desc?) / .DeleteName(collection, name)
    .WithDescription("...")                        // 审计描述
    .WithOptimization(true)                        // 默认开：同 key 合并（+100+50-20→+130）
    .WithValidation(true)
    .ExecuteAsync();                               // → UserCloudDataResult 枚举

await CloudData.ForUsers(u1, u2).ForAllUsers(b => b.AddCurrency("gold", 50)).ExecuteAllAsync();   // 逐用户提交，允许部分成功
await CloudData.ForUsers(buyer, seller).ForUser(...).ForUser(...).ExecuteSingleCommitAsync(desc); // 单 commit，任一失败整笔回滚（真 ACID）
await CloudDataApi.ExecuteTransactionAsync(List<TransactionOperation>, desc);                     // 手动 op 列表
```

Key 语义：远端**大小写不敏感**，≤180 字符禁空白；部分命中正常成功（不存在的 userId/key 直接缺席）。就绪时序：`await CloudData.WaitUntilReadyAsync()` / `CloudData.IsReady` / 事件 `Game.OnUserCloudDataServiceInitialization`。

### 9.4 配额/限流（无 1.0 式分桶次数表）

「短时间窗口」三维限流，限额按**运行中游戏实例**共享：

| 结果码 | 触发 | 处理 |
| --- | --- | --- |
| `TooManyReadOperations` | 短时间查询过多 | 合并 key/userId、降频 |
| `TooManyWriteOperations` | 短时间事务过多 | 合并事务、禁逐字段提交 |
| `MessageSizeTooLarge` | 单次/窗口数据量过大 | 缩 payload、游标分批 |

其他结果码：`Success / QueryUserIdMissing / ServiceNotInitialized / FailedToSend / TransactionCommitEmpty / InsufficientCurrency / CapExceeded / LocalOperationFailed`。命中限额不要立即原样重试（继续占窗口）。

### 9.5 与 1.0 直连研究的对照

- 分层：`CloudDataApi → CloudDataOperations+TransactionBuilder → CloudDataManager → IUserCloudDataProvider（引擎接口层）`；Manager 管「Provider 生命周期、请求 ID 映射、异步响应」——**请求/响应异步消息模式，与 1.0 Entrance request-id 对拍一致**。
- docs 无 ScoreArchive/Entrance/0xA000 字样；但 docs 出现 `(MessagePack)` / `[MessagePackObject]`——**合理推测 2.0 云数据仍走 Entrance 通道 + MessagePack 族，op 面扩展为 query/commit/scan/claim 结构化命令**。数据模型强烈暗示远端是事务型结构化存储（大小写不敏感 key、VarChar 类型名、UUID 列表、跨用户 commit 回滚）。
- 1.0 直连（entrance_client）仍是 1.0 项目的最底层答案；2.0 直连需先证实协议同通道（见 §11 遗留问题）。

## 10. 坑与教训沉淀

**抓包/Frida**：
- ws2_32 `connect` hook 抓不全连接（libhv 用 ConnectEx）——用 getpeername 兜底解析。
- WSARecv 实际字节数在 arg3（lpNumberOfBytesRecvd），别按 buffer 容量 dump；IOCP overlapped 场景 onLeave 时机不可靠——**native 函数 hook（协议层）比 socket 层 hook 更干净**。
- WSABUF Win64 布局 = len:u32@0 + pad + buf:ptr@8。
- tester 的 libgmessl SSL hook 只对走 gmesdk 的 TLS 有效，Entrance WSS（libhv 静态 OpenSSL）不吃这套。
- frida spawn 的进程在脚本退出后不会自动退出——抓完必须手动 Stop-Process（防多实例重复登录）。
- python print 要 flush=True（管道缓冲假阴性）；python 长输出被工具宿主截断时，jsonl 文件是权威数据源。
- **attach 抓包前必须核对进程 Path**（`Get-Process sce | Select Id,Path`）——编辑器 PIE 残留进程（停止调试后仍存活、仍发 stat 心跳）与 mini-runtime 游戏进程同名 `sce`/`SCE`，极易挂错；挂错后数据貌似合理（有 stat 有会话帧），污染结论（本轮实际踩中，误挂 lobby 态样本险些当游戏态结论）。
- **themis 反作弊实锤**：`frida attach scegame`（tester）→ DeviceAttachError，运行中 attach 被拒。frida spawn（进程创建即注入，themis 初始化前挂钩）未试——是唯一卡点。免抓包替代：lua 探针 ok/error 回调已给出全部权限结论。
- cargo examples 构建前置：`LIBCLANG_PATH=<pip clang>/native`；frida-sys 下载 devkit 需代理（HTTP_PROXY/HTTPS_PROXY=127.0.0.1:7897），无代理会**静默卡死**在 "Compiling frida-sys"。

**二进制逆向**：
- find_luareg.py 只查字符串首出现点的 8 字节绝对指针；对运行时 pushcclosure 注册块**无效**——修正版 `test/temp/find_str_refs.py`（全出现点 + RIP 相对 lea 扫描）。
- pefile 反查 IAT：FF25 跳转桩 disp 在偏移 2（`FF 25 disp32`，target=T+6+disp），import 地址 = IAT slot VA。
- capstone `lea rdx,[rip+disp]` 目标 = insn_va+7+disp（曾口算错位一节得假结论；已字节级核对修正）。
- sceengine-strings.txt 字符串 dump 不含中文串（非 ASCII 被过滤）——中文错误串需直接在 PE 里按 UTF-8 字节搜。
- SCEEngine.dll .text RVA→文件偏移 = -0xC00；disasm_at 吃文件偏移。
- entrance_sniff waitModule 列表 sceengine.dll 优先（编辑器/wineditor 构建 Entrance 在 dll，exe 基址误配 dll RVA 是坑）。
- VS std::string SSO：cap≥16 时 *rdx 为数据指针，size@+0x10。

**环境/注入**：
- mini-runtime 自托管 lobby = 云变量协议研究的理想沙盒：`runtime/Update/editor-pd.spark.xd.com/Res/startup/application/entrance/main.lua` 是 TNND 加密散文件，decrypt_file 解密→改→明文写回即可注入任意 lua（官方可执行明文）。注入点：登录成功分支 `IS_LOGIND = true` 之后（原版官方代码就在此处调 `sce.s.score_init`，证明 lobby 态 sce.s 可用）。启动 `scegame.exe -inner -headless -no_update "-server=editor-pd.spark.xd.com"`（cwd=runtime/）。**必须 -no_update**——否则自更新会整包重装 startup 冲掉注入（实测踩坑：main.lua 连同备份被更新器清掉，assemble_runtime.ps1 重组装恢复）。
- lobby lua 日志 = `runtime/logs/lua/lua-application-*.log`；native 日志 = `runtime/logs/lobby/lobby-*.log`（Entrance 逐条消息 ID）、`runtime/logs/game/game-*.log`。
- MCP 探针注意：lua.run_lua 属 danger 级，需在 `D:\sce_online\logs\bgd_csharp\config.json` 的 danger_allow 放行。

**tester 窗口操控**：
- tester 窗口会自发进入 iconic（-32000,-32000）：SW_RESTORE/SW_SHOWNORMAL/SetWindowPlacement 均无效，**必须 `SendMessage(WM_SYSCOMMAND, SC_RESTORE=0xF120)`**（唯一有效恢复手段）。
- mini-runtime capture CLI（WGC）对 iconic 窗口可正常离屏截图。
- mouse_event 模拟点击对 tester 大厅（Urho3D 自绘 UI）效果不稳定——已被 `-game/-tag/-ai_test` 参数通道取代，弃用。

## 11. 遗留问题 / 下轮入口

1. **0x0011 进局通知绑定**：若真局 Entrance 会话的建立帧可复刻，entrance_client 增加「进局绑定」步骤即成全权限直连（最终形态）。需 frida spawn 抓真局首连（themis 绕过是唯一卡点）。当前证据：真局权限 ≈ lobby（仅差局级授权的推测未被证实——实测真局查询类仍 Nopermission，授权是**地图级**而非连接/局级，0x0011 绑定的价值存疑，需重新评估）。
2. **未取到的子类型号/op 码**：QueryRankTotal/QueryMessage/SetMessage/DeleteMessage/ClientScoreInit、money_cost/list_modify/list_delete/item_use/client_score_set op 码——查询类需地图授权后才能触发（创作者中心开通功能后重测）；committer 未测 op 可按规律推算或在已授权地图补抓。
3. **服务端独有 API（world_data/subscribe/publish）不可达**：服务端跑在官方远端 host，其 Entrance 流量本机不可见。唯一可见路径 = B 模式 debug host 的 UDP 协议（KCP 嫌疑）逆向，工程量大，暂挂。
4. **2.0 协议同通道验证**（wasicore-03 §4 线索）：2.0 游戏态抓 Entrance 帧（entrance_sniff 复用），对比 1.0 的 0xA000 是否新增 msgid/op；或逆向 GameSparkCore.dll/引擎 provider 层找 op 表。若证实同通道，entrance_client 可扩展支持 2.0 富操作（跨用户事务/列表/名称注册表直连）。
5. **风控未知**：直连绕过引擎，但 Entrance 是官方客户端协议通道，帐号行为与官方客户端一致；建议低频使用直到摸清计数/风控。读写计费维度：直连只有队列限流（code=25），「每局每分钟 300 读+300 写」未在直连观测到——可在创作者中心云变量后台看调用统计进一步确认。
6. **探针修正（下轮真局用）**：S10 get_rank_total(map, key, events)（events 在 #3）；S13 item_add(player, key, item_name, count, extra, expire_type, expire_time?)（签名已定版，探针未改）。
7. **score_set(table) 的响应序列化形态未取样**（ScoreInitRes 中普通 table 积分的第三种条目形态）；readonly_map 的 key 空间 = 地图积分（大厅展示用），直连可读（已验证读 p_55a3 返回 ok 空）。
