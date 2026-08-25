# 游戏态云变量通道勘察：B 模式实证 + 权限闸门实验

> 研究日期：2026-08-24 | 状态：B 模式通道定性完成；money/rank/message 权限=服务端地图/局授权，客户端侧不可解
> 前置：cloudvar-06（权限矩阵）/ cloudvar-07（message_* 签名）
> 数据件：test/temp/game_state_sniff.jsonl（误挂 PIE 残留进程，lobby 态 TCP 通道样本）、game_state_sniff2.jsonl（B 模式正确样本）

## 0. 一句话结论

**B 模式（mini-runtime debug 局）客户端完全不连 Entrance**（游戏流量 = UDP 直连 debug host），sce.s 全体请求进黑洞；**money/rank/message/qitem 的 Nopermission 与凭证/登录 flags 无关**（editor 凭证、tester 凭证、f3/f5/f4 位变体全同拒），是服务端按地图/局授权，客户端侧实验不可解——剩余唯一验证路径 = 真 tester 局（发布地图 + tester 客户端进游戏）。

## 1. B 模式游戏态实验（mini-runtime debug start）

流程：`sce_app_mini-runtime debug start --project test_res002 --cred woaye主号 --hold 600`（凭证 woaye主号 token_type=11）→ assign_host（106.14.95.227:20350）→ 上传 1248 文件 → 远端局 session_id 起 → spawn 本地客户端（`<runtime>/version-13/SCE`，wineditor v147 引擎，与编辑器 version-13 **MD5 一致**）。

探针（CloudProbe S1-S14 + MessageProbe Q/S 系列，场景-加载完成 +20s 触发）结果：
- **全部 sce.s 调用无任何回调**（score_init/commit/money_init/rank/list_query/name_search 全黑洞）；仅 message_query 的 Q2 触发本地 5s timeout。
- 客户端参数校验照常工作（S10/S13 签名错误本地即报，见 §3）。
- native 日志 `Send stat[entrance_net_stat] failed: send message error` 每 30s 一条 = Entrance 发送层从未就绪。

**抓包定性（entrance_sniff attach 正确进程 65264）**：进程内**无任何 Entrance TCP 连接、无 ent_send 帧**；游戏流量全是 **UDP sendto/recvfrom ↔ 106.14.95.227:20400**（debug host，与 assign_host 同 IP 不同端口）。→ B 模式客户端走 UDP 直连 debug host，ScoreArchive 无承载通道。

**推论**：B 模式不适合做云变量游戏态实验。游戏态帧补抓必须真 tester 局。

### 对照样本（lobby 态 PIE 客户端 TCP 通道，误挂收获）

sniff1 误挂 PIE 残留进程（PID 16004，D:\sce_online\version-13\SCE），意外拿到 lobby 态通道样本：
- Entrance 长连接在 **123.56.153.41:19100**（TCP）。
- 帧型：26B 心跳（u32 总长 0x1a + seq u16 递增）；0x40=数据帧（`40 00 01 1x` + u32 载荷长 + u32 头长 0x1a + seq）；0x41=对端回包（镜像 seq）；0x49/0x02/0x01 其他控制帧。
- **0x40 数据帧载荷无明文标记**（probe_key1/ClientReadWriteMap/iscore 全部搜不到）；同长帧公共前缀仅 14B 但前 200B 有 ~156B 相同块（固定会话头 + 流加密特征）——**lobby 态客户端↔服务端业务载荷加密**。
- ent_send hook 只能看到 msgid 0x6001（stat 遥测，明文 protobuf）；**ScoreArchive 0xA000 帧在 lobby 态 PIE 客户端不走该 TCP 明文**——与 cloudvar-04/05 的结论（libhv 静态 OpenSSL 加密 WSS 通道）一致：0xA000 走 WSS(TLS)，ws2_32 层只能看到密文。

**方法学教训（重要）**：attach 抓包前必须核对进程 Path（`Get-Process sce | Select Id,Path`）——编辑器 PIE 残留进程（停止调试后仍存活、仍发 stat 心跳）与 mini-runtime 游戏进程同名 `sce`/`SCE`，极易挂错；挂错后数据貌似合理（有 stat 有会话帧），污染结论。

## 2. 权限闸门实验（entrance_client 直连，lobby 上下文）

| 实验 | 结果 |
| --- | --- |
| editor 凭证 money（MoneyInit f3=10） | Nopermission(13)（基线复现） |
| **tester 凭证**（`D:\sce_pc_tester\tester_1089\Win\User\user_info-e.production.spark.xd.com.json`，token_type=11 同 editor）money | Nopermission(13) |
| 登录帧 f3=1 / f5=1（ENT_LOGIN_EXTRA 1801/2801） | 同拒 |
| 登录帧 f4 flags 变体 0x1000041/42/48（ENT_F4） | 同拒 |

