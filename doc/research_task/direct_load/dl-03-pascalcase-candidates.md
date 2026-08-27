# dl-03 PascalCase 漏网枚举：isolation 阉割 × 双名注册差集

> 日期：2026-08-27 | 状态：✅ 静态差集完成（未实测；实测矩阵见 §6 待 dl-04 执行）
> 研究主线：模型/图集子图免数编直载 —— 线索二（PascalCase 漏网）

## 0. 输入材料与方法

**输入材料（全部只读）**：

| 材料 | 路径 |
| --- | --- |
| 双名注册机制与 io 先例 | [../../research/pak-io-native.md](../../research/pak-io-native.md) §2/§3/§4 |
| common 表 524 条注册项（机制 + 分类手册） | [../../research/common-table.md](../../research/common-table.md) |
| common 表逐函数签名（编辑器引擎） | [../../research/common-table-editor.md](../../research/common-table-editor.md) |
| common 表逐函数签名（对战平台引擎） | [../../research/common-table-tester.md](../../research/common-table-tester.md) |
| isolation 阉割源码（script-199，含 editor-patch 解锁注释但行全保留） | `sce_app_editor-patch/slots/script/199/common/isolation.lua` |
| isolation 阉割速查（交叉核对） | `sce_app_editor-patch/.trae/skills/sce-lib-script-199/api.md` |
| io 表注册表 dump（编辑器 sceengine.dll） | `target/io-editor.tsv`（133 行，全量双名） |
| io 表注册表 dump（tester scegame） | `target/io-tester.tsv`（133 行，全量双名） |

**方法**：
1. 从 isolation.lua 逐行提取完整阉割清单（包装类 + 置 nil 类 + package 元表拦截）。
2. 从 io-editor.tsv / io-tester.tsv（luaL_Reg 注册表逆向产物）枚举存在 PascalCase 变体的函数——**双名是否存在的判定一律以注册表 dump 为准，不靠命名规律猜**。
3. 求交集并分类：已有线上实测先例的标【已实证先例】；与资源加载/渲染主线相关的标重点候选；其余按模块分组。
4. 对非 io 表（os/debug/package/cmsg_pack/_G）单独论证双名是否可能成立。

**置信级定义**：【逆向实锤】= luaL_Reg 注册表 dump 直接见到双名条目；【源码实锤】= lua 源码直接可见；【实测】= 线上/PIE 行为验证；【语义推测】= 仅由机制类比推断，无直接证据。

## 1. isolation 阉割完整清单（【源码实锤】script-199 common/isolation.lua）

阉割只在 `__lua_state_name == 'StateGame'` 分支生效，纯 Lua 层操作，native 实现不动。

### 1.1 包装类（保留功能 + 路径沙箱 `full()`：相对路径拼到 `User/maps/<地图>/`，绝对路径在非 editor debug 下 error）

io 表 13 个：`read / write / copy / rename / remove / copy_to_folder / create_dir / exist_dir / exist_file / walk_dir / list / attribute_type / file_time`
_G 3 个：`dofile / loadfile / load`（loadfile/load 另强制 `mode='t'` 禁二进制）

### 1.2 置 nil 类（完全禁用）

- io 表 27 个：`walk_resource_dir / walk_absolute_dir / popen / check_resource_dir / check_resource_file / deserialize / serialize / is_serializing / read_cache / read_pak_entries / extract_pak / extract_pak_file / copy_cache_file / download_file / upload_file（auto_test 时豁免）/ add_resource_path / remove_resource_path / select_file / select_files / select_folder / select_folder_new / open_path_in_explorer / show_file_in_explorer / add_watch / remove_watch / empty_method / get_package_path`
- os 表 6 个：`execute / exit / remove / rename / setlocale / tmpname`
- debug 表 10 个：`getregistry / getupvalue / setlocal / getlocal / upvaluejoin / sethook / setupvalue / setuservalue / upvalueid / gethook`
- 其他 2 个：`package.loadlib`、`cmsg_pack.set_max_pack_byte_count`

### 1.3 元表拦截（非函数替换）

`_G.package` 套 metatable：写 `package.path` 逐条校验（禁 `..` / 盘符 / `/` 开头）。只拦 path 写入，不删函数。

