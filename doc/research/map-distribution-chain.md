# 地图分发链路：编辑器上传 → 双通道构建 → 客户端下载加载

> 最后验证：2026-09-02（update-info 双 variation 实时查询 + 客户端发布包下载解密条目级实证）
> 本文回答：一个正常游戏项目经星火编辑器上传/发布后，服务端与客户端内容各自去哪、谁能下载、游戏时如何加载。载荷包格式细节见 payload-packages.md；调试控制协议见 scegame-reverse.md §8；自建 host 视角见 self-host.md。

## 1. 总览

```
游戏项目（编辑器工程）
   │
   ├─ 调试（B 模式，不发布）──────────────────────────────┐
   │   编辑器/mini-runtime staging 白名单（含 script/src/ui/table/scene 全套）
   │   → 控制协议 0xF004/0xF008/0xF00A 上传到 debug host（312 文件实测）
   │   → host 侧 server VM 直接挂上传内容（@p_55a3/bgd_game_server/**）
   │   ※ 谁调试谁上传：服务端逻辑经上传链到达 host，不经任何下载包
   │
   └─ 发布（EDITOR.upload_map，菜单「发布/发布项目」）
       → 星火服务端构建，同一份项目产出两个通道的包：
       │
       ├─ 服务端通道（variation=server）
       │   server.zip 系列（bucket = sce-maps-pd-backend，ACL 私有，公网 403）
       │   官方 host 在阿里云内网拉取 → 起 server VM 跑线上真局
       │
       └─ 客户端通道（variation=client / windows_game）
           windows_game.7z 系列（bucket = sce-maps-pd，公共读）
           玩家游戏时经 update-info 查版本 → OSS 下载 → 挂载进引擎
```

**核心结论：客户端可下载的发布包里物理上不含服务端逻辑**（§4 条目级实证）——不是"拿不到授权"的问题，是内容根本不在包里。服务端构建（server.zip）走独立通道且 bucket 私有。任何"从官方下载包提取游戏服务端逻辑"的尝试都是死路；自研逻辑项目的服务端来源 = 本地项目源码（经 staging 上传链或发布构建）。

## 2. 上传侧（两条路径）

| 路径 | 触发 | 协议/接口 | 上传内容 | 去向 |
| --- | --- | --- | --- | --- |
| 调试上传 | 编辑器「调试」/ `debug start` | 控制协议 0xF004（≤100KB 整发）/ 0xF008（101400B 分块，先空声明）/ 0xF00A（FileEnd） | staging 白名单全量（script/src/ui/table/scene/res/ref/project + config.ini/libs.json/project.sce；map_starter 的 get_project_map_dirs 对应集合），路径全小写 | debug host 磁盘 → server VM 挂载 `@<项目名>/` |
| 发布上传 | 菜单「发布/发布项目」→ `EDITOR.upload_map(log_mark, promise, ...)`（xdeditor utils/event.lua:709，promise 全出口回 0/1） | 编辑器内部上传实现（未逆向，不需要） | 完整项目 | 星火服务端构建管线 → 双通道包 |

## 3. 服务端通道（variation=server，2026-09-02 实时实证）

```
POST https://updater-pd.tapsce.cn/api/map/update-info?list=p_55a3&version=2&api_version=13&sample=0&suffix=client&default_part=1&variation=server
```

