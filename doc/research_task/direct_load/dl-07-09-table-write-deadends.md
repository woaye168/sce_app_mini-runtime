# dl-07~09 数编表写入三连判死（get_game_table 直写 / reload_pak / obj 注入+重载）

> 日期：2026-08-28 | 状态：✅ 完成（PIE，全部阴性）
> 线索来源：用户提示「client_base table_writer.lua + table.lua——动态加载/构建数编并写入引擎 C++」
> 探针：`dl07_table_forge.lua` / `dl08_reload_pak.lua` / `dl09_forge_reload.lua`（test_res002/.bgd/src/client/test/）

## 0. 线索溯源结论

- `client_base/common/base/table_writer.lua` = **纯序列化器**（lua 表 → 文本，挂 `base.table_unpack`，全语料零调用点——休眠工具）。本身不碰引擎。
- `script-199/common/base/table.lua` = 客户端数编 lua 缓存装配：`base.table.<类型>` 首次访问时 `game.get_game_table(原生表名)` + `require('@@.obj.<类型>.<文件>')` 合并。**读链，无写链**。
- 推论：「写入引擎 C++」若有路，只可能是 ① get_game_table 返回表是 native 活视图（dl-07 证伪）；② reload_pak 重触发加载（dl-08/09 证伪）；③ frida 复用 LoadMapTable 内部 insert（render-18 §7，唯一剩余）。

## 1. dl-07：get_game_table 直写判死

| 实验 | 结果 |
| --- | --- |
| `game.GetGameTable('ActorModelData')` 形态 | **空表（n=0）、每次调用新副本（rawequal=false）、无元表** |
| 读已有条目（`6、远程普攻示例英雄.Model`） | nil |
| 写标记 → 重取 | **MISS**（写入不持久，各次调用互不可见） |
| 锻造 model/actor 条目写入 + merge_cache 补 lua 门 | lua 侧 `base.eff.cache` 读回 ok；`base.actor(锻造link)` = nil |
| `base.create_actor_at(锻造 actor link, point)` | nil |

→ 与 render-04 §1「get_game_table 注入 ❌（ActorModelData 返回空表）」完全一致（本轮独立复验）。**StateGame 下 native 注册表不经 get_game_table 暴露，直写判死。**

## 2. dl-08：reload_pak 语义探测

- `common.reload_pak / ReloadPak` 存在【逆向实锤注册表】。
- `reload_pak()` / `('p_55a3')` / `(项目绝对路径)` 全部静默返 nil；lua 日志无任何表加载行（PIE native 日志不落盘，无法直接观测）。
- 语义推测：大概率是 Urho3D ResourceCache 的包重载（资源文件层），不碰数编表。行为终审见 dl-09。

## 3. dl-09：obj 文件注入 + reload_pak 终审（判死）

流程：运行时把锻造 model/actor 条目（吉鲁鲁 prefab）**插进 4 个 obj 文件**（`script/obj/{model,actor}` + `ui/script/obj/{model,actor}`，`return entry_datas` 前注入，io.Write 落盘成功）→ merge_cache 补 lua 门（读回 ok）→ `reload_pak` 三形态 → `base.actor('$$p_55a3.actor.dl09_forge.root')`。

| 步骤 | 结果 |
| --- | --- |
| PRE 对照（注入前创建锻造 link） | nil ✓（符合预期） |
| obj 文件注入 | 4/4 WRITE=0 成功 |
| merge_cache + lua 读回 | ok |
| reload_pak × 3 形态 | 静默 nil |
| POST `base.actor(锻造 link)` | **nil —— native 仍不认** |

→ **reload_pak 不重触发数编表加载**（或触发了也不读 obj lua）。运行时注册表注入 lua 层路线全灭。

## 4. 免数编注册表注入：全路线状态板（截至 dl-09）

| 路线 | 状态 | 出处 |
| --- | --- | --- |
| merge_cache 虚拟数编 | ❌ lua 通 / native 拒 | render-20 G29/G33 |
| get_game_table 直写 | ❌ 空表/不持久 | render-04 + dl-07 |
| obj 文件运行时改写 + reload_pak | ❌ | dl-09 |
| load_map 触发表加载 | ❌（表加载=会话一次） | render-20 |
| LoadMainMap 运行时换图 | ❌ | render-22 |
| **frida 复用 LoadMapTable 内部 insert / 桶链注入 / 查找钩伪造条目** | ⬜ **唯一剩余主线** | render-18 §6/§7 |
| 大厅/mini-runtime 换游戏流程逆向（线上生产通道） | ⬜ | lowlevel 台账遗留 |

## 5. 副产物

- 地图 pak = TNND(UPAK) 直封（非 7z 链）；一次性解包列表脚本 `test/direct_load/pak_table_peek.py`；p_55a3 pak 1303 条目，无独立二进制数编文件——**数编数据实体 = obj lua 文件**（`ref/*.ref` 均为资源引用清单）。
- `game` 表 626 注册项带签名 dump：`test/direct_load/game-editor.out`（lua_api_dump 产物）。
- PIE 下 `game.GetMapPath()` 返回项目目录（c:/...），`io.Read/Write` 原生版（PascalCase）直接读写项目文件可用（dl-09 注入即经此）。
