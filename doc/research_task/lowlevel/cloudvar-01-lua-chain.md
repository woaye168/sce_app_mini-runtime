# 云变量（sce.s）Lua→native 实现链溯源

> 研究日期：2026-08-23 | 状态：Lua 层全通，native 发送点已定位，待提取 proto + 抓消息 ID
> 溯源对象：`.editor_src_mirror/`（script-199 / client_base / xdeditor-160 / lib_lobby-169 / tester_startup-364 / sceengine-strings.txt / scegame-tester-strings.txt）

## 0. 一句话结论

`sce.s` **不是 Lua 封装**，是引擎 C++（`LuaScore.cpp`）直接注册的 native 全局表；所有读写经**平台 Entrance 长连接**（`EntranceHeader` + 数字消息 ID + protobuf `CEProto::ScoreArchive::Msg`）发往内部称 "ScoreServer" 的服务。既不走游戏局 KCP，也不走 HTTP。Lua 层**没有**暴露"向 Entrance 发自定义消息"的通用函数，复用该连接须 native hook 或自建连接仿协议。

## 1. Lua 侧只有调用点（全库 grep 无 `sce.s = ` 赋值）

| 调用点 | 说明 |
| --- | --- |
| `xdeditor-160/test/scorearchive.lua:11-175` | ★ 官方自测脚本，最完整 API 样例：`get_commit/commit/score_addi/money_add/money_cost/score_init/name_search/list_query/list_add/list_modify/list_delete`，回调 `{ok, error(code,reason), timeout}` |
| `script-199/common/base/open_url_wrap.lua:73` | `sce.s.score_init(sce.s.readonly_map, 51, {...}, key)` |
| `client_base/common/base/account.lua:332` | 读控制台白名单云变量（key=user_id） |
| `lib_lobby-169/ui/src/proxy/proxy_score_info.lua:147..537` | 大厅批量读地图积分（readonly_map，多 key unpack 批量查；test 环境 key 加 `@test` 后缀） |
| `tester_startup-364/extracted/application/entrance/main.lua:2445-2453` | readwrite_map 写路径样例 |

官方样例（test/scorearchive.lua）：

```lua
sce.s.score_init(sce.s.readwrite_map, nil, {
    ok = function(score, iscore, sscore) end,       -- 三类值：通用/整数/字符串
    error = function(code, reason) end,
    timeout = function() end
}, "key")
local c = sce.s.get_commit()
c.score_addi(nil, "key", 10)   -- 参数1=目标玩家(player/user_id/nil=自己)
c.commit('测试', { ok=, error=, timeout= })
```

## 2. 调用链（Lua → native → 网络）

```
sce.s.<fn>(...)                                ← 纯 native 注册（__LUA_SCORE__）
  ↓ LuaScore.cpp（sceengine-strings :444850 源路径硬编码）
  ↓   sce.s 函数名表 :444933-444948：get_commit/score_init/money_init/name_search/
  ↓     get_rank_list/get_user_rank/get_rank_total/message_query/message_send/
  ↓     message_modify_read/message_delete/list_query/query_item/client_score_init/
  ↓     readonly_map/readwrite_map
  ↓   commit 对象方法表 :444881-444894：commit/score_set/score_seti/score_addi/score_sets/
  ↓     money_add/money_cost/list_add/list_modify/list_delete/item_add/item_use/
  ↓     name_new/client_score_set（元表 __COMMIT_METATABLE_REF__ :444880）
  ↓ protobuf 子消息（编码失败日志暴露 proto 名 :444857-444925）：
  ↓   Commit/MoneyInit/NameSearch/QueryRankList/QueryRank/QueryRankTotal/QueryMessage/
  ↓   SetMessage/DeleteMessage/QueryList/QueryItem/ClientScoreInit
  ↓   字段名可见：target_map/listId/item_name/count/expire_type/expire_time/
  ↓     target_user_id/src_user_id/message_id
  ↓ 顶层封装 CEProto::ScoreArchive::Msg（:444970）
  ↓ ★ 发送点 :444852 "Send Scorearchive msg to Sntrance failed."（Sntrance=Entrance 笔误）
Entrance 长连接（Entrance.cpp :456347；帧头 EntranceHeader :439973/439983；
  发送日志 :456425 "Send message to entrance, message : 0x%X"；
  连接参数 :456183 "Connecting to entrance, ip:%s port:%d protocol:%s"，TCP/KCP 可变；
  域名 :436614-436615 e.production.spark.xd.com / e.intl.spark.xd.com）
回包：Entrance 分发 → lobby 事件 on_scorearchive_message_event（:436414；
  payload 键 __scorearchive_message_type/__body/__key :440384-440387，key 做请求-响应关联）
  → LuaScore 解析响应 proto（:444950-444967：Result/MoneyInitRes/NameSearchRes/
    QueryMessageRes/SetMessageRes/QueryItemRes/DeleteMessageRes/QueryListRes；
    :440251-440256：MultiScoreInitRes/ClientScoreInitRes）→ Lua 回调
超时：约 3000ms（xdeditor trigger/lua-parser/basic_typetree/config.lua:321 注释 time_score_init=3000）
```