### 1.4 关键观察（漏网根源）

**全部 61 处阉割操作都只写小写名**。源码中无任何一处触碰 PascalCase 变体——与 pak-io-native.md §3 结论一致，本清单是完整复核（逐行，无遗漏）。

## 2. 双名注册枚举证据

### 2.1 io 表：双名注册【逆向实锤】（双引擎全量）

`target/io-editor.tsv`（133 行）与 `target/io-tester.tsv`（133 行）是 LuaIO 模块 luaL_Reg 注册表的精确 dump：**每一条注册项都同时有小写与 PascalCase 两个名字，指向同一函数 RVA，无一例外**（含命名不对称项：`exist_file→ExistsFile`、`check_resource_dir→CheckExistDir`、`check_resource_file→CheckExistsFile`、`add_skip_watch→AddSkipWatchFile`）。两引擎名字集合一致（RVA 不同）。

→ **凡在 io 表被阉割的函数，其 PascalCase 原生版必然存在**（40/40），置信级全部【逆向实锤】。

### 2.2 common 表：双名注册【逆向实锤】，但**不属于差集**

common-table-editor/tester.md 各 524 条注册项，绝大多数双名（含 `exit/Exit`、`shell/Shell`），但 **isolation 根本不碰 common 表**——小写名线上直接可用，无需走 PascalCase 漏网。common 中与本主线相关的资源/渲染函数（`reload_pak`、`load_shadercache_and_paks`、`has_full_shadercache`、`create_texture`、`get_documents_path`、`set_background_texture_path` 等）原生可用，记入 §5.4 备查，不计入候选数。

### 2.3 os / debug / _G / package / cmsg_pack：无 PascalCase 漏网

- **os 6 个、debug 10 个、`dofile/loadfile/load`**：均为 **Lua 标准库**（oslib/dblib/lbaselib），由 lua54.dll 标准注册器注册，不走引擎 LuaIO 式双名注册块；两份引擎 dump 中也无任何 PascalCase 版标准库函数踪迹。判定：**无漏网**（机制判断，高置信）。
- **`package.loadlib`**：标准库函数，同上，无漏网。
- **`cmsg_pack.set_max_pack_byte_count`**：引擎自定义 C 模块，无双名注册证据（无该表注册表 dump）。【语义推测】大概率无 PascalCase 变体，且与本主线无关，不投入实测。

## 3. 差集结果：候选总表（40 条，全部 io 表，全部【逆向实锤】）

> 签名取自 io-editor.tsv（反汇编推断，可选参数不标注；实测以官方调用点复核过的为准）。

### 3.1 【已实证先例】（pak-io-native.md §4 线上/PIE 实测通过，10 条）

| 小写名 | PascalCase 名 | 签名 | 阉割方式 | 实测状态 |
| --- | --- | --- | --- | --- |
| read_pak_entries | ReadPakEntries | (string pak路径) -> 1 条目数组 | 置 nil | ✅ 线上 TNND pak 透明解密读 1301 条目 |
| extract_pak_file | ExtractPakFile | (string pak, string 条目, string 目标) -> 1（0=成功） | 置 nil | ✅ 线上解 35MB mp4，三端通过 |
| extract_pak | ExtractPak | (string pak, string 目录) -> 1 | 置 nil | 注册实锤，行为未实测 |
| read_cache | ReadCache | () -> ? | 置 nil | ⚠️ 实测不可靠（假命中），勿用 |
| read | Read | (string path, ...) -> (err, content) | 包装 | ✅ 绝对路径可用（绕过沙箱） |
| write | Write | (string path, ..., string content) -> 1（0=成功） | 包装 | ✅ 无沙箱限制 |
| copy | Copy | (string src, string dst) -> 1 | 包装 | ✅ 签名复核 |
| list | List | (string path, integer mode) -> (err, 数组)，1=文件 2=目录 | 包装 | ✅ 枚举 Update/env 目录 |
| exist_file | ExistsFile | (string) -> 1 | 包装 | ✅ 绝对路径可用 |
| exist_dir | ExistDir | (string) -> 1 | 包装 | ✅ 绝对路径可用 |

