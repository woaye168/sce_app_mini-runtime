# render-09 — attach/detach_model 攻坚：socket 全集 + 客户端全矩阵证伪 + 官方挂载路径

> 2026-08-24 | 承接 render-06 遗留「attach 挂点名称规范待查」
> 结论：**挂点命名规范已完整拿到**（prefab 内 sockets 表 boneName↔socketName）；但 **客户端 `game.unit_attach_model` 在 PIE 全参数矩阵下均无可见效果**（native 调用不报错、静默无渲染）。1.0 官方附加模型路径 = 数编 `ActorAdditionModel`（自动挂载骨骼，socket 可空）。

## 1. 挂点（socket）命名规范 —— 破解

prefab（TNND 加密，XOR CREATEEASY 解出 JSON）内含 `sockets` 数组：`{type:"socketMesh", boneName, socketName, position/rotation/scale, tag}`。**挂点名 = socketName**（2.0 官方注释实证：`GetSocketPosition("socket_weapon_r")`「挂接点名称（如 socket_weapon_r 表示右手武器挂接点，socket_root 表示根挂接点）」，`wasicoresdk\18\api\client\gamecore_actorsystem.cs:1172-1195`）。

### sk_basic2（test_res002 英雄模型）socket 表

| boneName | socketName |
| --- | --- |
| Root | socket_blood_bar / socket_root_bar / socket_laser |
| Bip001 | socket_center |
| Bip001 Head | socket_head / socket_overhead / socket_mask |
| Bip001 Spine / Spine2 | socket_waist_l / socket_waist_r / socket_chest / socket_hit / socket_back |
| Bip001 L/R Hand | socket_hand_l / socket_hand_r / socket_magic_weapon(L) / socket_gun_weapon_l(L) / socket_sword_weapon(R) |
| Bip001 L/R Foot | socket_foot_l / socket_foot_r |

### new_sk_basic2 差异

`p_weapon_1/p_weapon_2`（注意是数字不是 l/r，且 `sockket_weapon_l` 有官方拼写错误）+ `p_weapon_r/p_weapon_l` → socket_weapon_r/l；Root → `Socket_Root`（大小写不同！）。

### jilulu_19ec socket 表

p_weapon_l/r → socket_weapon_l/r；Bip001 Spine2 → socket_hit/socket_back/socket_chest/socket_wing；Head → socket_overhead/socket_head/socket_mask；Root → socket_root_bar/socket_blood_bar/socket_root；L/R Hand → socket_hand_l/r；L/R Foot → socket_foot_l/r；Bip001 → socket_center；Spine → socket_waist_l/r。另有 rootBoneName 系列（BN_QL_QB_01 等 = 布料/头发物理骨）。

**工具**：`test/temp/dec_strs.py`（TNND 解密 + 可读串提取，一次性探针，用法 `python dec_strs.py <file>...`，产出 `<file>.dec`）。

## 2. 客户端 attach 全矩阵证伪（PIE test_res002，英雄 id=1）

| 轮次 | 参数形态 | 结果 |
| --- | --- | --- |
| U22 | jilulu @ `socket_hand_r` / `Bip001 R Hand` / `p_weapon_r`（单挂点三形态） | 全部 ok=true，**无可见效果**（截图 capture_1787518399/424/448） |
| U23b | jilulu/大平面 @ `socket_hand_r`+`socket_root`（双挂点，45s 长窗口） | 同上（capture_1787519063） |
| U24 | jilulu @ 双挂点 + arg5=true | 同上（capture_1787519338） |
| U25 | **官方武器 sm_jian**（staticMesh 剑）@ `socket_hand_r`+`socket_root` / `socket_sword_weapon` 单点 | 同上（capture_1787519874/9965） |

- 宿主模型 sk_basic2 确认拥有全部所用 socket（§1），排除「挂点不存在」。
- 截图坑沉淀：**MCP capture_game（WGC 离屏恢复路径）单张耗时可达 ~30s**，9s 短窗口全部落空，45s 长窗口才稳定命中（U23 两轮废于此）。
- 服务端验证未果：新建 `SrvRenderProbe.lua` + init.lua 加 require，服务端日志（停调试刷新后）无任何探针行——`restart_last_debug` 增量疑似不拾取**新增文件**（内容变更能生效，新文件不能），或「玩家-连入」未触发。下轮改进：探针直接写在既有 init.lua 顶层并加注册时日志。

