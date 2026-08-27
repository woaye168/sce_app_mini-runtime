-- ============================================================================
-- 全屏过场动画 —— 原生 imgui 版（不依赖 cgui 框架）
-- ----------------------------------------------------------------------------
-- 与 cutscene_cgui.lua 同功能，但直接走引擎 imgui 三段式（begin_ui/props2/state/end_ui）。
-- 用途：演示 cgui 之下 webview 双向桥的原生接法——所有「框架自动做的事」这里都手写。
--
-- 关键差异（cgui 帮你做的，imgui 版要自己写）：
--   1. 视图生命周期：cg.mount 每帧驱动 → imgui 要自己在 on_post_update 里 begin/end_view
--   2. props 下发：cg.webview 收 opts 表 → imgui.props2(fn, sig) 函数形式 + 内容签名
--   3. 状态读取：cg 在 begin 块内 core.state() → imgui 要 begin_ui 后、end_ui 前 imgui.state()
--   4. JS→lua 桥登记：cgui on_web_message 自动登记 → imgui 版手写 base.ui.map + register_event
--
-- 机制依据：mini-runtime doc/research/webview-bridge.md
-- ============================================================================

local M = {}

local imgui = require '@appui.imgui'  -- 引擎原生（cgui 之外）

-- ============================== pak 提取 ==============================
local APP_DIR = tostring(common.get_app_dir())
local MAP = tostring(__MAIN_MAP__ or 'p_55a3')

local function resolve_video(res_file)
    -- 线上从 pak 解出；编辑器退回项目目录
    if io.ExtractPakFile then
        local dest = APP_DIR .. 'User/maps/' .. MAP .. '/' .. res_file
        if io.ExistsFile(dest) then return dest end
        for _, upd in ipairs({ 'Update', 'update' }) do
            local le, dirs = io.List(APP_DIR .. upd, 2)
            if le == 0 and dirs then
                for _, envdir in ipairs(dirs) do
                    local pak = envdir .. '/Res/maps/' .. MAP .. '/' .. MAP .. '.pak'
                    if io.ExistsFile(pak) then
                        if io.ExtractPakFile(pak, 'res/' .. res_file, dest) == 0 then return dest end
                    end
                end
            end
        end
    end
    if common.is_game_play_in_editor and common.is_game_play_in_editor() then
        return game.GetMapPath() .. '/res/' .. res_file
    end
    return nil
end

local VIDEO_PATH = resolve_video('shenyi.mp4')

-- ============================== 播放器 HTML ==============================
-- 注意 <meta charset>（写盘无 BOM 否则中文乱码）/ playsinline（iOS 内联）/
-- cover + scale(1.02)（填满 + 消除右缘舍入缝）/ 全套 user-select（禁长按放大镜与文字选择）
local function player_html(video_file)
    return '<html><head>'
        .. '<meta charset="UTF-8">'
        .. '<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no">'
        .. '<style>html,body{margin:0;padding:0;width:100%;height:100%;overflow:hidden;background:#000;touch-action:none;'
        .. '-webkit-user-select:none;user-select:none;-webkit-touch-callout:none}'
        .. '#v{width:100%;height:100%;object-fit:cover;display:block;transform:scale(1.02);opacity:0}'
        .. '.bar{position:fixed;top:16px;right:16px;display:flex;gap:10px;z-index:100}'
        .. '.btn{padding:10px 18px;background:rgba(0,0,0,.6);color:#fff;border:1px solid #fff;border-radius:6px;font-size:16px}'
        .. '</style></head><body>'
        .. '<video id="v" webkit-playsinline playsinline muted autoplay src="' .. video_file .. '"></video>'
        .. '<div class="bar"><button class="btn" id="replay">重播</button><button class="btn" id="skip">跳过</button></div>'
        .. '<script>var v=document.getElementById("v");'
        -- 安卓 WebView 首帧解码前画默认占位图（灰底+大播放三角，闪 ~0.5s）：opacity:0 起手，playing 再显示
        .. 'v.addEventListener("playing",function(){v.style.opacity=1});'
        .. 'function send(t){try{window.scelua.send_string(JSON.stringify({type:t}))}catch(e){}}'
        .. 'var un=function(){v.muted=false;v.volume=1;v.play();send("video_touch")};'
        .. 'v.addEventListener("touchstart",un,{once:true});v.addEventListener("mousedown",un,{once:true});'
        .. 'v.addEventListener("ended",function(){send("video_ended")});'
        .. 'document.getElementById("replay").addEventListener("click",function(e){e.stopPropagation();v.muted=true;v.currentTime=0;v.play();send("video_replay")});'
        .. 'document.getElementById("skip").addEventListener("click",function(e){e.stopPropagation();v.pause();send("video_skip")});'
        .. 'window.addEventListener("GlobalEvent",function(e){var m=e.detail.message;'
        .. 'if(m.type=="replay"){v.currentTime=0;v.play()}else if(m.type=="close"){v.pause()}});'
        .. '</script></body></html>'
