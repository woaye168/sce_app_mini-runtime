# 引擎 common 表全集（运行时枚举 + 注册表逆向双实证）

> 最后验证：2026-08-26（运行时 `pairs(common)` 枚举 517 名 + 双引擎 luaL_Reg 注册表逆向 524 条）
> 本文是 `common` 全局表的分类手册。**逐函数签名（反汇编推断）见分引擎清单**：
> [common-table-editor.md](common-table-editor.md)（星火编辑器 sceengine.dll）/ [common-table-tester.md](common-table-tester.md)（对战平台 scegame）。
> 双引擎注册名集合**完全一致**（524 条逐一比对无差异）。`libs/types/sce_common.d.lua` 只收录约 80 个。

## 0. 机制要点

- **来源**：`common` 表由引擎 C++ 层注入 `_G`（系统级能力：平台/argv/选项/统计/渲染设置/窗口/剪贴板等）。
- **★ 双名注册**：与 LuaIO 同机制——绝大多数函数同时注册**小写与 PascalCase**两个名字（含 `exit`/`Exit`、`shell`/`Shell`、`test`/`Test`），指向同一 native 函数。这再次印证 pak-io-native.md 的双名结论是整个引擎 Lua 注册的普遍模式。
- **isolation 不碰 common**：StateGame 的 isolation.lua 只阉割 io/os/debug/package/cmsg_pack，common 全量保留——**线上玩家端下表全部可用**。
- **命名不对称的坑**（以注册表/运行时枚举为准，勿信 types 文件）：
  - 仅 PascalCase 存在：`GetRegionSelect`/`RequestRegionSelect`/`ForceRegionSelect`、`GetSystemDiskFreeSpace`、`SetGameWorldLowFPSEnable`（小写侧是拼写变体 `set_game_world_lowfps_enable`/`set_gameworld_lowfps_enable` 两版并存）
  - 双名非机械转换：`is_wifi` ↔ `GetIsWifi`、`get_anti_addict_token_id` ↔ `GetAntiAddictTokenAndId`、`string_hash` ↔ `GetStringHash`、`is_ipad/is_iphone/is_iphoneX` ↔ `GetIsIpad/GetIsIphone/GetIsIphoneX`
  - 官方拼写错误原样注册：`SetUseSyetemCursor`（Syetem）
- **注册函数结构**（lua_api_dump 实证）：注册表条目 → thunk（`jmp` 真实实现）→ 实现体经跳板 stub 调 lua54 取参——签名推断工具链见 examples/lua_api_dump.rs。
- **枚举方法**（任何端可复现）：

```lua
for k, v in pairs(common) do log.info(k .. ':' .. type(v)) end
-- 或复制到剪贴板：common.copy_to_clipboard(...)
```

## 1. 平台 / 设备信息

| 名称（双名合并） | 说明 |
| --- | --- |
| get_platform / GetPlatform | 平台名 'Windows'/'Android'/'iOS'/'Web'/'Wx' |
| is_iphone / GetIsIphone、is_ipad / GetIsIpad、is_iphoneX / GetIsIphoneX | ★ iOS 设备细分（types 未收录） |
| get_android_version / GetAndroidVersion | Android 系统版本 |
| get_notch_height / GetNotchHeight、get_bangs_height / GetBangsHeight、get_safe_area_insets / GetSafeAreaInsets | 刘海/安全区 |
| get_battery_info / GetBatteryInfo、get_power_info / GetPowerInfo | 电量/电源信息（移动端） |
| is_wifi / GetIsWifi | Wi-Fi 网络判断 |
| get_local_ip / GetLocalIP、get_local_mac_address / GetLocalMacAddress | 本机 IP/MAC |
| **get_documents_path / GetDocumentsPath** | ★★ **应用 Documents 目录**——iOS 沙盒可写根（pak 提取落点修复的关键候选；types 未收录） |
| get_device_detail / GetDeviceDetail、get_detail / GetDetail | 设备/画质档位 |
| get_default_language / GetDefaultLanguage、get_system_language / GetSystemLanguage、get_localization_language / GetLocalizationLanguage、set_localization_language / SetLocalizationLanguage | 语言 |
| get_app_env / GetAppEnv | 应用环境（types 未收录） |

## 2. 应用 / 包 / 版本