### 3.2 ★ 重点候选（资源加载/包管理/缓存——本任务主线直接相关，13 条）

| 小写名 | PascalCase 名 | 签名 | 阉割方式 | 一句话理由 |
| --- | --- | --- | --- | --- |
| add_resource_path | AddResourcePath | (string) -> ? | 置 nil | ★★ 资源搜索路径注入——挂载额外目录后引擎资源系统（模型/贴图/图集）可直接从新路径加载，免数编直载最可能的杠杆 |
| remove_resource_path | RemoveResourcePath | (string) -> ? | 置 nil | AddResourcePath 配套清理 |
| walk_resource_dir | WalkResourceDir | (string, integer, boolean, integer) -> 1 | 置 nil | 资源目录枚举（pak 感知），可枚举线上 pak 内模型/图集条目清单 |
| check_resource_file | CheckExistsFile | (string, integer) -> 1 | 置 nil | 资源存在性探测（pak 感知），探针基础设施 |
| check_resource_dir | CheckExistDir | (string, integer) -> 1 | 置 nil | 资源目录存在性探测（注意 PascalCase 是不对称命名 CheckExistDir） |
| get_package_path | GetPackagePath | (string, string) -> 1 | 置 nil | 包文件路径定位（pak 落点推导） |
| copy_cache_file | CopyCacheFile | (string, string) -> 1 | 置 nil | 从引擎资源缓存复制文件到磁盘——若模型/贴图已在缓存，可直接取磁盘副本 |
| download_file | DownloadFile | (string, string) -> ? | 置 nil | 下载资源到磁盘（CDN 拉模型/图集散文件的兜底通道） |
| walk_absolute_dir | WalkAbsoluteDir | (string, integer, boolean) -> 1 | 置 nil | 绝对路径目录枚举（提取落点/缓存目录盘点） |
| deserialize | Deserialize | () -> ? | 置 nil | 反序列化（签名异常，疑非标准取参；语义待考，可能与资源对象还原相关） |
| serialize | Serialize | (userdata) -> ? | 置 nil | 序列化（与 empty_method 同 RVA 0x1281920/0x1983370，疑似占位空实现，低期望） |
| is_serializing | IsSerializing | (userdata) -> 1 | 置 nil | 序列化状态查询（配套） |
| upload_file | UploadFile | (string, string) -> ? | 置 nil（auto_test 豁免） | 上传（诊断回传用，非主线但属资源 IO） |

### 3.3 其余候选（按模块分组，17 条）

**文件操作（包装类，PascalCase 版绕过 `full()` 沙箱、绝对路径直传）**：

| 小写名 | PascalCase 名 | 签名 | 阉割方式 |
| --- | --- | --- | --- |
| create_dir | CreateDir | (string) -> 1 | 包装 |
| remove | Remove | (string, boolean, boolean, string) -> ? | 包装 |
| rename | Rename | (string, string) -> 1 | 包装 |
| copy_to_folder | CopyToFolder | (string, string, boolean, boolean) -> 1 | 包装 |
| walk_dir | WalkDir | (string, integer) -> 1 | 包装 |
| attribute_type | AttributeType | (string) -> 1 | 包装 |
| file_time | FileTime | (integer) -> ? | 包装 |

**文件对话框（PC 端，线上移动端大概率空实现）**：

| 小写名 | PascalCase 名 | 签名 | 阉割方式 |
| --- | --- | --- | --- |
| select_file | SelectFile | (string, string, string) -> 1 | 置 nil |
| select_files | SelectFiles | (string, string, string) -> 1 | 置 nil |
| select_folder | SelectFolder | (string) -> 1 | 置 nil |
| select_folder_new | SelectFolderNew | (string, string, boolean) -> 1 | 置 nil |
| open_path_in_explorer | OpenPathInExplorer | (string) -> 1 | 置 nil |
| show_file_in_explorer | ShowFileInExplorer | (string) -> 1 | 置 nil |

**文件监听 / 其他**：

| 小写名 | PascalCase 名 | 签名 | 阉割方式 |
| --- | --- | --- | --- |
| add_watch | AddWatch | (string, boolean, boolean) -> 1 | 置 nil |
| remove_watch | RemoveWatch | (string) -> 1 | 置 nil |
| popen | Popen | (string, string) -> ? | 置 nil |
| empty_method | EmptyMethod | (userdata) -> ? | 置 nil |

