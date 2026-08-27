# FINDINGS.md — loaded_modules_server 研究过程发现（v2 迭代）

> 硬要求落盘：研究过程中的新发现、对前序结论的修正、工具知识，即时记录于此。
> 按时间倒序追加，每条标注日期（2026-08-27 起）。
> **目录约定（2026-08-27 用户定）**：本目录只放文档与输入材料；全部过程工具/中间 JSON（`*.py` / `parsed/` / `status-*.json`）在仓库 `test/loaded_modules_server/`。

## 2026-08-27 解析器建成 & 重大归属修正

### F1. dump 真实格式（已实锤，修正 methodology.md 的"非纯净 JSON 碎片"说法）

- 全文件 = **一个 JSON 字符串字面量**（首尾 `"`），内容是被转义的 JSON 对象文本。
- **序列化器只转义引号不转义反斜杠**：内容中的 `\` 原样写出，导致 `\"` 二义（转义引号 vs 内容反斜杠+结束引号）。**消歧规则**：`\"` 后（跳空白）紧跟 `,` `}` `]` `:` ⇒ 是"内容 `\` + 字符串结束"。
- 混有 Lua 十进制转义（`\13`=CR）与原始换行（续行）。GBK 乱码区显示为 `■■■■`。
- 健壮解析 = 两级容错解码（见 `test/loaded_modules_server/parse_valuetree.py`）：外层手工反转义 → 内层容错 JSON 递归解析（字符串接受十进制转义/原始控制字符/`\"` 结构符消歧）。
- 解析结果：**374 个顶层键**全部解析成功（非此前 regex 提取的 363！）。

### F2. 前序研究的 11 个漏键（重大）

regex `\\"(@[^\\"]+)\\"` 只认 `@` 前缀，漏掉了 **11 个无 `@` 前缀的顶层键**：
`_G`、`coroutine`、`debug`、`io`、`lni`、`math`、`os`、`package`、`string`、`table`、`utf8`
（Lua 标准库 + 全局表，都在 package.loaded 里）。其中 **`_G` 是全局表序列化，是本 dump 最大的情报源**（见 F3）。
216 键口径不变（`@` 前缀非 `@p_55a3`），stdlib 11 键作为范围外附录处理。

### F3. 归属陷阱修正（推翻提示词 §dump 结构的既有结论）

- 提示词称"`backend`/`room` 唯一出现处在 `@common/base/position` 的值内联展开里"——**错误**。
  真相：`@common/base/position` 是 hash 序中 `_G` 之前的最后一个 `@` 键，前序 regex 分析把紧随其后的 `_G` 值区域误归给它。
- 实测（全量解析）：`@common/base/position` 值极小（仅 `ScreenPos` 类）；**`backend` 全文出现 1 次，路径为 `_G.base.backend`**（不是提示词说的 `base.game.backend`）。
- `_G.base` 共 **977 个顶层字段**（含 lua_plus 扁平函数全集、Event* 类族、backend/room/select_hero 等）。
- `_G.base.backend` = 12 函数：`init_game_config / parse_detail / query_admin_email / query_score_log / query_score_log_by_user / query_score_log_detail / query_user_email / query_user_payment / send_email / set_log_sample_per_sessions / stay / url`（与 url_demo.lua 的 `base.backend.url` 用法吻合）。
- `_G.base.room` = 3 函数：`find_game_list / find_room / sync_room_info`。
- `_G.base.select_hero` = 8 函数：`on_click / on_init / on_random / on_select / op_click / op_init / op_select / op_stop`。
- `_G.base.game` = 130 键（引擎 game 对象方法面：`end_game / get_env / event_* / create_scene_copy ...`），**不含 backend**。
- 结论：common-base-game 组（`@common/base/game/*` 15 键）的运行时证据应优先从 `_G.base.game.*` 对照；`backend/room/select_hero` 归 `_G.base.*` 直下（语义宿主 = base 全局，非任何 @ 键）。

### F4. 值形态统计（227 个非 @p_55a3 键）

