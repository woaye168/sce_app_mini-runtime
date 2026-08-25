# 渲染突破：imgui 立即模式直驱 = StateGame 渲染的正确底层通道

> 研究日期：2026-08-23 | 状态：✅ 实测实证（test_res002 PIE）
> 前置：render-01（注册块穷举）、render-02（base.ui 探针矩阵）
> 线索来源：用户提示「webview 用 base.ui.* 无法在编辑器中渲染，但用 imgui 可以」

## 0. 一句话结论

`ui.imgui_begin_view/begin_ui/props/end_ui/end_view`（每帧立即模式驱动）是**比 base.ui 声明式创建更底层、且能激活更多控件类型的通道**：webview（真加载网页）与 video（HTML5 播放器）在 StateGame（PIE 游戏局）内经 imgui 直驱**渲染成功**，而 base.ui 创建的同类型控件完全不渲染。imgui_* 不是"另一个 UI 库"，它就是 native ui.* 的立即模式入口，与 base.ui 共用控件系统。

## 1. 实测矩阵（PIE StateGame，截图存证）

| 控件 | base.ui.create | imgui 每帧直驱 | 备注 |
| --- | --- | --- | --- |
| panel/image/sprites/spine/particle | ✅ | （未试，预计 ✅） | base.ui 足够 |
| **webview** | ❌（含延迟 set url 补救） | ✅ **真渲染网页**（baidu 导航栏可见） | capture_1787500721.png |
| **video** | ❌ | ✅ **播放器 UI 渲染**（file:// 内容被 CEF 拦，0:00） | capture_1787500928/155.png |
| scene | ❌ | ❌（含 independent=true） | StateGame 内确认死亡 |

- webview 内容出现在 **游戏视口 WGC 截图**里 → webview 是**引擎内嵌渲染**（离屏 WebView2 合成进游戏 UI 纹理），不是独立 HWND overlay。
- video = webview + HTML5 `<video>`（GUIVideo，strings 452635-452640 内嵌 HTML 模板实证）；file:// 协议被拦（媒体不加载），data:/http(s) 待测；`run_js` 属性可注入 JS（play() 等）。
- base.ui webview 失败**不是创建时序问题**：T15 在创建后 60 帧/300 帧两次 set_control_prop url 仍不渲染。

## 2. imgui 直驱用法（StateGame 实测可用代码）

```lua
base.event_register(base.game, '游戏-更新', function()
    ui.imgui_begin_view('main', 'my_view')           -- root='main'（StateGame UI 根）
    if ui.imgui_begin_ui('webview', 'my_web') then   -- begin 返回 false 时本帧跳过
        ui.imgui_props(true, 'https://www.baidu.com', function(show, url)
            return { show = show, url = url,
                     layout = { width = 320, height = 200,
                                position_type = 'absolute', position = { 1560, 260 } } }
        end)
        ui.imgui_end_ui('webview', 'my_web')
    end
    ui.imgui_end_view('main', 'probe_view')
end)
```

