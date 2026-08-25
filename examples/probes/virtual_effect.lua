-- 动态虚拟数编（用户共享实现，注释前缀已清洗；研究任务探针；2026-08-25 自 test_res002 .bgd/src/client 转移固化）
-- 出处：sce_app_mini-runtime doc/research/lowlevel/virtual_effect.lua
---深拷贝(no self)
---@param obj table @源表: 任意
---@return table @任意
local function deep_copy(obj)
    local visited = {} -- 记录已访问的表，避免循环引用导致的无限递归

    local function _copy(obj)
        -- 非表类型直接返回（数字、字符串、布尔值、函数等）
        if type(obj) ~= 'table' then
            return obj
        end

        -- 如果已访问过该表，直接返回之前创建的副本
        if visited[obj] then
            return visited[obj]
        end

        -- 创建新表并记录到已访问列表
        local new_table = {}
        visited[obj] = new_table

        -- 递归复制所有字段（包括元表）
        for k, v in pairs(obj) do
            new_table[_copy(k)] = _copy(v)
        end

        -- 处理元表
        local mt = getmetatable(obj)
        if mt then
            setmetatable(new_table, _copy(mt)) -- 深拷贝元表
        end

        return new_table
    end

    return _copy(obj)
end

--- @class path_parser
--- 路径解析器类，用于解析类似 @.xxx 和 @[123] 的路径字符串
local path_parser = {}
path_parser.__index = path_parser

--- 创建新的路径解析器实例
--- @param path string 要解析的路径字符串，必须以 '@' 开头
--- @return path_parser 新的路径解析器实例
function path_parser.new(path)
    if type(path) ~= 'string' or not path:match('^@') then
        error('路径必须以 \'@\' 开头')
    end

    local self = setmetatable({}, path_parser)
    self:_parse(path)
    return self
end

--- 解析路径字符串
--- @private
--- @param path string 要解析的路径字符串
function path_parser:_parse(path)
    self._keys = {}
    local index = 2 -- 跳过开头的 '@'
    local len = #path

    while index <= len do
        local char = path:sub(index, index)

        if char == '.' then
            -- 点语法：字符串键
            index = index + 1
            local start = index
            while index <= len and path:sub(index, index):match('[%w_]') do
                index = index + 1
            end
            local key = path:sub(start, index - 1)
            if key == '' then
                error('点语法后必须有键名')
            end
            table.insert(self._keys, key)

        elseif char == '[' then
            -- 方括号语法：数字键
            index = index + 1
            local start = index
            while index <= len and path:sub(index, index):match('%d') do
                index = index + 1
            end

            if path:sub(index, index) ~= ']' then
                error('缺少闭合方括号')
            end

            local key_str = path:sub(start, index - 1)
            if key_str == '' then
                error('方括号内必须有数字')
            end

            local key = tonumber(key_str)
            if not key then
                error('无效的数字键: ' .. key_str)
            end

            table.insert(self._keys, key)
            index = index + 1 -- 跳过 ']'

        else
            error('无效的路径语法: ' .. path:sub(index))
        end
    end

    if #self._keys == 0 then
        error('路径不能为空')
    end
end

--- 获取解析后的键序列
--- @return table 键序列数组
function path_parser:get_keys()
    return self._keys
end

--- @class virtual_data
--- 虚拟数据操作类，用于通过路径访问和修改表数据
local virtual_data = {}
virtual_data.__index = virtual_data

--- 创建新的虚拟数据操作实例
--- @param data table 要操作的数据表
--- @return virtual_data 新的虚拟数据操作实例
function virtual_data.new(data)
    local self = setmetatable({}, virtual_data)
    self._data = data
    return self
end

--- 通过路径设置值
--- @param path string 路径字符串，格式如 '@.property.sub_property'
--- @param value any 要设置的值
function virtual_data:set_value(path, value)
    local parser = path_parser.new(path)
    local keys = parser:get_keys()

    local current = self._data
    for i = 1, #keys - 1 do
        local key = keys[i]

        -- 检查中间路径是否存在
        if current[key] == nil then
            error('路径不存在: ' .. table.concat(keys, '.', 1, i))
        end

        if type(current[key]) ~= 'table' then
            error('路径冲突: \'' .. tostring(key) .. '\' 不是表')
        end

        current = current[key]
    end

    -- 设置最终键的值
    current[keys[#keys]] = value
end

--- 通过路径获取值
--- @param path string 路径字符串，格式如 '@.property.sub_property'
--- @return any|nil 获取到的值，如果路径不存在则返回nil
function virtual_data:get_value(path)
    local parser = path_parser.new(path)
    local keys = parser:get_keys()

    local current = self._data
    for i = 1, #keys do
        local key = keys[i]

        if current == nil then
            return nil
        end

        current = current[key]
    end

    return current
end

--- 获取底层数据表
--- @return table 底层数据表
function virtual_data:get_data()
    return self._data
end

--- @class virtual_effect
--- 虚拟数编操作类，用于创建和操作游戏效果
local virtual_effect = {}
virtual_effect.__index = virtual_effect

--- 创建新的虚拟数编
--- @param tpl_eff_id string 模板效果ID
--- @param new_eff_name string 新效果名称
--- @return virtual_effect|nil 新的虚拟数编实例，创建失败时返回nil
function virtual_effect.new(tpl_eff_id, new_eff_name)
    local ok, eff_data = pcall(base.eff.cache, tpl_eff_id)

    if not ok or type(eff_data) ~= 'table' then
        log.error(string.format('创建虚拟数编id失败 create_eff_id，tpl_eff_id：%s', tpl_eff_id))
        return nil
    end

    local new_eff = deep_copy(eff_data)
    local eff_link = string.format('$$%s.%s.%s.%s', __MAIN_MAP__, new_eff['Class'], new_eff_name, new_eff['ID'])

    new_eff['Name'] = new_eff_name
    new_eff['Link'] = eff_link

    -- 合并数编
    base.eff.merge_cache({
        ['dict'] = {
            [eff_link] = new_eff,
        },
    })

    local self = setmetatable({}, virtual_effect)
    self._data = new_eff
    self._link = eff_link
    self._virtual_data = virtual_data.new(new_eff)
    return self
end

--- 通过路径设置效果属性
--- @param path string 路径字符串
--- @param value any 要设置的值
function virtual_effect:set_value(path, value)
    self._virtual_data:set_value(path, value)
end

--- 通过路径获取效果属性
--- @param path string 路径字符串
--- @return any|nil 获取到的属性值
function virtual_effect:get_value(path)
    return self._virtual_data:get_value(path)
end

--- 获取数编链接
--- @return string 数编链接
function virtual_effect:get_link()
    return self._link
end

--- 获取数编数据表
--- @return table 数编数据表
function virtual_effect:get_data()
    return self._data
end

-- 暴露给触编的方法
--- @param tpl_eff_id string 模板效果ID
--- @param new_eff_name string 新效果名称
--- @return virtual_effect|nil 新的虚拟数编实例，创建失败时返回nil
function virtual_effect:create_virtual_effect(tpl_eff_id, new_eff_name)
    return virtual_effect.new(tpl_eff_id, new_eff_name)
end

return virtual_effect
