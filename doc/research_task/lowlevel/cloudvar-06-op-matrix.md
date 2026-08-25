# ScoreArchive 全操作矩阵：op 码表 + MessagePack 值编码 + 权限矩阵

> 研究日期：2026-08-24 | 状态：客户端全 API 帧解码完成（写矩阵全解）；服务端 subscribe/publish 不可达（见 §4）
> 前置：cloudvar-04（帧格式）/ cloudvar-05（直连 PoC）
> 数据件：test/temp/full_matrix.jsonl + full_matrix_decode.txt

## 0. 一句话结论

Commit 的每个操作 = `{f1 key, f2 op码, f3 user_id, ...}`，**值用 MessagePack 编码**（f4/f5/f6 按 op 不同）；排行榜/物品/消息查询子消息类型号全部拿到。lobby 态权限：读 + commit 写放行；money/item/rank/name 系返回 Nopermission(13)——按 state/地图授权。

## 1. Commit op 码表（f2 值，实测）

| op | 方法 | 附加字段 |
| --- | --- | --- |
| 0 | score_set（任意值） | f4 bytes = **MessagePack** 序列化值 |
| 3 | score_seti | f5 varint ivalue |
| 4 | score_addi | f5 varint 增量 |
| 7 | score_sets | f6 str svalue |
| 13 | money_add | f5 varint 金额 |
| 15 | list_add | f4 bytes = MessagePack 值 |
| 18 | item_add | f4=MessagePack 额外信息, f5=count, f7=item_name, f9=expire_type, f10=str '"9999-12-31 23:59:59"'（带引号的日期串！MySQL 直拼实锤）, f11=? |
| 20 | name_new | f4=str name, f7=str key（f1 空；value 字段未确认） |

未取到（调用签名未试通/权限不足未发送）：money_cost/list_modify/list_delete/item_use/client_score_set——op 码可按规律推算或在游戏态补抓。

**MessagePack 实证**（score_set {a=1,b='x',nested={1,2}}）：
`83 c401 61 01 c406 6e6573746564 92 01 02 c401 62 c401 78`
= fixmap(3) → bin8'a':1, bin8'nested':fixarray(2)[1,2], bin8'b':bin8'x'。字符串用 bin8（0xc4）家族。序列化 64K 上限（客户端文档）即指此编码。

## 2. 查询类子消息类型号（Msg.f3，实测）

| f3 | 子消息 | f4 body |
| --- | --- | --- |
| 2 | Commit | 见 §1（f4=repeated op + f2 desc） |
| 4 | ScoreInit（读） | f1=user_id(varint 原始字节), f2=repeated key |
| 10 | MoneyInit | {f1 user_id} |
| 20 | NameSearch | {f1 key, f2 name_substr} |
| 36 | QueryRankList | {f1 key, f3 起始名次, f4 结束名次, f5 str 'iscore'} |
| 38 | QueryRank（get_user_rank） | {f1 key, f2 str 'iscore'} |
| 48 | QueryList | {f1 user_id, f2 key, f3 limit} |
| 70 | QueryItem | {f1 user_id, f2 key} |

响应 f3：5=ScoreInitRes、49=QueryListRes、100=Result（f1=code, f2=reason）。请求-响应对齐：响应 f6 单调序号；客户端串行等待即可。

## 3. 权限矩阵（editor-pd lobby 态实测）

| API | 结果 |
| --- | --- |
| score_init 读（readonly/readwrite） | ✅ |
| commit：score_set/seti/sets/addi、list_add、name_new、item_add | ✅ 发送成功且服务端接受（commit Result code=0） |
| money_add / money_init / query_item / get_rank_list / get_user_rank | ❌ Nopermission(13)（lobby 态无权限；游戏态/授权地图内待测） |
| name_search | ❌ TableNotFound(1146)（该 key 的名字表未建） |
| message_send / message_query | ⚠️ 客户端签名校验失败未发送（"target_user_id参数不是合法整数"/"#3 table expected"——签名与文档不同，待试通） |

**重要修正（对 cloudvar-05 §4-②）**：读写的 target_map=ClientReadWriteMap 在 lobby 态落到与游戏同一空间（复读游戏侧写入的值一致）——target_map 是按**当前上下文地图**解析的逻辑名，lobby 态也有默认地图上下文。

## 4. 服务端独有 API 的可达性结论

- `score.*`（服务端）= 同一 LuaScore native 族的服务端封装（server 脚本库未镜像）。服务端跑在官方远端 host，**其 Entrance 流量本机不可见**——subscribe_channel/publish_message/world_data_* 的帧无法从客户端环境抓取。
- 可行补抓路径（未做）：在有 sce.s 全 API 的游戏态（tester 线上测试环境跑注入探针的地图）补抓 message_*/money/item/rank 帧；服务端独有消息需 host 侧 hook（出界）。
- publish/subscribe 频道 pub/sub 是官方"更底层"通道（ac 层支付/实名在用）——如需要，优先在服务端 lua 用官方 API 使用之（无需逆向）。

## 5. 自建客户端编码要点（完整）

```
登录：0x0001 {f1 'default', f4 0x1000040, f6 token_type, f7 token, f19 1, f20 ''}
读：  0xA000 Msg{f1 source, f2 'ClientReadWriteMap', f3 4, f4 {f1 uid_varint_bytes, f2 key...}}
写：  0xA000 Msg{f1 source, f2 'ClientReadWriteMap', f3 2, f4 Commit{repeated op, f2 desc}}
      op = f1 key + f2 op码 + f3 user_id + (f4 msgpack | f5 varint | f6 str | ...)
值编码：MessagePack（字符串 bin8/bin16 家族）
响应：信封 f1=0xA000 → body f3=100 Result{f1 code, f2 reason} / f3=5 ScoreInitRes / f3=49 QueryListRes
已知 code：0=成功 13=Nopermission 1146=TableNotFound（MySQL 原码）
```

## 6. entrance_client 表值写入实证（2026-08-24 补）

`set <key> <json>`（op0 score_set）与 `ladd <key> <json>`（op15 list_add）已实现并线上验证：
- 手写 JSON→MessagePack 编码器（examples/entrance_client.rs `mp_encode`），复刻观测编码（字符串 bin8/bin16 家族；弃用 rmp-serialize——其 str 走 fixstr 与观测不符）。
- `set bgd_probe_mp '{"a":1,"b":"x","nested":[1,2]}'` → code=0，ScoreInitRes 复读字节级一致（`83c40161...920102`）。
- `ladd bgd_probe_list '{"item":"sword","n":3}'` → code=0，QueryListRes 含原值 + 服务端时间戳（`2026-08-24 04:46:09`，list 项带落库时间）。
- PowerShell 调参注意：json 参数用单引号包裹（`'{...}'`），双引号需 `""` 转义（cmd 风格）会被 PS 吞掉。
