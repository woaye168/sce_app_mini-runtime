Canvas API 完整使用手册
base.ui.canvas 提供了两套独立的绘图系统：
1. Brush (矢量画笔): 用于绘制高频更新的几何图形（线、圆、多边形、贝塞尔曲线）。
2. Texture (纹理操作): 用于像素级操作（填充区域、读取像素、保存纹理数据）。

1. 矢量画笔 (Brush)
适用场景：节点连线、动态 UI 图形、无需保存状态的实时绘制。
初始化：
local brush = base.ui.brush:create(canvas_ui.id)
1.1 基础控制
API	说明	示例
clear()	清空整个 Canvas (包含纹理内容)	brush:clear()
set_line_width(w)	设置线条宽度	brush:set_line_width(2)
set_line_color(c)	设置线条颜色 (Hex/RGBA)	brush:set_line_color('#FF0000')
set_fill_color(c)	设置填充颜色 (Hex/RGBA)	brush:set_fill_color('rgba(0,0,0,0.5)')
rotate(x, y, ang)	围绕点(x,y)旋转坐标系 (角度制)	brush:rotate(100, 100, 90)
1.2 图形绘制
API	说明	示例
draw_line(x1,y1,x2,y2)	画直线	brush:draw_line(0,0,100,100)
draw_circle(x,y,r)	画空心圆 (使用 line_color)	brush:draw_circle(50,50,20)
fill_circle(x,y,r)	画实心圆 (使用 fill_color)	brush:fill_circle(50,50,20)
draw_polygon(pts)	画空心多边形	brush:draw_polygon({{0,0},{100,0},{50,100}})
fill_polygon(pts)	画实心多边形	brush:fill_polygon({{0,0},{100,0},{50,100}})
draw_image(path,x,y,w,h)	绘制图片	brush:draw_image('icon.png',0,0,64,64)
1.3 路径绘制 (Path)
API	说明	示例
path_line_to(x,y)	移动路径点	brush:path_line_to(10,10)
path_bezier_curve_to(...)	绘制贝塞尔曲线 (2次/3次)	brush:path_bezier_curve_to(cx1,cy1,cx2,cy2,ex,ey)
path_stroke(close)	描绘路径 (true闭合)	brush:path_stroke(false)

2. 纹理操作 (Texture)
适用场景：涂鸦板、迷雾擦除、像素读取、截图保存。
初始化：
local texture = canvas_ui:get_brush()
前置要求：必须先调用 set_size。
2.1 基础设置
API	说明	示例
set_size(w, h)	[必须] 设置纹理分辨率	texture:set_size(512, 512)
set_name(name)	设置纹理名称	texture:set_name('MyTex')
set_fill_color(c)	设置填充颜色	texture:set_fill_color('#00FF00')
2.2 像素操作
API	说明	示例
fill_rect(x,y,w,h)	填充矩形	texture:fill_rect(0,0,100,100)
fill_circle(x,y,r)	填充圆形	texture:fill_circle(50,50,20)
clear_circle(x,y,r,s)	擦除圆形(变透明, s=羽化)	texture:clear_circle(50,50,20,2)
fill_pixel(x,y)	填充单像素	texture:fill_pixel(10,10)
set_blur(r)	全图高斯模糊	texture:set_blur(5)
2.3 数据读写 (Data)
API	说明	示例
get_pixel_color(x,y)	获取像素颜色值 (int)	local c = texture:get_pixel_color(0,0)
get_compressed_data(cb)	异步获取纹理数据 (用于保存)	texture:get_compressed_data(function(d) end)
set_compressed_data(d)	设置纹理数据 (用于恢复)	texture:set_compressed_data(data)

3. 完整代码示例
local c = base.ui.canvas { layout = { width = 800, height = 600 } }
local c_ui = base.ui.create(c)

