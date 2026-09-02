//! lua 宿主（0.5.0 R4 GameHost 编排复刻）：内嵌 lua54 VM 跑项目服务端逻辑（B+ 路线）。
//!
//! **开工先决决策项**（0.5.0 R4 提交说明要求）：
//! ① lua 宿主选型 = **mlua（lua54 + vendored）内嵌**进 mini-runtime 进程（game_host 线程持有 VM，
//!    单进程零 IPC；lua54 = 引擎同款语义：整除/位运算/goto/utf8）。
//! ② lua 运行时物理落盘 = **全部磁盘现读、零内嵌**：项目树 = 控制面接收落盘的
//!    `runtime/User/host_upload/<project>/script/`；引擎包 = 载荷 `_m` 目录
//!    （@lib_* 按 EditorStartGame f12 版本表，server_lua_plus 用本机明文包，
//!    @common.base 由 Rust shim 直接建好 base 后返回桩）；触发器/TSTL 面以容错桩加载（不用即不验）。
//!
//! 消息路由：c2h 0x7006 {f1: cmsg{type,args}} → base.ui.proto[type](player, args)；
//! 服务端 base.game:ui(name, ensure)(data) 广播 / player:ui(name, ensure)(data) 定向 →
//! 0x7008 {f1 cmsg(args), f2 seq, f3 type_id, f4 type_name（首现携带）}（线格式 = §13.9）。
//!
//! 事件泵：event_register(base.game, 事件名, fn) 登记；泵驱动 游戏-帧（50ms）/玩家-连入/断线。
//!
//! shim 面取证：self-host.md §11 + 0.5.0 需求 R4（核对基准 = script-199 common/base/server.lua:184-234）。

use crate::core::cmsg_pack::{self, CVal};
use mlua::{Function, Lua, RegistryKey, Result as LuaResult, Table, Value};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

/// lua → Rust 的出站 0x7008 消息（game_host 排空后发 KCP）
#[derive(Debug)]
pub struct OutMsg {
    /// None = 广播（game:ui），Some(uid) = 定向（player:ui）
    pub target_uid: Option<i64>,
    pub type_name: String,
    pub args: Vec<u8>, // cmsg_pack 字节
}

/// 日志行（lua log.* → 0xF00C + stdout；pos=代码位置，frame=逻辑帧号——编辑器调试信息面板列）
#[derive(Debug)]
pub struct LogLine {
    pub level: String,
    pub text: String,
    pub pos: String,
    pub frame: u64,
}

/// 调度器任务（base.next / wait / timer_wait / timer_loop）
struct Task {
    due: Instant,
    interval: Option<Duration>,
    key: RegistryKey,
}

/// 事件 handler 两种形态（官方语义）：
/// - Simple（event_register）：(trg=nil, ...位置参数)，如 玩家-按键按下 → (nil, player, key)
/// - Trigger（trigger:add_event_common）：(当前触发器, e 表)，e = {evt_name, player?, key?, key_keyboard?}
enum Handler {
    Simple(RegistryKey),
    Trigger { cb: RegistryKey, trig: RegistryKey },
}

/// 共享内部状态（闭包捕获）
struct Inner {
    out: VecDeque<OutMsg>,
    logs: VecDeque<LogLine>,
    /// event_register 登记表：事件名 → handler 列表
    events: HashMap<String, Vec<Handler>>,
    /// 玩家表：uid → （昵称， 注册表键指向 player 表）
    players: HashMap<i64, (String, RegistryKey)>,
    /// 0x7008 type_id 分配（官方从 5 起；id 全局一致，f4 名字按会话首现由 game_host 侧判定）
    type_ids: HashMap<String, u32>,
    next_type_id: u32,
    started: Instant,
    tasks: Vec<Task>,
    /// 数编缓存（eff.init_cache 填充）：'$$id' → CVal
    data_cache: HashMap<String, CVal>,
    cache_inited: bool,
    /// 收到但 type 未注册的消息（记录一次用于排障）
    unrouted_once: Vec<String>,
    /// 逻辑帧号（pump_frame 每帧 +1；日志「帧号」列）
    frame_no: u64,
}

pub struct LuaBrain {
    lua: Lua,
    inner: Rc<RefCell<Inner>>,
    script_root: PathBuf,
    payload_m: PathBuf,
    lua_plus_root: PathBuf,
    libs: HashMap<String, String>,
}

/// 包解析失败时的容错桩（记录并返回空表）——加载期宽容策略
fn stub_module(lua: &Lua, name: &str) -> LuaResult<Table> {
    crate::srv_log!("[lua-host] 包缺失容错桩: {name}");
    lua.create_table()
}

impl LuaBrain {
    /// 创建并执行加载链（script/main.lua）
    pub fn new(
        upload_script: PathBuf,
        runtime_dir: &Path,
        env_domain: &str,
        libs: &[(String, String)],
        project: &str,
    ) -> LuaResult<Self> {
        let payload_m = runtime_dir
            .join("Update")
            .join(env_domain)
            .join("Res")
            .join("_m");
        let lua_plus_root = PathBuf::from(r"D:\sce_online\Update\editor-pd.spark.xd.com\Res\_m\maps\server_lua_plus\14");
        // 引擎 lua 是全库（含 debug/io/os——bgd_libs_server.common.api.log 用 debug 实证）；unsafe_new 即全库
        let lua = unsafe { Lua::unsafe_new() };
        let inner = Rc::new(RefCell::new(Inner {
            out: VecDeque::new(),
            logs: VecDeque::new(),
            events: HashMap::new(),
            players: HashMap::new(),
            type_ids: HashMap::new(),
            next_type_id: 5,
            started: Instant::now(),
            tasks: Vec::new(),
            data_cache: HashMap::new(),
            cache_inited: false,
            unrouted_once: Vec::new(),
            frame_no: 0,
        }));
        let brain = Self {
            lua,
            inner,
            script_root: upload_script,
            payload_m,
            lua_plus_root,
            libs: libs.iter().cloned().collect(),
        };
        brain.install_globals(project)?;
        brain.install_require();
        // 数编缓存装载（obj 树）
        brain.load_data_cache();
        // 加载链：script/main.lua
        let main_path = brain.script_root.join("main.lua");
        let chunk = std::fs::read(&main_path)
            .map_err(|e| mlua::Error::runtime(format!("读服务端入口失败 {}: {e}", crate::core::disp(&main_path))))?;
        let name = format!("@{}", crate::core::disp(&main_path));
        brain
            .lua
            .load(&sanitize_lua(&chunk))
            .set_name(&name)
            .exec()
            .map_err(|e| mlua::Error::runtime(format!("服务端 main.lua 执行失败: {e}")))?;
        crate::srv_log!("[lua-host] 服务端 lua 加载链完成");
        Ok(brain)
    }

