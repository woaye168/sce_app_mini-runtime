-- 渲染底层 API 探针（研究任务探针；2026-08-25 自 test_res002 .bgd/src/client 转移固化；勿发布正式版）
-- 本轮：U28 imgui video 控件 http mp4 实播测试（render-03 遗留：file:// 被 CEF 拦，http 待测）
local M = {}

local function mark(s)
    log.info('[RenderProbe] ' .. s)
end

-- local VIDEO_URL = 'http://127.0.0.1:8899/sample.mp4'
local VIDEO_URL = 'https://oss.laf.run/h63emw-cloud-bin/%E7%A5%9E%E6%84%8F.mp4'
local WV_HTML = require('src.client.RenderProbeHtml')
local started = false
local wv_started = false

base.event_register(base.game, '游戏-更新', function()
    ui.imgui_begin_view('main', 'my_view')
    if ui.imgui_begin_ui('video', 'my_video') then
        ui.imgui_props(true, VIDEO_URL, function(show, url)
            if not started then
                started = true
                mark('U28 video imgui driving, video_url=' .. url)
            end
            return {
                show = show, video_url = url,
                layout = { width = 640, height = 360, position_type = 'absolute', position = { 300, 200 } },
            }
        end)
        ui.imgui_end_ui('video', 'my_video')
    end
    -- U29 webview imgui 线上验证（render-05 遗留：编辑器 PIE 可渲染，线上 tester 未验证）
    if ui.imgui_begin_ui('webview', 'my_wv') then
        ui.imgui_props(true, WV_HTML, function(show, html)
            if not wv_started then
                wv_started = true
                mark('U29 webview imgui driving, html_len=' .. #html)
            end
            return {
                show = show, html = html,
                layout = { width = 400, height = 300, position_type = 'absolute', position = { 1000, 300 } },
            }
        end)
        ui.imgui_end_ui('webview', 'my_wv')
    end
    ui.imgui_end_view('main', 'my_view')
end)

return M
