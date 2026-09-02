# 自建 host 可行性研究（2026-09-02）

> 目标：mini-runtime 实现完整「自建 host」（替代官方云端调试 host），云端 host 保留为界面可选项。
> 方法：静态考古（sceengine.dll 字符串 / xdeditor-160 / script-199 / client_base-78 明文）+ update-info 通道探测 + 实机实验（编辑器补丁 MCP 驱动）。
> **一句话结论：中继 host 已交付并双链路验证（§9）；会话面已破解到"c2h 全明文 + h2c=无密钥 ZCompress 压缩"（§10，初判"加密"已翻案）；server VM API 面已 oracle 全量取证（§11）。对 test_res002 类自研逻辑项目，真本地 host 的剩余工作 = GameHost.lua 编排复刻 + 薄 native shim（游戏用到的 native 面极小）；通用服务端引擎本体（977 键 base 的 native 实现）仍不可得（§4）。**

## 1. 自建 host 的三层组成与现状

| 层 | 内容 | 现状 |
| --- | --- | --- |
| 控制面 | TCP 5003，0xF000 段协议（EditorLogin/上传/起局/日志回推/心跳/销毁） | **已完整逆向并实证可自建**（scegame-reverse.md §8；editor-patch examples/host_stub.rs 全链跑通编辑器侧） |
| 会话面 | KCP（UDP）游戏会话：客户端进局握手/login(userid)/地图加载/状态同步 | **已破解**：CE1 握手族 + 标准 KCP + 3B 流分帧 + c2h 明文 protobuf/cmsg_pack + h2c ZCompress 压缩（§10 / scegame-reverse.md §13） |
| 服务端运行时 | 跑地图 script/main.lua 服务端半身的引擎（host.exe = NE 引擎 server 构建 + GameHost.lua + server_common/server_lua_plus 包） | **不可得（本文 §4 五条独立证据）** |

## 2. 控制面接入点（实锤）

- 编辑器「调试(本地服务器)」（inner 菜单，menu_bar.lua:2124）/ argv `use_local_host`（map_starter/init.lua:111）= 固定连 `127.0.0.1:5003`，token 写死 `'qwert'`。
- **编辑器侧没有任何按需拉起本地 host 的逻辑**（2026-09-02 实机实验：5003 空监听下触发本地服务器调试，`update_host` 直接返回 ret=-3 管线中止；全程无进程拉起、无目录创建）——5003 的对端完全由外部提供，即自建 host 的官方接入点。
- use_local_host 模式下 PIE 客户端拿到 `-host_ip=127.0.0.1 -host_port=5003`（editor-patch 侧实证）；**KCP 会话端口 = 控制端口 + 50（引擎硬编码）**，客户端实际 dial 5053（§9 / scegame-reverse.md §13.1）——自建 host 的 UDP 必须双端口监听。

## 3. 编辑器/客户端引擎不含 server 半身（字符串实证）

sceengine.dll（version-13，editor 构建）全量字符串考古（证据件 `test\temp\test_strings_gameplay.txt`）：

- GamePlay 类全集 = `GamePlayBase / GamePlayLocal / GamePlayOnline / GamePlayLobby / GamePlayLive / GamePlayReplay / GamePlayInEditor`——**全是客户端形态，无 GamePlayServer**。
- 网络消息处理器只注册了 host→client 方向：`NetEventHandlerImpl2<GamePlayBase, _ACGame_Protocol_GameHostServer_*>`（CameraFocus/FovModeChange/SightChange/SyncUnitAttributeConfig/UnitEvent/UnitModelUpdate/UnitOwnerChanged/UnitParticleUpdate/UnitStateMachineTransit/UnitStateMachineUpdate）；client→host 方向的 handler 注册不在此二进制（在 server 构建里）。
- KCPNetwork 实现完整在客户端（`[kcp] KCP will connect to %s:%d` 等，源路径 `D:\BuildPC\NE_pd\Client\src\Game\Network\UDP\KCPNetwork.cpp`）。
- host 相关字符串：`bin-release/host.exe`、启动模板 `-d -startupmode=2 -nogfw -noserviceagent -port=%d -config_path="%s?.lua" -log_folder="%s" -lua_log_on_console`、`/GameHost.lua`、`/server/gamehost/`、`User/server/gamehost/config/`、`GameHostServerLauncher`（copy/CreateDir/DeleteDir 错误串）、`/data/GameHostIP`、`/data/GameHostPort`。
- xdeditor lua 层对 host 编排零参与（gamehost/GameHost/host.exe 全库 0 命中）；`DebugManager` 全为 native 绑定。

## 4. 服务端运行时不可得（五条独立证据，2026-09-02）

