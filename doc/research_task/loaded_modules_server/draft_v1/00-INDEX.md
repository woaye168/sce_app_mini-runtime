# 服务端 package.loaded 模块全景（api-13 / StateGame）

> 研究对象：`doc/research_task/loaded_module_server_package_loaded.txt`——服务端（StateGame）Lua 虚拟机 `package.loaded` 的运行时 dump（16.9MB，脱机运行时抓自 test_res002 / p_55a3）。
> 本文档覆盖 dump 中**全部 363 个已加载模块键**，逐模块给出源码出处或推测标注。
> 过程产物（提取工具、映射表、覆盖报告）在 `doc/research_task/loaded_modules_server/`。

## dump 是什么

服务端 Lua 状态里 `package.loaded` 的序列化结果。键即模块名、值即模块表（巨型数据模块的值把文件撑到了 16.9MB，含技能/ Buff 等编辑器数据）。键有两种形态：

| 形态 | 含义 | 示例 |
| --- | --- | --- |
| `@<库>/<路径>` | 跨库引用（`@` 是引擎的跨包模块机制） | `@common/base/unit`、`@lua_plus/base/base_lua_plus/skill` |
| `@p_55a3/<路径>` | 游戏项目自身（ProjectName=p_55a3，即 test_res002） | `@p_55a3/bgd_game_server/server/skillsystem` |

## 分组索引（363 模块全覆盖）

| 文档 | 分组 | 模块数 | 有源码 | 说明 |
| --- | --- | --- | --- | --- |
| [common-base.md](common-base.md) | script 包 · common 库 base 层 | 93 | 60 | 游戏脚本运行库本体 |
| [common-base-game.md](common-base-game.md) | common 库 base/game 子层 | 15 | 0 | **全部引擎内嵌**，附 `base.game.*` 组级反查 |
| [lua-plus.md](lua-plus.md) | server_lua_plus 包 + tds_score | 47 | 42 | 触发器（lua+）API 层，注解最全 |
| [map-libs.md](map-libs.md) | defaultui / global_default / lib_common_ai / lib_control / lib_game_options / lib_common_sounds | 42 | 19 | 官方地图库服务端部分 |
| [smallcard-libs.md](smallcard-libs.md) | smallcard_get_items / inventory / mail | 19 | 13 | 小卡库服务端部分 |
| [project-bgd.md](project-bgd.md) | p_55a3 的 bgd_game_server / bgd_libs_server / 入口 | 78 | 78 | 本项目业务代码（源码在 .bgd/src、.bgd/libs） |
| [project-obj.md](project-obj.md) | p_55a3 的 obj/scene 数据模块 | 69 | 69 | 编辑器生成的物编数据（data/dict/init） |

合计：363 模块，281 个有直接源码，82 个无源码（已全部给出调用点反查和/或语义推测）。

## 模块来源与加载机制（关键结论）

1. **`@common` = script 包 v199**（`Res/_m/script/199/script/common/`）。运行时根名 `common`，dump 键加 `@` 前缀。其 `base/init.lua` 是 base 层引导：建立 `_G.base` 全局、包装 `game_events` 回调（xpcall 保护）、`base.game.lni = require 'lni_loader'`（C++ 实现）、`base.tsc = require 'base.lualib_bundle'`（TypeScriptToLua 编译运行库，纯 TS 生成物）。
2. **base 层三类来源**：
   - ✅ 包内有完整 Lua 实现（如 `buff.lua`、`timer.lua`、`unit.lua`）；
   - 🔀 转发桩（全文件仅 `return require '@base.base.xxx'`，实现在 client_base 库——编辑器侧同样不随 script 包分发，如 `deque`/`class`/`event` 等）；
   - ⚠️ 完全无源码（包内连桩都没有，如 `damage`/`heal`/`selector`/`inventory`/`shop` 等 33 个 + `game/*` 15 个）——引擎内嵌（C++ 注册或引擎 UPAK 内嵌 Lua），只能推测。
