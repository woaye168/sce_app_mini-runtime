# render-19: SCE.GameWorld 的 load_map / set_map_dir / create_scene / use_light_group 逆向

> 对象：`target/debug/runtime/version-13/sceengine.dll`（api13，PE32+ x64，image_base 0x180000000）。
> 方法：luaL 注册表定位（字符串 → RIP-relative lea 引用）+ capstone 线性反汇编。
> 源文件水印：`D:\BuildPC\NE_pd\Client\src\Game\Unit\GameWorld.cpp`、`...\Engine\Scene\LightGroupTimer.cpp`。

## 结论先行

| 方法 | 参数（lua） | 语义 |
| --- | --- | --- |
| `set_map_dir(dir)` | 1 个字符串 | **纯赋值**：把字符串存进 GameWorld 成员 `+0x5e8`（m_map_dir），无任何加载/校验。供后续 load_map 拼路径用。构造函数默认从引擎上下文取游戏根路径填入。 |
| `load_map(name, flag?)` | 字符串 + 可选 bool | 两步：① 预载：读 `<m_map_dir>/scene/<name>/area_save.lua`（触发区域信息）；② 在 GameWorld 的**地图注册表**（`+0x408` 哈希表 + `+0x450` 记录数组）里按 name 的 sdbm 哈希查记录，查不到则打日志 `Failed to load map, scene[%s] is invalid.` 并失败返回；查到则拼 `scene/<记录目录名>/` 下的 `map.acmap` + `ClientCollision.dat` + `HeightData.dat` + `Sight.dat` 四个文件交给场景加载器。**name 不是任意路径，是注册表里的地图名/ID**。 |
| `create_scene(flag?)` | 可选 bool | 无字符串参数。新建空 Scene 对象（0x428 字节）替换 `this+0x240` 场景指针；flag 为假时额外把 `this+0x258` 对象（相机类）做一次绑定调用。**不负责加载地图**。 |
| `use_light_group(path, time?)` | 字符串 + 可选 number(float) | 调用 `GameWorld+0x28` 指向的 LightGroupTimer 子对象：日志 `Switch light group: %s.`（LightGroupTimer.cpp:0x5a），经资源管理器加载 path 指定的 lightgroup 资源并切换，float 参数为切换时间/淡入。**`+0x28` 为 null 时立即空指针硬崩**（见下）。 |

### use_light_group 硬崩原因（高度可疑点）

impl `0x18187e040` 开头：

```asm
mov rax, [rcx]        ; rcx = GameWorld+0x28（成员指针的值 = LightGroupTimer 对象）
call qword ptr [rax+8] ; 虚调用
```

`GameWorld+0x28` 存的是 LightGroupTimer **对象指针**。若该成员为 null（innerWorld 未完成场景/渲染初始化时极可能为 null），`call [rax+8]` 即读地址 0x8 → 访问违规，**lua pcall 拦不住，进程直接崩**。我们传 `'Editor/Light/Engine/default.lightgroup'` 崩掉，大概率不是路径问题，而是 `+0x28` 为 null（调用时机/前提未满足）。即便非 null，path 也需是资源系统可解析的 lightgroup 资源路径。

## 注册表（方法 → wrapper VA）

GameWorld 类注册函数 `0x181346aa0`（类名串 `"GameWorld"` @ `0x1826cf6b8`），经 helper `0x181a81fc0`（rdx=名, r8=函数）逐个注册：

| 方法 | wrapper VA | impl / 关键调用 |
| --- | --- | --- |
| set_map_dir | 0x181347790 | 0x18175f540（string 赋到 this+0x5e8） |
| create_scene | 0x181347070 | 0x18174b260 |
| load_map | 0x1813472f0 | 预载 0x18174a340 → 主载 0x181755220 → 场景加载 0x181755630 |
| purge | 0x181347430 | — |
| setup_viewport | 0x181347cf0 | — |
| destroy_viewport | 0x181347110 | 0x18174c620 |
| add_game_unit | 0x181346ce0 | — |
| remove_game_unit | 0x1813474f0 | — |
| add_game_actor | 0x181346c40 | — |
| remove_game_actor | 0x181347460 | — |
| set_render_path | 0x181347860 | — |
| set_render_target_link | 0x181347b00 | — |
| set_camera_info | 0x181347570 | — |
| use_light_group | 0x181347dc0 | 0x18187e040（LightGroupTimer） |
| ReleaseRenderTargetLink | 0x181347ff0 | — |
| GetGameWorldInfos | 0x181347eb0 | — |

