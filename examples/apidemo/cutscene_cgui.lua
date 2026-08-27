local M = {}
-- ============================================================================
-- 全屏过场动画 + 多通道视频对照 demo（完整参考实现）
--
-- 主功能（全屏过场动画，双向桥全用）：
--   画面：webview 全屏 + 自控 HTML（playsinline 内联，iOS 绕过强制全屏播放器）
--   文件：pak 经 io.ExtractPakFile 解到 User/maps/<map>/，webview url=file:// 加载同目录
--         播放器 HTML（同为 file 源，绕 WKWebView/miniblink 的 file 拦截）
--   声音：引擎 GUIVideo 模板写死 muted → 双轨，引擎音效系统播独立 ogg 音轨；
--         用户触摸视频 → JS 取消视频静音 + scelua 通知 lua 停独立音轨（切视频原声）
--   桥：  JS→lua  on_web_message（触摸/播完/跳过/重播回报）
--         lua→JS  run_js 派发自定义事件（指令）
--   UI：  视频上「重播/跳过」（JS 内嵌，webview 无法被 lua 控件覆盖）；
--         播完/跳过 → lua 按钮变「已完播」（JS 真实回报驱动）
--
-- 对照组（研究保留）：
--   A/B 原生 video 控件（PC/Android 直出；iOS 走 webview 内联——原生控件 iOS 强制全屏判死）
--   C  https 在线视频（app-box/CDN 路线对照）
--   桥探针  imgui 通道 JS→lua 链路验证（scelua 注入 + base.ui.map 登记 + register_event）
--   诊断面板  上屏诊断 + 剪贴板导出（iOS 拉不到日志靠它）
--   common 清单  pairs(common) 运行时枚举（517 注册名，见 mini-runtime common-table.md）
--
-- 机制依据：mini-runtime doc/research/pak-io-native.md（§7 iOS 终版结论）
-- ============================================================================

local cg = bgd_api.client.cgui
local sound = bgd_api.client.sound