1. **本地全盘无**：`D:\sce_online\**\host.exe` 与 `**\gamehost\**` glob 零命中；`GameHost.lua` 不在任何客户端载荷。
2. **pak 内无**：Update Res 下 4 个 pak（fonts/uistyle/xdeditor/xdeditor_startup，含 TNND 解密后）grep `gamehost|GameHost` 全不命中；script-199/client_base-78 明文镜像同样 0 命中。
3. **update-info 无 host 二进制**：`variation=server` 变体真实存在（见 §5），但 `list=host/gamehost` 返回空 ref_items（包不存在）。
4. **server 包 OSS ACL 拦截**：server_common/174、server_lua_plus/14、global_default/60、甚至地图 p_55a3/1 的 server.zip 全部列得出但下载 403 `AccessDenied: bucket acl`（代理/直连均拒）——sce-maps-pd-backend 桶不公开，官方 host 在阿里云内网拉取。
5. **引擎不会自举 host**：debug_via_remote=0 实机实验（§6）证明本地没有任何可用的 server 启动路径；`GameHostServerLauncher` 建的 config 目录为空（copy 源不存在）。

## 5. ★ update-info `variation=server` 通道（新发现）

`POST https://updater-pd.tapsce.cn/api/map/update-info?...&variation=server` 返回**服务端构建包**（bucket = `sce-maps-pd-backend`，文件名 `server.zip`）：

- `server_common` v174（449KB）、`server_lua_plus` v14、各依赖库（lib_control/defaultui/spark_core/lib_ui…）与地图本体（p_55a3 v1, packet_type=1）都有 server 构建；
- 查地图名时 ref_items 自动带出 `server_common@174 + server_lua_plus@14 + global_default@60`（与 EditorStartGame f12 固定三库一致，互为印证）；
- 下载被 bucket ACL 拦截（§4.4）。若未来拿到可读凭证，此通道 = 服务端 lua 栈的完整来源。

## 6. ★ debug_via_remote=0 实机实验（2026-09-02）

方法：编辑器运行时 `common.add_argv('debug_via_remote','0')`（native argv 存活着读，menu_bar.lua:56 的 lua 本地变量仍是加载期的 true）→ 触发「调试/调试」。

实录：

