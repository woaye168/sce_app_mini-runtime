# 渲染底层 API 实测探针记录（test_res002 PIE，2026-08-23）

> 状态：首轮实测完成。最大成果：**.effect 特效直路径实证可用**（定位了"不显示"的真正原因）；webview/video/scene-prefab 在 PIE 不渲染（真屏验证）。
> 环境：编辑器 api13 PIE（MCP start_debug/capture_game + CopyFromScreen 真屏对照）；探针 = `.bgd/src/client/RenderProbe.lua`（勿提交 git）。

## 0. 结果总表

| 实验 | 结论 | 证据 |
| --- | --- | --- |
| panel/image/sprites | ✅ 正常（阳性对照） | 截图 capture_1787499994.png |
| **particle 控件 .effect 直路径** | ✅ **可用！绕开数据编辑器** | 同截图两条紫色横幅特效 |
| scene 控件 model.name=prefab 路径 | ❌ PIE 不渲染（三种路径形态都不行） | 真屏 screen_full2.png |
| webview 控件（baidu URL / html raw） | ❌ PIE 不渲染，`ui.check_webview_environment()` 返回 true 也没用 | 真屏对照 |
| video 控件 file:// | ❌ PIE 不渲染 | 真屏对照 |
| game.GetTexture | ⚠️ 返回 string（任何入参都返回 '浅草'），不是纹理 userdata；疑似「名字查询」语义，自定义纹理通道未打通 | 日志 |

## 1. ★ 特效直路径的正确用法（用户痛点破解）

用户反馈「特效直接用项目中的特效文件不显示」——**根因不是路径，是属性类型**：

```lua
base.ui.particle {
    layout = { width = 300, height = 300, ... },
    effect = 'libs/res/particle/demo/p_12sc_effect_new_6o1_dl47/particle.effect',  -- 带不带 .effect 扩展名都行
    play = true,
    speed = 1,                -- ❗必须 number；字符串 '1' 会导致整个特效不渲染
    direct_scale = { 1, 1 },  -- ❗必须 table；字符串 '1,1' 同罪
}
```

- 实测：`speed='1'`/`direct_scale='1,1'`（字符串）→ 控件创建成功但**完全不渲染且无报错**；换成 number/table 立即显示。
- 路径为 pak 感知资源路径（bgd 构建 `src/res/particle/...`→`res/effect/bgd_game_client/...` 改写后形态即可）。
- 同一特效文件两种引用形态（带/不带 .effect）均渲染成功。
- 待测：世界特效（非 UI 向 .effect）在 UI particle 控件的表现差异（view_mode 属性）、粒子在 2D UI 视角的朝向问题。

## 2. 各控件实测细节

- **sprites**：`image='@p_55a3/image/sprites/bgd_game_client/desert_packed.png'` 仍显示橙色占位——注意 **@前缀 = map_settings.json 的 ProjectName（p_55a3）而非项目目录名（test_res002）**；本次仍未渲出纹理，怀疑 `sprite_size='128,128'` 字符串类型同病（应 table {128,128}），下轮用数值类型复测。
- **scene**：model.name 三种形态（prefab 全路径 / 目录 / 单位 key 风格名）均不渲染。scene 控件在 PIE 可能整体不可用（bench 注释「video/scene/window 等线上会崩溃」——该注释的「线上」指 tester；PIE 实测 scene 也不出画面）。UI 内 3D 模型通道未打通，后续方向：GameWorld+viewport（小地图同款机制）或世界内 actor。
- **webview**：`check_webview_environment()=true`，baidu URL 与 html 原始串均不渲染（真屏验证，非截图通道问题）。疑 webview/video 是 app_box/大厅容器通道，游戏局内不可用。
- **video**：file:// 不渲染（PIE）。结合 bench 注释，video 控件不建议投入。
- **game.GetTexture**：任何字符串入参都返回 string '浅草'——行为像「按路径查纹理名，未命中给默认名」。LUA_METATABLE_TEXTURE 的 set_data userdata 从哪来仍未找到（可能需要先拿到真正的 texture userdata，候选：材质系统/单位纹理）。

## 3. 坑沉淀（本轮新增）

1. **particle/sprites 等控件的数值属性对类型敏感**：字符串数字/逗号串会让控件静默不渲染（无日志）。控件属性一律用 number/table 原生类型。
2. **`base.wait` 在事件回调 for 循环里不真正按序延迟**（时间戳全同帧），且会引发框架 timer 的 "on_timer函数为空" 刷屏错误——探针时序分隔要改用别的方式（或接受同帧）。
3. **PIE 客户端 native 日志不落本地**（D:\sce_online\logs\game 无新文件）——native 层资源加载失败在 PIE 没有日志出口，只能靠截图目视 + 变体矩阵。
4. **WGC 截图看不到 webview 覆盖层**属预期，但本轮真屏 CopyFromScreen 也看不到 → 真不渲染。判别法：capture_game（游戏视口裁剪）+ CopyFromScreen（真屏）双通道对照。
5. base.ui.create(tpl, '组名') 同帧批量创建多个根控件正常（此前「只有第一个显示」的猜测被证伪——R0 先显示只是因为它在屏幕左上而其它都失败）。

## 4. 下轮探针清单

1. sprites 定格用全数值类型复测（sprite_size={128,128} row_frame_count=8 frame_count=1 start_frame=2 end_frame=2 interval=100000）。
2. scene 控件 + camera_info 显式相机（init_position/default_rotation 等，格式见 tile_editor camera_attribute_panel.lua）。
3. 世界内模型：`game.create_actor` / EffectActor.set_asset 传 prefab 路径（需要数编 link 对照组——test_res002 无单位数编，考虑在数编里注册一个测试单位）。
4. game.GetTexture 语义确认（找 '浅草' 来源：全局纹理表？默认纹理名？）+ LUA_METATABLE_TEXTURE userdata 获取路径。
5. GameWorld+viewport 小地图链路复刻。
