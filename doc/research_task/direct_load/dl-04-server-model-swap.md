# dl-04 双端联动实测：服务端换模族（PIE，批次 1~5）

> 日期：2026-08-27~28 | 状态：✅ 批次 1~5 完成（PIE 实测，线上未验）
> 主线问题：「客户端一直不成功，是不是因为服务端没有？」——服务端 StateGame 才是模型/单位权威入口。
> 探针：`test_res002/.bgd/src/server/test/dl04_server_model{,2,3,4,5}.lua` + 客户端观测 `.bgd/src/client/test/dl04_client_watch.lua`（轮询 `game.get_unit_model_path/get_unit_asset` + `on_unit_model_changed` 钩子）。

## 0. 核心结论（★ = 主线级）

- ★ **双端联动假设成立**：服务端 `unit:set_asset(已注册数编 Model link)` 是**活的权威换模通道**，~1.7s 后客户端同步：asset link 变、prefab 变时 `model_path` 变 + 引擎回调 `base.event.on_unit_model_changed(id, path)` 双端触发 + 视觉切换。【实测 PIE】
- ★ **服务端换模仍须注册 link**：set_asset 对裸 prefab 路径（无论该 prefab 是否属于已注册条目）一律静默 miss——与 render-21 客户端分派链同构（按 link 查 MODEL 表，miss 静默）。**免数编直载在服务端侧同样卡注册表**。【实测 PIE】
- ★ 客户端裸路径通道（render-06 `game.unit_change_model(id, prefab路径)`）与服务端权威通道（set_asset link）是**两套独立机制**：前者本地直渲免数编，后者权威同步但须数编。生产上「本地模型进世界」可用 render-06 通道（线上 user_libs 分发 render-11 已证）；「全玩家一致的服务端换模」须先注册数编 link（数编脚本化 render-10 已通）。
- `Unit:set_model`（服务端 dump 独有）**全参数形态返回 false、无任何效果，判死**（疑似占位/未接线实现）。【实测 PIE】
- `Unit:model_swap_push/pop` 内嵌 lua 实现报 `common/base/unit.lua:1428: table index is nil`，**判死**（引擎内嵌变体，无源码；疑依赖特定单位初始化状态）。【实测 PIE】
- `base.crop.set_model_unit/fresh_unit_model` 是**作物系统专用**（`crop.lua:114 field 'unit_list' nil` / `:264 'UnitList' nil`——须先经 crop 系统建单位），非通用换模通道，对本任务判死。【实测 PIE】
- `Unit:change_type(已注册 unit root link)` 返回 true 且客户端 asset 同步切换——**整单位类型切换通道活**（换类型连带换模型/技能/属性）。【实测 PIE】

## 1. 实测矩阵（全部 PIE StateGame，英雄 = 场景放置默认单位 $$default_units_ts.unit.星火战士.root，id=1）

### 批次1（dl04_server_model.lua）——首轮摸底
| 调用 | 返回 | 客户端反应 |
| --- | --- | --- |
| `hero:set_model('characters/_user/p_55a3_jilulu_19ec_a8oz/model.prefab')` | false | 无 |
| `hero:set_model('nonexistent/xxx/model.prefab')` | false | 无 |
| `hero:set_model('$$p_55a3.unit.主控.Model')`（失效 link） | false | 无 |
| `hero:model_swap_push(jilulu路径)` | ERR unit.lua:1428 table index is nil | — |
| `hero:model_swap_pop()` | nil | 无 |
| `base.unit_set_model(hero, jilulu路径)`（=`unit:set_asset`，server_lua_plus unit.lua:1171 源码实锤） | nil | 无 |
| `hero:set_asset('$$p_55a3.unit.主控.Model')` | nil | 无 |
| `hero:update_actor()` | ERR bad argument #2（须 string） | — |