注：另有**模块级** `load_map`/`LoadMainMap`/`SaveJson`/`reset`/`Reset`（wrapper `0x181337a40` 等，表在 .rdata `0x26cd100` 区，self 校验用魔数 0xfff0b9d7 的单体模式），与 GameWorld 方法同名但不同入口，别混淆。

## load_map 调用链细节（证据）

wrapper `0x1813472f0`：
- arg1 self 校验（"GameWorld"）；arg2 `luaL_checkstring` → C++ string；arg3 `lua_toboolean` 类转换（缺省 false）。
- 对 name 算 djb2 哈希（`edx=0x1505; imul 0x21`）。
- `call 0x18174a340(this, hash, &name, &name, arg5=0)` —— 预载。
- `call 0x181755220(this, &out, &name, flag)` —— 主载。

预载 `0x18174a340`：
- `r15 = this + 0x5e8 + (arg5<<4)`，arg5=0 → 正是 m_map_dir。
- 拼串：`<m_map_dir>` + `/scene/` + `<name>` + `/area_save.lua`（常量串 `/scene/` @0x182699e88、`/area_save.lua` @0x18269c590）。
- `call 0x181753e00(...)`：日志串 `Load combined scene trigger area info => failed to open file %s.` / `failed to read file: %s`（GameWorld.cpp）——以**数据文件**方式读取触发区域信息。

主载 `0x181755220`：
- `call 0x18174dcf0(this, &name)` → 注册表查找：`this+0x408` 哈希表（桶 `+0x418`，sdbm 哈希 `imul 0x1003f`），命中后取 `entry[+0x28]` 作索引，返回 `this+0x450 + idx*0xC0` 的记录；未命中返回 `this+0x458` 空记录。
- 记录布局：`+0x00` u32 地图 id；`+0x18` 目录名长度；`+0x20` 目录名指针。记录名长度 0 → 日志 `Failed to load map, scene[%s] is invalid.`（GameWorld.cpp:0x36d），返回 false。
- 命中后拼四条路径（相对路径，常量串 `scene/` @0x18269c588、`map.acmap` @0x182699ea0、`ClientCollision.dat` @0x18269c648、`HeightData.dat` @0x18269d690、`Sight.dat` @0x18269d8c8）：
  `scene/<记录目录名>/map.acmap`、`.../ClientCollision.dat`、`.../HeightData.dat`、`.../Sight.dat`
- `call 0x181755630(this, &sceneOut, 记录id, &acmap路径, [&Sight, &Height, &Collision, flag])` —— 真正的场景加载器（内部走引擎资源系统，相对路径由 VFS/pak 层解析）。

set_map_dir impl `0x18175f540`：整函数就是把参数字符串 assign 到 `this+0x5e8`，别无他物（另有兄弟函数 `0x18175f5e0` 把 `extra_maps/<name>/` 写进 `this+0x5f8`，由 SetGameInfo 启动链调用，未见 lua 导出）。

create_scene impl `0x18174b260`：new 0x428 字节 Scene（ctor 0x1808c0ba0），原子替换 `this+0x240`；从旧场景/上下文中继承若干标志位（`+0x6c9/+0x6cc/+0x6d0/+0x6d4`）；wrapper 在 arg2 为假时对 `this+0x258` 做绑定调用。

## 地图目录来源（map_dir 的正常值）

