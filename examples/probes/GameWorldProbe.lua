-- GameWorld+viewport / scene 控件渲染矩阵探针（研究任务探针；2026-08-25 自 test_res002 .bgd/src/client 转移固化）
-- 依据：defaultui_63 uiworldscript.lua（UIWorld 官方封装）、render-03（scene 控件 StateGame 死亡旧结论）
-- 矩阵：
--   G1 imgui scene + resource=actor 数编 link + independent（无 UIWorld 直驱复活试验）
--   G2 defaultui.UIWorld 官方路径（GameWorld + set_render_target_link + 控件 resource='RT:..'）
--   G3 image 控件直接吃 RT 链接（RT 作为纹理源试验）
local M = {}
local function mark(s)
    log.info('[GWProbe] ' .. s)
end

local function try(tag, fn)
    local ok, err = pcall(fn)
    mark(tag .. (ok and ' ok' or (' FAIL: ' .. tostring(err))))
end

local CAM = '$$.camera_property.camerapro.root'
local ACTOR = '$$p_55a3.actor.bgd_jilulu_attach.root'
local UNIT = '$$p_55a3.unit.主控.root' -- 英雄单位（内嵌 Model，可独立渲染；addition 表现需宿主不渲染）
local PREFAB = 'characters/_user/p_55a3_jilulu_19ec_a8oz/model.prefab'

-- 崩溃隔离开关（2026-08-24 首轮整矩阵崩 → 分组隔离）
local ENABLE_G1 = false  -- scene 控件（G1-only 实证：存活但不渲染）
local ENABLE_G2 = false  -- UIWorld GameWorld + RT（G2-only 实证：存活，世界渲到 RT）
local ENABLE_G3 = false  -- image 吃 RT（崩溃源，禁用）
local G2_RT_TO_G1 = false -- G4 实证：scene 吃 RT 三变体均不渲染
local ENABLE_G5 = false  -- G5 运行时组件只建出 panel（无 native UIScene），由 G11 页面流取代
local ENABLE_G6 = false  -- base.ui.scene 只建模板未 base.ui.create（G6 无画面根因），由 G7 取代
local ENABLE_G7 = false  -- G7 无 independent 不渲染（实证）；G8a 证明 independent=true 出画面但缺光
local ENABLE_G8 = false  -- G8 完成：independent=true 是关键；srclocate 待修（UIScene 是 class table）
local ENABLE_G9 = false  -- G9 实证：independent 出暗画面（光照无效）；cls 有 __ui_type/prop/metadata 待 dump
local ENABLE_G10 = false -- G10 实证：native 类型='UIScene'（非 scene）；ambient/lightgroup/camera 全无效
local ENABLE_G11 = false -- G11 文件页面流已被编辑器保存管线清除（页面文件被删）→ G17 运行时构造取代
local ENABLE_G12 = false -- G12 实证：裸 UIScene 属性全静默吞（set_control_prop 无校验），画面无变化
local ENABLE_G13 = false -- G13 完成：组件2 part，RenderTarget 写到了实例（'RT:table:...'），根 panel
local ENABLE_G14 = true  -- G14 ★实证：Create(false,CAM,'default') 载图世界 + BindToUIScene = 地图画面进 UI（截图 capture_1787566438）
local ENABLE_G15 = false -- G15：actor 放到 (3325,3325) 出画（相机焦点在原点附近，init_position 为空）
local ENABLE_G16 = true  -- G16: 相机对焦原点（距离300）+ 吉鲁鲁放 (0,0,0)（PIE 已实证，截图 capture_1787566974）
local ENABLE_G17 = true  -- ★ G17: 纯运行时构造页面（component+page_template 内存建类，免疫编辑器清理管线）
local ENABLE_G18 = false -- ★ G18 实证：无 light 时 scene 透明无内容（a-e）/纯黑（f 无 color）；particle 也不可见
local ENABLE_G19 = false -- ★ G19 实证：CreateActor 拒绝 particle link（defaultui GetActorFactory 报错"UI场景不支持该表现类型：Particle"）
local ENABLE_G20 = false -- ★ G20 实证：7 变体全黑（正负对照无差异）→ light 游戏态彻底无效，scene model 通道判死（生产走 UIWorld）
local ENABLE_G21 = false -- ★ G21 ★实证：数编 ActorEffect 条目进 UIWorld 成功（紫色横幅特效渲染在吉鲁鲁处，截图 capture_1787573079）
local ENABLE_G22 = false -- ★ G22 ★实证：项目包内自定义 renderpath（res/renderpaths/bgd_snapshot_red.xml）红底渲染成功；CEMap 对照正常（截图 capture_1787572683）
local ENABLE_G23 = false -- ★ G23 ★实证：SCE 目录 dump（ModelActor/EffectActor.set_asset、GameWorld.load_map/set_map_dir/use_light_group，详见 render-17 §3）
local ENABLE_G24 = false -- ★ G24 实证：ModelActor.new() 无参=nil（免数编创建需 link 种子）；手建 actor+add_game_actor 不渲染（疑缺 show）；use_light_group 硬崩（炸整个编辑器）；set_map_dir/load_map 无报错
local ENABLE_G25 = false -- ★ G25 实证：假 link 种子三种全 nil（免数编创建判死：link 必须数编已注册）；show(true) 不是手建 actor 缺失步骤
local ENABLE_G26 = false -- ★ G26 无效实验：手建 actor 放 (0,0,120) 与 G16 原点吉鲁鲁投影重叠，无法判别（设计失误）
local ENABLE_G27 = false -- ★ G27 ★实证：手建 SCE.ModelActor+add_game_actor 在 (-150,0,0) 渲染成功（双吉鲁鲁，截图 capture_1787575007）——G24/25 不可见=出画；手建通道活
local ENABLE_G28 = false -- ★ G28 实证：set_asset 换哪吒成功（截图 capture_1787579397），但用户复验修正：set_asset 必须用数编表ID（$$p_55a3.model.nezha.root），裸 prefab 路径不渲染（用户已改码，原路径行注释保留）
local ENABLE_G29 = false -- ★ G29 实证：merge_cache 虚拟条目 lua 侧可读（readback ok）但 CreateActor=nil——merge_cache 只写 lua 缓存（eff.lua:155），native 注册表启动时定，运行时只读
local ENABLE_G30 = false -- ★ G30 实证：手建 EffectActor+show 不渲染（需 play('cast') 触发，条目 EventCreation=on_cast_start）；CreateActor 种子横幅 ok，但 set_asset 换特效（res/ 与无前缀两种形态）均无报错画面不变——特效侧裸路径 set_asset 无效（同模型侧结论：资产加载走数编注册）
local ENABLE_G31 = false -- ★ G31 实证：load_map('default') 注册表命中真加载（native 日志 will load map/Begin load map.acmap/Load Map Time 0.014s）但 RT 黑（bgfx DX 0x80070057）；set_map_dir 拼接不带分隔符（值必须末尾带 /）；bogus 名只 preload 失败。terrain 三缺（weightmap/heightmap/landscape.xml）初次加载就有=良性
local ENABLE_G32 = false -- ★ G32: load_map 后重建相机/actor 验证画面恢复 + set_map_dir 带尾斜杠复测
local ENABLE_G33 = false -- ★ G33 实证（render-20）：用户 virtual_effect 三入口终审——CreateActor/ModelActor.new=nil、set_asset(虚拟model link)静默 no-op（视觉对照）；lua 层读回全通。虚拟数编有效域=仅 lua cache 消费者
local ENABLE_G34 = false -- ★ G34a 实证：PIE 游戏 lua 无模块级 load_map/LoadMainMap/reset/SaveJson/EDITOR（仅 game.load_combined_map/load_map_to_cache 场景缓存 API）——绑定在 xdeditor 侧，游戏侧只能 frida 直调
local ENABLE_G35 = false -- ★ G35 实证：unit_change_model 触发引擎自调 ctx getter 成功（render-22；配合 frida 捕 (L,ctx)）
local ENABLE_G36 = false -- ★ G36 实证：注入未生效前 CreateActor(未注册link) 抛 lua 错误（uiworldscript:279 不防御 nil 缓存）——与 native nil 是两种死亡形态（render-22 §5）

local g1_id_logged = false