    /// 排空出站消息（game_host 每轮调用）
    pub fn drain_out(&self) -> Vec<OutMsg> {
        self.inner.borrow_mut().out.drain(..).collect()
    }

    /// 排空日志
    pub fn drain_logs(&self) -> Vec<LogLine> {
        self.inner.borrow_mut().logs.drain(..).collect()
    }

    /// type_id 分配（id 全局一致；f4 是否带名字 = 每会话首现，由 game_host 侧会话状态判定）
    pub fn type_id_of(&self, name: &str) -> u32 {
        let mut g = self.inner.borrow_mut();
        if let Some(&id) = g.type_ids.get(name) {
            id
        } else {
            let id = g.next_type_id;
            g.next_type_id += 1;
            g.type_ids.insert(name.to_string(), id);
            id
        }
    }

    // ================= 事件泵 =================

    /// 帧泵（game_host 每轮调用）：到期任务 + 游戏-帧（调用方控频）
    pub fn pump_frame(&self) {
        self.inner.borrow_mut().frame_no += 1;
        self.fire_event("游戏-帧", &[Value::Nil], &[]);
        // 到期调度任务
        let due: Vec<Task> = {
            let mut g = self.inner.borrow_mut();
            let now = Instant::now();
            let mut out = Vec::new();
            let mut keep = Vec::new();
            for t in g.tasks.drain(..) {
                if t.due <= now {
                    out.push(t);
                } else {
                    keep.push(t);
                }
            }
            g.tasks = keep;
            out
        };
        for t in due {
            let r: LuaResult<()> = (|| {
                let f: Function = self.lua.registry_value(&t.key)?;
                f.call::<()>(())
            })();
            if let Err(e) = r {
                self.log_line("error", &format!("调度任务抛错: {e}"));
            }
            if let Some(interval) = t.interval {
                // timer_loop：重排
                let mut g = self.inner.borrow_mut();
                g.tasks.push(Task {
                    due: Instant::now() + interval,
                    interval: Some(interval),
                    key: t.key,
                });
            } else {
                let _ = self.lua.remove_registry_value(t.key);
            }
        }
    }

