# 游戏项目 p_55a3（bgd_game_server / bgd_libs_server / 入口）

模块数：78。来源：服务端 `package.loaded` dump（loaded_module_server_package_loaded.txt）。

源码覆盖：78/78；其余标注 ⚠️ 无源码并附调用点反查/语义推测。

---

### `@p_55a3/bgd_game_server`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时根 = 构建合并入口 init）
- 状态：✅ 有源码（`.bgd\src\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/api`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\api\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/api/math_util`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\api\math_util.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Dist` | `(ax, ay, bx, by)` | 两点间欧氏距离 |
| `M.DistSq` | `(ax, ay, bx, by)` | 两点间距离平方（仅需比较远近时用，省去开方） |
| `M.Clamp` | `(v, minV, maxV)` | 把 v 钳制在 [minV, maxV] |

### `@p_55a3/bgd_game_server/common/bagconfig`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\BagConfig.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.GetItem` | `(itemId)` | 按 id 取物品配置 |
| `M.GetSize` | `(cfg, rotated)` | 取物品占格大小（rotated 时宽高互换），返回 w, h |
| `M.RollEquipStats` | `(cfg, given)` |  |
| `M.CanPlace` | `(items, cols, rows, itemId, x, y, rotated, ignoreUid)` | 检查物品能否放在 (x, y)（不含 x,y 是否已被占用的判断——见下方逐格检测） |
| `ignored` | `(uid)` | ignoreUid 支持单个 uid 或 uid 集合表 |
| `M.FindSpot` | `(items, cols, rows, itemId)` | 自动找一个能放下的位置，优先横放，其次竖放 |
| `M.TrySwap` | `(items, cols, rows, dragUid, tx, ty)` | 交换/移动模拟（客户端预览与服务端权威共用同一份，保证判定一致）。 |
| `occupy` | `(x, y, w, h)` |  |
| `M.RotateSpot` | `(items, cols, rows, item)` | 旋转落位（双端共用）：物品在 (x,y) 翻转旋转态后，若原位放不下 |
| `M.GetIconPath` | `(itemId)` | 物品图标路径（与项目内其他 UI 资源引用保持一致：src/res/... 相对项目根） |

### `@p_55a3/bgd_game_server/common/config`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\config.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/const/game_common_const_tpl`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\const\game_common_const_tpl.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/desert`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\desert.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/gameconfig`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\GameConfig.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/gameevents`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\GameEvents.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/playercolor`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\PlayerColor.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.GetPlayerColor` | `(uid)` | 根据 uid 稳定分配颜色（djb2 字符串哈希 → 取模到调色板） → string color 颜色字符串 |

### `@p_55a3/bgd_game_server/common/protocol`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\Protocol.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/sceinit`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\SceInit.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/common/shopconfig`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\ShopConfig.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.GetTab` | `(tabId)` | 取页签配置 |
| `M.GetPack` | `(packId)` | 取礼包配置 |
| `M.GetTabPacks` | `(tabId)` | 取页签下全部礼包（按配置顺序） |
| `M.CycleId` | `(tab, now)` | 当前周期序号（now 缺省取当前时间；整周期对齐 UNIX 时间，双端各自计算结果一致） |
| `M.CycleRemain` | `(tab, now)` | 当前周期剩余秒数 |
| `M.GetCurrencyIcon` | `(curType)` | 货币图标路径 |
| `M.BuyAllSummary` | `(tabId, boughtFn)` | 一键购买：汇总页签内可购买的付费礼包（跳过免费与已达限购的）， |

### `@p_55a3/bgd_game_server/common/skillconfig`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\common\SkillConfig.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.GetSkill` | `(skillId)` | 按 id 取技能配置 |
| `M.GetValue` | `(skill, level, key)` | 按等级取技能数值：基准值 + (等级-1) × 每级成长 |

### `@p_55a3/bgd_game_server/server`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/server/api`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\api\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/server/api/sync_util`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\api\sync_util.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `RoughSize` | `(v, depth)` | 粗略估算表的序列化大小（字节）。不精确，仅用于发送失败时区分「真过长」与其他原因 |
| `M.BuildPlayerList` | `()` | 构建在线玩家列表（uid/昵称/位置/颜色/血量/攻击力） |
| `M.BroadcastScoreboard` | `()` | 广播计分板（击杀/死亡变化、玩家进出场时调用） |
| `M.SendStats` | `(playerData, ensure)` | 定向下发玩家面板属性（进图 / 拾取 / 药水 / 技能后调用） |

### `@p_55a3/bgd_game_server/server/bagsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\BagSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `EmitBagChanged` | `(uid)` | 广播背包内容变化事件（属性缓存失效等；参数约定：uid, playerData） |
| `NewItem` | `(bag, itemId, x, y, rotated, count, stats, forge)` | 创建物品实例并放入背包列表 |
| `FindItem` | `(bag, itemUid)` | 在背包中查找物品实例 |
| `RemoveItem` | `(bag, itemUid)` | 移除物品实例 |
| `ConsumeStack` | `(bag, item)` | 消耗堆叠数量（count<=1 时整堆移除） |
| `AddToBag` | `(bag, itemId, count, givenStats, forge)` | 向背包添加物品：先叠满已有的同类堆叠，剩余部分自动寻位开新堆 |
| `M.InitPlayerBag` | `(playerData)` | 初始化玩家的背包数据（在玩家首次上线时调用） |
| `M.Sync` | `(uid)` | 把背包数据同步给指定客户端 |
| `M.AddItem` | `(uid, itemId, count, stats, forge)` | 给玩家发放物品（自动堆叠/寻位；stats 为装备随机属性表、forge 为锻造信息，均可为 nil） |
| `M.AddItemOrDrop` | `(uid, itemId, count, stats, forge)` | 添加物品，背包放不下的部分在玩家位置生成地面掉落（宝箱开启用） |
| `M.ConsumeItem` | `(uid, itemId, count)` | 从背包消耗指定数量的物品（用于使用药水等），成功返回 true |
| `M.SortBag` | `(uid)` | 整理背包：合并同类堆叠 -> 按占格面积从大到小重新紧凑摆放 |
| `OnReqOpenBag` | `(uid)` | 打开背包：回发全量数据 |
| `OnReqBagAdd` | `(uid)` | 随机放入一个物品（测试用；排除技能书，技能书只能靠地上刷取） |
| `OnReqBagMove` | `(uid, data)` | 移动物品：target_uid 指向其他物品时尝试合并堆叠或交换位置 |
| `OnReqBagRotate` | `(uid, data)` | 旋转物品：翻转旋转态（宽高互换），原位放不下时自动最小平移 |
| `OnReqBagDiscard` | `(uid, data)` | 丢弃物品：从背包移除，并在玩家当前位置生成地面掉落（可被其他人拾取） |
| `OnReqBagSplit` | `(uid, data)` | 拆分堆叠：分出一半到空格位 |
| `OnReqBagSort` | `(uid)` | 整理背包 |
| `OnReqBagUseItem` | `(uid, data)` | 使用物品：宝箱开启（消耗 1 个，随机获得 1~3 件物品，背包满则掉地）/ 技能书升级（对应技能 +1 级） |
| `OnReqForgeItem` | `(uid, data)` |  |

