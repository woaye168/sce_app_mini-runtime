# message_* 客户端 API 签名破解（反汇编 + PIE 实证）

> 研究日期：2026-08-24 | 状态：签名全解（发送成功），权限需游戏态（lobby Nopermission）
> 前置：cloudvar-06 §3（message_* 签名未试通）；本文闭合该遗留项
> 探针：test_res002/.bgd/src/client/MessageProbe.lua（保留可复用）

## 1. wrapper 定位（SCEEngine.dll，version-13）

注册不在静态 luaL_Reg 表（find_luareg 绝对指针查不到），而是运行时 `lua_pushcclosure` 注册块（VA 0x181319d1a 起，函数名字符串与 wrapper 函数指针交替 lea）：

| 函数 | wrapper VA | 注册点 |
| --- | --- | --- |
| get_commit | 0x18131dde0 | 0x181319d1a |
| score_init | 0x181323990 | |
| money_init | 0x181321e40 | |
| name_search | 0x1813225c0 | |
| get_rank_list | 0x18131e150 | |
| get_user_rank | 0x18131ed10 | |
| get_rank_total | 0x18131e7f0 | |
| message_query | 0x1813215b0 | 0x181319dcb |
| message_send | 0x181321990 | 0x181319de1 |
| message_modify_read | 0x1813212a0 | 0x181319df7 |
| message_delete | 0x181320f60 | 0x181319e0d |
| list_query | 0x1813208d0 | |

**方法学沉淀**：find_luareg.py 只查字符串首出现点的 8 字节绝对指针；对运行时 pushcclosure 注册块无效。修正版 `test/temp/find_str_refs.py`（全出现点 + RIP 相对 lea 扫描）。IAT 跳转桩（FF25）反查导入名用 pefile（见下）。

## 2. 反汇编签名（arg 检查逐个识别，lua C API 经 pefile IAT 反查确认）

引擎 lua = **lua54.dll**（Lua 5.4）。

### message_send(uid, key, arg3:int, value, events?)

| arg | 检查 | 说明 |
| --- | --- | --- |
| 1 | helper 0x18131e080（Player\|integer，-1=非法） | target_user_id；**bgd 框架 Player 包装表不被接受**（报"player参数不是合法的类型（player/integer/nil)"），需传整数 uid 或 nil |
| 2 | lua_isstring + lua_tolstring | key（string，必填） |
| 3 | lua_isnumber + lua_tointegerx | **整数，必填**，语义未最终确认（候选：read 初值/src_user_id；arg3=0 已发送成功到服务端并被鉴权拒绝，说明格式被接受） |
| 4 | helper 0x181325170 序列化（栈顶 lua_tolstring 取回） | value（任意 lua 值，MessagePack 家族序列化） |
| 5 | gettop>=5 时 events parser 0x18131df80 | events {ok,error,timeout}（可选） |

### message_query(uid, key, events, arg4?)

- arg1 同 send 的 uid helper；arg2 = key string；**arg3 = events（必填**——缺失即上次观测的 "#3 table expected"）；gettop>=4 时解析可选 arg4（候选：read 过滤，未验证）。

### message_modify_read / message_delete

- wrapper 0x1813212a0 / 0x181320f60（未逐指令核）；错误串池有 `message_id参数不是合法整数`、`read参数不是合法布尔值`、`messageId参数不是合法整数`（camelCase 变体在 0x181322cf4/0x181323942 被引用，疑似 subscribe/push 回调路径）。

### 错误串池（.rdata 0x26ca300-0x26ca980，中文 UTF-8）

`key不是字符串` `value不是合法的整数` `target_map不是字符串` `value暂时只支持字符串` `listId参数不是合法整数[字符串]` `value参数不是合法table` `item_name参数不是字符串` `count参数不是数字/整数` `expire_type参数不是数字` `expire_time参数不是字符串` `expire_type参数错误，不是0 1 2中的一个` `item_id参数不是整数` `key参数不是string` `name_substr参数不是字符串` `start不是整数` `number不是整数` `other_key参数不是string` `player参数不是合法的类型（player/integer)` `is readed参数不是bool` `target_user_id参数不是合法整数`（无 .text xref，疑似 server 侧残串或经拼接引用）`message_id参数不是合法整数` `read参数不是合法布尔值` `messageId参数不是合法整数`。

**坑**：sceengine-strings.txt 字符串 dump 不含中文串（非 ASCII 被过滤），中文错误串需直接在 PE 里按 UTF-8 字节搜。

## 3. PIE 实证（test_res002，editor-pd lobby 态，2026-08-24 12:47 运行）

| 调用 | 结果 |
| --- | --- |
| message_query(nil, key, ev) | 发送成功 → error code=13 Nopermission |
| message_query(uid, key, ev) | 同上 |
| message_send(uid, key, 0, {text,n}, ev) | 同上 |

- **签名试通**：不再触发客户端参数校验错误，请求到达服务端被鉴权拒绝（与 money/item/rank 同——lobby 态无权限）。
- PIE 本机 uid = **38672742**（日志 `common/base/player.lua:330 local user：38672742`；Player repr `{player|1-user-1|""|38672742}`）。bgd `base.local_player()` 是 lua 包装表，native API 不认；uid 从此日志/repr 提取。
- 帧补抓（QueryMessage/SetMessage 子类型号 + arg3 语义确认）必须游戏态（tester 进图 / mini-runtime B 模式），同 money/rank 遗留项合并推进。

## 4. 工具链备注（本次固化/修正）

- `test/temp/find_str_refs.py`：字符串全出现点 + 绝对指针 + RIP lea 扫描（find_luareg 的修正版）。
- pefile 反查 IAT：FF25 跳转桩 disp 在偏移 2（`FF 25 disp32`，target=T+6+disp），import 地址=IAT slot VA。
- cargo examples 构建前置：`LIBCLANG_PATH=<pip clang>/native`；frida-sys 下载 devkit 需代理（HTTP_PROXY/HTTPS_PROXY=127.0.0.1:7897），无代理会静默卡死在 "Compiling frida-sys"。
- SCEEngine.dll .text RVA→文件偏移 = -0xC00；disasm_at 吃文件偏移。