| 名称 | 说明 |
| --- | --- |
| get_app_dir / GetAppDir | 应用安装根目录 |
| get_binary / GetBinary、get_binary_version / GetBinaryVersion、get_package / GetPackage | 二进制名/版本/安装包名 |
| set_custom_binary / SetCustomBinary | 覆盖 binary 名（types 未收录） |
| get_full_cmdline / GetFullCmdline | 完整命令行 |
| get_git_info_hash / GetGitInfoHash、get_git_info_string / GetGitInfoString | 构建 git 信息 |
| get_referenced_libs / GetReferencedLibs | 引用库清单（types 未收录） |
| apply_mode / ApplyMode | 模式应用（语义待考） |
| get_map_pak_version / GetMapPakVersion、set_map_pak_version / SetMapPakVersion | 地图 pak 版本读写 |

## 3. 命令行参数 argv

| 名称 | 说明 |
| --- | --- |
| has_arg / HasArg、get_argv / GetArgv、add_argv / AddArgv、remove_argv / RemoveArgv | argv 查/读/增/删 |

## 4. 窗口与显示（PC 为主）

| 名称 | 说明 |
| --- | --- |
| hide_window / HideWindow、show_window / ShowWindow、raise_window / RaiseWindow | 窗口隐藏/显示/前置 |
| set_window_position / SetWindowPosition、set_window_minimum_size / SetWindowMinimumSize、set_window_maximum_size / SetWindowMaximumSize、set_window_resizable / SetWindowResizable、set_window_width_height_ratio / SetWindowLockWidthHeightRatio | 窗口位置/尺寸/比例锁 |
| get_desktop_resolution / GetDesktopResolution、get_desktop_workarea / GetDesktopWorkArea | 桌面分辨率/工作区 |
| set_fullscreen / SetFullscreen、toggle_fullscreen / ToggleFullscreen、get_fullscreen / GetFullscreen | 全屏 |
| set_landscape / SetLandscape、set_orientation / SetOrientation、get_orientation / GetOrientation | 横竖屏 |
| set_resolution / SetResolution、get_resolution / GetResolution、get_resolutions / GetResolutions、set_logic_view / SetLogicView | 分辨率/逻辑分辨率 |
| show_nav、set_home_indicator / SetHomeIndicator | 系统导航栏/Home 指示条（移动端） |

## 5. 键值存储 / 游戏选项

| 名称 | 说明 |
| --- | --- |
| get_value / GetValue、set_value / SetValue | 持久化键值 |
| get_option / GetOption、register_option / RegisterOption | 选项读/注册 |
| save_string_option / SaveStringOption、save_float_option / SaveFloatOption、save_boolean_option / SaveBoolOption | 选项保存（落盘） |
| set_string_option / SetStringOption、set_float_option / SetFloatOption、set_boolean_option / SetBoolOption | 选项设置（不落盘） |
| set_default_string_option / SetDefaultStringOption、set_default_float_option / SetDefaultFloatOption、set_default_boolean_option / SetDefaultBoolOption | 选项默认值 |
| set_current_game / SetCurrentGame | 设置当前游戏标识（影响选项作用域） |
| storage_settings / StorageSettings、unstorage_settings / UnstorageSettings | 设置持久化/解除（types 未收录） |

## 6. 时间 / 性能 / 统计上报

| 名称 | 说明 |
| --- | --- |
| utc_time / UtcTime、get_system_time / GetSystemTime | 时间戳 |
| stat_sender / StatSender、send_user_stat / SendUserStat、send_http_user_stat / SendHttpUserStat、send_error_stat / SendErrorStat、send_app_record / SendAppRecord、send_broadcast / SendBroadcast | 统计上报 |
| send_autotest_log / SendAutotestLog | 自动化测试日志 |
| record_stage / RecordStage、record_stage_clear / RecordStageClear、send_record_stage / SendRecordStage | 启动/更新阶段耗时 |
| report_game_size / ReportGameSize | 包体大小上报 |
| get_current_fps / GetCurrentFPS、get_current_ping / GetCurrentPing、get_jank_count / GetJankCount、get_current_draw_call / GetCurrentDrawCall、get_traffic / GetTraffic | 实时性能 |
| get_server_cost / GetServerCost、get_server_cpu_usage / GetServerCpuUsage、get_server_GC_count / GetServerGCCount、get_score_call_count / GetScoreCallCount | 服务端/云变量调用统计 |
| get_unit_count / GetUnitCount、get_client_unit_count / GetClientUnitCount、get_ticked_unit_count / GetTickedUnitCount、get_unit_wait_gc_count / GetUnitWaitGCCount、get_buff_count / GetBuffCount | 单位/Buff 计数 |
| write_profile_detail / WriteProfileDetail、profile_begin_block / ProfileBeginBlock、profile_end_block / ProfileEndBlock | 性能采样 |

