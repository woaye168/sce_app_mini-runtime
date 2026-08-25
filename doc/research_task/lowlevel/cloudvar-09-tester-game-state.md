# tester 真局云变量权限矩阵（线上实证）+ 自动化进游戏

> 研究日期：2026-08-24 | 状态：真局（星火对战平台 tester，test 环境）权限矩阵实证完成
> 前置：cloudvar-08（B 模式证伪 + 权限闸门实验）；本文闭合「游戏态补抓」主线
> 探针：test_res002 CloudProbe S1-S14 + MessageProbe（已发布到测试环境，pak v97）

## 0. 一句话结论

**真局游戏态权限 ≈ lobby 态**：money_init/rank/message/query_item 仍 Nopermission(13)——这些 op 的授权是**地图级**（创作者中心开通）而非连接/局级；**游戏态新解锁：commit money_add（写货币）放行**（lobby 态同 op 是 Nopermission）；score 读写/list/client_score_set 两态均放行。

## 1. 自动化进游戏（★ 无需点击的官方通道，已实证）

```
tester_1089.exe -game=p_55a3 -tag=test [-ai_test=1]
```

- **参数必须 `-key=value` 形式**：`-game p_55a3`（空格形）会被解析为「flag 存在但值为空」→ TO_START_GAME_MAP 停在 app_android 壳死等（踩坑实录）。
- 启动链：startup 读 argv('game') → 下载更新该图 → quick_start 大厅壳（无 wx_lobby 时 = **app_android**；无 game 参数时 = **app_box** 盒子大厅）→ app_android 内 lib_lobby 模式页 → **`-ai_test=1` 触发免交互开局**（proxy_guide.check_immediately_start: argv.has('ai_test') → match_game:start_single → 秒匹配进图）。
- 大厅进真游戏两段式：大厅（app_android/app_box 也是「地图」）→ 匹配 → 真游戏地图。
- 登录：凭证持久化自动登录（token_type=11，`Login success, user id : 38672742@<实例id>`）。
- 相关机制（源码实证，tester_startup-364/app_box/lib_lobby-170）：
  - `start-game://<project_id>&tag=test` URL scheme（startup switch_game 广播处理；本机未注册协议）。
  - `lobby.register_event('reload_app')` / `app.reload_with_command_line(cmdline)`（native 触发的换参重启）。
  - 第二实例转发在本构建未生效（实测无转发）。
- **tester 大厅（lobby 态）也在收发 0xA000**：app_box 的 score_info 用 ScoreArchive 读地图评分/最近游玩（lobby 日志逐条 msgid 实证）——lobby 态读权限两环境一致。

## 2. 真局权限矩阵（2026-08-24 14:57，test 环境 p_55a3 真局，uid 38672742）

| API | lobby 态（editor/tester） | 真局（tester test 环境） |
| --- | --- | --- |
| score_init 读 | ✅ | ✅ |
| commit score_seti/sets/set | ✅ | ✅ |
| list_query / list_add | ✅ | ✅ |
| client_score_set | （未测） | ✅ |
| **money_add（commit）** | ❌ Nopermission | ✅ **ok** |
| money_init（读） | ❌ Nopermission | ❌ Nopermission |
| get_rank_list / get_user_rank | ❌ Nopermission | ❌ Nopermission |
| query_item | ❌ Nopermission | ❌ Nopermission |
| message_query / message_send | ❌ Nopermission | ❌ Nopermission |
| name_search | ❌ TableNotFound(1146) | ❌ TableNotFound(1146) |

- **money 读写不对称**：真局可 money_add 累加但 money_init 读不回——货币查询类需要地图在创作者中心开通（rank/message/query_item/money_init 同理推断）。
- 对 entrance_client 直连的含义：直连（lobby 上下文）可做的操作集合已封顶（score 读写 + list + name_new + item_add）；money_add 需游戏局上下文（真局的 Entrance 会话），lobby 直连不可达。

## 3. themis 反作弊实锤

- `frida attach scegame` → **DeviceAttachError**（tester 进程受 themis 保护，运行中 attach 被拒）。
- 未试：frida spawn（进程创建即注入，themis 初始化前挂钩）——scegame 直启需完整参数（已拿到：`-server=e.production.spark.xd.com -wx_lobby=app_android -launcher="../tester_1089.exe" -game=p_55a3 -tag=test -ai_test=1`，launcher 转发后形态）。
- **免抓包替代**：lua 探针的 ok/error 回调已给出全部权限结论；真局 0xA000 帧格式与 lobby 同构（cloudvar-04 已解），补帧价值降级。

## 4. 窗口操控坑（tester）

- tester 窗口会自发进入 iconic（-32000,-32000）：SW_RESTORE/SW_SHOWNORMAL/SetWindowPlacement 均无效，**必须 `SendMessage(WM_SYSCOMMAND, SC_RESTORE=0xF120)`**（唯一有效恢复手段，已固化进 test/temp/click_win.ps1 注释可用的经验）。
- mini-runtime capture CLI（WGC）对 iconic 窗口可正常离屏截图（真局画面实证）。
- mouse_event 模拟点击对 tester 大厅（app_box，Urho3D 自绘 UI）效果不稳定；**已被 §1 参数通道取代，弃用**。

## 5. 下轮入口

- entrance_client 直连补「游戏局上下文」：真局 Entrance 会话的建立帧（0x0011 进局通知）若要直连复刻，需 frida spawn 抓真局首连（themis 绕过是唯一卡点）。
- 探针修正（下轮真局用）：S10 get_rank_total(map, key, events)（events 在 #3）；S13 item_add(player, key, item_name, count, extra)（cloudvar-08 §3 已修签名，探针未改）。
