# 云变量静态补遗 + 协议逆向执行清单

> 研究日期：2026-08-23 | 状态：静态补遗完成；工具执行待跑
> 前置：cloudvar-01-lua-chain.md（Lua→native→Entrance 全链）

## 1. LuaScore 块精校（sceengine-strings 444851-444974）

- score 函数表(444933-444948)：get_commit/score_init/money_init/name_search/get_rank_list/get_user_rank/get_rank_total/message_query/message_send/message_modify_read/message_delete/list_query/query_item/client_score_init/readonly_map/readwrite_map；注册名 `__LUA_SCORE__`(444949)
- commit 方法表(444881-444894)：commit/score_set/score_seti/score_addi/score_sets/money_add/money_cost/list_add/list_modify/list_delete/item_add/item_use/name_new/client_score_set；元表 `__COMMIT_METATABLE_REF__`(444880)
- proto 名（编码失败日志暴露）：Commit/MoneyInit/NameSearch/QueryRankList/QueryRank/QueryRankTotal/QueryMessage/SetMessage/DeleteMessage/QueryList/QueryItem/ClientScoreInit；响应：Result/MoneyInitRes/NameSearchRes/QueryMessageRes/SetMessageRes/QueryItemRes/DeleteMessageRes/QueryListRes（+ MultiScoreInitRes/ClientScoreInitRes :440251-440256）
- 字段名：target_map/listId/item_name/count/expire_type/expire_time/target_user_id/src_user_id/message_id/name_substr
- **「protobuf Encoding failed: %s, proto name: Commit」(444857) = 标准 protobuf 库日志** → 生成代码大概率内嵌 FileDescriptorProto（区别于控制协议手写 wire）→ proto_extract 有望直接提出 ScoreArchive descriptor

## 2. 新线索

- **message_send/message_query**：官方玩家间消息通道，可借作自定义传输（仍计次数，schema 自由）
- **sce.httplib**（445042-445110）游戏 lua 全功能 HTTP：GET/POST/header/query/json/input(字符串|文件|流|函数)/output(路径|流|函数)/progress/限速 → 自建中转服务器路线零逆向成本
- **map_publisher.connect_random_terrain_server**（444992-444999）：引擎内置第三方 TCP 通道先例，帧格式日志 445039（"send request to random terrain server, message type [%d], request id [%d]"），逆向价值高
- io.DownloadFile/UploadFile（443679-443681）
- 端点全集：e.production/e.intl.spark.xd.com（Entrance 436614-436615）、statistic-server-%s（454483）、publisher-<env>.spark.xd.com:9000（管理面，cloudvar-01 §4）
- base.s 服务端封装不在镜像内（服务端脚本库未镜像）；scegame-tester strings 证明局内 host 同链路（cloudvar-01 §2 旁证）

## 3. 直读直写路线评估

| 路线 | 说明 | 自由度 | 逆向成本 |
| --- | --- | --- | --- |
| A. 仿 Entrance 客户端直连 | 需 proto descriptor + 消息 ID + 鉴权字段（抓包解决）；游戏外进程或游戏内 lua socket 均可仿 | 最高 | 最高 |
| B. native hook | CSharpScore_* 导出符号(511880-511886)或 LuaScore 发送点 detour | 高 | 触碰安全红线，仅本机研究 |
| C. 自建 HTTP 中转 + sce.httplib | 数据存自己服务器；自由 schema/批量/关联查询 | 高（但不经 ScoreServer） | 零 |
| D. message_* 借用 | 官方玩家消息通道 | 中（仍计次数） | 零 |

## 4. 工具执行 runbook（在 sce_app_mini-runtime 下）

1. `cargo run --example proto_extract -- D:\sce_online\version-13\sceengine.dll out score`（同时对 tester 的 scegame 跑一遍交叉验证）
2. find_xref 定位 "Send Scorearchive msg to Sntrance failed."（strings:444852）→ 发送函数 → 上移取 Entrance 消息 ID 常量
3. test_res002 玩家-连入事件放 sce.s.score_init/get_commit 调用作触发源 → entrance_login_capture（SSL 明文）或 frida_capture（ws2_32）抓 Entrance 帧
4. pe_imports 确认 Entrance 连接 TLS 栈归属（libgmessl?）
5. MCP 探针注意：lua.run_lua 属 danger 级，需在 `D:\sce_online\logs\bgd_csharp\config.json` 的 danger_allow 放行
