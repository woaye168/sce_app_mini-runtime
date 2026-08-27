p_lx61 = p_lx61 or {}
local map_self = p_lx61
local component = require '@common.base.gui.component'
local new = component.new
local bind = component.bind
local alias = component.alias
local getset = component.getset
local control_util = require '@common.base.gui.control_util'

-- webview的 html 模板
-- 想要vscode的代码提示，可以卸载单独的html文件里，然后用io.write读取。
-- 但是注意 io.write 默认读取的是 User/Map/ 目录。需要用项目文件路径，需要自己构建绝对路径。
-- 特别注意，由于又是js代码，又是lua代码，在写js的时候可能会写成lua形式然后，调试里报token语法错误。
local my_html = [[
<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <meta name="HandheldFriendly" content="false">
    <style>
        body, html {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
        }
        #fullscreen-iframe {
            width: 100%;
            height: 100%;
            border: none;
            position: fixed;
            top: 0;
            left: 0;
        }
        #btn_msg {
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 10px 20px;
            background-color: white;
            border: 1px solid #ccc;
            border-radius: 4px;
            cursor: pointer;
        }
    </style>
    <script>
        // 从js端发送消息给lua端, 使用window.scelua.send_string
        function send_to_lua_msg() {
            const message = {
                type: "hello_lua"
            };
            window.scelua.send_string(JSON.stringify(message))
        }

        // 工具函数, 格式化时间
        function get_time(time) {
            const date = time
            const year = date.getFullYear()
            const month = date.getMonth() + 1
            const day = date.getDate()
            const hours = date.getHours()
            const minutes = date.getMinutes()
            const timeStr = `${year}/${month}/${day} ${hours}:${minutes.toString().padStart(2, '0')}`

            return timeStr
        }

        // 创建全局监听,监听来自lua的消息
        window.addEventListener('GlobalEvent', event => {
            console.log(event)

            // 获取传递的数据
            const msg = event.detail.message
            const type = msg.type
            const msgId = msg.msgId
            const content = msg.content
            
            // 根据消息类型处理数据
            switch(type) {
                case 'hello_js':
                console.log(`${content} | ${String(msgId)} --${get_time(new Date())}`)
                break

                case 'change_url':
                const iframe = document.getElementById('fullscreen-iframe')
                // iframe.src = `${content}?t=${Date.now()}`
                iframe.src = content
                break

                default:
                console.warn('未知类型:', type)
            }
        })
    </script>
</head>
<body>
    <iframe
        id="fullscreen-iframe"
        src="https://www.bilibili.com/video/BV1KiGgzXELq/?share_source=copy_web&vd_source=079957da36512265dcdd5ae82faa027d"
        allowfullscreen>
    </iframe>
    <button id="btn_msg" onclick="send_to_lua_msg()">
        发送消息给lua
    </button>
</body>
</html>
]]

-- 已经在 组件 MyWeb 中动态构建，这里没用了。
-- local my_run_js = [[
-- document.addEventListener("DOMContentLoaded", function () {
-- 	window.scelua.send_string("DOMContentLoaded");
-- });
-- ]]