1. lua 侧照常 assign_host（106.14.95.227:13170）+ update_host ret=0（云控制连接建立）；
2. **native debug_game 读到 argv=0 → GameHostServerLauncher 激活** → 创建 `D:\sce_online\User\server\gamehost\config\`（空目录，copy 源不存在）；
3. 无 host.exe 拉起、无新监听端口、无 server 进程；
4. PIE 客户端进程被拉起（lua-game 日志文件创建）但**日志 0 字节**——客户端没有可连的 host，进局失败。

结论：`debug_via_remote=0` = 「host 由本地 GameHostServerLauncher 拉起 host.exe」的官方内部开发路径，**正式版环境因缺 host.exe 与 server config 源而必然失败**。GameHostServerLauncher 的契约（copy config → 拉 `bin-release/host.exe -port=%d -config_path=...`）即官方本地 host 的集成面。

## 7. server VM 契约（供未来实现参考）

服务端 lua 在 host.exe 的 lua VM 里跑，加载链与客户端同构（common/init→main→base/init），差异层由 server_common 包提供（`base.game:ui` / `player:ui` / 服务端收包回调等，客户端包里没有）。host 必须提供的 native 面（据 script-199/client_base-78 对称性推断 + 调用点实证）：

- 消息层：cmsg_pack.pack/unpack、ui_message 双向投递（客户端侧对称物 `base.event.on_ui_message(_new)`，script/common/base/server.lua:193/214）、`game.send_ui_message` 的服务端镜像、`base.event.on_server_clock(clock)` 心跳驱动（server.lua:67）、`base.event.on_update` 帧驱动；
- 运行时公共件：log/log_file、common.*（has_arg/get_argv/...）、include/require_folder、`__lua_state_name`、`__MAIN_MAP__`、lni；
- 世界模拟层（最大）：数编表、单位创建/属性同步/视野、场景管理——本质是引擎 GamePlay 的服务端半身。

## 8. 可行架构选项

- **A. 中继 host（本地门面 + 云端芯）**：自建进程监听 5003（TCP 控制面完整实现 + UDP KCP 按客户端源端口做 NAT 式转发到 assign_host 分配的云 host）。编辑器/客户端全部指本地，玩法真实。价值 = 统一入口/全流量可观/为真本地化占位；**不脱机**。**✅ 已实现并端到端验证（§9）**。
- **B. 壳 host（脱机但无玩法）**：控制面 + 自研 KCP 最小会话（login/start/scene），客户端能进图渲染但服务端逻辑缺席（Req_* 无人处理）。前置 = ~~KCP 会话协议抓包逆向~~（已完成 §10）→ 现瓶颈 = ZCompress 位流格式复刻（无密钥，§10 ⑤）+ 最小会话消息序列确认（可经中继 capture + VM hook 取证）。bgd 游戏的服务端驱动度高，空壳客户端很快卡在等同步。
- **B+（test_res002 类自研逻辑项目的真本地 host）**：游戏服务端不用引擎单位模拟（native 面仅 event_register/ui/proto/auxiliary/log/帧事件，§11），GameHost.lua 编排 + 薄 native shim 即可跑真玩法——**对本项目这是可达终点**；server_common 本地不可得但可用 script@199 近似（用户逆向结论：阉割客户端内容+加服务端内容）。
- **C. 真本地 host（完整脱机）**：需要 host.exe（或等价 server 引擎）+ server 包，§4 已证全部渠道拿不到。**当前不可行**，除非官方开放 server 包下载或分发 host 二进制。

## 9. ★ 中继 host 端到端验证（2026-09-02，M1~M3 交付，已双链路验证）

实现：`src/core/local_host.rs`（TCP 控制面协议感知中继 + UDP KCP NAT + 全流量 jsonl capture），CLI `host start` / `debug start --host local`，GUI 调试页 host 模式下拉（云端直连默认保留）。

**关键协议发现——KCP 会话端口 = 控制端口 + 50（引擎硬编码）**：

- 中继首版只监听 UDP 5003，客户端 Network 落实锤：`KCP will connect to 127.0.0.1:5053`（命令行 `-host_port=5003`，引擎自行 +50）；云端对照 `13738→13788` 同规律。
- 症状：5053 无监听 → ICMP 不可达 → 客户端 `recvfrom() error 10054` 刷屏 → `kcp connect failed at 4.010000` → **lua VM 不起**（lua-game 0 字节）。KCP 建连是 lua 启动的前置闸门。
- 修复：UDP 双端口监听（5003/5053），5053 转发到 `cloud.port + 50`。

**验证结果（两条入口链路全绿）**：

| 入口 | 链路 | 结果 |
| --- | --- | --- |
| `debug start --host local` | CLI → 中继 → assign → 上传 312 文件 → 起局 → 外部客户端 KCP 127.0.0.1:52532→106.14.95.227:20820 | 客户端 lua-game 正常（on_enter_game/技能下发/Sync_PlayerStats），截图确认进局渲染完整 |
| 编辑器「调试(本地服务器)」 | 菜单（token=qwert 本地放行）→ 中继换真 token → 云端 → PIE KCP 127.0.0.1:53030→20820 | lua.game_info=StateGame，capture_game 截图确认 PIE 进局 |

**排障纪律**：assign/云连接失败必须回 0xF001 result≠0，否则编辑器 `co.call(DebugManager.update_host)` 永远悬挂卡死调试管线（已内建 login_fail 闭包）。

## 9.5 ★★★ 壳 host（真本地会话面，0.5.0 R3 交付）

实现：`src/core/host_server.rs`（控制面 TCP 服务端）+ `src/core/kcp_server.rs`（CE1 握手 + KCP 服务端 + 3B 流分帧）+ `src/core/game_host.rs`（会话编排）+ `src/core/host_templates.rs`（官方 h2c 消息序列模板，AUTO-GENERATED）。入口：`host start --shell`（PIE 用）/ `debug start --host shell`（自带客户端拉起，CLI 常驻承载）。

**行为基线 = scegame-reverse.md §13.9 的登录→进局序列**：登录应答模板（type 2 + 0x15）→ 客户端自驱进度 30/45/95/100 → msg 5 触发初始化消息群（模板原序：0x6/0x102/0x100/0x7008 __sync_game_info/0x112/0x5004/0x1129/0x1105/0x1120/0x10e/0x103/0x10d/0x109/Sync_* 全量集）→ 0x31007 tick（200ms 周期）+ 0xF100→0xf101 回显 + 0x1001→0x1108 应答。h2c 发送走 ZCompress 原样模式（§13.8 旁路）。0x6011/msg 5 收到即弃；0x7006 玩法上行壳期忽略。

**踩坑记录**：
- Windows UDP 绑定特异性：中继绑 127.0.0.1:5053 与壳 host 绑 0.0.0.0:5053 可共存，包投递给更具体的绑定——调试期僵尸中继会"劫持"客户端流量，症状 = 壳 host 零收包。排障先清场。
- 起局信号同步把 `push_log` 放在 `state` 锁内 → Mutex 不可重入死锁（线程冻结于 起局，之后客户端 KCP 无人应答）。纪律：锁内只取标志，动作出锁再做。
- 客户端 ping（0x1001→0x1108）校验「发送 sessionid == 接收 sessionid」：登录应答/0x1108/__sync_game_info 三处必须一致——壳期固定为基准 capture 的常量（GAME_SESSION_ID，免补丁）。
- 客户端 CE1SYN ~10ms 快速重发：同一来源重复 SYN 必须复用既有会话（否则 conv 漂移）。

**验收实录（2026-09-02）**：`debug start --host shell` 全链——控制面 EditorLogin/上传 1277 文件（逐文件 0xF010）/EditorStartGame → 客户端 KCP CE1 握手 → login result[0] → 客户端自驱加载 100% → 初始化消息群 → 客户端日志 "Game host notify loading finished / notify start game"（与官方会话逐行一致）→ 截图确认沙漠场景+HUD+角色渲染（玩法缺席为壳预期）。零 assign、零云端外联。

## 9.6 ★★★ GameHost 编排复刻（真本地服务端，0.5.0 R4 交付）

实现：`src/core/lua_host.rs`（mlua lua54 vendored 内嵌 VM + shim 面 + 自研 require 加载链）+ `src/core/cmsg_pack.rs`（cmsg_pack msgpack 变体 pack/unpack + lua 互转）+ `game_host.rs` 接线（起局建脑/登录玩家连入/0x7006 路由/出站 0x7008/50ms 帧泵/停局毁脑）。

**开工先决两决策项**（需求 R4 要求记入提交说明）：① lua 宿主选型 = **mlua（lua54 + vendored）内嵌**进 mini-runtime 进程（game_host 线程持有 VM，单进程零 IPC；lua54 = 引擎同款语义：整除/位运算/goto/utf8）；② lua 运行时物理落盘 = **全部磁盘现读、零内嵌**（项目树 = 控制面落盘的 `runtime/User/host_upload/<project>/script/`；引擎库 = 载荷 `_m` 按 EditorStartGame f12 版本表；server_lua_plus 用本机明文包）。

**消息路由**：c2h 0x7006 {f1: cmsg{type,args}} → `base.ui.proto[type](player, args)`（引擎内建通道 `__client_key_down/up` 不走 proto，由 host 原生转 玩家-按键按下/松开 事件——script-199 game.lua:517-525 实证）；服务端 `base.game:ui(name)(data)` 广播 / `player:ui(name)(data)` 定向 → 0x7008 {f1 cmsg(args), f2 seq, f3 type_id, f4 type_name（首现携带）}。**广播在无就绪会话时挂起、进图 burst 后补发**（官方「后进玩家拿世界状态」语义；BOSS 5s 首刷早于客户端接入则永久隐身——test_res002 实测）。

**事件泵双形态**（官方语义泛化，非游戏特判）：`event_register` handler 位参直调（玩家-按键按下 = (trg, player, key)）；触发器 `add_event_common` handler 收 `(当前触发器, e 表)`（e={evt_name, player, key, key_keyboard}）。

**踩坑记录（全部实机实证）**：
- **mlua 0.10.5 `Vec<Value>` 传参吞前导 nil**：`f.call(vec![Nil, t, b])` 到 lua 侧第二参变 nil（独立探针 test/temp/mlua_probe 实证，已删）；必须 `Variadic::from_iter`。症状 = 事件 handler 第二参（playerObj）神秘为 nil。
- **引擎 lua 词法器放行 ≥0x80 标识符**（TSTL 产物含中文参数名，global_default/lua_declare.lua:62）；stock lua54 拒绝 → 加载前 `sanitize_lua` 确定性改写 `_xHH`（字符串/注释不动、行号不变、同名同改）。
- **`base.clock()` 单位 = 毫秒**（官方 timer.lua cur_frame 按 on_update(delta*1000) 步进）；`base.wait(timeout, cb)` timeout 同为毫秒。给秒则全部时间驱动逻辑慢 1000 倍（BOSS 首刷 5s 变 1.4 小时）。
- **TSTL 类壳必须真实现**：`__TS__Class2/ClassExtends/SuperTypeArgumentsFuncWrapper`（prototype 链/____constructor/__call 实例化），lua_declare.lua 全量建类链，弱桩即炸。
- **`base.tsc.CLASSES.os` 必须预置真 os 库**：TSTL 产物 `os = CLASSES.os or __TS__Class2("os")` 覆盖全局 os；缺则 os.time 全灭（ShopSystem 限购/同步全炸，商店货币显示 0）。
- **`require('<dir>')` = `<dir>/init.lua`**（包目录约定，bgd_game_server 等）。
- **`Lua::unsafe_new()` 全库**（bgd log 模块用 debug.traceback）。
- **eff.cache 返回值补 Formulas 空表**：官方 cache 有 schema 默认值注入，obj 裸数据没有（trigger_validator 直接 `data.Formulas.X = fn`）。
- **cmsg_pack lua→线 数组判定必须「键恰好 1..=n 无额外键」**：`raw_len()` 只量连续前缀，稀疏整数键表（商店 bought={1..4,11..32}）误判数组丢尾部 → 客户端稀疏键读取全落空（每日/每周/每月已售罄不更新，特惠 1-4 正常——test_res002 实测对照）。
- **排障打点**：game_host 出站首现类型 println（载荷 cmsg_pack::debug_short 预览）；`玩法上行未注册 handler` 每类型记一次。

**验收实录（2026-09-02，test_res002 全脱机进局人工游玩）**：lua 加载链完整（bgd 四端初始化 + BagSystem/ShopSystem/GMSystem/GameServer/草丛连通区域 63）→ 玩家上线（OnPlayerJoin 全量执行）→ 移动校验（Req_PlayerMove 到达、越距拒绝+回拉）、攻击、技能（黑幕/解药恢复 102）、背包（拆分/获得/锻造/交换）、商店（GM 发放 money/gem → 各标签购买成功：free/money/gem 三货币 + 日/周/月限购 + 限量买完按钮变「已售罄」人工确认）、组队（创建队伍）、F1 触发器（`Srv_Verify_Key F1 ok` × N）、刷怪（BOSS 按点刷新 + 技能书刷取/超时循环 + 帧驱动回血 hp 95→200）→ 客户端截图渲染确认。**零 assign、零云端外联**。多人未验（待多客户端支持）。

## 9.7 ★ 三态 host 与本地账号多人（0.5.0 R5 交付）

- **host 模式三态**：`cloud`（云端直连，默认）/ `relay`（本地中继观测抓包）/ `local`（真本地脱机 lua 服务端）。**语义切换：0.4.x 的 `local` = 中继，0.5.0 起 `local` = 真本地**，中继由 `relay` 承接（CLI `--host` 与调试页下拉同步；`shell` 为开发期内部值，映射 local 并 warn）。触点：`src/core/debug.rs` HostMode / `src/main.rs` / `src/ui/debug.rs`。
- **多客户端**：CLI `debug start --clients N`（附加凭证按凭证库顺序取，须有登录态）；多开间隔 6s 串行（凭证注入 user_info 单文件互斥）；pidfile 多行、stop 全杀。
- **「本地服务器」标签页**（ui/local_server.rs + core/local_accounts.rs + core/local_play.rs）：SQLite 账号库（exe 旁 local_accounts.db，rusqlite bundled），账号创建/删除/每账号「启动/停止」；局未起时首个启动自动上传起局（控制连接全局持有保 0xF00C 日志通道）。
- **本地账号 = 合成凭证直通**：`login=1 + token=local-<userid> + token_type=11` 即可过客户端大厅自动登录闸门，**全程零网络零真实凭证**（玩家甲/乙 90000002/90000001 双开进局实证 2026-09-02）——这也是整体验收「断网+无凭证」的通行证。
- **多账号正确性**：登录应答/0x102 模板按实际 userid 原位补丁（varint 等长；本地账号 userid 90000001 起分配在 4 字节 varint 区间）。
- **PIE 验收（2026-09-02，lua 编排脑在线）**：编辑器「调试（本地服务器）」连 standalone `host start --local`（token=qwert 放行）→ 上传/EditorStartGame → lua 加载链 → PIE 客户端进局渲染（截图确认）→ 停止调试 0xF01B teardown → 第二局完整再接入。与官方本地调试完全同构。
- **host 界面生命周期**：「本地服务器」标签页 启动/重启/停止 host（game_host STOP 信号 + 控制面非阻塞 accept 轮询退出；重启=等旧实例让出端口后 ensure_running）。
- **0xF00C 位置/帧号**：编辑器「调试信息面板」的位置列=f4、帧号列=f3（我们曾写死 "shell-host"/0 被用户发现）。修复：lua 日志从 bgd 格式化文本抓 `[xxx.lua:NNN]` 进 f4，f3=逻辑帧计数（pump_frame 每帧 +1）；host 内部日志保持 pos="shell-host"。
- **踩坑（2026-09-03，v0.5.0 回归 bug）**：**Windows 下 `listener.set_nonblocking(true)` 后 accept 出的套接字继承非阻塞标志**（与 Unix 不同！），handle_conn 阻塞读帧即 10035 WSAEWOULDBLOCK → 服务端断连 → 客户端报 10053「你的主机中的软件中止了一个已建立的连接」。修复：accept 后显式 `stream.set_nonblocking(false)`。教训：控制面为支持停止改非阻塞 accept 轮询时，必须记得还原连接套接字。
- **踩坑②（2026-09-03，v0.5.0 多人 bug）**：**0x7008 的 f2 seq 与 f4 type_name 首现都是会话级状态**——客户端按连接维护 type_id↔名字映射表（§13.5 on_ui_message_new）。v0.5.0 错按全局首现/全局 seq，第三个客户端起收不到 f4 名字映射 → 玩法下发全丢（表现：第三人看不到任何玩家，Req_PlayerList 狂重试）。修复：seq/seen_types 移入 SessionBrain，广播按会话各自组帧，pending_broadcast 改存裸载荷（type_id, name, args）补发时按会话组帧。复验：三客户端 Req_PlayerList 各仅 1 次（修前第三人 5+ 次重试）。
- **踩坑③（2026-09-03）**：local_play 每个账号启动都重建 staging，局运行中客户端正占用 staging 文件 → 第三人起 staging::create 撞文件锁（os error 32）。修复：staging 只在局未起时生成，后续账号复用本局目录；合成凭证注入加 sharing 冲突重试（20×500ms）。
- **服务器日志面板（0.5.1）**：`core/logbus.rs` 总线（`srv_log!` 宏 = println! + 5000 行环形缓冲 tee），game_host/host_server/lua_host 全部 stdout 日志进总线；「本地服务器」页右栏日志面板（滚屏/暂停滚屏 = 只冻结自动滚动照收新行、关键字筛选、清空）。
- **host 停止联动（0.5.2）**：停止/重启 host 联动 taskkill 本页启动的全部账号客户端；页级 1s `request_repaint_after` 周期重绘——客户端被外部直接关窗时行状态自动回落（此前 egui 无交互不重绘，tasklist 探活永不执行）。
- **秒开（0.5.4）**：首启动 22.5s → 3.6s（无变更再启动 1s）。三板斧：① staging 内容级增量（`files_equal` 逐字节比对跳过未变文件；硬链接只做首装快路径，跨卷自动降级复制——**教训：remove+hard_link 的"先删后失败"会丢文件，跨卷场景硬链接只在目标不存在时尝试**）；② 上传流水线化（每 20 文件 drain → 每 200，本地 host ack 即发即回，一问一答是大头）；③ `upload_project_incremental`（本地 host 同机直读目标端，内容一致跳过传输——无变更再启动 = 0 文件上传）。host 侧 upload 起始不再 remove_dir_all 清目录（write_upload 本就覆盖写）。
- **联机 + 在线增量分发（0.6.0）**：本地服务器身兼游戏 host 与分发源。host 起时附 HTTP 分发服务（distrib.rs，端口=控制口+80=5083）：`/manifest`（staging 清单+xxh64）+ `/file` + `/base_assets`。对端 mini-client（bin/mini_client.rs，eframe 轻量启动器）输 IP/端口/userid → 引擎走官方 OSS 免鉴权（缺则 payload）→ 基座资产由房主 /base_assets 下发（**对端零 GitHub token**）→ 拉 manifest 增量比对本地缓存（size+xxh64）→ 只下变化文件 → 合成凭证进局。**踩坑**：① reqwest LAN 必须 `.no_proxy()`（系统代理拐走局域网请求）；② bsdtar 不能写 7z（只读），/base_assets 打 zip；③ Windows accept 套接字继承 nonblocking 的坑在 distrib 同样要 set_nonblocking(false)。本机模拟对端（127.0.0.1 三账号互见）+ 全量下载（1277 文件）已实证。**踩坑④（2026-09-03，对端进不去局）**：mini-client 载荷同步 `project_libs` 传空（误以为"依赖库随游戏文件分发"）→ 对端 `_m/maps/` 库（global_default/lib_control/...）全缺 → 客户端 lua 入口 `p_xxx/main.lua:3` 的 `require '@global_default/lua_declare'` 报错，**卡在「正在加载游戏逻辑」**，Req_PlayerList 永不发出（host 侧表现：登录+定向出站正常，此后该会话静默）。本机验收没暴露是因为本机 runtime 早装过这些库——**联机功能验收必须用干净目录模拟对端**（本次用 test/temp/fresh_client 全量重下复现）。修复：manifest 加 `libs`（staging libs.json 键），mini-client 先拉 manifest 再按 libs 判就绪（api_pak_version.json[api] 含全部键才跳过 payload sync），旧装坏的 runtime 据此自愈只补库不重下引擎。**排障手法**：客户端 game 日志 `Send stat[client_lua_error]` 计数 >0 = lua 有未捕获错误；lua 日志（logs/lua/lua-game-*.log）进程退出才冲刷——kill 客户端进程后再读才能拿到真实报错文本。**踩坑⑤（2026-09-03，对端引擎下载失败）**：`verify::proxy()` 默认回退 `http://127.0.0.1:7897` 是开发机便利残留——对端机器没这个本地代理，payload 全部官方请求走死代理报 "error sending request"。修复：仅显式设置 `MINI_RUNTIME_PROXY` 才走代理（星火官方接口是国内 CDN，直连即可；update-info 免签名，无权限要求）。**教训：分发到别人机器的功能，任何"本机默认"都是地雷**。

