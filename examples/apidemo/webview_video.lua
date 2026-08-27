p_lx61 = p_lx61 or {}
local map_self = p_lx61
local component = require '@common.base.gui.component'
local new = component.new
local bind = component.bind
local alias = component.alias
local getset = component.getset
local control_util = require '@common.base.gui.control_util'

function map_self.screen_to_ui(px)
    return px * base.ui.auto_scale.current_scale()
end

function map_self.ui_to_screen(em)
    return em / base.ui.auto_scale.current_scale()
end

local video_html = [[
<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <meta name="HandheldFriendly" content="false">
    <style>
        * {
            margin: 0;
            padding: 0;
            overflow: hidden;
            -webkit-touch-callout: none; /* 禁用长按菜单 */
            -webkit-user-select: none;   /* 禁用文本选择 */
            user-select: none;           /* 标准语法 */
            -webkit-tap-highlight-color: transparent; /* 移除点击高亮 */
        }

        body,
        html {
            height: 100%;
            background: #000;
        }

        #videoContainer {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }

        #fullscreenVideo {
            width: 100%;
            height: 100%;
            object-fit: cover;
        }

        .video-controls {
            position: fixed;
            top: 20px;
            right: 20px;
            display: flex;
            gap: 10px;
            z-index: 100;
        }

        .sound-control {
            position: fixed;
            top: 20px;
            left: 20px;
            z-index: 100;
        }

        .control-btn {
            padding: 8px 16px;
            background-color: rgba(0, 0, 0, 0.7);
            color: white;
            border: 1px solid white;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
            transition: all 0.3s;
        }

        .control-btn:hover {
            background-color: rgba(255, 255, 255, 0.2);
        }
    </style>
</head>

<body>
    <div id="videoContainer">
        <!-- <video id="fullscreenVideo" width="100%" height="100%" webkit-playsinline playsinline
            controlsList="noplaybackrate autoplay nodownload nofullscreen noremoteplayback" disablePictureInPicture="true" muted
            style="object-fit:fill"> -->
        <video id="fullscreenVideo" autoplay preload="auto" width="100%" height="100%" webkit-playsinline playsinline
            controlsList="noplaybackrate nodownload nofullscreen noremoteplayback" disablePictureInPicture="true"
            style="object-fit:fill">
            <source src="https://oss.laf.run/ckjiwl-cloud-bin/神意.mp4" type="video/mp4" />
        </video>

        <!-- 声音控制 -->
        <button class="control-btn sound-control" id="soundBtn">关闭声音</button>

        <!-- 播放控制 -->
        <div class="video-controls">
            <button class="control-btn" id="playBtn">开始</button>
            <button class="control-btn" id="pauseBtn">暂停</button>
            <button class="control-btn" id="replayBtn">重放</button>
            <button class="control-btn" id="closeBtn">X</button>
        </div>
    </div>

    <script>
        const video = document.getElementById('fullscreenVideo')
        const soundBtn = document.getElementById('soundBtn')
        const playBtn = document.getElementById('playBtn')
        const pauseBtn = document.getElementById('pauseBtn')
        const replayBtn = document.getElementById('replayBtn')
        const closeBtn = document.getElementById('closeBtn')

        // 播放视频
        function playVideo(){
            video.currentTime = 0
            video.play()
        }

        // 停止视频
        function stopVideo(){
            video.currentTime = 0
            video.pause()
        }

        // 发送消息给Lua
        function sentToLua(...arg){
            const [type, msgId, content] = arg
            const message = {type, msgId, content}
            window.scelua.send_string(JSON.stringify(message))
        }

        // 声音控制
        soundBtn.addEventListener('click', () => {
            video.muted = !video.muted
            soundBtn.textContent = video.muted ? '开启声音' : '关闭声音'
        });

        // 播放控制
        playBtn.addEventListener('click', () => video.play())
        pauseBtn.addEventListener('click', () => video.pause())
        replayBtn.addEventListener('click', () => {
            console.log('点击了重放按钮')
            playVideo()
        })

        // 关闭控制
        closeBtn.addEventListener('click', () => {
            console.log('点击了关闭按钮')
            stopVideo()
            sentToLua('stop_video')
        })

        // 监听网页加载完毕
        document.addEventListener('DOMContentLoaded', function() {
            console.log('DOM 加载完成')
            sentToLua('DOMContentLoaded')
        })

        // 创建全局监听,监听来自lua的消息
        window.addEventListener('GlobalEvent', event => {
            console.log(event)

            // 获取传递的数据
            const msg = event.detail.message
            const type = msg.type
            const msgId = msg.msgId
            const content = msg.content

            // 根据消息类型处理数据
            switch (type) {
                case 'play_video':
                    console.log('收到lua msg : play_video')
                    playVideo()
                    break
                    
                // case 'replay_video':
                //     // console.log(`${content} | ${String(msgId)} --${get_time(new Date())}`)
                //     console.log('收到lua msg : replay_video')
                //     video.load()
                //     video.play()
                //     break

                // case 'stop_video':
                //     console.log('收到lua msg : stop_video')
                //     video.stop()
                //     break

                default:
                    console.warn('未知类型:', type)
            }
        })
    </script>
</body>
</html>
]]