- `true` ×148（已加载但无表导出）/ table ×77 / `"<function>"` ×2。
- 含 `<max depth exceeded>` 截断的键 ×12（文档中必须标注"字段不全"）。
- 含 TSTL 类结构（`prototype`/`____super`）的键 ×44。
- 跨键重复共享内联表 117 个（`test/loaded_modules_server/parsed/shared_tables.json`），主要用于识别 `_G`/`lualib_bundle`/`lua_declare` 的全场内联。

### F5. tds_score 顶层键实锤

- `@common/base/tds_score` = table，**116 个函数**（详情见 `test/loaded_modules_server/parsed/fields/common__base__tds_score.json`）——是 tds_score 族中最有料的键。
- `@tds_score`、`@tds_score/{new_base_score,score,tds_score,use_mysql}` 值均为 `true`（无表导出，实现引擎侧）。
- `_G.score` = 28 函数（`get / get_rank_list / list_query / world_data_get / subscribe_channel / message_*` 等，云数据/排行榜 API 面）。

## 工具产物

> 2026-08-27 起，全部过程工具/中间 JSON 迁至仓库 `test/loaded_modules_server/`（不污染文档目录）。

| 文件 | 说明 |
| --- | --- |
| `test/loaded_modules_server/parse_valuetree.py` | dump 值树解析器（两级容错解码 + 字段树 + 全量成员索引 + 共享表识别） |
| `test/loaded_modules_server/parsed/keys_index.json` | 227 键索引（分组/来源/值形态/函数数/截断路径/类数） |
| `test/loaded_modules_server/parsed/fields/<key>.json` | 每键字段树（采样结构）+ members（全量函数路径/类/截断/子表） |
| `test/loaded_modules_server/parsed/shared_tables.json` | 跨键共享表内容哈希 → 出现键列表 |

## 2026-08-27 draft_v2 分组研究完成（216/216 定稿）

分组发现详见各分组 FINDINGS 分片（`FINDINGS-common-base.md` / `FINDINGS-lua-plus.md` / `FINDINGS-map-libs.md` / `FINDINGS-game-smallcard.md`）。跨组最重要的结论：

1. **引擎内嵌接管模式**（G1）：script 包同名文件 `error_info/game_result/item/player/scene/select_hero/unit.lua` 在服务端被引擎变体整体接管——dump 只有 `@common/base/game/*` 键，无 `@common/base/game_result` 等。拿真实源码需走引擎内嵌包提取（pak-extract 路线）。
2. **lua_plus chunk 独立 env**（LP2）：裸全局赋值不进真实 `_G`（32 个 `*_check` 检查器 dump 全缺席），写共享表/显式 `_G.` 才进。待实测 Q0：游戏 chunk 是否同 env。
3. **幽灵函数教训**（LP3）：源码提取必须剔除 `--[[ ]]` 块注释（attack.lua 全文在块注释中，draft_v1 误列 14 个函数）。
4. **云变量读写分离事务模型**（LP7）：读 `score.get` 同步返回 `error_code, data[], err_msg`；写走 committer 事务（`get_commit` → `set/add/money_*` → `commit`），结果经 `base.last_commit_*` 动态字段暴露。
5. **局内聊天=云频道 pub/sub**（LP5）：`gamechat_send_message` → `score.publish_message('@lib_gamechat_'..session_id, ...)`。
6. **lib_game_options 三模块是服务端云数据+Redis 通道完整范例**（M6）：`base.s.subscribe_message('Redis.Server2Host.Channel...')` 命名规范、score_init/get_commit/commit 用法、detection.check_text 敏感词。
7. **共享表哈希互证**（F4/M3）：`lua_declare` dump 值 730 函数多为 common 库共享类内联（20+ 条跨键同哈希记录），归属已修正；服务端版 lua_declare 是未分发文件。
8. **待实测问题清单**：lua-plus 组 Q0~Q11（FINDINGS-lua-plus.md）+ game/smallcard 组 8 项（FINDINGS-game-smallcard.md），实测阶段逐项执行回写【实测】。

## 2026-08-27 实测批次 1（编辑器 PIE，test_res002，探针 `.bgd/src/server/test/probe_server_apis.lua`）