-- webview 测试组件
local MyWeb = component 'MyWeb' {
    base.ui.panel {
        color = 'rgba(59, 59, 59, 1)',
        swallow_event = true,
        z_index = 9999,
        layout = {
            width = -1,
            height = -1,
            width_grow = 1,
            height_grow = 1,
            width_shrink = 1,
            height_shrink = 1,
            margin = 0,
            padding = 0,
            direction = 'col',
            row_content = 'start',
            col_content = 'start',
        },
        base.ui.panel 'header' {
            color = 'rgba(41, 41, 41, 1)',

            layout = {
                width = -1,
                height = 110,
                width_grow = 1,
                direction = 'row',
                row_content = 'start',
                col_content = 'center',
                padding = {
                    top = 20,
                    bottom = 20,
                    left = 200,
                    right = 200,
                },
            },
            base.ui.panel 'url' {
                color = 'rgb(16, 16, 16)',
                round_corner_radius = 100,
                layout = {
                    width = 100,
                    height = -1,
                    width_grow = 1,
                    height_grow = 1,
                    direction = 'row',
                    row_content = 'start',
                    col_content = 'start',
                    padding = {
                        left = 30,
                        right = 30,
                    },
                },
                base.ui.input 'input' {
                    text = '星火编辑器',
                    font = {
                        size = 30,
                        color = 'rgba(183, 183, 183, 1)',
                    },
                    layout = {
                        width = -1,
                        height = -1,
                        width_grow = 1,
                        height_grow = 1,
                    },
                },
                base.ui.button 'btn_openweb' {
                    color = 'rgba(41, 41, 41, 1)',
                    round_corner_radius = 100,
                    layout = {
                        width = 100,
                        height = -1,
                        height_grow = 1,
                        margin = {
                            top = 5,
                            bottom = 5,
                            left = 20,
                            right = -25,
                        },
                    },
                    base.ui.label 'btn_openweb_text' {
                        text = 'Go',
                        font = {
                            size = 30,
                            color = 'rgba(183, 183, 183, 1)',
                        },
                    },
                },
            },
            base.ui.button 'btn_close' {
                round_corner_radius = 100,
                image = 'image/icons/关闭1.png',
                font = {
                    size = 30,
                    color = 'rgba(0, 0, 0,1)',
                },
                layout = {
                    width = 50,
                    height = 50,
                    margin = {
                        left = 600,
                    },
                },
            },
        },
        base.ui.webview 'webview' {
            layout = {
                width = 1000,
                height = 800,
                width_grow = 1,
                height_grow = 1,
                width_shrink = 1,
                height_shrink = 1,
                row_self = 'start',
                col_self = 'start',
            },

            -- 以下是已经知道作用的属性
            -- 特别注意，开启webview 需要满足的条件：
            -- 1、使用 base.ui.webview
            -- 2、开启webview之前，需要先使用 ui.set_enabled_in_game('webview', true) 开启游戏的webview模块能力
            -- 3、所在的星火环境，支持webview的能力。ui.check_webview_environment() 检测是否支持。调用返回 true\false
            -- 4、需要在经过线上的lobby，才有能看到webview的效果。换句话说，编辑器中调试会看不到webview，需要传到线上。用对战平平台看

            -- 由于webview调试需要用到星火对战平台，这里列出对战平台下载地址

            -- 正式环境
            -- 线上版编辑器下载: https://package-pd.spark.xd.com/pd_editor.html
            -- 线上版本编辑器下载，星火文档: https://doc.sce.xd.com/Manual/GameLaunch/First
            -- 线上版对战平台下载(PC): https://package-pd.spark.xd.com/pd_client_pc.html
            -- 线上版对战平台下载(安卓): https://package-pd.spark.xd.com/pd_client_android.html
            -- 线上版对战平台下载(IOS): 苹果Appstore 下载
            -- 线上版创作者中心（也可以从编辑器内跳转）: https://developer.spark.xd.com/

            -- 测试环境
            -- 测试版编辑器下载: https://package-pd.spark.xd.com/alpha_editor.html
            -- 测试版本编辑器下载，星火文档: https://doc.sce.xd.com/Manual/Welcome/CommonLinks
            -- 测试版对战平台下载(PC): https://package-pd.spark.xd.com/alpha_client_pc.html
            -- 测试版对战平台下载(安卓): https://package-pd.spark.xd.com/alpha_client_android.html
            -- 测试版对战平台下载(IOS): 无
            -- 测试版创作者中心（也可以从编辑器内跳转）: https://developer-alpha.spark.xd.com/

            -- 设置 webview的html 内容
            -- js发消息给lua, js中使用: window.scelua.send_string(JSON.stringify(message))
            -- js接受lua消息, js中使用: window.addEventListener，具体监听需要根据 run_js 的构建全局监听器
            html = my_html,

            -- 自动注入运行的js，下面 method 的 init 中有动态构建，所以这里注释了。
            -- run_js = '', 
            -- run_js = my_run_js,

            -- 设置webview内容为外部url
            -- 注意: 当html属性存在内容时候，此设置将无效，webview会优先使用html属性的内容
            -- url = 'start-game://p_1ax1',
            -- url = 'https://www.baidu.com',
            -- url = 'https://beta.unity3d.com/jonas/AngryBots/', -- wasm加载外部游戏

            -- 开启webview的开发者工具，也就是chrome的F12
            -- 注意: 这个开启之后，只有使用星火对战平台PC版本才能看出效果，在打开webview的时候自动打开
            web_dev_tools = true,

            -- 已测试，没有效果
            -- 使用独立窗口的WebView（若对应环境实现了的话）
            isolated = true,

            -- 下面这几个还没研究懂，但是不重要，前面的够用的，先放着，有时间再研究折腾
            -- web_type = 'web', -- 看起来像是设置 user_agent 但是似乎没作用，也许设置星火的游戏类型，比如minigame
            -- web_message = '', -- 还不知道作用，似乎是与 on_web_message 有关
            -- web_import_script = sdk_js, -- minigame使用需要导入的js， 参考: D:\sce_open\res\startup\application\mini_game\main_page.lua

            event = {
                -- 只有绑定了 on_web_message 才能接受来自js的消息。
                -- 前面已经说明了,js向lua发送消息的方法: window.scelua.send_string(JSON.stringify(message)) 
                -- 这里我把事件是放在init中写，当然，我也可以直接在这里写, 像下面注释里的
                -- on_web_message = function(message)
                --     -- 这里的 message 接受的就是 js发送的 message。
                --     -- 但是注意js是序列化发过来的，所以使用的时候需要反序列化回去 base.json.decode(message)
                -- end,
                on_web_message = bind.on_web_message,
            },
        },
    },
    method = {
        init = function(self)
            self.on_web_message = function(message)
                -- 调试看参数用
                -- map_self:io_write('web_msg.txt', map_self:json_encode(message))
                -- log.info('js2lua message :', map_self:json_encode(message))

                -- 先把收到的消息反序列化
                local msg_obj = base.json.decode(message)

                -- 拿到消息类型
                local type = msg_obj.type

                -- 根据消息类型处理事件
                if type == 'hello_lua' then

                    -- 这里我是我来测试webview的js发送消息给lua的测试
                    -- 注意: 如果没有安装星火的聊天预制库，使用 lib_gamechat_2:gamechatclient_send_message 会报错的噢~
                    lib_gamechat_2:gamechatclient_send_message('这条消息来自webview: ' .. os.date('%Y/%m/%d %H:%M:%S'))
                end
            end

            -- 注册关闭按钮事件
            self['@btn_close.event.on_click'] = function()
                self['@.show'] = false

                -- 发送消息给js
                self:sendWeb({
                    type = 'hello_js',
                    msgId = 1,
                    content = '这条消息来自游戏内部的lua',
                })
            end

            -- 注册搜索按钮事件
            self['@btn_openweb.event.on_click'] = function()
                local url_input = self['@input.text']
                log.info(url_input)

                local url = 'https://search.bilibili.com/all?from_source=webtop_search&search_source=5&keyword=' .. self['@input.text']

                -- 发送界面更新事件给js
                self:sendWeb({
                    type = 'change_url',
                    msgId = 2,
                    content = url,
                })

                -- 下面的代码是直接改src, 对于不支持iframe的页面,需要用下面这种方式打开.
                -- if not string.find(url_input, 'https://', 1, true) and not string.find(url_input, 'https://', 1, true) then
                --     -- self['@webview.url'] = 'https://www.bing.com/search?q=' .. self['@input.text']
                --     url = 'https://www.bing.com/search?q=' .. self['@input.text']

                -- elseif string.match(url_input, 'www%.[%w-]+%.(com|cn|net)') then
                --     -- self['@webview.url'] = 'https://' .. self['@input.text']
                --     url = 'https://' .. self['@input.text']
                -- else
                --     -- self['@webview.url'] = self['@input.text']
                --     url = self['@input.text']
                -- end
            end
        end,

        -- 这个方法构建lua发送消息给js的标准形式
        -- 如果改了这里，js里的接收也需要相应调整
        sendWeb = function(self, msg)
            local send_str = string.format('window.dispatchEvent(new CustomEvent(\'GlobalEvent\',{ detail: { message: %s } }))', base.json.encode(msg))
            log.info('send_str:', send_str)
            -- 用全局事件发给js，正式环境js会做混淆
            self['@webview.run_js'] = send_str
        end,
    },
}

-- webview 的组件实例
local web_test = nil

-- 测试函数
function map_self:test_online(...)
    log.info('client: test_online')

    -- 这东西为什么不直接作为依赖库的一个方法让游戏来调用？主要还是因为on_exit()里面可能有app.reload()，会跨Lua虚拟机执行逻辑（游戏的Lua虚拟机间接把启动页的虚拟机重载了）
    -- base.game:send_broadcast('switch_game', 'start-game://p_1ax1')
    -- common.open_url('start-game://p_1ax1')
    -- base.game:send_broadcast('switch_game', 'start-game://p_1ax1')

    if not web_test then
        -- -- 检测是否支持webview
        -- log.info('check_webview', ui.check_webview_environment())
是
        -- -- 开启webview模块能力
        ui.set_enabled_in_game('webview', true)
        -- -- 创建webview控件
        web_test = new(MyWeb {})
    else
        web_test.show = true
    end
end

function map_self:api_online(...)
    log.info('client: api_online')
end