## 7. JSON / 哈希

| 名称 | 说明 |
| --- | --- |
| json_decode / JsonDecode、json_encode / JsonEncode | JSON |
| get_md5 / GetMD5、get_md5_from_http_stream / GetMD5FromHttpStream | MD5 |
| get_file_md5 / GetFileMD5、get_file_crc32 / GetFileCrc32、get_file_sha1 / GetFileSHA1 | 文件哈希 |
| string_hash / GetStringHash | 字符串哈希（双名不对称） |

## 8. 系统交互

| 名称 | 说明 |
| --- | --- |
| open_url / OpenUrl | 系统默认程序打开 URL/外部程序 |
| get_clipboard / GetClipboard、copy_to_clipboard / CopyToClipboard | ★ 剪贴板读写（iOS 无日志时导出诊断的通道） |
| create_shortcut / CreateShortcut、create_desktop_short_cut / CreateDesktopShortCut | 桌面快捷方式 |
| shell / Shell | ★ 执行 shell（危险；types 未收录，慎用） |
| generate_qrcode / GenerateQRCode | 生成二维码（types 未收录） |
| pack_latest_log / PackLatestLog | ★ 打包最新日志（移动端取日志的另一条路；types 未收录） |
| Exit、force_exit / ForceExit | 退出（注意：无小写 exit） |
| request_sdk_exit / RequestSDKExit | SDK 退出流程 |
| reset_game_network / ResetGameNetwork、set_direct_connect_host_mode / SetDirectConnectHostMode | 网络重置/直连 host 模式 |
| save_replay_next_game / SaveReplayNextGame | 下局录像 |
| disconnect_test / DisconnectTest、cheat_codes / CheatCodes、test / Test | 测试/作弊码 |

## 9. AppStore / 支付 / 大区

| 名称 | 说明 |
| --- | --- |
| init_appstore_product / InitAppStoreProduct、appstore_buy_diamond / AppStoreBuyDiamond、appstore_buy_diamond_success / AppStoreBuyDiamondSuccess | AppStore 内购 |
| is_wxpay_supported / GetIsWXPaySupported | 微信支付支持 |
| get_anti_addict_token_id / GetAntiAddictTokenAndId | 防沉迷 token |
| GetRegionSelect、RequestRegionSelect、ForceRegionSelect | 大区选择（仅 PascalCase 系） |

## 10. 卸载 / 更新辅助

| 名称 | 说明 |
| --- | --- |
| report_uninstall_progress / ReportUninstallProgress、report_uninstall_result / ReportUninstallResult | 卸载进度/结果上报 |
| set_need_clear_resource_cache / SetNeedClearResourceCache | 资源缓存清理标记 |
| reload_pak / ReloadPak、reload_shadercache / ReloadShaderCache、load_shadercache_and_paks / LoadShaderCacheAndPaks、has_full_shadercache / HasFullShaderCache | pak/着色器缓存重载 |
| reload_font_map / ReloadFontMap | 字体映射重载 |

## 11. 渲染 / 画质设置（大量，setter/getter/toggle 成组）

- 质量：set_render_quality(_no_check)/get_render_quality、set_msaa/SetMSAA、set_ambient_occlusion_type/get_ambient_occlusion_type、set_render_mask
- 阴影：set_draw_shadows/get_draw_shadows、toggle_shadow、set_bakedshadow/is_bakedshadow、set_off_screen_shadow_enabled/is_off_screen_shadow_enabled、set_planer_shadow_enabled/is_planer_shadow_enabled、set_shadowmap_size、baking_shadowmap_once、set_merge_directional_light_and_point_light/is_merge_directional_light_and_point_light
- 光照/粒子：set_point_light_enabled/get_point_light_enabled、set_particle_lod_level/get_particle_lod_level、set_particle_dynamic_batch_enabled/get_particle_dynamic_batch_enabled、toggle_particle、get_effect_emitters_count
- 蒙皮：set_simple_skinning_enabled/get_simple_skinning_enabled、set_compute_skinning_enabled/get_compute_skinning_enabled、get_active_bones/get_active_primitives
- 后处理：set_postprocess_enabled/get_postprocess_enabled、toggle_postprocess、open_and_set_posteffect/remove_posteffect
- cluster：set_use_cluster/is_use_cluster、toggle_instance
- 全局材质排除：set_global_material_pixel_excludes / set_global_material_vertex_excludes
- 背景/皮肤：set_background_texture_path、set_background_texture_uv、set_skin_type、toggle_bg、toggle_terrain
- 着色器：process_shader、clear_shaders、clear_part_shaders、clear_shaders_from_startup
- 其他：toggle_animation、toggle_ui_scene、toggle_vsync/get_vsync/set_vsync、create_texture、show_debug_view、set_lag_thresholds、set_game_world_lowfps_enable（两版拼写并存）
- 画布显隐：set_gameplay_canvas_visible、set_bloodstrip_canvas_visible、set_minimap_canvas_visible、set_riseletter_canvas_visible
- 调试可视化：toggle_show_unit_radius / toggle_show_select / toggle_show_boundingbox / toggle_show_unit_collision_grid、toggle_game_ui

