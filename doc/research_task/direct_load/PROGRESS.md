# direct_load 研究进度看板

> 任务：模型 / 图集子图免数编直载（提示词见本目录「研究任务提示词.md」）
> 发起：2026-08-27 | 本文件实时更新，中断后从这里续作。

## 阶段状态

| 阶段 | 文档 | 状态 |
| --- | --- | --- |
| 1. 知识修正快赢 | [dl-01-knowledge-corrections.md](dl-01-knowledge-corrections.md) | ✅ 完成（2026-08-27） |
| 2. 服务端渲染面盘点（线索一静态） | [dl-02-server-render-surface.md](dl-02-server-render-surface.md) | ✅ 完成（2026-08-27，98 条候选 + Top 10 实测顺序；实测待 dl-04） |
| 3. PascalCase 漏网枚举（线索二静态） | [dl-03-pascalcase-candidates.md](dl-03-pascalcase-candidates.md) | ✅ 完成（2026-08-27，静态差集；实测矩阵待 dl-04） |
| 4. 双端联动实测矩阵 | [dl-04-server-model-swap.md](dl-04-server-model-swap.md) | 🔵 进行中（换模族 5 批次完成：set_asset 注册 link 通道走通+双端同步实锤；set_model/model_swap/crop 判死；特效族待测） |
| 5. 图集子图专项 | 待定 | 🔵 进行中 |
| 6. 线上实证 | 待定 | ⬜ 未开始 |
| 7. 定稿收口（doc/research/） | 待定 | ⬜ 未开始 |

## 实验矩阵（随研究推进填充）

| 编号 | 实验 | 通道/假设 | 状态 | 证据 |
| --- | --- | --- | --- | --- |
| DL04-A | 服务端 `Unit:set_model` 全形态 | 线索一 | ❌ 判死（全 false 无效果） | dl-04 §1 批次1/2/4 |
| DL04-B | 服务端 `Unit:set_asset(注册 Model link)` | 线索一 | ✅ 走通（~1.7s 同步客户端，asset/model_path/EVENT/视觉全变） | dl-04 §1 批次3/4/5 |
| DL04-C | 服务端 set_asset 裸 prefab 路径 | 免数编 | ❌ 判死（静默 miss，含已注册条目的路径形态） | dl-04 §1 批次1/5 E1 |
| DL04-D | `model_swap_push/pop` | 线索一 | ❌ 判死（内嵌 lua 报 table index nil） | dl-04 §1 批次1/3 |
| DL04-E | `base.crop.set_model_unit/fresh_unit_model` | 官方业务先例 | ❌ 判死（crop 系统专用） | dl-04 §1 批次5 E5/E6 |
| DL04-F | `Unit:change_type(注册 unit link)` | 线索一 | ✅ 走通（asset 同步） | dl-04 §1 批次4/5 |
| DL04-G | set_asset 独立 model 条目（$$..model.*.root） | 参数形态 | ✅ 通 | dl-04 §1 批次5 E2 |
| DL04-H | set_asset actor link / 未注册项目 link | 参数形态 | ❌ 静默 miss | dl-04 §1 批次5 E3/E4 |
| DL05 | 特效双端联动（create_actor_at 等） | 线索一 | ⚠️ 已测但无效化：目标 actor 条目已被编辑器清理（render-10 数编被清）；通道本身待有效条目复测 | [dl-05-server-effect.md](dl-05-server-effect.md) |
| DL06 | PascalCase 候选 PIE 实测（dl-03 §6） | 线索二 | ✅ 完成：io 漏网 14/14 存在；**game/ui 表双名注册补证实锤（各精确 50%）**；io 资源探测族 PIE 不命中散文件（待线上 pak 复测）；**dl06 探针致编辑器卡死（已隔离，io 高危探测须逐个隔离）** | [dl-06-pascalcase-pie.md](dl-06-pascalcase-pie.md) |
| DL07 | 客户端 change_model 广播方向 | render-06 遗留 | 未测 | |
| DL08 | get_game_table 直写锻造条目 | 用户线索 table_writer/table.lua | ❌ 判死（空表/写不持久/每次新副本） | [dl-07-09-table-write-deadends.md](dl-07-09-table-write-deadends.md) |
| DL09 | obj 文件运行时注入 + reload_pak 重载 | 同上延伸 | ❌ 判死（obj 注入落盘 ok，reload_pak 三形态静默，native 仍不认锻造 link） | 同上 |
| DL10 | frida 注册表注入（render-18 §6/§7：复用 insert/桶链注入/查找钩伪造） | 免数编主线 | ⬜ **唯一剩余主线** | render-18 |
| DL11 | 大厅/mini-runtime 换游戏流程逆向（线上生产通道） | 免数编主线 | ⬜ | lowlevel 台账 |

## 当前进展日志