3. **`@lua_plus` = server_lua_plus 包 v14**（`Res/_m/maps/server_lua_plus/14/server_lua_plus/`）。触发器（可视化"lua+"）的服务端 API 层：大量 `function base.<领域>_<动作>(...)` 扁平封装，带 `@ui/@description/@belong` 注解——**这是推测引擎 API 的最佳素材**（包装器内部直接调用引擎对象方法，如 `damage:get_damage()`）。`base/` 与 `lp/base/` 两份内容相同（lp = lua_plus 缩写副本）。
4. **`@tds_score`**：TDS 排行榜/积分库，无独立分发包、引擎侧实现；`use_mysql` 键名表明可走 MySQL 存储。server_lua_plus 有 `base_lua_plus/tds_score.lua` 包装器可参照。
5. **官方地图库**（`@defaultui`/`@lib_common_ai`/`@smallcard_*` 等）：地图库的 `src/` 是服务端代码、`ui/` 是客户端。注意 **lib_common_ai 的 AI 实现（ai/、class/、ai_templates/）未随编辑器侧包分发**——其 `src/main.lua` 仅 4 行（`require_folder 'ai'` 等），运行时模块来自客户端变体或引擎内嵌；smallcard_inventory 的 `proto`/`proto_v2`/`score_save` 同理（服务端协议/存档模块不下发）。
6. **`@p_55a3`** = 游戏项目 test_res002：
   - `bgd_game_server/*` ↔ `.bgd/src/`（bgd 框架构建产物，模块名运行时小写化，源码文件名大写开头）；
   - `bgd_libs_server/*` ↔ `.bgd/libs/`（框架库）；
   - `obj/effect/<类型>/data` ↔ `script/obj/`（编辑器物编数据生成的数据模块，每个类型有 init/dict/data 三件套）；
   - `trigger_validator`、`trigger_module_main_1` ↔ 触发器 TS 编译产物（项目 `script/tsconfig.json` 的 `files` 可见）。
7. **数据模块占大头**：`@p_55a3/obj/effect/*/data`（50+ 个）是纯数据表，dump 的 16.9MB 主要是它们的值（技能数值、Buff 配置等，含 `<node-value>` 模板引用语法）。

## 标注约定

| 标注 | 含义 |
| --- | --- |
| ✅ 有源码 | api-13 解密源码或项目源码直接命中，签名为真实提取 |
| 🔀 转发桩 | 本包文件仅一行 `return require '...'`，实现他处；附调用点反查 |
| ⚠️ 无源码（推测） | 引擎实现/未分发；给出命名语义推测 + 全语料调用点反查（`base.x.fn(...)` 实参形态）+ lua_plus 封装对照 |
| 研判 | 人工结合知识库/代码证据的结论性注释 |

签名表中 `→` 后缀为 `@return` 注解摘录；`(self, ...)`/`:` 形式为方法定义原文。

## 推测方法论（如何对待"无源码"模块）

1. **调用点反查**：在 5640 个 Lua 文件语料（api-13 全解密 + test_res002 全量）中搜索 `base.<模块>.<fn>(实际参数)` / `base.<模块>:<fn>(...)`，得到真实参数形态；
2. **lua_plus 封装对照**：触发器层 `function base.<领域>_<动作>(...)` 的定义带完整 `@ui` 中文注解，其函数体直接揭示引擎对象方法（如 `damage:set_current_damage(amount)`）；
3. **命名语义 + 同类类比**：参照有源码的同类模块（如 `loot_pool` 有 lua_plus 包装器）推断职责；
4. **知识库交叉验证**：sce_app_editor-patch 的 sce-lib-script-199 知识库（isolation/加载链/桩机制）。

局限：反查只能发现"语料中被用过的"形态，引擎模块的完整 API 面必然大于观测集；实例方法（`damage:get_xxx()` 经局部变量调用）无法按模块归属聚合，未纳入。
