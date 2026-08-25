-- 消息通道（message_send/query/modify_read/delete）签名试通探针（研究任务探针；2026-08-25 自 test_res002 .bgd/src/client 转移固化）
-- 依据：SCEEngine.dll wrapper 反汇编（见 lowlevel/cloudvar-07-message-api.md）
--   message_send  wrapper 0x181321990: arg1=uid/Player, arg2=key string, arg3=int(lua_isnumber), arg4=value(序列化), arg5=events(可选)
--   message_query wrapper 0x1813215b0: arg1=uid/Player, arg2=key string, arg3=events(必填), arg4=可选
local M = {}

local function mark(s)
    log.info('[MsgProbe] ' .. s)
end

local function ev(tag)
    return {
        ok = function(...)
            local parts = {}
            for i, v in ipairs({ ... }) do
                parts[#parts + 1] = string.format('arg%d=%s', i, bgd_api.common.json.encode_x(v))
            end
            mark(('<<< %s ok %s'):format(tag, table.concat(parts, ' ')))
        end,
        error = function(code, reason)
            mark(('<<< %s error code=%s reason=%s'):format(tag, tostring(code), tostring(reason)))
        end,
        timeout = function()
            mark(('<<< %s timeout'):format(tag))
        end,
    }
end

local function step(name, fn)
    mark('>>> ' .. name)
    local ok, err = pcall(fn)
    if not ok then
        mark(('!!! %s pcall failed: %s'):format(name, tostring(err)))
    end
end

local KEY = 'bgd_msg_probe'

local function run()
    local lp = base.local_player()
    mark('local_player=' .. tostring(lp))
    -- 第一轮实证：bgd Player 包装表不被 native 接受（player参数不是合法的类型）；
    -- PIE 本地 uid 见 player.lua 日志「local user：38672742」，本轮直接用整数 uid + nil 变体
    local uid = 38672742
    mark('using uid=' .. uid)

    -- Q0: nil 形态（nil=本机玩家?）
    step('Q0 message_query(nil, key, events)', function()
        sce.s.message_query(nil, KEY, ev('Q0'))
    end)

    -- Q1: 整数 uid 形态
    base.wait(2000, function()
        step('Q1 message_query(uid, key, events)', function()
            sce.s.message_query(uid, KEY, ev('Q1'))
        end)
    end)

    -- S1: arg3=0 试发
    base.wait(5000, function()
        step('S1 message_send(uid, key, 0, table, events)', function()
            sce.s.message_send(uid, KEY, 0, { text = 'hello_probe', n = 1 }, ev('S1'))
        end)
    end)

    -- Q2: 再查，拿到 objectId 后试 modify_read / delete
    base.wait(9000, function()
        step('Q2 message_query 复读', function()
            sce.s.message_query(uid, KEY, {
                ok = function(result)
                    mark('<<< Q2 ok result=' .. bgd_api.common.json.encode_x(result))
                    if type(result) == 'table' and result[1] then
                        local oid = result[1].objectId or result[1].message_id
                        mark('Q2 first objectId=' .. tostring(oid))
                        if oid then
                            step('M1 message_modify_read(uid, oid, true, events)', function()
                                sce.s.message_modify_read(uid, oid, true, ev('M1'))
                            end)
                            step('D1 message_delete(uid, oid, events)', function()
                                sce.s.message_delete(uid, oid, ev('D1'))
                            end)
                        end
                    end
                end,
                error = function(code, reason)
                    mark(('<<< Q2 error code=%s reason=%s'):format(tostring(code), tostring(reason)))
                end,
                timeout = function()
                    mark('<<< Q2 timeout')
                end,
            })
        end)
    end)

    mark('probe scheduled')
end

base.event_register(base.game, '场景-加载完成', function()
    -- B 模式抓包窗口：延迟 20s 等 entrance_sniff attach（2026-08-24）
    base.wait(20000, function()
        run()
    end)
end)

return M