- 2026-08-27 任务启动；通读提示词与必读材料（webview-bridge.md / pak-io-native.md / lowlevel README / render-03/05/12）。
- 2026-08-27 **dl-01 完成**：16 条过时结论修正落盘（render-03/05/12 + lowlevel README），修正块互链 webview-bridge.md / pak-io-native.md。关键认知更新：webview 双向桥全通（cgui 内置三步登记）→ 图集子图新增全双工 canvas2d 保底通道；PascalCase 漏网有线上实证先例。
- 2026-08-27 **dl-02 完成**：服务端渲染面盘点 98 条候选（A 单位创建/换模 30 / B actor 24 / C 特效 8 / D 场景 18 / E 资源加载 10 / F 其他 8）。重大线索：**Unit 类双端不对称**——服务端独有 `Unit:set_model / model_swap_push/pop / set_model_attribute / set_asset / set_particle` 整套换模面（客户端的 change_model/attach_model 不在服务端 dump）；dump-only 引擎注册函数 `base.stop_actor/play_actor/unit_update_actor/scene_object/player_jump_scene_object`；`base.crop.set_model_unit/fresh_unit_model` 证明服务端有官方运行时换模业务先例。Top 10 优先实测清单见 dl-02 文末。
- 2026-08-27 **dl-03 完成**：差集收敛——io 表 40 条 PascalCase 漏网候选（全【逆向实锤】，双引擎 io-*.tsv 133 行全量双名；isolation 61 处阉割全写小写名逐行复核）；os/debug/package/cmsg_pack 21 条判定无漏网（标准库不走引擎双名注册）。重点候选 13 条（AddResourcePath/WalkResourceDir/CheckExistsFile/CopyCacheFile 等资源通道）。双名普遍性结论：io/common 两表实锤，game/ui/actor 游戏对象表无证据（语义推测），已设计 PIE 三段式探针 + 双名补证探针待 dl-04。
- 2026-08-27~28 **dl-04 批次 1~5 完成（换模族 PIE 实测）**：★ 双端联动假设成立——服务端 `unit:set_asset(注册 Model link)` ~1.7s 同步客户端（asset/model_path/on_unit_model_changed/视觉全链路）；★ 但免数编仍卡注册表（裸路径静默 miss，E1 连「已注册条目的 prefab 路径」都 miss）。set_model 全 false 判死、model_swap_push 内嵌 lua 报错判死、crop 族系统专用判死、change_type 走通。新坑沉淀：PIE 无 GameServer 无英雄、服务端日志实时可读（台账旧结论修正）、PIE 客户端周期性重载、归因须单变量大间隔、all_caches() 返 nil。详见 dl-04。
- 2026-08-28 **dl-05~09 完成**：① dl-05 特效联动——目标 actor 条目已被编辑器清理（render-10 数编被清坑坐实），须先重新注册再复测；② dl-06 PascalCase PIE——io 漏网 14/14 存在、**game/ui 表双名注册实锤（各 50%）**、**dl06 探针卡死编辑器（隔离原则：io 高危一次一个）**；③ dl-07/08/09 数编写入三连判死（get_game_table 空表不持久 / reload_pak 静默 / obj 注入+reload 终审 native 不认）——**lua 层注册表注入路线全灭，剩 frida（DL10）与换游戏流程（DL11）**。副产物：pak=TNND(UPAK) 直封、数编实体=obj lua、game 表 626 项签名 dump（test/direct_load/game-editor.out）、pak_table_peek.py。

## 关键输入材料索引（常用）

| 材料 | 路径 |
| --- | --- |
| 前序渲染台账 | [../lowlevel/README.md](../lowlevel/README.md) |
| 服务端 API 地图 | [../loaded_modules_server/draft_v2/](../loaded_modules_server/draft_v2/) |
| 服务端 dump 原件 | [../loaded_modules_server/loaded_module_server_package_loaded.txt](../loaded_modules_server/loaded_module_server_package_loaded.txt) |
| 双名注册全集 | [../../research/common-table.md](../../research/common-table.md) |
| PascalCase 先例 | [../../research/pak-io-native.md](../../research/pak-io-native.md) |
| 视频突破定稿 | [../../research/webview-bridge.md](../../research/webview-bridge.md) |
| api-13 依赖包源码（只读） | `D:/sce_open/api-13/2026_08_27/` |
| 探针套件 | `examples/probes/`（sce_app_mini-runtime） |

## 工作约定速记（提示词 §实测方法）

- 测试项目 `test_res002`，探针写 `.bgd/src/{server,client}/`，不提交 git，测完不清探针。
- 服务端探针挂 `玩家-连入`；客户端入口 `.bgd/src/client/init.lua`；日志只用 `log.info`（服务端停止调试后才刷新）。
- 线上：MCP publish_project → `tester_1089.exe -game=<图> -tag=test -ai_test=1`；起局加 `-width=800 -height=540`。
- 引擎 lua 不接受 `|` 前缀注释行；MCP 截图单张 ~30s；调试完关客户端/编辑器/mini-runtime。