base.next(function()
    -- ===========================
    -- 1. 纹理操作 (Texture)
    -- ===========================
    local texture = c_ui:get_brush()
    texture:set_size(800, 600) -- 必须设置大小！
    
    -- 背景填充
    texture:set_fill_color('rgba(0,0,0,0.1)')
    texture:fill_rect(0, 0, 800, 600)
    
    -- 擦除效果 (挖洞)
    texture:clear_circle(400, 300, 50, 5)

    -- 数据保存与恢复演示
    base.timer(1000, function()
        texture:get_compressed_data(function(data)
            if data then
                log.info('Data saved, size:', #data)
                -- texture:set_compressed_data(data) -- 需要恢复时调用
            end
        end)
    end)

    -- ===========================
    -- 2. 矢量绘图 (Brush)
    -- ===========================
    local brush = base.ui.brush:create(c_ui.id)
    -- brush:clear() -- 若需要清空上面的纹理，取消注释
    
    -- 画贝塞尔曲线
    brush:set_line_width(3)
    brush:set_line_color('#FF0000')
    brush:path_line_to(100, 300)
    brush:path_bezier_curve_to(200, 100, 600, 500, 700, 300)
    brush:path_stroke(false)
    
    -- 画多边形
    brush:set_fill_color('#FFFF00')
    brush:fill_polygon({{50,50}, {100,100}, {50,150}})
end)


canvas 画图程序

  -- 例子 你猜我画实现
  -- 创建状态管理器
  local canvas_state = {
      is_drawing = false,
      draw_brush = nil,
      last_x = 0,
      last_y = 0
  }

  -- 定义画板 UI
  local drawing_canvas = base.ui.canvas {
      -- id = 'my_drawing_board',
      layout = {
          width = 800,
          height = 600,
          -- 确保居中或有固定位置，方便观察
          row_self = 'center',
          col_self = 'center'
      },
      -- 样式设置
      -- image = '',                   -- 背景图（可选）
      color = 'rgba(0, 0, 0, 0.3)', -- 浅色背景方便看清

      event = {
          -- 1. 鼠标按下：开始绘画
          on_click = function() end

          -- 辅助：鼠标离开 Canvas 范围也可选择是否停止绘画
          -- on_mouse_leave = function(self) self.is_drawing = false end
      }
  }

  -- 创建并显示 UI
  local canvas_ui = base.ui.create(drawing_canvas)

  base.event_register(base.game, '鼠标-按下', function(trg, key)
      -- trg,key
      -- key : button_left(左键)， button_right(右键),， button_middle(滚轮键)
      log.info('触发了: 鼠标-按下')

      if key == 'button_left' then
          if not canvas_state.draw_brush then
              log.info('------没有画笔，创建画笔')
              canvas_state.draw_brush = base.ui.brush:create(canvas_ui.id)
              canvas_state.draw_brush:set_line_color('rgb(255, 0, 0)')
              canvas_state.draw_brush:set_line_width(3)
          end

          local screen_x, screen_y = common.get_mouse_screen_pos()
          local rx, ry = canvas_ui:rect()

          canvas_state.last_x = screen_x - rx
          canvas_state.last_y = screen_y - ry
      end
  end)

  -- 2. 全局鼠标移动监听
  base.game:event('鼠标-移动', function()
      log.info('------鼠标-移动')
      if canvas_state.draw_brush then
          local screen_x, screen_y = common.get_mouse_screen_pos()
          local rx, ry = canvas_ui:rect()
          local cur_x = screen_x - rx
          local cur_y = screen_y - ry

          base.next(function()
              canvas_state.draw_brush:draw_line(
                  canvas_state.last_x,
                  canvas_state.last_y,
                  cur_x,
                  cur_y
              )
          end)

          canvas_state.last_x = cur_x
          canvas_state.last_y = cur_y
      end
  end)

  -- 3. 全局鼠标松开监听
  base.game:event('鼠标-松开', function()
      -- canvas_state.is_drawing = false
  end)

canvas 使用例子：
-- -- canvas例子一
  -- -- 1. 创建 UI
  -- local c = base.ui.canvas {
  --     layout = { width = 800, height = 600 }
  -- }
  -- local c_ui = base.ui.create(c)
  -- log.info('---------c_ui')
  -- log.info('---------c_ui 1')
  -- log.info(bgd_api.common.json.encode_x(c_ui))
  -- log.info('---------c_ui 2')
  -- log.info(bgd_api.common.json.encode_x(getmetatable(c_ui)))

  -- -- 2. 必须在下一帧绘制
  -- base.next(function()
  --     -- ===========================
  --     -- A. 矢量绘图 (Vector)
  --     -- ===========================
  --     local brush = base.ui.brush:create(c_ui.id)
  --     brush:clear() -- 清空

  --     log.info('---------brush')
  --     log.info('---------brush 1')
  --     log.info(bgd_api.common.json.encode_x(brush))
  --     log.info('---------brush 2')
  --     log.info(bgd_api.common.json.encode_x(getmetatable(brush)))

  --     -- 1. 画线
  --     brush:set_line_width(5)
  --     brush:set_line_color('#FF0000') -- 红线
  --     brush:draw_line(50, 50, 200, 50)

  --     -- 2. 画圆 (实心 + 空心)
  --     brush:set_fill_color('rgba(0, 255, 0, 0.5)') -- 半透明绿
  --     brush:fill_circle(100, 150, 40)

  --     brush:set_line_width(2)
  --     brush:set_line_color('#0000FF') -- 蓝边
  --     brush:draw_circle(200, 150, 40)

  --     -- 3. 画多边形 (三角形)
  --     brush:set_fill_color('#FFFF00')
  --     brush:fill_polygon({ { 300, 100 }, { 350, 200 }, { 250, 200 } })

  --     -- 4. 贝塞尔曲线 (波浪线)
  --     brush:set_line_color('#00FFFF')
  --     brush:set_line_width(3)
  --     brush:path_line_to(50, 300) -- 移动画笔到起点
  --     -- 控制点(150, 200), 终点(250, 300)
  --     brush:path_bezier_curve_to(150, 200, 250, 300)
  --     brush:path_stroke(false) -- 绘制路径，不闭合

  --     -- ===========================
  --     -- B. 纹理操作 (Texture)
  --     -- ===========================
  --     local texture = c_ui:get_brush()

  --     -- 1. 在右下角填充一个半透明蓝色矩形
  --     texture:set_size(800, 600)
  --     texture:set_fill_color('rgba(0, 0, 255, 0.7)')
  --     texture:fill_rect(400, 300, 100, 100)

  --     -- 2. 擦除中间一个圆洞
  --     texture:clear_circle(450, 350, 30, 5) -- 半径30，羽化边缘5

  --     log.info('---------texture')
  --     log.info('---------texture 1')
  --     log.info(bgd_api.common.json.encode_x(texture))
  --     log.info('---------texture 2')
  --     log.info(bgd_api.common.json.encode_x(getmetatable(texture)))
  -- end)


  -- canvas例子二
  -- local c = base.ui.canvas {
  --     layout = { width = 800, height = 600 }
  -- }
  -- local c_ui = base.ui.create(c)

  -- base.next(function()
  --     -- ==========================================
  --     -- 1. 纹理画笔 (Texture Brush) - 像素级操作
  --     -- ==========================================
  --     -- 获取纹理画笔
  --     local tex_brush = c_ui:get_brush()

  --     -- 【关键步骤】必须手动设置纹理大小！否则无法绘制
  --     -- 通常设置为和 UI 控件一样大，或者是它的倍数
  --     tex_brush:set_size(800, 600)

  --     -- 填充一个绿色矩形背景
  --     tex_brush:set_fill_color('rgba(0, 255, 0, 1)')
  --     tex_brush:fill_rect(50, 50, 200, 200)

  --     -- 在纹理上“挖”一个洞（变透明）
  --     tex_brush:clear_circle(150, 150, 50, 2) -- x, y, r, smooth

  --     -- ==========================================
  --     -- 2. 矢量画笔 (Vector Brush) - 几何图形绘制
  --     -- ==========================================
  --     -- 创建矢量画笔
  --     local vec_brush = base.ui.brush:create(c_ui.id)

  --     -- 注意：vec_brush:clear() 会清空整个 Canvas (包括上面的纹理绘制)
  --     -- 所以如果混用，不要在这里调用 clear()，或者在 clear() 之后再画纹理
  --     -- vec_brush:clear()

  --     -- 画一条红线穿过上面的矩形
  --     vec_brush:set_line_width(5)
  --     vec_brush:set_line_color('#FF0000')
  --     vec_brush:draw_line(0, 0, 300, 300)

  --     -- 画贝塞尔曲线
  --     vec_brush:set_line_color('#0000FF')
  --     vec_brush:path_line_to(400, 100)
  --     vec_brush:path_bezier_curve_to(500, 0, 600, 200, 700, 100)
  --     vec_brush:path_stroke()
  -- end)


  -- canvas例子二 get_compressed_data 和  set_compressed_data
  -- local c = base.ui.canvas {
  --     layout = { width = 500, height = 500 }
  -- }
  -- local c_ui = base.ui.create(c)

  -- base.next(function()
  --     local texture = c_ui:get_brush()
  --     texture:set_size(500, 500)

  --     -- 1. 先画点东西，不然读出来全是透明的
  --     texture:set_fill_color('#FF0000') -- 红色
  --     texture:fill_rect(0, 0, 100, 100)

  --     -- ==================================================
  --     -- A. get_pixel_color (同步获取)
  --     -- ==================================================
  --     -- 注意：坐标必须在 set_size 的范围内，否则可能返回 0 或报错
  --     local color_int = texture:get_pixel_color(50, 50)

  --     -- color_int 是一个整数，通常格式为 ARGB 或 RGBA（取决于引擎实现）
  --     -- 简单的验证方法：
  --     if color_int ~= 0 then
  --         log.info('Get Pixel Success:', color_int)
  --     else
  --         log.warn('Get Pixel Failed or Transparent')
  --     end


  --     -- ==================================================
  --     -- B. get_compressed_data (异步获取)
  --     -- ==================================================
  --     -- 用途：截图、保存画布状态、传输纹理数据
  --     texture:get_compressed_data(function(data)
  --         if not data then
  --             log.error('Failed to get texture data')
  --             return
  --         end

  --         log.info('Texture Data Size:', #data)

  --         -- ==================================================
  --         -- C. set_compressed_data (设置数据)
  --         -- ==================================================
  --         -- 模拟场景：3秒后将画布还原（比如实现了“撤销”功能）
  --         base.wait(3000, function()
  --             -- 先清空或改乱画布，证明数据还原有效
  --             texture:set_fill_color('#0000FF')
  --             texture:fill_rect(0, 0, 500, 500) -- 变成全蓝

  --             log.info('Restoring texture data...')

  --             base.wait(3000, function()
  --                 -- 还原回之前的红色方块状态
  --                 texture:set_compressed_data(data)
  --             end)
  --         end)
  --     end)
  -- end)


  -- -- canvas例子三
  -- -- 创建 UI
  -- local c = base.ui.canvas {
  --     layout = { width = 1000, height = 1000, row_self = 'start', col_self = 'start' },
  --     event = {
  --         on_click = function(self)
  --             log.info('点击')
  --         end,
  --     }
  -- }
  -- local c_ui = base.ui.create(c)

  -- base.next(function()
  --     -- =========================================================
  --     -- PART 1: Texture (纹理操作) - 必须先设置 size
  --     -- =========================================================
  --     local texture = c_ui:get_brush()
  --     texture:set_size(1000, 1000)     -- 1. set_size (必填)
  --     texture:set_name('demo_texture') -- 2. set_name (可选)

  --     -- 3. fill_rect: 填充背景色
  --     texture:set_fill_color('rgba(0, 0, 0, 0.1)') -- 4. set_fill_color
  --     texture:fill_rect(0, 0, 1000, 1000)

  --     -- 5. fill_circle: 画一个蓝色实心圆
  --     texture:set_fill_color('#0000FF')
  --     texture:fill_circle(200, 200, 50)

  --     -- 6. clear_circle: 在圆中间挖个洞 (带羽化)
  --     texture:clear_circle(200, 200, 30, 5)

  --     -- 7. fill_pixel: 随机画一些噪点
  --     texture:set_fill_color('#FFFFFF')
  --     for i = 1, 100 do
  --         texture:fill_pixel(math.random(0, 400), math.random(0, 400))
  --     end

  --     -- 8. set_blur: 全图模糊 (会模糊上面画的所有东西)
  --     -- texture:set_blur(30)

  --     -- 9. get_pixel_color: 获取某点颜色
  --     local color_int = texture:get_pixel_color(200, 200)
  --     log.info('Pixel Color Int:', color_int)

  --     -- 10. get/set compressed_data: 数据存取 (高级用法)
  --     -- texture:get_compressed_data(function(data)
  --     --     log.info('Got data length:', #data)
  --     --     -- texture:set_compressed_data(data) -- 还原数据
  --     -- end)


  --     -- =========================================================
  --     -- PART 2: Brush (矢量绘图) - 每一帧都会覆盖在纹理之上
  --     -- =========================================================
  --     local brush = base.ui.brush:create(c_ui.id) -- 1. create

  --     -- 2. clear: 如果不想保留上面的纹理内容，可以取消注释下面这行
  --     -- brush:clear()

  --     -- 3. draw_line: 画红线
  --     brush:set_line_width(5)          -- 4. set_line_width
  --     brush:set_line_color('#FF0000')  -- 5. set_line_color
  --     brush:draw_line(50, 50, 350, 50) -- 6. draw_line

  --     -- 7. draw_circle / fill_circle
  --     brush:set_line_width(2)
  --     brush:set_line_color('#00FF00')
  --     brush:draw_circle(100, 150, 30)              -- 空心圆

  --     brush:set_fill_color('rgba(0, 255, 0, 0.5)') -- 8. set_fill_color
  --     brush:fill_circle(200, 150, 30)              -- 实心圆

  --     -- 9. draw_polygon / fill_polygon: 三角形
  --     local points = { { 300, 120 }, { 350, 200 }, { 250, 200 } }
  --     brush:set_fill_color('#FFFF00')
  --     brush:fill_polygon(points) -- 实心

  --     brush:set_line_color('#000000')
  --     brush:draw_polygon(points) -- 描边


  --     -- 10. draw_image: 绘制图片
  --     -- 配置路径
  --     -- 由于Canvas路径查找是全局路径而不是地图路径，所以这里需要用绝对路径
  --     local root_dir = io.get_root_dir()
  --     local app_dir = io.get_app_dir()

  --     -- user_data_path 这里没用上，先放这里做个备选使用。
  --     -- 当无法很好获取路径的时候，可以用io.write把文件写到user_data_path路径在使用。
  --     local user_data_path = io.get_user_data_path()

  --     -- 组装项目跟路径
  --     -- 注意: dating_test_01 需要替换成自己map的文件夹
  --     local map_path = io.get_app_dir() .. '/Res/maps/' .. __MAIN_MAP__ .. '/ui/'
  --     local map_path_local = io.get_root_dir() .. 'Res/maps/clear_test_01/ui/'
  --     local is_local = io.exist_dir(map_path_local)

  --     -- 线上环境需要更改路径
  --     -- 线上环境，因为权限问题io.exist_dir可能会全部false
  --     -- 所以，使用线上优先使用，判断本地环境后更改的策略来替换路径
  --     -- 已知问题: 星火对战平台 win版本不可用。后续有时间再解决。
  --     if is_local then
  --         map_path = map_path_local
  --     end

  --     -- 请替换为实际存在的图片路径
  --     log.info(fmt('%simage/mw.png', map_path))
  --     log.info(fmt('@%s/image/mw.png', __MAIN_MAP__))
  --     -- brush:draw_image(fmt('%simage/mw.png', map_path), 0, 0, 1280, 720)
  --     brush:draw_image(fmt('@%s/image/mw.png', __MAIN_MAP__), 0, 0, 1280, 720)

  --     -- 11. rotate: 旋转画布 (影响后续绘制)
  --     brush:rotate(500, 200, 45)      -- 围绕 (500,200) 旋转 45度
  --     brush:set_fill_color('#00FFFF')
  --     brush:fill_circle(500, 200, 30) -- 这个圆会被旋转(虽然圆转了看不出来，但坐标系变了)
  --     brush:rotate(500, 200, -45)     -- 复原旋转

  --     -- 12. Path API: 贝塞尔曲线
  --     brush:set_line_color('#FF00FF')
  --     brush:set_line_width(3)

  --     -- 移动到起点
  --     brush:path_line_to(50, 300) -- 13. path_line_to

  --     -- 3次贝塞尔曲线: 控制点1, 控制点2, 终点
  --     brush:path_bezier_curve_to(150, 200, 250, 400, 350, 300) -- 14. path_bezier_curve_to

  --     -- 描边 (false 表示不自动闭合路径)
  --     brush:path_stroke(false) -- 15. path_stroke
  -- end)

  -- -- log.info('base.ui.update_event', type(base.ui.update_event))

  -- -- base.ui.update_event(c_ui, 'on_touch_begin', function(self)
  -- --     log.info('on_touch_begin')
  -- -- end)

  -- -- base.ui.update_event(c_ui, 'on_touch_move', function(self)
  -- --     log.info('on_touch_move')
  -- -- end)

  -- -- base.ui.update_event(c_ui, 'on_touch_end', function(self)
  -- --     log.info('on_touch_move')
  -- -- end)

  -- -- base.ui.update_event(c_ui, 'on_click', function(self)
  -- --     log.info('on_click')
  -- -- end)

  -- -- base.event_register(base.game, '鼠标-按下', function(...)
  -- --     -- trg,key
  -- --     -- key : button_left(左键)， button_right(右键),， button_middle(滚轮键)
  -- --     log.info('触发了: 鼠标-按下')
  -- --     log.info(map_self:json_encode({ ... }, 3))
  -- -- end)

  -- base.event_register(base.game, '鼠标-松开', function(...)
  --     -- trg,key
  --     -- key : button_left(左键)， button_right(右键),， button_middle(滚轮键)
  --     log.info('触发了: 鼠标-松开')
  -- end)

  -- base.event_register(base.game, '鼠标-移动', function(...)
  --     -- 只有在鼠标有按键按下时，移动事件才会有效果
  --     -- 移动事件会存在1帧触发多次的情况，多次触发取决于鼠标移动速度
  --     log.info('触发了: 鼠标-移动')
  -- end)


  -- local drawing_bg = base.ui.panel {
  --     layout = {
  --         width = 800,
  --         height = 600,
  --         row_self = 'center',
  --         col_self = 'center'
  --     },

  --     color = 'rgba(0, 0, 0, 0.3)', -- 浅色背景方便看清

  -- }
  -- base.ui.create(drawing_bg)


历史研究
p_lx61 = p_lx61 or {}
local map_self = p_lx61
local component = require '@common.base.gui.component'
local new = component.new
local bind = component.bind
local alias = component.alias

-- 定义画笔颜色常量
local RED = 'rgba(255, 0, 0, 1)'
local GREEN = 'rgba(0, 255, 0, 1)'
local BLUE = 'rgba(0, 0, 255, 1)'
local BLACK = 'rgba(0, 0, 0, 1)'
local YELLOW = 'rgba(255, 255, 0, 1)'
local WHITE = 'rgba(255, 255, 255, 1)'

-- 定义画板变量默认值
local canvas_test -- 画板控件
local handle = 'my_test' -- 画板id
local rotate = 0

-- 配置路径
-- 由于Canvas路径查找是全局路径而不是地图路径，所以这里需要用绝对路径
local root_dir = io.get_root_dir()
local app_dir = io.get_app_dir()

-- user_data_path 这里没用上，先放这里做个备选使用。
-- 当无法很好获取路径的时候，可以用io.write把文件写到user_data_path路径在使用。
local user_data_path = io.get_user_data_path()

-- 组装项目跟路径
-- 注意: dating_test_01 需要替换成自己map的文件夹
local map_path = io.get_app_dir() .. '/Res/maps/' .. __MAIN_MAP__ .. '/ui/'
local map_path_local = io.get_root_dir() .. 'Res/maps/dating_test_01/ui/'
local is_local = io.exist_dir(map_path_local)

-- 线上环境需要更改路径
-- 线上环境，因为权限问题io.exist_dir可能会全部false
-- 所以，使用线上优先使用，判断本地环境后更改的策略来替换路径
-- 已知问题: 星火对战平台 win版本不可用。后续有时间再解决。
if is_local then
    map_path = map_path_local
end

local MyCanvas = component 'MyCanvas' {
    base.ui.panel {
        color = 'rgba(0, 0, 0, 1)', -- 画板背景
        layout = {
            width = 1000,
            height = 800,
        },
        base.ui.canvas '画布' {
            id = handle,
            type = 'canvas',
            color = 'rgba(0, 0, 0, 1)', -- 画笔默认颜色
            layout = {
                width_grow = 1,
                height_grow = 1,
                width_shrink = 1,
                height_shrink = 1,
                row_self = 'start',
                col_self = 'start',
            },
        },
    },
}

function map_self:test_online(...)
    log.info('client: test_online')
    canvas_test = new(MyCanvas {})

    -- canvas 控件创建之后，如需立即使用，则需在下一帧调用才有效果
    base.next(function()
        -- 注意，这里的路径地址使用的前面构建的项目绝对路径: map_path
        ui.draw_image(handle, map_path .. 'image/item/1.png', 0, 0, 100, 100)
    end)
end

function map_self:api_online(...)
    log.info('client: run api_online')

    -- canvas 控件创建之后，相当于创建了一个画板，需要什么自己画上即可。
    -- 由于本方法和 canvas_test 本来就不在一帧里，这里就可以不用base.next了。

    -- 下面依次举例，来演示用法。

    -- 正式画画之前，先把画板擦干净
    -- handle 是前面 base.ui.canvas 的 id, 指定要在哪个画板上作业。
    ui.clear(handle)

    -- 例1: 画线
    -- 从坐标0,0 到 坐标 100,0 ,画一条10像素宽的白线，
    -- 画笔函数: ui.draw_line(handle, x1, y1, x2, y2)
    ui.set_line_width(handle, 10) -- 设置线的宽度
    ui.set_line_color(handle, WHITE) -- 设置线的颜色，这里设置为白色
    -- 根据需求坐标应该是0,0 - 100,0
    -- 由于线宽10像素，所以在高度上加5(线宽的1半)。不然线就会在画板之外了。
    -- 线条的宽度的理解: 扯着线条同时两个方向拉宽，所以1半在外面，1半在里面。
    ui.draw_line(handle, 0, 5, 100, 5)

    -- 例2: 画圆
    -- 在坐标100,120 的位置，画一个半径50的圆，
    -- 另外，需要圆的边框是蓝色10像素，圆填充用黄色。
    -- 画笔函数: ui.draw_circle(handle, x, y, r)
    ui.set_line_width(handle, 10) -- 设置线的宽度
    ui.set_line_color(handle, BLUE) -- 设置制线的颜色，这里设置为蓝色
    ui.set_fill_color(handle, YELLOW) -- 设置制填充颜色，这里设置为黄色
    ui.draw_circle(handle, 100, 120, 50) -- 勾勒圆的线条
    -- 圆的线宽是10，所以加下来填充的圆半径是40。也就是: 外圈半径-线宽
    -- 圆的边框理解: 把边框和内部填充看成是1个圆环套着1个实心圆。
    ui.fill_circle(handle, 100, 120, 40) -- 填充圆的颜色

    -- 例3: 画多边形
    -- 在坐标0,250 的位置，画一个边长200的正方形，
    -- 需要正方形的边框是绿色10像素，正方形填充用红色

    -- 使用多边形画笔, 需要先计算多边形的顶点, 然后顺时针构建画笔点路径.
    -- 计算过后,4个顶点坐标按照顺时针排序是: 左上0,250, 右上200,250, 右下200,450, 左下0,450, 

    -- 画笔函数: ui.draw_polygon(handle, base.json.encode(polygon))
    -- 顺时针 ==> polygon = { { 0, 0 }, { 0, 1 }, { 1, 1 }, { 1, 0 } }
    ui.set_line_width(handle, 10) -- 设置线的宽度
    ui.set_line_color(handle, GREEN) -- 设置制线的颜色，这里设置为绿色
    ui.set_fill_color(handle, RED) -- 设置制填充颜色，这里设置为红色
    ui.draw_polygon(handle, base.json.encode({{0, 250}, {200, 250}, {200, 450}, {0, 450}})) -- 勾勒线条
    ui.fill_polygon(handle, base.json.encode({{5, 255}, {195, 255}, {195, 445}, {5, 445}})) -- 填充颜色

    -- 特别说明:
    -- 1) 事实上,多边形的绘画, 既可以顺时针, 也可以逆时针. 自行实验.
    -- 2) 如果刚创建canvas控件 ,就需要立即使用ui方法绘画, 需要放在下一帧.
    -- 3) brush.lua 里可以看函数定义

    -- 其他一些列子
    -- 5秒之后 清空画板重新画些内容
    base.wait(5000, function()
        local destroy
        local total = 0
        destroy = canvas_test:on_tick(function(delta)
            total = total + delta
            if total > 1000 then
                destroy()
            end

            ui.clear(handle)

            -- 画三角形
            ui.set_line_color(handle, YELLOW)
            ui.set_line_width(handle, 50)
            ui.draw_polygon(handle, base.json.encode({{400, 0}, {300, 50}, {400, 200}}))

            -- 画个正方形
            -- 设置线宽和颜色
            ui.set_line_color(handle, WHITE)
            ui.set_line_width(handle, 5)
            ui.draw_polygon(handle, base.json.encode({{0, 0}, {0, 180}, {180, 180}, {180, 0}}))
            ui.set_fill_color(handle, BLUE)
            ui.fill_polygon(handle, base.json.encode({{5, 5}, {5, 175}, {175, 175}, {175, 5}}))

            -- 画图片
            -- draw_image(path, x, y, w, h)
            ui.draw_image(handle, map_path .. 'image/item/1.png', 0, 300, 100, 100)

            -- 旋转内容
            -- rotate(x, y, angle)
            ui.rotate(handle, 200, 200, 45)
            ui.draw_image(handle, map_path .. 'image/item/3.png', 220, 300, 100, 100)
            ui.rotate(handle, 200, 200, -45)
        end)

    end)