关键事实：
- **tester lobby 与 editor lobby 连同一 Entrance**：`wss://entrance-new-pd.tapsce.cn:443`（tester lobby 日志实证）——无独立 tester 端点。
- tester lobby 登录 `isGameFlag : true`（editor lobby `game_flag is: false`）——但凭证/flags 实验表明这不是 ScoreArchive 授权的开关（至少不在 f3/f5/f4 低 4 位）。
- 剩余可能：① 0x0011 进局通知绑定真局会话后服务端才放行（局授权）；② 地图需在创作者中心配置对应功能（地图授权）。两者都只能真局/真实配置验证。
- tester 凭证 userid 同为 38672742（登录响应 `user id : 38672742@140729326959375`，@ 后缀 = 设备/实例标识？）。

## 3. 客户端 API 签名终表（wrapper 反汇编 + 探针实证，全闭合）

错误串定位法修正：capstone `lea rdx,[rip+disp]` 目标 = insn_va+7+disp（上次口算错位一节得假结论；已用字节级核对修正）。

| API | 签名（客户端） | 依据 |
| --- | --- | --- |
| sce.s.message_send | **(player\|uid\|nil src, key:string, target_user_id:int, value:any, events?)** | arg3 报错串='target_user_id参数不是合法整数'（0x1826cba70）；arg1='player参数不是合法的类型（player/integer/nil)'；arg2='key参数不是字符串'；arg4 序列化器；arg5 events |
| sce.s.message_query | (player\|uid\|nil, key:string, events, arg4?) | events 必填 #3（"#3 table expected" 来源） |
| sce.s.message_modify_read / message_delete | (player, message_id:int, [read:bool,] events?)（推断） | 错误串池 message_id/read 布尔 |
| committer.item_add | **(player, key:string, item_name:string, count:number, extra?:any)** | arg3 报错='item_name参数不是字符串'、arg4='count参数不是数字'、arg5 序列化器 |
| sce.s.get_rank_list | ([map_name,] key, start:int, number:int, other_key?, events?) | 双 string 前缀判 map_name；isnumber×2 必填 |
| sce.s.get_rank_total | ([map_name,] key, events, ...?) | 探针：#3 须 table（events），'iscore' 放 #3 报错 |
| sce.s.get_user_rank | (player, key, other_key?, events?) | 头部 player helper + 双 isstring |
| committer 全套 wrapper VA | score_set 0x181324170 / score_seti 0x181324560 / score_addi 0x1813235a0 / score_sets 0x181324570 / money_add 0x181321e20 / money_cost 0x181321e30 / list_add 0x18131fcd0 / list_modify 0x181320470 / list_delete 0x1813200f0 / item_add 0x18131f110 / item_use 0x18131f9f0 / name_new 0x1813221e0 / client_score_set 0x18131d1d0 | 注册块 0x18131de06-0x18131df32（字符串/wrapper 交替 lea） |

commit/sce.s 注册方式 = 运行时 lua_pushcclosure 块（非静态 luaL_Reg），定位法见 cloudvar-07 §1。

## 4. 工具迭代（本轮固化）

- **entrance_client 扩展**：新增 `money`/`rank`/`urank`/`qitem`/`names` 子命令（f3=10/36/38/70/20，结构按 cloudvar-06 §2）；`ENT_F4`（登录 flags）、`ENT_LOGIN_EXTRA`（登录帧追加 hex 字段）两个实验变量。
- **entrance_sniff 修正**：waitModule 列表 sceengine.dll 优先（编辑器/wineditor 构建 Entrance 在 dll，exe 基址误配 dll RVA 是坑）；wineditor v147 的 hook 点 = **send RVA 0x1aa3770（send(conn,msgid,frame,len) 已验证）/ recv log RVA 0x1aa1b19（msgid=[rsp+0x50]）**，经 ENT_SEND_RVA/ENT_RECV_RVA 传入。
- entrance_sniff 构建：frida-sys 需代理下载 devkit（无代理静默卡死）+ LIBCLANG_PATH 指向 pip clang/native。

## 5. 下轮入口（按价值排序）

1. ~~tester 凭证取样~~ 已完成（§2，无差异）。剩余真局验证：发布 test_res002 到测试环境 → tester_1089 进游戏 → entrance_sniff 抓 scegame（RVA 需按当前 scegame 版本重定位，themis 干扰未实测）——可一举补全 message_*/money/item/rank 游戏态帧 + 判明局授权机制。**需用户确认发布**。
2. 若真局证实 0x0011 进局绑定后 Entrance 直连放行，则 entrance_client 增加"进局绑定"步骤即成全权限直连（最终形态）。
3. B 模式 UDP 协议（KCP 嫌疑）逆向 = host 侧云变量（subscribe/publish/world_data）的唯一可见路径（debug host 流量本机可见！），工程量大，暂挂。