local function drive()
    ui.imgui_begin_view('main', 'gw_view')
    -- G1: scene + resource=actor link
    if ENABLE_G1 and ui.imgui_begin_ui('scene', 'gw1') then
        ui.imgui_props(true, ACTOR, function(show, res)
            if not g1_id_logged then
                g1_id_logged = true
                local st = ui.imgui_state()
                mark('G1 imgui_state.id=' .. tostring(st and st.id))
            end
            return {
                show = show, independent = true,
                layout = { width = 400, height = 400, position_type = 'absolute', position = { 500, 150 } },
            }
        end)
        ui.imgui_end_ui('scene', 'gw1')
    end
    -- G3: image 控件吃 RT 链接
    if ENABLE_G3 and ui.imgui_begin_ui('image', 'gw3') then
        ui.imgui_props(true, 'RT:gw_probe_img', function(show, img)
            return {
                show = show, image = img,
                layout = { width = 300, height = 300, position_type = 'absolute', position = { 950, 200 } },
            }
        end)
        ui.imgui_end_ui('image', 'gw3')
    end
    ui.imgui_end_view('main', 'gw_view')
end

base.event_register(base.game, '场景-加载完成', function()
    base.wait(5000, function()
        mark('defaultui=' .. tostring(defaultui) .. ' ImportSCEContext=' .. tostring(ImportSCEContext) .. ' SCE=' .. tostring(SCE))

        -- ★ G6: base.ui.scene 官方 API（base_doc_client.d.lua：创建 ui 场景控件显示一个模型）
        if ENABLE_G6 then
            try('G6-scene', function()
                mark('base.ui.scene=' .. tostring(base.ui and base.ui.scene))
                local ctrl = base.ui.scene({
                    resource = UNIT,
                    light = { directional = { direction = { -1148, -1530, 1000 }, color = { 1, 1, 1.1 }, shadow = false } },
                    layout = { width = 400, height = 400, position_type = 'absolute', position = { 500, 150 } },
                })
                mark('G6 ctrl=' .. tostring(ctrl))
                _G.__gw_g6 = ctrl
                if ctrl and ctrl.ui then
                    local rok, x, y, w, h = pcall(function()
                        return ctrl.ui:rect()
                    end)
                    mark('G6 rect: ' .. tostring(rok) .. ' ' .. tostring(x) .. ',' .. tostring(y) .. ' ' .. tostring(w) .. 'x' .. tostring(h))
                end
            end)
            -- 变体：resource 用 actor link 对照
            try('G6b-scene-actor', function()
                local ctrl2 = base.ui.scene({
                    resource = ACTOR,
                    light = { directional = { direction = { -1148, -1530, 1000 }, color = { 1, 1, 1.1 }, shadow = false } },
                    layout = { width = 400, height = 400, position_type = 'absolute', position = { 950, 150 } },
                })
                mark('G6b ctrl=' .. tostring(ctrl2))
            end)
        end

        -- ★ G7: base.ui.scene + base.ui.create 官方流（script-199 common/test/scene.lua）
        -- model.name=数编条目显示名；particle.name=数编粒子显示名；无需 camera_info（有默认相机）
        if ENABLE_G7 then
            local function g7(tag, extra)
                try(tag, function()
                    local props = {
                        color = 'rgba(255, 0, 0, 0.25)', -- 半透明红看控件占位
                        show = true,
                        layout = { width = 400, height = 400, position_type = 'absolute', position = extra.pos },
                    }
                    for k, v in pairs(extra) do
                        if k ~= 'pos' then props[k] = v end
                    end
                    local tpl = base.ui.scene(props)
                    local u, b = base.ui.create(tpl, 'G7_' .. tag)
                    mark(tag .. ' created ui=' .. tostring(u))
                end)
            end
            g7('G7a-model-unit', { pos = { 100, 150 }, model = { name = '主控', facing = 0, position = { 0, 0, 0 } } })
            g7('G7b-model-actor', { pos = { 550, 150 }, model = { name = 'bgd吉鲁鲁附着模型', facing = 0 } })
            g7('G7c-particle', { pos = { 1000, 150 }, particle = { name = 'lib_control_assist_circle' } })
            g7('G7d-model+particle', {
                pos = { 1450, 150 },
                model = { name = '主控', facing = 30, scale = 1 },
                particle = { name = 'lib_control_assist_circle' },
            })
        end

        -- ★ G8: 内省 + scene 变体矩阵
        if ENABLE_G8 then
            -- 8-1: @gameui.component 源码定位（debug.getinfo 拿真实文件路径）
            try('G8-srclocate', function()
                local comp = require('@gameui.component')
                local info = debug.getinfo(comp.UIScene, 'S')
                mark('G8 UIScene source=' .. tostring(info and info.source) .. ' line=' .. tostring(info and info.linedefined))
                local ckeys = {}
                for k in pairs(comp) do ckeys[#ckeys + 1] = tostring(k) end
                mark('G8 component keys=' .. table.concat(ckeys, ','))
            end)
            -- 8-2: G5 组件实例结构 dump（RenderTarget 写往何处）
            try('G8-instdump', function()
                local inst = _G.__gw_uiscene
                if not inst then
                    mark('G8 no inst')
                    return
                end
                local ik = {}
                for k, v in pairs(inst) do ik[#ik + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                mark('G8 inst fields=' .. table.concat(ik, ','))
                local mt = getmetatable(inst)
                if mt then
                    local mk = {}
                    for k in pairs(mt) do mk[#mk + 1] = tostring(k) end
                    mark('G8 inst mt=' .. table.concat(mk, ','))
                    if type(mt.__index) == 'table' then
                        local pk = {}
                        for k, v in pairs(mt.__index) do pk[#pk + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                        mark('G8 inst proto=' .. table.concat(pk, ','))
                    end
                end
                -- RenderTarget setter 溯源：写后读回 inst 字段变化
                mark('G8 inst.RenderTarget(before)=' .. tostring(inst.RenderTarget))
            end)
            -- 8-3: scene 变体矩阵
            local function g8(tag, extra)
                try(tag, function()
                    local props = {
                        color = 'rgba(0, 255, 0, 0.35)',
                        show = true,
                        layout = { width = 300, height = 300, position_type = 'absolute', position = extra.pos },
                    }
                    for k, v in pairs(extra) do
                        if k ~= 'pos' then props[k] = v end
                    end
                    local u = base.ui.create(base.ui.scene(props), 'G8_' .. tag)
                    local rok, x, y, w, h = pcall(function() return u:rect() end)
                    mark(tag .. ' ui=' .. tostring(u) .. ' rect=' .. tostring(rok) .. ' ' .. tostring(x) .. ',' .. tostring(y) .. ' ' .. tostring(w) .. 'x' .. tostring(h))
                end)
            end
        end
        -- ★ G9: independent=true + light 矩阵（G8a 出画面但全黑=无光照）
        if ENABLE_G9 then
            local LIGHT = { directional = { direction = { -1148, -1530, 1000 }, color = { 1, 1, 1.1 }, shadow = false } }
            local function g9(tag, extra)
                try(tag, function()
                    local props = {
                        color = 'rgba(0, 0, 255, 0.2)',
                        show = true,
                        independent = true,
                        light = LIGHT,
                        layout = { width = 300, height = 300, position_type = 'absolute', position = extra.pos },
                    }
                    for k, v in pairs(extra) do
                        if k ~= 'pos' then props[k] = v end
                    end
                    local u = base.ui.create(base.ui.scene(props), 'G9_' .. tag)
                    mark(tag .. ' ui=' .. tostring(u))
                end)
            end
            g9('G9a-unit', { pos = { 100, 600 }, model = { name = '主控' } })
            g9('G9b-actor', { pos = { 450, 600 }, model = { name = 'bgd吉鲁鲁附着模型' } })
            g9('G9c-modelentry', { pos = { 800, 600 }, model = { name = '默认动画预览模型' } })
            g9('G9d-unit+particle', { pos = { 1150, 600 }, model = { name = '主控' }, particle = { name = 'lib_control_assist_circle' } })
            g9('G9e-particle-only', { pos = { 1500, 600 }, particle = { name = 'lib_control_assist_circle' } })
            -- 修正 G8 内省：UIScene 是 class table，getinfo 取方法
            try('G9-srclocate', function()
                local comp = require('@gameui.component')
                local cls = comp.UIScene
                for _, mname in ipairs({ 'new', 'RenderTarget', 'set_RenderTarget', 'get_RenderTarget' }) do
                    local f = rawget(cls, mname)
                    if type(f) == 'function' then
                        local info = debug.getinfo(f, 'S')
                        mark('G9 UIScene.' .. mname .. ' @ ' .. tostring(info.source) .. ':' .. tostring(info.linedefined))
                    end
                end
                local ck = {}
                for k, v in pairs(cls) do ck[#ck + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                mark('G9 UIScene cls=' .. table.concat(ck, ','))
            end)
        end

        -- ★ G10: 组件类深度 dump + scene 光照/相机矩阵
        if ENABLE_G10 then
            try('G10-clsdump', function()
                local cls = require('@gameui.component').UIScene
                mark('G10 __ui_type=' .. tostring(cls.__ui_type))
                local function dumpkeys(t, name)
                    if type(t) ~= 'table' then
                        mark('G10 ' .. name .. '=' .. tostring(t))
                        return
                    end
                    local ks = {}
                    for k, v in pairs(t) do ks[#ks + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                    mark('G10 ' .. name .. '={' .. table.concat(ks, ',') .. '}')
                end
                dumpkeys(cls.prop, 'prop')
                dumpkeys(cls.__prop_def_table, 'prop_def')
                dumpkeys(cls.metadata, 'metadata')
                dumpkeys(cls.__template_metadata, 'tpl_meta')
                dumpkeys(cls.method, 'method')
                if type(cls.prop) == 'table' and type(cls.prop.RenderTarget) ~= 'nil' then
                    mark('G10 prop.RenderTarget=' .. tostring(cls.prop.RenderTarget))
                end
            end)
            local function g10(tag, extra)
                try(tag, function()
                    local props = {
                        show = true,
                        independent = true,
                        layout = { width = 300, height = 300, position_type = 'absolute', position = extra.pos },
                    }
                    for k, v in pairs(extra) do
                        if k ~= 'pos' then props[k] = v end
                    end
                    local u = base.ui.create(base.ui.scene(props), 'G10_' .. tag)
                    mark(tag .. ' ui=' .. tostring(u))
                end)
            end
            local M = { name = '主控' }
            g10('G10a-ambient-tbl', { pos = { 100, 100 }, model = M, ambient_color = { 1, 1, 1 } })
            g10('G10b-ambient-str', { pos = { 450, 100 }, model = M, ambient_color = 'rgba(255,255,255,1)' })
            g10('G10c-lightgroup', { pos = { 800, 100 }, model = M, lightgroup = 'Editor/Light/Engine/default.lightgroup' })
            g10('G10d-camtbl', { pos = { 1150, 100 }, model = M, camera_info = { init_position = { X = 0, Y = 0 }, default_rotation = { X = -30, Y = 0, Z = 0 }, max_distance = 300, filed_of_view = 45 } })
            g10('G10e-zoom', { pos = { 1500, 100 }, model = M, zoom = 2 })
        end

        -- ★ G11: 手写页面内嵌 UIScene 组件（native UIScene 控件唯一官方创建路径）+ BindToUIScene
        if ENABLE_G11 then
            try('G11-page', function()
                local page = base.gui_new('GWProbePage')
                mark('G11 page=' .. tostring(page))
                if not page then
                    return
                end
                _G.__gw_page = page
                local pk = {}
                for k, v in pairs(page) do pk[#pk + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                mark('G11 page fields=' .. table.concat(pk, ','))
                -- 命名控件访问：生产实证 base.gui_get_part(page, '名') = page.part[name][1]（p_2xgc 排行榜）
                local ctrl = base.gui_get_part(page, 'gw_scene')
                mark('G11 gui_get_part gw_scene=' .. tostring(ctrl))
                if not ctrl and page.part then
                    local pk2 = {}
                    for k in pairs(page.part) do pk2[#pk2 + 1] = tostring(k) end
                    mark('G11 page.part keys=' .. table.concat(pk2, ','))
                end
                _G.__gw_uiscene2 = ctrl
                base.wait(300, function()
                    try('G11-bind', function()
                        local c = _G.__gw_uiscene2
                        if not c then
                            mark('G11-bind no ctrl')
                            return
                        end
                        mark('G11 ctrl.ui=' .. tostring(c.ui) .. ' RenderPath=' .. tostring(c.RenderPath))
                        if c.ui then
                            local rok, x, y, w, h = pcall(function() return c.ui:rect() end)
                            mark('G11 rect: ' .. tostring(rok) .. ' ' .. tostring(x) .. ',' .. tostring(y) .. ' ' .. tostring(w) .. 'x' .. tostring(h))
                        end
                        local world = _G.__gw_world1
                        if world then
                            local r = world:BindToUIScene(c)
                            mark('G11 BindToUIScene => ' .. tostring(r))
                        else
                            mark('G11-bind no world')
                        end
                    end)
                end)
            end)
        end
        -- ★ G17: 纯运行时构造页面（无文件，免疫编辑器清理管线）
        if ENABLE_G17 then
            try('G17-page', function()
                local component = require('@common.base.gui.component')
                local gui_pkg = require('@common.base.gui.package')
                local ctrl_wrapper = require('@common.base.gui.ctrl_wrapper')
                local gameui = require('@gameui.component')
                local t = gui_pkg.page_template {
                    flatten_template = {
                        ctrl_wrapper.panel {
                            disabled = false,
                            layout = { direction = 'row', grow_height = 1, grow_width = 1 },
                            name = 'GWProbeRT',
                            show = true,
                        },
                        0,
                        gameui.UIScene {
                            RenderPath = 'EngineRes/RenderPaths/GameSnapshot.xml',
                            UseShadow = false,
                            disabled = false,
                            layout = {
                                col_self = 'start',
                                row_self = 'start',
                                position = { 500, 150 },
                                width = 600,
                                height = 500,
                            },
                            name = 'gw_scene',
                            show = true,
                        },
                        1,
                    },
                }
                local cls = component('GWProbeRT') { t.template, event = {}, prop = {}, method = {}, state = {} }
                local page = cls:new()
                _G.__gw_page = page
                local ctrl = base.gui_get_part(page, 'gw_scene')
                mark('G17 page=' .. tostring(page) .. ' ctrl=' .. tostring(ctrl) .. ' ctrl.ui=' .. tostring(ctrl and ctrl.ui))
                _G.__gw_uiscene2 = ctrl
            end)
        end

        if ENABLE_G13 then
            base.wait(900, function()
                try('G13-dump', function()
                    local cls = require('@gameui.component').UIScene
                    -- cls[1] = 组件根控件模板
                    local root = cls[1]
                    if type(root) == 'table' then
                        local rk = {}
                        for k, v in pairs(root) do rk[#rk + 1] = tostring(k) .. ':' .. tostring(v) end
                        mark('G13 cls[1]=' .. table.concat(rk, ','))
                    end
                    mark('G13 __part_count=' .. tostring(cls.__part_count) .. ' __ui_type=' .. tostring(cls.__ui_type))
                    -- 页面组件实例的 part 链
                    local page = _G.__gw_page
                    local ctrl = _G.__gw_uiscene2
                    if ctrl then
                        local function dumpparts(t, name)
                            if type(t) ~= 'table' then
                                mark('G13 ' .. name .. '=' .. tostring(t))
                                return
                            end
                            local ks = {}
                            for k, v in pairs(t) do ks[#ks + 1] = tostring(k) .. '=' .. tostring(v) end
                            mark('G13 ' .. name .. '={' .. table.concat(ks, ',') .. '}')
                        end
                        dumpparts(rawget(ctrl, '__part'), 'ctrl.__part')
                        dumpparts(rawget(ctrl, 'part'), 'ctrl.part')
                        dumpparts(rawget(ctrl, 'child'), 'ctrl.child')
                        dumpparts(rawget(ctrl, 'base'), 'ctrl.base')
                        -- 组件类自身（__class）
                        local c2 = rawget(ctrl, '__class')
                        if c2 then
                            mark('G13 ctrl.__class.__ui_type=' .. tostring(c2.__ui_type) .. ' name=' .. tostring(c2.name))
                        end
                        -- RenderTarget 现在值（G11 bind 写过）
                        mark('G13 ctrl.RenderTarget=' .. tostring(rawget(ctrl, 'RenderTarget') or (ctrl.prop and ctrl.prop.RenderTarget)))
                        if ctrl.prop then
                            dumpparts(ctrl.prop, 'ctrl.prop')
                        end
                    end
                end)
            end)
        end

        -- ★ G14: part 类型 dump + 载真实场景的世界 Bind 视觉验证
        if ENABLE_G14 then
            base.wait(1200, function()
                try('G14-parts', function()
                    local ctrl = _G.__gw_uiscene2
                    if ctrl and ctrl.part then
                        for i, p in pairs(ctrl.part) do
                            local ui = type(p) == 'table' and (rawget(p, 'ui') or (type(p[1]) == 'table' and rawget(p[1], 'ui'))) or nil
                            mark('G14 part[' .. tostring(i) .. ']=' .. tostring(p) .. ' ui=' .. tostring(ui))
                        end
                    end
                end)
                try('G14-mapworld', function()
                    local world3 = defaultui.UIWorld:Create(false, CAM, 'default')
                    mark('G14 world3=' .. tostring(world3))
                    if not world3 then
                        return
                    end
                    local ctrl = _G.__gw_uiscene2
                    if ctrl then
                        local r = world3:BindToUIScene(ctrl)
                        mark('G14 BindToUIScene(map) => ' .. tostring(r))
                    end
                    _G.__gw_world3 = world3
                end)
            end)
        end

        -- ★ G18: scene 控件 render-16 配方验证矩阵（全部 independent=true）
        if ENABLE_G18 then
            local function g18(tag, extra)
                try(tag, function()
                    local props = {
                        color = 'rgba(255, 255, 0, 0.15)', -- 淡黄占位便于看控件区域
                        show = true,
                        independent = true,
                        layout = { width = 300, height = 300, position_type = 'absolute', position = extra.pos },
                    }
                    for k, v in pairs(extra) do
                        if k ~= 'pos' then props[k] = v end
                    end
                    local u, b = base.ui.create(base.ui.scene(props), 'G18_' .. tag)
                    mark(tag .. ' ui=' .. tostring(u) .. ' bind=' .. tostring(b))
                    return b
                end)
            end
            g18('G18a-scale3D', { pos = { 50, 650 }, model = { name = '主控', facing = 0, scale3D = 0.7, anim = 'Idle', anim_fade_time = 0.1 } })
            g18('G18b-scale', { pos = { 400, 650 }, model = { name = '主控', facing = 0, scale = 0.7 } })
            g18('G18c-particle', { pos = { 750, 650 }, particle = { name = 'lib_control_assist_circle' } })
            g18('G18d-model+particle', { pos = { 1100, 650 }, model = { name = '主控', facing = 30, scale3D = 0.7 }, particle = { name = 'lib_control_assist_circle' } })
            g18('G18e-badanim', { pos = { 1450, 650 }, model = { name = '主控', anim = '不存在的动画名xyz' } })
            -- G18f: 动态 bind（创建后改 name/anim/scale3D，验证 render-16 不确定点6）
            try('G18f-dynbind', function()
                local u, b = base.ui.create(base.ui.scene({
                    show = true, independent = true,
                    layout = { width = 300, height = 300, position_type = 'absolute', position = { 1550, 150 } },
                    model = { name = '主控', facing = 0, scale3D = 0.7 },
                }), 'G18f')
                mark('G18f ui=' .. tostring(u) .. ' bind=' .. tostring(b))
                if b then
                    local bk = {}
                    for k, v in pairs(b) do bk[#bk + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                    mark('G18f bind keys=' .. table.concat(bk, ','))
                end
                base.wait(2500, function()
                    try('G18f-write', function()
                        b.anim = 'Idle'
                        b.scale3D = 1.5
                        mark('G18f bind written anim/scale3D')
                    end)
                end)
            end)
        end

        -- ★ G19: UIWorld CreateActor 吃 particle link（免数编 ActorEffect 试验，复用 G16 world3）
        if ENABLE_G19 then
            base.wait(2500, function()
                try('G19-particle-actor', function()
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G19 no world3')
                        return
                    end
                    local pa = world3:CreateActor('$$lib_control.particle.lib_control_assist_circle.root')
                    mark('G19 particle-actor=' .. tostring(pa))
                    if pa then
                        try('G19-pos', function()
                            pa:set_position(120, 0, 0)
                        end)
                        try('G19-play', function()
                            pa:play('Idle')
                        end)
                    end
                end)
            end)
        end

        -- ★ G20: scene 决定性矩阵（全部无 color、independent=true；黑=渲染激活，透明=无内容）
        if ENABLE_G20 then
            local LIGHT = { directional = { direction = { -1148, -1530, 1000 }, color = { 1, 1, 1.1 }, shadow = false } }
            local function g20(tag, extra)
                try(tag, function()
                    local props = {
                        show = true,
                        independent = true,
                        layout = { width = 300, height = 300, position_type = 'absolute', position = extra.pos },
                    }
                    for k, v in pairs(extra) do
                        if k ~= 'pos' then props[k] = v end
                    end
                    local u = base.ui.create(base.ui.scene(props), 'G20_' .. tag)
                    mark(tag .. ' ui=' .. tostring(u))
                end)
            end
            g20('G20a-unit', { pos = { 50, 650 }, model = { name = '主控', facing = 0 } })
            g20('G20b-badname', { pos = { 400, 650 }, model = { name = '不存在的单位xyz', facing = 0 } })
            g20('G20c-particle', { pos = { 750, 650 }, particle = { name = 'lib_control_assist_circle' } })
            g20('G20d-unit+light', { pos = { 1100, 650 }, model = { name = '主控', facing = 0 }, light = LIGHT })
            g20('G20e-unit+light+zoom', { pos = { 1450, 650 }, model = { name = '主控', facing = 0 }, light = LIGHT, zoom = 3 })
            g20('G20f-unit+overlight', { pos = { 50, 150 }, model = { name = '主控', facing = 0 }, light = { directional = { direction = { 0, -1000, 500 }, color = { 10, 10, 10 }, shadow = false } } })
            g20('G20g-unit+light2', { pos = { 400, 150 }, model = { name = '主控', facing = 0 }, light = { directional = { direction = { 0, -1, 0.5 }, color = { 1, 1, 1 }, shadow = false } }, ambient_color = { 1, 1, 1 } })
        end

        -- ★ G21: 数编 ActorEffect 条目进 UIWorld（复用 G16 world3，放吉鲁鲁旁边）
        if ENABLE_G21 then
            base.wait(2500, function()
                try('G21-effect-actor', function()
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G21 no world3')
                        return
                    end
                    local ea = world3:CreateActor('$$p_55a3.actor.bgd_demo_effect.root')
                    mark('G21 effect-actor=' .. tostring(ea))
                    if ea then
                        try('G21-pos', function()
                            ea:set_position(0, 0, 0) -- 与吉鲁鲁同点，必在画面内
                        end)
                        try('G21-play', function()
                            ea:play('cast')
                        end)
                    end
                end)
            end)
        end

        -- ★ G22: 自定义 renderpath（项目包内 xml）与 CEMap 对照（G17 同款运行时页面流）
        if ENABLE_G22 then
            try('G22-pages', function()
                local component = require('@common.base.gui.component')
                local gui_pkg = require('@common.base.gui.package')
                local ctrl_wrapper = require('@common.base.gui.ctrl_wrapper')
                local gameui = require('@gameui.component')
                local function mkpage(name, renderpath, pos)
                    local t = gui_pkg.page_template {
                        flatten_template = {
                            ctrl_wrapper.panel {
                                disabled = false,
                                layout = { direction = 'row', grow_height = 1, grow_width = 1 },
                                name = name,
                                show = true,
                            },
                            0,
                            gameui.UIScene {
                                RenderPath = renderpath,
                                UseShadow = false,
                                disabled = false,
                                layout = { col_self = 'start', row_self = 'start', position = pos, width = 350, height = 300 },
                                name = 'gw_scene',
                                show = true,
                            },
                            1,
                        },
                    }
                    local cls = component(name) { t.template, event = {}, prop = {}, method = {}, state = {} }
                    local page = cls:new()
                    return base.gui_get_part(page, 'gw_scene')
                end
                local ctrlA = mkpage('GWProbeRPRed', 'res/renderpaths/bgd_snapshot_red.xml', { 1150, 150 })
                local ctrlB = mkpage('GWProbeRPCEMap', 'EngineRes/RenderPaths/CEMap.xml', { 1520, 150 })
                mark('G22 ctrlA=' .. tostring(ctrlA) .. ' ctrlB=' .. tostring(ctrlB))
                _G.__gw_ctrlA = ctrlA
                _G.__gw_ctrlB = ctrlB
                base.wait(400, function()
                    try('G22-worlds', function()
                        local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 300)
                        local wA = defaultui.UIWorld:Create(false, CAM, 'default')
                        wA:SetCameraPosition(pos[1], pos[2], pos[3])
                        wA:SetCameraRotation(-70, 0, 0)
                        local a = wA:CreateActor(ACTOR)
                        if a then
                            a:set_position(0, 0, 0)
                        end
                        mark('G22 bindA(red custom rp)=' .. tostring(wA:BindToUIScene(_G.__gw_ctrlA)))
                        local wB = defaultui.UIWorld:Create(false, CAM, 'default')
                        wB:SetCameraPosition(pos[1], pos[2], pos[3])
                        wB:SetCameraRotation(-70, 0, 0)
                        mark('G22 bindB(cemap)=' .. tostring(wB:BindToUIScene(_G.__gw_ctrlB)))
                    end)
                end)
            end)
        end

        -- ★ G23: ImportSCEContext 能力目录 dump（找绕数编的直载 API）
        if ENABLE_G23 then
            try('G23-sce-dump', function()
                local SCE2 = ImportSCEContext(nil)
                local function keys(t)
                    local ks = {}
                    for k, v in pairs(t) do ks[#ks + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                    table.sort(ks)
                    return table.concat(ks, ',')
                end
                mark('G23 SCE={' .. keys(SCE2) .. '}')
                for _, name in ipairs({ 'GameWorld', 'ModelActor', 'EffectActor', 'GameUnit', 'BeamActor', 'MaterialActor', 'AdditionModelActor' }) do
                    local sub = rawget(SCE2, name)
                    if type(sub) == 'table' then
                        mark('G23 SCE.' .. name .. '={' .. keys(sub) .. '}')
                        local mt = getmetatable(sub)
                        if mt and type(mt.__index) == 'table' then
                            mark('G23 SCE.' .. name .. '.__index={' .. keys(mt.__index) .. '}')
                        end
                    end
                end
            end)
            -- innerWorld（GameWorld 实例）方法目录
            base.wait(2000, function()
                try('G23-gameworld-dump', function()
                    local w = _G.__gw_world3
                    if not w then
                        mark('G23 no world3')
                        return
                    end
                    local iw = w.innerWorld
                    local function keys(t)
                        local ks = {}
                        for k, v in pairs(t) do ks[#ks + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                        table.sort(ks)
                        return table.concat(ks, ',')
                    end
                    mark('G23 innerWorld=' .. tostring(iw))
                    if iw then
                        mark('G23 innerWorld fields={' .. keys(iw) .. '}')
                        local mt = getmetatable(iw)
                        if mt and type(mt.__index) == 'table' then
                            mark('G23 innerWorld.__index={' .. keys(mt.__index) .. '}')
                        end
                    end
                    -- UIWorld 实例方法
                    mark('G23 world3 fields={' .. keys(w) .. '}')
                    local wmt = getmetatable(w)
                    if wmt and type(wmt.__index) == 'table' then
                        mark('G23 world3.__index={' .. keys(wmt.__index) .. '}')
                    end
                end)
            end)
        end

        -- ★ G24: set_asset 免数编直载 + load_map/set_map_dir/use_light_group 探查
        if ENABLE_G24 then
            base.wait(3000, function()
                local SCE3 = ImportSCEContext(nil)
                local world3 = _G.__gw_world3
                if not world3 then
                    mark('G24 no world3')
                    return
                end
                local iw = world3.innerWorld
                -- G24a: 有效 link 建 ModelActor + set_asset 换资产（先验证 API 形态）
                try('G24a-model-setasset', function()
                    local ma = SCE3.ModelActor.new('$$p_55a3.actor.bgd_jilulu_attach.root')
                    mark('G24a ma=' .. tostring(ma))
                    if ma then
                        iw:add_game_actor(ma)
                        ma:set_position({ -150, 0, 0 })
                        try('G24a-getmesh', function()
                            local mesh = ma:get_mesh_asset()
                            mark('G24a get_mesh_asset=' .. tostring(mesh))
                            if type(mesh) == 'table' then
                                local mk = {}
                                for k, v in pairs(mesh) do mk[#mk + 1] = tostring(k) .. '=' .. tostring(v) end
                                mark('G24a mesh={' .. table.concat(mk, ',') .. '}')
                            end
                        end)
                        try('G24a-setasset', function()
                            ma:set_asset('characters/_user/jilulu_19ec/model.prefab')
                        end)
                        try('G24a-play', function()
                            ma:play('Idle')
                        end)
                    end
                end)
                -- G24b: 无参建 ModelActor（免数编直载尝试）
                try('G24b-model-nolink', function()
                    local ma = SCE3.ModelActor.new()
                    mark('G24b ma(no arg)=' .. tostring(ma))
                    if ma then
                        iw:add_game_actor(ma)
                        ma:set_position({ -300, 0, 0 })
                        try('G24b-setasset', function()
                            ma:set_asset('characters/_user/jilulu_19ec/model.prefab')
                        end)
                        try('G24b-play', function()
                            ma:play('Idle')
                        end)
                    end
                end)
                -- G24c: 无参建 EffectActor + set_asset 直载特效（免数编尝试）
                try('G24c-effect-nolink', function()
                    local ea = SCE3.EffectActor.new()
                    mark('G24c ea(no arg)=' .. tostring(ea))
                    if ea then
                        iw:add_game_actor(ea)
                        ea:set_position({ 150, 0, 0 })
                        try('G24c-setasset', function()
                            ea:set_asset('res/effect/bgd_libs_client/demo/p_12sc_effect_new_6o1_dl47/particle.effect')
                        end)
                    end
                end)
                -- G24d: load_map / set_map_dir（独立离屏世界，不绑控件）
                -- 坑：use_light_group('Editor/Light/Engine/default.lightgroup') 硬崩（整个编辑器进程消失，2026-08-24 G24 首轮）
                try('G24d-map-apis', function()
                    local infos = SCE3.GetGameWorldInfos()
                    if type(infos) == 'table' then
                        local ik = {}
                        for k, v in pairs(infos) do ik[#ik + 1] = tostring(k) .. '=' .. tostring(v) end
                        mark('G24d GetGameWorldInfos={' .. table.concat(ik, ',') .. '}')
                    else
                        mark('G24d GetGameWorldInfos=' .. tostring(infos))
                    end
                    local wX = SCE3.GameWorld:new()
                    wX:create_scene(false)
                    try('G24d-set_map_dir', function()
                        wX:set_map_dir('default')
                    end)
                    try('G24d-load_map', function()
                        wX:load_map('default')
                    end)
                    wX:purge()
                    wX:__release()
                    mark('G24d map apis done')
                end)
            end)
        end

        -- ★ G25: 手建 actor 补 show(true)；假 link 种子 + set_asset 免数编再试
        if ENABLE_G25 then
            base.wait(3000, function()
                local SCE3 = ImportSCEContext(nil)
                local world3 = _G.__gw_world3
                if not world3 then
                    mark('G25 no world3')
                    return
                end
                local iw = world3.innerWorld
                -- G25a: 手建 ModelActor（有效 link）+ show(true)
                try('G25a-manual-show', function()
                    local ma = SCE3.ModelActor.new('$$p_55a3.actor.bgd_jilulu_attach.root')
                    mark('G25a ma=' .. tostring(ma))
                    if ma then
                        iw:add_game_actor(ma)
                        ma:set_position({ -150, 0, 0 })
                        try('G25a-show', function()
                            ma:show(true)
                        end)
                        try('G25a-play', function()
                            ma:play('Idle')
                        end)
                    end
                end)
                -- G25b: 手建 EffectActor（有效 link）+ show(true)
                try('G25b-manual-effect', function()
                    local ea = SCE3.EffectActor.new('$$p_55a3.actor.bgd_demo_effect.root')
                    mark('G25b ea=' .. tostring(ea))
                    if ea then
                        iw:add_game_actor(ea)
                        ea:set_position({ 150, 0, 0 })
                        try('G25b-show', function()
                            ea:show(true)
                        end)
                    end
                end)
                -- G25c: 假 link 种子 + set_asset（免数编直载终极尝试）
                for i, seed in ipairs({ '', '$$p_55a3.actor.bgd_seed_void.root', 'x' }) do
                    try('G25c-seed[' .. i .. ']=' .. seed, function()
                        local ma = SCE3.ModelActor.new(seed)
                        mark('G25c[' .. i .. '] ma=' .. tostring(ma))
                        if ma then
                            iw:add_game_actor(ma)
                            ma:set_position({ -300, 0, 0 })
                            try('G25c[' .. i .. ']-setasset', function()
                                ma:set_asset('characters/_user/jilulu_19ec/model.prefab')
                            end)
                            try('G25c[' .. i .. ']-show', function()
                                ma:show(true)
                            end)
                            try('G25c[' .. i .. ']-play', function()
                                ma:play('Idle')
                            end)
                        end
                    end)
                end
            end)
        end

        -- ★ G28: set_asset 真换模型（jilulu 种子 → 哪吒），相机 600，放 (-150,0,0)
        if ENABLE_G28 then
            base.wait(3000, function()
                try('G28-swap-asset', function()
                    local SCE3 = ImportSCEContext(nil)
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G28 no world3')
                        return
                    end
                    local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 600)
                    world3:SetCameraPosition(pos[1], pos[2], pos[3])
                    world3:SetCameraRotation(-70, 0, 0)
                    local ma = SCE3.ModelActor.new('$$p_55a3.actor.bgd_jilulu_attach.root')
                    mark('G28 ma=' .. tostring(ma))
                    if ma then
                        world3.innerWorld:add_game_actor(ma)
                        ma:set_position({ -150, 0, 0 })
                        try('G28-setasset-nazha', function()
                            -- ma:set_asset('characters/_user/p_55a3_nazha_wuwuqi_xin1_85sc_w72l/model.prefab')
                            -- 这里要用数编表ID ，直接用模型路径不会成功渲染。
                            ma:set_asset('$$p_55a3.model.nezha.root')
                        end)
                        ma:show(true)
                        try('G28-play', function()
                            ma:play('Idle')
                        end)
                        mark('G28 swap to nazha done')
                    end
                end)
            end)
        end

        -- ★ G29: 运行时动态构建数编（virtual_effect 模式：cache 深拷贝 → merge_cache 虚拟条目 → CreateActor）
        if ENABLE_G29 then
            base.wait(3500, function()
                -- G29b: 运行时虚拟数编（深拷贝 root+Model 子节点 → merge_cache → CreateActor 虚拟 link）
                try('G29b-virtual-entry', function()
                    local function deepcopy(obj, visited)
                        if type(obj) ~= 'table' then
                            return obj
                        end
                        visited = visited or {}
                        if visited[obj] then
                            return visited[obj]
                        end
                        local nt = {}
                        visited[obj] = nt
                        for k, v in pairs(obj) do
                            nt[deepcopy(k, visited)] = deepcopy(v, visited)
                        end
                        return nt
                    end
                    local ROOT = '$$p_55a3.actor.bgd_jilulu_attach.root'
                    local CHILD = '$$p_55a3.actor.bgd_jilulu_attach.Model'
                    local VROOT = '$$p_55a3.actor.bgd_virtual_nazha.root'
                    local VCHILD = '$$p_55a3.actor.bgd_virtual_nazha.Model'
                    local root = base.eff.cache(ROOT)
                    local child = base.eff.cache(CHILD)
                    mark('G29b child type=' .. type(child))
                    if type(child) == 'table' then
                        local ck = {}
                        for k, v in pairs(child) do
                            if type(v) ~= 'table' then
                                ck[#ck + 1] = tostring(k) .. '=' .. tostring(v)
                            end
                        end
                        mark('G29b child fields=' .. table.concat(ck, ','))
                    end
                    local nr = deepcopy(root)
                    nr.Name = 'bgd虚拟哪吒'
                    nr.Link = VROOT
                    nr.Model = VCHILD -- 子节点引用是全 link（G29a 实证）
                    local dict = { [VROOT] = nr }
                    if type(child) == 'table' then
                        local nc = deepcopy(child)
                        nc.Link = VCHILD
                        nc.Asset = 'characters/_user/nazha_wuwuqi_xin1_85sc/model.prefab'
                        dict[VCHILD] = nc
                    end
                    base.eff.merge_cache({ dict = dict })
                    mark('G29b merge_cache done')
                    -- 读回验证
                    local back = base.eff.cache(VROOT)
                    mark('G29b readback=' .. tostring(back) .. ' NodeType=' .. tostring(back and back.NodeType))
                    -- 进世界
                    local world3 = _G.__gw_world3
                    if world3 then
                        local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 600)
                        world3:SetCameraPosition(pos[1], pos[2], pos[3])
                        world3:SetCameraRotation(-70, 0, 0)
                        local va = world3:CreateActor(VROOT)
                        mark('G29b virtual actor=' .. tostring(va))
                        if va then
                            va:set_position(-150, 0, 0)
                            try('G29b-play', function()
                                va:play('Idle')
                            end)
                        end
                    end
                end)
                try('G29a-eff-dump', function()
                    local ek = {}
                    for k, v in pairs(base.eff) do ek[#ek + 1] = tostring(k) .. ':' .. tostring(type(v)) end
                    table.sort(ek)
                    mark('G29a base.eff={' .. table.concat(ek, ',') .. '}')
                    local entry = base.eff.cache('$$p_55a3.actor.bgd_jilulu_attach.root')
                    mark('G29a entry type=' .. type(entry))
                    if type(entry) == 'table' then
                        local function dump(t, prefix, depth)
                            if depth > 3 then
                                return
                            end
                            for k, v in pairs(t) do
                                if type(v) == 'table' then
                                    mark('G29a ' .. prefix .. tostring(k) .. '={...}')
                                    if depth < 2 then
                                        dump(v, prefix .. tostring(k) .. '.', depth + 1)
                                    end
                                else
                                    mark('G29a ' .. prefix .. tostring(k) .. '=' .. tostring(v))
                                end
                            end
                        end
                        dump(entry, '', 1)
                    end
                end)
            end)
        end

        -- ★ G30: EffectActor 种子（bgd_demo_effect 数编条目）+ set_asset 换特效
        -- 目的：验证特效侧「一颗种子 link + set_asset = 任意本地特效免逐条数编」（对应 G28 模型侧实证）
        -- 注：第一轮手建 SCE.EffectActor.new+add_game_actor+show 不渲染——特效需 play('cast') 触发（条目 EventCreation=on_cast_start），改走 G21 实证路径
        if ENABLE_G30 then
            base.wait(3500, function()
                try('G30-swap-effect', function()
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G30 no world3')
                        return
                    end
                    local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 600)
                    world3:SetCameraPosition(pos[1], pos[2], pos[3])
                    world3:SetCameraRotation(-70, 0, 0)
                    local ea = world3:CreateActor('$$p_55a3.actor.bgd_demo_effect.root')
                    mark('G30 ea=' .. tostring(ea))
                    if ea then
                        ea:set_position(-150, 0, 0) -- 与 G28 哪吒同位（已实证可见）
                        try('G30-play-cast', function()
                            ea:play('cast')
                        end)
                        mark('G30 seed effect played')
                        -- 先渲染种子特效 6s，再换资产重新触发：截图对比是否真换
                        base.wait(6000, function()
                            try('G30-setasset-res-prefix', function()
                                ea:set_asset('res/effect/_user/uitexiao3_a4wc/particle.effect')
                            end)
                            mark('G30 set_asset(res/...) done')
                            try('G30-replay-1', function()
                                ea:play('cast')
                            end)
                            base.wait(6000, function()
                                try('G30-setasset-noprefix', function()
                                    ea:set_asset('effect/_user/uitexiao3_a4wc/particle.effect')
                                end)
                                mark('G30 set_asset(no res/ prefix) done')
                                try('G30-replay-2', function()
                                    ea:play('cast')
                                end)
                            end)
                        end)
                    end
                end)
            end)
        end

        -- ★ G31: GameWorld.load_map/set_map_dir 语义实测（render-19 逆向结论验证）
        if ENABLE_G31 then
            base.wait(4500, function()
                try('G31-loadmap', function()
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G31 no world3')
                        return
                    end
                    local iw = world3.innerWorld
                    -- G31a: 重载当前地图（'default' 必然已注册——G14 Create 第三参即为它）
                    try('G31a-loadmap-default', function()
                        local r = iw:load_map('default', false)
                        mark('G31a load_map(default)=' .. tostring(r))
                    end)
                    -- G31b: set_map_dir 到项目根（绝对路径）+ 再次 load_map
                    base.wait(2000, function()
                        try('G31b-setdir-loadmap', function()
                            iw:set_map_dir('C:/Users/woaye/Documents/SCE Projects/test_res002')
                            mark('G31b set_map_dir done')
                            local r2 = iw:load_map('default', false)
                            mark('G31b load_map(default) after set_map_dir=' .. tostring(r2))
                        end)
                    end)
                    -- G31c: 未注册地图名（负对照，预期打 Failed to load map 日志返回 false/nil）
                    base.wait(4000, function()
                        try('G31c-loadmap-bogus', function()
                            local r3 = iw:load_map('bgd_no_such_map', false)
                            mark('G31c load_map(bogus)=' .. tostring(r3))
                        end)
                    end)
                end)
            end)
        end

        -- ★ G32: load_map 后重建相机/actor 验证画面恢复 + set_map_dir 带尾斜杠复测
        if ENABLE_G32 then
            base.wait(4500, function()
                try('G32-reload-and-rebuild', function()
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G32 no world3')
                        return
                    end
                    local iw = world3.innerWorld
                    -- 复测 set_map_dir 带尾斜杠（G31b 发现拼接不带分隔符）
                    try('G32-setdir-slash', function()
                        iw:set_map_dir('C:/Users/woaye/Documents/SCE Projects/test_res002/')
                        mark('G32 set_map_dir(尾斜杠) done')
                    end)
                    try('G32-loadmap', function()
                        iw:load_map('default', false)
                        mark('G32 load_map(default) done')
                    end)
                    -- 2s 后重建相机 + 重建吉鲁鲁 actor（load_map 清空场景）
                    base.wait(2000, function()
                        try('G32-rebuild', function()
                            local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 300)
                            world3:SetCameraPosition(pos[1], pos[2], pos[3])
                            world3:SetCameraRotation(-70, 0, 0)
                            mark('G32 camera re-set')
                            local a = world3:CreateActor(ACTOR)
                            mark('G32 actor=' .. tostring(a))
                            if a then
                                a:set_position(0, 0, 0)
                                try('G32-play', function()
                                    a:play('Idle')
                                end)
                            end
                        end)
                    end)
                end)
            end)
        end

        -- ★ G33: 用户动态虚拟数编实测（virtual_effect.lua 原样 + merge_cache 子节点扩展）
        -- 矩阵：a=虚拟 ActorEffect 指向未注册特效 psmd（真实 dl47 对照）；b=用户类建虚拟 ActorModel（nazha 资产）CreateActor+ModelActor.new 双入口；c=虚拟 model link 喂 set_asset
        if ENABLE_G33 then
            base.wait(3500, function()
                local virtual_effect = require('src.client.virtual_effect')
                mark('G33 __MAIN_MAP__=' .. tostring(__MAIN_MAP__))
                local world3 = _G.__gw_world3
                if not world3 then
                    mark('G33 no world3')
                    return
                end
                local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 600)
                world3:SetCameraPosition(pos[1], pos[2], pos[3])
                world3:SetCameraRotation(-70, 0, 0)
                local function dcopy(obj, visited)
                    if type(obj) ~= 'table' then
                        return obj
                    end
                    visited = visited or {}
                    if visited[obj] then
                        return visited[obj]
                    end
                    local nt = {}
                    visited[obj] = nt
                    for k, v in pairs(obj) do
                        nt[dcopy(k, visited)] = dcopy(v, visited)
                    end
                    return nt
                end

                -- G33a: 虚拟 ActorEffect（root+Particle_1 全套）→ 未注册特效 psmd
                try('G33a-virtual-effect', function()
                    -- 对照组：真实 dl47 特效（已注册数编）放 (150,0,0)
                    local ea0 = world3:CreateActor('$$p_55a3.actor.bgd_demo_effect.root')
                    mark('G33a real effect actor=' .. tostring(ea0))
                    if ea0 then
                        ea0:set_position(150, 0, 0)
                        ea0:play('cast')
                    end
                    local VROOT = '$$p_55a3.actor.bgd_virtual_psmd.root'
                    local VCHILD = '$$p_55a3.actor.bgd_virtual_psmd.Particle_1'
                    local nr = dcopy(base.eff.cache('$$p_55a3.actor.bgd_demo_effect.root'))
                    local nc = dcopy(base.eff.cache('$$p_55a3.actor.bgd_demo_effect.Particle_1'))
                    mark('G33a src root=' .. type(nr) .. ' child=' .. type(nc))
                    if type(nr) ~= 'table' or type(nc) ~= 'table' then
                        return
                    end
                    nr.Name = 'bgd虚拟特效psmd'
                    nr.Link = VROOT
                    nr.Effect = VCHILD -- 编译产物里 Effect=全 link（obj/effect/actor/data.lua:45794 实证）
                    nc.Link = VCHILD
                    nc.Asset = 'res/effect/bgd_libs_client/demo/p_12sc_effect_new_6o1_psmd/particle.effect'
                    base.eff.merge_cache({ dict = { [VROOT] = nr, [VCHILD] = nc } })
                    local back = base.eff.cache(VROOT)
                    mark('G33a readback=' .. tostring(back) .. ' NodeType=' .. tostring(back and back.NodeType) .. ' Effect=' .. tostring(back and back.Effect))
                    local ea = world3:CreateActor(VROOT)
                    mark('G33a virtual effect actor=' .. tostring(ea))
                    if ea then
                        ea:set_position(-150, 0, 0)
                        ea:play('cast')
                    end
                end)

                -- G33b: 用户 virtual_effect.new 原样建虚拟 ActorModel + Model 子节点虚拟化（nazha 资产）
                try('G33b-virtual-model', function()
                    local ve = virtual_effect.new('$$p_55a3.actor.bgd_jilulu_attach.root', 'bgd_virtual_nazha')
                    mark('G33b ve=' .. tostring(ve) .. ' link=' .. tostring(ve and ve:get_link()))
                    if not ve then
                        return
                    end
                    local VCHILD = '$$p_55a3.actor.bgd_virtual_nazha.Model'
                    local nc = dcopy(base.eff.cache('$$p_55a3.actor.bgd_jilulu_attach.Model'))
                    if type(nc) == 'table' then
                        nc.Link = VCHILD
                        nc.Asset = 'characters/_user/p_55a3_nazha_wuwuqi_xin1_85sc_w72l/model.prefab'
                        base.eff.merge_cache({ dict = { [VCHILD] = nc } })
                        ve:set_value('@.Model', VCHILD) -- 用户 set_value API 改子节点引用（编译产物 Model=全 link）
                    end
                    local back = base.eff.cache(ve:get_link())
                    mark('G33b readback NodeType=' .. tostring(back and back.NodeType) .. ' Model=' .. tostring(back and back.Model))
                    local a1 = world3:CreateActor(ve:get_link())
                    mark('G33b CreateActor=' .. tostring(a1))
                    if a1 then
                        a1:set_position(-150, 0, 0)
                        a1:play('Idle')
                    end
                    local SCE3 = ImportSCEContext(nil)
                    local a2 = SCE3.ModelActor.new(ve:get_link())
                    mark('G33b ModelActor.new=' .. tostring(a2))
                    if a2 then
                        world3.innerWorld:add_game_actor(a2)
                        a2:set_position({ 150, 0, 0 })
                        a2:show(true)
                        a2:play('Idle')
                    end
                end)

                -- G33c: 虚拟 model 条目 link 喂 set_asset（真实 jilulu 种子 actor）
                try('G33c-virtual-model-setasset', function()
                    local vm = virtual_effect.new('$$p_55a3.model.nezha.root', 'bgd_virtual_nezha2')
                    mark('G33c vm=' .. tostring(vm) .. ' link=' .. tostring(vm and vm:get_link()))
                    if not vm then
                        return
                    end
                    local SCE3 = ImportSCEContext(nil)
                    local ma = SCE3.ModelActor.new('$$p_55a3.actor.bgd_jilulu_attach.root')
                    mark('G33c ma=' .. tostring(ma))
                    if ma then
                        world3.innerWorld:add_game_actor(ma)
                        ma:set_position({ 0, 0, 150 })
                        ma:show(true)
                        ma:play('Idle')
                        ma:set_asset(vm:get_link())
                        mark('G33c set_asset(virtual model link) done')
                    end
                end)
            end)
        end

        -- ★ G34: 模块级 LoadMainMap/reset 可达性 + 表重载验证（xdeditor「强制重新加载项目」机制移植；render-19 模块级函数）
        if ENABLE_G34 then
            base.wait(4000, function()
                -- G34a: 全局可达性 dump（安全）
                try('G34a-globals', function()
                    local names = { 'load_map', 'LoadMainMap', 'reset', 'Reset', 'SaveJson', 'EDITOR', 'load_combined_map', 'load_map_to_cache' }
                    for _, n in ipairs(names) do
                        if _G[n] ~= nil then
                            mark('G34a _G.' .. n .. '=' .. tostring(_G[n]))
                        end
                    end
                    for _, t in ipairs({ 'game', 'base' }) do
                        local tt = _G[t]
                        if type(tt) == 'table' then
                            for _, n in ipairs(names) do
                                if tt[n] ~= nil then
                                    mark('G34a ' .. t .. '.' .. n .. '=' .. tostring(tt[n]))
                                end
                            end
                        end
                    end
                    if type(base) == 'table' and type(base.game) == 'table' then
                        for _, n in ipairs(names) do
                            if base.game[n] ~= nil then
                                mark('G34a base.game.' .. n .. '=' .. tostring(base.game[n]))
                            end
                        end
                    end
                    mark('G34a globals dump done')
                end)
                -- G34b: LoadMainMap 同名调用（中风险；native 日志看是否二次 Begin loading table）
                try('G34b-loadmainmap', function()
                    if type(LoadMainMap) == 'function' then
                        mark('G34b calling LoadMainMap(abs path)...')
                        local r = LoadMainMap('C:/Users/woaye/Documents/SCE Projects/test_res002')
                        mark('G34b ret=' .. tostring(r))
                    elseif type(load_map) == 'function' then
                        mark('G34b calling load_map(abs path)...')
                        local r = load_map('C:/Users/woaye/Documents/SCE Projects/test_res002')
                        mark('G34b ret=' .. tostring(r))
                    else
                        mark('G34b no LoadMainMap/load_map global')
                    end
                end)
            end)
        end

        -- ★ G35: unit_change_model 触发引擎自调 ctx getter（render-06 实证 API；配合 frida 钩 0x181cbcb01 捕 (L,ctx) 直调 LoadMainMap）
        if ENABLE_G35 then
            base.wait(6000, function()
                try('G35-unit-change-model', function()
                    mark('G35 before: ' .. tostring(game.get_unit_model_path(1)))
                    game.unit_change_model(1, 'characters/_user/jilulu_19ec/model.prefab')
                    mark('G35 after: ' .. tostring(game.get_unit_model_path(1)))
                end)
            end)
        end

        -- ★ G36: LoadMainMap(probe_map001) 重载后验证注入条目（lua 读回 + native CreateActor 双测）
        if ENABLE_G36 then
            base.wait(6500, function()
                try('G36-injected-actor', function()
                    local LINK = '$$p_55a3.actor.bgd_injected_nazha.root'
                    local back = base.eff.cache(LINK)
                    mark('G36 cache(' .. LINK .. ')=' .. tostring(back) .. ' NodeType=' .. tostring(back and back.NodeType))
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G36 no world3')
                        return
                    end
                    local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 600)
                    world3:SetCameraPosition(pos[1], pos[2], pos[3])
                    world3:SetCameraRotation(-70, 0, 0)
                    local a = world3:CreateActor(LINK)
                    mark('G36 CreateActor=' .. tostring(a))
                    if a then
                        a:set_position(150, 0, 0)
                        a:play('Idle')
                        mark('G36 injected actor placed at (150,0,0)')
                    end
                end)
            end)
        end

        -- ★ G16: 相机对焦原点 + 吉鲁鲁放原点
        if ENABLE_G16 then
            base.wait(1500, function()
                try('G16-frame', function()
                    local world3 = _G.__gw_world3
                    if not world3 then
                        mark('G16 no world3')
                        return
                    end
                    local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, -70, 0, 0, 300)
                    world3:SetCameraPosition(pos[1], pos[2], pos[3])
                    world3:SetCameraRotation(-70, 0, 0)
                    local a = world3:CreateActor(ACTOR)
                    mark('G16 actor=' .. tostring(a))
                    if a then
                        a:set_position(0, 0, 0)
                        try('G16-play', function()
                            a:play('Idle')
                        end)
                        mark('G16 actor at origin')
                    end
                end)
            end)
        end

        if ENABLE_G12 then
            base.wait(800, function()
                try('G12-setup', function()
                    -- 类来源定位
                    local cls = require('@gameui.component').UIScene
                    mark('G12 require_url=' .. tostring(cls.require_url) .. ' package_url=' .. tostring(cls.package_url) .. ' cls[1]=' .. tostring(cls[1]))
                    -- 裸 UIScene 控件（带布局）
                    local raw = base.ui.view {
                        type = 'UIScene',
                        name = 'gw_raw2',
                        show = true,
                        layout = { width = 600, height = 500, position_type = 'absolute', position = { 600, 200 } },
                    }
                    mark('G12 raw=' .. tostring(raw) .. ' id=' .. tostring(raw and raw.id))
                    if not raw then
                        return
                    end
                    _G.__gw_raw2 = raw
                    local id = raw.id or 'main[gw_raw2]'
                    -- RT 属性名矩阵（每条独立 try；image 吃 RT 有崩溃风险放最后）
                    for _, prop in ipairs({ 'RenderTarget', 'render_target', 'render_target_link' }) do
                        try('G12 prop ' .. prop, function()
                            ui.set_control_prop(id, prop, 'RT:gw_probe')
                            mark('G12 set ' .. prop .. ' done')
                        end)
                    end
                    try('G12 prop RenderPath', function()
                        ui.set_control_prop(id, 'RenderPath', 'EngineRes/RenderPaths/GameSnapshot.xml')
                    end)
                    -- 崩溃风险组：UIScene 的 image 属性吃 RT（G3 普通 image 硬崩的前科）
                    try('G12 prop image=RT', function()
                        ui.set_control_prop(id, 'image', 'RT:gw_probe')
                    end)
                    mark('G12 all set')
                end)
            end)
        end

        if ENABLE_G2 then
            try('G2-create', function()
                mark('UIWorld=' .. tostring(defaultui and defaultui.UIWorld))
                local world = defaultui.UIWorld:Create(false, CAM)
                mark('G2 world=' .. tostring(world) .. ' innerWorld=' .. tostring(world and world.innerWorld))
                -- 相机：用官方 CalculateLensPosition（注视原点，pitch30 yaw0，距离300）
                local pos = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, 30, 0, 0, 300)
                world:SetCameraPosition(pos[1], pos[2], pos[3])
                world:SetCameraRotation(30, 0, 0)
                world:SetViewSize(400, 400)
                world.innerWorld:set_render_target_link('RT:gw_probe')
                try('G2-unit', function()
                    local unit = world:CreateUnit(UNIT)
                    mark('G2 unit=' .. tostring(unit))
                    if unit then
                        unit.set_position(unit, 0, 0, 0)
                    end
                end)
                -- 第二只世界（yaw 180 对照）
                local world2 = defaultui.UIWorld:Create(false, CAM)
                local pos2 = defaultui.UIWorld:CalculateLensPosition(0, 0, 0, 30, 180, 0, 300)
                world2:SetCameraPosition(pos2[1], pos2[2], pos2[3])
                world2:SetCameraRotation(30, 180, 0)
                world2:SetViewSize(300, 300)
                world2.innerWorld:set_render_target_link('RT:gw_probe_img')
                try('G2-unit2', function()
                    local u2 = world2:CreateUnit(UNIT)
                    mark('G2 unit2=' .. tostring(u2))
                end)
                _G.__gw_world1 = world
                _G.__gw_world2 = world2
                mark('G2 worlds ready (RT:gw_probe / RT:gw_probe_img)')
                -- G2 的 RT 接到 G1 scene 控件（BindToUIScene 语义的裸控件复刻尝试）
                if G2_RT_TO_G1 then
                    for _, prop in ipairs({ 'resource', 'render_target', 'RenderTarget' }) do
                        try('G4 scene ' .. prop .. '=RT', function()
                            ui.set_control_prop('main[gw_view]>scene0', prop, 'RT:gw_probe')
                        end)
                    end
                end
                -- ★ G5: 运行时组件框架实例化 UIScene（libs_components 同款路径）+ BindToUIScene
                if ENABLE_G5 then
                    try('G5-component', function()
                        local comp_mod = require('@gameui.component')
                        mark('G5 comp_mod=' .. tostring(comp_mod) .. ' UIScene=' .. tostring(comp_mod and comp_mod.UIScene))
                        local inst = comp_mod.UIScene({
                            RenderPath = 'EngineRes/RenderPaths/GameSnapshot.xml',
                            name = 'gw_uiscene',
                            disabled = false,
                            UseShadow = false,
                        }):new()
                        mark('G5 inst=' .. tostring(inst) .. ' inst.ui=' .. tostring(inst and inst.ui))
                        _G.__gw_uiscene = inst
                        -- 等两帧让控件拿到真实尺寸，再 BindToUIScene
                        base.wait(200, function()
                            try('G5-bind', function()
                                local rect_ok, x, y, w, h = pcall(function()
                                    return inst.ui:rect()
                                end)
                                mark('G5 rect: ' .. tostring(rect_ok) .. ' ' .. tostring(x) .. ',' .. tostring(y) .. ' ' .. tostring(w) .. 'x' .. tostring(h))
                                local r = world:BindToUIScene(inst)
                                mark('G5 BindToUIScene => ' .. tostring(r))
                            end)
                        end)
                    end)
                end
            end)
        end

        -- G1b: 用 set_control_prop 给 G1 scene 控件补 camera/资源变体（id 取自 G1 日志后再试，先用猜的 id 矩阵）
        for _, id in ipairs({ 'main[gw1]>scene0', 'main[gw_view]>scene0', 'main[gw1]>gw1' }) do
            try('G1b camera_info ' .. id, function()
                ui.set_control_prop(id, 'camera_info', CAM)
            end)
        end

        base.event_register(base.game, '游戏-更新', drive)
        mark('frame driving registered')
    end)
end)

return M
