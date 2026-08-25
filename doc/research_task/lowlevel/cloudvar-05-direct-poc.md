# 云变量直读直写 PoC 成功实录（2026-08-23）

> 状态：✅ 跑通——纯 Python（websockets 库）直连 Entrance，无引擎/无 sce.s，读+写+复读验证全过
> 前置：cloudvar-04-protocol.md（帧格式全解）
> 代码：test/temp/cloudvar_poc.py（一次性 PoC；固化版本见 examples/ 或后续工具化）

## 0. 结论

**星火线上云变量可以完全脱离引擎直读直写**：`wss://entrance-new-pd.tapsce.cn`（WS 根路径）→ 发 0x0001 登录帧（body 含凭证 `token` 字段 385 字符）→ 收 0x0002 → 收发 0xA000 ScoreArchive 帧即可。全部字段手写 protobuf wire，无加密无签名无反重放。这绕开了 sce.s/score 的类型分桶与签名限制——可以构造任意 op 组合、批量 key、自定义 source 名。

## 1. PoC 实测记录（editor-pd 环境，userid 38672742）

```
connect wss://entrance-new-pd.tapsce.cn ✓
>> 0x0001 login{f1 'default', f4 0x1000040, f6 11, f7 token(385B), f19 1, f20 ''}
<< 0x0010 {f1 0}（server stop notify，建连即收，忽略）
<< 0x0002 登录响应 559B：f1 varint 0=成功；body{f1 user_id, f2 nick('一只小蘑菇'), f6 ..., ...}
>> 0xA000 ScoreInit{user, keys=[cloudprobe_key1, cloudprobe_skey1]}
>> 0xA000 Commit{op_seti(cloudprobe_poc_i,777), op_sets(cloudprobe_poc_s,'direct_write'), desc='poc_direct'}
>> 0xA000 ScoreInit{keys=[cloudprobe_poc_i, cloudprobe_poc_s]}
<< 0x7001（登录后配置下发，skip）
<< A000 f3=5 ScoreInitRes：cloudprobe_key1=12345, cloudprobe_skey1='hello_probe'（游戏侧写入的值，读到 ✓）
<< A000 f3=100 Result{f1=0}（commit 成功 ✓）
<< A000 f3=5 ScoreInitRes：cloudprobe_poc_i=777, cloudprobe_poc_s='direct_write'（直写复读 ✓）
```

## 2. 响应结构精解（ScoreInitRes，f3=5）

body = `f1 bytes 条目`（repeated），条目内：

```
f1 varint  = user_id
f2 bytes   = 积分值组：
  f2 bytes = 数字积分条目 {f1 str key, f2 varint ivalue}   （可 repeated）
  f3 bytes = 字符串积分条目 {f1 str key, f2 str svalue}     （可 repeated）
  （普通 table 积分为另一种序列化形态，待取样——score_set(table) 未测）
f6 varint  = 该条目时间戳/内部 id
```

错误/确认（f3=100）：body = {f1 varint error_code, f2 str reason}。已知错误码：0=成功、13=Nopermission、1146=TableNotFound（MySQL 原码直通）。

请求-响应关联：响应 f6 = 服务端分配的单调序号；实测不需要客户端带 key（f5 空/f6 0 即可），串行请求-等待即可对齐。

## 3. 凭证与登录细节

- token = `User/user_info-<env>.json` 的 **`token` 字段**（385 字符，BBAXRA... 开头；不是 login_token/access_token）。
- 登录帧 f4 = 0x1000040（固定 flags，直接重放）；f6 = 11 = token_type（编辑器凭证）；f19 = 1。
- 遥测 0x6001 实测**不需要**先发；登录直接成功。
- 连接保活：官方 ~30s 一批 0x6001；PoC 短连接没测保活，长时间连接需观察服务端踢人策略。

## 4. 能力边界与待验证问题

1. **读写次数计费维度未确认**：服务端按"局"统计 300 读/300 写每分钟（官方文档）。直连连接算不算"局"、是否绕开计数——需要在创作者中心云变量后台看调用统计，或压测观察限流错误码。
2. **source 字段**（Msg.f1）：PoC 用 'poc' 也能读写成功（服务端不校验？）。lobby 态官方值 'startup'；游戏态预计为地图名。**target_map 才是权限关键**：'ClientReadWriteMap' = 当前上下文地图的读写区——直连时它解析到哪张图（token 不绑定地图！）待深挖：可能落到某个默认/全局空间，或由 source 推导。⚠️ PoC 能读到游戏侧写的 cloudprobe_key1，说明 ClientReadWriteMap 在直连上下文解析到了**同一空间**（大概率按用户+默认图）。
3. **服务端 score.* 独有功能**（world_data/subscribe_channel/publish_message/list 写）的子消息类型号未取——可用同样方法在 lobby 注入探针补齐（subscribe/publish 是频道 pub/sub，价值高）。
4. readonly_map 的 key 空间 = 地图积分（大厅展示用），直连同样可读（已验证 readonly 读 p_55a3 返回 ok 空）。
5. 正式环境（e.production.spark.xd.com）entrance 域名相同（entrance-new-pd.tapsce.cn）；tester 凭证 token_type 可能不同（待取样）。
6. **风控未知**：直连绕过了引擎，但 Entrance 是官方客户端协议通道，帐号行为与官方客户端一致；仍建议低频使用直到摸清计数/风控。

## 5. 与既有认知的修正

- cloudvar-01 §2 推测"proto descriptor 可提取"——已证伪（二进制无 .proto 字符串，全部手写 wire）；正确路线 = 本目录的动态帧捕获。
- credential-userid.md 称"entrance 登录走 WSS，ws2_32 只能抓密文"——对 socket 层成立，但 **native 协议层 hook（发送函数入口/接收日志点）可直接拿明文帧**，比 SSL hook 更干净（本方法学可推广到任何 libhv 静态 TLS 的通道）。
- "云变量必须经游戏局/官方 API"——证伪：Entrance 连接 + token 即可，局外可用（lobby 态/任意自建进程均可）。
