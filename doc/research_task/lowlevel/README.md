# 底层 API 研究台账（渲染 + 云变量）

> 任务发起：2026-08-23 | 目标：① 渲染底层 API（图像/视频/模型/特效/spine/图集子图，绕过 base.ui/imgui/canvas_texture 局限）② 云变量底层 API（绕过 sce.s 分桶与计数，尽量直读直写）
> 方法：源码溯源（.editor_src_mirror）+ 二进制字符串/导出考古 + Frida 抓包 + test_res002 实测探针
> 本目录文档编号即阅读顺序；本文件实时更新进展。

## 文档索引

| 文件 | 内容 | 状态 |
| --- | --- | --- |
| [cloudvar-01-lua-chain.md](cloudvar-01-lua-chain.md) | 云变量 Lua→native→Entrance 全链溯源 | ✅ 完成 |
| [cloudvar-02-static-addendum.md](cloudvar-02-static-addendum.md) | 云变量静态补遗 + 协议逆向 runbook | ✅ 静态完成 |
| [cloudvar-03-msgid-and-env.md](cloudvar-03-msgid-and-env.md) | 消息 ID 表（0xA000=ScoreArchive）+ 官方 API 文档镜像 + 抓包沙盒环境 | ✅ 完成 |
| [cloudvar-04-protocol.md](cloudvar-04-protocol.md) | Entrance/ScoreArchive 帧格式完整解码（双向实证） | ✅ 完成 |
| [cloudvar-05-direct-poc.md](cloudvar-05-direct-poc.md) | ★ 直读直写 PoC 跑通（纯 Python 连 Entrance 读写云变量） | ✅ 完成 |
| [cloudvar-06-op-matrix.md](cloudvar-06-op-matrix.md) | 全操作矩阵：op 码表 + MessagePack 值编码 + 权限矩阵 | ✅ 完成 |
| [cloudvar-07-message-api.md](cloudvar-07-message-api.md) | ★ message_* 签名破解（反汇编+PIE 实证；发送成功，权限需游戏态） | ✅ 签名完成 |
| [cloudvar-08-gamestate-and-permissions.md](cloudvar-08-gamestate-and-permissions.md) | ★ B 模式游戏态定性（UDP 直连无 Entrance=sce.s 黑洞）+ 权限闸门实验（凭证/flags 全证伪）+ 签名终表 | ✅ 完成 |
| [cloudvar-09-tester-game-state.md](cloudvar-09-tester-game-state.md) | ★ 真 tester 局权限矩阵 + `-game=x -tag=test -ai_test=1` 自动化进游戏通道 + themis 挡 attach 实锤 | ✅ 完成 |
| [render-01-native-registry.md](render-01-native-registry.md) | 渲染 native 注册块穷举 + 各通道结论 | ✅ 静态完成 |
| [render-02-probes.md](render-02-probes.md) | base.ui 探针矩阵：特效直路径破解（属性类型敏感）、sprites/spine 实证 | ✅ 完成 |
| [render-03-imgui-channel.md](render-03-imgui-channel.md) | ★ imgui 立即模式直驱通道：webview/video StateGame 内渲染成功 | ✅ 完成 |
| [render-04-model-chain.md](render-04-model-chain.md) | ★ 本地模型：数编注册链全解（entry_data.ini→obj lua→native），actor/set_asset 实测矩阵 | ✅ 完成 |
| [render-05-webview-bridge.md](render-05-webview-bridge.md) | webview=miniblink 内核；canvas2d 通道 + run_js(lua→JS) 实证；~~JS→lua 未通~~ → **已通**（见 [webview-bridge.md](../research/webview-bridge.md)） | ✅ 完成 |
| [render-06-unit-change-model.md](render-06-unit-change-model.md) | ★ unit_change_model 完整破解：prefab 相对路径直渲本地模型（根因=负 id preview 单位无效，须真实单位） | ✅ 完成 |
| [render-11-pak-resources.md](render-11-pak-resources.md) | 发布 pak 资源规则：自定义模型依赖件不进地图 pak，线上靠 user_libs 分发（✅ 用户 E2E 实证） | ✅ 完成 |
| [render-07-managed-dll.md](render-07-managed-dll.md) | 官方 C# managed dll 逆向（TNND 解密+dnfile）：架构地图 + Sprite2D.TextureRect 图集源矩形线索 | ✅ 完成 |
| [render-08-atlas-uv.md](render-08-atlas-uv.md) | tiled 图集专题：UI 控件 UV 矩阵全证伪（染色归因法破假阳性）+ 三通道生产排序 | ✅ 完成 |
| [render-09-attach-model.md](render-09-attach-model.md) | attach/detach 攻坚：socket 命名规范破解（prefab sockets 表）+ 客户端全矩阵证伪 + 官方 ActorAdditionModel 路径 | ✅ 客户端证伪 |
| [render-10-actor-attach.md](render-10-actor-attach.md) | ★ 官方 actor 附着通道全破解：数编脚本化（模板+时间戳闸门生效流程）+ create_actor_at + attach_to 视觉实证 | ✅ 完成 |
| [wasicore-01-discovery.md](wasicore-01-discovery.md) | ★★ 星火 2.0（WasiCore/C#/WASM）官方全代码栈发现：证据链 + 与 1.0 的关系 | ✅ 完成 |
| [wasicore-02-render-api.md](wasicore-02-render-api.md) | ★ 2.0 渲染 API 面：TextureRect 源矩形/Canvas(NanoVG)/RuntimeParticleBuilder/Prefab/Spine/RTT | ✅ 完成 |
| [wasicore-03-clouddata-api.md](wasicore-03-clouddata-api.md) | ★ 2.0 云数据 API 面：六桶/ListItem/名称注册表/跨用户 ACID 事务/游标扫描 | ✅ 完成 |
| [wasicore-04-adoption.md](wasicore-04-adoption.md) | 2.0 采用路径：创建/迁移/构建发布/限制/版本/Runtime MCP 官方调试 | ✅ 完成 |
| [render-12-online-imgui.md](render-12-online-imgui.md) | ★ imgui 通道线上实证：video https mp4 播完 + webview canvas2d 上屏（tiled 图集生产通道坐实） | ✅ 完成 |
| [render-13-gameworld-uiworld.md](render-13-gameworld-uiworld.md) | GameWorld/UIWorld 组件栈全解 + RT 崩溃归因 | ✅ 完成 |
| [render-14-uiscene-page-flow.md](render-14-uiscene-page-flow.md) | ★★ UIScene 通道完整打通：手写页面脚本化 + BindToUIScene 视觉实证（本地模型进 UI）；scene 控件"死亡"结论修正；**编辑器+线上发布均生效（用户实证）** | ✅ 完成 |
| [cloudvar-10-ratelimit-and-corrections.md](cloudvar-10-ratelimit-and-corrections.md) | ★ 直连限流实证（~57 并发窗口 code=25，无分钟硬计数，稳态 130ops/s）+ money_add 全环境放行修正 + item_add 签名定版 | ✅ 完成 |
| [render-15-gameui-package.md](render-15-gameui-package.md) | '@gameui' 包 lua 物理位置（update 缓存 gameui/48，TNND 已解密）+ native require 前缀表 + uiscene.lua 组件源 + 组件清单 | ✅ 完成 |
| [render-16-scene-model-name.md](render-16-scene-model-name.md) | ★ scene 控件 name 命名空间破解：model=单位表节点名 / particle=特效表节点名；light 游戏态不生效；knead_human 语义 | ✅ 静态完成 |
| [render-17-dynamic-uiworld.md](render-17-dynamic-uiworld.md) | ★★ 动态渲染深化：特效进 UIWorld（ActorEffect 条目）+ 项目包内自定义 renderpath 实证 + SCE native API 目录 + **免数编攻坚 G24~G30（native 注册表运行时只读，创建/set_asset 均须数编 link）** | ✅ 完成 |
| [render-18-native-registry-reverse.md](render-18-native-registry-reverse.md) | ★★ native 数编注册表与 set_asset 静态逆向：GameDataManagerImp 表族 + 注册口判死 + frida hook 点清单 | ✅ 静态完成 |
| [render-19-gameworld-loadmap.md](render-19-gameworld-loadmap.md) | GameWorld load_map/set_map_dir/create_scene/use_light_group 逆向（load_map 语义=注册表地图名，非任意路径） | ✅ 静态完成 |
| [render-20-virtual-eff-and-tableload.md](render-20-virtual-eff-and-tableload.md) | ★ 动态虚拟数编（用户实现）三入口终审（lua 通/native 拒）+ **表加载=会话一次、load_map 永不触发表加载**（伪造目录注入判死）+ lua `|` 注释前缀坑 | ✅ 完成 |
| [render-21-setasset-entry-mutation.md](render-21-setasset-entry-mutation.md) | ★★ set_asset/注册表动态逆向全解：分派链（vfunc+0x60/+0x70 按类型分表查找，miss 静默 je）、ACTOR 表桶链、**typeid=djb2-32(link) 精确验证**、MODEL 条目布局、Tier-1 纯路径改写判死（apply 消费预载句柄） | ✅ 完成 |
| [render-22-loadmainmap-reload.md](render-22-loadmainmap-reload.md) | ★ xdeditor「强制重新加载项目」机制移植：**重载链=EDITOR.unload_map/load_map 仅编辑器壳 lua 态可用**；模块级 LoadMainMap 游戏侧三轮实证判死（同名 no-op/异名进程爆死/reset AV）；frida 直调 native 通路打通（getter 签名/lua54 导出） | ✅ 完成 |

