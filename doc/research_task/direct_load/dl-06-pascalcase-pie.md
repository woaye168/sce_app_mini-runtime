# dl-06 PascalCase 漏网 PIE 实测 + 游戏对象表双名补证

> 日期：2026-08-28 | 状态：✅ 完成（PIE 客户端 StateGame；线上 tester 复测待做）
> 探针：`test_res002/.bgd/src/client/test/dl06_client_io.lua`（当前在 init.lua 注释隔离，见 §3 卡死坑）

## 1. io 漏网候选存在性（PIE 客户端）——14/14 全存在

`AddResourcePath / RemoveResourcePath / WalkResourceDir / CheckExistsFile / CheckExistDir / GetPackagePath / CopyCacheFile / DownloadFile / WalkAbsoluteDir / Deserialize / Serialize / IsSerializing / Read / Write` 全部 = function，且 `io.Write ~= io.write`、`io.Read ~= io.read`（原生版与 isolation 包装版确为不同闭包）。【实测 PIE】

## 2. 行为探测结果

| 函数 | 结果 | 判定 |
| --- | --- | --- |
| `CheckExistsFile('res/characters/_user/.../model.prefab', 0)` | **false**（文件真实存在） | ⚠️ PIE 散文件不命中——疑查 pak/缓存命名空间；线上 pak 环境复测才有定论 |
| `CheckExistsFile(不存在路径)` | false | 对照正常 |
| `CheckExistDir('res', 0)` | false | 同上 |
| `WalkResourceDir('res', 1, true, 0)` | 空表 | 同上 |
| `GetPackagePath('p_55a3', '')` | nil | 语义未明 |
| `AddResourcePath(dir)` → `CheckExistsFile(相对名)` | 前后均 false | ⚠️ 本探测形态下无效果（可能 CheckExistsFile 不走 resource path 列表；需换引擎真实加载（如贴图/模型）做判定） |
| `CopyCacheFile(prefab资源名, 绝对目标)` | 返回 1（非 0），副本未落盘 | ❌ 本例失败（资源或不在缓存/语义不同） |
| `WalkAbsoluteDir(app_dir, 2, false)` | table n=13 | ✅ 可用（绝对路径目录枚举） |
| `IsSerializing()` | true | 记录（语义待考） |
| `Serialize()/Deserialize()/DownloadFile()/AddResourcePath()` 无参 | 静默 nil（不抛类型错） | 签名探测无效，只能从逆向签名走 |
| `GetPackagePath()/Write()` 无参 | 抛 `bad argument #1 (string expected)` | 确认首参 string |

## 3. ★ 卡死坑（重要）

**本探针跑完后编辑器进程死亡/卡死**（MCP 桥不可达，需强制关编辑器重启）。隔离复验：注释本探针后调试稳定。嫌疑人 = io 高危调用组（`AddResourcePath/RemoveResourcePath/CopyCacheFile` 直接改引擎资源系统状态；或 `WalkResourceDir/Serialize` 系）。**后续原则：io 高危探测逐个隔离跑，一次会话只测一个**；精确归因未做（不值得为它再炸几次编辑器）。

## 4. ★★ 双名注册普遍性补证（dl-03 §4 未决议项已闭环）

PIE 客户端运行时 `pairs` 枚举【实测】：

| 表 | 函数总数 | PascalCase 数 | 结论 |
| --- | --- | --- | --- |
| `game` | 624 | **312（精确一半）** | ✅ 全量双名注册 |
| `ui` | 164 | **82（精确一半）** | ✅ 全量双名注册 |
| `actor` / `scene` | — | — | 全局无此表（不是独立全局表） |

→ 结论升级：**双名注册是引擎全部自研 luaL_Reg 模块的普遍机制**（io/common/game/ui 四表全部实锤）。dl-03 的「游戏对象表无证据」状态解除。
另：`game` 表注册表已由 lua_api_dump 完整导出（626 条目含签名）存 `test/direct_load/game-editor.out`（编辑器引擎 sceengine.dll）。

## 5. 对主线的意义

- io 资源通道（AddResourcePath 等）在 PIE 的行为判定**不可用 CheckExistsFile 做探针**（它不命中散文件）；下一步判定方式 = 直接让引擎加载新路径下的真实资源（贴图/模型/prefab），线上 pak 环境用 `WalkResourceDir` 枚举 pak 条目做 pak 感知验证。
- game/ui 无双名「漏网」需求（isolation 不阉它们），但双名普遍性对「引擎内嵌表」（如服务端 io）推断有用。
- game 表 626 注册项中含 `set_actor_asset(integer, string)`（actor id + asset 串）——又一个 set_asset 入口（id 直驱），留作后续候选。

## 6. 关联

- 候选清单与方法论：[dl-03-pascalcase-candidates.md](dl-03-pascalcase-candidates.md)
- 先例：[../../research/pak-io-native.md](../../research/pak-io-native.md)