### 3.4 无漏网（阉割有效，21 条）

os 6（execute/exit/remove/rename/setlocale/tmpname）、debug 10、_G 3（dofile/loadfile/load）、package.loadlib、cmsg_pack.set_max_pack_byte_count —— 依据见 §2.3。

## 4. 双名注册是否普遍存在：证据结论

| 表 | 双名注册 | 证据 |
| --- | --- | --- |
| **io（LuaIO 模块）** | ✅ 全量双名，无一例外 | 【逆向实锤】双引擎 luaL_Reg 注册表 dump 各 133 行逐条成对（io-editor.tsv / io-tester.tsv） |
| **common（引擎全局表）** | ✅ 绝大多数双名（含不对称命名与个别仅 PascalCase 项） | 【逆向实锤】双引擎注册表各 524 条 + 运行时 `pairs(common)` 枚举 517 名互证（common-table*.md） |
| **os/debug/package/_G 标准库** | ❌ 无双名 | 机制判断：标准库走 lua54 标准注册器，非引擎注册块；dump 中无任何踪迹 |
| **game / ui / actor / scene 等游戏对象表** | ❓ 未知 | **无任何注册表 dump 证据**——两张已 dump 表（io/common）全中只是「引擎自研 luaL_Reg 模块」的 2 个样本，不能外推。需 dl-04 对 game/ui 表跑 `lua_api_dump` 或 PIE 运行时 `pairs` 枚举补证 |

**结论**：双名注册在「引擎 C++ 自研的 luaL_Reg 模块」内目前 2/2 全中（io、common），可视为引擎注册惯例的高置信假设；但对游戏对象表（game/ui/actor——本主线最关心的模型/effect/scene API 所在层）**只有语义推测，无实锤**，必须实测补证。os/debug 等标准库明确不适用。

## 5. 补充备查

### 5.1 common 表中本主线可用（无需漏网，isolation 不碰）

`reload_pak(string)` / `load_shadercache_and_paks()` / `has_full_shadercache()` / `create_texture(string, int, int)` / `get_documents_path()` / `set_background_texture_path/uv`（编辑器构建为空实现 RVA 0x1281920）/ `reload_font_map()`。双名与小写均可用，优先级高于 io 漏网通道。

### 5.2 io 表未被阉割但同表注册的函数（小写本就可用）

`unzip_file / zip_file / unzip_file_from_mem(_keep_case) / unzip_tarlz4_file / copy_not_decode / delete_dir / create_symbol_link / file_size / clear_logs / save_file_dialog / add_skip_watch / remove_skip_watch / clear_skip_watch / folder_skip_watch / pause_watch / write_to_file_end / close_package_holding / close_all_packages_holding / set_package_io_mode / set_multi_threaded_resource_loading`——isolation 未碰（`unzip_file/zip_file` 在源码里本就注释掉未启用）。其中 `set_multi_threaded_resource_loading`、`set_package_io_mode`、`close_package_holding` 与资源加载管线相关，值得 dl-04 一并探测。

### 5.3 官方堵洞风险

PascalCase 漏网是 isolation 的实现疏漏，官方任何版本可能补阉割——所有使用点必须 `if io.XxxYyy == nil then 回退`（先例：extractor.lua 对 `io.extract_pak_file` 的判 nil 守卫）。

## 6. PIE 实测矩阵设计（待 dl-04 执行）

**探针落点**：test_res002 `.bgd/src/client/`（客户端入口 init.lua 挂探针）与服务端各一份（StateGame 阉割双端同生效，需双端分别验证）；日志 `log.info`，不提交 git。

**三段式探测模板（每个候选函数）**：

```lua
-- ① 存在性探测
log.info('probe AddResourcePath: ' .. tostring(io.AddResourcePath))
-- ② 签名探测（故意错参，读 C 侧 luaL_error 的类型提示）
local ok, err = pcall(io.AddResourcePath)  -- 无参 → 报错信息暴露期望类型，如 "string expected, got no value"
log.info('sig probe: ' .. tostring(err))
-- ③ 行为探测（最小副作用用例，见下表逐条）
```