### `@p_55a3/bgd_game_server/server/bosssystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\BossSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `RandomFreePos` | `()` | 随机找一个非墙体位置 |
| `SpawnBoss` | `(stats)` | 刷新一只 BOSS |
| `DropBooksAround` | `(boss, count)` | 以 BOSS 死亡点所在地块为圆心，方形螺旋逐地块铺技能书（每地块 1 本） |
| `KillBoss` | `(uid)` | BOSS 死亡：掉落技能书（以死亡点为圆心逐地块铺书，每块地一本，散落成堆） |
| `M.DamageBossAt` | `(x, y, range, damage, attackerKey)` | 玩家攻击 BOSS（普攻 / 技能 AoE 调用）：范围内扣血或击杀，并累计仇恨 |
| `PickTarget` | `(boss)` | 选目标（MMO 规则）： |
| `BossSmoothMove` | `(boss, tx, ty, step, dtSec)` |  |
| `StartReturn` | `(boss)` | 进入脱战回巢：清仇恨/攻击计时/目标缓存，回出生点途中闪避伤害（DamageBossAt 豁免） |
| `ProcessBossAI` | `(boss, now)` | 单只 BOSS 的 AI：状态机（idle 待机 / combat 战斗 / return 回巢）+ 单体/范围攻击 |
| `M.ProcessFrame` | `()` | 帧驱动：补刷大/小 BOSS + 所有 BOSS AI |
| `M.HasBig` | `()` | 场上是否还有大 BOSS |
| `M.CountSmall` | `()` | 当前小 BOSS 数量 |

### `@p_55a3/bgd_game_server/server/buffsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\BuffSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Add` | `(playerData, buffId, durationMs, data)` | 添加/刷新一个 buff（重复添加覆盖旧的到期时间与数据） |
| `M.Get` | `(playerData, buffId)` | 取 buff（含到期自动判定：已到期返回 nil） |
| `M.Has` | `(playerData, buffId)` | 是否拥有生效中的 buff |
| `M.Remove` | `(playerData, buffId)` | 移除指定 buff |
| `M.ClearNegative` | `(playerData)` | 清除所有减益 buff（定身等；供净化/解药用，增益保留） |
| `M.ClearAll` | `(playerData)` | 清除全部 buff |
| `M.IsRooted` | `(playerData)` | 是否被定身（不能移动） |
| `M.IsGhost` | `(playerData)` | 是否穿墙状态（移动校验豁免墙体） |
| `M.HasSpeedBoost` | `(playerData)` | 是否加速状态（移动速度乘倍率） |
| `M.IsDisguised` | `(playerData)` | 是否伪装状态（免伤 / 不可被选中 / 敌人 AI 忽略） |
| `M.GetDisguiseSpeed` | `(playerData)` | 伪装期间的移速加成（未伪装返回 0） |
| `M.GetShieldAmount` | `(playerData)` | 剩余护盾值（无护盾或已到期返回 0） |
| `M.SetShieldAmount` | `(playerData, amt)` | 覆写剩余护盾值（伤害抵扣后回写；无生效护盾时静默忽略） |

### `@p_55a3/bgd_game_server/server/bushsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\BushSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.SpawnBush` | `(ownerKey, skill)` | 生成一片草丛 |
| `M.BushAt` | `(x, y)` | 返回包含该世界坐标的草丛 uid（任意动态草丛）；不在任何草丛返回 nil |
| `M.ProcessFrame` | `()` | 帧驱动：超时消失 |

### `@p_55a3/bgd_game_server/server/config`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\config.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/server/const/game_server_const_tpl`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\const\game_server_const_tpl.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/server/dropsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\DropSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `RandomFreePos` | `()` | 随机找一个非墙体位置（地图边界内） → number? y |
| `CountGroundBooks` | `()` | 统计当前地上的技能书数量 |
| `M.ProcessFrame` | `()` | 每帧驱动：技能书超时消失 + 按随机间隔补充刷取 |
| `M.SpawnDropItem` | `(x, y, itemId, count, stats, expireMs, forge)` | 在地图指定点生成一个指定物品的地面掉落（击杀掉落/丢弃物品通用） |
| `M.SpawnDrops` | `(x, y, count)` | 在地图指定点生成随机掉落（默认 1~2 件：装备为主，30% 概率掉药水） |

### `@p_55a3/bgd_game_server/server/gameserver`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\GameServer.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `OnPlayerJoin` | `(_, playerObj, is_reconnect)` | 玩家连入：首次创建数据并重发全量同步；重连复用旧数据（保留战绩/背包/属性） |
| `OnPlayerLeave` | `(_, playerObj)` | 玩家断线：广播离开 + 清理数据 + 刷新计分板与视野 |
| `HandlePlayerList` | `(uid)` | 响应客户端请求：下发在线玩家列表 + 自己的面板属性（首次进图时客户端加载完成后请求） |
| `SafeProcess` | `(name, fn)` | 帧驱动单系统保护：一个系统抛错不影响其他系统 |

### `@p_55a3/bgd_game_server/server/gmsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\GMSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `Toast` | `(playerObj, text, warn)` |  |
| `M.AddCurrency` | `(operatorPlayer, targetUid, currency, amount)` | 发放货币：targetUid 目标玩家 id，currency = 'money'\|'gem'，amount 数量（>0） |