end

-- 播放器 HTML 写到 mp4 旁（io.write 沙箱恰好落 User/maps/<map>/ 同目录），url=file:// 加载
local function player_url(video_path, video_file, player_name)
    if not video_path then return nil end
    local hp = APP_DIR .. 'User/maps/' .. MAP .. '/' .. player_name
    local html = player_html(video_file)
    local _, old = io.Read(hp)
    if old ~= html then io.write(player_name, html) end
    return 'file://' .. hp
end

-- ============================== 双轨音频 ==============================
-- 原生 video 控件模板写死 muted → 引擎音效系统播独立音轨（pak 资源路径免解 pak）
local function audio_start(path)
    ui_sound.play_sound(path, 100, false, 0, 'video_audio')
end
local function audio_stop()
    ui_sound.stop_sound('video_audio')
end
local AUDIO_PATH = 'src/res/sound/video_shenyi.ogg' -- 构建时改写运行时路径

-- ============================== 状态 + JS→lua 桥 ==============================
local PLAYING = false
local FINISHED = false
local hooked_id = nil

local function on_web_message(message)
    local ok, obj = pcall(base.json.decode, message)
    if not (ok and obj) then return end
    if obj.type == 'video_touch' then
        audio_stop()
    elseif obj.type == 'video_ended' or obj.type == 'video_skip' then
        PLAYING = false
        FINISHED = true
        audio_stop()
    elseif obj.type == 'video_replay' then
        FINISHED = false
        audio_start(AUDIO_PATH)
    end
end

-- ★ imgui 通道 JS→lua 桥的关键：把控件 id 登记进 base.ui.map + register_event 订阅。
--   引擎 ui_events.on_web_message 经 base.ui.map[id].event.on_web_message 派发；
--   imgui 建的控件默认不在 map → 手动登记才收得到（cgui 的 on_web_message 就是封装了这一步）
local function hook_bridge(ctrl_id)
    if hooked_id or not (base.ui and base.ui.map and base.ui.gui) then return end
    hooked_id = ctrl_id
    base.ui.map[ctrl_id] = base.ui.map[ctrl_id] or {}
    base.ui.map[ctrl_id].event = { on_web_message = on_web_message }
    pcall(base.ui.gui.register_event, ctrl_id, 'on_web_message')
end

-- ============================== imgui 三段式渲染 ==============================
-- cg.mount 每帧驱动等价物：游戏更新事件里 begin_view/end_view 包住控件树
local VIEW = 'cutscene'
base.game:event('on_post_update', function()
    -- 入口按钮：panel 容器 + state().on_real_click
    if imgui.begin_view('main', VIEW) then
        if imgui.begin_ui('button', 'play_btn') then
            imgui.props2(function()
                return {
                    text = PLAYING and '播放中…' or (FINISHED and '已完播' or '全屏播放'),
                    layout = { position = { 20, 20 }, width = 220, height = 60 },
                }
            end, PLAYING, FINISHED)
            local st = imgui.state()
            if st and st.on_real_click == 1 and not PLAYING then
                PLAYING = true
                FINISHED = false
                audio_start(AUDIO_PATH)
            end
            imgui.end_ui('button', 'play_btn')
        end

        -- 全屏视频层
        if PLAYING then
            if imgui.begin_ui('panel', 'video_layer') then
                imgui.props2(function()
                    return {
                        color = 'rgba(0,0,0,1)', -- 黑底垫层挡 webview 右缘透出
                        layout = { position = { 0, 0 }, width = 1, height = 1, width_grow = 1, height_grow = 1 },
                    }
                end, true)
                if imgui.begin_ui('webview', 'player') then
                    imgui.props2(function()
                        return {
                            url = player_url(VIDEO_PATH, 'shenyi.mp4', 'player_a.html'),
                            layout = { width = 1, height = 1, width_grow = 1, height_grow = 1 },
                        }
                    end, VIDEO_PATH)
                    -- begin 块内读 state 拿真实控件 id，登记桥
                    local st = imgui.state()
                    if st and (st.id or st.name) then
                        hook_bridge(st.id or st.name)
                    end
                    imgui.end_ui('webview', 'player')
                end
                imgui.end_ui('panel', 'video_layer')
            end
        end
        imgui.end_view('main', VIEW)
    end
end)

return M
