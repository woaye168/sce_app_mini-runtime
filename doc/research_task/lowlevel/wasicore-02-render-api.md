# wasicore-02 — 2.0 渲染 API 面（Sprite2D/Canvas/粒子/模型/Spine，官方文档+API 声明实证）

> 2026-08-24 | 来源：`wasicoresdk\18\wasicoresdk\api\{client,server}\*.cs` + `docs\systems\*` + `code_sample\src\*` 示例
> 结论速查：**图集源矩形 API 存在且官方**——场景侧 `StaticSprite2D.TextureRect`，UI 侧 `Canvas.DrawImage(image, sourceRect, destRect)`；粒子/模型/Spine/RTT 全部纯代码可用。**渲染类 API 几乎全仅客户端**（服务端只有 SceneGraph/物理/导航 + Mesh/Texture 资源壳）。

## 1. Sprite2D 家族（场景内 2D 渲染）——TextureRect 确切存在

出处 `api\client\gamegraph_nodesystem_component_graphics.cs:1350-1434`：

```csharp
public class StaticSprite2D : Drawable2D, INodeComponent, IDisposable
{
    public Sprite2D? Sprite { get; set; }
    public RectangleF DrawRect { get; set; }        // UseDrawRect=true 时生效（本地绘制矩形）
    public RectangleF TextureRect { get; set; }     // UseTextureRect=true 时生效（源纹理/UV 裁剪矩形）
    public bool UseDrawRect { get; set; }
    public bool UseTextureRect { get; set; }
    public BlendMode BlendMode { get; set; }
    public bool FlipX / FlipY / SwapXY { get; set; }
    public Color Color { get; set; }                // unlit 乘色 tint
    public float Alpha { get; set; }
    public bool UseHotSpot { get; set; }            // HotSpot 归一化 pivot
    public Vector2 HotSpot { get; set; }
    public Material? CustomMaterial { get; set; }
    public void SetFlip(bool flipX, bool flipY, bool swapXY = false);
}
public class AnimatedSprite2D : StaticSprite2D { /* +AnimationSet/Animation/Entity/Skin/LoopMode/Speed/SetAnimation(name,loop)/SetSkin(name) */ }
```

资源加载（`api\client\gamegraph_resourcesystem.cs:1052-1090`）：

```csharp
public sealed class Sprite2D : Resource
{
    // 注释原文：For a project file placed at res/icon/coin.png, pass "icon/coin.png"
    public static Sprite2D? Load(string path);
}
public class Texture2D : Texture
{
    public static Texture2D? Load(GameCore.ResourceType.Texture texturePath);
    public static Texture2D? CreateRenderTarget(int width, int height, bool hdr = true, int ms = 1, bool autoResolve = true);
    public RenderSurface? RenderSurface { get; }
}
// 通用：ResourceCache.GetResource<T>(string path)
```

**tiled 图集官方解**：整张图集作一张贴图 → 每图元一个 `StaticSprite2D` 节点设 `TextureRect`；UI 层则用 `Canvas.DrawImage` 源矩形重载（§2）或 `Sprites` 控件 `SpriteSheet(...)`。场景侧路径走 `res/` 根，UI 侧走 `ui/` 根，规则不同（`docs\guides\resourcepathguide.md`）。

## 2. Canvas / CanvasAnimated（NanoVG 即时绘制，替代 canvas_texture_*）

- 文档原文："`Canvas` 是对底层 NanoVG 绘制能力的 C# 封装……业务代码应调用 WasiCore `Canvas` API，不应直接使用 `nvg*` 底层函数"。仅客户端（`#if CLIENT`），订阅 `OnRender` 驱动；`CanvasAnimated` 提供帧计时（须 `StartTiming()`）。
- 出处 `docs\systems\canvasdrawingsystem.md`、`api\client\gameui_control_primitive.cs:83(Canvas)/1847(CanvasAnimated)`。
- **图片子矩形 drawImage 四个重载**（:427-490）：

```csharp
void DrawImage(Image image, float x, float y);
void DrawImage(Image image, float x, float y, float width, float height);
void DrawImage(Image image, float sx, float sy, float sw, float sh, float x, float y, float w, float h);
void DrawImage(Image image, RectangleF sourceRect, RectangleF destRect);
// 另有 DrawTexture(Texture2D, ...) 同源矩形重载（:1793-1805），可直接画 RenderTarget
```

