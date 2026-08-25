# render-18 — ★★ native 数编注册表与 set_asset 静态逆向（sceengine.dll api13）

> 研究日期：2026-08-25 | 状态：✅ 静态分析完成，frida 动态验证待跑
> 对象：`target/debug/runtime/version-13/sceengine.dll`（PE32+ x64，50.8MB，imagebase 0x180000000）
> 工具：examples/find_xref.rs / disasm_at.rs（rustc 直编绕过 frida-sys 的 libclang 依赖：`rustc --edition 2021 -O examples/find_xref.rs -o test/temp/find_xref.exe`；disasm_at 加 `--extern capstone=target/debug/deps/libcapstone-*.rlib -L target/debug/deps`）
> 前置：render-17（G24~G30 实证：创建/换资产全部卡在 native 数编注册表）
> 本文 VA/RVA 并记：RVA = VA - 0x180000000；.text 文件偏移 = RVA - 0xC00；.rdata 文件偏移 = RVA - 0x1200；.data 文件偏移 = RVA - 0x1A00

## 0. 一句话结论

1. **数编注册表 = GameDataManagerImp（`Client\src\Game\Table\GameDataMgrImp.cpp`）持有的按 `typeid(u64)` 索引的表族**（ActorTable/UnitTable/SpellTable…，u64 疑似 link 字符串哈希）；加载走启动期 `LoadMapTable`（"Map table is loaded, skip it." 一次性闸），**全 dll 无任何 Register/Insert/Reload TableEntry 字符串与 lua 暴露口——运行时插入口不存在于符号面，唯一可行路径是 frida 找到加载期的容器 insert 函数后直接调用**。
2. **set_asset 的 wrapper 对参数零解析**：`luaL_checkstring(L,2)` 取字符串后直接虚调用 `actor->vtable[+0xa0](string)`；ModelActor/EffectActor/GameUnit **共用同一个 wrapper**（注册在基类 GameActor 块上，派生类不重复注册），行为差异全在各自 vtable +0xa0 的覆写。裸路径失败点在虚函数内部的注册表查找（ModelActor 侧有 "cannot find typeid(%llu) in ActorTable" / "Cannot find model in ActorTable" 日志点）。
3. **特效侧发现一个「字符串哈希注册表 + 裸路径回退加载」的函数**（VA 0x18129fb50）：先按 `h=h*0x1003f+c` 哈希在桶链容器查 key 字符串，miss 后调用 `0x180777810(loader, &path, 1)` 疑似按路径直载，再失败才打 "can't find effect : %s"——**这与 G30 裸路径 set_asset 无报错无效果并存的现象矛盾，提示该函数可能不是 EffectActor::SetAsset，或回退加载"成功但渲染未刷新"**，是头号 frida 验证点。
4. 旁路：Urho3D `ResourceCache` 在 dll 内完整存在（GetResource/AddManualResource/AddPackageFile…），lua 可触达的最近点是 `set_mesh_asset_material` wrapper（内部 GetSubsystem('ResourceCache'）按名取 Material）——**材质资产走 ResourceCache 按路径/名解析，不依赖数编**。

## 1. CreateActor → GetActorTableEntry 调用链

### 1.1 定位

- 失败日志串 `"[CreateActor:%d,%s] failed, GetActorTableEntry failed."` VA 0x182725cb8（.rdata 0x2724ab8），lea xref 唯一：**VA 0x1816fa31f**（RVA 0x16fa31f）。
- 所在函数 = CreateActor native 实现，**VA 0x1816fa290（RVA 0x16fa290）**，签名约 `(this, rdx=&id_inout, r8=?, r9=key_struct)`：
  - `[this+0x330]` 非空检查（世界/表持有者）；
  - `[rdx]`（actor id）≤0 时从 `[this+0x44c]` 取并自减（id 分配器）；
  - 查表是**两级虚调用**（0x1816fa2dd 起）：
    ```
    mov rax,[rcx]         ; this
    lea rdx,[rbp-0x30]
    call [rax+0x60]       ; → 取 TableManager/GameDataManager 接口
    mov rcx,[rax]
    mov rax,[rcx]
    mov r8,r15            ; r15 = key 结构体（+8 = link C 字符串）
    lea rdx,[rbp-0x38]    ; &out entry
    call [rax+0x50]       ; = GetActorTableEntry(key) → [rbp-0x38]=entry
    ```
  - entry 为 null → 打失败日志：`r8d=[r14]`（id）、`r9=[r15+8]`（**link 原字符串**）→ lookup key 是 link 字符串（或其哈希），不是预解析 ID。
- 兄弟日志串（同区 .rdata 0x2724ab8~0x2724bf0）：
  `[CreateActor:%d] failed, CreateActor failed.` / `unit already exists.` / `[CreateUnit:%d] failed, GetUnitTableEntryByName failed.` / `[CreateActor:%d,%s] failed in AdvancedSpellEffects, GetActorTableEntry failed.`

### 1.2 注册表容器形态

- RTTI 类型名块（.data 0x2da5838~0x2da5e48，连续排列）给出完整表族：
  `DecorationTableEntry / ActorModelTableEntry / ActorEffectTableEntry / ActorSoundTableEntry / ActorTableEntry / ModelActorEntry / EffectActorEntry / GridActorEntry / SoundActorEntry / TextActorEntry / MaterialActorEntry / AdditionModelActorEntry / ActionActorEntry / BeamActorEntry / UnitTableEntry / SpellTableEntry / SoundTableEntry / UnitSoundTableEntry / ClientSpellTableEntry / ClientBuffTableEntry / AnimationTableEntry / MapInfoTableEntry / ConstantTableEntry / ConfigTableEntry / LightningTableEntry / EventParticleTableEntry / CameraTableEntry / UnitModelTableEntry / AccessConfigTableEntry / SpellAssistTableEntry / **MapTableManager** / **GameDataManager / GameDataManagerImp**`
- 源文件路径证据：`D:\BuildPC\NE_pd\Client\src\Game\Table\GameDataMgrImp.cpp`、`GameDataStructures.cpp`（10 处日志点：VA 0x181822b8e / 0x181831427 / 0x181831d47 / 0x181831f36 / 0x1818327cd / 0x181833074 / 0x18183310c / 0x1818344f2 / 0x1818345dd / 0x18183895f）。
- **lookup key = u64 typeid**：ModelActor/GameUnit 初始化失败日志 `"ModelActor ==> Init, but cannot find typeid(%llu) in ActorTable."`（VA 0x182730040，xref 0x181782987 / 0x181794799 / 0x18179bd4b）、`"GameUnit ==> Init, but can not find unitTable by typeid(%llu)."`、`"SetModelComponent, but cannot find typeid(%llu) in ActorTable."`（VA 0x182730360，xref 0x181783e80）。typeid(%llu) 为 64 位——疑似 link 字符串的哈希（与 §3 特效表同款思路）。
- 取条目函数样本（VA 0x181782710）：`[actor+0x28]` → vfunc+0x60 取 manager → **manager vfunc+0x58(out, r8d=typeid)** → entry 缓存进 `[actor+0xe0]`。

### 1.3 写入/插入路径（启动加载）

- 加载日志 `"==> Begin loading table [%s]. "` VA 0x182726588，xref **VA 0x1816fd2c9**（RVA 0x16fd2c9）——所在大函数（约 0x1816fcXXX 起）即 `LoadMapTable` 族：`r15`=manager（`[r15+0x3e8]`=表名字符串作日志参数、`[r15+0x3f0/0x3f8]`=路径串、`[r15+0x68]`=状态），内有读文件/解析/拼装字符串大量逻辑。
- `"Map table is loaded, skip it."`（VA 0x1827265c8）——**一次性加载闸**（已加载则跳过），佐证运行时不会再进插入路径。
- 相关字符串：`'$MapPath$/table/;$MapPath$/'`、`'%s/tableH'`、`'4_load_table'`、`'-not_read_table'`（exe 启动参数，可整表不读）。
- **运行时注册口静态判否**：全 dll 字符串面不存在 `RegisterEntry` / `ReloadTable` / `EntryManager` / `AddTableEntry` / `InsertTableEntry`（0 命中）；`TableManager` 仅 RTTI 一处。lua 注册面（render-01 穷举 + 本文 §2 绑定区）也无表注册 API。
- **结论**：注册表写入只发生在 LoadMapTable 加载期；运行时想插入条目，只能 frida 定位加载函数内部的容器 insert（std 容器 emplace/operator[]）后用正确参数直接调用——这是下一步动态工作的核心目标。

## 2. set_asset 解析链

### 2.1 lua 绑定区（VA 0x181345570 起的大注册函数）

绑定辅助：`0x181a81ac0`=注册类(rcx=binder, rdx=基类名, r8=类名)、`0x181a81fc0`=注册方法(rcx, rdx=名, r8=wrapper)、`0x181a81890`=checkudata(L, idx, 类名)。

类继承注册顺序（证据：0x1813456a5~0x1813457f6 的连续 lea）：

```
GameActor(基='')               → 方法: play/pause/resume/stop/attach_to/detach/set_asset/show...
ModelActor(基=GameActor)       → 自有: play/stop/is_playing/get_mesh_asset/set_mesh_asset_material（不注册 set_asset！）
AdditionModelActor/EffectActor/BeamActor/MaterialActor(基=GameActor)  → 均不注册 set_asset
GameUnit(基=ModelActor)
GameWorld → set_map_dir/create_scene/load_map/purge/setup_viewport/destroy_viewport/add_game_unit/remove_game_unit/add_game_actor/remove_game_actor/set_render_path/set_render_target_link/set_camera_info/use_light_group
```

### 2.2 set_asset wrapper（VA 0x181345a10，RVA 0x1345a10）

```
lea  r8, ["GameActor"]; edx=1; call 0x181a81890   ; actor = checkudata(L,1,"GameActor")
xor  r8d,r8d; edx=2;          call 0x181cbc9f3     ; s = luaL_checkstring(L,2)
（构造引擎 String，SSO 栈上）
mov  rax,[rbx]; lea rdx,[&str]; mov rcx,rbx
call [rax+0xa0]                                    ; ★ actor->vtable[+0xa0](String) = SetAsset
xor  eax,eax                                       ; 无返回值
```

- **参数零解析**：wrapper 不检查 '$$' 前缀、不查表、不报错——一切交给虚函数。
- **ModelActor / EffectActor / GameUnit 同一 wrapper**（均继承 GameActor 块注册）；差异在各自 vtable+0xa0 覆写。静态无法定位 vtable（本 dll vtable 不含 COL RVA 反向引用，RTTI 链断裂），→ frida 从实例 `[obj]+0xa0` 直接读。
- ModelActor 侧 impl 候选区：0x18177cXXX~0x181784XXX（ModelActor.cpp 日志串簇：GetAnimationPath/UpdateAnimationAction/Init/SetModelComponent），其中含 §1.2 的 typeid 查表与 `"Cannot find model in ActorTable, actorName[%s]"`（VA 0x1827303b0，xref 0x181783e80 区域）。
- **裸路径失败点推断**（ModelActor）：SetAsset 把入参当 link → 哈希 → ActorTable typeid 查找 → 裸路径哈希必 miss → 日志 + 静默 return（与 G28 裸路径无崩溃仅不渲染一致）。
- `get_mesh_asset` wrapper（约 VA 0x181345f0f 起）返回 `{uuid=, type=, skeletalMesh=, staticMesh=, path=}` 表——**mesh 条目内含文件路径**，可作 link↔路径对照的调试手段。
- `set_mesh_asset_material` wrapper（VA 0x1813464b0）：内部 GetSubsystem('Material'/'BaseResourceCache'/'ResourceCache')（0x1813466ef~0x181346782）→ **材质按名/路径走 ResourceCache，不碰数编**。

## 3. ★ 特效注册表与疑似路径回退（VA 0x18129fb50，RVA 0x129fb50）

定位链：`"can't find effect : %s"`（VA 0x1826bd868）唯一 lea xref = 0x18129fd56，所在函数 0x18129fb50（lua 绑定形态：rsi=lua_State*，rdi=引擎对象）。

关键片段（0x18129fcb0~0x18129fd86）：

```
; —— 字符串哈希：h = h*0x1003F + c（32 位，SDBM 变体）
imul ebx, ebx, 0x1003f
movsx eax, al
add  ebx, eax
...
; —— 桶链容器：this+0x110 = {+4: bucketCount, +0x10: buckets[]}
mov  rdx, [rdi+0x110]
mov  ecx, [rdx+4]; dec ecx
and  rcx, rax                 ; bucket = hash & (count-1)
mov  rbx, [rdx+rcx*8+0x10]    ; 桶头节点
; —— 链走查，[rbx+0x20]=key C 字符串，strcmp(0x1822f5dda) 比对
; hit: 0x18129fd86  cmp rbx,[rdi+0x108]（end 哨兵）→ 相等 = miss
; miss → 0x18129fcfc:
call 0x180e052b0(..., line=0x13b4)          ; 日志
mov  rcx,[rdi+0xf8]; call 0x1809e5c50        ; 旧资产处理
call 0x180704920(rdi) → rcx                  ; 取 loader/cache
lea  rdx,[&path]; mov r8b,1; call 0x180777810 ; ★ 疑似按路径直载
test rax,rax; jne 成功
; 再失败 → "can't find effect : %s"（0x18129fd56，line=0x13b9）
```

- 容器形态确证：**std::unordered_map 风格桶链哈希表，key = 原始字符串（strcmp 判等），哈希 = h*0x1003F+c**。
- **存在 miss→路径加载回退**（0x180777810，r8b=1）。这函数归属待 frida 确认（无 lea/abs 引用，纯虚调用到达）：若它是 EffectActor::SetAsset，则特效裸路径理论上可直载，G30 失败需另找原因（如加载成功但 play 状态/材质未刷新）；若它是 UI particle 控件 effect 属性 setter（render-02 已实证吃 .effect 路径），则 EffectActor::SetAsset 是另一实现。两种归属都通——**这是解释 G30 矛盾的关键节点**。

## 4. 旁路资源加载（ResourceCache）

- Urho3D ResourceCache 完整内嵌（源 `Urho3D\Resource\ResourceCache.cpp`），内部 API 字符串族：`BaseResourceCache_GetResource / GetExistingResource / AddManualResource / AddPackageFile / ReloadResource / Exists / FileExist / ScanDir…`（命名是日志 tag 形态，非导出函数）。
- lua 触达评估：lua 绑定面**无 ResourceCache 直接暴露**；最近的通道 = `game.GetTexture`（render-01）与 `set_mesh_asset_material`（§2.2，内部 GetResourceCache）。即 **纹理/材质可按路径加载，模型/特效资产不能**（模型走 SkeletalArchetypeInstance/prefab 链，绑在数编条目上）。
- `'%s/model.prefab'`、`'/model.prefab'` 有 xref（0x180fabe00 / 0x181195a6e / 0x181196f8a / 0x18119792a），属编辑器/预制体加载链（EPrefab/Prefab.cpp 多在 Editor 侧），游戏态价值低。
- `AsyncLoadingStaticPrefab`、`AnimSceneController_GetPrefabPath` 等字符串存在但未发现 lua 直达口。

## 5. 机制图（本轮刷新）

```
lua: SCE.ModelActor.new(link) / UIWorld:CreateActor(link)
  └─► CreateActor impl (VA 0x1816fa290)
        └─► [this+0x330]→vfunc+0x60 → manager → vfunc+0x50(out, key{+8=link字符串})
              └─► GameDataManagerImp 表族：按 typeid(u64≈hash(link)) 查 ActorTable
                    miss → "[CreateActor:%d,%s] failed, GetActorTableEntry failed." → 返回 nil

lua: actor:set_asset(x)   ← ModelActor/EffectActor/GameUnit 同一 wrapper
  └─► wrapper (VA 0x181345a10)：luaL_checkstring → 零解析
        └─► actor->vtable[+0xa0](x)               ← 各类覆写
              ModelActor 侧: x 当 link → typeid 哈希查 ActorTable → miss 日志+静默返回（G28 现象）
              EffectActor 侧: 疑另有实现（0x18129fb50 候选：哈希桶链查字符串 → miss 回退 0x180777810 按路径直载）

注册表写入：仅 LoadMapTable（启动期，"Map table is loaded, skip it." 一次性）
  → 运行时插入 = frida 复用加载期 insert 函数（唯一可行路径）
```

## 6. frida hook 点建议（按优先级）

| # | 目标 | VA（RVA） | 目的 |
|---|---|---|---|
| 1 | CreateActor impl 入口 | 0x1816fa290（0x16fa290） | dump arg3 key 结构体（+8 link 串）；在 0x1816fa2f5 call 处读 `[rax+0x50]` 解析 GetActorTableEntry 实际地址 |
| 2 | GetActorTableEntry 实际函数（#1 解析） | 动态 | dump manager this（容器基址）、key、返回 entry；进函数看 HashMap 布局 → **找配套 insert** |
| 3 | set_asset wrapper | 0x181345a10（0x1345a10） | dump 入参串；读 `[obj→vtable]+0xa0` 分别解析 ModelActor/EffectActor/GameUnit 的 SetAsset 地址（回答 impl 是否同源） |
| 4 | 特效注册表函数 | 0x18129fb50（0x129fb50） | 确认归属（与 #3 的 EffectActor SetAsset 地址比对）；dump 容器 this、命中/miss 走向 |
| 5 | 路径回退加载器 | 0x180777810（0x777810） | 确认是否「按文件路径直载特效」；dump 参数/返回值——G30 矛盾的总钥匙 |
| 6 | LoadMapTable 加载函数 | 0x1816fd2c9 所在函数（入口需向前扫） | 启动期 dump manager this + 容器 insert 调用点 → 提取 insert 函数原型供运行时复用 |
| 7 | typeid 哈希算法确认 | 0x181782710 系（manager vfunc+0x58 调用点） | 对已知 link 算哈希比对 typeid，确认 hash(link)==typeid 即可手工伪造/预知 key |

## 7. 下一步

1. frida 按 §6 #1→#2 链解析 GetActorTableEntry，拿到容器对象与 insert——**若 insert 可在运行时安全调用（参数 = 构造的 TableEntry），则"虚拟数编"真正打通**（配合 G29 的 lua 层 merge_cache 同步给 lua 侧读）。
2. frida #3/#4/#5 一次性回答 set_asset 两侧实现差异与裸路径真实失败点。
3. 若注册表插入判死，退路 = §4 的 ResourceCache 通道（材质/纹理自由）+ UI particle 控件 .effect 直路径（render-02 已实证）+ 数编条目脚本化预生成（render-17 §1 流程）。
