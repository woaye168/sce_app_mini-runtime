# 过程记录：服务端 package.loaded 模块研究

> ⚠️ 状态更新：第一轮产物已移至 `draft_v1/`（中间态，未做 dump 值树解析，字段级覆盖不全——如 `backend.url` 这类嵌套字段未纳入）。后续研究按本目录 `研究任务提示词.md` 执行，定稿后再进 `doc/research/`。

## 任务

`loaded_module_server_package_loaded.txt`（16.9MB，657 行，服务端 StateGame `package.loaded` dump）→ 全覆盖知识文档（363 模块、函数签名、无源码标注推测）。

## 关键过程发现

1. **dump 结构**：非纯净 JSON，是带转义引号（`\"`）的序列化碎片，混有 GBK 显示乱码（`■■■■`）。模块键用正则 `\\"(@[^\\"]{1,200})\\"\s*:` 提取，得 **363 个唯一 `@` 前缀键**；点分键 0 个（服务端全部走 `@` 跨库形态）。
2. **键 → 源码映射规则**（覆盖 281/363）：
   - `@common/<p>` → `sce_open/api-13/2026_08_27/script/199/common/<p>.lua`
   - `@lua_plus/<p>` → `server_lua_plus/14/<p>.lua`（`base/` 与 `lp/base/` 内容重复）
   - 地图库 `@<lib>/<p>` → 依次试 `src/ script/ ui/script/ ui/src/` 子根
   - `@p_55a3/bgd_game_server/<p>` → `test_res002/.bgd/src/<p>.lua`（**大小写不敏感**，运行时模块名小写化）
   - `@p_55a3/bgd_libs_server/<p>` → `test_res002/.bgd/libs/<p>.lua`
   - `@p_55a3/obj|scene|main|trigger_*` → `test_res002/script/`
3. **82 个无源码模块的分布与原因**：
   - `@common/base/game/*`（15）+ `@common/base/` 部分（33）：引擎内嵌，base/init.lua 证实 `base.game` 全局由引擎侧扩展；
   - `@lib_common_ai/ai|class|ai_templates/*`（20）：**编辑器侧包未分发 AI 实现**（src/main.lua 仅 require_folder 四行），运行时来自客户端变体/引擎；
   - `@smallcard_inventory/proto*`、`score_save`、`@smallcard_mail/mail|module`、`@smallcard_get_items/module`（6）：服务端协议/逻辑不下发编辑器；
   - `@tds_score/*`（5）：TDS 排行榜库引擎侧实现，`use_mysql` 表明 MySQL 存储通道；
   - `@defaultui/actor`、`@defaultui/default_ui`（目录命名空间模块）等（3）。
4. **推测三手段**：调用点反查（`base.<mod>.<fn>(实参)` 正则，5640 文件语料）/ lua_plus 扁平封装对照（`function base.<领域>_<动作>` 带 @ui 注解）/ 命名语义。
5. **转发桩识别**：全文件仅 `return require '@base.base.xxx'`（6 个命中），实现在 client_base 库（与 sce-lib-script-199 知识库结论一致）。

## 产物清单

| 文件 | 说明 |
| --- | --- |
| `module_source_map.json` | 363 键 → 源码路径/归属 完整映射（机读） |
| `methodology.md` | 本文件 |

生成工具为一次性 Python 脚本（临时目录执行，未入库；规则即上述映射 + 签名提取 + 反查，可从 module_source_map.json 复现）。

## 复现/更新

编辑器升级导致包版本变化时：重跑 `D:\sce_open\decrypt_lua_packages.py` 出新日期目录 → 按上表映射规则重提取（键集合以新 dump 为准）。
