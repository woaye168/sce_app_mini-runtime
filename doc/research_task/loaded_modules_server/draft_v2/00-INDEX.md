# 服务端 package.loaded 模块全景（api-13 / StateGame）— draft_v2

> 研究对象：`../loaded_module_server_package_loaded.txt`——服务端（StateGame）Lua 虚拟机 `package.loaded` 运行时 dump（16.9MB，脱机运行时抓自 test_res002 / p_55a3）。
> **v2 相对 v1 的升级**：全量值树解析（解析器 `test/loaded_modules_server/parse_valuetree.py`）→ 每键字段树/全量函数路径/类结构/截断点；共享内联表哈希归属修正；每字段置信级标注。
> 本目录文档为**工作版**，经用户审核定稿后才复制到 `doc/research/loaded_modules_server/`。

## dump 是什么（v2 已修正结论）

服务端 Lua 状态 `package.loaded` 的序列化结果。**真实格式**：一个 JSON 字符串字面量包裹的 JSON 对象文本；序列化器只转义引号不转义反斜杠（`\"` 二义需按后继结构符消歧），混有 Lua 十进制转义（`\13`）与原始换行（详见 `../FINDINGS.md` F1）。

- **374 个顶层键**（非此前 regex 提取的 363）：363 个 `@` 键 + 11 个无前缀键（`_G` + Lua 标准库 10 个）。
- 本任务范围 = `@` 前缀且非 `@p_55a3/` 的 **216 键**；`@p_55a3/*`（147 键，游戏项目自身）出范围；stdlib 11 键作附录。
- 值形态（216 键）：`true` ×148（已加载无表导出）/ table ×67 / `"<function>"` ×1。
- 特殊值标记：`"<function>"` / `"<cyclic reference>"` / `"<max depth exceeded>"`（截断处字段不全，文档已逐处标注；截断集中于 TSTL `_descriptors` 属性访问器，不损 API 面）。
- **值内联展开**：共享引用按值内联。跨键同内容哈希（`test/loaded_modules_server/parsed/shared_tables.json`，117 条）识别共享表；`_G` 值内联了半个全局空间（`base` 977 字段、Event* 类族、`backend/room/select_hero`、`score` 28 函数、`poi.*` AI 设施等）。

## 分组索引（216 键全覆盖）

| 文档 | 分组 | 键数 | 有源码 | 无源码 | 说明 |
| --- | --- | --- | --- | --- | --- |
| [common-base.md](common-base.md) | script 包 · common 库 base 层 | 93 | 60 | 33 | 游戏脚本运行库本体；含 8 转发桩、19 个引擎内嵌 TSTL 类模块 |
| [common-base-game.md](common-base-game.md) | common 库 base/game 子层 | 15 | 0 | 15 | **全部引擎内嵌**（接管 script 包同名文件）；证据主源 `_G.base.game`（130 键） |
| [lua-plus.md](lua-plus.md) | server_lua_plus 包 v14 + tds_score | 47 | 42 | 5 | 触发器 API 层，577 个 `base.*` 扁平函数源码↔运行时 100% 对齐 |
| [map-libs.md](map-libs.md) | defaultui / global_default / lib_common_ai / lib_control / lib_game_options / lib_common_sounds | 42 | 19 | 23 | 官方地图库服务端部分；lib_common_ai AI 实现未分发 |
| [smallcard-libs.md](smallcard-libs.md) | smallcard_get_items / inventory / mail | 19 | 13 | 6 | 小卡库服务端部分；proto/mail 等协议模块不下发 |

合计 216 键 = 134 有源码 + 82 无源码。核验状态看板：`../PROGRESS.md`。

## 范围外附录（11 键）

`_G`（全局表，最大情报源）、`coroutine / debug / io / lni / math / os / package / string / table / utf8`（Lua 标准库）。`_G` 的字段树在 `test/loaded_modules_server/parsed/fields/_G.json`（5121 函数路径 / 373 类）。

**stdlib 在 StateGame 的运行时面**（全部【dump 实锤】；粗体为引擎扩展，超出 Lua 5.4 标准库）：

| 键 | dump 函数面 |
| --- | --- |
| `coroutine` | create/resume/yield/status/wrap/running/isyieldable/close + **async / async_next / sleep / sleep_one_frame / call / will_async / co_wrap / promise / as_promise / multi_promise**（引擎协程扩展，即 `base.co` 的底层） |
| `string` | 标准族 + **find_end**（lua_plus 注册） |
| `table` | 标准族（无 unpack 外变动）+ **contain / getn / pop_front**（引擎/lua_plus 扩展） |
| `os` | clock/date/difftime/time + **time_ms** |
| `debug` | debug/getinfo/getmetatable/getuservalue/setcstacklimit/setmetatable/traceback（裁剪后子集） |
| `io` | close/flush/lines/open/read/type/write |
| `math` / `utf8` / `package` | math 标准族 + `math.type`；utf8 标准族；package 仅暴露 `searchpath` 函数（裁剪） |
| `lni` | 值 = `"<function>"`（lni 加载器本体；`base.game.lni = require 'lni_loader'` 经 preload 桩到 client_base 实现） |
| `_G` | 66 顶层键全列（含 `__MAIN_MAP__="p_55a3"`、`base` 977 字段、`score` 28 函数、`poi.*`、`bgd_api`、`bgd_const`、`present`、`throw/try/try_wrap/class/instance_of` 等框架全局） |