### `@p_55a3/bgd_game_server/server/mapdata`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\MapData.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `GetGid` | `(col, row)` | 取某格 gid（0 基 col,row），越界返回 0 |
| `FloodFill` | `(col, row)` |  |
| `M.GetGrassRegion` | `(wx, wy)` | 根据世界坐标返回所在草丛区域 id（不在草丛返回 nil） |
| `M.IsInBounds` | `(x, y)` | 判断玩家矩形（左上角 x,y，边长 PLAYER_SIZE）是否在地图边界内 |
| `M.IsInWall` | `(x, y)` | 判断玩家碰撞盒是否与墙体重叠（与客户端 MapRender.IsPositionInWall 同规则） |
| `M.FindNearestFree` | `(x, y, maxRadius)` | 螺旋向外找一个离 (x,y) 最近的非墙体位置（重生点校正用） |

### `@p_55a3/bgd_game_server/server/playercombat`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\PlayerCombat.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.ApplyDamage` | `(victimUidKey, damage, killerUidKey)` | 对玩家造成伤害：扣血 + 广播血量变化 + 死亡掉落；返回命中信息 |
| `ValidateMove` | `(playerData, uidKey, x, y, now)` | 移动严格校验：边界 + 墙体（穿墙状态豁免）+ 召唤墙 + 定身 + 位移速度上限（加速/伪装状态按倍率放宽） |
| `M.HandlePlayerMove` | `(uid, data)` | 处理客户端上报的位置：严格校验后存储并广播；非法则回拉客户端到服务端权威位置 |
| `M.HandlePlayerAttack` | `(uid, data)` | 处理玩家攻击请求（攻击力 = 基础攻击 + 背包装备之和；范围伤害） |
| `M.HandleReqPickup` | `(uid, data)` | 处理拾取请求（走近装备后客户端上报，服务端做距离与状态校验） |
| `M.HandlePlayerRespawn` | `(uid, data)` | 处理重生请求：恢复血量/蓝量，校正出生点（边界+墙体），广播复活 |

### `@p_55a3/bgd_game_server/server/playermanager`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\PlayerManager.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `ComputeAttrs` | `(playerData)` | 一次遍历背包算齐四项装备属性（读取新四属性 stats 表 + 锻造加成） |
| `GetAttrs` | `(playerData)` |  |
| `M.GetPlayerAtk` | `(playerData)` | 计算玩家总攻击力：基础攻击 + 背包内全部装备的攻击力之和 |
| `GetPotionCounts` | `(bag)` | 统计背包里各类药水的数量，返回稠密数组 { {id=, count=}, ... }（按 id 升序，保证双端稳定） |
| `M.GetPlayerSpeedBonus` | `(playerData)` | 计算玩家装备移速加成：背包内全部装备（鞋子/翅膀）的移速之和（读取 stats 表） |
| `M.GetPlayerDef` | `(playerData)` | 计算玩家装备防御力加成：背包内全部防具（盾牌/铠甲/头盔等）的防御之和（读取 stats 表） |
| `M.GetPlayerMaxHp` | `(playerData)` | 计算玩家最大生命：基础生命 + 背包内全部装备的生命加成之和（衣服主属性 hp） |
| `M.RefreshMaxHp` | `(playerData)` | 刷新玩家最大生命并夹紧当前血量（装备变更/重生前调用） |
| `M.GetPlayerStats` | `(playerData)` | 汇总玩家面板属性（发给客户端展示） |
| `M.CreateDefaultPlayerData` | `(userId)` | 创建新玩家的默认数据（仅首次连入调用；重连请复用旧数据） |
| `M.RemovePlayerData` | `(uidKey)` | 清理玩家数据（断线时调用；uid 传字符串 key） |

### `@p_55a3/bgd_game_server/server/regensystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\RegenSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.HandleReqUsePotion` | `(uid, data)` | 处理使用药水 |
| `M.ProcessRegen` | `()` |  |

### `@p_55a3/bgd_game_server/server/sceinit`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\SceInit.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_game_server/server/shopsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\ShopSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.InitPlayerShop` | `(playerData)` | 初始化玩家的商店数据（在玩家首次上线时调用） |
| `GetBoughtCount` | `(playerData, pack, now)` | 查询某礼包当前周期的已购次数（记录周期与当前周期不一致时视为未购买） |
| `RecordBought` | `(playerData, pack, now)` | 记录一次购买（落在当前周期上） |
| `M.SyncWallet` | `(playerData)` | 同步钱包（金币/钻石变化时定向下发） |
| `M.Sync` | `(uid)` | 同步商店全量（打开商店 / 购买后） |
| `M.AddCurrency` | `(uid, curType, amount)` | 加货币（GM 发放等入口；curType = 'money'\|'gem'，uid 数字或字符串均可） |
| `M.AddMoney` | `(uid, amount)` | 加金币（击杀奖励等入口；uid 数字或字符串均可） |
| `GrantRewards` | `(playerData, rewards)` | 发放奖励：物品入背包（满则掉地），货币入钱包；返回奖励描述列表（Toast 用） |
| `Toast` | `(playerData, text, warn)` |  |
| `DoBuy` | `(playerData, pack, now, priceOverride)` | 执行一次购买（price 可覆盖，一键购买时传折扣价）；失败返回 false+原因，成功返回 true+奖励描述 |
| `OnReqShopBuy` | `(uid, data)` |  |
| `OnReqShopBuyAll` | `(uid, data)` | 一键购买：汇总页签内可购买的付费礼包，按折扣逐包折算，余额足够才整单成交 |

### `@p_55a3/bgd_game_server/server/skills/skillaoe`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillAoE.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillbase`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillBase.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.CastAoE` | `(uidKey, skill, pos)` | 以施法者位置为中心的范围伤害结算（火球/冰霜等 AoE 技能共用） |

### `@p_55a3/bgd_game_server/server/skills/skillblack`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillBlack.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillblink`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillBlink.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.CanCast` | `(ctx)` | 预校验：目标距离/边界/墙体，不通过则拒绝施放（SkillSystem 不会扣蓝/进 CD） |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillbreak`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillBreak.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillbush`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillBush.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillcure`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillCure.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skilldisguise`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillDisguise.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillghost`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillGhost.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillheal`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillHeal.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillpull`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillPull.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillroot`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillRoot.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillshield`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillShield.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillspeed`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillSpeed.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillsummon`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillSummon.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skills/skillwall`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\skills\SkillWall.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.Cast` | `(ctx)` |  |