- 图元：线/矩形/圆角矩形/圆/椭圆/三角形/`PathF`（贝塞尔/弧）+ 纯色/线性/径向/矩形渐变 Paint + 变换栈 + `ClipRect`；文本 `DrawText/DrawTextBox/MeasureText/CreateFont`；动画图 `DrawAnimatedImage(IAnimatedImageSource, x, y)`。
- `Image` 资源 = `GameCore.ResourceType.Image`：图片放项目 `ui/` 下，代码从 `ui/` 下一级写（`ui/image/abc.png` → `new Image("image/abc.png")`），**禁止 ui/ 前缀、禁止放 user_files**。
- 性能定位：每帧 C# 驱动即时绘制；移动端有专项调优 skill（`ai\skills\canvas-mobile-performance`）；控件不 `Destroy()` 会导致移动端 FBO/纹理泄漏。
- **AnimatedImageSource**（`docs\systems\animatedimagesourcesystem.md`）：sprite sheet 帧动画数据源，构造参数 `IGameLink<GameDataAnimatedImage>`（必须数编定义），支持命名动画/图层/命中框 `GetLayerBounds("hitbox",...)`。选型对照（`spriteanimationchoiceguide.md`）：UI 装饰序列帧用 `Sprites` 控件 `.SpriteSheet(frameW, frameH, frameCount, framesPerRow)`；游戏对象用 `AnimatedImageSource + CanvasAnimated`。

## 3. 运行时粒子/特效（RuntimeParticleBuilder，免数编）

出处 `docs\ai\skills\runtime-particle-builder\skill.md`、`docs\systems\gamegraphaudioparticle.md`、示例 `code_sample\src\RuntimeParticleBuilderSample\`。仅客户端。

```csharp
// 直载本地 .effect（文件放 res/effect/...，代码从 effect/ 起写）
var effect = ParticleSystem.Load("effect/SampleSpells/WarStomp/WarStomp/particle.effect");
effect.AddToNode(node);                       // 挂载即播放
ParticleSystem.Play(node); ParticleSystem.Stop(node, killOnDeactivate: false);
ParticleSystem.SetTickSpeed(node, 2.0f); ParticleSystem.SetUnitScale(node, 0.5f);

// 纯代码构建
var particle = ParticleSystem.CreateRuntime();
var sparks = particle.AddSpriteEmitter().Timing(duration: 0.9f, loops: 1, localSpace: true);
sparks.Spawn.Burst(64); sparks.Lifetime.Range(0.35f, 0.8f);
sparks.Location.Box/Sphere/Cylinder/Cone/MeshSurface(...);
sparks.Velocity.Range(...); sparks.Size.Range(...).OverLife(...); sparks.Color.OverLife(...);
sparks.Material.Texture(ParticleTextureSlot.Diffuse, (ResourceTexture)"effect/.../uv_once_t_flare.png")
                 .Blend(ParticleBlendMode.AddAlpha).UseVertexColor();
