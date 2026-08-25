# render-16 — scene 控件 model/particle/light 解析规则（name 命名空间破解）

> 研究日期：2026-08-24 | 状态：✅ 静态破解 + PIE 实测收尾（G18/G20：light 游戏态判死 → 通道无生产价值；模型/特效预览走 UIWorld）
> 方法：script-199 引擎自测文件 + GUI 绑定层 + 引擎字符串表 + test_res002 探针实证记录交叉印证

## 0. 一句话结论

**`model.name` 解析的是【数编单位（unit）表条目的节点名】**（不是 actor 名、不是 model 条目名、不是显示名）；**`particle.name` 解析【数编特效（particle）表条目节点名】**；`light` 在游戏态（StateGame/PIE）疑似整体不生效；`independent=true` 是 StateGame 出图前置条件。

## 1. 控件本质与属性通道

- `base.ui.scene` = native **GUIScene** 控件的 Lua 模板封装。绑定层全部走属性通道：
  - script-199 `common/base/template/scene.lua:36-80` — `update_model/camera_info/particle/buff/light` 均调 `base.ui.gui.set_*(self.id, {[k]=v})`
  - script-199 `common/base/ui/ui.lua:69-94` — `base.ui.gui.set_*` 实际是 `ui.set_control_prop(id, '<去掉set_前缀>', ...)` 的语法糖，**model/particle/buff/light/camera_info 都只是 GUIScene 普通属性，无 Lua 侧校验/转换**（解释 render-14 G12「裸 UIScene 属性全静默吞」）
  - `template/scene.lua:82` — 仅当 `__lua_state_name` 为 StateGame/StateApplication 时创建 native `'scene'` 控件，否则退化为 `'panel'`
- GUIScene 完整 native 属性键（sceengine-strings.txt:452523-452557 与 scegame-tester-strings.txt:538264-538298 完全一致）：

| 分组 | 键 |
| --- | --- |
| 场景 | `independent`, `camera_info`, `rotation`, `rotation_ue`, `rotation_qua`, `fov`, `zoom`, `orthographic` |
| model 子表 | `name`, `facing`, `pitch`, `pause`, `scale3D`, `anim`, `anim_fade_time`, `can_edit` |
| 内容 | `particle`, `buff` |
| 光照 | `light`, `directional`, `zone`, `ambient_color`, `lightgroup` |
| 捏人 | `knead_human`, `part_name`, `value`, `part_cloth`, `save`, `avatar_path`, `icon_path` |

- 坑：引擎自测 `test/particle.lua:13` 用 `scale=0.5`，但键表**没有 K_SCALE、只有 K_SCALE3D**（官方库 lib_ui_48 `drag_the_place.lua:132` 用 `scale3D=0.7`）——`scale` 很可能落到通用 2D 缩放，模型缩放写 `scale3D`。`anim` 字符串 `'Idle'` 与数组 `{'0_idle2'}` 两种形态都见于官方代码。

## 2. name 解析规则（核心）

**model.name = 数编单位表条目的节点名**（link 类型段之后的节点路径名：`$$p_55a3.unit.主控.root` → `'主控'`）。证据链：

1. 引擎自测用例（script-199 `common/test/scene.lua:10` '斧王'、`test_ac.lua:23` '剑圣'、`scenes.lua:32` '鹿目圆香'）全部是**单位条目名**；particle 用例（`test/particle.lua:16` '剑刃风暴'）是**特效条目名**。
2. GUIScene.cpp 错误串 `"Try to set unit card camera, but ui scene is nullptr."`（scegame-tester-strings.txt:538296）——model 语义是 **"unit card"（单位卡片）**，按单位预览。
3. native 按名查单位表通道：`"[CreateUnit:%d] failed, GetUnitTableEntryByName failed."`（scegame-tester-strings.txt:538689）；GameDataMgrImp 分别装载 `UnitData / ActorData / obj/model/model.lua / obj/particle/particle.lua`，name→条目映射按表独立。
4. test_res002 实证反证（GameWorldProbe G7~G10）：`'主控'`（unit 节点名）→ **有暗色画面**（解析成功、模型渲出但无光照）；`'bgd吉鲁鲁附着模型'`（actor 显示名）→ 空；`'默认动画预览模型'`（model 条目显示名）→ 空。三种写法只有单位名出画面，锁定命名空间 = 单位表。
5. 命名粒度：节点名需精确（大小写/字符）。注意 test_res002 '主控' 恰好"节点名 == 显示名"，静态无法 100% 排除"按显示名"（见 §6 不确定点 1）。

**particle.name = 数编特效表条目节点名**（如 `$$lib_control.particle.lib_control_assist_circle.root` → `'lib_control_assist_circle'`）。**buff** 推测为 buff 表条目名（无自测覆盖）。

