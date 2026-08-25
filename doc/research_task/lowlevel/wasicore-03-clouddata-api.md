# wasicore-03 — 2.0 云数据（CloudData）API 面

> 2026-08-24 | 来源：`SDK\docs\systems\clouddatasystem.md`、`guides\clouddataquickstart.md`、`best-practices\clouddatabestpractices.md`、`ai\skills\cloud-data\skill.md`、示例 `code_sample\src\UserCloudDataTest\{README.md,UserCloudDataTestMode.cs}`（约 1600 行）
> 结论：**2.0 云数据 = 结构化多桶 KV + UUID 列表 + 唯一名称注册表 + 跨用户 ACID 事务 + 游标扫描 + 模糊名称搜索**，全面超越 1.0 云变量；但仍「按 userId+key 点查」，无任意 WHERE。仅服务端可用。传输层被 provider 完全封装（新逆向线索见 §4）。

## 1. API 面（`GameCore.UserCloudData`，双入口 `CloudDataApi` / `CloudData` 别名）

### 数据类型（6 桶）

| 类型 | 语义 |
| --- | --- |
| BigInt | 64 位整数 |
| VarChar255 | ≤255 字符串 |
| Blob | 二进制（raw byte[] / `Utf8BlobData` / `DoubleBlobData` 子类型标记） |
| Currency | 货币（不足自动 `InsufficientCurrency`） |
| CappedData | 上限 + 定时重置（`UserDataResetOption.Daily()/Weekly()/Monthly()/Never`），存已消耗量，含 Cap/LastUpdateTime/NextResetTime |
| ListItem | 全局唯一雪花 UUID 列表项 |

### 查询（批量，多 userId × 多 key）

```csharp
await CloudData.QueryUserDataAsync(long[] userIds, VarChar180[] keys);   // 通用
await CloudData.QueryPlayersDataAsync(players, keys);
await CloudData.QueryCurrencyAsync(userIds, keys);
await CloudData.QueryCappedDataAsync(userIds, keys);
await CloudData.QueryUserListItemsAsync(userId, key, maxCount);                  // 最新 N 条
await CloudData.ScanUserListItemsAsync(userId, key, batchSize, beforeItemUuid);  // 游标分批（排他游标，UUID 倒序）
await CloudData.FindListItemByIdAsync(long itemId);                              // 全局 ID 反查（跨列表）
await CloudData.CheckNameClaimedStatusAsync(collectionKey, name);                // 名称占用
await CloudData.SearchClaimedNamesAsync(collectionKey, namePart);                // 模糊搜索 ← 最接近关联查询
await CloudData.QueryUserNameBatchAsync(long[] userIds);                         // 平台用户名反查
```

### 写入 = 流式事务（TransactionBuilder）

```csharp
await CloudData.ForUser(userId)                    // 或 ForPlayer / ForUser(User)
    .SetData(key, value)                           // int/long/string/byte[]/bool 自动路由桶
    .AddToData(key, delta)
    .DeleteData(key, CloudDataType.Blob)           // 删除需显式带类型
    .AddCurrency(key, n) / .CostCurrency(key, n)
    .ModifyCappedData(key, delta, cap, UserDataResetOption.Daily())
    .ResetCappedData(key)
    .PrepareListItem(key, byte[]) → ListItemReference   // 事务前即可拿 .Id
    .PrepareUtf8ListItem(key, json) / .PrepareDoubleListItem(key, d)
    .AddListItem(itemRef) / .AddListItems(refs)
    .UpdateListItem(itemId, bytes) / .UpdateListItemUtf8 / .UpdateListItemDouble
    .DeleteListItem(itemId) / .DeleteListItems(refs)
    .MoveListItem(itemId, "warehouse")             // 跨列表移动（背包→仓库）
    .ClaimName(collection, name, desc?) / .DeleteName(collection, name)
    .WithDescription("...")                        // 审计描述
    .WithOptimization(true)                        // 默认开：同 key 合并（+100+50-20→+130）
    .WithValidation(true)
    .ExecuteAsync();                               // → UserCloudDataResult 枚举
```

### 多用户/跨用户原子提交