// 四种发射器：AddSpriteEmitter() / AddMeshEmitter(modelPath) / AddBeamEmitter(src,dst) / AddRibbonEmitter()
// 专有模块：SubUV.Grid/Frame/Random、Mesh.Material/Technique、Beam.Tiling/Taper/Noise、Ribbon.Render/Tessellation
particle.ExportEffect("RuntimeParticles/MyEffect", overwrite: true);  // 仅 Debug，写入项目 res/RuntimeParticles
```

边界：仅客户端；纹理路径相对 `res/`；`ExportEffect` 仅 Debug；不参与服务端同步（服务端事件→客户端本地建）；数编驱动技能特效仍优先 `ActorParticle`/`GameDataActorParticle`。注意 `effectsystem.md` 是另一套（技能效果树 Gameplay 执行引擎）。

## 4. 模型加载 + 离屏渲染/视口（RendererViewportSample）

```csharp
public class Mesh : Resource { public static Mesh? Load(Model meshPath); }        // .mdl
public sealed class Prefab : Resource
{
    public static Prefab? Load(GameCore.ResourceType.Prefab prefabPath);
    public Node CreateInstance(Node parent);          // 必须传 parent（生命周期契约）
    public Node CreatePhysicsInstance(Node parent);
}
// 主流：var prefab = Prefab.Load("characters/General/SK_Basic2/model.prefab");
//       var inst = prefab.CreateInstance(parentNode);
// 坑：返回 PrefabInstance 根，组件在子节点 → GetComponentInChildren<AnimatedModelComponent>()
// 或 Mesh + 组件：node.CreateComponent<StaticMeshComponent>().Mesh = Mesh.Load(".../m.mdl");
```

路径规则：云端/资源库资源直接写 `characters/.../model.prefab`；项目本地未上云的放 `res/...`，代码从 `res/` 下一级写。示例资产常量见 `code_sample\src\ArtAsset\ModelAsset.cs`。

离屏渲染/多视口（`code_sample\src\RendererViewportSample\RendererViewportSample.cs`）：
- 主视口：`Renderer.SetupMainViewport(scene, camera)` / `GetMainViewport()`
- 画中画：`Renderer.GetNextFreeViewportIndex()` + `CreateViewport(scene, camera)` + `viewport.SetRect(...)` + `Renderer.SetViewport(i, vp)`；每视口可 `SetRenderPath("EngineRes/RenderPaths/CEMapSSAO.xml")`
- RTT：`Texture2D.CreateRenderTarget(w,h)` → `RenderSurface.SetViewport(0, vp)` + `UpdateMode=UpdateAlways` + `QueueUpdate()`；结果可贴 PBR 材质（`Textures[TextureUnit.PBRAlbedo]=rt`）、Panel（`.Image(rt,"名")` + `RuntimeTexture.Register`）、Canvas（`DrawTexture`）
- 程序化网格：`new MeshBuilder().SetPositions(...).SetIndices(...).SetTexCoords(...).Build()`

## 5. Spine / Spriter（AnimatedSprite2D）

出处 `docs\systems\gamegraph2danimation.md`、`api\client\gamegraph_resourcesystem.cs:37-66`、示例 `AnimatedSprite2DSample`：

```csharp
var spriter = AnimationSet2D.Load("AnimatedSprite2DSample/GoldIcon.scml");
var spine   = AnimationSet2D.Load("AnimatedSprite2DSample/spineboy.skel");  // 路径必须带扩展名（与 UI Spine 控件相反）
sprite.Entity = "entity_000";                    // Spriter 选 entity
sprite.Skin = "default"; sprite.SetSkin("default");
sprite.SetAnimation("walk", LoopMode2D.ForceLooped);
```

- 路径：优先 `ui/`（Spriter/序列帧 `ui/image/...`，Spine 三件套 `ui/spine/...`），代码不写 ui/ 前缀但**带扩展名**；fallback 到 `res/`。UI 层 Spine 控件 `Resource="Spine/hero/hero"` 不带后缀；运行时支持 Spine 4.2/4.1 及 3.8 `.skel`，旧 `.json` 不兼容。
- 边界：非 billboard（自己 LookAt）；unlit 不受光照，`Color/Alpha` 手动 tint；排序 `Layer/OrderInLayer`；多 png 的 `.atlas` 相对路径须与导出一致。3D 骨骼动画 = `AnimatedModelComponent + AnimationController`（`gamegraphskeletalanimation.md`）。

## 6. 客户端 vs 服务端对照

- **仅客户端**：全部 `gameui_*`（Canvas/Spine/Sprites/控件）、`gamesystemui_*`、`gamegraph_spinesystem`、图形组件（Camera/Light/StaticSprite2D/AnimatedSprite2D/ParticleEmitter——server 侧 `gamegraph_nodesystem_component_graphics.cs` 只剩 Frustum/Ray 两个 struct）、Sprite2D/AnimationSet2D/ParticleSystem/Prefab/SoundResource/RenderTarget 资源类、AudioManager。
- **双端**：`gamegraph_nodesystem*`（SceneGraph/Node/物理/导航/ScriptComponent——服务端可做权威 SceneGraph + 3D 物理）、SoundSource 组件（发声仅客户端）、`gamecore_*` Gameplay 系统。
- 服务端 `gamegraph_resourcesystem.cs` 只有 Mesh.Load/Shader/Texture2D.Load/ResourceCache 四个壳（配合服务端预烘焙模型碰撞）。

## 7. 对 1.0 研究线的回收

- render-07 的「Sprite2D.TextureRect 能力在 C# 层存在」→ 本文件证实它是 **2.0 官方公开 API**，lua/触发图无任何触达路径（触发图 V1/V2 节点穷举无 Sprite2D，见 wasicore-01 §1）。
- 1.0 lua 项目的图集/特效/模型答案仍是 render-02~08 既有通道；2.0 API 只在新建/迁移 2.0 项目时可用。
