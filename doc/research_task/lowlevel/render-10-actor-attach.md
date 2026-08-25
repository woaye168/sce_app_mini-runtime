# render-10 — ★ 官方 actor 附着通道全破解（数编脚本化 + create_actor_at + attach_to 视觉实证）

> 2026-08-24 | 承接 render-09（unit_attach_model 客户端全矩阵证伪）
> 结论：**本地模型附着到单位挂点的官方通道完整打通并视觉实证**——脚本化数编条目（entry_data.ini）→ 强制重编译 → `base.create_actor_at(数编link, point)` → `actor:attach_to(单位id, 'socket名')`。截图：吉鲁鲁模型附着在英雄头顶 socket_overhead（capture_1787523735.png）。

## 1. 一句话用法（test_res002 实证）

```lua
-- 数编条目 $$p_55a3.actor.bgd_jilulu_attach（ActorModel，Asset=jilulu prefab）已注册
local a = base.create_actor_at('$$p_55a3.actor.bgd_jilulu_attach.root', base.point(3325, 3325, 0))
a:attach_to(1, 'socket_overhead')   -- 英雄 id=1；挂点名=socketName（render-09 §1 表）
-- 换点：a:attach_to(1, 'socket_hand_r')；卸除：a:detach()（actor.lua:406）
```

- 底层 native：`game.attach_actor_to_socket(actor_id, target_id, socket)`（wrapper RVA 0x12a54b0）/ `game.attach_actor_to_anchor(actor_id, anchor)`（0x12a5480）——与死掉的 `unit_attach_model`（0x12a7f30）是**不同 API**。
- script-199 封装链：`common/base/actor.lua:329 mt:attach_to(target, socket)`（target 可为 unit 或 actor）；`base_lua_plus/actor.lua:18 base.create_actor_at(name, point)`；服务端侧 `common/base/server.lua:96/156 register_func('actor'/'unit','attach_to',...)`（服务端也可发起 attach）。
- attach_to 内有 `AttachForwardOnce` 分支：只定位不附着（用 `game.get_socket_position(target, socket or 'Socket_Root')`）；正常分支 self.hosted=true + native attach。

## 2. 数编条目脚本化（完整模板 + 生效流程）★

### 条目模板（ActorModel 内嵌 Model，仿 default_units 的「通用模型表现」与项目「主控」）

`editor/table/entry_data/actor/<条目名>/entry_data.ini`：

```lua
[#CONFIG]
'Version' = 14                       -- 与项目既有条目一致（test_res002 主控=14，default_units=13）

['Model']                            -- 内嵌 Model 子节点（link = $$<ns>.actor.<名>.Model）
'Version' = 1
'NodeType' = '$$.model.Model'
'Inherit' = '$$.template@model.Model.root'
'Data' = {
    'Editor' = {},
    'Game' = {
        'Asset' = 'characters/_user/jilulu_19ec/model.prefab',   -- ← 本地模型 prefab 相对路径
    },
}
'UIConfig' = {}

['root']                             -- 主节点（link = $$<ns>.actor.<名>.root）
'Version' = 1
'NodeType' = '$$.actor.ActorModel'
'Inherit' = '$$.template@actor.ActorModel.root'
'Data' = {
    'Editor' = { 'CollectRes' = true },
    'Game' = {
        'Name' = '显示名',
        'Model' = 'Model',           -- 引用内嵌子节点
    },
}
'UIConfig' = {}
```

- ActorAdditionModel（自动骨骼挂载版）字段模板：`bak_20260719/single_simple_ts_template_8/ui/script/plugin/obj_editor_v2/config/entry_data/template@actor/#actor#addition#model/entry_data.ini`（Model/SocketName/Offset/Rotation/FollowRotation/CreationFilter/EventCreation/EventDestruction/sync）。
- 独立 Model 条目模板：`default_units/35/.../entry_data/model/defaultmodelwithgenericfootstep/entry_data.ini`（Asset + Events 动画事件 + ActorArray）。
- link 规则：`$$<项目ns>.<类型>.<条目目录名>[.子节点]`；目录名即条目 id；补丁条目目录名可写全 link（`$$.actor.xxx`）。

### 生效流程（外部写入 → 编译进 obj）★★ 本轮踩通