- GameWorld 构造（`0x181346d70`）：new 0x780 字节对象后，`call [ctx+0x160]` 取路径字符串 → `set_map_dir` 默认值（应为游戏根/地图根）。
- 启动链 SetGameInfo 区（`0x1817245xx-0x1817248xx`）有 `-map_path must be an absolution path[%s]`、`gamePath_: %s`、`SetGameInfo => gameName_(%s) != mapName(%s)` —— **正式流程里 map_dir 来自命令行 `-map_path` 绝对路径**。编辑器/PIE 下应为项目地图目录的绝对路径。
- `set_map_dir` 接受任意字符串（无校验），绝对路径最稳。

## 对研究目标（运行时动态加载地图）的评估

1. **load_map 不是"给我路径就加载"**：name 必须先存在于 GameWorld 的地图注册表，否则只会得到一条 `Failed to load map, scene[%s] is invalid.` 日志。注册表的填充点未完全定位（疑似 SetGameInfo/地图列表初始化链），这是最大不确定性——**PIE 里先拿当前已加载地图的 name 试**，它必然已注册。
2. 路径形态：name = 地图目录名（`<map_dir>/scene/<name>/` 与 VFS 相对路径 `scene/<记录目录名>/` 共用此名），不是文件路径、不是 pak 内路径。
3. pak 感知：四个场景文件走引擎资源系统的相对路径解析（与引擎其他资源一致，松散文件/pak 均可）；`area_save.lua` 走 `<map_dir>` 拼出的文件路径直接 open——**map_dir 必须指向真实文件系统目录**（pak 内的 area_save.lua 读不到，只会打 failed to open 日志，疑似可降级继续）。
4. UIWorld innerWorld 上调用：load_map 会替换 `this+0x240` 场景内容（经 0x181755630 内对旧场景的处理），对 innerWorld 意味着把它的场景整个换掉；与 create_scene/destroy_viewport/purge 的配合顺序需实测。

## PIE 实测建议（lua 探针草案）

```lua
-- 探针 0：拿到 innerWorld（按现有代码路径调整）
local SCE = ImportSCEContext(nil)
local w = UIWorld and UIWorld.innerWorld  -- 或既有 GameWorld 实例

-- 探针 1：当前地图名重载（必然已注册，验证链路本身）
-- 先打印当前地图名来源（数编表/base.game 里取），假设为 cur
w:load_map(cur, false)   -- 观察日志：无 "Failed to load map" 即链路通

-- 探针 2：set_map_dir + 异名地图（验证 map_dir 语义）
w:set_map_dir('D:/sce_online/Res/maps/<项目>')  -- 含 scene/<name>/ 的真实目录，绝对路径
w:load_map('<另一地图目录名>', false)
-- 观察日志：
--   "Load combined scene trigger area info => failed to open file ..." → area_save.lua 路径不对，map_dir 形态需调整
--   "Failed to load map, scene[xxx] is invalid." → 注册表没有该 name，需先弄清注册来源

-- 探针 3：空场景
w:create_scene()          -- 无参
w:create_scene(true)      -- flag=true，跳过 +0x258 绑定

-- 探针 4：use_light_group 崩溃前提验证（会硬崩，单独一局测）
-- A. 裸 innerWorld 直接调（预期崩，验证 +0x28 为 null 假说）：
-- w:use_light_group('x', 0)
-- B. 先 create_scene + setup_viewport 再调（若不再崩=前提不足导致；仍崩=路径/资源问题）：
w:create_scene()
-- w:setup_viewport(...)  -- 按既有调用形态
w:use_light_group('Editor/Light/Engine/default.lightgroup', 0)
-- 注：第三参是 number（切换时间），别省成 nil 以外的类型；pcall 无法拦 AV，务必分批测
```

日志观察点：游戏/编辑器日志里搜 `Failed to load map`、`Load combined scene trigger area info`、`Switch light group` 三个串即可定位走到哪一步。

## 遗留问题

- 地图注册表（`+0x408/+0x450`）的填充入口未定位（疑似 SetGameInfo 链或地图列表配置加载）——决定"任意项目地图能否直接 load_map"。
- `0x181755630` 场景加载器内部（pak/松散文件解析顺序、对旧场景的处置）未展开。
- arg3 flag 的精确语义（透传到场景加载器，疑似 reload/重启类开关）未定性。
