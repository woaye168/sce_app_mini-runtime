# render-20 — ★ 动态虚拟数编（用户实现）实测终审 + native 表加载序列实证（load_map 不触发表加载）

> 研究日期：2026-08-25 | 状态：✅ PIE 实证完成（G33 三轮矩阵 + native 日志事件序列）
> 前置：render-17（G24~G30 免数编攻坚）、render-18（native 注册表静态逆向）、render-19（load_map 逆向）、G29（merge_cache lua 层成功 native 层失败）
> 用户指令（2026-08-25 06:53）：用共享的 get_eff_cache.lua / virtual_effect.lua 实现动态虚拟数编、渲染到 G17 UIWorld、落盘后继续攻克完全摆脱数编
> 探针：test_res002 `.bgd/src/client/GameWorldProbe.lua` G33 + `.bgd/src/client/virtual_effect.lua`（用户代码原样，注释前缀已清洗见 §3）

## 0. 一句话结论

1. **动态虚拟数编（merge_cache）终审判死（用户代码原样复验，三入口全部实证）**：lua 层读写完全正常（`base.eff.cache` 读回、NodeType/Effect/Model 字段正确、用户 `virtual_effect.new/set_value/get_link` API 全工作），但 native 三入口全部拒绝——`UIWorld:CreateActor(虚拟link)`=nil、`SCE.ModelActor.new(虚拟link)`=nil、`set_asset(虚拟model link)`=静默无效果（视觉对照实证）。**虚拟数编的有效域 = 仅 lua 层 `base.eff.cache` 消费者**（如逻辑层技能/buff 配置读取），渲染入口一律不通。
2. **native 表加载序列实证（12 个会话事件扫描）**：`==> Begin loading table [p_55a3]` **每游戏会话仅启动时一次**；任何运行时 `load_map`（含 UIWorld innerWorld 载场景、含 bogus 地图名）**均不触发表加载**——「伪造地图目录 + load_map 注入数编」路线判死。运行时注入 native 注册表只剩 frida 路线（render-18 §6 已备 hook 点）。
3. **新坑（高价值）**：星火引擎 lua **不接受 `|` 前缀注释行**（`|--`/`|---`，Emmy 变体风格）——`unexpected symbol near '|'`，文件整卷加载失败。用户共享文件需先清洗注释前缀（本仓已固化清洗后版本）。

## 1. G33 实测矩阵与结果（2026-08-25 18:27 会话，restart_last）

探针设计（G17 运行时页面 + G14 UIWorld 载 'default' 场景 + G16 原点吉鲁鲁对照）：

| # | 实验 | 手法 | 结果 |
| --- | --- | --- | --- |
| G33a 对照 | 真实 dl47 特效（已注册 `$$p_55a3.actor.bgd_demo_effect.root`） | `world3:CreateActor` + `play('cast')` | ✅ actor=table，紫色横幅渲染（截图 capture_1787653774） |
| G33a | **虚拟 ActorEffect**（深拷贝 root+Particle_1 全套，子节点 Asset 改**未注册**特效 psmd） | merge_cache → `CreateActor('$$p_55a3.actor.bgd_virtual_psmd.root')` | lua 读回 ✅（NodeType=ActorEffect、Effect=VCHILD 正确）；**CreateActor=nil** |
| G33b | **用户 virtual_effect.new 原样**建虚拟 ActorModel（jilulu 模板，Model 子节点虚拟化指 nazha prefab，`ve:set_value('@.Model',VCHILD)`） | `CreateActor(ve:get_link())` + `SCE.ModelActor.new(ve:get_link())` 双入口 | ve=table、link 正确、读回 NodeType=ActorModel/Model=VCHILD ✅；**双入口均 nil** |
| G33c | **虚拟 model 条目**（`virtual_effect.new('$$p_55a3.model.nezha.root','bgd_virtual_nezha2')`）喂 `set_asset`（真实 jilulu 种子 actor，置于 (0,0,150)） | `ma:set_asset('$$p_55a3.model.bgd_virtual_nezha2.root')` | 调用无报错；**视觉实证模型保持吉鲁鲁未变**（截图两蓝紫小人同形，无哪吒）→ 静默 no-op |

日志摘（lua-game log 18:28:00）：

```
G33 __MAIN_MAP__=p_55a3
G33a real effect actor=table / readback NodeType=ActorEffect Effect=$$p_55a3.actor.bgd_virtual_psmd.Particle_1 / virtual effect actor=nil
G33b ve=table link=$$p_55a3.actor.bgd_virtual_nazha.root / readback NodeType=ActorModel Model=...nazha.Model / CreateActor=nil / ModelActor.new=nil
G33c vm=table link=$$p_55a3.model.bgd_virtual_nezha2.root / ma=table / set_asset(virtual model link) done（画面不变）
```

- `__MAIN_MAP__` 客户端存在（='p_55a3'），用户代码无需改动即可在 client 运行。
- native 失败**无任何日志进 lua log**（`[CreateActor...] GetActorTableEntry failed.` 也未出现）——死亡点静默，只能靠返回值/视觉判定。
- 条目结构实证（编译产物 `script/obj/effect/actor/data.lua`）：root 的 `Effect`/`Model` 字段 = **子节点全 link**（非短 ID）；虚拟化必须整套（root + 子节点）注册并改引用。
- nezha model 编译条目 Asset = `characters/_user/p_55a3_nazha_wuwuqi_xin1_85sc_w72l/model.prefab`（on-disk 目录名形态）；jilulu actor 的 Model 子节点 Asset = `characters/_user/jilulu_19ec/model.prefab`（库内原名形态）——**同一 VFS 文件两种路径形态并存**，虚拟化填哪一种都行（前提是 native 真去读，目前读不到）。

