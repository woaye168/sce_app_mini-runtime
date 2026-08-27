# FINDINGS-lua-plus.md — lua-plus 分组研究发现（draft_v2，2026-08-27）

> 配套产物：`draft_v2/lua-plus.md`、`status-lua-plus.json`、`parsed/lua_plus_src_functions.json`（提取器 `extract_lua_plus.py` 产出，含逐函数签名+注解+行号）。**注：status/parsed/脚本类过程产物 2026-08-27 起统一在仓库 `test/loaded_modules_server/`。**

## 一、新发现

### LP1. lua_plus 源码↔运行时完全对齐（577/577）

从 `server_lua_plus\14\base\` 提取有效 `base.*` 函数定义 **577 个**（剔除 `--[[ ]]` 块注释后），与 dump `_G.base` 注册面（977 顶层字段、775 个一级函数）逐一比对：**全部命中**（含嵌套路径 `base.ui.proto.gamechatclient_send_message` / `base.ui.proto.component_event`）。lua_plus 层不存在"源码有但运行时动态裁剪"或"运行时动态注册未知函数"的情况（在 base.* 扁平面内）。

### LP2. lua_plus chunk 跑在独立 env（裸全局隔离）——本组最重要机制发现

证据链（全部为 dump 反面实锤）：
- `obj_check.lua` 把 32 个检查器赋为裸全局（`unit_check = base.obj_check.unit_check` 等），`_G` dump 中**全部 ABSENT**；
- `test.lua` 的裸全局函数 `debug_bp_confident`，`_G` dump ABSENT；
- 同包 `localization.lua` 用**显式** `_G.get_text = function...` 写法，`_G` dump **IN**；
- 对共享表的写入全部可见：`base.*` 577 函数、`string.find_end`、`table.pop_front/getn` 均在 dump 中出现。

结论：lua_plus 各模块 chunk 的 env 不是真实 `_G`；裸全局赋值落在 env 表，写共享表（base/string/table）与显式 `_G.` 赋值才进真实全局。**待确认**：游戏项目 chunk 是否与 lua_plus 同 env——若是，游戏侧可用 `unit_check` 等；若否，游戏侧裸用 `unit_check` 会 nil 报错。这直接影响 test_res002 代码写法（见待实测 Q0）。

### LP3. draft_v1 误列 14 个"幽灵函数"（已修正）

- `attack.lua` **全文**在 `--[[ ... ]]` 块注释中（8 个 `base.attack_*`，draft_v1 列了 7 个）；
- `unit.lua` 的 `unit_event_dispatch/notify/has/subscribe/unsubscribe` 5 个（"TODO：等触发器设计"块注释）；
- `player.lua` 的 `player_event_dispatch/notify` 2 个（"TODO:等 jj 设计"块注释）。

dump 全文检索这 14 个名字 **0 次出现**，双重确认从未注册。教训：从源码提取函数清单必须处理块注释（提取器 v2 已处理）。

### LP4. 触发器"任意对象"哨兵 = base.game 本尊

`global_variable.lua`：`base.any_unit / any_unit_id / any_player / any_skill / any_eff_param / any_mover / any_item` 全部赋值为 `base.game`（引擎 game 对象兼任通配符）。即触发器里"任意单位"不是特殊对象而是 game 对象本身，下游靠 `any_unit_check` 类检查器特判。

### LP5. 局内聊天 = 云变量频道消息（score.publish_message 官方用途实锤）

`gamechat.lua`：`base.gamechat_send_message` → `score.publish_message('@lib_gamechat_'..session_id, {src_user_name, text, time})`。这给出 `_G.score` 频道消息族（publish_message/subscribe_channel/subscribe_message）的明确语义锚点：跨端/跨玩家消息走云频道 pub/sub，不占游戏内 proto。

### LP6. ScoreCommitter 方法面 vs lua_plus 包装器有一处对不上（疑点）

dump `@common/base/tds_score` 的 `ScoreCommitter.prototype`（19 键）方法：`commit / set / add / clear / money_set / money_add / money_add_ex / money_cost / rank_add / rank_set / list_add / list_delete / list_modify / withlimit_add / name_new / add_finish_callback / ____constructor`——**无 `addi`**。而 `tds_score.lua` 的 `base.score_c_addi` 调 `c.addi{user_id, key, value}`。可能：① v14 包装器写的是旧版方法名，当前引擎已改名 `add`（则 score_c_addi 是死路径，调用即 nil 错误）；② `__index` 动态分发（TSTL 类不太可能）；③ dump 截断漏了（ScoreCommitter.prototype 19 键未见截断标记，可能性低）。**待实测 Q2**。

### LP7. 云变量读写分离的事务式模型（源码实锤）

- 读：`score.get / score.money_get{user_id=tostring(player:user_id()), key=...}` 同步返回 `error_code, data[], err_msg`；data 元素按类型取 `i_value / s_value / value`；
- 写：committer 事务——`score.get_commit(game_name)` 建请求（`'__MAIN_MAP__'` 特判为 nil=本地图），`c.set/add/money_*/rank_*/list_*` 追加操作，`c.commit()` 一次性提交；结果经 `base.last_commit_success / last_commit_error_code / last_commit_error_msg` 暴露给触发器（这三个是**动态字段**，无调用时 base 上不存在，dump 中未见属正常）。
- `user_id` 取自 `player:user_id()`（虚拟用户 Id，对应 lua_plus `base.player_user_id`）。

### LP8. @common/base/tds_score = tds_score 族 TSTL 类注册表（36 类全清单已定稿）

36 顶层类 = 21 参数类（`_key`/`_user_id`/`_timetype`/`_limit` 描述符属性）+ 10 纯数据载体类（prototype 仅 3 键）+ ScoreCommitter + 周期限购继承族（ScoreAddWithLimitParam → Hour/Day/Week/Month/Year 5 子类）。109 处截断全部是描述符 get/set 访问器，属性名完整。数据载体类 prototype 为全场单例（跨 6 键共享内联）。

## 二、云变量 / tds_score 待实测问题清单（交主控安排实测）

| # | 问题 | 实测建议 |
| --- | --- | --- |
| Q0 | 游戏 chunk 与 lua_plus 是否同 env：游戏侧裸写 `unit_check(u)` 是否可用？ | test_res002 服务端探针直接调 `unit_check`，对比 `base.obj_check.unit_check` |
| Q1 | 服务端 `score.get` 底层通道：是否与客户端 Entrance 0xA000 云变量段同协议同服务？读写计次/限频规则？`score.check_permission` 的权限面？ | mini-runtime B 模式起局 + 抓包（对照 cloudvar-04 协议）；直接调 `score.get{user_id=...,key=...}` 观察返回结构 |
| Q2 | `base.score_c_addi`→`c.addi` 是否死路径（LP6）？ | 探针：建 committer 后 `pcall(function() c.addi{...} end)` 与 `c.add{...}` 对照 |
| Q3 | `score_c_commit` 的返回约定：源码 `local ec, j, err_msg = c.commit()` 后把 **j 赋给 last_commit_error_code**（第二返回值是错误码还是数据？）成功时 j 是什么？ | 实测一次真实提交，打印三返回值 |
| Q4 | `score.get_commit(nil)`（本地图）与指定图名的行为差异；跨图写是否可行 | 两图名各试一次 |
| Q5 | `world_data_get / world_list_query(_by_uuid)`（world 域）vs `list_query`（图域？）的语义与权限差异 | 分别调用对比返回 |
| Q6 | 周期限购族：`withlimit_add` + `withlimit_query` 与 ScoreAddWith{Hour..Year}LimitParam 的对应关系、限额重置时点 | 写读回环实测 |
| Q7 | 频道消息：`publish_message` 的频道名命名规则（`@lib_gamechat_<session>` 是约定还是必须）、订阅方（客户端？其他玩家服务端？）、消息体大小/频率上限 | 双端订阅发布回环 |
| Q8 | `message_send/query/modify_read/delete`（站内信式云消息）与频道消息的关系 | 调用看返回结构 |
| Q9 | `database_type` 返回值与 `@tds_score/use_mysql` 的关系（是否存在 MySQL 后端可选） | 直接调 `score.database_type()` |
| Q10 | `is_old_player` / `test_cloud_value` 语义与用途 | 直接调用观察 |
| Q11 | 读接口 `score.get` 是否必须在协程内（对照 base.backend.url 的协程约束）？还是会同步阻塞 | 协程内/外各调一次 |

## 三、过程产物

| 文件 | 说明 |
| --- | --- |
| `extract_lua_plus.py` | lua_plus 源码函数提取器 v2（剔除块注释、采集函数后置 `---@` 注解、与 `_G.base` dump 交叉核对） |
| `gen_draft_v2_lua_plus.py` | draft_v2 文档组装器（注解读 parsed/lua_plus_src_functions.json + 内置模块注记） |
| `parsed/lua_plus_src_functions.json` | 42 文件全部函数定义（名/签名/行号/注解），可复用于其他组或复查 |