## 3. native 链补全（version-13 sceengine.dll）

- attach wrapper `0x12a20d0`：arg1=id（int）；argc≥2；单位 lookup（`0x1816fbc50`，找不到静默 return）；**要求 `[unit+0x1f0]` 组件非空**（否则跳到退出 0x1812a22ed）；arg2=path→自定义串；arg3/arg4=挂点串（可省）；arg5=bool（可省）；调核心 `0x18176e8b0(组件, path, bool, hand_point, hold_point)`。
- 核心 `0x18176e8b0`：path 哈希（0x1003f 链）查 `[组件+0xe8]` 附着表——**同 path 重复 attach 会先拆除旧的**（toggle/替换语义）；`[组件+0x80]` 非空时 `0x181771d90(组件, path)` 加载资源（失败静默退出）→ `[组件+0x28]->vtable[0x1a0]()` → 内层 `0x1817af940(node_mgr, 组件, path, 资源, bool, hand_point, hold_point)`。
- detach wrapper `0x1812a2740`：按 id 找单位 + arg2 path → `0x180751e50` 拆除。
- **下轮入口**：frida hook `0x1817af940` 确认客户端调用是否到达内层 / 参数是否正确（对照 change_model 的成功路径 0x1785350）。

## 4. 官方附加模型路径（1.0 数编，对照实证）

其他游戏 pak 镜像（`game_p_2xgc` / `game_p_1ax1` 的 `ui/script/obj/actor/actor.lua`）存在大量数编节点 **`ActorAdditionModel`（「附属模型表现节点（自动挂载骨骼）」）**：

```lua
entry_datas['$$.unit.test_inventory_user.p_2xgc_ActorAdditionModel'] = {
    ['Model'] = '$$.unit.test_inventory_user.p_2xgc_Model',  -- 数编 Model link
    ['SocketName'] = '',            -- 可空（空=自动按骨骼蒙皮对齐）
    ['Offset']/['Rotation'] = {...},
    ['EventCreation'] = 'on_cast_start',  ['EventDestruction'] = 'on_cast_stop',
    ['EventCreationModel'] = '',          ['EventDestructionModel'] = 'Death',
    ['FollowRotation'] = 1, ['TrimInsideTriangles'] = false, ['sync'] = true,
    ['NodeType'] = 'ActorAdditionModel', ['Template'] = 'ActorAdditionModel',
}
```

与 2.0 `GameDataActorAdditionModel` 同一机制（wasicore-02 引用过的 autoattachedmodelpresentationsystem.md）：**对齐由美术资源蒙皮决定（骨骼名与宿主完全一致），Socket 字段填了也不生效**；挂错资源的表现 = 显示在脚下/原点。

## 5. 当前判断与下轮假设

- 客户端 attach 调用「成功但无渲染」最可能原因（按嫌疑排序）：
  1. **attach 是服务端权威状态**，客户端调用只登记不复制（对照：change_model 客户端调用有本地应用 0x1785350 直接换模，attach 核心未见对称的本地视觉应用——内层 0x1817af940 的 node_mgr 来自组件 vtable，可能客户端镜像单位缺呈现上下文）。
  2. 被挂模型需按宿主骨架蒙皮（2.0 机制同）；但 jilulu/plane/sm_jian 至少应在脚下/原点可见——完全不可见更支持假设 1。
  3. `[unit+0x1f0]` 组件在客户端镜像单位上为 null（wrapper 静默退出）——frida hook wrapper 入口即可证（看是否走到 0x18176e8b0）。
- 下轮动作：① frida hook 0x12a20d0/0x18176e8b0/0x1817af940 定死亡点；② 服务端 attach 验证（探针写 init.lua 顶层 + 注册日志）；③ 数编 ActorAdditionModel 在 test_res002 手工建一条验证官方路径可用性（顺带固化 entry_data.ini 模板，render-04 遗留）。