## 2. native 表加载序列实证（editor-2026-08-25 06_40_47_048.log 全会话扫描）

12 个游戏会话的事件序列（`Begin loading table` vs `will load map`）：

```
[每会话] ==> Begin loading table [p_55a3]   ← 仅启动时一次（Game:N）
[每会话] will load map: scene/default/map.acmap  ← 主世界初始载图
[每会话] will load map: scene/default/map.acmap  ← G14 UIWorld innerWorld 载 'default' 场景（第2次）
[G31会话] Load combined scene trigger area info => failed to open file .../bgd_no_such_map/area_save.lua
[G31会话] will load map: scene/bgd_no_such_map/map.acmap  ← bogus 名也尝试载场景，但不触发表加载
```

- **表加载 = 会话级一次性**（render-18「Map table is loaded, skip it.」闸门实证）；**load_map 永不触发表加载**（12 会话零反例，含 UIWorld innerWorld、含 bogus 图）。
- 由此**判死「set_map_dir 伪造目录 + load_map 注入数编」路线**：load_map 只读场景四件（map.acmap/ClientCollision.dat/HeightData.dat/Sight.dat）+ area_save.lua，与表无关。
- 附带实证 PathSearcher 类型映射（表加载紧随的 AddPath 序列）：`type[0]=地图根 / type[1]=ui/script / type[2]=table / type[3]=ui / type[4]=data`；数编编译产物经 `SendWriteFile: p_55a3, p_55a3/script/obj/effect/<type>/data.lua` 同步进游戏（type[0] 根路径下 `script/obj/`）。
- native 注册表真实数据源高度疑似 = `script/obj/effect/*/data.lua`（与 lua cache 同源）；`table/` 目录（type[2]）只有 constant.ini/mapinfo.ini/floatingtexttemplate.json，不含 actor 条目。**精确文件清单待 frida hook LoadMapTable 读文件点确认。**

## 3. 新坑：引擎 lua 注释前缀 `|` 整卷判死

- 用户共享的 `virtual_effect.lua`/`get_eff_cache.lua` 注释风格为 `|--` 与 `|---`（Emmy 变体）。星火 lua 词法把 `|` 当运算符：文件首行 `|--` → **`xxx.lua:1: unexpected symbol near '|'`**，整卷 require 失败（G33 首轮 18:23 会话实况，错误经 on_tick error 抛出）。
- 修复：行首 `|--`→`--`、`|---`→`---`（已固化到 `.bgd/src/client/virtual_effect.lua`，共 5 行）。**后续凡接收外部 lua 文件先 grep `^\|--` / `^\|---` 清洗**。
- 悬案（无害，仅记录）：GameWorldProbe.lua 源文件首行同为 `|--`，但构建产物 `ui/script/bgd_game_client/client/GameWorldProbe.lua` 首行被剥离为 `-- GameWorld`（同管线对新建的 virtual_effect.lua 却原样保留 `|--`）——疑似构建管线对「已入库旧文件」有某种规范化处理，机制未明，不影响运行。
- restart_last 语义补充实证：**已入库文件的内容变更 restart_last 能拾取**（virtual_effect.lua 清洗后 restart 生效）；新增文件仍需 full（与既有坑一致）。

## 4. 免数编攻防图（本轮刷新）

```
已实证可用（免逐条数编）：
  主世界真实单位  game.unit_change_model(正id, prefab相对路径)   [render-06]
  纹理/材质      ResourceCache 按路径（game.GetTexture / set_mesh_asset_material） [render-18 §4]
  UI 特效        particle 控件 .effect 直路径                    [render-02]
  spine          .skel 直路径                                    [render-02]
  渲染管线       UIScene RenderPath 吃项目包内 xml               [render-17 G22]

已实证判死：
  merge_cache 虚拟数编 → native 三入口（CreateActor/*.new/set_asset）   [本文 G33，复验 G29]
  load_map / set_map_dir 注入数编（永不触发表加载）                     [本文 §2]
  裸路径 set_asset（模型/特效双侧）                                     [render-17 G28/G30]
  假 link 创建（无 native 注册即 nil）                                  [render-17 G24/G25]

唯一剩余主线：
  frida 运行时注入 native 注册表（render-18 §6 hook 点已备）
    #1 CreateActor impl 0x1816fa290 → 解析 GetActorTableEntry 实际地址
    #2 dump 注册表容器 → 找配套 insert → 运行时直调（配合 lua merge_cache 双侧同步）
    #3 set_asset wrapper 0x181345a10 → 读 vtable+0xa0 分派三类 impl
    #4/#5 特效注册表 0x18129fb50 + 路径回退加载器 0x180777810（G30 矛盾总钥匙）
  旁路：frida 直调 path 型加载器（unit_change_model apply 0x1785350 / 特效 0x180777810）
       作用到 UIWorld actor/unit 的 native 对象上——免注册表换资产
```

## 5. 下一步

1. frida 按 render-18 §6 #1→#2 解析 GetActorTableEntry 容器与 insert（编辑器 PIE 可 attach；先核对进程 Path 防挂错）。
2. hook set_asset vtable+0xa0 分派 + 0x18129fb50 归属确认 → 回答 G30 特效裸路径矛盾（加载成功但渲染未刷新？还是归属他处）。
3. 若注册表 insert 判死：退路 = ResourceCache 通道（材质/纹理）+ UI particle 直路径 + 主世界 unit_change_model + 数编脚本化预生成（render-10 流程）。