### `@p_55a3/bgd_game_server/server/skillsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\SkillSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.GetPlayerSkillLevel` | `(playerData, skillId)` | 取玩家技能等级（未升级默认 1 级；最高 SkillConfig.MAX_SKILL_LEVEL） |
| `M.LevelUpSkill` | `(playerData, skillId)` | 给玩家技能加 1 级（吃技能书调用），返回是否成功（满级返回 false） |
| `M.GetEffectiveSkill` | `(playerData, skillId)` | 构建按等级放大的技能配置副本（mp/cd 不随等级成长，其余带 *_per_lvl 的数值按等级计算） |
| `M.SyncSkillLevels` | `(uidKey)` | 定向下发技能等级（进图/吃技能书后调用） |
| `M.UseSkillBook` | `(uid, playerData, skillId)` | 使用技能书（拾取/背包使用统一入口）：满级提示、升级、Toast、SyncSkillLevels 一处实现 |
| `M.HandleReqUseSkill` | `(uid, data)` | 处理使用技能 |

### `@p_55a3/bgd_game_server/server/summonsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\SummonSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.SpawnMinions` | `(ownerKey, skill, lvl)` | 召唤一批召唤怪（数量随等级提升上限） |
| `M.DamageMinionsAt` | `(x, y, range, attackerKey, damage)` | 攻击某点附近的召唤怪（玩家普攻 / 技能 AoE 调用；不能打主人及其队友的召唤怪） |
| `ScanNearestEnemy` | `(m, now)` | 找最近的敌人（主人及其队友外、未伪装、存活）；结果按 TARGET_SCAN_INTERVAL 降频缓存 |
| `M.ProcessFrame` | `()` | 帧驱动：跟随 / 追击 / 攻击 + 超时消失 |
| `M.ClearOwner` | `(ownerKey)` | 玩家断线时清掉其召唤怪 |

### `@p_55a3/bgd_game_server/server/teamsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\TeamSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `PlayerName` | `(p)` |  |
| `BuildTeamData` | `(team)` | 构建下发给客户端的队伍数据（按入队顺序） |
| `SyncTeamTo` | `(uidKey)` | 定向下发玩家当前队伍状态（不在队 => team = nil） |
| `BroadcastTeam` | `(teamId)` | 广播队伍最新状态给全体成员 |
| `M.BuildTeamMap` | `()` | 构建全员队伍状态映射（uid -> team_id，0=未组队），供客户端"靠近玩家邀请/合并"预判 |
| `M.BroadcastTeamMap` | `()` | 广播全员队伍状态映射（队伍成员构成变化 / 玩家进出后调用） |
| `RemoveFromTeam` | `(team, uidKey)` | 从队伍移除玩家（不处理队长移交/解散，调用方负责） |
| `TeamFull` | `(team)` | 队伍是否满员 |
| `DismissTeam` | `(teamId)` | 解散队伍：清空所有成员的队伍状态 |
| `M.IsSameTeam` | `(aKey, bKey)` | 两个 uidKey 是否同队（aKey == bKey 视为同队；非玩家 key 如 'boss'/nil 返回 false） |
| `M.CreateTeam` | `(uidKey)` | 创建队伍（自己当队长） |
| `M.Invite` | `(fromKey, toKey)` | 邀请（fromKey 邀请 toKey 加入自己的队伍；自己未组队则先自动创建队伍，邀请人成为队长） |
| `M.AcceptInvite` | `(toKey, fromKey)` | 接受邀请：加入邀请者的队伍 |
| `M.DeclineInvite` | `(toKey, fromKey)` | 拒绝邀请 |
| `M.MergeInvite` | `(fromKey, toKey)` | 队长发起合并（自己是队长，目标也是队长，两队不同且合并后不超上限） |
| `M.AcceptMerge` | `(toKey, fromKey)` | 同意合并：两队合并，攻击力更高的队长当新队长（平手：发起方当队长） |
| `M.DeclineMerge` | `(toKey, fromKey)` | 拒绝合并 |
| `M.LeaveTeam` | `(uidKey)` | 主动退出队伍（队长退出自动移交，只剩自己则解散） |
| `M.Kick` | `(leaderKey, targetKey)` | 队长踢人 |
| `M.Dismiss` | `(leaderKey)` | 队长解散队伍 |
| `M.HandlePlayerLeave` | `(uidKey)` | 玩家断线：自动退出队伍（队长移交/解散），并清理相关邀请 |
| `M.GetInviteablePlayers` | `(uidKey)` | 可邀请玩家列表（在线、未组队、非自己），供客户端队伍面板显示 |
| `M.HandleReqCreate` | `(uid, _)` |  |
| `M.HandleReqInvite` | `(uid, data)` |  |
| `M.HandleReqAccept` | `(uid, data)` |  |
| `M.HandleReqDecline` | `(uid, data)` |  |
| `M.HandleReqMergeInvite` | `(uid, data)` |  |
| `M.HandleReqMergeAccept` | `(uid, data)` |  |
| `M.HandleReqMergeDecline` | `(uid, data)` |  |
| `M.HandleReqLeave` | `(uid, _)` |  |
| `M.HandleReqKick` | `(uid, data)` |  |
| `M.HandleReqDismiss` | `(uid, _)` |  |
| `M.HandleReqList` | `(uid, _)` |  |

### `@p_55a3/bgd_game_server/server/visibilitysystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\VisibilitySystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `RegionOf` | `(uidKey)` | 取玩家所在草丛区域（无位置数据返回 nil） |
| `RefreshPair` | `(observer, observerRegion, targetKey, target)` | 计算 observer 能否看到 target，并在变化时推送 |
| `M.UpdateForPlayer` | `(moverKey)` | 增量重算：只刷新与移动者相关的可见性（移动者看所有人 + 所有人看移动者） |
| `M.UpdateAll` | `()` | 全量重算（玩家连入/断线时调用） |

### `@p_55a3/bgd_game_server/server/wallsystem`