### 批次2（dl04_server_model2.lua）——枚举与自身 link
- `base.eff.all_caches()` 返回 nil（不是表）——枚举数编缓存此路不通。
- `hero:get_name()` = `$$default_units_ts.unit.星火战士.root`；`base.eff.cache(hero:get_name())` 字段树实锤：`ModelData='$$default_units_ts.unit.星火战士.Model'`（★ set_asset 官方参数形态 = 数编 cache 的 ModelData 字段，spell_assist_control.lua:646 官方调用点互证）。
- `set_model(自身有效 Model link)` 也返回 false → set_model 的 false 与 link 有效性无关，函数本身不工作。

### 批次3（dl04_server_model3.lua）——首次阳性
| 调用 | 返回 | 客户端（watcher） |
| --- | --- | --- |
| `set_model('$$default_units_ts.unit.6、远程普攻示例英雄.Model')` | false | （无即时反应） |
| `set_asset(同上)` | nil | **+1.9s：model_path sk_basic2→sk_worker，asset→远程英雄.Model，EVENT on_unit_model_changed 触发** |
| `change_type('$$default_units_ts.unit.6、远程普攻示例英雄.root')` | true | （模型已是 M_A，无可观察增量） |
| `set_model(OWN)` 复原 | false | +1.6s 复原（该事件实际归属 RESTORE 前的状态迁移，归因见批次4单变量复测） |

### 批次4（dl04_server_model4.lua）——单变量归因（15s 间隔）
| 调用 | 返回 | 客户端 |
| --- | --- | --- |
| D1 `set_model(M_A)` | false | **无任何变化** → set_model 判死 |
| D2 `set_asset(M_B=13、骑乘英雄.Model)`（同 prefab sk_basic2，不同 link） | nil | +1.7s **asset→M_B**；model_path 不变、无 EVENT（prefab 相同，native strcmp 短路只更新 asset 字段） |
| D3 `unit_set_model(hero, M_A)`（=set_asset） | nil | +1.6s **EVENT + model_path→sk_worker + asset→M_A**（prefab 不同走完整换模） |
| D4 `change_type(远程英雄.root)` | true | 无增量（模型已是 M_A，自洽） |
| D5 `set_model(OWN)` | false | 无 → 模型停在 M_A 至会话结束（再次确认 set_model 死） |

### 批次5（dl04_server_model5.lua）——参数形态边界 + crop + change_type 视觉对照
| 调用 | 返回 | 客户端 | 判定 |
| --- | --- | --- | --- |
| E1 `set_asset('characters/general/sk_worker/model.prefab')`（已注册条目的 Asset 路径） | nil | 无 | ❌ 路径形态判死（注册表按 link 哈希查） |
| E2 `set_asset('$$default_units_ts.model.defaultmodelwithgenericfootstep.root')`（独立 model 条目） | nil | +1.8s asset 切换（path 不变，该条目 Asset 非单位模型） | ✅ **model.root 独立条目 link 也通** |
| E3 `set_asset('$$p_55a3.actor.bgd_jilulu_attach.root')`（actor link） | nil | 无 | ❌ actor link 不行 |
| E4 `set_asset('$$p_55a3.unit.主控.Model')`（项目 主控 单位 model link） | nil | 无 | ❌ 该条目运行时未注册（项目数编条目须真正被打包/加载） |
| E5 `base.crop.set_model_unit(hero, M_A)` | ERR crop.lua:114 'unit_list' nil | — | ❌ crop 系统专用 |
| E6 `base.crop.fresh_unit_model(hero, M_A)` | ERR crop.lua:264 'UnitList' nil | — | ❌ 同上 |
| E7 `change_type('$$default_units_ts.unit.13、骑乘英雄.root')` | true | +1.7s asset→骑乘英雄.Model | ✅ 活 |
| RESTORE `set_asset(OWN)` | nil | +1.9s asset→星火战士.Model | ✅ 复原正常 |

## 2. 机制认知更新