【实测】已确认：
- **Q0**：游戏 chunk 裸全局 `unit_check` = **nil**（游戏 chunk 与 lua_plus 不同 env 或同样隔离）；`base.obj_check.unit_check` = function（经共享表可用）。
- **`base.s` 面**：score_init/money_init/world_data_init/list_query/get_commit/subscribe_message/publish_message/stat_upload/test_cloud_value 全部 function 存在。
- **`backend.url` 不强制协程**：事件处理器（主线程）直调 GET 成功返回真实 HTTP 响应（`{code=0,message,data={timestamp,method,clientIp,params}}`，返回值为**字符串**形态 JSON）。协程内同样可用（url_demo 范式非必需但推荐——阻塞式调用在协程内不卡主帧待进一步确认）。
- **`score.get` 必须协程内**：主线程直调报 `common/base/co.lua:56: cannot wrap coroutine by main thread!!!`（pcall 可捕获）。
- **坑（重要）**：`backend.url` 传 `(url, table)` 二参形态触发**协程级错误，pcall 无法拦截**——错误对象 table 穿透 TRY 直达 co.lua:28 的 async 错误处理器，**同一条 async 链后续探针全部不执行**。教训：风险探针必须各自独立 async。
- 日志：`backend.url` 返回含 `clientIp 106.14.95.227`（真实外网请求，编辑器 PIE 环境）。

## 2026-08-27 实测批次 2（编辑器 PIE，探针全 async 隔离，user_id=38672742）

【实测】签名（由引擎错误消息/行为直接获得，error 带层级标注，中文原文）：
- `backend.query_score_log(map_name, user_id, sub_system, key, begin_time, end_time, callback)` —— 7 参
- `backend.query_score_log_by_user(map_name, user_id, begin_time, end_time, callback)` —— 5 参
- `backend.query_score_log_detail(map_name, request_id, inner_id, callback)` —— 4 参
- `backend.query_user_email / query_admin_email` —— 需 callback 参（经 common/base/isolation.lua 校验层，:103/:92）
- `backend.stay` —— 恰好 4 参（"参数不是4个"）
- `backend.send_email` —— 内部对 start_time 做算术（时间相关参数）
- `backend.init_game_config()` 无参 OK 无返回；`parse_detail({})` OK 返回表 `[{}]`；`set_log_sample_per_sessions(1)` OK
- `backend.query_user_payment({map_name, user_id})` OK 返回 `0`（无支付记录）
- `backend.url('POST', url)` → 实际以 **GET** 命中目标（二参被忽略或形参序不对）；`url(url, 'POST')` → OK 但**无任何返回值**；`url(url, {method=...})` → 协程级错误（pcall 不拦截）。**POST 支持仍无实锤，url 可能仅支持 GET**

【实测】room：
- `room.sync_room_info({room_code, room_cur_number})` OK 无返回（上报/同步语义，fire-and-forget）
- `room.find_game_list({})` → 返回二值 `({}, 3)`
- `room.find_room({game_name, room_mode, extra={tag}})` → 引擎侧 log `find_room error` + 返回 **-2**（错误码；编辑器调试环境无房可找）
- **room 实现是引擎内嵌 Lua**：traceback 显示 `common/base/room/init.lua:141/147`（带 find_room_res 调试日志）——api-13 解密包中无此文件

【实测】score / 云变量（重大）：
- **写-读回环走通**：`score.get_commit()` → `c.add{user_id, key, value=1}` → `c.commit()` 返回 `(0, {})`；随后 `score.get{user_id, key}` 返回 `(0, [{user_id=38672742, key='__probe__', value=1, raw_i_value=1}])`——**服务端云变量直写实锤可用**
- `score.database_type()` → `"mysql"`（Q9 解答：默认即 MySQL 后端）
- `score.check_permission()` → `("p_55a3", "old")` 二返回（图名 + 权限档?）
- `committer` 方法面实测：`add/set/commit/name_new/money_add`=function，**`addi`=nil、`rank_add`=nil**——Q2 实锤：`base.score_c_addi` 与 rank_add 相关包装器是**死路径**（调用即 nil 错误），lua_plus v14 包装器与当前引擎版本漂移
- Q3：`c.commit()` 成功时返回 `(0, {})`——第二返回是**数据表不是错误码**；lua_plus 的 score_c_commit 把它赋给 last_commit_error_code 语义存疑
- `last_commit_success/error_code/error_msg` 在裸 score.get_commit 流程后**仍为 nil**——这三个动态字段由 lua_plus 包装器设置，非引擎行为（FINDINGS LP7 措辞已修正）
- `score.get_rank_list({})` → `tds_score/use_mysql.lua:530: field 'get_rank_list' is nil`——**MySQL 后端未实现 get_rank_list**（版本漂移实锤）
- `score.is_old_player(uid字符串)` → ERR `tds_score/score.lua:483: index a string value (local 'params')`——参数须为 table
- `score.test_cloud_value()` → 需 params table（`tds_score/score.lua:490`）
- `score.list_query({})` → "key参数不是字符串"；`score.name_exist({})` → "name_substr参数不是字符串"（均按位置/命名参校验）
- **tds_score 实现是引擎内嵌 Lua**（`tds_score/score.lua`、`tds_score/use_mysql.lua`，api-13 无此文件；pak-extract 可挖）