- 归属：游戏项目 .bgd/src（bgd_game_server 运行时形态）
- 状态：✅ 有源码（`.bgd\src\server\WallSystem.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `ShapeOffsets` | `(shape, count)` | 生成某形状的偏移列表（数量精确到 count，多了随机剔除、少了向前补足） |
| `add` | `(x, y)` |  |
| `Rotate` | `(offs, dirX, dirY)` | 把局部偏移旋转到面朝方向 |
| `M.SpawnWallLine` | `(ownerKey, skill, dirX, dirY)` | 生成一堵墙（形状随机组合） |
| `M.IsBlocked` | `(x, y)` | 判断玩家碰撞盒（左上角 x,y）是否与任何墙体重叠（与 MapData.IsInWall 同规则，双端一致） |
| `M.DamageWallsAt` | `(x, y, range, damage)` | 攻击范围内的墙体（玩家普攻 / 技能 AoE 调用）：扣血或摧毁 |
| `M.ProcessFrame` | `()` | 帧驱动：超时消失 |
| `M.ClearOwner` | `(ownerKey)` | 玩家断线时清掉其墙体 |

### `@p_55a3/bgd_libs_server`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时根 = 构建合并入口 init）
- 状态：✅ 有源码（`.bgd\libs\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/common`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/common/api`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/common/api/class`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\class.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `class` | `(classname, super)` | 类创建接口 → any cls 类表（实际类型由调用方 @class 声明决定） |
| `cls.ctor` | `()` |  |
| `cls.new` | `(...)` |  |
| `cls.new` | `(...)` |  |
| `cls.class_name` | `()` |  |

### `@p_55a3/bgd_libs_server/common/api/co`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\co.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `coroutine_resume_with_check` | `(co, ...)` |  |
| `wrap` | `(func)` | 将异步回调转换为协程 |
| `call` | `(func, ...)` |  |
| `async` | `(fn, ...)` |  |

### `@p_55a3/bgd_libs_server/common/api/deque`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\deque.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `push_back` | `(self, elem)` |  |
| `push_front` | `(self, elem)` |  |
| `pop_back` | `(self)` |  |
| `pop_front` | `(self)` |  |
| `__len` | `(self)` |  |
| `close` | `(self, clean, recursive_clean)` |  |
| `closed` | `(self)` |  |
| `back` | `(self)` |  |
| `front` | `(self)` |  |
| `create_deque` | `()` | → deque |
| `create_queue` | `()` | → queue |

### `@p_55a3/bgd_libs_server/common/api/event_bus`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\event_bus.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `traceback` | `(msg)` |  |
| `M.on` | `(name, func)` | 订阅事件 → number id 订阅 id，用于 off |
| `M.once` | `(name, func)` | 订阅事件，触发一次后自动退订 → number id 订阅 id，用于 off |
| `M.off` | `(name, id)` | 退订 |
| `M.emit` | `(name, ...)` | 发布事件（同步派发）。监听器执行期间允许 on/off，本次派发使用快照不受影响。 |
| `M.clear` | `(name)` | 清空订阅。name 为 nil 时清空全部事件。 |
| `M.listener_count` | `(name)` | 查询事件当前订阅数（调试用） → number |

### `@p_55a3/bgd_libs_server/common/api/event_deque`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\event_deque.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `push_front` | `(self, elem)` |  |
| `push_back` | `(self, elem)` |  |
| `_pop_front` | `(self, timeout, callback)` |  |
| `_pop_back` | `(self, timeout, callback)` |  |
| `co_pop_back` | `(self, timeout)` |  |
| `co_pop_front` | `(self, timeout)` |  |
| `close` | `(self, clean)` |  |
| `closed` | `(self)` |  |
| `__len` | `(self)` |  |
| `_push_callback` | `(self, timeout, callback)` |  |
| `create_event_deque` | `()` | → event_deque |
| `create_event_queue` | `()` | → event_queue |

### `@p_55a3/bgd_libs_server/common/api/exception`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\exception.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `traceback` | `(msg, level)` |  |
| `Exception.make` | `(cls, ...)` |  |
| `Exception._make` | `(cls, trace_level, ...)` |  |
| `__Exception_index` | `(self, cls, key)` |  |
| `Exception:ctor` | `(msg)` |  |
| `mt.__index` | `(t, key)` |  |
| `Exception:set_traceback` | `(trace)` |  |
| `Exception:set_previous_exception` | `(err)` |  |
| `Exception:_to_string_to_t` | `(t)` |  |
| `Exception:to_string` | `()` |  |
| `Exception:__tostring` | `()` |  |
| `to_exception` | `(err)` | → Exception |
| `throw` | `(e)` |  |
| `_default_exception_handler` | `(err)` |  |
| `set_default_exception_handler` | `(func)` |  |
| `get_default_exception_handler` | `()` |  |

### `@p_55a3/bgd_libs_server/common/api/json`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\json.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `M.encode` | `(data)` | Json序列化 → string |
| `M.decode` | `(str)` | Json反序列化 → table |
| `M.encode_x` | `(data, depth_max)` | Json序列化:增强版 → string |
| `is_array` | `(t)` |  |
| `encode_custom` | `(value, current_depth, _key)` |  |

### `@p_55a3/bgd_libs_server/common/api/log`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\log.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `debug` | `()` |  |
| `info` | `()` |  |
| `warn` | `()` |  |
| `error` | `()` |  |
| `alert` | `()` |  |
| `is_trigger_src` | `(src)` | 判断是否为触编（可视化触发器）生成文件：触编面板靠 [文件:行号] 映射回可视化节点。 → boolean |
| `to_source_loc` | `(loc)` | 将引擎运行时路径还原为框架源码路径（构建为逐文件直拷，行号 1:1 对应源码）： → string 源码路径位置（含 .bgd 根目录） |
| `caller_location` | `()` | 取真实业务调用位置（必须从 M.xxx 方法体内直接调用）。 → string 形如 .bgd/src/client/GameScene.lua:42 或 ui/script/p_55a3/trigger_module_main_1.lua:39 |
| `pass` | `(lv, remark_str)` | 判断一条日志是否通过过滤 → boolean |
| `strip_remark` | `(args)` | 按约定剥离备注：仅在设置了关键词过滤且参数总数 >= 2 时，最后一个参数才作为备注剥离 → string\|nil remark_str 备注（已转字符串） |
| `emit` | `(lv, tag, fn, args, loc)` | 普通级别方法：过滤并输出（全部剩余参数拼接，带 [级别] [位置] 前缀） |
| `count_format_specifiers` | `(fmt)` | 统计 fmt 中的格式符个数（%% 转义不计） → number |
| `emitf` | `(lv, tag, fn, args, loc)` | 格式化级别方法（xxxf）：过滤并输出（string.format 后带 [级别] [位置] 前缀）。 |
| `M.set_level` | `(level)` | 设置最低显示级别：低于该级别的日志不输出；'none' 屏蔽全部日志 |
| `M.get_level` | `()` | 获取当前最低显示级别 → CommonLogLevel level 级别名 |
| `M.set_keyword` | `(kw)` | 设置备注关键词过滤：仅显示「备注」模糊包含任一关键词的日志。 |
| `M.get_keyword` | `()` | 获取当前备注关键词过滤（'\|' 分割的形式），未设置返回 nil → string\|nil |
| `M.debug` | `(...)` | 调试级日志（最详细，通常仅在调试构建输出）。 |
| `M.info` | `(...)` | 信息级日志（常规运行信息）。 |
| `M.warn` | `(...)` | 警告级日志（可恢复的问题）。 |
| `M.error` | `(...)` | 错误级日志（运行错误，编辑器下可能弹窗）。 |
| `M.debugf` | `(...)` | 调试级格式化日志（同引擎 log.debugf）。 |
| `M.infof` | `(...)` | 信息级格式化日志（同引擎 log.infof）。 |
| `M.warnf` | `(...)` | 警告级格式化日志（同引擎 log.warnf）。 |
| `M.errorf` | `(...)` | 错误级格式化日志（同引擎 log.errorf）。 |

### `@p_55a3/bgd_libs_server/common/api/promise`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\promise.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `get` | `(self, timeout, callback)` |  |
| `co_result` | `(self, timeout)` |  |
| `co_error` | `(self, timeout)` |  |
| `co_get` | `(self, timeout)` |  |
| `set` | `(self, value, err)` |  |
| `try_set` | `(self, value, err)` |  |
| `set_result` | `(self, v)` |  |
| `try_set_result` | `(self, v)` |  |
| `set_error` | `(self, err)` |  |
| `try_set_error` | `(self, err)` |  |
| `ready` | `(self)` |  |
| `promise:__call` | `()` | → promise |
| `get` | `(self, timeout, callback)` |  |
| `co_get` | `(self, timeout)` |  |
| `_start` | `(self, promise_list, timeout)` |  |
| `ready` | `(self)` |  |
| `multi_promise:__call` | `(promise_list, join_type, timeout)` | → multi_promise |

### `@p_55a3/bgd_libs_server/common/api/protocol`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\protocol.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `traceback` | `(msg)` |  |
| `M.init` | `(side)` | 初始化协议层（由框架入口自动调用，业务无需调用） |
| `M.on` | `(name, handler)` | 注册协议处理函数（带 xpcall 保护，handler 抛错不会中断引擎消息循环） |
| `M.on_many` | `(tbl)` | 批量注册协议处理函数 |
| `M.off` | `(name)` | 注销协议处理函数 |
| `M.send_to_server` | `(name, data)` | 客户端发送消息给服务端。【客户端】 |
| `M.broadcast` | `(name, data, ensure)` | 服务端广播消息给所有客户端。【服务端】 → boolean\|nil ok 实测成功返回 nil；仅消息过长返回 false（判断失败用 == false，勿用 if not ok） |
| `M.send` | `(player, name, data, ensure)` | 服务端发送消息给指定玩家。【服务端】 → boolean\|nil ok 实测成功返回 nil；仅消息过长返回 false（判断失败用 == false，勿用 if not ok） |
| `M.has_handler` | `(name)` | 查询协议名是否已通过本模块注册 handler（调试用） → boolean |
| `M.registered_names` | `()` | 列出全部已注册协议名（调试用） → string[] |

### `@p_55a3/bgd_libs_server/common/api/read_db`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\read_db.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `_M.create_db` | `(data)` | 暴露类构造方法给触发编辑器 → CommonReadDb |
| `normalize_value` | `(value)` | 类型转换和标准化函数 |
| `values_equal` | `(a, b)` | 值比较函数（支持类型转换） |
| `compare_values` | `(a, b, desc)` | 值比较函数（用于排序，支持类型转换） |
| `value_in_range` | `(value, min_val, max_val)` | 范围比较函数 |
| `generate_cache_key` | `(sort_fn)` | 辅助函数：生成缓存键 |
| `shallow_copy` | `(value)` | 辅助函数：浅拷贝 |
| `deep_copy` | `(value)` | 辅助函数：深拷贝 |
| `normalize_record` | `(record)` | 数据标准化处理 |
| `_M.new` | `(data_array, auto_normalize)` | 创建只读数据集 → CommonReadDb |
| `_M:_normalize_data` | `()` | 数据标准化处理 |
| `_M:_build_default_indexes` | `()` | 构建基础索引（ID必建） |
| `_M:add_index` | `(field_name, is_unique)` | 添加数据索引（增强版，支持类型转换） |
| `_M:remove_index` | `(field_name)` | 移除索引 |
| `_M:rebuild_index` | `(field_name)` | 重建索引 |
| `_M:get_index` | `(field_name, value)` | 获取索引值（增强版，支持类型转换） |
| `_M:_cleanup_cache` | `()` | 智能缓存清理 |
| `query_chain.new` | `(source)` | 查询链构造函数 |
| `query_chain:where` | `(condition_fn)` | WHERE 条件筛选 |
| `query_chain:with` | `(field, value)` | 快捷索引查询（增强版，支持类型转换） |
| `query_chain:range` | `(field, min_val, max_val)` | 范围查询支持（增强版，支持类型转换） |
| `query_chain:sort_by` | `(sort_fn)` | 排序控制 |
| `query_chain:order_by` | `(field, desc)` | 便捷排序（增强版，支持类型转换） |
| `query_chain:limit` | `(count)` | 取前N条 |
| `query_chain:select` | `(fields)` | 选择特定字段 |
| `query_chain:group_by` | `(field)` | 分组查询支持 |
| `query_chain:count` | `()` | 统计数量 |
| `query_chain:_execute_pipeline` | `()` | 执行管道（按调用顺序执行，支持类型转换） |
| `query_chain:_apply_filters_and_sort` | `()` | 保持向后兼容的内部方法 |
| `query_chain:exec` | `()` | 执行查询 |
| `_M:query` | `()` | 链式查询系统 |
| `_M:get_all` | `()` | 获取所有数据 |
| `_M:get_by_id` | `(id)` | 通过ID获取记录（增强版，支持类型转换） |
| `_M:batch_get_by_ids` | `(ids)` | 批量通过ID获取记录（增强版，支持类型转换） |
| `_M:count` | `()` | 获取数据总数 |
| `_M:has_index` | `(field_name)` | 检查索引是否存在 |
| `_M:get_index_fields` | `()` | 获取所有索引字段 |
| `_M:validate_data` | `()` | 数据验证 |
| `_M:get_stats` | `()` | 获取统计信息 |
| `_M:_count_indexes` | `()` |  |
| `_M:_count_cache` | `()` |  |
| `_M:clear_cache` | `()` | 清除所有缓存视图 |
| `_M:rebuild_indexes` | `()` | 重建所有索引 |
| `_M:renormalize_data` | `()` | 重新标准化数据 |
| `_M:set_auto_normalize` | `(enabled)` | 设置自动标准化 |

### `@p_55a3/bgd_libs_server/common/api/tosceclass`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\api\ToSceClass.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `ToSceClass` | `(M)` | 这是一个通用的补丁方法，用于兼容特殊环境的构造函数逻辑 |
| `M:____constructor` | `(...)` | 2. 注入特殊环境需要的 ____constructor |

### `@p_55a3/bgd_libs_server/common/config`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\config.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/common/const/keyboard`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\common\const\keyboard.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/server`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\server\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/server/api`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\server\api\init.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/server/config`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\server\config.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/bgd_libs_server/server/const/server_event`