```csharp
await CloudData.ForUsers(u1, u2).ForAllUsers(b => b.AddCurrency("gold", 50)).ExecuteAllAsync();   // 逐用户提交，允许部分成功
await CloudData.ForUsers(buyer, seller).ForUser(...).ForUser(...).ExecuteSingleCommitAsync(desc); // 单 commit，任一失败整笔回滚（真 ACID）
await CloudDataApi.ExecuteTransactionAsync(List<TransactionOperation>, desc);                     // 手动 op 列表
```

Key 语义：远端**大小写不敏感**，≤180 字符禁空白；部分命中正常成功（不存在的 userId/key 直接缺席）；结果 `UserCloudDataResult<T>`（`if (result)` / `.IsSuccess`）。

## 2. 服务端限定

- 硬规则：CloudData **只能服务端**用（示例全在 `#if SERVER`）；客户端无 UserCloudData 缓存 API，官方路径 = 服务端读完经快照/消息/同步属性推客户端。
- 就绪时序：`await CloudData.WaitUntilReadyAsync()` / `CloudData.IsReady` / `Game.IsUserCloudDataServiceInitialized` / 事件 `Game.OnUserCloudDataServiceInitialization`。

## 3. 配额/限流（无 1.0 式分桶次数表）

官方只给行为级说明：「短时间窗口」三维限流，限额按**运行中游戏实例**共享：

| 结果码 | 触发 | 处理 |
| --- | --- | --- |
| `TooManyReadOperations` | 短时间查询过多 | 合并 key/userId、降频 |
| `TooManyWriteOperations` | 短时间事务过多 | 合并事务、禁逐字段提交 |
| `MessageSizeTooLarge` | 单次/窗口数据量过大 | 缩 payload、游标分批 |

其他结果码：`Success / QueryUserIdMissing / ServiceNotInitialized / FailedToSend / TransactionCommitEmpty / InsufficientCurrency / CapExceeded / LocalOperationFailed`。命中限额不要立即原样重试（继续占窗口）。

## 4. 底层协议线索（对接 cloudvar 系列）

- 分层：`CloudDataApi → CloudDataOperations+TransactionBuilder → CloudDataManager → IUserCloudDataProvider（引擎接口层）`；Manager 管「Provider 生命周期、请求 ID 映射、异步响应」，`AsyncWaiter` 支持 await——**请求/响应异步消息模式，与 1.0 Entrance request-id 对拍一致**。
- docs 无 ScoreArchive/Entrance/0xA000 字样；但 `docs\framework_overview.md:43` 出现 `(MessagePack)`、`api_reference.md` 有 `[MessagePackObject]`——**合理推测 2.0 云数据仍走 Entrance 通道 + MessagePack 族，op 面扩展为 query/commit/scan/claim 结构化命令**。
- 数据模型强烈暗示远端是事务型结构化存储（大小写不敏感 key、VarChar 类型名、UUID 列表、跨用户 commit 回滚）。
- **新逆向线索（下轮可做）**：2.0 游戏态抓 Entrance 帧（entrance_sniff 复用），对比 1.0 的 0xA000 ScoreArchive 是否新增 msgid/op；或逆向 GameSparkCore.dll/引擎 provider 层找 op 表。若证实同通道，entrance_client 可扩展支持 2.0 富操作（跨用户事务/列表/名称注册表直连）。

## 5. UserCloudDataTest 示例覆盖（14 阶段）

双入口一致性 / 专门查询+批量 / 部分命中 / `UserData<T>` 包装 / 批量性能计时 / 事务合并验证 / CappedData 全生命周期（Daily/Weekly/Monthly + CapExceeded）/ ListItem 全套（Prepare→Add→Find→Update→Move→Delete）/ 游标扫描（空/单批/多批）/ 名称注册（重复声明失败→删除→再声明）/ 平台用户名批量反查 / 错误处理（无效 userId、空 key 抛 ArgumentException、InsufficientCurrency）/ 并发事务一致性 / Blob 删除。**未覆盖**：排行榜、跨用户 ExecuteSingleCommitAsync 交易、手动 TransactionOperation 列表（仅 docs 有）。测试 UserId 为模拟值（100/101/102），真实环境需 `player.User.UserId > 0`。

## 6. 与 1.0 直连研究的对照结论

- 2.0 API 面本质是 1.0 云变量 KV 的全面升级（六桶、UUID 列表、名称表、ACID 事务、游标、三维限流）。
- 我们的 1.0 直连 PoC（cloudvar-05/06，entrance_client）仍是 1.0 项目的最底层答案；2.0 直连需先证实协议同通道（§4 线索）。