- 主项：`p_55a3` v1，packet_type=1，url = `sce-maps-pd-backend.oss-cn-shanghai.aliyuncs.com/p_55a3/master/Version/1/server.zip`（1.27MB / 解压 13.7MB，map_api_version=13）。
- ref_items 自动带出服务端依赖全集：`server_common@174`（450KB）、`server_lua_plus@14`、`global_default@60`、`default_units@42` + 项目各库的 server.zip（lib_control/defaultui/smallcard_* 等，全在 sce-maps-pd-backend bucket）。注意服务端有专属的 `default_units`/`default_units_ts`（单位定义的服务端构建）。
- **下载实测 HTTP 403**（bucket ACL 私有，代理/直连均拒）——官方 host 在阿里云内网拉取（self-host.md §4.4）。若未来拿到可读凭证，此通道 = 服务端 lua 栈完整来源（self-host.md §5）。
- 服务端运行形态（云 host oracle 实证，self-host.md §11）：server VM 三层挂载 = `@common`（server_common）+ `@lua_plus`（server_lua_plus）+ `@<项目名>`（**= 项目 `script/` 目录整树**，入口 `script/main.lua`；test_res002 实例的 package.loaded 含 bgd_game_server/**、bgd_libs_server/**、obj/**、scene/default/area_save，与本地 `script/` 一一对应）。

### 3.1 服务端目录与入口（泛化结构，test_res002 实证）

**对任意星火项目：服务端脚本 = 项目根 `script/` 目录，服务端逻辑入口 = `script/main.lua`。** `bgd_game_server`/`bgd_libs_server` 只是 bgd 框架项目的构建产物目录名（框架 game_dir/libs_dir 派生），不是通用结构——非 bgd 项目的 script/ 下是官方模板自己的组织方式。

`script/main.lua` 是编辑器/构建链生成的**包装入口**（标记分段结构，与客户端 ui/src/main.lua 的包装同构）：

```
---require_common---      require"@common.base"（server_common 根）+ @global_default.lua_declare
---scene_folder---        require_folder("scene")
---init data object cache---  base.eff.init_cache()（数编缓存）
---require libs---        lib_xxx = require"@lib_xxx.main"（libs.json 依赖库，服务端构建版）
---origin_main_file---    项目自身 main 原文（test_res002 = bgd 构建合并：require bgd_libs_server/bgd_game_server + 游戏入口代码）
---ts_module---           require "trigger_module_main_1" / "trigger_validator"（触编编译产物）
---load_default_units---  base.game.init_units（地编单位初始化）
return { [<项目名>] = <项目名>, ... }  -- 模块表
```

`script/` 下的泛化内容构成（test_res002 实例）：`main.lua`（入口）+ `trigger_module_main_1.lua`/`trigger_validator.lua`（触编编译产物）+ `obj/**`（数编 lua 表）+ `scene/**`（场景保存）+ 项目自有目录（bgd 项目 = `bgd_game_server/`游戏服务端 + `bgd_libs_server/`框架服务端）。

## 4. 客户端通道（variation=client，2026-09-02 实时实证 + 条目级拆解）

```
POST ...&variation=client  → 主项：p_55a3 v1，packet_type=1，
url = sce-maps-pd.oss-cn-shanghai.aliyuncs.com/p_55a3/master/Version/1/windows_game.7z（公共读，2.1MB / 解压 30.2MB）
```

包格式三层（payload-packages.md §2）：**TNND 加密**（magic `TNND` 4 字节不加密，**body 从 offset 4 起** XOR 循环密钥 `CREATEEASY`）→ 7z → 内容 = `p_55a3.pak`（UPAK）+ `libs.json`。

**★ p_55a3.pak 条目级拆解（721 条，PowerShell UPAK 解析，留档 test/temp/p55a3_chain/）**：

| 顶层目录 | 条目数 | 性质 |
| --- | --- | --- |
| ui | 294 | 客户端 UI/客户端 lua（ui/src 等） |
| map_ref_res | 262 | 地图引用资源（贴图/特效） |
| game_hud | 125 | HUD |
| ref / res / scene / atmosphere / table | 43 | 引用定义/资源/场景/数编 |
| config.ini / libs.json / project | 3 | 配置 |

**`server` 关键词条目 = 0；无 `script/`（服务端 lua）、无 `src/`（触编）、无 bgd_game_server 任何痕迹。** 客户端发布包 = 纯客户端消费内容，服务端逻辑物理不在其中。

## 5. 游戏时加载链（玩家侧）

1. scegame 启动 → 官方自更新（update-info，variation=windows_game/client）→ 基础包/注册表包挂载（scegame-reverse.md §1 启动序列）。
2. 进具体地图：update-info 查地图版本 → OSS 下载 windows_game.7z → `added package maps/<map>/<map>.pak` + `_m/maps/<lib>/<ver>/<lib>.pak`（依赖库按 api_pak_version.json 注册表版本）。
3. 线上真局：客户端 KCP 连云端官方 host（host 侧已内网拉好 server.zip 起 server VM）；调试局：客户端连 debug host（本地或云端），服务端内容来自上传链。

## 6. 对 mini-runtime 的含义

- **0.5.0（真本地 host）的服务端逻辑来源 = 自有项目的服务端目录**（test_res002：项目根 `script/` 整树，入口 `script/main.lua`——含 `bgd_game_server/`（bgd 构建产物的游戏服务端）+ `bgd_libs_server/` + `obj/**` 数编 + `scene/**` + 触编编译产物），经 staging 白名单链供给自研 host——与官方 debug host 的"谁调试谁上传"同构。见 doc/requirements/0.5.0.md「游戏服务端逻辑来源」节。
- 不要试图从客户端发布 pak 提取服务端逻辑（§4 实证不在其中）；server.zip 不可得是 bucket ACL 问题而非格式问题。
- 调试 staging（src/core/staging.rs 白名单）已天然包含服务端所需的 script/src——0.4.0 中继链 312 文件实证。

## 7. 实证记录（2026-09-02）

```powershell
# 服务端通道查询（返回 server.zip 系列，backend bucket）
curl -X POST "https://updater-pd.tapsce.cn/api/map/update-info?list=p_55a3&version=2&api_version=13&sample=0&suffix=client&default_part=1&variation=server" -H "Content-Length: 0"
# server.zip 可达性：HTTP 403（bucket ACL 私有）
curl -o NUL -w "%{http_code}" "https://sce-maps-pd-backend.oss-cn-shanghai.aliyuncs.com/p_55a3/master/Version/1/server.zip"
# 客户端通道查询 + 下载 + TNND 解密（跳过 4B magic，body XOR CREATEEASY）+ tar 解 7z + UPAK 条目解析
curl -X POST "...&variation=client" -H "Content-Length: 0"
curl -o p55a3_client.7z "https://sce-maps-pd.oss-cn-shanghai.aliyuncs.com/p_55a3/master/Version/1/windows_game.7z"
```

留档：`test/temp/p55a3_chain/`（原始 7z / 解密 7z / p_55a3.pak / libs.json）。