1. **服务端→客户端模型同步链**：服务端 set_asset/change_type → 引擎广播 → 客户端 native 更新 → `base.event.on_unit_model_changed(id, path)` 双端回调（script-199 unit.lua:1568 进 `单位-模型改变` 事件 + destroy/create actors）。同步延迟实测 ~1.6~1.9s（PIE 本地）。
2. **set_asset 幂等短路**：目标 link 的 prefab 与当前相同 → 只更新 asset 字段，不发 on_unit_model_changed（native strcmp 短路，render-06 同款）。
3. **有效 link 来源**：`script/obj/model/model.lua`（项目构建产物）列出本会话注册的全部 Model link；运行时 `unit cache.ModelData` 给单位当前模型 link。
4. **有效 Model link 形态**：`$$<包>.unit.<单位名>.Model` 与 `$$<包>.model.<条目名>.root` 均可；`$$<包>.actor.*` 不可；失效/未注册 link 静默 miss。
5. **引擎内嵌 lua 黑盒**：`set_model`/`model_swap_push`/`crop` 族为引擎内嵌 TSTL 变体（dll 明文无字符串，lua_api_dump 无 luaL_Reg 锚点），报错行号对应内嵌版 unit.lua/crop.lua——目前无法静态读源码，只能靠返回值/错误信息黑盒推断。

## 3. 对任务主线的意义

- **模型直载（免数编把本地模型渲进世界）**：已有通道不变——客户端 render-06（真实单位 + prefab 裸路径，本地直渲）+ render-11 线上分发。服务端权威通道本轮打通但**只认注册 link**，不能把「未注册本地模型」广播给所有玩家。
- **要让服务端广播自定义模型**，剩余路径：① 数编脚本化预注册（render-10 已通，非免数编但可脚本批量）；② frida 注册表注入（render-21/22 遗留路线：克隆伪造条目/桶链注入）——若注入客户端 MODEL 表成功，set_asset 服务端广播 link 即闭环。
- **特效直载加强（次级目标）**：服务端创建特效通道（`base.create_actor_at`/`EffectParam:create_actor`/`create_beam_effect` 等，dl-02 B/C 组）尚未实测，是下一批双端联动对象。
- **图集子图**：本批不涉及，走专项（线索三）。

## 4. 下一步（dl-05 候选）

1. 特效双端联动批次：`base.create_actor_at('$$p_55a3.actor.bgd_demo_effect.root', point)`（服务端）→ 客户端观察；`EffectParam:create_actor` / `base.create_beam_effect`。
2. PascalCase 候选 PIE 实测（dl-03 §6 矩阵）：`io.AddResourcePath`/`WalkResourceDir`/`CheckExistsFile` 等存在性→签名→行为；顺带验证 `io.AddResourcePath` 能否让引擎资源系统解析 pak 外散文件（若通，render-06 通道的资源面扩到任意磁盘目录）。
3. 客户端 `unit_change_model` 广播方向验证（render-06 遗留：客户端换模后服务端/其他客户端是否可见）。
4. frida 注册表注入（render-21 遗留路线②③⑤）——工作量最大，放在特效/PascalCase 之后。

## 5. 过程坑沉淀（本轮新增）

- **PIE 无对局逻辑时无英雄单位**：`GameServer`/`GameClient` 不加载时世界只有天空盒，且地图未放默认单位时 `player_get_hero` 无对象——实验前确认场景有 `is_main_hero=1` 单位（scene/default/unit_save.lua）。
- **服务端日志实时可读**：`D:/sce_online/logs/server/lua-game-server-<日期>-<id>.log`（MCP get_game_logs source=game_server 给最新文件）——lowlevel 台账「服务端日志停止调试后才刷新」对本环境不再成立（至少 PIE 是这样）。
- **PIE 客户端会话会周期性重载**（约 15~50s 一次，疑官方看门狗/玩法流程），长序列探针要把单步压进单会话窗口或接受跨会话重放。
- **归因必须单变量 + 大间隔**：set_asset 有 1.6~1.9s 同步延迟，批次3 的 6s 间隔造成 C1/C2 归因歧义，批次4 用 15s 间隔 + 三模型交叉才定案。
- `base.eff.all_caches()` 返回 nil——「枚举数编」别指望它，改用项目构建产物 `script/obj/model/model.lua` 静态取 link。
