# render-07 — 官方 C# managed dll 逆向（用户线索：两个 package 的 AppBundle/managed）

> 2026-08-24 | 用户线索：`official_dotnet_bcl_package/6` 与 `official_client_deps_dll_package/23` 下的 `ui/AppBundle/managed`
> 结论：**架构地图 + 一个重要线索（Sprite2D.TextureRect = 图集源矩形能力存在于 C# 层）**；云变量在 C# 层只有缓存容器，无协议突破点。

## 0. 解密方法（TNND，逐文件 XOR）

这批 dll 全部 TNND 加密（magic `TNND` + 逐字节 XOR `CREATEEASY`，跳过 4 字节头）：

```python
key = b'CREATEEASY'
body = bytearray(data[4:])
for i in range(len(body)): body[i] ^= key[i % len(key)]  # 解出 MZ 头 .NET 程序集
```

解密件存 `test/temp/managed_dec/`（6 个 dll 全部 MZ 校验通过）。元数据用 pip 包 **dnfile** 读（TypeDef/MethodDef 表）。

## 1. 内容地图

| dll | 大小 | 内容 |
| --- | --- | --- |
| **GameCore.dll** | 1.2MB | 触发器/.NET 游戏逻辑核心：ActorSystem(37)、AbilitySystem、MovementSystem、EntitySystem、Protocol（ProtoEntity/ProtoPlayer/ProtoScene 等实体同步协议）、ResourceType（Model/Prefab/Texture/Particle=纯路径包装，Path 属性 + 隐式 string 转换）、UserCloudData |
| **GameGraph.dll** | 400KB | **C# 场景图/渲染资源层**（Urho3D 血统，UrhoInterop.EngineObject）：ResourceSystem（ResourceCache/Texture2D/MeshBuilder/Sprite2D/Prefab/ParticleSystem 全套 Builder/RenderSurface）、MaterialSystem（PBR/FlatLambert/Water/Terrain）、SpineSystem（SpineInstance/Skeleton/Bone/Slot/TrackEntry/AnimationState）、相机/灯光/Zone/DebugRenderer（AddLine/AddMesh...） |
| GameUI.dll | 450KB | UI 控件层 C# 侧（Control/Brush/DesignSystem/CameraSystem/TriggerEvent） |
| GameData.dll | 21KB | 数编全局配置壳（GameDataGlobalConfig.TestGameMode 等 15 类型） |
| TriggerEncapsulation.dll | 216KB | 触发器封装（Commands/Event/QuestSystem/UIProperty/Messaging） |
| Events.dll | 25KB | 事件定义 |
| dotnet_bcl_package/6 | — | 纯 BCL（System.* / Microsoft.*），无游戏逻辑，无线索 |

## 2. 对各研究主线的价值判定

### 云变量：无突破点
C# 侧只有 `GameCore.UserCloudData.UserData<T>`（Records/GetByKey/GetByUserId/GetByUserIds/GetByKeys/HasUserId/HasKey）+ `UserNames`（GetUserName）——**纯客户端缓存容器**，协议与存储全在 native（LuaScore.cpp→Entrance，见 cloudvar 系列）。GameCore 全 dll UTF-16 字符串无 score/archive/entrance/mysql/redis 命中。

### 渲染：一个重要线索 + 架构确认
- **Sprite2D 有 `DrawRect`/`TextureRect`/`FlipX/Y`/`UseTextureRect` 属性**（GameGraph.ResourceSystem.Sprite2D）——**图集源矩形（UV 裁剪）能力在引擎 C# 层存在**，只是 lua ui 控件属性没暴露（render-01 已确认 ui 属性无 UV）。这解答了「tiled 图集为什么只能 clip/sprites 迂回」：能力在，接口没开到 lua。可能的触达路径：触发图（节点图）系统或未来的 native ui 属性扩展。
- `MeshBuilder`/`CreateColorMaterial`/`GetShapeMeshPath`/`AddPrimitiveShape`（primitive_model_ 字符串在 GameCore）——程序化网格能力也在 C# 层。
- RenderSurface（RenderSurfaceUpdateMode）= 离屏渲染目标包装——scene 控件/视口的底层。
- SpineSystem 全套（SpineInstance 有 GetAnimationNames/GetSkinNames）——spine 播放在 C# 层有完整状态机包装。
- 这些 C# 类是否可被游戏 lua 触达：**未知，大概率只能经触发图/数编系统间接驱动**（WasmCallbackRegistry 暗示 wasm 互操作场景）。lua→C# 直调通道未发现。

## 3. 遗留线索

- 触发图（节点图）系统如果暴露 Sprite2D/TextureRect 节点，则是 tiled 图集的官方通道——下轮可查 xdeditor 节点图定义（GameGraph.NodeSystem）里哪些节点对游戏作者可用。
- GameUI.CameraSystem 有完整相机包装（Fov/Ortho/Zoom/ScreenToWorldPoint...），与 lua `game.GetCamera/set_camera`（render-04）对照可补相机参数语义。
- 本批 dll 的版本=客户端依赖包 23；编辑器内同名 dll 可能更新，路径 `D:\sce_online\update\editor-pd.spark.xd.com\res\_m\maps\official_client_deps_dll_package\`。