    /// 事件泵出口：Simple 位参直调；Trigger 组 e 表（evt_name + e_fields）带触发器对象
    fn fire_event(&self, name: &str, args: &[Value], e_fields: &[(&str, Value)]) {
        // RegistryKey 不可 Clone：借内一次解出全部 handler（函数 + 触发器表）
        let handlers: Vec<(Function, Option<Table>)> = {
            let g = self.inner.borrow();
            g.events
                .get(name)
                .map(|hs| {
                    hs.iter()
                        .filter_map(|h| {
                            match h {
                                Handler::Simple(k) => self
                                    .lua
                                    .registry_value::<Function>(k)
                                    .ok()
                                    .map(|f| (f, None)),
                                Handler::Trigger { cb, trig } => {
                                    let f = self.lua.registry_value::<Function>(cb).ok()?;
                                    let t = self.lua.registry_value::<Table>(trig).ok()?;
                                    Some((f, Some(t)))
                                }
                            }
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        for (f, trig) in handlers {
            let r: LuaResult<()> = match trig {
                None => {
                    // mlua 0.10.5 Vec<Value> 传参会吞前导 nil（test/temp/mlua_probe 实证）→ 必须 Variadic
                    f.call::<()>(mlua::Variadic::from_iter(args.iter().cloned()))
                }
                Some(trig) => (|| {
                    let e = self.lua.create_table()?;
                    e.set("evt_name", name)?;
                    e.set("event_name", name)?;
                    for (k, v) in e_fields {
                        e.set(*k, v.clone())?;
                    }
                    f.call::<()>((trig, e))
                })(),
            };
            if let Err(e) = r {
                let arg_types: Vec<&str> = args.iter().map(|a| a.type_name()).collect();
                self.log_line("error", &format!("事件[{name}] handler 抛错: {e}（实参 {arg_types:?}）"));
            }
        }
    }

    /// 玩家连入（login 后调用；is_reconnect = 该 uid 已有数据）
    pub fn player_join(&self, uid: i64, nick: &str) {
        let is_reconnect = self.inner.borrow().players.contains_key(&uid);
        let player = match self.make_player(uid, nick) {
            Ok(p) => p,
            Err(e) => {
                self.log_line("error", &format!("建玩家对象失败: {e}"));
                return;
            }
        };
        // 入玩家表
        {
            let key = self.lua.create_registry_value(player.clone()).unwrap();
            self.inner
                .borrow_mut()
                .players
                .insert(uid, (nick.to_string(), key));
        }
        self.fire_event(
            "玩家-连入",
            &[Value::Nil, Value::Table(player.clone()), Value::Boolean(is_reconnect)],
            &[("player", Value::Table(player))],
        );
    }

    /// 玩家断线
    pub fn player_leave(&self, uid: i64) {
        // RegistryKey 不可 Clone：借内直接解出 player 表
        let player = {
            let g = self.inner.borrow();
            g.players
                .get(&uid)
                .and_then(|(_, k)| self.lua.registry_value::<Value>(k).ok())
        };
        let Some(Value::Table(player)) = player else { return };
        self.fire_event(
            "玩家-断线",
            &[Value::Nil, Value::Table(player.clone())],
            &[("player", Value::Table(player))],
        );
        // 玩家表保留（重连用数据在 lua 侧 PlayerManager，这里 key 是否清由框架语义决定——保留以便重连判定）
    }

    /// 客户端玩法上行（0x7006）：分发 base.ui.proto[type](player, args)
    pub fn on_client_msg(&self, uid: i64, cmsg: &[u8]) {
        let Some((CVal::Map(entries), _)) = cmsg_pack::unpack(cmsg) else {
            self.log_line("warn", "0x7006 内层非 cmsg map");
            return;
        };
        let mut ty: Option<String> = None;
        let mut args = CVal::Nil;
        for (k, v) in &entries {
            match k {
                CVal::Str(s) if s == b"type" => {
                    if let CVal::Str(s) = v {
                        ty = Some(String::from_utf8_lossy(s).to_string());
                    }
                }
                CVal::Str(s) if s == b"args" => args = v.clone(),
                _ => {}
            }
        }
        let Some(ty) = ty else { return };
        // 引擎内建通道（不走 base.ui.proto，官方由 host 原生转事件——script-199 game.lua:517-525 实证）：
        // __client_key_down/up {player_id, key} → 玩家-按键按下/松开
        if ty == "__client_key_down" || ty == "__client_key_up" {
            let event = if ty == "__client_key_down" { "玩家-按键按下" } else { "玩家-按键松开" };
            let key_str = match &args {
                CVal::Map(entries) => entries.iter().find_map(|(k, v)| match (k, v) {
                    (CVal::Str(k), CVal::Str(v)) if k == b"key" => {
                        Some(String::from_utf8_lossy(v).to_string())
                    }
                    _ => None,
                }),
                _ => None,
            };
            let Some(key_str) = key_str else { return };
            let player_val = {
                let g = self.inner.borrow();
                match g.players.get(&uid) {
                    Some((_, k)) => self.lua.registry_value::<Value>(k).unwrap_or(Value::Nil),
                    None => Value::Nil,
                }
            };
            let key_val = Value::String(self.lua.create_string(&key_str).unwrap());
            self.fire_event(
                event,
                &[Value::Nil, player_val.clone(), key_val.clone()],
                &[("player", player_val), ("key", key_val.clone()), ("key_keyboard", key_val)],
            );
            return;
        }
        let r: LuaResult<()> = (|| {
            let base: Table = self.lua.globals().get("base")?;
            let ui: Table = base.get("ui")?;
            let proto: Table = ui.get("proto")?;
            let handler: Value = proto.get(ty.as_str())?;
            let Value::Function(f) = handler else {
                // 未注册：每种 type 记一次（对照云端行为的排障入口）
                let mut g = self.inner.borrow_mut();
                if !g.unrouted_once.contains(&ty) {
                    g.unrouted_once.push(ty.clone());
                    drop(g);
                    self.log_line("warn", &format!("玩法上行未注册 handler: {ty}"));
                }
                return Ok(());
            };
            let args_lua = cmsg_pack::to_lua(&self.lua, &args)?;
            // player 对象（RegistryKey 不可 Clone：借内直接解出）
            let player_val = {
                let g = self.inner.borrow();
                match g.players.get(&uid) {
                    Some((_, k)) => self.lua.registry_value::<Value>(k)?,
                    None => Value::Nil,
                }
            };
            f.call::<()>((player_val, args_lua))
        })();
        if let Err(e) = r {
            self.log_line("error", &format!("玩法 handler[{ty}] 抛错: {e}"));
        }
    }

    // ================= 全局环境 =================

    fn log_line(&self, level: &str, text: &str) {
        crate::srv_log!("[lua] {text}");
        push_log_inner(&self.inner, level, text);
    }

    fn make_player(&self, uid: i64, nick: &str) -> LuaResult<Table> {
        let t = self.lua.create_table()?;
        t.raw_set("__uid", uid)?;
        t.raw_set("__nick", nick)?;
        // get_nick_name
        t.set(
            "get_nick_name",
            self.lua.create_function(|_, t: Table| t.raw_get::<String>("__nick"))?,
        )?;
        // ui 定向消息：player:ui(name, ensure)(data)
        let inner = Rc::clone(&self.inner);
        t.set(
            "ui",
            self.lua.create_function(move |lua, (t, name, _ensure): (Table, String, Value)| {
                let uid: i64 = t.raw_get("__uid")?;
                let inner = Rc::clone(&inner);
                Ok(lua.create_function(move |_lua, data: Value| {
                    let args = cmsg_pack::from_lua(&data)?;
                    let bytes = cmsg_pack::pack_to_vec(&args);
                    inner.borrow_mut().out.push_back(OutMsg {
                        target_uid: Some(uid),
                        type_name: name.clone(),
                        args: bytes,
                    });
                    Ok(())
                })?)
            })?,
        )?;
        Ok(t)
    }

    fn install_globals(&self, project: &str) -> LuaResult<()> {
        let g = self.lua.globals();
        // 引擎全局标记（oracle 实证，self-host.md §11）
        g.set("__IN_HOST__", true)?;
        g.set("__MAIN_MAP__", project)?;
        g.set("__GAME_ID__", project)?;

        // ---- log ----
        let log = self.lua.create_table()?;
        for level in ["info", "warn", "error", "debug"] {
            let inner = Rc::clone(&self.inner);
            let lv = level.to_string();
            log.set(
                level,
                self.lua.create_function(move |_, args: mlua::Variadic<Value>| {
                    let text = args
                        .iter()
                        .map(|v| match v {
                            Value::String(s) => s.to_string_lossy(),
                            other => format!("{other:?}"),
                        })
                        .collect::<Vec<_>>()
                        .join("\t");
                    crate::srv_log!("[lua] {text}");
                    push_log_inner(&inner, &lv, &text);
                    Ok(())
                })?,
            )?;
        }
        log.set("set_level", self.lua.create_function(|_, _: Value| Ok(()))?)?;
        log.set("set_keyword", self.lua.create_function(|_, _: Value| Ok(()))?)?;
        g.set("log", log)?;

        // ---- print → log.info ----
        {
            let inner = Rc::clone(&self.inner);
            g.set(
                "print",
                self.lua.create_function(move |_, args: mlua::Variadic<Value>| {
                    let text = args
                        .iter()
                        .map(|v| match v {
                            Value::String(s) => s.to_string_lossy(),
                            other => format!("{other:?}"),
                        })
                        .collect::<Vec<_>>()
                        .join("\t");
                    crate::srv_log!("[lua] {text}");
                    push_log_inner(&inner, "info", &text);
                    Ok(())
                })?,
            )?;
        }

        // ---- cmsg_pack ----
        let cmsg = self.lua.create_table()?;
        cmsg.set(
            "pack",
            self.lua.create_function(|_, v: Value| {
                let cv = cmsg_pack::from_lua(&v)?;
                Ok(pack_to_lua_string(&cmsg_pack::pack_to_vec(&cv)))
            })?,
        )?;
        cmsg.set(
            "unpack",
            self.lua.create_function(|lua, s: mlua::String| {
                let bytes = s.as_bytes();
                match cmsg_pack::unpack(&bytes) {
                    Some((v, _)) => cmsg_pack::to_lua(lua, &v),
                    None => Err(mlua::Error::external("cmsg_pack.unpack 解析失败")),
                }
            })?,
        )?;
        g.set("cmsg_pack", cmsg)?;

        // ---- require_folder ----
        {
            let script_root = self.script_root.clone();
            g.set(
                "require_folder",
                self.lua.create_function(move |lua, name: String| {
                    require_folder(lua, &script_root, &name)
                })?,
            )?;
        }

        // ---- base ----
        self.install_base(&g, project)?;
        Ok(())
    }

    fn install_base(&self, g: &Table, _project: &str) -> LuaResult<()> {
        let base = self.lua.create_table()?;

        // base.event_register(owner, 事件名, fn)
        {
            let inner = Rc::clone(&self.inner);
            base.set(
                "event_register",
                self.lua.create_function(move |lua, (_owner, name, f): (Value, String, Function)| {
                    let key = lua.create_registry_value(f)?;
                    inner
                        .borrow_mut()
                        .events
                        .entry(name)
                        .or_default()
                        .push(Handler::Simple(key));
                    Ok(Value::Nil)
                })?,
            )?;
        }

        // base.game
        let game = self.lua.create_table()?;
        // game:ui(name, ensure)(data) — 广播
        {
            let inner = Rc::clone(&self.inner);
            game.set(
                "ui",
                self.lua.create_function(move |lua, (_t, name, _ensure): (Table, String, Value)| {
                    let inner = Rc::clone(&inner);
                    Ok(lua.create_function(move |_lua, data: Value| {
                        let args = cmsg_pack::from_lua(&data)?;
                        let bytes = cmsg_pack::pack_to_vec(&args);
                        inner.borrow_mut().out.push_back(OutMsg {
                            target_uid: None,
                            type_name: name.clone(),
                            args: bytes,
                        });
                        Ok(())
                    })?)
                })?,
            )?;
        }
        // game:server（服务端侧死代码容错桩：返回吃掉参数的函数）
        game.set(
            "server",
            self.lua.create_function(|lua, _name: Value| {
                lua.create_function(|_, _: Value| Ok(()))
            })?,
        )?;
        // init_units（引擎世界模拟面，B+ 路线不模拟地编单位 → 桩）
        game.set("init_units", self.lua.create_function(|_, _: ()| Ok(()))?)?;
        base.set("game", game)?;

        // base.ui = { proto = {}, bind = {} }；base.proto = {}
        let ui = self.lua.create_table()?;
        ui.set("proto", self.lua.create_table()?)?;
        ui.set("bind", self.lua.create_table()?)?;
        base.set("ui", ui)?;
        base.set("proto", self.lua.create_table()?)?;

        // base.auxiliary.get_player_id(player)
        base.set(
            "auxiliary",
            self.lua.create_table_from([(
                "get_player_id",
                Value::Function(self.lua.create_function(|_, p: Value| {
                    match &p {
                        Value::Table(t) => t.raw_get::<i64>("__uid"),
                        other => Err(mlua::Error::runtime(format!(
                            "get_player_id 参数类型异常: {}",
                            other.type_name()
                        ))),
                    }
                })?),
            )])?,
        )?;

        // base.clock()：逻辑时钟 = 脑启动起算的【毫秒】（官方 timer.lua cur_frame 按 on_update(delta*1000) 步进，
        // test_res002 GameConfig.BOSS_SPAWN 毫秒值 + PlayerCombat (now-last)/1000 双向实证）
        {
            let started = self.inner.borrow().started;
            base.set(
                "clock",
                self.lua.create_function(move |_, _: ()| Ok(started.elapsed().as_secs_f64() * 1000.0))?,
            )?;
        }

        // 调度器：next(fn) / wait(sec, fn) / timer_wait(fn, sec) / timer_loop(fn, sec)
        {
            let inner = Rc::clone(&self.inner);
            base.set(
                "next",
                self.lua.create_function(move |lua, f: Function| {
                    let key = lua.create_registry_value(f)?;
                    inner.borrow_mut().tasks.push(Task {
                        due: Instant::now(),
                        interval: None,
                        key,
                    });
                    Ok(())
                })?,
            )?;
        }
        {
            let inner = Rc::clone(&self.inner);
            base.set(
                "wait",
                // 官方 base.wait(timeout, on_timer)，timeout 单位毫秒（timer.lua cur_frame+timeout，帧=1ms）
                self.lua.create_function(move |lua, (ms, f): (f64, Function)| {
                    let key = lua.create_registry_value(f)?;
                    inner.borrow_mut().tasks.push(Task {
                        due: Instant::now() + Duration::from_secs_f64((ms / 1000.0).max(0.0)),
                        interval: None,
                        key,
                    });
                    Ok(())
                })?,
            )?;
        }
        {
            let inner = Rc::clone(&self.inner);
            base.set(
                "timer_wait",
                // base_lua_plus 秒级封装：timer_wait(time秒, func)
                self.lua.create_function(move |lua, (secs, f): (f64, Function)| {
                    let key = lua.create_registry_value(f)?;
                    inner.borrow_mut().tasks.push(Task {
                        due: Instant::now() + Duration::from_secs_f64(secs.max(0.0)),
                        interval: None,
                        key,
                    });
                    Ok(())
                })?,
            )?;
        }
        {
            let inner = Rc::clone(&self.inner);
            base.set(
                "timer_loop",
                // base_lua_plus 秒级封装：timer_loop(time秒, func)
                self.lua.create_function(move |lua, (secs, f): (f64, Function)| {
                    let key = lua.create_registry_value(f)?;
                    inner.borrow_mut().tasks.push(Task {
                        due: Instant::now() + Duration::from_secs_f64(secs.max(0.01)),
                        interval: Some(Duration::from_secs_f64(secs.max(0.01))),
                        key,
                    });
                    Ok(())
                })?,
            )?;
        }

        // base.math 扩展（lua math 的超集面，游戏用到的）
        let mathx = self.lua.create_table()?;
        {
            let lmath: Table = g.get("math")?;
            mathx.set("min", lmath.get::<Value>("min")?)?;
            mathx.set("max", lmath.get::<Value>("max")?)?;
            mathx.set("ceil", lmath.get::<Value>("ceil")?)?;
            mathx.set("abs", lmath.get::<Value>("abs")?)?;
            mathx.set("floor", lmath.get::<Value>("floor")?)?;
            mathx.set("sqrt", lmath.get::<Value>("sqrt")?)?;
            mathx.set("random", lmath.get::<Value>("random")?)?;
            mathx.set("pi", lmath.get::<Value>("pi")?)?;
        }
        mathx.set(
            "random_int",
            self.lua.create_function(|_, (a, b): (i64, i64)| {
                Ok(rand_range(a, b))
            })?,
        )?;
        mathx.set(
            "included_angle",
            self.lua.create_function(|_, _: Value| Ok(0.0f64))?,
        )?;
        base.set("math", mathx)?;

        // base.eff：数编缓存
        let eff = self.lua.create_table()?;
        {
            let inner = Rc::clone(&self.inner);
            eff.set(
                "has_cache_init",
                self.lua.create_function(move |_, _: ()| Ok(inner.borrow().cache_inited))?,
            )?;
        }
        {
            let inner = Rc::clone(&self.inner);
            eff.set(
                "init_cache",
                self.lua.create_function(move |_, _: ()| {
                    inner.borrow_mut().cache_inited = true;
                    Ok(())
                })?,
            )?;
        }
        {
            let inner = Rc::clone(&self.inner);
            eff.set(
                "cache",
                self.lua.create_function(move |lua, id: Value| {
                    let key = match &id {
                        Value::String(s) => s.to_string_lossy(),
                        Value::Integer(i) => i.to_string(),
                        Value::Number(n) => n.to_string(),
                        other => format!("{other:?}"),
                    };
                    match inner.borrow().data_cache.get(&key).cloned() {
                        Some(v) => {
                            let lv = cmsg_pack::to_lua(lua, &v)?;
                            // trigger_validator 会对 spell 类条目直接 data.Formulas.X = fn；
                            // obj 原文不含 Formulas 空表（官方 cache 有 schema 默认值注入），此处补齐
                            if let Value::Table(t) = &lv {
                                if matches!(t.raw_get::<Value>("Formulas"), Ok(Value::Nil)) {
                                    t.raw_set("Formulas", lua.create_table()?)?;
                                }
                            }
                            Ok(lv)
                        }
                        None => Ok(Value::Nil),
                    }
                })?,
            )?;
        }
        base.set("eff", eff)?;

        // base.tsc（TSTL 类库：lua_declare.lua 全量建类链，必须真实现 prototype/继承/实例化）
        let tsc = self.lua.create_table()?;
        let classes = self.lua.create_table()?;
        // 引擎预置 CLASSES.os = 真 os 库（TSTL 产物 `os = CLASSES.os or __TS__Class2("os")` 会覆盖全局 os——
        // 缺此项则 os.time 全灭，test_res002 ShopSystem.lua:63 实证）
        classes.set("os", g.get::<Value>("os")?)?;
        tsc.set("CLASSES", classes)?;
        // TSTL 类壳：c.prototype.__index = c.prototype（字段）+ constructor + __call 实例化（____constructor）
        let make_class = |lua: &Lua, name: Value| -> LuaResult<Table> {
            let c = lua.create_table()?;
            let proto = lua.create_table()?;
            proto.set("__index", proto.clone())?;
            proto.set("constructor", c.clone())?;
            c.set("prototype", proto)?;
            c.set("__name", name)?;
            let mt = lua.create_table()?;
            mt.set(
                "__call",
                lua.create_function(|lua, (cls, args): (Table, mlua::Variadic<Value>)| {
                    let proto: Table = cls.get("prototype")?;
                    let inst = lua.create_table()?;
                    inst.set_metatable(Some(proto.clone()));
                    if let Value::Function(f) = proto.get::<Value>("____constructor")? {
                        let mut call_args = vec![Value::Table(inst.clone())];
                        call_args.extend(args.into_iter());
                        f.call::<()>(mlua::Variadic::from_iter(call_args))?;
                    }
                    Ok(Value::Table(inst))
                })?,
            )?;
            c.set_metatable(Some(mt));
            Ok(c)
        };
        tsc.set(
            "__TS__Class2",
            self.lua
                .create_function(move |lua, name: String| make_class(lua, Value::String(lua.create_string(&name)?)))?,
        )?;
        tsc.set(
            "__TS__Class",
            self.lua
                .create_function(move |lua, _: ()| make_class(lua, Value::Nil))?,
        )?;
        // 继承：child.____super = parent；child.prototype metatable __index = parent.prototype（链式上溯）
        tsc.set(
            "__TS__ClassExtends",
            self.lua.create_function(
                |lua, (child, parent, _factory): (Table, Table, Value)| {
                    child.set("____super", parent.clone())?;
                    let proto: Table = child.get("prototype")?;
                    let parent_proto: Table = parent.get("prototype")?;
                    let mt = lua.create_table()?;
                    mt.set("__index", parent_proto)?;
                    proto.set_metatable(Some(mt));
                    Ok(())
                },
            )?,
        )?;
        // 超类构造包装：(cls, args, ctor) → fn(self) ctor(self, unpack(args)) end
        tsc.set(
            "__TS__SuperTypeArgumentsFuncWrapper",
            self.lua.create_function(
                |lua, (_cls, args, ctor): (Value, Table, Function)| {
                    lua.create_function(move |_, selfv: Value| {
                        let mut call_args = vec![selfv];
                        for a in args.clone().sequence_values::<Value>() {
                            call_args.push(a?);
                        }
                        ctor.call::<()>(mlua::Variadic::from_iter(call_args))
                    })
                },
            )?,
        )?;
        base.set("tsc", tsc)?;

        // 触发器/效果参数/场景几何/类型判别等引擎面 → 容错桩（B+ 路线不用；调用即空转）
        for name in [
            "scene_point", "circle", "get_area_unit", "unit", "unit_all_skill",
            "table_to_point", "instance_of", "force_as", "target_filter_validate",
            "target_filter_validate_on_unit", "object_store_value", "object_restore_value",
            "eff_param_caster", "eff_param_target_unit", "eff_param_target_point",
            "eff_param_cast_phase", "eff_param_main_target_unit", "eff_param_origin_point",
            "eff_param_get_by_name", "eff_param_get_parent", "eff_param_get_cache",
            "eff_param_level_data", "error_pending_kill",
        ] {
            base.set(name, self.lua.create_function(|_, _: mlua::Variadic<Value>| Ok(Value::Nil))?)?;
        }
        // base.trigger_new(fn, ...) → 真触发器对象：add_event_common 登记进事件泵（test_res002 F1测试 实证必需），
        // 其余方法经 metatable __index 容错空转
        {
            let inner = Rc::clone(&self.inner);
            base.set(
                "trigger_new",
                self.lua.create_function(move |lua, args: mlua::Variadic<Value>| {
                    let cb = args.iter().find_map(|v| match v {
                        Value::Function(f) => Some(f.clone()),
                        _ => None,
                    });
                    let trig = lua.create_table()?;
                    if let Some(f) = cb {
                        trig.raw_set("__cb", f)?;
                    }
                    let inner2 = Rc::clone(&inner);
                    trig.set(
                        "add_event_common",
                        lua.create_function(move |lua, (t, opt): (Table, Table)| {
                            let event: String = opt.get("event_name")?;
                            if let Value::Function(f) = t.raw_get::<Value>("__cb")? {
                                let cb = lua.create_registry_value(f)?;
                                let trig_key = lua.create_registry_value(t)?;
                                inner2.borrow_mut().events.entry(event).or_default().push(Handler::Trigger {
                                    cb,
                                    trig: trig_key,
                                });
                            }
                            Ok(())
                        })?,
                    )?;
                    let mt = lua.create_table()?;
                    mt.set(
                        "__index",
                        lua.create_function(|lua, _: (Table, Value)| {
                            lua.create_function(|_, _: mlua::Variadic<Value>| Ok(Value::Nil))
                        })?,
                    )?;
                    trig.set_metatable(Some(mt));
                    Ok(Value::Table(trig))
                })?,
            )?;
        }
        // base.any_player / base.target_filters / base.backend / base.eff_param（表桩）
        for name in ["any_player", "target_filters", "backend", "eff_param"] {
            base.set(name, self.lua.create_table()?)?;
        }
        // base.json（lni/json 面：bgd 框架用自己的纯 lua json；引擎 json 兜底）
        base.set("json", self.lua.create_table()?)?;

        // base 兜底 __index：未登记键首次访问给容错函数（记一行日志，防加载期炸）
        {
            let inner = Rc::clone(&self.inner);
            let mt = self.lua.create_table()?;
            mt.set(
                "__index",
                self.lua.create_function(move |lua, (_t, k): (Table, Value)| {
                    let name = match &k {
                        Value::String(s) => s.to_string_lossy(),
                        other => format!("{other:?}"),
                    };
                    crate::srv_log!("[lua-host] base.{name} 未 shim（容错空函数）");
                    let _ = &inner; // 占位（预留计数）
                    lua.create_function(|_, _: mlua::Variadic<Value>| Ok(Value::Nil))
                })?,
            )?;
            base.set_metatable(Some(mt));
        }

        g.set("base", base)?;
        Ok(())
    }

    // ================= 包加载 =================

    fn install_require(&self) {
        // 完全自研 require：模块名 → 文件解析 + package.loaded 缓存
        let g = self.lua.globals();
        let loaded = self.lua.create_table().unwrap();
        g.set("__loaded", loaded.clone()).unwrap();
        let script_root = self.script_root.clone();
        let payload_m = self.payload_m.clone();
        let lua_plus_root = self.lua_plus_root.clone();
        let libs = self.libs.clone();
        let require = self
            .lua
            .create_function(move |lua, name: String| {
                // 缓存
                let loaded: Table = lua.globals().get("__loaded")?;
                if let Value::Table(t) = loaded.get(name.as_str())? {
                    return Ok(Value::Table(t));
                }
                if let Value::Boolean(true) = loaded.get(name.as_str())? {
                    return Ok(Value::Boolean(true));
                }
                let v = load_module(lua, &script_root, &payload_m, &lua_plus_root, &libs, &name)?;
                loaded.set(name.as_str(), v.clone())?;
                Ok(v)
            })
            .unwrap();
        g.set("require", require).unwrap();
        // package 兼容面（框架 lua 可能摸 package.loaded）
        let package = self.lua.create_table().unwrap();
        package.set("loaded", loaded).unwrap();
        package.set("path", Value::String(self.lua.create_string("").unwrap())).unwrap();
        g.set("package", package).unwrap();
    }

    /// 数编缓存：obj/**/*.lua 逐个执行，合并返回的 module_datas（'$$id' → 数据表）
    fn load_data_cache(&self) {
        let obj_root = self.script_root.join("obj");
        if !obj_root.is_dir() {
            return;
        }
        let mut files = Vec::new();
        collect_lua(&obj_root, &mut files);
        files.sort();
        let mut count = 0usize;
        for f in files {
            let Ok(chunk) = std::fs::read(&f) else { continue };
            let name = format!("@{}", crate::core::disp(&f));
            let r: LuaResult<Value> = self.lua.load(&sanitize_lua(&chunk)).set_name(&name).eval();
            if let Ok(Value::Table(t)) = r {
                for pair in t.pairs::<Value, Value>().flatten() {
                    let (k, v) = pair;
                    let key = match &k {
                        Value::String(s) => s.to_string_lossy(),
                        other => format!("{other:?}"),
                    };
                    if let Ok(cv) = cmsg_pack::from_lua(&v) {
                        self.inner.borrow_mut().data_cache.insert(key, cv);
                        count += 1;
                    }
                }
            }
        }
        crate::srv_log!("[lua-host] 数编缓存: {count} 条（obj 树 {} 文件）", count_files(&self.script_root.join("obj")));
        self.inner.borrow_mut().cache_inited = true;
    }
}

fn count_files(dir: &Path) -> usize {
    let mut n = 0;
    collect_lua(dir, &mut Vec::new());
    // collect_lua 只收文件数
    fn count(d: &Path, n: &mut usize) {
        if let Ok(rd) = std::fs::read_dir(d) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    count(&p, n);
                } else if p.extension().map(|e| e == "lua").unwrap_or(false) {
                    *n += 1;
                }
            }
        }
    }
    count(dir, &mut n);
    n
}

fn collect_lua(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                collect_lua(&p, out);
            } else if p.extension().map(|e| e == "lua").unwrap_or(false) {
                out.push(p);
            }
        }
    }
}