## 10. ★ KCP 会话协议初步分析（2026-09-02，中继 capture 实证）

数据源：中继全流量 capture（`host_capture-*.jsonl`，双会话：外部客户端 conv=0x14 + PIE conv=0x15）。
分析工具：`examples/kcp_capture_parse.rs`（stats/flow；本机缺 libclang 时 rustc 直编，文件头有命令）。

**① 握手（明文 ASCII 魔法）**：

```
c→h  CE1SYN    (13B: magic + 00 00 01 63 b8 03 05，重发至应答)
h→c  CE1SYACK  (16B: magic + conv(4 LE) + 时间戳?)   ← 服务器下发 conv
c→h  CE1ACK    (6B)
h→c  CE1SYNACK (16B: magic + conv + 00 00 00，重发至首个 PUSH)
```

**② KCP 层**：标准 KCP 头（conv/cmd/frg/wnd/ts/sn/una/len，全 LE），cmd 0x51=PUSH / 0x52=ACK；每客户端独立 conv（0x14/0x15 递增）。

**③ 流式分帧**：客户端带 `-kcp_stream` 时，PUSH payload = **3 字节 LE 长度前缀（= KCP len）+ 消息体**。

**④ c2h（客户端→host）= 明文 protobuf**：外壳 `f1{ f1 varint msg_type, f2 bytes body }`。已识别 msg_type：

