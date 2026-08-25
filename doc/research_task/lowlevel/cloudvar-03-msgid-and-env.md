# 云变量协议突破：Entrance 消息 ID 表 + 抓包环境 + 官方 API 文档镜像

> 研究日期：2026-08-23 | 状态：消息 ID 实证；帧内容 dump 方法已锁定（待做）
> 前置：cloudvar-01 / cloudvar-02

## 0. 一句话结论

**scegame 的 lobby 日志（`logs/lobby/lobby-*.log`）逐条记录 Entrance 收发消息 ID**（"Send message to entrance, message : 0x%X, buffer size : %d" / "Receive entrance message : 0x%04X"）——消息 ID 不用逆向，读日志即可。**云变量（ScoreArchive）= Entrance 消息 0xA000**。Entrance = `wss://entrance-new-pd.tapsce.cn:443`（editor-pd 环境同域）；本机 SSL hook（libgmessl）抓不到该 WSS（TLS 栈不在 libgmessl 导出，疑似 libhv 静态 OpenSSL），**改走「hook Entrance 发送函数拿序列化后 buffer」路线**（TLS 之前，明文）。

## 1. 官方 API 文档镜像（重大发现）

`test_res002/.bgd/libs/types/class_cloud.d.lua` = 官方云变量文档的完整 EmmyLua 转录（含双端 API 全集与限制）：

- **服务端 `score.*` 全局模块**（仅服务端，协程 RPC，sync 返回 error_code/data/err_msg）：`get/money_get/list_query/list_query_by_uuid/get_commit(committer: set/add/addi/money_add/money_cost/list_add/list_modify/list_delete/name_new/world_data_set/world_list_add)/message_send/message_query/message_modify_read/message_delete/name_search/subscribe_channel/unsubscribe_channel/subscribe_message/publish_message/world_data_get/test_cloud_value`。
- **客户端 `sce.s.*`**（仅客户端，异步 {ok,error,timeout}）：get_commit/score_init/money_init/name_search/list_query/query_item + readonly_map/readwrite_map。
- 限制：每局每分钟 300 读 + 300 写；单次 ≤32M、每分钟写 ≤48M；客户端序列化 ≤64K；写次数 = commit+message_send+modify_read+delete。
- `test_res002/project/map_settings.json` 有 `"EnableSetScoreName": true`。
- **subscribe_message/publish_message = 官方更底层频道 pub/sub**（ac 层支付/实名在用，在线消息，schema 自由）。

## 2. 抓包环境（重要方法学沉淀）

**mini-runtime 自托管 lobby = 云变量协议研究的理想环境**：
- `runtime/Update/editor-pd.spark.xd.com/Res/startup/application/entrance/main.lua` 是 TNND 加密散文件；decrypt_file 解密→改→明文写回即可注入任意 lua（官方可执行明文）。
- 注入点：登录成功分支 `IS_LOGIND = true` 之后（main.lua:2456 区域，startup 369）；原版官方代码就在这里调 `sce.s.score_init(sce.s.readwrite_map, ...)`（uplog_check），证明 lobby 态 sce.s 可用。
- 启动：`scegame.exe -inner -headless -no_update "-server=editor-pd.spark.xd.com"`（cwd=runtime/）。**必须 -no_update**，否则自更新会整个重装 startup 包冲掉注入（本次实测踩坑：忘加后 main.lua 连同备份被更新器清掉，assemble_runtime.ps1 重组装恢复）。
- lobby lua 日志 = `runtime/logs/lua/lua-application-*.log`；native 日志 = `runtime/logs/lobby/lobby-*.log`（Entrance 逐条消息 ID）、`runtime/logs/game/game-*.log`。
- 探针用例结果（editor-pd，userid 38672742）：score_init readonly/readwrite ok；commit(score_seti/score_sets) ok；money_init → error 13 Nopermission；name_search → **error 1146 TableNotFound（MySQL 错误码直通！服务端 = MySQL 实锤）**；list_query ok。

## 3. Entrance 消息 ID 表（lobby 日志实证，editor-pd 2026-08-23）

| ID | 方向 | 含义 | 证据 |
| --- | --- | --- | --- |
| 0x0001 | → | 登录请求（422B，含 token） | 登录时序 |
| 0x0002 | ← | 登录响应 | 紧跟 0x0001 |
| 0x0010 | ← | server stop notify | 连接刚建立即收 |
| 0x0011 | → | 进局相关通知（28B） | notify start game 时点 |
| 0x3040 | ⇄ | 大厅局会话（assign/create?） | 17B 心跳式往复 |
| 0x3060/0x3062/0x3063/0x3068/0x3069/0x3083 | ⇄ | 大厅局状态序列 | 进大厅局前后 |
| 0x6001 | → | 会话保活/杂项（~30s 批量，100-900B 多种） | 全程周期发送 |
| 0x7001 | ← | 登录后下发（配置?） | 登录响应后 |
| **0xA000** | ⇄ | **★ ScoreArchive 云变量消息（CEProto::ScoreArchive::Msg）** | 探针 P1-P7 七个调用 + 官方 2 个 = 9 发 9 收，时序毫秒级对齐 |

探针消息大小记录（发送序）：63/64/74/133/92/38/57/75/67 字节（对应 score_init readonly、score_init rw×2、commit、money_init、name_search、list_query 等子消息混合）。

## 4. 网络通道全景（本次抓包实证，editor-pd lobby 态）

| 通道 | 地址 | 协议 | 内容 |
| --- | --- | --- | --- |
| Entrance | entrance-new-pd.tapsce.cn:443（解析到 47.101.215.72 等） | WSS（TLS 不在 libgmessl，ws2_32 send/recv 层为密文） | 登录/云变量/大厅局管理 |
| 大厅局 | 101.133.173.123:11248 UDP | KCP（sendto/recvfrom，帧头 `4C .. .. .. 51/52` = 'L..Q/R'） | 大厅玩法消息（好友/最近游玩/状态） |
| 不明 TCP | 123.56.116.126:19100 | 明文（含 32hex token + base64 段 + `t[0]: <数字>` 文本） | 待查（im/global_chat/统计?） |
| 本地回环 | 127.0.0.1 多对 | TCP | wasm/node 桥 |

**坑沉淀**：① ws2_32 `connect` hook 抓不全连接（libhv 用 ConnectEx）——用 getpeername 兜底解析；② WSARecv 实际字节数在 arg3（lpNumberOfBytesRecvd），别按 buffer 容量 dump；③ WSABUF Win64 布局 = len:u32@0 + pad + buf:ptr@8；④ tester 的 libgmessl SSL hook 只对走 gmesdk 的 TLS 有效，Entrance WSS 不吃这套；⑤ python print 要 flush=True（管道缓冲假阴性）。

## 5. 下一步：帧内容 dump（已定位方法）

- 编辑器 dll 反汇编（0x181aa3a5f xref 处）：日志调用点在 Entrance 发送函数内，虚调用 `conn->send(buf, msgid, 3)`（rcx=conn, rdx=&std::string buf, r8d=msgid, r9d=3）。
- 计划：find_xref 在 scegame.exe 上定位同款字符串 → 回溯函数入口 → frida hook 入口 dump (msgid, std::string buffer)（VS std::string SSO：cap≥16 时 *rdx 为数据指针，size@+0x10）→ 得到 0xA000 明文帧 → 离线 wire 解析（protobuf 手写 wire，字段号从样本反推）。
- 接收方向同理（"Receive entrance message" 日志点附近 hook 分发函数）。