fn pack_to_lua_string(bytes: &[u8]) -> Vec<u8> {
    bytes.to_vec()
}

/// 日志入队统一入口：pos 从文本抓 `[<path>.lua:<行号>]`（bgd log 模块已把调用点格式进文本），frame 取当前逻辑帧号
fn push_log_inner(inner: &Rc<RefCell<Inner>>, level: &str, text: &str) {
    let pos = parse_pos(text);
    let mut g = inner.borrow_mut();
    let frame = g.frame_no;
    g.logs.push_back(LogLine {
        level: level.into(),
        text: text.into(),
        pos,
        frame,
    });
}

/// 从 bgd 格式化日志文本提取代码位置（`[xxx.lua:123]` → `xxx.lua:123`；抓不到给 "lua"）
fn parse_pos(text: &str) -> String {
    if let Some(idx) = text.find(".lua:") {
        let after = &text[idx + 5..];
        let digits = after.chars().take_while(|c| c.is_ascii_digit()).count();
        if digits > 0 {
            let before = &text[..idx];
            let start = before
                .rfind('[')
                .map(|i| i + 1)
                .or_else(|| before.rfind(' ').map(|i| i + 1))
                .unwrap_or(0);
            return text[start..idx + 5 + digits].to_string();
        }
    }
    "lua".to_string()
}