| msg_type | 语义 | body |
| --- | --- | --- |
| 1 | 登录 | {f1 userid, f2 "userid" 字符串, f6 "", f7=1, f8=1} |
| 3 | 进度/加载 | {f1 = 递增值 30/45/95/100} |
| 5 | 状态 | {f1 = 0} |
| 0x2000 | 周期心跳（~0.5s） | {f1 i32, f2 i32}（常为 0,0） |
| 0x6011 | UI 视图同步 | {f1..f5, f6="ui-61-nil" / "main[map_view]>P0>P0"（UI 路径）, f8 hash} |
| 0x7006 | **玩法协议** | {f1 varint, f2 bytes = MessagePack `{type:"Req_PlayerList", args:[]}`} |
| 0xF100 | 时钟同步 | {f1 {f1=unix_ms, f2=conv}} + f2 double |

**⑤ h2c（host→客户端）= 无加密，是自研 ZCompress 压缩（2026-09-02 深夜实锤翻案）**：初判"加密"系误判。证据链：① 熵 7.36 bit/byte（真加密应≈8.0）；② 非同长度消息首 5 字节高度集中（`dd ea d8 f0 46`，68% 消息共享）= 每消息内嵌 Huffman 树头；③ 成对 XOR 出现 20+ 字节长零串 = 消息间共享明文前缀（真流密码不会）；④ deflate/zlib/lz4/zstd 全部解不开；⑤ 引擎字符串实锤 `ZCompress.cpp` / `ZCompressAdapter.cpp` / `when huffman encoding, some symbol has %d bits` / `rebuild huffman tree finished` / `[Compress(%x)]`（D:\BuildPC\NE_pd\Client\src\Game\Network\ZCompress\）。**结论：h2c 传输层 = [3B LE 帧长][ZCompress(消息)]，无密钥、无密码学——只剩格式逆向（纯函数，可对照 dll 反汇编或已知明文推断）**。
- 语义层旁路已实证：VM 内 hook `base.event.on_ui_message(_new)` 可直接拿到解码后的 cmsg_pack 消息（type_id 数字映射表经 type_name 首次下发建立）；cmsg_pack = msgpack 变体（0xc4 bin8 字符串），`cmsg_pack.pack({type='Req_PlayerList',args={}})` 与抓包字节逐字节一致——**c2h/h2c 的玩法消息体序列化完全打通**。
- 待验：是否存在"不压缩"标志位/开关（若有，自建 host 可直接发明文，`[kcp] %x fec switch %d` 显示 FEC 有运行时开关，压缩或有同款）。