【实测】base.game：
- `base.game.set('k','v1')` + `base.game.get('k')` → `'v1'` 回环成功（至少局内会话级 KV）
- `base.game.get_env()` → `"pd"`；`get_session_id()` → 7678695568834625539（大整数）；`get_server_tag()` 无参 OK 无返回（参数待究）

【坑】GO 隔离验证有效：首个探针（url table 形态）协程级崩溃不再团灭其他探针。

## 2026-08-27 实测批次 3+4（编辑器 PIE，收尾）

【实测】backend：
- `url(url,'POST','x=1')` 三参 → OK 无返回；`url(url,'POST',cb)` → 协程级错误（错误对象=function，pcall 不拦截）。**结论：url 仅 GET 单参形态可靠带响应；多参静默无返回，回调形态崩溃**。
- `stay` 签名进一步收窄：**4 参，1/3 参=table、2/4 参=integer**（"2,4参数不是integer"）。
- `query_user_email({map_name,user_id}, cb)` / `query_admin_email({map_name}, cb)` → OK，**回调收到 `"UnknownMySQLException"`**——编辑器调试环境 MySQL 侧无表，同时实锤 backend 查询系底层走 MySQL。
- `query_score_log` 7 参带回调 → OK，回调收 `[{}]`。
- `send_email` 需 start_time 字段（未走通）。

【实测】score 余量：
- `test_cloud_value{user_id, key}` → `(0, 1)` OK。
- `is_old_player{user_id}` → `(0, true)` OK（参数必须表）。
- `list_query{key=}` → `(0, {})` OK；位置参报错（params 必须表）。
- `name_exist`：`{name_substr=}` 与位置字符串**均**报"name_substr参数不是字符串"——参数形态未走通（疑 `(db, name_substr)` 双位置参，未再试）。
- `world_data_get/world_list_query`：`{worldId=1}` 仍报"worldId参数不是合法整数"——疑需真实世界 ID 或键名不同（未走通）。
- Q4：`get_commit('__MAIN_MAP__')` 字面量与 `get_commit('不存在图')` 均报 `no permission to operator game score[...]`（tds_score/score.lua:112）——**跨图/特殊名写有权限闸门**；本地图必须无参（lua_plus 包装器把 `'__MAIN_MAP__'` 转 nil 的原因实锤）。⚠️ 批次5 修正：`'__MAIN_MAP__'` 字面量本就是错误用法，正确是不带引号的全局 `__MAIN_MAP__`（值=主图名），详见批次5。
- Q7 **频道回环走通**：`subscribe_message(chan, {ok=,error=,timeout=})` → true（callback 必须是**表**不是函数）；`publish_message(chan, {text,...})` → ok 回调收 `{message={...}}`。
- Q8：`message_send{key=}` 需 `target_user_id` 整数；`message_query{key=}` → `(0, {})` OK。

