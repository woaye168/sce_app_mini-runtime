# render-15 — '@gameui' 包 lua 物理位置 + require 解析机制 + UIScene 组件源

> 研究日期：2026-08-24 | 状态：✅ 完成（render-14 §5 遗留闭合）
> 方法：api_pak_version.json 定位包版本 → update 缓存找包 → decrypt_mirror 解密（只读源，输出 test/temp/gameui-48-script/）

## 1. 物理位置

```
D:\sce_online\update\editor-pd.spark.xd.com\res\_m\gameui\<版本>\gameui\ui\script\*.lua   ← 全部 TNND 加密
```

- 版本由 `D:\sce_online\update\editor-pd.spark.xd.com\api_pak_version.json` 决定（`#package_path.gameui = "Res/_m/gameui"`）：**api 12→47，api 13→48，api 2000→52**。
- **test_res002（api 13）用 gameui 48**，48 的 lua 齐全（uiscene.lua 等 62 个）。
- **镜像找不到的原因**：`.editor_src_mirror/gameui-52/` 取自 api 2000 的包，而 update 缓存中 gameui **52 只有 ui/image、没有 script 目录**（47/48 均有完整 script）。52 的 lua 去向仍悬案（embedded_packages 里也没有），api 13 链路不受影响。
- 同源副本：`sce_app_mini-runtime/runtime/Update/editor-pd.spark.xd.com/Res/_m/gameui/48/...`（payload sync 下载，同样 TNND）。
- **解密产物（明文）**：`sce_app_mini-runtime/test/temp/gameui-48-script/`（62 个 lua，含 uiscene.lua、component/init.lua）。
  - 复现命令：`sce_app_editor-patch/target/debug/examples/decrypt_mirror.exe "<update缓存>/res/_m/gameui/48/gameui/ui/script" "<输出目录>"`

## 2. require('@gameui.xxx') 解析机制（native 侧）

- 实现在 sceengine.dll / commandtool 的 C++ `PathSearcher` / `PathSearcherClient` / `LuaLoader`（`Client\src\Game\PathSearcher\PathSearcherClient.cpp`）。
- 内置前缀→脚本根表（sceengine-strings.txt 行 443836-443842 铁证）：
  ```
  script/common          ← '@common.xxx'（script 包）
  client_base/common
  gameui → gameui/ui/script  (+ gameui/ui 资源根)   ← '@gameui.xxx'
  appui  → appui/ui/script
  ```
- `require('@gameui.UIScene')` → 包根 `Res/_m/gameui/48/gameui/` + 脚本根 `ui/script/` + 点号转目录 → `ui/script/uiscene.lua`。
- 组件注册链：`@gameui.component`（component/init.lua）调 script-199 `common/base/gui/package.lua` 的 `load_component`，逐个 `require(require_url)` 后打 `package_name='gameui'`、`require_url`；`meta_info_str='@gameui/uiscene.lua 39'` 由此推导。`require(url, lib_env)` 双参数形式是引擎扩展。
- 旁证字符串：`added package[%s] with prefix[%s]`、`%s/ui/script`（commandtool-strings.txt 行 129835-129851；sceengine-strings.txt 行 457357-457375）。

## 3. uiscene.lua（gameui 48，全 78 行）摘要

- `component 'UIScene'{...}`（`__ui_type='UIScene'`）。模板 = 根 panel(644×846) + 内层撑满 panel，**`image = bind.RenderTarget`——RenderTarget 是 bind 属性，直接转发到内层 panel 的 image 属性**；native UIScene 把控件渲染目标写进该 image（native 侧链接，lua 无更多代码，与 render-13 结论一致）。
- `metadata.editable_prop.RenderPath` = 编辑器下拉：`EngineRes/RenderPaths/GameSnapshot.xml`（半透明混合）/ `CEMap.xml`（不透明）。
- EmmyLua 注解声明（method/prop/data/event/state 全空表，**实现在 native/TS 侧**）：
  - UIScene 字段：`CameraLink`、`RenderPath`、`UseShadow`、`World`(UIWorld)、`RenderTarget`；方法 `InitWorld(): UIWorld`、`DestroyWorld()`
  - UIWorld：`setup_viewport/destroy_viewport/create_scene/purge/__release/set_render_target_link(image)/set_camera_info/set_render_path`
  - 枚举：ViewpotFormat（RGBA=58/RGBA16F=66/MOBILE_HDR=75，与 render-14 §3.2 BindToUIScene 的 render_type 66/75 对应）、UIWorldRenderPath 两值同上

## 4. gameui 包组件清单（component/init.lua 注册）

**UIScene**、timershow、msgbox、msgbox_btn、progress、btn_icon、normal_btn、normal_rect、attachable_panel、active_button、simpleui_button/text/picture、Buff列表/Buff图标/Buff描述（prefab.buff.*）、transition_label、input_paste、number_input_paste，外加 sci/gf/xf 三系 btn/rect 别名（全部映射回 normal_btn/normal_rect）。

script 目录其余文件（未注册为编辑器组件）：arrow、btn、btn_icon_test、corner、icon_circle_frame、icon_num、icon_square_frame、mouse_partical、timershowclient、triangle、component/virtual_list(+horizontal)/virtual_table、prefab/{btn,rect,buff}/* 共 62 个 lua。

## 5. 影响与后续

- render-14 §3.2「RenderTarget 转发细节」闭合：组件层只是一层 bind 属性转发，裸 `base.ui.view{type='UIScene'}` 缺的是这层转发（native 属性名是 RenderTarget，但 set_control_prop 直写不进去的原因仍在 native 属性注册侧——不影响生产，走组件/页面通道即可）。
- 组件能力目录已全量可读（test/temp/gameui-48-script/），后续查 gameui 组件行为直接读明文。
- 悬案：gameui 52（api 2000）无 script 目录——2.0 项目的 gameui lua 来源待查（WasiCore 体系可能不再需要）。