end

function map_self:test_dating(...)
    log.info('client: run test_dating')
end

function map_self:api_dating(...)
    log.info('client: run api_dating')
end


后面高级的用法，输出图片集，还没会

    local handle = 'my_test' -- 画板id
    if key == bgd_const.keyboard.r then
        local canvas = base.ui.canvas '画布' {
            id = handle,
            type = 'canvas',
            color = 'rgba(0, 0, 0, 1)', -- 画笔默认颜色
            layout = {
                width = 1000,
                height = 1000,
                row_self = 'start',
                col_self = 'start',
            },
        }

        local canvas_ui, bind = base.ui.create(canvas)
        log.info(bgd_api.common.json.encode_x(canvas_ui, 2))
        log.info(bgd_api.common.json.encode_x(getmetatable(canvas_ui), 2))
        -- log.info(bgd_api.common.json.encode_x(bind, 2))
        -- log.info(bgd_api.common.json.encode_x(getmetatable(bind), 2))

        local brush = canvas_ui:get_brush()
        log.info(bgd_api.common.json.encode_x(brush, 2))
        log.info(bgd_api.common.json.encode_x(getmetatable(brush), 2))

        -- 定义画笔颜色常量
        local RED = 'rgba(255, 0, 0, 1)'
        local GREEN = 'rgba(0, 255, 0, 1)'
        local BLUE = 'rgba(0, 0, 255, 1)'
        local BLACK = 'rgba(0, 0, 0, 1)'
        local YELLOW = 'rgba(255, 255, 0, 1)'
        local WHITE = 'rgba(255, 255, 255, 1)'
        base.next(function()
            -- ui.set_line_width(handle, 10)        -- 设置线的宽度
            -- ui.set_line_color(handle, BLUE)      -- 设置制线的颜色，这里设置为蓝色
            -- ui.set_fill_color(handle, YELLOW)    -- 设置制填充颜色，这里设置为黄色
            -- ui.draw_circle(handle, 100, 120, 50) -- 勾勒圆的线条
            -- -- 圆的线宽是10，所以加下来填充的圆半径是40。也就是: 外圈半径-线宽
            -- -- 圆的边框理解: 把边框和内部填充看成是1个圆环套着1个实心圆。
            -- ui.fill_circle(handle, 100, 120, 40) -- 填充圆的颜色
            -- ui.draw_image(handle, game.get_map_path() .. '/ui/image/item/1.png', 0, 0, 100, 100)

            -- brush:set_blur(0.3)

            local io = package.loaded['io']
            local path = require '@common.base.path'
            local img_path = tostring(path(game.get_map_path()) / 'ui/image/item/1.png')
            local file = io.open(img_path, "r")
            local content = file:read("*a")
            file:close()

            log.info(#content)
            for i = 1, math.min(100, #content) do
                local byte = content:byte(i)
                log.info(string.format("%02X ", byte))
                if i % 16 == 0 then log.info("\n") end
            end

            ui.canvas_texture_set_compressed_data(handle, { content:byte(1, -1) })
            ui.canvas_texture_get_compressed_data(handle, function(compressed_data)
                local data = cmsg_pack.pack(compressed_data)
                log.info(bgd_api.common.json.encode_x({ compressed_data:byte(1, -1) }, 2))
                log.info(bgd_api.common.json.encode_x(data, 2))
            end)
            -- brush:set_compressed_data(content)
            -- brush:get_compressed_data()
        end)

        log.info('ui.', bgd_api.common.json.encode_x(ui, 3))

        -- brush:set_size(3000, 3000)
        -- brush:set_fill_color('rgba(255, 0, 0, 1)')

        -- local pixel_color = brush:get_pixel_color(1, 1)
        -- log.info(bgd_api.common.json.encode_x(pixel_color, 2))
    end