**重点候选行为探测设计**：

| # | 函数 | 存在性/签名探测 | 行为探测（最小探针） | 判定标准 |
| --- | --- | --- | --- | --- |
| 1 | io.AddResourcePath | `~= nil` + 无参 pcall 读类型错误 | 建临时目录放一张测试贴图 → `AddResourcePath(目录)` → 用 `io.CheckExistsFile('相对资源路径')` 或引擎贴图加载 API 试载 → `RemoveResourcePath` 清理 | 新路径内资源可被引擎资源系统命中 |
| 2 | io.WalkResourceDir | 同上 | 对已知地图资源目录（如 `res`）调用，逐条 log 返回 | 返回条目与 pak/散文件内容吻合（PIE 散文件 / tester pak 各跑一次对比） |
| 3 | io.CheckExistsFile / io.CheckExistDir | 同上 | 传已知存在的资源路径（`res/xxx.png`）与不存在路径对照 | 存在=true / 不存在=false，且 pak 内条目可命中（pak 感知实锤） |
| 4 | io.GetPackagePath | 同上 | 传地图名/包名组合试探（签名 (string,string) 语义待辨，先用错参错误信息收窄） | 返回合法 pak 磁盘路径 |
| 5 | io.CopyCacheFile | 同上 | 对已加载过的贴图资源名 → 目标绝对路径；`io.ExistsFile(目标)` 复核 | 目标文件落盘且内容合法（文件头 magic 校验） |
| 6 | io.DownloadFile | 同上 | 下载一个已知 http 小文件到 User 目录 | 文件落盘、内容一致（生产兜底通道，低优先级） |
| 7 | io.Deserialize / io.Serialize / io.IsSerializing | 存在性 + pcall 错参 | 仅 pcall 探测不深挖（serialize 与 empty_method 同 RVA，疑似空实现） | 记录返回/报错，语义归档 |
| 8 | io.Read / io.Write（包装逃逸复核） | `io.Write ~= io.write`（与被包装的小写版不是同一闭包即为原生） | 写绝对路径临时文件 → `io.Read` 读回比对 → 删除 | 沙箱外绝对路径读写成功 |
| 9 | io.WalkAbsoluteDir | 同上 | 枚举 `common.get_app_dir() .. 'Update'` | 与 io.List 结果交叉一致 |

**游戏对象表双名补证探针（§4 未决议项）**：

```lua
-- PIE 客户端：枚举 game/ui 表，比对是否存在 PascalCase 键
for _, t in ipairs({'game', 'ui', 'actor'}) do
    local tbl = _G[t]
    if tbl then
        local n_pascal = 0
        for k, v in pairs(tbl) do
            if type(v) == 'function' and k:match('^[A-Z]') then n_pascal = n_pascal + 1 end
        end
        log.info(t .. ' pascal funcs: ' .. n_pascal)
    end
end
-- 若有命中，对具体函数做 io.AddResourcePath 同款三段探测；
-- 零命中则结论收敛为「双名仅限引擎底层模块（io/common），游戏对象表无漏网」。
```

**执行注意**：
- 每个探针先判 nil（§5.3）；行为探测产生的临时文件全部落在 `User/maps/<地图>/` 或 app_dir 临时目录，测完清理。
- tester 线上复测时重点验证 1/2/3/5（资源通道），它们决定「免数编直载」能否走通 pak 环境。
- 编辑器 PIE 下 isolation 的 `full()` 因 `editor_server_debug`/`editor_lobby_debug` argv 可能放行绝对路径，**包装逃逸的价值要在 tester 线上验证才有定论**（PIE 下小写包装版可能也好使，属环境差异陷阱）。

## 7. 关联文档

- 先例与机制：[../../research/pak-io-native.md](../../research/pak-io-native.md)
- common 表全集：[../../research/common-table.md](../../research/common-table.md) / [editor](../../research/common-table-editor.md) / [tester](../../research/common-table-tester.md)
- 前序修正：[dl-01-knowledge-corrections.md](dl-01-knowledge-corrections.md)
- 进度看板：[PROGRESS.md](PROGRESS.md)
