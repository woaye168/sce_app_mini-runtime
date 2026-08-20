# 脱机登录与调试：事实研究（0.1.0 前置）

> 日期：2026-08-21
> 证据：client_base-78 全量还原件（镜像在 `D:/sce_online/Res/maps/bgd_glzy/.editor_src_mirror/client_base-78/`，77 个 lua；统一逆向镜像目录，与 script-199/xdeditor-160/tester 系列并列）+ script-199/xdeditor-160 镜像（editor-patch 仓）+ sceengine.dll 字符串考古
> 重点纠正：「真实登录逻辑在 client_base 的 base/account.lua」——script-199 的同名文件只是 `return require '@base.xxx'` 转发桩，本仓已用 restore_game.py 一键还原 client_base-78 全量（7z→解密→UPAK 解包）

## 1. 登录体系全链路（client_base-78 精读结论）

### 1.1 分层

```
xdeditor ui/login.lua            # 编辑器 UI 层：扫码窗口 + TapTap device flow + refresh_token
client_base base/account.lua     # ★ 账号核心：凭证文件读写 / token 管理 / 登录方式选择 /
                                 #   登录结果处理 / HTTP 签名 / lobby 事件桥
client_base base/lobby.lua       # entrance 长连接事件桥：lobby_events.on_*（native 回调）→
                                 #   dispatch('登录'/'已连接'/'断开连接'...)；接口集 setmetatable
                                 #   转发 native lobby.*（register_event/请求类函数）
native lobby.*（sceengine.dll）   # entrance 连接/request_token_login/request_guest_login/...
```

### 1.2 凭证文件（account.lua:70-76, 239-249）

- 路径：`User/user_info[-<_G.IP>].json`（编辑器根下；默认 IP 时无后缀）。
- 读写：`load_user_info`（io.read）/`account.save`（io.write + io.serialize 异步）。
- 字段（本机实证）：access_token / guest_id / login / login_token / login_token_secret / login_type / token / token_type / version。
- 初始化顺序（account.lua:96-125 init()）：默认值 → 读存档覆盖 → argv guest_id 覆盖 → `lobby.set_guest_id`。

### 1.3 登录方式选择（account.lua:200-227 login()）

```
platform.is_win() or auto_guest or allow_guest or (token_cache and token_valid)
  → token_valid() ? lobby.request_token_login(token_type, token)   # 有 token 走 token
                   : lobby.request_guest_login()                    # 否则游客
否则 → sdk_login()（lobby.request_sdk_login，拉起 TapTap 客户端）
```

token_valid() = token 非空且 token_type ∈ [11,14]（11=编辑器TapTap / 13=手机 / 14=安卓容器 / 999=游客）。

### 1.4 登录结果闭环（account.lua:350-410 on_login_result）

`lobby_events.on_login_response(error_code, ...)`（lobby.lua:1297-1301，native 回调）→ `dispatch('登录', error_code, ...)` → account 的 on_login_result：

- 成功（error_code==0）：`logined=true`；记录 latest_login_info（user_id/login_id/user_name/login_way/hash_token=login_token/hash_secret）；`set_token(tk, login_way)`；`set_login_token(login_token, login_token_secret)`（HTTP 签名对，注释「不会序列化, 长期有效」——**实证：本机凭证文件里有 login_token 落盘**）；`save()` 回写。
- 失败：清 token/login_token/login_type，`logined=false`；argv `token_cache` 时重试 sdk_login 一次。
- StateEditor 断连保护：`on_lobby_disconnect` 里 `_G.XDEDITOR_MAIN_UI_READY` 时保持登录态（account.lua:443-450）。

### 1.5 HTTP 签名（account.lua:412-441，脱机复用点）

```
header.noise = 随机 7 位数
header.time_str = unix 秒字符串
pre_sign = noise\n + time_str\n + content_md5(默认空)\n + login_token\n + login_token_secret
header.token = login_token
header.sign = md5(pre_sign)
```

token/secret 缺省取 `account_data.login_token(_secret)`，回退 `latest_login_info.hash_token(_secret)`。**脱机工具只要从凭证文件读 login_token/secret 就能签名调内部 API**（updater/api-version 等查询类）。

### 1.6 lobby 事件全集（lobby.lua:95-1346，native→Lua 回调表）

`lobby_events.on_entrance_connected / on_entrance_disconnected / on_entrance_connect_error_catched / on_login_response / on_login_at_other_place_notify / on_notify_service_stopped / on_logout / on_sdk_login_result / on_start_game_notify / on_game_connected / on_game_disconnected / on_game_connect_failed / on_game_login_error / on_host_downloading_map_notify / on_reload_with_cmdline ...`