- 归属：游戏项目 .bgd/libs（bgd_libs_server 运行时形态）
- 状态：✅ 有源码（`.bgd\libs\server\const\server_event.lua`）
- （未提取到顶层函数定义，可能为纯数据/副作用模块）

### `@p_55a3/main`

- 归属：游戏项目 script/（触发器/数据生成物）
- 状态：✅ 有源码（`script\main.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `base.ui.proto.__client_custom_event_struct_creater` | `(_, msg)` |  |
| `_send_custom_event_struct_creater` | `(param_name, param_struct)` |  |

### `@p_55a3/trigger_module_main_1`

- 归属：游戏项目 script/（触发器/数据生成物）
- 状态：✅ 有源码（`script\trigger_module_main_1.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `p_55a3.mcp_srv_func` | `()` |  |
| `_TRIG_F1测试_FUNC` | `(当前触发器, e)` |  |
| `p_55a3.mcp_srv_verify` | `()` |  |
| `_TRIG_Srv_Verify_Key_FUNC` | `(当前触发器, e)` |  |

### `@p_55a3/trigger_validator`

- 归属：游戏项目 script/（触发器/数据生成物）
- 状态：✅ 有源码（`script\trigger_validator.lua`）

| 函数 | 签名 | 说明 |
| --- | --- | --- |
| `validator.validator_240644303` | `(AI)` |  |
| `validator.validator_104986873` | `(AI)` |  |
| `validator.validator_243164383` | `(AI)` |  |
| `validator.validator_75033395` | `(AI)` |  |
| `validator.validator_39637044` | `(AI)` |  |
| `validator.validator_95389468` | `(AI)` |  |
| `validator.validator_9902169` | `(AI)` |  |
| `validator.validator_76233091` | `(AI)` |  |
| `validator.validator_42264203` | `(AI)` |  |
| `validator.validator_47369300` | `(AI)` |  |
| `validator.validator_179222928` | `(AI)` |  |
| `validator.validator_230146962` | `(AI)` |  |
| `validator.validator_48711270` | `(AI)` |  |
| `validator.validator_2653852` | `(AI, 发出刺激的单位, 激怒预期, 逃跑预期)` |  |
| `validator.validator_861644` | `(AI)` |  |
| `validator.validator_122531029` | `(AI)` |  |
| `validator.validator_143910259` | `(AI)` |  |
| `validator.validator_96471359` | `(AI)` |  |
| `validator.validator_160602573` | `(AI)` |  |
| `validator.validator_215516772` | `(AI)` |  |
| `validator.validator_242771` | `(AI)` |  |
| `validator.validator_230485642` | `(AI)` |  |
| `validator.validator_170334925` | `(AI)` |  |
| `validator.validator_213132334` | `(AI)` |  |
| `validator.validator_243771185` | `(AI, 发出刺激的单位, 激怒预期, 逃跑预期)` |  |
| `validator.validator_199302381` | `(AI)` |  |
| `validator.validator_93988750` | `(AI)` |  |
| `validator.validator_177931605` | `(AI)` |  |
| `validator.validator_258570479` | `(AI)` |  |
| `validator.validator_260043675` | `(AI)` |  |
| `validator.validator_95332923` | `(AI)` |  |
| `validator.validator_139726900` | `(AI)` |  |
| `validator.validator_170633122` | `(AI)` |  |
| `validator.validator_164974595` | `(AI)` |  |
| `validator.validator_27625786` | `(AI)` |  |
| `validator.validator_6213429` | `(AI)` |  |
| `validator.validator_265007906` | `(效果节点)` |  |
| `validator.validator_126068103` | `(效果节点)` |  |
| `validator.validator_33561757` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_99777765` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_23451517` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_97896931` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_248042905` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_226870051` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_21232577` | `(Buff1, Buff2)` |  |
| `validator.validator_145515012` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_157653182` | `(Buff1, Buff2)` |  |
| `validator.validator_233470208` | `(Buff1, Buff2)` |  |
| `validator.validator_180756668` | `(效果节点, 单位A, 单位B, 过滤函数Id)` |  |
| `validator.validator_255597210` | `(治疗量, 效果节点, 治疗实例, 治疗目标, 治疗来源, 是否暴击)` |  |
| `validator.validator_148805509` | `(治疗来源, 治疗目标, 治疗实例, 效果节点)` |  |
| `validator.validator_34781199` | `(伤害来源, 伤害目标, 伤害类型, 是否暴击, 伤害值, 效果节点, 伤害实例)` |  |
| `validator.validator_189928549` | `(伤害来源, 伤害目标, 伤害实例, 效果节点)` |  |
| `validator.validator_59680661` | `(伤害来源, 伤害目标, 伤害类型, 是否暴击, 伤害值, 效果节点, 伤害实例)` |  |
| `validator.validator_209125194` | `(伤害来源, 伤害目标, 伤害实例, 效果节点)` |  |
| `validator.validator_22685921` | `(伤害来源, 伤害目标, 伤害类型, 是否暴击, 伤害值, 效果节点, 伤害实例)` |  |
| `validator.validator_191493992` | `(伤害来源, 伤害目标, 伤害实例, 效果节点)` |  |
| `validator.validator_147940717` | `(伤害来源, 伤害目标, 效果节点, 伤害实例)` |  |
| `validator.validator_44065067` | `(死亡单位, 击杀者, 单位死亡类型)` |  |
| `validator.validator_166947453` | `(被分配经验的单位, 被击杀的单位, 击杀者)` |  |
| `validator.validator_182033581` | `(X轴数值, Y轴数值, 主控单位)` |  |
| `validator.validator_226425550` | `(主控单位)` |  |
| `validator.validator_233319472` | `(效果节点)` |  |
| `validator.validator_45417306` | `(效果节点)` |  |
| `validator.validator_56562507` | `(效果节点)` |  |
| `validator.validator_83540934` | `(效果节点)` |  |
| `validator.validator_12509021` | `(效果节点, 阶段)` |  |
| `validator.validator_64459937` | `(效果节点)` |  |
| `validator.validator_196209676` | `(效果节点)` |  |
| `validator.validator_91593426` | `(效果节点)` |  |
| `validator.validator_84202342` | `(效果节点, 阶段)` |  |
| `validator.validator_204988239` | `(效果节点)` |  |
| `validator.validator_208749477` | `(效果节点)` |  |
| `validator.validator_40290592` | `(效果节点)` |  |
| `validator.validator_171995275` | `(效果节点)` |  |
| `validator.validator_27087888` | `(效果节点)` |  |
| `validator.validator_187397518` | `(效果节点)` |  |
| `validator.validator_101654218` | `(效果节点)` |  |
| `validator.validator_236306566` | `(效果节点)` |  |
| `validator.validator_196723837` | `(效果节点)` |  |
| `validator.validator_219498177` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_220220686` | `(效果节点)` |  |
| `validator.validator_130739784` | `(效果节点)` |  |
| `validator.validator_154180313` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_243543281` | `(效果节点)` |  |
| `validator.validator_58691808` | `(效果节点)` |  |
| `validator.validator_76703967` | `(效果节点)` |  |
| `validator.validator_110577815` | `(效果节点)` |  |
| `validator.validator_69151185` | `(效果节点)` |  |
| `validator.validator_153290600` | `(效果节点)` |  |
| `validator.validator_149816263` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_190311381` | `(效果节点)` |  |
| `validator.validator_12576726` | `(效果节点)` |  |
| `validator.validator_142703019` | `(效果节点)` |  |
| `validator.validator_111053058` | `(效果节点)` |  |
| `validator.validator_67644446` | `(效果节点)` |  |
| `validator.validator_211391127` | `(效果节点)` |  |
| `validator.validator_63878418` | `(效果节点)` |  |
| `validator.validator_20649543` | `(效果节点, 阶段)` |  |
| `validator.validator_56366968` | `(效果节点)` |  |
| `validator.validator_108653932` | `(效果节点)` |  |
| `validator.validator_34088597` | `(效果节点)` |  |
| `validator.validator_157054375` | `(效果节点)` |  |
| `validator.validator_157541609` | `(效果节点)` |  |
| `validator.validator_67885651` | `(效果节点)` |  |
| `validator.validator_126854600` | `(效果节点)` |  |
| `validator.validator_20859394` | `(效果节点)` |  |
| `validator.validator_51649446` | `(效果节点)` |  |
| `validator.validator_236127537` | `(效果节点)` |  |
| `validator.validator_223462450` | `(效果节点)` |  |
| `validator.validator_189007863` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_19086676` | `(效果节点)` |  |
| `validator.validator_259665378` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_120343679` | `(效果节点)` |  |
| `validator.validator_134539874` | `(效果节点)` |  |
| `validator.validator_222950169` | `(效果节点)` |  |
| `validator.validator_228903203` | `(效果节点)` |  |
| `validator.validator_2540000` | `(效果节点)` |  |
| `validator.validator_45382866` | `(效果节点)` |  |
| `validator.validator_205128689` | `(效果节点)` |  |
| `validator.validator_253897080` | `(效果节点)` |  |
| `validator.validator_219325151` | `(效果节点)` |  |
| `validator.validator_3959591` | `(效果节点)` |  |
| `validator.validator_142918539` | `(效果节点)` |  |
| `validator.validator_219120192` | `(效果节点)` |  |
| `validator.validator_117687348` | `(效果节点)` |  |
| `validator.validator_155076026` | `(效果节点)` |  |
| `validator.validator_117198562` | `(效果节点)` |  |
| `validator.validator_146821332` | `(效果节点)` |  |
| `validator.validator_14977813` | `(效果节点, 阶段)` |  |
| `validator.validator_36377917` | `(效果节点, 流逝时间, 移动器)` |  |
| `validator.validator_175936324` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_45075472` | `(效果节点, 引发响应的效果节点, 原始伤害值, 当前伤害值, 伤害实例)` |  |
| `validator.validator_16625413` | `(效果节点)` |  |
| `validator.validator_182754740` | `(效果节点)` |  |
| `validator.validator_187962312` | `(效果节点)` |  |
| `validator.validator_119924838` | `(效果节点)` |  |
| `init_validator_0` | `(self)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.OnAdd` | `(...)` |  |
| `data.OnProvoke` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.OnAdd` | `(...)` |  |
| `data.OnProvoke` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.Behavior` | `(...)` |  |
| `data.OnAdd` | `(...)` |  |
| `data.OnRemove` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.Distance` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.Distance` | `(...)` |  |
| `data.Angle.LocalOffset` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.Validators` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.PeriodChangeIndex` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.Func` | `(...)` |  |
| `data.FCureRule` | `(...)` |  |
| `data.FCriticalRule` | `(...)` |  |
| `data.FDamageRule` | `(...)` |  |
| `data.FCriticalRule` | `(...)` |  |
| `data.FDamageRule` | `(...)` |  |
| `data.FCriticalRule` | `(...)` |  |
| `data.FDamageRule` | `(...)` |  |
| `data.FCriticalRule` | `(...)` |  |
| `data.FLeechRule` | `(...)` |  |
| `data.XPGrantRule.XPGrant` | `(...)` |  |
| `data.XPGrantRule.XPDistributionCheck` | `(...)` |  |
| `data.FProtoRequest` | `(...)` |  |
| `data.FProtoStopRequest` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
| `data.Formulas.ChargeMax` | `(...)` |  |
| `data.Formulas.Range` | `(...)` |  |
| `data.Formulas.Time` | `(...)` |  |
| `data.Formulas.ChargeCooldown` | `(...)` |  |
| `data.Formulas.Mana` | `(...)` |  |
| `data.Formulas.Cooldown` | `(...)` |  |