-- ============================== 诊断收集 ==============================
local DIAG = {}
local function diag(s)
    DIAG[#DIAG + 1] = s
    log.info('[aye] ' .. s)
end
local function diag_text()
    return table.concat(DIAG, '\n')
end

-- ============================== 环境判断 ==============================
local ok_pf, platform = pcall(require, '@base.base.platform')
if not ok_pf then platform = nil end
local IS_EDITOR = common.is_game_play_in_editor and common.is_game_play_in_editor() or false
local IS_IOS = platform and platform.is_ios() or false
local IS_ANDROID = platform and platform.is_android() or false
diag(('env editor=%s ios=%s android=%s platform=%s'):format(
    tostring(IS_EDITOR), tostring(IS_IOS), tostring(IS_ANDROID),
    tostring(common.get_platform and common.get_platform())))

-- ============================== pak 提取 ==============================
local APP_DIR = tostring(common.get_app_dir())
local MAP = tostring(__MAIN_MAP__ or 'p_55a3')
diag('app_dir=' .. APP_DIR)

--- 把地图 pak 里的 entry 解到 User/maps/<map>/，返回绝对路径（编辑器无 pak 时退回项目目录）
local function extract_from_map_pak(entry, out_name)
    if io.ExtractPakFile == nil or io.List == nil or io.ExistsFile == nil then
        diag('FATAL: PascalCase io 不存在')
        return nil
    end
    local dest = APP_DIR .. 'User/maps/' .. MAP .. '/' .. out_name
    if io.ExistsFile(dest) then return dest end
    for _, upd in ipairs({ 'Update', 'update' }) do
        local le, dirs = io.List(APP_DIR .. upd, 2)
        if le == 0 and dirs then
            for _, envdir in ipairs(dirs) do
                local pak = envdir .. '/Res/maps/' .. MAP .. '/' .. MAP .. '.pak'
                if io.ExistsFile(pak) then
                    local r = io.ExtractPakFile(pak, entry, dest)
                    if r == 0 and io.ExistsFile(dest) then
                        diag('extracted=' .. dest)
                        return dest
                    end
                end
            end
        end
    end
    return nil
end

local function resolve_video(res_file)
    local extracted = extract_from_map_pak('res/' .. res_file, res_file)
    if extracted then return extracted end
    if IS_EDITOR then return game.GetMapPath() .. '/res/' .. res_file end
    return nil
end

local PATH_A = resolve_video('shenyi.mp4')
local PATH_B = resolve_video('leishen.mp4')
diag('PATH_A=' .. tostring(PATH_A))
diag('PATH_B=' .. tostring(PATH_B))

-- ============================== 播放器 HTML 模板 ==============================
-- 画面适配：object-fit 统一 cover（填满裁多余方向）。
--   PC 竖屏/窗口比例也照 cover——contain 在任何平台都会留黑（PC 上下黑块实测）。
--   cover 溢出裁切消除舍入缝；视频元素放大 2%（scale(1.02)）消除 iOS 右缘 ~2px 透出。
-- 中文按钮乱码根因：播放器文件 io.write 写盘无 <meta charset>，浏览器按 latin-1 解码 → 必加。
-- 文字不可选：user-select:none 全套前缀（桥探针 web 文字 PC 实测可被框选）。
local VIDEO_FIT = 'cover'
local CSS_NOSELECT = '-webkit-user-select:none;user-select:none;-webkit-touch-callout:none'

-- 全屏版：内嵌「重播/跳过」按钮；触摸取消静音并回报；ended 真实回报；收 lua GlobalEvent 指令
local function player_html(video_file)
    return '<html><head>'
        .. '<meta charset="UTF-8">'
        .. '<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no">'
        .. '<style>'
        .. 'html,body{margin:0;padding:0;width:100%;height:100%;overflow:hidden;background:#000;touch-action:none;' .. CSS_NOSELECT .. '}'
        .. '#v{width:100%;height:100%;object-fit:' .. VIDEO_FIT .. ';display:block;transform:scale(1.02);opacity:0;' .. CSS_NOSELECT .. '}'
        .. '*{-webkit-touch-callout:none}' -- iOS 长按放大镜（callout）全元素禁掉
        .. '.bar{position:fixed;top:16px;right:16px;display:flex;gap:10px;z-index:100}'
        .. '.btn{padding:10px 18px;background:rgba(0,0,0,.6);color:#fff;border:1px solid #fff;'
        .. 'border-radius:6px;font-size:16px;-webkit-tap-highlight-color:transparent;' .. CSS_NOSELECT .. '}'
        .. '</style></head><body>'
        .. '<video id="v" webkit-playsinline playsinline muted autoplay src="' .. video_file .. '"></video>'
        .. '<div class="bar">'
        .. '<button class="btn" id="replay">重播</button>'
        .. '<button class="btn" id="skip">跳过</button>'
        .. '</div>'
        .. '<script>'
        .. 'var v=document.getElementById("v");'
        -- 安卓 WebView 对无 poster 的 <video> 首帧解码前画默认占位图（灰底+大播放三角，闪 ~0.5s）；
        --   起手 opacity:0 隐藏，playing 事件（首帧真正上屏）再显示——占位图整段不画
        .. 'v.addEventListener("playing",function(){v.style.opacity=1});'
        .. 'function send(t){try{window.scelua.send_string(JSON.stringify({type:t}))}catch(e){}}'
        .. 'var un=function(){v.muted=false;v.volume=1;v.play();send("video_touch")};'
        .. 'v.addEventListener("touchstart",un,{once:true});'
        .. 'v.addEventListener("mousedown",un,{once:true});'
        .. 'v.addEventListener("ended",function(){send("video_ended")});'
        -- 重播/跳过：click 之外补 touchstart（移动端 WKWebView click 有 300ms 延迟/可能不触发）；
        --   且重播必须同时复位 muted（用户若触摸过视频已取消静音，重播独立音轨会重启 → 避免双声）
        .. 'var onReplay=function(e){e.stopPropagation();v.muted=true;v.currentTime=0;v.play();send("video_replay")};'
        .. 'var onSkip=function(e){e.stopPropagation();v.pause();send("video_skip")};'
        .. 'document.getElementById("replay").addEventListener("click",onReplay);'
        .. 'document.getElementById("replay").addEventListener("touchstart",onReplay);'
        .. 'document.getElementById("skip").addEventListener("click",onSkip);'
        .. 'document.getElementById("skip").addEventListener("touchstart",onSkip);'
        .. 'window.addEventListener("GlobalEvent",function(e){var m=e.detail.message;'
        .. 'if(m.type=="replay"){v.currentTime=0;v.play()}'
        .. 'else if(m.type=="close"){v.pause()}});'
        .. '</script></body></html>'
end

-- 内嵌区简易版（无控制按钮，仅触摸取消静音回报）
local function inline_html(video_file)
    return '<html><head>'
        .. '<meta charset="UTF-8">'
        .. '<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no">'
        .. '<style>html,body{margin:0;padding:0;width:100%;height:100%;overflow:hidden;background:#000;touch-action:none}'
        .. 'video{width:100%;height:100%;object-fit:' .. VIDEO_FIT .. ';display:block;transform:scale(1.01);opacity:0}</style>'
        .. '</head><body>'
        .. '<video id="v" webkit-playsinline playsinline muted autoplay src="' .. video_file .. '"></video>'
        .. '<script>var v=document.getElementById("v");'
        -- 安卓首帧前占位图（灰底+大播放三角）隐藏：playing 事件再显示
        .. 'v.addEventListener("playing",function(){v.style.opacity=1});'
        .. 'var un=function(){v.muted=false;v.volume=1;v.play();'
        .. 'try{window.scelua.send_string(JSON.stringify({type:"video_touch"}))}catch(e){}};'
        .. 'v.addEventListener("touchstart",un,{once:true});'
        .. 'v.addEventListener("mousedown",un,{once:true});'
        .. '</script></body></html>'
end

-- 播放器 HTML 写成文件放在 mp4 旁（沙箱版 io.write 恰好落 User/maps/<map>/ 同目录），
-- webview 用 url=file:// 加载（页面与视频同为 file 源即可读，绕 WKWebView/miniblink 拦截）。
-- 模板内容变化时重写（版本演进自愈）。
local function write_player(player_name, html)
    local hp = APP_DIR .. 'User/maps/' .. MAP .. '/' .. player_name
    local _, old = io.Read(hp)
    if old ~= html then
        io.write(player_name, html)
        diag('player_write=' .. hp .. ' exist=' .. tostring(io.ExistsFile(hp)))
    end
    return 'file://' .. hp
end
local function player_url(video_path, video_file, player_name)
    if not video_path then return nil end
    return write_player(player_name, player_html(video_file))
end
local function inline_player_url(video_path, video_file, player_name)
    if not video_path then return nil end
    return write_player(player_name, inline_html(video_file))
end

-- ============================== 双轨音频 ==============================
-- 原生 video 控件模板写死 muted → 声音统一由引擎音效系统播独立音轨（pak 资源路径免解 pak）
local VIDEO_AUDIO_TYPE = 'video_audio'
local VIDEO_AUDIO = {
    a = 'src/res/sound/video_shenyi.ogg',  -- 构建时改写为运行时路径
    b = 'src/res/sound/video_leishen.ogg',
}
local function audio_start(which)
    local p = VIDEO_AUDIO[which]
    if not p then return end
    sound.Sound.new(p, 100, false, 0, VIDEO_AUDIO_TYPE):play()
    diag('audio_start=' .. p)
end
local function audio_stop()
    sound.Stop(VIDEO_AUDIO_TYPE)
end

-- ============================== webview 桥探针：手势事件测试台 ==============================
-- 蓝色区做手势 → cgui 注入的捕获脚本经 on_web_event 回传（press/release/click/
-- double_click/long_press/move），蓝色区顶部实时刷最近事件，lua 诊断同步记录；
-- lua 按钮「让Web背景闪烁」经 run_js 演示 lua→JS。
local BRIDGE = { last_ev = '-', js_cmd = nil, devtools = false }
local function bridge_html()
    return '<html><head>'
        .. '<meta charset="UTF-8">'
        .. '<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no">'
        .. '<style>html,body{margin:0;width:100%;height:100%;background:#234;color:#fff;overflow:hidden;touch-action:none;' .. CSS_NOSELECT .. '}'
        .. '#t{display:flex;flex-direction:column;width:100%;height:100%;align-items:center;justify-content:center;font-size:20px}'
        .. '#ev{font-size:40px;font-weight:bold;color:#8f8}</style>'
        .. '</head><body><div id="t"><div id="ev">做手势</div>'
        .. '<div>点/双击/长按/拖动（JS→Lua 全事件）</div></div>'
        -- JS 侧自检 + 最近事件显示。自检：首帧把 scelua 类型写上屏（确认注入）。
        -- 捕获脚本（cgui 注入）每个手势也会调 __show 更新标题。
        .. '<script>window.__show=function(t){document.getElementById("ev").textContent=t};'
        .. 'console.log("bridge page loaded, scelua=" + typeof scelua);' -- console 转发探针
        .. 'document.getElementById("ev").textContent="scelua="+typeof scelua;'
        .. 'window.addEventListener("GlobalEvent",function(e){'
        .. 'if(e.detail.message.type=="flash"){document.body.style.background="#456";'
        .. 'setTimeout(function(){document.body.style.background="#234"},150)}});'
        .. '</script></body></html>'
end
local function bridge_probe_draw()
    -- lua→JS：待发的闪烁指令经 run_js 下发
    local rj = nil
    if BRIDGE.js_cmd then
        rj = 'window.dispatchEvent(new CustomEvent("GlobalEvent",{detail:{message:{type:"flash"}}}));'
        BRIDGE.js_cmd = nil
    end
    cg.webview('aye_bridge_wv', {
        html = bridge_html(),
        run_js = rj,
        web_dev_tools = BRIDGE.devtools, -- lua 控制 devtools 开关
        web_console_log = true,          -- JS console → lua log（直接看 JS 报错）
        -- 全事件回调：cgui 注入的捕获脚本把手势包成 __wvev 消息路由到这里
        on_web_event = function(ev)
            local desc = ev.type
                .. (ev.step_x and (' 步进 ' .. math.floor(ev.step_x) .. ',' .. math.floor(ev.step_y)) or '')
                .. (ev.delta_x and ('  累计 ' .. math.floor(ev.delta_x) .. ',' .. math.floor(ev.delta_y)) or '')
                .. (ev.duration_ms and ('  ' .. ev.duration_ms .. 'ms') or '')
            BRIDGE.last_ev = desc
            diag('WEV ' .. desc)
        end,
        layout = { width = 960, height = 270 }, -- 拉长一倍
    })
end

-- ============================== JS→lua 消息（视频） ==============================
local PLAYING = false   -- 全屏播放中
local FINISHED = false  -- 完播（JS ended 真实回报 / 跳过也算完播）

local function on_web_message(message)
    diag('wv_msg=' .. tostring(message))
    local ok, obj = pcall(base.json.decode, message)
    if not (ok and obj) then return end
    if obj.type == 'video_touch' then
        audio_stop() -- 视频原声接管
        diag('audio_stop=touch')
    elseif obj.type == 'video_ended' or obj.type == 'video_skip' then
        PLAYING = false
        FINISHED = true -- 播完与跳过都算完播
        audio_stop()
        diag('finished=' .. obj.type)
    elseif obj.type == 'video_replay' then
        FINISHED = false
        -- 触摸过视频后原声接管（独立音轨已停）；重播时若视频已被取消静音则 JS 侧复位 muted，
        -- 否则独立音轨 + 视频原声双声。重播统一回独立音轨。
        audio_start('a')
        diag('replay -> audio restart')
    end
end

-- ============================== common 表清单（研究用） ==============================
local COMMON_LIST = {}
for k, v in pairs(common) do
    COMMON_LIST[#COMMON_LIST + 1] = k .. ':' .. type(v)
end
table.sort(COMMON_LIST)

-- ============================== UI 状态 ==============================
local IS_SHOW_A = false
local IS_SHOW_B = false
local IS_SHOW_C = false
local IS_SHOW_DIAG = false
local IS_SHOW_BRIDGE = false

-- 按钮布局参数：移动端整体放大（字号 + 按钮高度/内边距）+ 向内偏移避开刘海/挖孔
local IS_MOBILE = IS_IOS or IS_ANDROID
local BTN_FONT = IS_MOBILE and { size = 34 } or nil
local BTN_OFF = IS_MOBILE and 120 or 20 -- 距左/上边缘偏移（刘海）
local BTN_GAP = IS_MOBILE and 20 or 8
-- 按钮尺寸：字号之外按钮本身也要放大（高度 + 内边距撑大点击热区）
local function BTN_OPTS()
    if not IS_MOBILE then return nil end
    return {
        font = BTN_FONT,
        layout = { height = 88, padding = { left = 28, right = 28, top = 8, bottom = 8 } },
    }
end

local VIDEO_C = 'https://laf.codejoy.games/hanm3j-cloud-bin/shenyi.mp4' -- 线上 http 对照

cg.mount('aye_video_test', function()
    -- 左上按钮列（移动端大字号 + 内移避刘海）
    cg.pin('tl', BTN_OFF, BTN_OFF, function()
        cg.col(function()
            -- 主功能：全屏过场动画
            cg.button_primary(
                PLAYING and '播放中…' or (FINISHED and '已完播' or '全屏播放'),
                function()
                    if PLAYING then return end
                    PLAYING = true
                    FINISHED = false
                    audio_start('a') -- 独立音轨直出（视频静音起手）
                    diag('play start')
                end,
                'aye_btn_play',
                BTN_OPTS()
            )
            cg.spacer(BTN_GAP)
            -- 对照组
            cg.button_ghost(IS_SHOW_A and '隐藏A' or '播放A(原生video)', function()
                IS_SHOW_A = not IS_SHOW_A
                if IS_SHOW_A then audio_start('a') else audio_stop() end
            end, 'aye_btn_a', BTN_OPTS())
            cg.spacer(BTN_GAP)
            cg.button_ghost(IS_SHOW_B and '隐藏B' or '播放B(原生video)', function()
                IS_SHOW_B = not IS_SHOW_B
                if IS_SHOW_B then audio_start('b') else audio_stop() end
            end, 'aye_btn_b', BTN_OPTS())
            cg.spacer(BTN_GAP)
            cg.button_ghost(IS_SHOW_C and '隐藏线上' or '播放线上(https)', function()
                IS_SHOW_C = not IS_SHOW_C
            end, 'aye_btn_c', BTN_OPTS())
            cg.spacer(BTN_GAP)
            cg.button_ghost(IS_SHOW_BRIDGE and '关桥探针' or '桥探针', function()
                IS_SHOW_BRIDGE = not IS_SHOW_BRIDGE
            end, 'aye_btn_bridge', BTN_OPTS())
            if IS_SHOW_BRIDGE then
                cg.spacer(BTN_GAP)
                -- lua→JS：让 web 蓝色区闪烁（演示 lua→JS run_js 指令）
                cg.button_primary('Lua让Web闪烁', function()
                    BRIDGE.js_cmd = true
                end, 'aye_btn_bridge_bump', BTN_OPTS())
                cg.spacer(BTN_GAP)
                -- lua 控制 devtools 开关（引擎原生 web_dev_tools 属性）
                cg.button_ghost(BRIDGE.devtools and '关DevTools' or '开DevTools', function()
                    BRIDGE.devtools = not BRIDGE.devtools
                    diag('devtools=' .. tostring(BRIDGE.devtools))
                end, 'aye_btn_devtools', BTN_OPTS())
                cg.text('最近手势: ' .. BRIDGE.last_ev, 'aye_bridge_last', { font = BTN_FONT })
            end
            cg.spacer(BTN_GAP)
            cg.button_ghost(IS_SHOW_DIAG and '隐藏诊断' or '显示诊断', function()
                IS_SHOW_DIAG = not IS_SHOW_DIAG
            end, 'aye_btn_diag', BTN_OPTS())
        end)
    end)

    -- 全屏过场动画层（webview + 播放器文件 + 内嵌控制按钮 + 双向桥）
    -- 黑底垫层：webview 右缘像素舍入缝会透出下层（疑引擎 webview 物理像素对齐 bug），
    --   在 webview 下垫全屏纯黑 panel 挡住透出
    if PLAYING then
        -- fullscreen 本身是 direction=col 的 panel：给它纯黑底色当垫层挡右缘透出，
        -- 单个子元素 webview 填满（不能再套 box 放两个子元素——col 布局会纵向排列把视频顶下半屏）
        cg.fullscreen(function()
            cg.webview('aye_player', {
                url = player_url(PATH_A, 'shenyi.mp4', 'player_a.html'),
                on_web_message = on_web_message,
                layout = { width = 1, height = 1, width_grow = 1, height_grow = 1 },
            })
        end, nil, { color = 'rgba(0,0,0,1)' })
    end

    -- 对照组视频区（屏幕中心）
    if IS_SHOW_A or IS_SHOW_B or IS_SHOW_C then
        cg.pin('cc', 0, 0, function()
            cg.row(function()
                if IS_SHOW_A then
                    if IS_IOS then
                        cg.webview('aye_video_a', {
                            url = inline_player_url(PATH_A, 'shenyi.mp4', 'inline_a.html'),
                            on_web_message = on_web_message,
                            layout = { width = 640, height = 360 },
                        })
                    else
                        cg.video(PATH_A, 'video_a', { layout = { width = 640, height = 360 } })
                    end
                end
                if IS_SHOW_B then
                    if IS_IOS then
                        cg.webview('aye_video_b', {
                            url = inline_player_url(PATH_B, 'leishen.mp4', 'inline_b.html'),
                            on_web_message = on_web_message,
                            layout = { width = 640, height = 360 },
                        })
                    else
                        cg.video(PATH_B, 'video_b', { layout = { width = 640, height = 360 } })
                    end
                end
                if IS_SHOW_C then
                    cg.video(VIDEO_C, 'video_c', { layout = { width = 640, height = 360 } })
                end
            end)
        end)
    end

    -- 桥探针区（屏幕正中）
    if IS_SHOW_BRIDGE then
        cg.pin('cc', 0, 0, function()
            bridge_probe_draw()
        end)
    end

    -- 诊断面板（左下，iOS 拉不到日志靠它 + 剪贴板；移动端内移避刘海）
    if IS_SHOW_DIAG then
        cg.pin('bl', BTN_OFF, -BTN_OFF, function()
            cg.col(function()
                cg.button_primary('复制诊断到剪贴板', function()
                    common.copy_to_clipboard(diag_text())
                    diag('copied, lines=' .. #DIAG)
                end, 'aye_btn_copy', BTN_OPTS())
                cg.spacer(BTN_GAP)
                cg.button_ghost('复制common清单', function()
                    common.copy_to_clipboard(table.concat(COMMON_LIST, '\n'))
                    diag('common copied, n=' .. #COMMON_LIST)
                end, 'aye_btn_copy_common', BTN_OPTS())
                cg.spacer(8)
                cg.box(function()
                    cg.scroll(function()
                        cg.text(diag_text(), 'aye_diag_text')
                    end)
                end, { layout = { width = 560, height = 300 } })
            end)
        end)
    end
end, { root_z = 1000 })

return M
