# 直连限流实测 + money_add/item_add 权限与签名修正

> 研究日期：2026-08-24 | 状态：限流维度实证；money_add 旧结论修正；item_add 签名定版
> 前置：cloudvar-06（操作矩阵）/ cloudvar-09（真局权限矩阵）——本文含对两者的修正

## 0. 一句话结论

直连读限流 = **并发窗口（~57 in-flight）+ code=25 task_queue_limit_exceeded**，无账号/每分钟硬计数（秒级恢复，稳态 ~130 ops/s）；**money_add 在 lobby 态也放行**（cloudvar-06/09「真局才解锁」结论修正）；item_add 签名定版 `(player, key, item_name, count, extra, expire_type:int, expire_time?:str)`（线上实证 ok）。

## 1. 读写计数/限流维度实测（entrance_client burst，editor-pd lobby 直连）

| 实验 | 结果 |
| --- | --- |
| burst 350 读（7ms 发完） | 350/350 响应（796ms，≈440 ops/s 应答速率）：**57 ok + 293 × code=25 task_queue_limit_exceeded** |
| 10s 后单读 | ✅ 恢复 ok（无限流后遗症） |
| 再次 burst 100 读 | **精确复现 57 ok + 43 限流** |

结论：
- 限流器 = **在途请求并发窗口（容量 ~57-64）**，超窗即拒 code=25（自定义码，非 MySQL/HTTP 家族）。
- **未观测到「每局每分钟 300 读+300 写」硬计数**——该限制按文档是「每局」维度，直连无局上下文，只有队列限流。直连读写的吞吐天花板 = 窗口 × 单 op 延迟 ≈ **130 ops/s 稳态**（突发 ~440 ops/s 应答）。
- 对「关联查询/批量读」设计：用单连接流水 + 窗口内并发（≤50 在途）即可跑满；无需担心分钟级配额。

## 2. money_add 结论修正（重要）

| 环境 | money_add（op13/commit） | money_init（读，f3=10） |
| --- | --- | --- |
| cloudvar-06 旧记录（lobby） | ❌ Nopermission | ❌ Nopermission |
| 今日 PIE lobby（S12） | ✅ ok | ❌ Nopermission |
| 今日 entrance_client 直连 lobby（madd） | ✅ code=0 | ❌ Nopermission |
| 今日真 tester 局（S12） | ✅ ok | ❌ Nopermission |

- **money_add 在 lobby/直连/真局全放行**——cloudvar-06 §3 的「money_add Nopermission」与 cloudvar-09 §0/§2 的「真局解锁 money_add」均修正为：**money_add 本来就全环境放行**（旧观测误判，原因未考；可能是当时 op 编码或会话状态问题）。
- **读写不对称实锤**：money_add 写放行、money_init 读 Nopermission——货币「只写不读」是服务端设计（读=查询类，按地图授权；写=commit 类，统一放行）。
- 推论：**commit 类写操作（score_set/seti/sets/addi、list_add、item_add、money_add、client_score_set、name_new）全环境放行；查询类（money_init/rank/message/query_item/name_search）按地图授权**。entrance_client 直连的能力边界 = 全部写 + score/list 读。

## 3. item_add 签名定版（三次迭代 + 反汇编 + 线上实证）

`committer.item_add(player, key, item_name, count:number, extra:any, expire_type:int, expire_time?:string)`

| arg | 校验（wrapper 0x18131f110） | 说明 |
| --- | --- | --- |
| 1 | player helper | nil=自己 |
| 2 | lua_isstring | key（云变量 key） |
| 3 | lua_isstring | item_name |
| 4 | lua_isnumber | count |
| 5 | 序列化器 0x181325170 | **extra（任意值，MessagePack）** |
| 6 | lua_isnumber | **expire_type（0/1/2）** |
| 7 | lua_isstring（可选） | expire_time（'9999-12-31 23:59:59' 格式） |

- 迭代史：arg5=table → 'expire_type参数不是数字'（以为 arg5=expire_type）；arg5=0 → 同错（实际 arg6 才是 expire_type）→ 反汇编确认 arg5=序列化器、arg6=isnumber、arg7=isstring → 定版 PIE 实证 **S13 ok**。
- 直连 `iadd probe_item_key sword 1`（op18：f4=msgpack extra, f5=count, f7=item_name, f9=expire_type=0, f10='"9999-12-31 23:59:59"'）→ code=0。
- get_rank_total 签名实证：`(map?, key, events, ...)`（events 必填 #3，服务端 Nopermission 但客户端校验通过）。

## 4. 工具迭代

- entrance_client 新增：`burst <key> <n>`（限流压测：连发 score_init + 响应码分布/吞吐统计）、`madd <currency> <amount>`（op13）、`iadd <key> <item_name> [count] [extra_json]`（op18）。
- PowerShell 调参 json 用单引号（旧坑 cloudvar-06 §6 复踩）。
