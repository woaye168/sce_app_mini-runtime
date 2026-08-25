-- 云变量底层协议抓包探针（研究任务探针；2026-08-25 自 test_res002 .bgd/src/client 转移固化；勿发布正式版）
-- 目的：在 tester（线上测试环境）客户端进程内触发各类 sce.s 调用，
--       配合 ssl_sniff.py（libgmessl SSL_read/write hook）抓 Entrance 明文帧。
-- 步骤间用 base.wait 拉开时间，便于把 lua 日志与网络帧按时间对齐。
local M = {}
local map_name = __MAIN_MAP__

local function mark(s)
    log.info('[CloudProbe] ' .. s)
end

local function ev(tag)
    return {
        ok = function(...)
            local args = { ... }
            local parts = {}
            for i, v in ipairs(args) do
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

local function run()
    mark('probe start, readonly_map=' .. tostring(map_name) .. ' readwrite_map=' .. tostring(map_name))

    step('S1 score_init readonly p_55a3', function()
        sce.s.score_init(map_name, nil, ev('S1'), 'p_55a3')
    end)
    -- base.wait(4000)

    step('S2 score_init readwrite probe_key1', function()
        sce.s.score_init(map_name, nil, ev('S2'), 'probe_key1')
    end)
    -- base.wait(4000)

    step('S3 commit score_seti probe_key1', function()
        local c = sce.s.get_commit()
        c.score_seti(nil, 'probe_key1', 12345)
        c.score_sets(nil, 'probe_skey1', 'hello_probe')
        c.commit('cloud_probe', ev('S3'))
    end)
    -- base.wait(4000)

    step('S4 score_init readwrite 复读 probe_key1', function()
        sce.s.score_init(map_name, nil, ev('S4'), 'probe_key1', 'probe_skey1')
    end)
    -- base.wait(4000)

    step('S5 money_init', function()
        sce.s.money_init(nil, ev('S5'))
    end)
    -- base.wait(4000)

    step('S6 name_search', function()
        sce.s.name_search('probe_name', 'probe', ev('S6'))
    end)
    -- base.wait(4000)

    step('S7 list_query', function()
        sce.s.list_query(map_name, nil, 'probe_list', 10, ev('S7'))
    end)

    -- ===== 游戏态补全矩阵（2026-08-24：签名经 wrapper 反汇编确认，见 cloudvar-07/08）=====
    -- get_rank_list([map,] key, start, number, other_key?, events?)
    step('S8 get_rank_list', function()
        sce.s.get_rank_list(map_name, 'probe_key1', 1, 10, 'iscore', ev('S8'))
    end)

    -- get_user_rank(player|uid, key, other_key?, events?)
    step('S9 get_user_rank', function()
        sce.s.get_user_rank(nil, 'probe_key1', 'iscore', ev('S9'))
    end)

    -- get_rank_total([map,] key, events, ...)（2026-08-24 实证：events 在 #3）
    step('S10 get_rank_total', function()
        sce.s.get_rank_total(map_name, 'probe_key1', ev('S10'))
    end)

    -- query_item(player, callback, key?)
    step('S11 query_item', function()
        sce.s.query_item(nil, ev('S11'), 'probe_item')
    end)

    -- commit: money_add（游戏态权限验证）
    step('S12 commit money_add', function()
        local c = sce.s.get_commit()
        c.money_add(nil, 'money', 100)
        c.commit('probe_money_add', ev('S12'))
    end)

    -- commit: item_add(player, key, item_name, count, extra, expire_type:int, expire_time?:str)
    -- 2026-08-24 三次迭代定版：arg5=extra(序列化)、arg6=expire_type(number)、arg7=expire_time(string)
    step('S13 commit item_add', function()
        local c = sce.s.get_commit()
        c.item_add(nil, 'probe_item_key', 'sword', 1, { quality = 1 }, 0, '9999-12-31 23:59:59')
        c.commit('probe_item_add', ev('S13'))
    end)

    -- commit: client_score_set（客户端专属写，lobby 未试通）
    step('S14 commit client_score_set', function()
        local c = sce.s.get_commit()
        local ok2, err2 = pcall(function()
            c.client_score_set(nil, 'probe_ckey1', 777)
        end)
        if not ok2 then
            mark('S14 client_score_set pcall: ' .. tostring(err2))
        end
        c.commit('probe_client_score_set', ev('S14'))
    end)

    mark('probe done')
end

base.event_register(base.game, '场景-加载完成', function()
    -- B 模式抓包窗口：延迟 20s 等 entrance_sniff attach（2026-08-24）
    base.wait(20000, function()
        run()
    end)
end)

return M