## 12. 内存 / 引用调试（types 基本未收录）

| 名称 | 说明 |
| --- | --- |
| memory_profiler_begin / MemoryProfilerBegin、memory_profiler_end / MemoryProfilerEnd、set_callstack_memory_profiler_enable / SetCallstackMemoryProfilerEnable | 内存 profiler |
| snapshot_memory / SnapshotMemory | 内存快照 |
| dump_allocs_to_file / DumpAllocsToFile、dump_simple_allocs_to_file / DumpSimpleAllocsToFile、dump_gpu_resource_memory / DumpGpuResourceMemory | 分配/GPU 资源 dump |
| get_current_memory / GetCurrentMemory、get_memory_used / GetMemoryUsed、get_malloc_memory_size / GetMallocMemorySize | 内存读数 |
| get_lua_object_ref_info / GetLuaObjectRefInfo、begin_ref_stack_info / BeginRefStackInfo、end_ref_stack_info / EndRefStackInfo、get_ref_stack_info / GetRefStackInfo、get_ref_stack_info_external / GetRefStackInfoExternal | Lua 引用追踪 |
| cpp_break_point / CPPBreakPoint | C++ 断点触发 |
| trigger_rdoc_capture / TriggerRenderdocCapture | RenderDoc 抓帧 |

## 13. 编辑器 / 场景

| 名称 | 说明 |
| --- | --- |
| is_game_play_in_editor / IsGamePlayInEditor | ★ PIE 判断（环境分流用） |
| lock_scene_view / LockSceneView、unlock_scene_view / UnlockSceneView | 场景视图锁 |
| change_editor_api / ChangeEditorApi | 切换编辑器 API 版本 |
| get_choose_api_window_times / GetChooseAPIWindowTimes、set_choose_api_window_times / SetChooseAPIWindowTimes | API 选择窗口计数 |
| enable_game_lua / EnableGameLua | 启用游戏 lua（语义待考） |

## 14. 游戏控制 / 输入

| 名称 | 说明 |
| --- | --- |
| set_game_speed / SetGameSpeed | 逻辑速度倍率 |
| set_max_fps / SetMaxFps、set_min_fps / SetMinFps、set_lock_max_fps / SetLockMaxFps + 对应 get | 帧率 |
| set_sound_volume / SetSoundVolume、set_sound_volume_by_class / SetSoundVolumeByClass、set_scene_mute / SetSceneMute | 音量 |
| get_mouse_screen_pos / GetMouseScreenPos | 鼠标/触摸坐标 |
| set_cursor_shape / SetCursorShape、set_cursor_visible / SetCursorVisible、set_use_system_cursor / SetUseSyetemCursor（拼写错误原样注册） | 光标 |
| game_is_start / GameIsStart | 游戏是否已开始 |

## 15. 使用注意

1. **判 nil 再调**：不同平台/构建注册面有差异，上游惯例 `if common.xxx then ... end`。
2. **types 文件不全**：`libs/types/sce_common.d.lua` 只覆盖约 1/3，写代码时以本文 + 目标端实测为准。
3. **移动端差异**：iOS/Android 构建的注册面未逐端枚举——在目标端跑 §0 的枚举代码（aye.lua 诊断面板的「复制common清单」按钮已带此能力）。
4. 危险项：`shell`、`Exit/force_exit`、`cheat_codes`、`set_direct_connect_host_mode` 等勿进生产路径。