## 模块来源与加载机制（关键结论，v2 修订）

1. **`@common` = script 包 v199**：`base/init.lua` 建立 `_G.base`、包装 game_events、`base.game` 由引擎侧扩展。base 层三类来源：✅ 包内完整实现 / 🔀 转发桩（8 个：base 侧 6 + preload 侧 2，实现在 client_base 库，桩的 dump 值即实现侧导出）/ ⚠️ 引擎内嵌（19 个 TSTL 类模块 + `game/*` 15 个，api-13 全语料无 Lua 源码）。
2. **引擎内嵌接管**：script 包同名文件（error_info/game_result/item/player/scene/select_hero/unit.lua）在服务端被引擎变体整体接管（FINDINGS G1），服务端权威 API 面以 dump 为准。
3. **`@lua_plus` = server_lua_plus 包 v14**：577 个 `base.<领域>_<动作>` 扁平函数带中文注解；**chunk 跑独立 env**——裸全局不进 `_G`（FINDINGS LP2）。`base/` 与 `lp/base/` 内容相同。
4. **`@tds_score`**：TDS 排行榜/积分库引擎侧实现。类注册表在 `@common/base/tds_score`（36 类 + ScoreCommitter 16 方法）；读取面在 `_G.score`（28 函数）；云数据读写分离事务模型（FINDINGS LP7）。
5. **官方地图库**：入口运行时返回 = `{[包名]=命名空间} + ____module/____return 合并`（FINDINGS M1/G6）；`ui/script` 来源键在服务端 dump 几乎全为 true（M5）；lib_common_ai 运行时访问点是全局 `poi.*`（M2）。
6. **`@p_55a3`**（出范围）：游戏项目 test_res002 自身代码，v1 的 project-bgd/project-obj 两份存档保留在 `../draft_v1/`。

## 置信级标注约定

| 级别 | 含义 |
| --- | --- |
| 【实测】 | 实机调试验证（注明实测环境），最高置信 |
| 【dump 实锤】 | 值树中直接出现（名字确定；签名不知时附调用点实参形态） |
| 【源码实锤】 | 源码中直接提取（签名+注释确定；已剔除块注释幽灵函数） |
| 【反查推测】 | 语料调用点反查得到的形态 |
| 【语义推测】 | 仅按命名/同类类比推测 |

当前 216 键主置信级分布：源码实锤 80 / 源码+dump 双证 45 / dump 实锤 39 / 反查推测 18 / 语义推测 32 / 其他混合 2（明细见 `../PROGRESS.md` 与各 status-*.json）。

## 过程产物

> 过程工具与中间数据（解析器/脚本/解析 JSON/状态 JSON）统一放在**仓库 `test/loaded_modules_server/`**（不污染文档目录）；文档与输入材料留在本目录。

| 文件 | 位置 | 说明 |
| --- | --- | --- |
| `parse_valuetree.py` | `test/loaded_modules_server/` | dump 值树解析器（两级容错解码） |
| `parsed/keys_index.json` / `parsed/fields/*.json` / `parsed/shared_tables.json` / `parsed/lua_plus_src_functions.json` | `test/loaded_modules_server/parsed/` | 键索引 / 每键字段树+全量成员 / 共享表哈希 / lua_plus 源码函数提取 |
| `status-*.json` / `merge_status.py` / `gen_progress.py` / `audit_*.py` | `test/loaded_modules_server/` | 分组状态 / 看板与审计脚本 |
| `PROGRESS.md` | 本目录 | 216 键核验看板 |
| `FINDINGS*.md` | 本目录 | 过程发现（主文件 + 4 个分组分片），含待实测问题清单 |
| `module_source_map.json` / `methodology.md` | 本目录 | v1 键→源码映射 / v1 方法论（部分结论已被 FINDINGS F1~F3 修正） |

## 覆盖报告

- 216/216 键已解析值树、已定稿（`test/loaded_modules_server/merge_status.py` 校验：缺 0 / 多 0）。
- 截断键 12 个（截断路径已在各文档标注"字段不全"；几乎全为 `_descriptors` 访问器）。
- **服务端能力实测已完成**（2026-08-27 编辑器 PIE × 4 批次，探针 `test_res002/.bgd/src/server/test/probe_server_apis.lua`，结论已回写各分组文档【实测】标注）：
  - **云变量服务端直读直写走通**：`score.get_commit → c.add → c.commit → score.get` 回环实锤；写须协程内；`database_type()="mysql"`；跨图写有权限闸门；`addi/rank_add/get_rank_list` 等为版本漂移死路径/缺失接口。
  - **频道消息 pub/sub 走通**：`subscribe_message(chan, {ok,error,timeout})` + `publish_message` 回环实锤。
  - **backend**：`url` 仅 GET 单参可靠（不强制协程）；query 系签名全由引擎错误消息实锤；底层走 MySQL（UnknownMySQLException）。
  - **room**：三函数实测（find_room 返 -2、find_game_list 返二值、sync_room_info fire-and-forget）；实现为引擎内嵌 `common/base/room/init.lua`。
  - **协程模型通用规则**已归纳（FINDINGS.md 批次 3+4 节）。
- 未走通项（如实标注）：`url` POST 无实锤、`name_exist`/`world_*` 参数形态、`stay`/`send_email` 字段细节——见 FINDINGS.md 批次 3+4。