1. 编辑器**关闭状态**下写入 ini（开着项目写会被保存清理逻辑当失效文件删除——陷阱见 §3）。
2. **删 `script/obj/save_info.json` 和 `ui/script/obj/save_info.json`**（关键！三文件时间戳一致时 `need_save=false`，obj 永不重生成——本轮最大卡点）。
3. 顺手把 `editor/table/save_info.json` 的 timestamp 改大（保险）。
4. 重开编辑器（`EVENT.load_map_done` 全量扫描 entry_data 目录发现新条目 + 发现时间戳不一致置 `force_complete_save_next_time`）。
5. full 调试（保存管线重生成 `script/obj` 与 `ui/script/obj`）→ 条目进 `ui/script/obj/actor/actor.lua`、`model/model.lua`、`effect/actor/{data,dict}.lua`。
6. 之后 restart_last_debug 即可正常使用（条目已在 obj 产物里）。

### 数编发现/编译机制（xdeditor obj_editor_v2 源码实证）

- **发现**：无清单/索引；`EVENT.load_map_done` → `type_config_loader.load_entry_data`（:430）对每包扫 `editor/table/entry_data/<类型>/<条目>/entry_data.ini` 两层目录。运行期无目录监听，外部新增必须重开项目。
- **编译**：`script/obj`（服务端）与 `ui/script/obj`（客户端）由**保存管线**从内存数编生成（`type_config_saver.lua`），`clear_dir_files` 会清掉不在内存集合的旧产物。
- **时间戳闸门**：`editor/table/save_info.json` vs `script/obj/save_info.json`、`ui/script/obj/save_info.json` 包级比对；一致则 `need_save=false` 跳过生成（外部写 ini 不触 editor/table 时间戳 → 不重生成——这就是首轮 create 返回 nil 的根因）。
- **陷阱**：编辑器开着项目时外部写入新条目再保存 → 保存清理逻辑（map_info.lua:654-689）把"磁盘有内存无"的 ini 当失效文件**删除**。必须先重开项目再保存。

## 3. 本轮排障时间线（方法论沉淀）

1. 写 ini → restart_last 调试 → create_actor_at=nil（obj 未重生成）。
2. 重启编辑器（条目已在磁盘）→ 还是 nil（时间戳闸门，obj 仍未重生成）。
3. 改大 editor/table 时间戳 → full 调试 → 仍然 nil（加载时三戳一致，force 标志未置；保存管线不再重判）。
4. **删两个 obj save_info.json + 改大 editor 戳 + 重开编辑器 + full 调试** → ✅ 进 obj → create_actor_at 成功 → attach_to 视觉实证。
5. 彩蛋：`[9244]` 等日志方括号数字不是进程 pid（进程查无此 pid），是线程/通道 id；PIE 游戏态进程 = exe 名为 `SCE` 的进程（Get-Process SCE；模块含 SCEEngine.dll）。

## 4. frida 旁证（render-09 死亡点收尾）

- hook attach wrapper impl 0x12a20d0 / 核心 0x176e8b0 / 内层 0x17af940（VA→RVA 修正后，三 SCE 进程全 hook 成功）→ 客户端调 `game.unit_attach_model` 时**零命中**（pcall ok=true）——该注册在客户端 VM 可能根本没绑到 `game` 表，或为死 API。叠加 U22~U25 全矩阵无渲染，**unit_attach_model 判死（至少客户端侧不可用）**，官方活通道 = actor attach（本文）。
- luaL_Reg 定位固化：`test/temp/find_luareg.py`（字符串 VA → 全二进制搜 8 字节小尾引用 → .rdata luaL_Reg 表项 +8 得函数 VA/RVA；render-06 方法论工具化）。
- `UnitAttachModel`（PascalCase）与 `unit_attach_model` 同一 wrapper（0x12a7f3 0）；`attach_actor_to_socket`=0x12a54b0、`attach_actor_to_anchor`=0x12a5480、`unit_detach_model`=0x12a7fc0、`unit_change_model`=0x12a7f90→impl 0x12a23b0。

## 5. 遗留

- `actor:attach_to` 的服务端发起 + 联机同步表现（sync 字段语义）未实测。
- ActorAdditionModel（自动骨骼挂载，socket 可空）未实建——适合"穿在身上的部件"场景（蒙皮对齐由美术资源决定）。
- attach 后动画播放（actor_play_anim 对新挂模型）与 `attach_to_anchor`（锚点=UI/血条挂接？）未试。
- 数编脚本化的 bgd 工具化（bgd_sce_tools 集成"写 ini + 删 obj save_info + bump 戳"一键流程）是框架侧自然后续。
- 线上发布：characters 资产进 pak 规则仍未验证（render-04 遗留）；数编 CollectRes=true 可能参与资源收集（待发布验证）。