## 3. light 生效条件（游戏态疑似不生效）

- 官方类型注解（bgd_sce_framework `template/.bgd/libs/types/class_doc_client.d.lua:39-51`）标准写法：
  `light = { directional = { direction = {-1148,-1530,1000}, color = {1,1,1.1}, shadow = false } }`
- 但 PIE 实证矩阵（GameWorldProbe G9/G10）：`independent=true + light` → 出暗画面（光照无效）；追加 `ambient_color`（table/字符串）、`lightgroup='Editor/Light/Engine/default.lightgroup'`、`camera_info` → **全部无效**。
- 推断：StateGame 下 GUIScene 光照链被渲染管线忽略（控件复用世界 zone 或 UI 通道不打光）；light 也许仅在 StateApplication（大厅/非对局）生效。**需实测**。
- **生产建议**：要正常光照的模型预览，放弃 GUIScene light，走 render-14 的 **UIWorld 通道**（世界灯光正常供光）。

## 4. knead_human（捏人）语义

使用方代码在压缩包内（defaultui_63.dec 等为 UPAK，未解）；从 native 键与错误串定语义：

- `knead_human`：GUIScene 显示捏人角色（配套 `ChangeClothSceneController` 换装场景控制器字符串）
- `part_name` + `value`：按部位名设置捏人参数值
- `part_cloth`：换装（Lua 模板层专门 watch 了 `part_cloth`/`save`，template/scene.lua:117-118）
- `save` + `avatar_path` + `icon_path`：当前捏人结果快照保存头像（配套错误串 `"Save head icon FAILED! path[%s]"`，scegame-tester-strings.txt:538298）
- 即捏人流：scene 开 `knead_human` → `part_name/value` 调形 → `part_cloth` 穿衣 → `save`+`avatar_path/icon_path` 出头像文件。
- 后续：用 mini-runtime examples/restore_game 解 UPAK 拿 defaultui_63 内 lua，可补完整时序。

## 5. 最小可用配方（test_res002）

```lua
local tpl = base.ui.scene {
    show = true,
    independent = true,          -- StateGame 出图前提（G7/G8a 实证）
    layout = { width = 300, height = 300,
               position_type = 'absolute', position = { 100, 100 } },
    model = {
        name = '主控',           -- 单位条目节点名（$$p_55a3.unit.主控.root）
        facing = 180, pitch = -70,
        scale3D = 0.7,           -- 不要写 scale（GUIScene 无 K_SCALE）
        anim = 'idle', anim_fade_time = 0.1,
    },
    particle = { name = 'lib_control_assist_circle' },  -- 特效条目节点名
    light = { directional = { direction = { -1148, -1530, 1000 },
                              color = { 1, 1, 1.1 }, shadow = false } },  -- 官方写法，游戏态实证无效
}
local ui, bind = base.ui.create(tpl, '模型预览')   -- 必须 base.ui.create（G6 实证：只建模板无画面）
```

预期：能出模型但偏暗（光照不生效的已知状态）。要光照走 UIWorld 通道（render-14 §1）。

## 6. 不确定点清单 → PIE 实测收尾（2026-08-24 G18/G19/G20）

1. **节点名 vs 显示名**：test_res002 唯一单位'主控'两者相同，仍无法区分（未造新单位，悬置；生产建议按节点名写）。
2. **light 生效条件 → 判死（游戏态）**：G20 七变体（官方 light / 过曝 color={10,10,10} / ambient_color / zoom / 正负 name 对照）**全部纯黑方块，无任何差异**——StateGame 下 GUIScene 光照链完全不工作，且黑底使 name 解析无法视觉判别。是否仅 StateApplication 生效未验（无生产价值，不追）。
3. **buff 子表**：未验（无生产价值，不追）。
4. **anim 形态**：非法动画名不报错（G18e 静默）；字符串/数组形态未区分（黑底无法判别）。
5. **scale vs scale3D**：黑底无法判别（不追）。
6. **动态 bind**：G18f 创建后写 `bind.anim/scale3D` 不报错（bind 表无可枚举键，经 metatable 工作），视觉效果黑底无法判别（不追）。
7. **knead_human 完整时序**：仍待解 UPAK。

**G18 附加发现**：scene 控件无 `color` 属性时 independent 黑底是不透明的；给 `color=rgba(...,0.15)` 时整个控件（含黑底）变半透明——看到的就是"透明无内容"假象。**结论：scene 控件 model/particle 通道在当前版本 StateGame 无生产价值（light 判死 → 内容恒黑/不可见），模型/特效预览一律走 UIWorld 通道（render-14/17）。**

**G19 附加发现**：`UIWorld:CreateActor` 直传 particle link 被拒（`UI场景不支持该表现类型：Particle`）——特效进 UIWorld 必须包数编 ActorEffect 条目（render-17 §1）。