- imgui_* 全集 12 个（sceengine-strings 443431-443452）：begin/end_view、begin/end_ui、begin/end_wrapper、props/props2、data/state/view_data/view_state。
- props 语义：位置参数 = 上次返回值回灌，最后一个参数 = 产出属性表的函数（参考 xdeditor trigger_select.lua:1286 注释掉的官方用法 `imgui.props(show, url, fn) returning {show=,url=}）。
- begin_ui 的 type 参数直接透传 native 控件类型（'webview'/'video'/'scene'/'panel'/'label'/'input' 均可）。
- ⚠️ 必须每帧驱动（立即模式：不驱动即消失）。官方封装参考 appui-50 ui/script/imgui/imgui.lua（含 rmgui 模板嵌 imgui 机制 begin_template/end_template）。
- 每帧 pcall 保护：签名错误会在每帧抛错，首次 log 后静默。

## 3. 机制推断（base.ui vs imgui 为何一生一死）

- base.ui 声明式：控件 JSON 进 wait_to_create 队列 → 次帧 add_childs_t 落引擎 → watch 逐项 set_control_prop。webview/video 这类**带外部子系统（WebView2/CEF）的控件**在该路径下 native 侧未完成激活（疑需在 imgui view 上下文里每帧 tick 驱动其生命周期/纹理上传）。
- imgui 路径每帧 begin/props/end → native 每帧刷新控件状态 → WebView2 离屏纹理得以持续合成。
- scene 控件两路都死 → 它的依赖更深（可能要游戏世界场景图/数编资产，StateGame UI 层不供电）。

## 4. 对四个用户痛点的最终回答（2026-08-23 时点）

1. **特效本地文件直用** → ✅ 已破：particle 控件 `effect='<.effect 路径>'` + `speed`(number) + `direct_scale`(table)（render-02 §1；类型错了静默不渲）。
2. **tiled 图集** → 维持既有结论：sprites 网格定格（数值类型！render-02 §2/本轮 T1 实证）/ clip 视窗 / 离线 chunk 合成；native 无 UV 通道。
3. **spine** → ✅ 完全自由：`resource='<.skel 无扩展名路径>'`（本轮 T8 实证播放 Run_Battle_00 动画）。
4. **本地模型** → ❌ UI 通道未打通（scene 控件 StateGame 死亡）；候选未试：世界内 actor（需数编 link）、GameWorld+viewport。webview html（canvas2d/WebGL）经 imgui 已成为可用的自定义 2D 渲染逃生口。
5. **视频** → 机制打通（imgui video 控件渲染播放器）；file:// 被 CEF 拦，http(s)/data: 待测；注意 bench 注释称 video 线上（tester）会崩——生产慎用，需线上验证。

## 5. 遗留探针（下轮）

- ~~imgui video 用 http(s) mp4 测试实际播放~~ → **2026-08-24 U28 实证成功**（见 §7）。
- webview run_js/web_message 双向通信实测（lua↔JS 桥）→ canvas2d 离屏渲染评估。
- scene 控件死因深挖：StateEditor 可用（编辑器内模型预览），StateGame 死（base.ui/imgui/independent 全试）——查 native 注册差异或数编依赖。
- **世界内模型（unit:change_model/attach_model）未实测**：test_res002 玩家不是引擎单位（`base.local_player():get_hero()` 在 t0/+8s/+15s 恒 nil，PIE 对局世界未真正起来——画面只剩天空盒与 HUD）。要测需在数编注册测试单位/用有单位的对局环境。候选参数形态已锁定：`game.unit_change_model(unit_id, path)` / `game.unit_attach_model(unit_id, path, hand_point, hold_point)`（script-199 unit.lua:1232-1241），path 直接传 `res/characters/_user/<名>/model.prefab`。

## 6. 追加发现（T13-T15 收尾实测）

- video 控件 file://：CEF 安全策略拦截本地文件（播放器框出、0:00 不加载）；video 线上崩溃风险见 bench 注释，生产不建议依赖。
- base.ui webview 在 StateGame 死亡与**时序无关**（T15：创建后 60/300 帧两次 set url 仍不渲染）——定性为声明式路径在 StateGame 不激活 webview native 层，imgui 每帧驱动是唯一已验证通道。
- `base.wait` 在事件回调/循环里不按预期延迟（同帧全执行），且诱发框架 timer "on_timer函数为空" 刷屏——探针时序改用 游戏-更新 帧计数（实测可靠）。
- frida Rust 绑定坑（工具固化教训）：ScriptHandler 回调线程内**实例状态不可靠**——File 句柄报 os error 6「句柄无效」、String 字段读出 NUL；跨线程状态走全局 OnceLock + 逐条 append 打开文件才稳（entrance_sniff.rs 实录）。

## 7. U28 视频实播实证（2026-08-24）

- **video 控件源属性 = `video_url`**（不是 webview 的 `url`）！出处：sceengine-strings 452631-452633（`GUIVideo` / `Key.K_SRC` / `"video_url"`）。属性名错了不报错、黑盒无请求（首轮用 `url` 时本地 http 服务零请求，改名后立即 200 拉流）。
- 内嵌 HTML 模板（strings 452635-452640）：`<video ... controls ... muted autoplay src="...">`——**自带 muted+autoplay**，http(s) 源给出即自动播，无需 run_js play()。
- 实证：imgui 每帧驱动 video 控件 + `video_url='http://127.0.0.1:8899/sample.mp4'`（本地 python http.server 供 2.8MB mp4）→ 播放器 UI + 视频画面渲染、进度条走完 0:05/0:05（截图 capture_1787543769.png；http 日志 GET 200）。
- 结论矩阵更新：**file:// 被 CEF 拦；http(s) mp4 实播可用**（编辑器 PIE StateGame）。线上 tester 可用性仍未验证（bench 注释称 video 线上会崩，生产需先验）。
- https 公网直链未测（本机网络代理因素），预计与 http 同（CEF 标准媒体加载）。