-- 正常编辑器启动项目
-- D:\SceOnline\星火编辑器.exe -inner -file_path="D:\SceOnline\Res\maps\dating_test_01\project.sce"
-- 测试编辑器启动项目
-- D:\SceOnline\星火编辑器.exe -inner -file_path="D:\星火对战平台PC_线上_250613\Win\update\e.production.spark.xd.com_test\Res\maps\p_lx61\project.sce"

local MyVideo = component 'MyVideo' {
    base.ui.panel {
        show = bind.show,
        color = 'rgba(0, 0, 0, 1)',
        layout = {
            width = 1,
            height = 1,
            direction = 'row',
            row_content = 'start',
            col_content = 'start',
            width_grow = 1,
            height_grow = 1,
            width_shrink = 1,
            height_shrink = 1,
        },
        -- 由于无法覆盖到webview的全屏video上，所有直接有webview中js来控制，这里注销
        -- base.ui.panel 'ctrl_video' {
        --     color = 'rgba(0, 0, 0, 1)',
        --     layout = {
        --         width = -1,
        --         height = 100,
        --         width_grow = 1,
        --         width_shrink = 1,
        --         row_self = 'end',
        --         col_self = 'start',
        --         direction = 'row',
        --         row_content = 'end',
        --         col_content = 'center',
        --         margin = {
        --             top = 30,
        --         },
        --         padding = {
        --             left = 120,
        --             right = 120,
        --         },
        --     },
        --     base.ui.button 'btn_replay' {
        --         color = 'rgba(91, 52, 247, 1)',
        --         layout = {
        --             width = 120,
        --             height = 80,
        --             margin = {
        --                 left = 20,
        --             },
        --         },
        --     },
        --     base.ui.button 'btn_skip' {
        --         color = 'rgba(223, 23, 219, 1)',
        --         layout = {
        --             width = 120,
        --             height = 80,
        --             margin = {
        --                 left = 20,
        --             },
        --         },
        --     },
        -- },
        base.ui.webview 'webview' {
            layout = {
                width = 1,
                height = 1,
                width_grow = 1,
                height_grow = 1,
                width_shrink = 1,
                height_shrink = 1,
            },
            html = video_html,
            -- run_js = '', 
            -- url = 'start-game://p_1ax1',
            web_dev_tools = true,

            event = {
                on_web_message = bind.on_web_message,
            },
        },
    },
    method = {
        init = function(self)
            self.on_web_message = function(message)
                -- 调试看参数用
                -- map_self:io_write('web_msg.txt', map_self:json_encode(message))
                log.info('js2lua message :', map_self:json_encode(message))

                -- 先把收到的消息反序列化
                local msg_obj = base.json.decode(message)

                -- 拿到消息类型
                local type = msg_obj.type

                -- 根据消息类型处理事件
                if type == 'DOMContentLoaded' then
                    log.info('Lua 收到 DOMContentLoaded')
                    self:play_video()
                elseif type == 'stop_video' then
                    log.info('Lua 收到 stop_video')
                    self['@.show'] = false
                end
            end

            -- -- 设置webview全屏，无视刘海屏的空白区
            -- local safe_insets = base.screen:get_safe_insets()
            -- local safe_top = safe_insets['top']
            -- local safe_bottom = safe_insets['bottom']
            -- local safe_left = safe_insets['left']
            -- local safe_right = safe_insets['right']

            -- local screen_width, screen_height = base.screen:get_resolution()
            -- self['@.layout.width'] = map_self.ui_to_screen(screen_width)
            -- self['@.layout.height'] = map_self.ui_to_screen(screen_height)
            -- self['@webview.layout.width'] = map_self.ui_to_screen(screen_width) + safe_left + safe_right
            -- self['@webview.layout.height'] = map_self.ui_to_screen(screen_height) + safe_top + safe_bottom

            -- -- 不知道为什么，变成负数 video 就会完全不显示
            -- -- if safe_left ~= 0 then
            -- --     self['@webview.layout.margin.left'] = -safe_left
            -- -- end

            -- -- if safe_top ~= 0 then
            -- --     self['@webview.layout.margin.top'] = -safe_top
            -- -- end

            log.info(base.json.encode(base.screen:get_safe_insets()))
            lib_gamechat_2:gamechatclient_send_message('安全边距: ' .. base.json.encode(base.screen:get_safe_insets()))
            base.wait(1000, function()
                lib_gamechat_2:gamechatclient_send_message('控件宽高: ' .. self['@webview.layout.width'] .. ' | ' .. self['@webview.layout.height'])
                lib_gamechat_2:gamechatclient_send_message('真实位置: ' .. base.json.encode(base.gui_get_rect(self['@webview'])))
            end)

            -- {
            --     "bottom":0,
            --     "left":0,
            --     "right":0,
            --     "top":0
            -- }

            -- function map_self.screen_to_ui(px)
            --     return px * base.ui.auto_scale.current_scale()
            -- end

            -- function map_self.ui_to_screen(em)
            --     return em / base.ui.auto_scale.current_scale()
            -- end

            -- -- 由于无法覆盖到webview的全屏video上，所有直接有webview中js来控制，这里注销
            -- -- 重放视频
            -- self['@btn_replay.event.on_click'] = function()
            --     self:sendWeb({
            --         type = 'replay_video',
            --         msgId = 1,
            --         content = self['@.video_src'],
            --     })
            -- end

            -- -- 跳过视频
            -- self['@btn_close.event.btn_skip'] = function()
            --     -- 停止视频
            --     self:sendWeb({
            --         type = 'stop_video',
            --         msgId = 2,
            --         content = self['@.video_src'],
            --     })
            --     -- 关闭视频层
            --     self.show = false
            -- end
        end,

        -- 这个方法构建lua发送消息给js的标准形式
        -- 如果改了这里，js里的接收也需要相应调整
        send_web = function(self, msg)
            local send_str = string.format('window.dispatchEvent(new CustomEvent(\'GlobalEvent\',{ detail: { message: %s } }))', base.json.encode(msg))
            log.info('send_str:', send_str)
            -- 用全局事件发给js，正式环境js会做混淆
            self['@webview.run_js'] = send_str
        end,

        play_video = function(self)
            self:send_web({
                type = 'play_video',
                msgId = 1,
                content = '',
            })
        end,
    },
    prop = {
        show = getset {
            get = function(self) -- 使用 data 存储属性数据
                return self.data.show or true
            end,
            set = function(self, v)
                if self.data.show == v then
                    return false
                end

                log.info('show 值发生了改变，当前值:', v)
                self.data.show = v

                if v == true then
                    self:play_video()
                end

                return true -- 触发属性修改事件与绑定传播
            end,
        },
        video_src = 'https://oss.laf.run/ckjiwl-cloud-bin/神意.mp4',
    },
}

-- 定义video变量默认值
local video_test = nil -- 视频控件

-- 测试函数
function map_self:test_online(...)
    log.info('client: test_online')

    if not video_test then
        -- -- 检测是否支持webview
        -- log.info('check_webview', ui.check_webview_environment())

        -- -- 开启webview模块能力
        ui.set_enabled_in_game('webview', true)
        -- -- 创建video控件
        video_test = new(MyVideo {})
    else
        video_test.show = true
    end
end