`interface.logined` 的维护：on_login_response 置位 / on_entrance_disconnected 清除 / on_logout 清除。

## 2. TapTap device flow（自登录复现细节，xdeditor ui/login.lua）

1. `POST https://www.taptap.com/oauth2/v1/device/code`
   入参：client_id（`_G.client_id`，官方值）/ response_type='device_code' / scope='public_profile' / version='1.0' / platform='nodejs' / info='{"device_id":"PC"}'
   返回：device_code / user_code / verification_url / expires_in=300 / interval=2 / qrcode_url。
2. 展示 qrcode_url（应用中生成二维码图片即可，不需要引擎的 common.generate_qrcode/create_texture）。
3. 轮询 `POST /oauth2/v1/token`：grant_type='device_token' / client_id / secret_type='hmac-sha-1' / code=device_code / version='1.0' / platform='unity' / info='{"device_id":"PC"}'，每 interval 秒，直到 expires_in。
   状态：200+success → 成功；`authorization_pending` 未扫；`authorization_waiting` 已扫待确认；`access_denied` 拒绝。
4. 成功 data：`kid / access_token / token_type='mac' / mac_key / mac_algorithm='hmac-sha-1' / scope`。
   落盘：`token = mac_key..'$'..kid`，`token_type=11`，access_token 原样存，`version=1`。
5. refresh：`POST /oauth2/v1/token` grant_type='refresh_token' + token=access_token + token_type_hint='access_token' + info='{"device_id":"PC","uuid":"<guest_id>"}' → 返回同结构新 token。

## 3. 内部服务地址推导（client_base base/utility.lua:385-422 + ip.lua）

```
_G.IP（编辑器 = editor-pd.spark.xd.com）
  editor- → editor.          → editor.pd.spark.xd.com
  首段 → 服务名              → publisher-pd.spark.xd.com
  production → pd
  pd/master 在 need_use_new_domain → spark.xd.com → tapsce.cn
  命中映射 → https
结果：publisher = https://publisher-pd.tapsce.cn:9000
      updater   = https://updater-pd.tapsce.cn:9002
      login     = https://login-pd.tapsce.cn:9011
      assign_host = http://<_G.IP>:9007/api/v1/assign_host（game host 分配）
argv server / http 可覆盖 _G.IP / http 基址
```

## 4. 脱机调试事实（editor-runtime-mechanism.md §4 摘要 + 本仓补充）

- headless：`星火编辑器.exe -generate_and_debug_map -file_path=<路径>` → main.lua:711-718 → 登录回调 → map_starter。
- map_starter 流程：地图完整性检查 → load_project_file → `trigger_manager.generate_lua_only(map_path)` → clear_folder(调试目录) → 白名单复制 → assign_host（9007 签名 POST，body={api_version}）→ `DebugManager.update_host(ip, port, token)` → `DebugManager.debug_game{...}` → `os.exit(0)`。
- 弹窗风险：完整性检查失败弹 message_window 等人点（map_starter/init.lua:179-189）。
- `_G.__fortest_still_use_local_host=true` → host 固定 127.0.0.1:5003（官方后门）。
- 调试画面尺寸：-width/-height 按项目 debug_settings 比例（上限 2340x1080）。

## 5. 工具链（本仓 examples/）

- `restore_game.py`（从 editor-patch 仓搬运）：SCE 包一键还原——TNND 解密 → 7z 解压（py7zr/7z/tar 三路回退）→ UPAK 解包（含每条目 4 字节尾校验的 SCE 变体）→ 伪 KTX 图片就地解码 PNG（BC7/DXT1/DXT5/RGBA8/RGB8，Pillow+texture2ddecoder）。本仓 client_base-78 还原件即其产物。
- 用法：`python restore_game.py <加密7z> -o <输出目录> [--keep-temp] [--no-decode-images]`。

## 6. 研究镜像位置

还原件统一放逆向镜像目录 `D:/sce_online/Res/maps/bgd_glzy/.editor_src_mirror/`（与 editor-patch 仓既有研究共用，不在本仓 doc/research 里存源码）：

| 镜像 | 来源 | 说明 |
| --- | --- | --- |
| `.editor_src_mirror/client_base-78/` | `D:\sce_online\version-13\embedded_packages\client_base-78.7z` | 77 个 lua 全解密（common/base|update|uninstall|preload|auto_test 等）；require 根 = `common/` |
