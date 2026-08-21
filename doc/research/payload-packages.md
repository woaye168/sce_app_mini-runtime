# 载荷包体系：update-info、包格式、落位与版本注册表

> 最后验证：2026-08-21（C:\mini_dl_test 全链 100% 官方通道实测）
> 本文是「载荷 0 依赖自举」的完整契约。控制协议/B 模式编排见 scegame-reverse.md。

## 1. update-info 契约（版本发现唯一入口）

```
POST https://updater-pd.tapsce.cn/api/map/update-info?<参数全在 query string>
```

- **参数全在 query string、空 body（Content-Length: 0）、免签名**。⚠️ 把参数放 body 会 500；带签名头也 500。
- 关键 query 参数：`list=<包名;分隔>`、`version=2`、`api_version=<api>`、`sample=0`、`suffix=client`、`default_part=1`；编辑器侧包加 `variation=windows_editor`。
- 响应是**多行文本**：line1=版本号 / line2=buffer_type / **line3=JSON**（`items[] = {name, version, url, md5, size, original_size, path, variation, api_version}`）。
- **依赖自动展开**：查项目库会连带返回 global_default/spark_core/lib_ui 等隐式基础库（必须一并落位+登记）。

## 2. 包下载与格式三层

```
下载 https://<item.url>（OSS 公共读，md5 校验）
  → 可能是 7z，也可能带 TNND 加密头
  → 7z 内容 = 单个 <Name>.pak（首字母大写）+ 库的 libs.json
  → .pak 是 UPAK 变体（部分需解散文件）
```

| 层 | 格式 | 处理 |
| --- | --- | --- |
| 加密 | TNND：4 字节 magic `TNND` + 全文 XOR `CREATEEASY`（循环密钥） | 按 magic 识别，解密后才是 7z。**engineres 实测命中** |
| 归档 | 7z | Windows 10+ 自带 `tar -xf` 可解（bsdtar 支持 7z） |
| 包 | UPAK 变体：头 `UPAK`+u32 条目数+u32 总校验；条目 = 名字`\0`+u32 offset+u32 size+**u32 条目校验**（比标准 Urho3D 多 4 字节）；内容明文 | `_m` 注册表包与 script/appui/gameui 需解散文件；基础包保留 .pak 即可 |

## 3. 落位布局（实测）

| 包类型 | 落位 |
| --- | --- |
| 基础包（client_base/startup/fonts/engineres/uistyle/refconfig/shadercache_windows_ui/lite） | `Res/<name>/<name>.pak` |
| 注册表版本包（script/appui/gameui/lite/shadercache_editor_dxbc(_extra)） | `Res/_m/<name>/<ver>/<name>/<name>.pak` + 解散文件 |
| 项目依赖库（libs.json 的键 + 依赖展开） | `Res/_m/maps/<sub>/<ver>/<name>/` + 解散文件 + libs.json |
| 引擎包 `win` | scegame→`runtime/scegame.exe` + 全套 dll 落 runtime 根 |
| 基座资产（见下） | `ui/`、`fonts/` → `Update/<env>/Res/`；`characters/`、`effect/` → `Res/` |

## 4. 版本注册表合成（api_pak_version.json）

- 结构：`{ "#package_path": {<name>: <Res/_m/... 路径>}, "<api>": {<name>: <version>} }`。
- **凡 `_m` 落位的包都要登记**（含依赖自动展开项 global_default/spark_core/lib_ui）——否则引擎回退 `Res/maps/<lib>` 报 `cannot find package`（0.2.0 实测）。
- 官方注册表 `D:\sce_online\update\editor-pd.spark.xd.com\api_pak_version.json` 可作权威对照。

### 4.1 ★ 依赖库版本号 = 注册表权威（2026-08-21 用户指正 + 源码实证）

**依赖库（libs.json 的键）的版本号必须读 api_pak_version.json[\<api\>] 表**，不能直接用 update-info 返回的 version（那是该库的最新版，不一定是当前 api 适用的版本）。编辑器就是这么做的：

- `client_base update/core/local_version.lua` + `local_api_pak_version.lua`：编辑器更新时把下载的包按 `api_pak_version_manager:set(editor_api_version, 包名, 版本)` 写入注册表（StateEditor 模式，`same_api_version` 标记的入当前 api 表）。
- 游戏侧消费：`<api>` 表的 `{包名: 版本}` 经 `#package_path` 映射到 `Res/_m/...` 路径。
- **依赖库下载落位**（编辑器实测）：`update/<env>/res/_m/maps/`、`_m/maps/script_libs/`、`_m/maps/user_libs/` 三类——包是 7z，解压到这 3 个目录供编辑器消费。

**对 mini-runtime**：`resolve_libs` 选依赖库版本时，先读载荷注册表 `api_pak_version.json[<api>][<库名>]`；注册表缺失才回退 update-info 返回值/载荷目录最大版本。下载依赖库时按 `update-info` 的 path 字段（`Res/maps[/script_libs|/user_libs|/ai_templates]`）落 `_m/maps[/...]`。

## 5. 包清单（payload.rs 的常量）

- **基础包 BASE_PACKAGES**：client_base / startup / fonts / engineres / uistyle / refconfig / shadercache_windows_ui
- **注册表包 REGISTRY_PACKAGES**：script / appui / gameui / lite / shadercache_editor_dxbc / shadercache_editor_dxbc_extra
- **引擎包 ENGINE_PACKAGE**：`win`（⚠️ 是对战平台引擎，见 runtimes.md——但登录/跑游戏能力与编辑器一致，B 模式客户端可用）
- 编辑器侧（`variation=windows_editor`）：wineditor（version-2000 客户端，193MB）、xdeditor 等——B 模式不需要。
- `xdeditor_startup` 会被 update-info 返回但跳过（不在本次布局）。

## 6. 基座资产（update-info 不覆盖）

`ui/`（**游戏字体族 ui/font/regular**——缺了 UI 全部无文字）、`fonts/`、`characters/`、`effect/`、`anim/` 等编辑器基座资源，**任何 list 组合都不返回**，只能来自编辑器安装包。

**0.2.0 决策**：打成 `base_assets.7z` 随本仓 release 分发。
- 打包：`examples/pack_base_assets.ps1 -ProjectRoot <项目>`（从本机编辑器推导路径）。
- 下载：仓库私有 → 走 GitHub API + token（`MINI_RUNTIME_GITHUB_TOKEN` → 凭据 `bgd_sce_tools/github_token`；`MINI_RUNTIME_BASE_ASSETS_URL` 可覆盖公开直链）。
- 兜底：本机编辑器目录直接复制（tsconfig 推导）。

## 7. 版本跟随

每次 `payload sync` 实时查 update-info = 自动跟随星火服务器侧版本更新（实测 startup 364→365）。**编辑器/tester 升级后重跑 sync 即完成载荷升级**。

## 8. 判重与半成品

- 判重用「落位目录/scegame 是否存在」——**半成品目录会跳过重下**，手动删目录即可强制重下。
- 下载失败/中断不留半成品的状态目前靠目录存在性判断，改进点：可加 `.done` 标记。

## 9. 终验

C:\mini_dl_test：100% 官方通道下载载荷 → debug start → host 服务端全绿（首次加载lua完成/刷怪）→ 客户端 bgd 入口执行 → 截屏文字/BOSS/技能书全渲染。