## 固化工具（examples/）

| 工具 | 用途 |
| --- | --- |
| `entrance_sniff` | Entrance 帧明文 dump（native hook，双向 0xA000） |
| `entrance_client` | ★ 云变量直连读写 CLI（read/seti/sets/set/ladd/list；表值 msgpack 写入已实证字节级复读一致） |
| `probes/` | ★ 游戏内 lua 探针套件（2026-08-25 自 test_res002 转移固化）：GameWorldProbe（G1~G36 渲染/UIWorld/免数编矩阵）、CloudProbe（云变量）、RenderProbe/RenderProbeHtml（imgui/webview）、MessageProbe（message_*）、virtual_effect（动态虚拟数编，注释已清洗）——接入方式：拷入项目 `.bgd/src/client/` 并在 init.lua require |
| `probes/frida/` | frida 探针套件：registry_probe/2/3（set_asset 分派链/注册表容器/条目改写）、loadmainmap_probe/2（ctx getter 观察+LoadMainMap 直调）、addr_test（frida17 attach 可行性）、unit_model_hook/attach_hook（render-06/10 模板） |

## 遗留清单（下轮入口）

### 云变量
- ~~读写计数维度实测~~ → cloudvar-10 §1（限流=~57 并发窗口 code=25 task_queue_limit_exceeded，无分钟硬计数秒级恢复，稳态 ~130ops/s；创作者中心后台对照仍可做）。
- ~~tester 生产环境凭证 token_type 取样~~ → cloudvar-08 §2（token_type=11 同 editor，权限无差异；tester lobby 与 editor 同一 Entrance 端点）。
- ~~message_send/message_query 客户端签名试通~~ → cloudvar-07 完成；签名终表 cloudvar-08 §3（send: player,key,target_user_id,value,events?）。
- ~~游戏态（StateGame/tester）补抓 money/item/rank 系帧~~ → B 模式证伪（cloudvar-08 §1）→ **真 tester 局闭合（cloudvar-09）**：money_add 真局解锁；money_init/rank/message/query_item 真局仍拒 = **地图级授权**（创作者中心开通）；自动化进游戏通道固化（`-game=x -tag=test -ai_test=1`）；themis 挡 frida attach 实锤（spawn 未试）。
- 服务端独有（subscribe/publish/world_data）需 host 侧 hook，出界未做；官方 API 可直接用。
- ~~entrance_client 扩展：msgpack 表值写入~~ → 已完成（cloudvar-06 §6：set/ladd 线上实证；手写 mp 编码器，rmp-serialize 弃用移除）。剩余：message_*、批量/流水。