**重要旁证**：scegame-tester-strings :536993-536995 有同一套 LuaScore.cpp / 发送日志，:542686-542931 同样有 Entrance 连接代码——**游戏进程（含局内 host）内置相同链路**，服务端 lua 的 sce.s 也走本进程 Entrance 连接，不经客户端转发。

## 3. native 侧关键符号/字符串（sceengine-strings.txt 行号）

| 行号 | 字符串 | 含义 |
| --- | --- | --- |
| 444850 | `...\LuaScore.cpp` | Lua 绑定源文件 |
| 444949 | `__LUA_SCORE__` | 注册名 |
| 444852 | `Send Scorearchive msg to Sntrance failed.` | ★ 发送点（xref 即达发送函数） |
| 444970 | `CEProto::ScoreArchive::Msg` | 顶层 proto |
| 444953/444969 | `UnexpectedMessageFromScoreServer[` / `UnknownMessageFromScoreServer[` | 收包分发；服务端称 ScoreServer |
| 444873-444874 | `"9999-12-31 23:59:59"` / `NOW() + interval second` | ★ 物品过期用 MySQL 语法拼接——存储后端极可能 MySQL |
| 436414-436418 | `on_scorearchive_message_event`/`ScoreArchiveMessage` | lobby 事件表项 |
| 440280/444846 | `ClientReadonlyMap`/`ClientReadWriteMap` | 地图作用域枚举 |
| 440224-440256 | `CSharpScore` + `CSharpScore.cpp` | 编辑器 C# 侧同协议实现 |
| 511880-511886 | `CSharpScoreDataManager_Create/Destroy/QueryCategory/QueryLabel`、`CSharpScore_Create/Destroy/InitScore` | ★ 导出符号（P/Invoke 入口，可 hook） |
| 440282-440289 | `ScoreDataNode/ScoreCategoryNode/ScoreLabelNode/is_dir/expand/packet_type/show_priority/create_time` | 编辑器云变量管理界面分类/标签树 |
| 456347/456425/439973 | Entrance.cpp / 发送日志 / EntranceHeader | 承载通道 |

## 4. 网络通道判断

**云变量 = Entrance 长连接（CEProto::ScoreArchive::Msg），目标 ScoreServer。**

- 唯一发送失败日志（444852）字面即"发往 Entrance"。
- 回包事件在 lobby/entrance 事件表（436378 on_entrance_connected 紧邻 436414）。
- 游戏局 KCP（452693/456462）与 ScoreArchive 字符串零交集；base.proto 是局内 s2c，无关。
- HTTP 只用于**管理面**：编辑器开通/查询云变量空间 `https://publisher-<env>.spark.xd.com:9000/api/map/set-package-score-name|get-package-score-name`（xdeditor project_manager.lua:343,397；域名推导 client_base utility.lua:385-423）。错误码 3="SQL failed"（:371）佐证服务端 SQL 库。

**游戏 lua 可用网络通道盘点**：① 局内 base.proto（KCP，限局内双端）；② sce.httplib（native LuaHttp.cpp :445109-445110，`request{url,method,json,header,input,output}`，任意 URL）；③ lobby.send_luastate_broadcast（大厅↔游戏 lua 广播，client_base account.lua:318-323）；④ sce.s 专用 Entrance 通道（无通用自定义发送函数暴露）。

**编辑器侧管理代码**：xdeditor-160 `window/set_score_name_window.lua`（老项目云变量开通 UI，调 publisher HTTP API）；PIE 调试面板有「云变量请求次数」统计（gameplay_in_editor_view.lua:779）；触发器分类 score/score_game/score_committer（type_infer_basic.lua:323-325），`trigger/rule/积分.lua` 整文件被注释（动作定义在 C# 侧）。

## 5. 下一步逆向线索（按优先级）

1. **proto_extract 提取 CEProto::ScoreArchive 全族 + EntranceHeader 的 FileDescriptorProto**（mini-runtime examples 现成工具，对 sceengine.dll / scegame 用）。
2. **拿 Entrance 消息 ID**：find_xref 定位字符串 444852 的发送函数，上一层即得数字消息 ID；或 Frida hook 456425 日志点动态抓。
3. **Frida 抓明文**：mini-runtime `frida_capture`（ws2_32）/ `entrance_login_capture`（SSL_read/write）对 Entrance 连接抓帧。
4. **C# P/Invoke 入口**（511880-511886 导出符号）——编辑器管理界面可能走另一条查询通道（CSharpScore.cpp 字符串区 440224-440256）。
5. **请求-响应关联**：`__scorearchive_message_key`（440387）+ `LuaScore[CHECK_SERIALIZE_RESULT FAILED]`（444851）暗示串行化校验队列，hook 点清晰。
6. **ScoreServer 是否独立暴露端口**：目前只见 Entrance 转发，无独立地址字符串；若只经 Entrance 转发，则"直读直写存储服务器"= 仿 Entrance 客户端协议直连 Entrance 发 ScoreArchive::Msg。