**对架构选项的影响**：c2h 明文 + msgpack 玩法协议意味着「伪造客户端」廉价（oracle 探测服务端 API 面可直接手写 KCP 客户端发包）；「伪造 host」则需先复刻 h2c 的 ZCompress 压缩格式（无密钥纯格式，可行性见 ⑤）。

## 11. ★ 云 host oracle：server VM API 面实证（2026-09-02，M5）

方法：游戏服务端挂临时探针（`src/server/api_probe.lua`，log.info 输出 → 0xF00C 回读，用后已还原），在云端真实 host 里 dump server VM 全局。原始 dump 留档 `test/temp/probe-lines.txt`。

**实证结论（阿叶三条线索全部坐实）**：

- `_G`：`__IN_HOST__=1`、`__MAIN_MAP__/__GAME_ID__=p_55a3`、`bgd_api`、`score`、`cmsg_pack`、`autotest_log`、`os`（5 函数阉割版）、`io`（7 函数阉割版）。
- **server_common 的 require 根 = `@common`**（package.loaded 368 模块实证：`@common/base/**` 全家桶）——与客户端 script 包同根同构（线索 2）；**server_lua_plus 的加载形态 = `@lua_plus/base/base_lua_plus/**`**（42 个触编 API 模块，与本地明文包 `D:\sce_online\...\server_lua_plus\14\` 一一对应，线索 1）。
- `base`（977 键）= 服务端触编 API 全集（unit_*/skill_*/buff_*/score_*/quest_*/mover_* 等，即 server_lua_plus 挂载点）；`base.game`（130 方法）= 服务端游戏对象（create_scene_copy/close_scene/get_session_id/get_server_tag/keep_alive/end_game/set_winner/select_hero/**message**/**ui**/init_units/load_scene 等）；`base.auxiliary`（23）；`base.ui`={bind,proto}；`score`（29，云变量/排行榜）。
- 上传的地图包挂 `@p_55a3/` = **项目 `script/` 目录整树**（服务端入口 `script/main.lua`，泛化结构见 map-distribution-chain.md §3.1）：test_res002 实例含 `bgd_game_server/**`（bgd 构建产物的游戏服务端）+ `bgd_libs_server/**` + `obj/**`（数编表）+ `scene/default/area_save`。**来源 = 调试控制面上传链（0xF004/0xF008，本地 staging → host），不是任何下载包**——官方可下载的地图发布 pak 只含客户端消费内容（条目级实证见 map-distribution-chain.md §4），服务端构建是 `variation=server` 通道的 server.zip（§5，bucket ACL 403 不可得）；自研逻辑项目的服务端逻辑由本地项目源码经 staging 白名单（含 src/script）供给，无需也不可从官方 pak 提取。

**GameHost 复刻量评估**（host.exe + GameHost.lua 要补的仅剩）：

| 职责 | 依据 | 复刻难度 |
| --- | --- | --- |
| KCP 会话服务（CE1 握手/conv/重传/流分帧） | §10，标准 KCP + 3B 前缀 | 低（协议已明） |
| h2c 传输层 ZCompress 压缩格式 | §10 ⑤（无密钥，自研 Huffman） | ~~中（纯格式逆向，有 dll 反汇编+已知明文两条路）~~ **✅ 已复刻**（scegame-reverse.md §13.8，`src/core/zcompress.rs`，oracle/基准 capture 全量互验） |
| 控制面服务端（上传接收/起局/日志回传 0xF00C） | scegame-reverse.md §8，客户端侧已实现 | 中（镜像实现） |
| 地图加载编排（load_scene/init 链/obj 数编注入） | base.game.load_scene + package.loaded 结构 | 中（需逆向 GameHost.lua 行为，可经中继 0xF00C 全日志观察） |
| 消息路由（0x7006 → base.game.message/ui → 服务端 lua 回调） | c2h 明文 + base.game.message/ui 函数点 | 中 |
| 时钟/心跳（0xF100/0x2000） | §10 消息表（控制面另有 0xF01F 编辑器心跳，见 editor-debug-channels.md §2） | 低 |
| 服务端引擎本体（世界模拟/单位/技能/视野，base 977 键背后的 native） | 客户端引擎仅客户端构建（§3） | **B/B+ 路线不需要**——自研逻辑项目的服务端行为全在游戏自写 lua，native 调用面仅 6 项（薄 shim 即可）；仅 C 路线（通用 host，跑任意他图）才必须，那是 C 的真天花板 |

结论：**对 test_res002 类自研逻辑项目（B+ 路线），自建 host 全链复刻可行且工作量可控**——瓶颈只剩 h2c ZCompress 位流格式复刻（无密钥纯格式），「服务端引擎本体」一项根本不在关键路径上（游戏逻辑自写，GameHost.lua 编排 + 薄 shim 即够）。只有 C 路线（通用完整脱机、跑任意地图）才卡在引擎本体，与 §4 五条证据一致。现实路线：A（中继，已交付）→ ZCompress 复刻后 B/B+（壳 host / 真本地 host）。

## 12. 遗留研究项

- ~~KCP 会话协议抓包~~（§10 已完成一轮）；~~h2c "加密"~~（§10 ⑤ 翻案 = ZCompress 压缩）：~~下一步 = ZCompress 位流格式逆向~~（**2026-09-02 已完成**：Rust 复刻 + Frida oracle 1016/1016 + 基准 capture 3781/3781 全量互验，格式权威见 scegame-reverse.md §13.8；"不压缩开关"真伪 = 存在消息级原样标志（首字节 < 0x80 → data[1..]），无连接级开关）。顺带实锤：host 会把多个 KCP 段合并进一个 UDP 数据报，解析须逐段迭代（旧 capture 分析工具已修）。
- `scegame-reverse.md` ack 时序记录与本地实测的关系已调和（§8.5 ⚠️ 注解：我方上传方流水发送 vs 编辑器上传方本地逐文件等 0xF010 ack）；遗留 = 云端高延迟下官方 host 是否真容忍流水发送的实证。
- 手机调试场景 host_token 是否校验（editor-debug-channels.md §4.3 登记）。