### 渲染
- ~~unit:change_model/attach_model 实测~~ → render-06（change_model 破解）；render-09（unit_attach_model 客户端判死）→ **render-10 官方 actor 附着通道打通**（数编脚本化 + create_actor_at + attach_to 视觉实证）。
- ~~entry_data.ini model/actor 条目模板固化~~ → render-10 §2 完成（模板 + 生效流程：写 ini → 删两个 obj save_info.json → bump editor 戳 → 重开编辑器 → full 调试）。
- ~~webview JS→lua 鉴别~~ → render-05 §5：编辑器 PIE 里 base.ui webview 创建成功但**页面不加载**（check_webview_environment=true 不是阀门；死亡点=StateGame 渲染管线不驱动 webview 控件，此结论仍成立——webview-bridge.md §6.1 确认与互通无关），JS→lua 因此无消息；imgui 通道页面可跑。**2026-08-26 终版：JS→lua 双向桥全通**——imgui 控件手动登记 `base.ui.map[id].event.on_web_message` + `register_event` 即收消息，三端上线实测（[webview-bridge.md](../research/webview-bridge.md) §2）。~~webview 线上 tester 验证~~ → render-12 完成（canvas2d 上屏）。
- ~~video https mp4 播放实测~~ → render-12：线上真局实证（播放器播完 2:18 mp4 + webview canvas2d 上屏，imgui 通道线上不崩）。~~GameWorld+viewport 复刻~~ → render-13：UIWorld 组件栈全解+世界渲 RT 不崩；显示侧剩**数编 UIScene 模板 + BindToUIScene** 唯一路径（下轮：entry_data 脚本化建 UIScene 控件 + 线上复验）。
- ~~characters 等 3D 资产发布进 pak 规则确认~~ → render-11 完成 + 用户 E2E 实证（render-14 通道线上生效）。
- ~~用户线索：逆向 official_dotnet_bcl_package/6 与 official_client_deps_dll_package/23~~ → render-07 已完成（Sprite2D.TextureRect 线索；云变量无突破）。
- ~~新线索：触发图（节点图）系统是否暴露 Sprite2D/TextureRect 节点~~ → **证伪**（触发编辑器 V1/V2 节点穷举无 Sprite2D）；但顺藤挖出**星火 2.0 WasiCore 官方 C# 栈**（wasicore-01~04）：TextureRect/Canvas/RuntimeParticleBuilder/Prefab/Spine 全是 2.0 官方 API，1.0 lua 项目不可用（api 2000 独立项目类型）。
- 新线索：2.0 云数据协议逆向——entrance_sniff 抓 2.0 游戏态 Entrance 帧，验证是否同 0xA000 通道扩展 op 面（wasicore-03 §4）；若证实，entrance_client 可升级支持跨用户事务/列表/名称注册表直连。
- 新线索：2.0 零成本试玩——编辑器新建 code_sample 模板项目，实测 TextureRect 图集/RuntimeParticleBuilder/Canvas 在 PIE 与 tester 的表现（可顺便验证 2.0 编辑器入口在当前安装确实可用）。
- **免数编直载资源（主线，render-21 → render-22 再收窄）**：native 数编注册表运行时只读已五 routes 判死（创建/set_asset/merge_cache 虚拟数编/load_map 注入/LoadMainMap 运行时换图）；Tier-1 纯路径条目改写判死。**机制已全解**：typeid=djb2-32(link)、ACTOR 表桶链、set_asset 分派链、重载链归属编辑器壳。下轮：① hook apply 链（0x18177d5a0/0x181769c30/0x181783e30）定真实消费字段→完整条目改写；② 查找钩返回克隆伪造条目；③ 桶链注入全新注册条目（配合 lua merge_cache 双同步）；④ 大厅/mini-runtime 换游戏流程逆向（线上生产通道）；⑤ GameUnit/EFFECT 表 vfunc+0x70 补捕。