【实测】协程模型通用规则（归纳）：
1. **必须协程内**：`score.*` 读系（get/money_get/list_query...）——内部 `co.wrap` 当前协程，主线程报 "cannot wrap coroutine by main thread"。
2. **不强制协程**：`backend.url`（GET）主线程可用（疑似同步阻塞）；协程内不阻塞主帧（引擎回调恢复）。
3. **fire-and-forget 同步**：`room.sync_room_info`、`score.publish_message`、`backend.init_game_config` 等无返回型。
4. **回调型**：`backend.query_*`（7/5/4 参末位 callback）、`score.subscribe_message`（callback 表 {ok,error,timeout}）。
5. **坑**：引擎 API 错参可能抛**非 string 错误对象**（table/function），穿透 pcall 直达 co.async 处理器——探针/业务代码应对每个高风险调用独立 async 隔离。

## 2026-08-27 实测批次 5（用户指正吸收：`__MAIN_MAP__` 用法）

【实测】
- `__MAIN_MAP__` = **string 全局变量，值 = 主图名 `'p_55a3'`**；`__GAME_ID__` 同值；`_G.__MAIN_MAP__` 运行时存在，且 **dump 中本就有**（`_G.__MAIN_MAP__` = scalar `"p_55a3"`，此前 _G.json 未列出是采样策略缺陷——已修解析器：叶子全收、仅重量级子表采样，重跑后可见）。语料佐证：`require('@'..__MAIN_MAP__..'.obj.constant')`（defaultui）、`map_name = __MAIN_MAP__`（lib_equipment 服务端）等 30+ 处——**正确用法是不带引号的全局变量**。
- **修正批次 3 的 Q4 措辞**：`get_commit('__MAIN_MAP__')` 字面量字符串是**错误用法**（引擎小写化为 `__main_map__`，非真实图名 → 权限错误）。正确：`get_commit()` 无参（本地图）/ `get_commit(__MAIN_MAP__)` 全局 / `get_commit('p_55a3')` 显式真实图名——后两种均实测 OK（commit 返回 0）。
- lua_plus 包装器里的 `'__MAIN_MAP__'` 字符串比较是**触发器 UI 层的哨兵约定**（触编传字符串，包装器转 nil），与 Lua 层的全局变量不冲突——两层语境已区分。
- `name_exist` 双位置参 `(db, substr)` 与 `(__MAIN_MAP__, substr)` 仍报"name_substr参数不是字符串"——参数形态仍未走通（可能需表内特定键名，留待 pak-extract 拿 tds_score/score.lua 源码后定）。
- 坑（探针写法）：`base.wait` 回调跑在主线程，里面直接调 `score.get` 会因协程约束静默失败——wait 回调里要再套 async。

## 2026-08-27 独立审计（主控对分组产物的复核）

- `audit_coverage.py`：216 键在 draft_v2 五分组文档中**逐一在列，零缺漏**（lua-plus 44 节/47 键、map-libs 34 节/42 键均为合并节，键名全列出）。
- `audit_luaplus_577.py`：独立复核 lua-plus 组核心声明"577/577 base.* 函数与 _G.base dump 全命中"——用 `parsed/lua_plus_src_functions.json` 粗提 577 名与 `_G.json` members（4825 个 base.* 函数路径）比对，**未命中 0**，声明属实。（两个审计脚本在 `test/loaded_modules_server/`）
- 解析器采样策略修正：原"超 40 子键即截断采样"会丢标量/函数叶子（批次5 发现 `_G.__MAIN_MAP__` 被漏列）；已改为**叶子全收、仅重量级子表采样**，重跑后 `_G` 66 键全列（含 `__MAIN_MAP__="p_55a3"` dump 实锤）。
- 00-INDEX 值形态统计修正为 216 键口径：`true`×148 / table×67 / function×1（此前误写 227 键口径）。
- 复核 map-libs M2 声明"`_G.poi.ai` ≡ `@lib_common_ai/class/new`（9 函数全同）"：独立比对 `test/loaded_modules_server/parsed/fields/_G.json` 与 `lib_common_ai__class__new.json` 的 members，**9 函数逐一相同**，声明属实。
- 00-INDEX 补齐 stdlib 附录实内容：StateGame 的 coroutine 含引擎扩展（async/sleep/promise/as_promise/multi_promise/co_wrap 等 10 个，即 base.co 底层）、string.find_end、table.contain/getn/pop_front、os.time_ms、package 仅暴露 searchpath、lni 值为函数本体【dump 实锤】。