/// 引擎 lua 词法器放行 ≥0x80 标识符（TSTL 产物含中文参数名，lua_declare.lua:62 oracle 实证）；
/// stock lua54 拒绝。加载前把含非 ASCII 字节的标识符确定性改写为 `_xHH` 序列
/// （字符串/注释原样保留；行号不变；同名同改 → 跨模块引用一致）。
fn sanitize_lua(src: &[u8]) -> Vec<u8> {
    let n = src.len();
    // 无 ≥0x80 字节快路径
    if !src.iter().any(|&b| b >= 0x80) {
        return src.to_vec();
    }
    let mut out = Vec::with_capacity(src.len() + src.len() / 4);
    let mut i = 0usize;
    /// 读长括号起始 [=*[ → 返回等号数；不是长括号返回 None
    fn long_open(src: &[u8], i: usize) -> Option<usize> {
        if src.get(i) != Some(&b'[') {
            return None;
        }
        let mut j = i + 1;
        while src.get(j) == Some(&b'=') {
            j += 1;
        }
        if src.get(j) == Some(&b'[') {
            Some(j - i - 1)
        } else {
            None
        }
    }
    /// 从长括号内容起点跳到闭合 ]=*] 之后
    fn long_close(src: &[u8], mut i: usize, eq: usize) -> usize {
        while i < src.len() {
            if src[i] == b']' {
                let mut j = i + 1;
                let mut k = 0;
                while src.get(j) == Some(&b'=') {
                    j += 1;
                    k += 1;
                }
                if k == eq && src.get(j) == Some(&b']') {
                    return j + 1;
                }
            }
            i += 1;
        }
        src.len()
    }
    while i < n {
        let c = src[i];
        if c == b'-' && src.get(i + 1) == Some(&b'-') {
            // 注释：长注释 --[[ / 行注释
            if let Some(eq) = long_open(src, i + 2) {
                let end = long_close(src, i + 3 + eq, eq);
                out.extend_from_slice(&src[i..end]);
                i = end;
            } else {
                let mut j = i;
                while j < n && src[j] != b'\n' {
                    j += 1;
                }
                out.extend_from_slice(&src[i..j]);
                i = j;
            }
        } else if c == b'\'' || c == b'"' {
            // 短字符串（含转义）
            let mut j = i + 1;
            while j < n {
                if src[j] == b'\\' {
                    j += 2;
                } else if src[j] == c {
                    j += 1;
                    break;
                } else if src[j] == b'\n' {
                    break;
                } else {
                    j += 1;
                }
            }
            let j = j.min(n);
            out.extend_from_slice(&src[i..j]);
            i = j;
        } else if let Some(eq) = long_open(src, i) {
            // 长字符串
            let end = long_close(src, i + 2 + eq, eq);
            out.extend_from_slice(&src[i..end]);
            i = end;
        } else if c == b'_' || c.is_ascii_alphabetic() || c >= 0x80 {
            // 标识符
            let start = i;
            while i < n && (src[i] == b'_' || src[i].is_ascii_alphanumeric() || src[i] >= 0x80) {
                i += 1;
            }
            let name = &src[start..i];
            if name.iter().any(|&b| b >= 0x80) {
                out.push(b'_');
                for &b in name {
                    if b >= 0x80 {
                        out.extend_from_slice(format!("_x{b:02X}").as_bytes());
                    } else {
                        out.push(b);
                    }
                }
            } else {
                out.extend_from_slice(name);
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

fn rand_range(a: i64, b: i64) -> i64 {
    // 简易 xorshift（避免引入 rand 依赖）
    use std::sync::atomic::{AtomicU64, Ordering};
    static S: AtomicU64 = AtomicU64::new(0x9e3779b97f4a7c15);
    let mut x = S.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    S.store(x, Ordering::Relaxed);
    if b <= a {
        return a;
    }
    a + (x % ((b - a + 1) as u64)) as i64
}

/// require_folder：目录名（"scene"）或点分包路径（'bgd_game_server.common.const'）→ 目录下全部 .lua 逐个 require
fn require_folder(lua: &Lua, script_root: &Path, name: &str) -> LuaResult<Value> {
    let dir = script_root.join(name.replace('.', std::path::MAIN_SEPARATOR_STR));
    if !dir.is_dir() {
        crate::srv_log!("[lua-host] require_folder 目录不存在: {name}（容错空转）");
        return Ok(Value::Nil);
    }
    let mut files = Vec::new();
    collect_lua(&dir, &mut files);
    files.sort();
    for f in files {
        // 目录内文件 → 相对 script_root 的点分模块名
        let rel = f.strip_prefix(script_root).unwrap_or(&f);
        let module = rel
            .with_extension("")
            .to_string_lossy()
            .replace(['\\', '/'], ".");
        let require: Function = lua.globals().get("require")?;
        let _: Value = require.call(module)?;
    }
    Ok(Value::Nil)
}

/// 模块名 → 文件 → 执行。形态：
/// - `@common.base` → 返回 base（Rust 已建好）
/// - `@global_default.lua_declare` → 载荷 global_default 的 ui/script/lua_declare.lua（客户端侧文件近似）
/// - `@lib_xxx.<mod>` → 载荷 `_m/maps/script_libs/<name>/<ver>/<name>/script/<mod>.lua`（版本 = EditorStartGame f12）
/// - `@lua_plus.<mod>` → 本机明文 server_lua_plus/14/base/base_lua_plus/<mod>.lua
/// - 其余（bgd_game_server.xxx 等）→ 项目 script/ 树
fn load_module(
    lua: &Lua,
    script_root: &Path,
    payload_m: &Path,
    lua_plus_root: &Path,
    libs: &HashMap<String, String>,
    name: &str,
) -> LuaResult<Value> {
    if name == "@common.base" {
        return lua.globals().get("base");
    }
    let file: Option<PathBuf> = if let Some(rest) = name.strip_prefix("@global_default.") {
        // global_default 包（载荷内 ui/script 半身的 lua_declare 近似服务端用）
        payload_m
            .join("maps")
            .join("global_default")
            .join("60")
            .join("global_default")
            .join("ui")
            .join("script")
            .join(rest.replace('.', "/") + ".lua")
            .into()
    } else if let Some(rest) = name.strip_prefix("@lua_plus.") {
        lua_plus_root
            .join("base")
            .join("base_lua_plus")
            .join(rest.replace('.', "/") + ".lua")
            .into()
    } else if name.starts_with('@') {
        let trimmed = &name[1..];
        let (pkg, module) = trimmed.split_once('.').unwrap_or((trimmed, "main"));
        let ver = libs.get(pkg).cloned().unwrap_or_else(|| "-1".into());
        // script_libs / ai_templates / 顶层包三处候选
        let rel_module = module.replace('.', "/") + ".lua";
        let candidates = [
            payload_m.join("maps").join("script_libs").join(pkg).join(&ver).join(pkg).join("script").join(&rel_module),
            payload_m.join("maps").join("ai_templates").join(pkg).join(&ver).join(pkg).join("script").join(&rel_module),
            payload_m.join("maps").join(pkg).join(&ver).join(pkg).join("script").join(&rel_module),
        ];
        candidates.into_iter().find(|p| p.is_file())
    } else {
        // 项目 script/ 树：<name>.lua 或 <name>/init.lua（包目录约定）
        let rel = name.replace('.', std::path::MAIN_SEPARATOR_STR);
        let dir_init = script_root.join(&rel).join("init.lua");
        if dir_init.is_file() { Some(dir_init) } else { script_root.join(rel + ".lua").into() }
    };
    let Some(file) = file else {
        return stub_module(lua, name).map(Value::Table);
    };
    if !file.is_file() {
        return stub_module(lua, name).map(Value::Table);
    }
    let chunk = std::fs::read(&file).map_err(|e| mlua::Error::external(format!("读模块失败 {}: {e}", file.display())))?;
    let chunk_name = format!("@{}", crate::core::disp(&file));
    lua.load(&sanitize_lua(&chunk))
        .set_name(&chunk_name)
        .eval::<Value>()
        .map_err(|e| mlua::Error::external(format!("模块 {name} 执行失败: {e}")))
}

use std::time::Duration;