## 进度日志

- 2026-08-23 任务启动；通读两仓库 research；云变量 Lua 链溯源完成（cloudvar-01）。
- 2026-08-23 渲染静态穷举（render-01）；云变量静态补遗（cloudvar-02）。
- 2026-08-23 晚 云变量：消息 ID→双向帧解码→直连 PoC（cloudvar-03/04/05）。
- 2026-08-23 深夜 渲染 PIE：特效直路径、sprites/spine、imgui 激活 webview/video（render-02/03）；固化 entrance_sniff。
- 2026-08-24 凌晨 模型链全解（render-04：actor/set_asset 实测、数编注册链、运行时注入全证伪、entry_data.ini 脚本化路径）；webview canvas2d+run_js 实证（render-05）；云变量全操作矩阵（cloudvar-06：MessagePack 值编码、op 码表、权限矩阵）；固化 entrance_client（Rust 直连读写验证通过）。
- 2026-08-24 凌晨3点 **unit_change_model 完整破解**（render-06）：native 定位链（.rdata luaL_Reg→wrapper 0x12a7f90→impl 0x12a23b0→apply 0x1785350）；根因=客户端 preview 单位负 id 在 native 注册表 lookup 失败静默跳过；真实单位（英雄 id=1）+ prefab 相对路径 → 本地吉鲁鲁模型视觉切换实证（双截图对比）；getter `game.get_unit_model_path` 可直接验证。frida 坑沉淀：模块名 SCEEngine.dll 大小写、frida17 `Process.getModuleByName`、PIE 三进程布局。
- 2026-08-24 凌晨3点半 决定性三态验证：U15 平面态（角色消失只剩红框）/U16 吉鲁鲁蓝紫态/原模型深色态三截图定案；attach_model 签名反汇编（arg3/4 挂点可省，无挂点无可见效果）；**base.wait 并发注册语义坑**（同 tick 多个 wait 全相对注册时刻触发，非串联）；W2 补测 webview 前置开关（render-05 §5：编辑器里检测=true、创建成功但页面不加载，死亡点=StateGame 渲染驱动缺失）。
- 2026-08-24 凌晨4点 用户线索 managed dll 逆向（render-07）：TNND 解密 6 dll + dnfile 元数据；GameCore=触发器逻辑核心、GameGraph=C# 场景图（Spine/Material/ResourceCache/Sprite2D）；**Sprite2D.TextureRect = 图集源矩形能力在 C# 层存在**（lua 未暴露，触达路径待查触发图节点）；云变量 C# 侧仅缓存容器无突破点。
- 2026-08-24 本轮 **★★ 重大版图发现**：沿 scegame-tester 字符串（wasm `*_Import` / `StateGame Open Wasmtime` / `RuntimeMcpBridge.cpp`）挖出**星火 2.0 = WasiCore 官方 C#/WASM 栈**，本机编辑器已随包携带（WasiCoreSDK v18 全文档 + map_templates/64 code_sample 约30个 C# 示例 + gamesparkcore/gamesystemui AppBundle + api_pak_version.json "2000" 注册表）。触发图 Sprite2D 线索证伪（V1/V2 节点无），但 2.0 官方 API 全覆盖用户痛点：StaticSprite2D.TextureRect 图集源矩形、Canvas(NanoVG).DrawImage 源矩形（线上可用的官方 canvas）、RuntimeParticleBuilder 免数编特效 + 本地 .effect 直载、Prefab.Load 本地模型、AnimationSet2D Spine/Spriter、RTT 离屏渲染（wasicore-02）；云数据六桶+UUID 列表+名称注册表+跨用户 ACID 事务+游标扫描（wasicore-03）。**1.0 lua 项目不可用**（api 2000 独立项目类型，无 lua+C# 混合）；迁移=官方 migrate-1to2 AI 手册，逻辑重写中大成本（wasicore-04）。落盘 wasicore-01~04 四文档。
- 2026-08-24 本轮 entrance_client 扩展落地：`set`/`ladd` 子命令（op0/op15，手写 JSON→MessagePack 编码器复刻观测 bin8 家族编码）线上实证字节级复读一致（cloudvar-06 §6）；rmp-serialize 依赖移除（其 str 编码与观测不符）。
- 2026-08-24 本轮 attach_model 攻坚（render-09）：① **socket 命名规范破解**——prefab TNND 解密出 JSON sockets 表（boneName↔socketName），sk_basic2/new_sk_basic2（注意 `sockket_weapon_l` 官方拼写错误与 `Socket_Root` 大小写差异）/jilulu 全表落盘，2.0 官方注释实证挂点名=socketName；② native 链补全（attach wrapper 0x12a20d0→核心 0x18176e8b0→内层 0x1817af940，同 path attach 有 toggle 语义，要求 [unit+0x1f0] 组件非空）；③ **客户端全矩阵证伪**（U22~U25：单/双挂点、socketName/boneName/武器骨、arg5=true、官方武器 sm_jian，45s 长窗口截图全部无可见效果）；④ 官方 1.0 附加模型=数编 ActorAdditionModel（game_p_2xgc/1ax1 pak 镜像实证，自动骨骼挂载，socket 可空）。坑沉淀：MCP capture_game 单张 ~30s（短窗口落空教训）；restart_last_debug 疑似不拾取新增 lua 文件（服务端新探针文件未生效）；固化一次性探针 test/temp/dec_strs.py。下轮：frida hook 定死亡点 + 服务端 attach + 数编 ActorAdditionModel 实建验证。
- 2026-08-24 本轮 **★ render-10 官方 actor 附着通道完整打通并视觉实证**：① frida hook（VA/RVA 坑修正后）证实客户端 `unit_attach_model` 三级函数零命中=死 API，发现活 API `attach_actor_to_socket/anchor`；② **数编脚本化全流程踩通**——外部写 entry_data.ini 的生效链：删 `script/obj|ui/script/obj/save_info.json` + bump editor/table 戳 + 重开编辑器 + full 调试（时间戳闸门是最大卡点；编辑器开着写条目再保存会被清理逻辑删除）；③ 条目模板固化（ActorModel 内嵌 Model/ActorAdditionModel/独立 Model 三套）；④ U27 实证：`base.create_actor_at('$$p_55a3.actor.bgd_jilulu_attach.root', point)` + `a:attach_to(1,'socket_overhead')` → 吉鲁鲁附着英雄头顶（截图 capture_1787523735）；⑤ 工具固化 find_luareg.py（luaL_Reg 定位）；坑：日志方括号数字=线程 id 非 pid（PIE 游戏态进程 exe 名=SCE）。
- 2026-08-24 下午 **★ 三连突破**：① **message_\* 签名全解**（cloudvar-07/08：send=(player,key,target_user_id,value,events?)；wrapper 定位=pushcclosure 注册块 RIP-xref + pefile IAT 反查；中文错误串池定参数语义）；② **B 模式证伪**（cloudvar-08：debug 局客户端 UDP 直连 host 无 Entrance，sce.s 全黑洞——B 模式不能做云变量游戏态实验；凭证/登录 flags 权限实验全证伪）；③ **真 tester 局打通**（cloudvar-09：MCP publish_project 发布 pak v97 → **`tester_1089.exe -game=p_55a3 -tag=test -ai_test=1` 免交互自动进真局**——关键坑：参数必须 `-key=value`，空格形成空值 flag 卡死 app_android 壳）→ 真局权限矩阵（money_add 解锁；rank/message/money_init=地图级授权，真局仍拒）+ **imgui 通道线上实证**（render-12：video 播完 https mp4、webview canvas2d 棋盘上屏、不崩）。themis 实锤挡 frida attach（DeviceAttachError）。工具固化：entrance_client 扩 money/rank/urank/qitem/names + ENT_F4/ENT_LOGIN_EXTRA；entrance_sniff 模块识别修正（sceengine.dll 优先，wineditor hook 点 send 0x1aa3770/recv 0x1aa1b19）；find_str_refs.py；click_win.ps1（SC_RESTORE 是唯一有效恢复）。**坑：frida attach 前必须核对进程 Path**（PIE 残留进程与游戏进程同名 sce/SCE，挂错拿到貌似合理的假数据）。
- 2026-08-24 傍晚 **★ 直连限流与修正 + GameWorld 攻坚**（cloudvar-10 + render-13）：① burst 压测实证限流=~57 并发窗口 code=25（无分钟硬计数，10s 恢复，稳态 130ops/s）——直连读写性能天花板量化；② **money_add 全环境放行**（lobby/直连/真局实测，修正 cloudvar-06/09「真局才解锁」误判）；货币读写不对称=服务端设计（commit 写全放行、查询按地图授权）；③ item_add 签名定版 `(player,key,item_name,count,extra,expire_type:int,expire_time?)`（三次迭代+反汇编+线上 ok）；④ GameWorld/UIWorld 组件栈全解（defaultui_63 + p_2xgc 生产用法）：StateGame 建世界/载 actor/渲 RT 不崩；**image 控件吃 RT 链接硬崩无 dump（新坑）**；scene 控件全属性变体不渲染（死亡加固）；显示侧剩数编 UIScene 模板 + BindToUIScene 唯一路径（下轮 entry_data 脚本化）。工具固化：entrance_client 增 burst/madd/iadd；探针 GameWorldProbe（隔离开关式崩溃归因法）。
- 2026-08-24 晚 **★★ render 主线最后一块闭合**（render-14）：① **base.ui.scene 官方流破解**——script-199 引擎组自测文件（test/scene.lua 等）给出 `base.ui.scene(props)` 建模板 + `base.ui.create` 实例化 + `independent=true` 出画面三要素，render-03/13「scene 控件 StateGame 死亡」结论修正（死亡的是用法不是控件）；model={name=显示名}/particle={name=粒子名} 通道存在但名称解析规则未全摸清；② **手写 GUI 页面脚本化通道**（免编辑器）：ui/script/gui/page/<名>/{template,component}.lua + init.lua 注册一行，restart_last 即生效——比 entry_data.ini 数编脚本化更直接；③ **UIScene 通道全通**：`base.gui_new('页')` + `base.gui_get_part(page,'控件名')`（=page.part[name][1]，生产 p_2xgc 实证）+ `UIWorld:Create(false,CAM,'default')` 载真实地图 + 手动相机对焦 + `CreateActor(数编actor link)` + `BindToUIScene` → **吉鲁鲁本地模型在 UI 控件内清晰渲染**（截图 capture_1787566974）；④ 旁证：裸 `base.ui.view{type='UIScene'}` 可建 native 控件但 set_control_prop 全静默吞（无属性校验），不可走；'@gameui' 包 lua 物理位置未找到（待查）。
- 2026-08-24 晚 **用户实证收口**：render-14 UIScene/UIWorld 通道编辑器与线上发布全部生效 → render-11「自定义模型线上靠 user_libs 分发」假设坐实（E2E 闭合）、render-13 标记的 PCBox RT 管线风险排除。渲染主线（本地模型/图集/视频/webview/特效直路径）全部打通并线上可用。剩余尾巴：scene 控件 model/particle name 解析、'@gameui' lua 物理位置、ActorEffect 进 UIWorld、特效直路径线上复验。
- 2026-08-24 晚 **双尾巴闭合**（render-15/16，子代理静态攻坚）：① **'@gameui' lua 定位**——`D:\sce_online\update\editor-pd.spark.xd.com\res\_m\gameui\48\gameui\ui\script\`（api13→48；镜像 gameui-52 无 script 是 api2000 包残缺之故），decrypt_mirror 解密 62 个明文落 test/temp/gameui-48-script/；require 解析 = sceengine native PathSearcher 内置前缀表（'@gameui'→gameui/ui/script）；uiscene.lua 证实 RenderTarget=bind 属性转发内层 panel image；组件目录全量可读。② **scene 控件 name 命名空间破解**——model.name=**数编单位表节点名**（非 actor/model 条目/显示名；'主控'实证出图、其余两写法为空 + 引擎自测'斧王/剑圣'全是单位名 + native GetUnitTableEntryByName 佐证），particle.name=特效表节点名；light 游戏态疑似不生效（要光照走 UIWorld）；independent=true 是 StateGame 出图前提；knead_human 语义定（part_name/value/part_cloth/save+avatar_path 出头像）。不确定点清单（节点名vs显示名/动态 bind/anim 形态等）待 PIE 实测。
- 2026-08-25 **★★ render-17 免数编攻坚 G18~G30 全闭合**：特效进 UIWorld（G21 ActorEffect 条目+play('cast') 横幅实证）；**自定义 renderpath 吃项目包内 xml**（G22 红底实证）；SCE native API 目录 dump（G23）；手建 actor 通道活（G27 双吉鲁鲁）；**免数编三连判死**（G24/25 假 link=nil、G28 set_asset 须数编 ID〔用户复验修正〕、G29 merge_cache lua 通/native 拒、G30 特效裸路径无效）；新坑 use_light_group 硬崩。详见 render-17 文档。
- 2026-08-25 **★ render-18/19 静态逆向双落地**：① native 注册表=GameDataManagerImp 按 typeid(u64≈hash(link)) 索引的表族，加载仅 LoadMapTable 启动期（一次性闸），全 dll 无 Register/Reload 字符串——运行时插入只能 frida 复用加载期 insert；② set_asset wrapper 对参数零解析直接虚调 vtable+0xa0，ModelActor/EffectActor/GameUnit 同一 wrapper；③ 特效侧发现「字符串哈希注册表+裸路径回退加载」函数 0x18129fb50（G30 矛盾总钥匙）；④ GameWorld load_map=注册表地图名加载场景四件、set_map_dir 纯赋值、use_light_group +0x28 为 null 即硬崩（render-19）。
- 2026-08-25 **★ render-20 实证收口**：① G33 用用户 virtual_effect.lua 原样三入口终审虚拟数编（CreateActor/ModelActor.new=nil、set_asset 静默 no-op、视觉对照确认；lua 层全通）——虚拟数编有效域=仅 lua cache 消费者；② native 日志 12 会话扫描：**表加载=会话启动一次，任何 load_map（含 UIWorld innerWorld/bogus 图）永不触发表加载**——伪造目录注入路线判死；③ 新坑：引擎 lua 不接受 `|` 前缀注释行（整卷判死），外部文件需先清洗；restart_last 能拾取已入库文件内容变更。渲染免数编收窄至 frida 运行时注入唯一主线。
- 2026-08-25 **★★ render-21 动态逆向爆发**：① frida 链式解析打通（set_asset impl→[actor+0x28]vfunc+0x60→manager→vtbl 查找族）；② set_asset 分派链终版（ModelActor=0x17837d0/vfunc+0x60 MODEL 表、EffectActor=0x179c0a0/vfunc+0x70 EFFECT 表，miss 静默 je；0x18129fb50 排除归 UI particle setter）；③ ACTOR 表桶链全解（mgr+0x230，node{next,typeid@+0x18,entry 内联@+0x20}）+ **typeid=djb2-32(link) 精确验证**；④ MODEL 条目布局 dump（Asset=entry+0x28）；⑤ Tier-1 条目纯路径改写判死（apply 消费预载 mesh 句柄）；⑥ frida17 坑：3 字节 call 拒钩、readUtf8String(N) 遇 NUL 抛错。
- 2026-08-25 **★ render-22 换图机制移植实验（用户 20:26 线索）**：① xdeditor 重载链源码级全解（强制重新加载=EDITOR.unload_map+load_map；打开=+update_map_libs；EDITOR 全局=native 注入 xdeditor lua 态）；② **游戏 lua 无此绑定**（G34a 四层 dump 实证）；③ 模块级 LoadMainMap/reset frida 三轮直调判死：**同名 no-op（strcmp 门控，当前图标识=绝对路径）、异名 ret=0 但进程内存爆冲 32GB 死亡、reset 拆卸即 AV**——游戏客户端会话无法承受换图，重载必须编辑器侧完整编排；④ frida 直调通路打通（getter=(L,magic)→ctx、Win64 参数序坑、lua54 165 导出、坏进程 [ctx+0x10] 鉴别）；⑤ UIWorld CreateActor 对 lua 缓存缺项会抛 lua 错误（uiworldscript:279 不防御），与 native nil 是两种死亡形态。下轮：apply 链消费面逆向 / 克隆伪造条目 / 大厅 mini-runtime 换游戏流程（线上通道）。
- 2026-08-27 **知识修正（direct_load 任务 dl-01）**：依据 [webview-bridge.md](../research/webview-bridge.md)（双向桥全通 + 三端上线实测）与 [pak-io-native.md](../research/pak-io-native.md)（pak 提取 + 绝对路径视频播放三端实测），修正 render-03/05/12 及本 README 中过时的 webview/video 结论：「JS→lua 未通」→ 已通（imgui 控件需登记 base.ui.map + register_event）；「video 线上会崩」→ render-12 证伪；「CEF」→ miniblink；「scene 控件死亡」→ render-14 已修正。逐条修正清单见 [dl-01-knowledge-corrections.md](../direct_load/dl-01-knowledge-corrections.md)。

## 关键已知结论速查

### 云变量
- `sce.s` = native 全局表（LuaScore.cpp），无 Lua 封装层；发送点字符串 "Send Scorearchive msg to Sntrance failed."（sceengine-strings:444852）。
- 通道 = 平台 Entrance 长连接（EntranceHeader + 数字消息 ID + protobuf CEProto::ScoreArchive::Msg），服务端称 ScoreServer，存储后端疑似 MySQL（"NOW() + interval" 拼接 + 管理面 "SQL failed" 错误码）。
- 游戏 lua 无"向 Entrance 发自定义消息"的暴露函数；直读直写 = 仿 Entrance 客户端协议 或 native hook。
- 下一步：proto_extract 提取 CEProto::ScoreArchive；find_xref 定消息 ID；frida 抓 Entrance 明文。

### 渲染（既有研究 ui-render-atlas-canvas.md，editor-patch 仓库）
- ui.* 注册块 = 官方 native 边界全集（sceengine-strings:443417-443571）；控件属性无 UV/源矩形。
- canvas_texture_* 编辑器可用、线上 PCBox 硬崩（平台 bug，不可依赖）。
- **imgui 立即模式直驱（ui.imgui_begin_view/begin_ui/props）= StateGame 更底层通道**：webview/video 控件只有它能激活（render-03）；webview 是引擎内嵌离屏渲染（【2026-08-27 修正】miniblink 合成进 UI 纹理，非 CEF/HWND overlay）；**lua↔JS 双向桥已全通**（imgui 通道手动登记 base.ui.map + register_event；三端上线实测，[webview-bridge.md](../research/webview-bridge.md)）。
- **视频已完整突破（生产级）**：pak 提取（io.ExtractPakFile PascalCase 漏网）+ 绝对路径/file:// 播放 + 双轨音频 + 三端实测，见 [pak-io-native.md](../research/pak-io-native.md)。
- **特效 .effect 直路径可用**（particle 控件 effect 属性；属性类型必须 number/table，字符串静默不渲，render-02 §1）。
- spine .skel 直路径自由；~~scene 控件 StateGame 死亡（模型 UI 通道未打通）~~ → **render-14 已修正**（死亡的是用法不是控件；UIScene/UIWorld 通道本地模型进 UI 编辑器+线上均生效）；**unit_change_model = 世界内本地模型通道（render-06）：真实单位 id + prefab 相对路径即可，preview 负 id 无效**。
- 待补：GameWorld+viewport 复刻、~~webview run_js 双向通信~~（2026-08-26 双向桥全通，webview-bridge.md）、模型世界内通道实测。

## 进度日志

- 2026-08-23 任务启动；通读两仓库 research；云变量 Lua 链溯源完成（cloudvar-01）。
- 2026-08-23 渲染静态穷举完成（render-01）；云变量静态补遗（cloudvar-02）。
- 2026-08-23 晚 云变量连续突破：消息 ID（0xA000）→ 双向帧解码 → **纯 Python 直连 Entrance 读写云变量 PoC 成功**（cloudvar-03/04/05）。
- 2026-08-23 深夜 渲染 PIE 实测：特效直路径破解、sprites/spine 实证、**imgui 通道激活 webview/video**（render-02/03）；固化 Rust 工具 entrance_sniff（examples/，验证与 Python 版一致）。
- 工具固化备注：test/temp/ssl_sniff.py + decode_entrance.py + cloudvar_poc.py 为研究一次性脚本；正式工具 = examples/entrance_sniff.rs（Rust，LIBCLANG_PATH 需指向 pip libclang 的 clang/native）。
- 载荷沙盒现状：mini-runtime runtime/ 的 startup main.lua 仍带 CloudProbe 注入（原始件 同目录 .tnnd-bak）；test_res002 .bgd/src/client/ 留有 CloudProbe.lua/RenderProbe.lua + init.lua 两行 require（均标注勿提交 git）。
