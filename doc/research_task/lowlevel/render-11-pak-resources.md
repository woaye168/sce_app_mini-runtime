# render-11 — 发布 pak 资源规则（3D 资产/自定义模型的线上可达性）

> 2026-08-24 | 问题：render-06/10 的本地模型通道（change_model / actor attach）发布到线上后资产是否可达
> 结论：**项目 res/ 随 pak 发布但只含"导入存根"**；自定义模型的依赖件（m.mdl/材质/动画）不在地图 pak 内，线上依赖**平台资源库（user_libs）按原始路径下载**——PIE 能渲染是因为本机基座 Res 有全套资产。
> **2026-08-24 用户 E2E 实证：render-14 UIScene/UIWorld 通道（jilulu 本地模型）线上发布生效** → 下方假设 3 成立（原始路径引用线上可达，user_libs 分发实锤）。change_model/attach 线上虽未单独复验，资产可达性已通。

## 1. 实测证据链

| 证据 | 内容 |
| --- | --- |
| test_res002 已发布 pak 清单（`p_55a3-pak-list.txt`） | 含 `characters/_user/p_55a3_jilulu_19ec_a8oz/model.prefab`（改名存根）+ `res/anim/sk_basic2/...ani` + `map_ref_res/effect/**.mdl`（数编引用收集）；**无** jilulu 的 m.mdl/材质/动画 |
| 项目 res 目录 | `res/characters/_user/p_55a3_jilulu_19ec_a8oz/` **只有 model.prefab**；改名后内部引用仍指向**原始路径** `characters/_user/jilulu_19ec/model/m.mdl` 等（TNND 解密实证） |
| `res/project_resources.json` | 项目导入资源清单：`p_55a3_jilulu_19ec_a8oz` = {good_path: `jilulu_19ec/model.prefab/`, show_name: 吉鲁鲁}——编辑器"导入资源"= 复制改名 prefab + 登记此 json |
| 对照游戏 game_p_2xgc 的发布 pak | 同样模式：`res/characters/_user/p_2xgc_juchui.../model.prefab` 孤存根，引用原始路径 `characters/_user/juchui.../model/m.mdl` 不在包内；其 res/anim 有大量其他角色动画 |
| 本机 `D:\sce_online\Res\characters\_user\jilulu_19ec\` | 全套资产（mdl/材质/动画）存在——来源 = 平台资源库下载（PIE 渲染的真相） |

## 2. 规则推断（待 E2E 验证项标注）

1. **地图 pak 收什么**：`res/` 下已登记资源（project_resources.json 的存根 prefab）+ 数编/场景引用收集（`map_ref_res/`，含 effect 的 mdl 等依赖）+ res/anim 等已被引用收集的动画。
2. **自定义模型（characters/_user）的依赖件不进地图 pak**——两个游戏 pak 一致实证。
3. **线上可达性假设**：平台按 `user_libs` 通道分发资源库资产（mini-runtime 载荷体系里 `_m/maps/user_libs/` 落位目录与此呼应；本机 jilulu_19ec 全套 = 资源库下载而来）。若成立，则 render-06/10 用**原始路径**引用在线上也能渲染。**→ 待 E2E 验证（发布 tester + mini-runtime 线上运行时实测 jilulu 渲染）。**
4. **生产建议（当前）**：引用模型一律用**原始资源库路径**（`characters/_user/<名>/model.prefab`），不要用改名存根路径（存根内部引用仍指原始路径，且存根路径未必存在于其他用户机器）。
5. 数编 `Editor.CollectRes=true`（render-10 模板里照抄的）与"工具→资源统计和动画重定向"流程可能影响收集范围——未逐项验证。

## 3. 遗留

- ~~E2E 线上验证~~ → ✅ 2026-08-24 用户实证（render-14 UIWorld 通道，jilulu 线上渲染生效）。
- 资源统计流程对 pak 内容的影响（ CollectRes / objref.txt / full.ref 机制）。
- fbx/gltf→m.mdl 离线转换（render-04 遗留，独立课题）。
