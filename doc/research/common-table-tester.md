# common 表注册清单 —— 星火对战平台（BuildPCBox / scegame）

> 生成：2026-08-26 | 引擎：tester_1089/Win/scegame 47.9MB | 工具：examples/lua_api_dump.rs（注册表精确 + 签名启发式推断）
> 签名由反汇编推断（edx 下标 + lua54 取参调用），可能有漏参/误判；`?` = 未取到；重要函数以官方 lua 调用点复核。

| 注册名 | 函数 RVA | 推断签名 |
| --- | --- | --- |
| add_argv | 0x19cab50 | (string, string) -> ? |
| AddArgv | 0x19cab50 | (string, string) -> ? |
| apply_mode | 0x19cab80 | (userdata) -> ? |
| ApplyMode | 0x19cab80 | (userdata) -> ? |
| appstore_buy_diamond | 0x1983370 | (userdata) -> ? |
| appstore_buy_diamond_success | 0x1983370 | (userdata) -> ? |
| AppStoreBuyDiamond | 0x1983370 | (userdata) -> ? |
| AppStoreBuyDiamondSuccess | 0x1983370 | (userdata) -> ? |
| baking_shadowmap_once | 0x19cabb0 | (userdata) -> ? |
| BakingShadowMapOnce | 0x19cabb0 | (userdata) -> ? |
| begin_ref_stack_info | 0x19cabe0 | (userdata) -> ? |
| BeginRefStackInfo | 0x19cabe0 | (userdata) -> ? |
| change_editor_api | 0x19cac80 | (integer, integer) -> ? |
| ChangeEditorApi | 0x19cac80 | (integer, integer) -> ? |
| cheat_codes | 0x19cacb0 | (string) -> ? |
| CheatCodes | 0x19cacb0 | (string) -> ? |
| clear_part_shaders | 0x19cace0 | (userdata) -> ? |
| clear_shaders | 0x19cad10 | (userdata) -> ? |
| clear_shaders_from_startup | 0x19cad40 | () -> ? |
| ClearPartShaders | 0x19cace0 | (userdata) -> ? |
| ClearShaders | 0x19cad10 | (userdata) -> ? |
| ClearShadersFromStartup | 0x19cad40 | () -> ? |
| copy_to_clipboard | 0x19cad70 | (string) -> ? |
| CopyToClipboard | 0x19cad70 | (string) -> ? |
| cpp_break_point | 0x19cac50 | () -> ? |
| CPPBreakPoint | 0x19cac50 | () -> ? |
| create_desktop_short_cut | 0x19cada0 | (string, string, string) -> 1 |
| create_shortcut | 0x1983370 | (userdata) -> ? |
| create_texture | 0x19cadd0 | (string, integer, integer) -> 1 |
| CreateDesktopShortCut | 0x19cada0 | (string, string, string) -> 1 |
| CreateShortcut | 0x1983370 | (userdata) -> ? |
| CreateTexture | 0x19cadd0 | (string, integer, integer) -> 1 |
| disconnect_test | 0x19cae00 | (boolean, boolean) -> 1364 |
| DisconnectTest | 0x19cae00 | (boolean, boolean) -> 1364 |
| dump_allocs_to_file | 0x19cae30 | (userdata) -> ? |
| dump_gpu_resource_memory | 0x19cae60 | (userdata) -> ? |
| dump_simple_allocs_to_file | 0x19cae80 | (userdata) -> ? |
| DumpAllocsToFile | 0x19cae30 | (userdata) -> ? |
| DumpGpuResourceMemory | 0x19cae60 | (userdata) -> ? |
| DumpSimpleAllocsToFile | 0x19cae80 | (userdata) -> ? |
| enable_game_lua | 0x19caeb0 | (boolean) -> ? |
| EnableGameLua | 0x19caeb0 | (boolean) -> ? |
| end_ref_stack_info | 0x19caee0 | (userdata) -> ? |
| EndRefStackInfo | 0x19caee0 | (userdata) -> ? |
| Exit | 0x19caf10 | (userdata) -> ? |
| force_exit | 0x19caf40 | (userdata) -> ? |
| ForceExit | 0x19caf40 | (userdata) -> ? |
| ForceRegionSelect | 0x19caf60 | (string) -> ? |
| game_is_start | 0x19caf90 | (userdata) -> ? |
| GameIsStart | 0x19caf90 | (userdata) -> ? |
| generate_qrcode | 0x19cafe0 | (string, integer?) -> 2 |
| GenerateQRCode | 0x19cafe0 | (string, integer?) -> 2 |
| get_active_bones | 0x19cb010 | (userdata) -> 1 |
| get_active_primitives | 0x19cb040 | () -> 1 |
| get_ambient_occlusion_type | 0x19cb070 | (userdata) -> 1 |
| get_android_version | 0x19cb0b0 | (userdata) -> 1 |
| get_anti_addict_token_id | 0x1983370 | (userdata) -> ? |
| get_app_dir | 0x19a3aa0 | (userdata) -> 1 |
| get_app_env | 0x19cb0e0 | () -> 1 |
| get_argv | 0x19cb110 | (string) -> 1 |
| get_bangs_height | 0x19cb140 | () -> 1 |
| get_battery_info | 0x19cb170 | () -> 1 |
| get_binary | 0x19cb1a0 | () -> 1 |
| get_binary_version | 0x19cb1d0 | (userdata) -> 1 |
| get_buff_count | 0x19cb210 | (userdata) -> 1 |
| get_choose_api_window_times | 0x19cb270 | () -> 1 |
| get_client_unit_count | 0x19cb2a0 | () -> 1 |
| get_clipboard | 0x19cb2d0 | () -> 1 |
| get_compute_skinning_enabled | 0x19cb300 | (userdata) -> 1 |
| get_current_draw_call | 0x19cb340 | (userdata) -> 1 |
| get_current_fps | 0x19cb380 | (userdata) -> ? |
| get_current_memory | 0x19cb3c0 | (userdata) -> 1 |
| get_current_ping | 0x19cb420 | (userdata) -> 1 |
| get_debug_game_mobile | 0x19cb480 | () -> 1 |
| get_default_language | 0x19cb4b0 | (userdata) -> 1 |
| get_desktop_resolution | 0x19cb520 | (integer) -> 2 |
| get_desktop_workarea | 0x19cb550 | (userdata) -> 2 |
| get_detail | 0x19cb5a0 | (userdata) -> 1 |
| get_device_detail | 0x19cb5a0 | (userdata) -> 1 |
| get_disk_freespace | 0x19cc2f0 | (userdata) -> 1 |
| get_documents_path | 0x19cb5f0 | (userdata) -> 1 |
| get_draw_shadows | 0x19cb650 | (userdata) -> 1 |
| get_effect_emitters_count | 0x19cb690 | () -> 1 |
| get_file_crc32 | 0x19cb6c0 | (string, boolean) -> 1 |
| get_file_md5 | 0x19cb6f0 | (string, boolean) -> 1 |
| get_file_sha1 | 0x19cb720 | (string) -> 1 |
| get_full_cmdline | 0x19cb750 | (userdata) -> 1 |
| get_fullscreen | 0x19cb7b0 | (userdata) -> 1 |
| get_git_info_hash | 0x19cb7f0 | (userdata) -> 1 |
| get_git_info_string | 0x19cb830 | (userdata) -> 1 |
| get_jank_count | 0x19cb890 | (userdata) -> 1 |
| get_local_ip | 0x19cb8f0 | (userdata) -> 1 |
| get_local_mac_address | 0x19cb940 | (userdata) -> 1 |
| get_localization_language | 0x19cb990 | () -> 1 |
| get_lock_max_fps | 0x19cb9c0 | (userdata) -> 1 |
| get_lua_object_ref_info | 0x19cba00 | () -> ? |
| get_malloc_memory_size | 0x19cbad0 | () -> 1 |
| get_map_pak_version | 0x19cbb00 | (string) -> 1 |
| get_max_fps | 0x19cbb30 | (userdata) -> 1 |
| get_md5 | 0x19cba30 | (string, userdata) -> 1 |
| get_md5_from_http_stream | 0x19cbaa0 | (userdata) -> 1 |
| get_memory_used | 0x19cbb70 | (userdata) -> 1 |
| get_min_fps | 0x19cbbd0 | (userdata) -> 1 |
| get_mouse_screen_pos | 0x19cbc10 | (integer) -> 2 |
| get_notch_height | 0x19cbc40 | (userdata) -> 1 |
| get_option | 0x19cbc70 | (string) -> 1 |
| get_orientation | 0x19cbca0 | (userdata) -> 1 |
| get_package | 0x19cbce0 | (userdata) -> 1 |
| get_particle_dynamic_batch_enabled | 0x19cbd40 | (userdata) -> 1 |
| get_particle_lod_level | 0x19cbd80 | (userdata) -> 1 |
| get_platform | 0x19cbdd0 | (userdata) -> 1 |
| get_point_light_enabled | 0x19cbe20 | (userdata) -> 1 |
| get_postprocess_enabled | 0x19cbe60 | (userdata) -> 1 |
| get_power_info | 0x19cbea0 | (userdata) -> 3 |
| get_ref_stack_info | 0x19cbee0 | (integer) -> 1 |
| get_ref_stack_info_external | 0x19cbf10 | (number, boolean, boolean) -> ? |
| get_referenced_libs | 0x19cbf40 | () -> 1 |
| get_render_quality | 0x19cbfa0 | (userdata) -> 1 |
| get_renderer_name | 0x19cbfe0 | (userdata) -> 1 |
| get_resolution | 0x19cc030 | () -> 2 |
| get_resolutions | 0x19cc060 | () -> 1 |
| get_safe_area_insets | 0x19cc090 | (boolean) -> 4 |
| get_score_call_count | 0x19cc0c0 | (userdata) -> 1 |
| get_server_cost | 0x19cc120 | (userdata) -> 1 |
| get_server_cpu_usage | 0x19cc180 | (userdata) -> 1 |
| get_server_GC_count | 0x19cc1e0 | (userdata) -> 1 |
| get_simple_skinning_enabled | 0x19cc240 | (userdata) -> 1 |
| get_system_language | 0x19cc320 | () -> 1 |
| get_system_time | 0x19cc350 | () -> 1 |
| get_ticked_unit_count | 0x19cc380 | () -> 1 |
| get_traffic | 0x19cc3b0 | (userdata) -> 2 |
| get_unit_count | 0x19cc420 | (userdata) -> 1 |
| get_unit_wait_gc_count | 0x19cc480 | (userdata) -> 1 |
| get_value | 0x19cc520 | (string) -> 1 |
| get_vsync | 0x19cc4e0 | (userdata) -> 1 |
| GetActiveBones | 0x19cb010 | (userdata) -> 1 |
| GetActivePrimitives | 0x19cb040 | () -> 1 |
| GetAmbientOcclusionType | 0x19cb070 | (userdata) -> 1 |
| GetAndroidVersion | 0x19cb0b0 | (userdata) -> 1 |
| GetAntiAddictTokenAndId | 0x1983370 | (userdata) -> ? |
| GetAppDir | 0x19a3aa0 | (userdata) -> 1 |
| GetAppEnv | 0x19cb0e0 | () -> 1 |
| GetArgv | 0x19cb110 | (string) -> 1 |
| GetBangsHeight | 0x19cb140 | () -> 1 |
| GetBatteryInfo | 0x19cb170 | () -> 1 |
| GetBinary | 0x19cb1a0 | () -> 1 |
| GetBinaryVersion | 0x19cb1d0 | (userdata) -> 1 |
| GetBuffCount | 0x19cb210 | (userdata) -> 1 |
| GetChooseAPIWindowTimes | 0x19cb270 | () -> 1 |
| GetClientUnitCount | 0x19cb2a0 | () -> 1 |
| GetClipboard | 0x19cb2d0 | () -> 1 |
| GetComputeSkinningEnabled | 0x19cb300 | (userdata) -> 1 |
| GetCurrentDrawCall | 0x19cb340 | (userdata) -> 1 |
| GetCurrentFPS | 0x19cb380 | (userdata) -> ? |
| GetCurrentMemory | 0x19cb3c0 | (userdata) -> 1 |
| GetCurrentPing | 0x19cb420 | (userdata) -> 1 |
| GetDebugGameMobile | 0x19cb480 | () -> 1 |
| GetDefaultLanguage | 0x19cb4b0 | (userdata) -> 1 |
| GetDesktopResolution | 0x19cb520 | (integer) -> 2 |
| GetDesktopWorkArea | 0x19cb550 | (userdata) -> 2 |
| GetDetail | 0x19cb5a0 | (userdata) -> 1 |
| GetDeviceDetail | 0x19cb5a0 | (userdata) -> 1 |
| GetDocumentsPath | 0x19cb5f0 | (userdata) -> 1 |
| GetDrawShadows | 0x19cb650 | (userdata) -> 1 |
| GetEffectEmittersCount | 0x19cb690 | () -> 1 |
| GetFileCrc32 | 0x19cb6c0 | (string, boolean) -> 1 |
| GetFileMD5 | 0x19cb6f0 | (string, boolean) -> 1 |
| GetFileSHA1 | 0x19cb720 | (string) -> 1 |
| GetFullCmdline | 0x19cb750 | (userdata) -> 1 |
| GetFullscreen | 0x19cb7b0 | (userdata) -> 1 |
| GetGitInfoHash | 0x19cb7f0 | (userdata) -> 1 |
| GetGitInfoString | 0x19cb830 | (userdata) -> 1 |
| GetIsIpad | 0x1984ce0 | (userdata) -> 1 |
| GetIsIphone | 0x1984ce0 | (userdata) -> 1 |
| GetIsIphoneX | 0x1984ce0 | (userdata) -> 1 |
| GetIsWifi | 0x19cb860 | (userdata) -> 1 |
| GetIsWXPaySupported | 0x1984ce0 | (userdata) -> 1 |
| GetJankCount | 0x19cb890 | (userdata) -> 1 |
| GetLocalIP | 0x19cb8f0 | (userdata) -> 1 |
| GetLocalizationLanguage | 0x19cb990 | () -> 1 |
| GetLocalMacAddress | 0x19cb940 | (userdata) -> 1 |
| GetLockMaxFps | 0x19cb9c0 | (userdata) -> 1 |
| GetLuaObjectRefInfo | 0x19cba00 | () -> ? |
| GetMallocMemorySize | 0x19cbad0 | () -> 1 |
| GetMapPakVersion | 0x19cbb00 | (string) -> 1 |
| GetMaxFps | 0x19cbb30 | (userdata) -> 1 |
| GetMD5 | 0x19cba30 | (string, userdata) -> 1 |
| GetMD5FromHttpStream | 0x19cbaa0 | (userdata) -> 1 |
| GetMemoryUsed | 0x19cbb70 | (userdata) -> 1 |
| GetMinFps | 0x19cbbd0 | (userdata) -> 1 |
| GetMouseScreenPos | 0x19cbc10 | (integer) -> 2 |
| GetNotchHeight | 0x19cbc40 | (userdata) -> 1 |
| GetOption | 0x19cbc70 | (string) -> 1 |
| GetOrientation | 0x19cbca0 | (userdata) -> 1 |
| GetPackage | 0x19cbce0 | (userdata) -> 1 |
| GetParticleDynamicBatchEnabled | 0x19cbd40 | (userdata) -> 1 |
| GetParticleLodLevel | 0x19cbd80 | (userdata) -> 1 |
| GetPlatform | 0x19cbdd0 | (userdata) -> 1 |
| GetPointLightEnabled | 0x19cbe20 | (userdata) -> 1 |
| GetPostProcessEnabled | 0x19cbe60 | (userdata) -> 1 |
| GetPowerInfo | 0x19cbea0 | (userdata) -> 3 |
| GetReferencedLibs | 0x19cbf40 | () -> 1 |
| GetRefStackInfo | 0x19cbee0 | (integer) -> 1 |
| GetRefStackInfoExternal | 0x19cbf10 | (number, boolean, boolean) -> ? |
| GetRegionSelect | 0x19cbf70 | (string) -> 1 |
| GetRendererName | 0x19cbfe0 | (userdata) -> 1 |
| GetRenderQuality | 0x19cbfa0 | (userdata) -> 1 |
| GetResolution | 0x19cc030 | () -> 2 |
| GetResolutions | 0x19cc060 | () -> 1 |
| GetSafeAreaInsets | 0x19cc090 | (boolean) -> 4 |
| GetScoreCallCount | 0x19cc0c0 | (userdata) -> 1 |
| GetServerCost | 0x19cc120 | (userdata) -> 1 |
| GetServerCpuUsage | 0x19cc180 | (userdata) -> 1 |
| GetServerGCCount | 0x19cc1e0 | (userdata) -> 1 |
| GetSimpleSkinningEnabled | 0x19cc240 | (userdata) -> 1 |
| GetStringHash | 0x19cc280 | (string, userdata) -> 1 |
| GetSystemDiskFreeSpace | 0x19cc2f0 | (userdata) -> 1 |
| GetSystemLanguage | 0x19cc320 | () -> 1 |
| GetSystemTime | 0x19cc350 | () -> 1 |
| GetTickedUnitCount | 0x19cc380 | () -> 1 |
| GetTraffic | 0x19cc3b0 | (userdata) -> 2 |
| GetUnitCount | 0x19cc420 | (userdata) -> 1 |
| GetUnitWaitGCCount | 0x19cc480 | (userdata) -> 1 |
| GetValue | 0x19cc520 | (string) -> 1 |
| GetVSync | 0x19cc4e0 | (userdata) -> 1 |
| has_arg | 0x19cc550 | (string) -> 1 |
| has_full_shadercache | 0x19cc580 | (userdata) -> 1 |
| HasArg | 0x19cc550 | (string) -> 1 |
| HasFullShaderCache | 0x19cc580 | (userdata) -> 1 |
| hide_window | 0x19cc5c0 | (userdata) -> ? |
| HideWindow | 0x19cc5c0 | (userdata) -> ? |
| init_appstore_product | 0x1983370 | (userdata) -> ? |
| InitAppStoreProduct | 0x1983370 | (userdata) -> ? |
| is_bakedshadow | 0x19cc5f0 | (userdata) -> 1 |
| is_game_play_in_editor | 0x19cc630 | (userdata) -> ? |
| is_ipad | 0x1984ce0 | (userdata) -> 1 |
| is_iphone | 0x1984ce0 | (userdata) -> 1 |
| is_iphoneX | 0x1984ce0 | (userdata) -> 1 |
| is_merge_directional_light_and_point_light | 0x19cc680 | (userdata) -> 1 |
| is_off_screen_shadow_enabled | 0x19cc6c0 | (userdata) -> 1 |
| is_planer_shadow_enabled | 0x19cc700 | (userdata) -> 1 |
| is_use_cluster | 0x19cc740 | (userdata) -> 1 |
| is_wifi | 0x19cb860 | (userdata) -> 1 |
| is_wxpay_supported | 0x1984ce0 | (userdata) -> 1 |
| IsBakedShadow | 0x19cc5f0 | (userdata) -> 1 |
| IsGamePlayInEditor | 0x19cc630 | (userdata) -> ? |
| IsMergeDirectionalLightAndPointLight | 0x19cc680 | (userdata) -> 1 |
| IsOffScreenShadowEnabled | 0x19cc6c0 | (userdata) -> 1 |
| IsPlanerShadowEnabled | 0x19cc700 | (userdata) -> 1 |
| IsUseCluster | 0x19cc740 | (userdata) -> 1 |
| json_decode | 0x19cc780 | (string) -> ? |
| json_encode | 0x19cc7b0 | (boolean) -> 1 |
| JsonDecode | 0x19cc780 | (string) -> ? |
| JsonEncode | 0x19cc7b0 | (boolean) -> 1 |
| load_shadercache_and_paks | 0x19cc7e0 | () -> 1364 |
| LoadShaderCacheAndPaks | 0x19cc7e0 | () -> 1364 |
| lock_scene_view | 0x19cc810 | (userdata) -> ? |
| LockSceneView | 0x19cc810 | (userdata) -> ? |
| memory_profiler_begin | 0x19cc840 | (string) -> ? |
| memory_profiler_end | 0x19cc870 | (userdata) -> ? |
| MemoryProfilerBegin | 0x19cc840 | (string) -> ? |
| MemoryProfilerEnd | 0x19cc870 | (userdata) -> ? |
| open_and_set_posteffect | 0x19cc890 | (string, integer, number, number, number) -> ? |
| open_url | 0x19cc8c0 | (string, string) -> ? |
| OpenAndSetPostEffect | 0x19cc890 | (string, integer, number, number, number) -> ? |
| OpenUrl | 0x19cc8c0 | (string, string) -> ? |
| pack_latest_log | 0x19cc8f0 | (string, string) -> ? |
| PackLatestLog | 0x19cc8f0 | (string, string) -> ? |
| process_shader | 0x1983370 | (userdata) -> ? |
| ProcessShader | 0x1983370 | (userdata) -> ? |
| profile_begin_block | 0x19cc920 | (string, userdata) -> ? |
| profile_end_block | 0x19cc970 | (userdata) -> ? |
| ProfileBeginBlock | 0x19cc920 | (string, userdata) -> ? |
| ProfileEndBlock | 0x19cc970 | (userdata) -> ? |
| raise_window | 0x19cc990 | (userdata) -> ? |
| RaiseWindow | 0x19cc990 | (userdata) -> ? |
| record_stage | 0x19cc9c0 | (string, string) -> ? |
| record_stage_clear | 0x19cc9f0 | (string) -> ? |
| RecordStage | 0x19cc9c0 | (string, string) -> ? |
| RecordStageClear | 0x19cc9f0 | (string) -> ? |
| register_option | 0x19cca20 | (string) -> ? |
| RegisterOption | 0x19cca20 | (string) -> ? |
| reload_font_map | 0x19cca50 | () -> ? |
| reload_pak | 0x19cca80 | (string) -> ? |
| reload_shadercache | 0x19ccab0 | () -> ? |
| ReloadFontMap | 0x19cca50 | () -> ? |
| ReloadPak | 0x19cca80 | (string) -> ? |
| ReloadShaderCache | 0x19ccab0 | () -> ? |
| remove_argv | 0x19ccae0 | (string) -> ? |
| remove_posteffect | 0x19ccb10 | (string) -> ? |
| RemoveArgv | 0x19ccae0 | (string) -> ? |
| RemovePostEffect | 0x19ccb10 | (string) -> ? |
| report_game_size | 0x1983370 | (userdata) -> ? |
| report_uninstall_progress | 0x1983370 | (userdata) -> ? |
| report_uninstall_result | 0x1983370 | (userdata) -> ? |
| ReportGameSize | 0x1983370 | (userdata) -> ? |
| ReportUninstallProgress | 0x1983370 | (userdata) -> ? |
| ReportUninstallResult | 0x1983370 | (userdata) -> ? |
| request_sdk_exit | 0x19ccb70 | (userdata) -> ? |
| RequestRegionSelect | 0x19ccb40 | (string, string) -> ? |
| RequestSDKExit | 0x19ccb70 | (userdata) -> ? |
| reset_game_network | 0x1983370 | (userdata) -> ? |
| ResetGameNetwork | 0x1983370 | (userdata) -> ? |
| save_boolean_option | 0x19ccbb0 | (string, boolean, boolean) -> ? |
| save_float_option | 0x19ccbe0 | (string, number, boolean) -> ? |
| save_replay_next_game | 0x19ccc10 | (userdata) -> ? |
| save_string_option | 0x19ccc50 | (string, string, boolean) -> ? |
| SaveBoolOption | 0x19ccbb0 | (string, boolean, boolean) -> ? |
| SaveFloatOption | 0x19ccbe0 | (string, number, boolean) -> ? |
| SaveReplayNextGame | 0x19ccc10 | (userdata) -> ? |
| SaveStringOption | 0x19ccc50 | (string, string, boolean) -> ? |
| send_app_record | 0x19ccc80 | (string, string) -> ? |
| send_autotest_log | 0x19cccb0 | (string, string, string, string) -> ? |
| send_broadcast | 0x19ccce0 | (string) -> ? |
| send_error_stat | 0x19ccd10 | (string, integer, string) -> ? |
| send_http_user_stat | 0x19ccd40 | (string, string, string) -> ? |
| send_profile_detail | 0x19ccd70 | (string) -> ? |
| send_record_stage | 0x19ccda0 | (string) -> ? |
| send_user_stat | 0x19ccdd0 | (string, string) -> ? |
| SendAppRecord | 0x19ccc80 | (string, string) -> ? |
| SendAutotestLog | 0x19cccb0 | (string, string, string, string) -> ? |
| SendBroadcast | 0x19ccce0 | (string) -> ? |
| SendErrorStat | 0x19ccd10 | (string, integer, string) -> ? |
| SendHttpUserStat | 0x19ccd40 | (string, string, string) -> ? |
| SendProfileDetail | 0x19ccd70 | (string) -> ? |
| SendRecordStage | 0x19ccda0 | (string) -> ? |
| SendUserStat | 0x19ccdd0 | (string, string) -> ? |
| set_ambient_occlusion_type | 0x19cce00 | (integer, userdata) -> ? |
| set_background_texture_path | 0x1983370 | (userdata) -> ? |
| set_background_texture_uv | 0x1983370 | (userdata) -> ? |
| set_bakedshadow | 0x19cce50 | (boolean, userdata) -> ? |
| set_bangs_height | 0x19ccea0 | (number) -> ? |
| set_bloodstrip_canvas_visible | 0x19cced0 | (boolean) -> ? |
| set_boolean_option | 0x19ccf00 | (string, boolean) -> ? |
| set_callstack_memory_profiler_enable | 0x19ccf30 | (boolean) -> ? |
| set_choose_api_window_times | 0x19ccf60 | (integer) -> ? |
| set_compute_skinning_enabled | 0x19ccf90 | (boolean, userdata) -> ? |
| set_current_game | 0x19ccfe0 | (string) -> ? |
| set_cursor_shape | 0x19cd010 | (string, string) -> ? |
| set_cursor_visible | 0x19cd040 | (boolean, userdata) -> ? |
| set_custom_binary | 0x19cd090 | (string) -> ? |
| set_default_boolean_option | 0x19cd0c0 | (string, boolean) -> ? |
| set_default_float_option | 0x19cd0f0 | (string, number) -> ? |
| set_default_string_option | 0x19cd120 | (string, string) -> ? |
| set_direct_connect_host_mode | 0x19cd150 | (boolean, userdata) -> ? |
| set_draw_shadows | 0x19cd190 | (boolean, userdata) -> ? |
| set_float_option | 0x19cd1e0 | (string, number) -> ? |
| set_fullscreen | 0x19cd210 | (boolean, userdata) -> ? |
| set_game_speed | 0x19cd2a0 | (number) -> ? |
| set_game_world_lowfps_enable | 0x19cd2d0 | (boolean) -> ? |
| set_gameplay_canvas_visible | 0x19cd270 | (boolean) -> ? |
| set_gameworld_lowfps_enable | 0x19cd2d0 | (boolean) -> ? |
| set_global_material_pixel_excludes | 0x19cd360 | (string) -> ? |
| set_global_material_vertex_excludes | 0x19cd390 | (string) -> ? |
| set_home_indicator | 0x1983370 | (userdata) -> ? |
| set_lag_thresholds | 0x19cd3c0 | (integer, integer, userdata) -> ? |
| set_landscape | 0x1984ce0 | (userdata) -> 1 |
| set_localization_language | 0x19cd430 | (string) -> ? |
| set_lock_max_fps | 0x19cd460 | (integer, userdata) -> ? |
| set_logic_view | 0x19cd4b0 | (integer, integer, userdata) -> ? |
| set_map_pak_version | 0x19cd550 | (string, string, integer) -> ? |
| set_max_fps | 0x19cd580 | (integer, userdata) -> ? |
| set_merge_directional_light_and_point_light | 0x19cd5d0 | (boolean, userdata) -> ? |
| set_min_fps | 0x19cd620 | (integer, userdata) -> ? |
| set_minimap_canvas_visible | 0x19cd670 | (boolean) -> ? |
| set_msaa | 0x19cd4f0 | (number, userdata) -> ? |
| set_need_clear_resource_cache | 0x19cd6a0 | (boolean) -> ? |
| set_off_screen_shadow_enabled | 0x19cd6d0 | (boolean, userdata) -> ? |
| set_orientation | 0x19cd720 | (integer, userdata) -> ? |
| set_particle_dynamic_batch_enabled | 0x19cd770 | (boolean, userdata) -> ? |
| set_particle_lod_level | 0x19cd7c0 | (integer) -> ? |
| set_planer_shadow_enabled | 0x19cd7f0 | (boolean, userdata) -> ? |
| set_point_light_enabled | 0x19cd840 | (boolean, userdata) -> ? |
| set_postprocess_enabled | 0x19cd890 | (boolean) -> ? |
| set_render_mask | 0x19cd8c0 | (boolean, userdata) -> ? |
| set_render_quality | 0x19cd910 | (integer, string) -> ? |
| set_render_quality_no_check | 0x19cd940 | (integer, userdata) -> ? |
| set_resolution | 0x19cd990 | (integer, integer, boolean) -> ? |
| set_riseletter_canvas_visible | 0x19cd9c0 | (boolean) -> ? |
| set_scene_mute | 0x19cd9f0 | (boolean) -> ? |
| set_shadowmap_size | 0x19cda20 | (integer, userdata) -> ? |
| set_simple_skinning_enabled | 0x19cda70 | (boolean, userdata) -> ? |
| set_skin_type | 0x19cdac0 | (integer) -> ? |
| set_sound_volume | 0x19cdaf0 | (integer) -> ? |
| set_sound_volume_by_class | 0x19cdb20 | (string, integer) -> ? |
| set_string_option | 0x19cdb50 | (string, string) -> ? |
| set_use_cluster | 0x19cdb80 | (boolean, userdata) -> ? |
| set_use_system_cursor | 0x19cdbd0 | (boolean, userdata) -> ? |
| set_value | 0x19cdc70 | (string, string) -> ? |
| set_vsync | 0x19cdc20 | (boolean, userdata) -> ? |
| set_window_maximum_size | 0x19cdd10 | (integer, integer, userdata) -> ? |
| set_window_minimum_size | 0x19cdd80 | (integer, integer, userdata) -> ? |
| set_window_position | 0x19cddf0 | (integer, integer, userdata) -> ? |
| set_window_resizable | 0x19cde60 | (boolean, userdata) -> ? |
| set_window_width_height_ratio | 0x19cdca0 | (integer, integer, userdata) -> ? |
| SetAmbientOcclusionType | 0x19cce00 | (integer, userdata) -> ? |
| SetBackgroundTexturePath | 0x1983370 | (userdata) -> ? |
| SetBackgroundTextureUV | 0x1983370 | (userdata) -> ? |
| SetBakedShadow | 0x19cce50 | (boolean, userdata) -> ? |
| SetBangsHeight | 0x19ccea0 | (number) -> ? |
| SetBloodStripCanvasVisible | 0x19cced0 | (boolean) -> ? |
| SetBoolOption | 0x19ccf00 | (string, boolean) -> ? |
| SetCallstackMemoryProfilerEnable | 0x19ccf30 | (boolean) -> ? |
| SetChooseAPIWindowTimes | 0x19ccf60 | (integer) -> ? |
| SetComputeSkinningEnabled | 0x19ccf90 | (boolean, userdata) -> ? |
| SetCurrentGame | 0x19ccfe0 | (string) -> ? |
| SetCursorShape | 0x19cd010 | (string, string) -> ? |
| SetCursorVisible | 0x19cd040 | (boolean, userdata) -> ? |
| SetCustomBinary | 0x19cd090 | (string) -> ? |
| SetDefaultBoolOption | 0x19cd0c0 | (string, boolean) -> ? |
| SetDefaultFloatOption | 0x19cd0f0 | (string, number) -> ? |
| SetDefaultStringOption | 0x19cd120 | (string, string) -> ? |
| SetDirectConnectHostMode | 0x19cd150 | (boolean, userdata) -> ? |
| SetDrawShadows | 0x19cd190 | (boolean, userdata) -> ? |
| SetFloatOption | 0x19cd1e0 | (string, number) -> ? |
| SetFullscreen | 0x19cd210 | (boolean, userdata) -> ? |
| SetGamePlayCanvasVisible | 0x19cd270 | (boolean) -> ? |
| SetGameSpeed | 0x19cd2a0 | (number) -> ? |
| SetGameWorldLowFPSEnable | 0x19cd2d0 | (boolean) -> ? |
| SetGameWorldShowDebugView | 0x19cd300 | (number) -> ? |
| SetGameWorldShowNavMesh | 0x19cd330 | (number) -> ? |
| SetGlobalMaterialPixelExcludes | 0x19cd360 | (string) -> ? |
| SetGlobalMaterialVertexExcludes | 0x19cd390 | (string) -> ? |
| SetHomeIndicator | 0x1983370 | (userdata) -> ? |
| SetLagThresholds | 0x19cd3c0 | (integer, integer, userdata) -> ? |
| SetLandscape | 0x1984ce0 | (userdata) -> 1 |
| SetLocalizationLanguage | 0x19cd430 | (string) -> ? |
| SetLockMaxFps | 0x19cd460 | (integer, userdata) -> ? |
| SetLogicView | 0x19cd4b0 | (integer, integer, userdata) -> ? |
| SetMapPakVersion | 0x19cd550 | (string, string, integer) -> ? |
| SetMaxFps | 0x19cd580 | (integer, userdata) -> ? |
| SetMergeDirectionalLightAndPointLight | 0x19cd5d0 | (boolean, userdata) -> ? |
| SetMinFps | 0x19cd620 | (integer, userdata) -> ? |
| SetMiniMapCanvasVisible | 0x19cd670 | (boolean) -> ? |
| SetMSAA | 0x19cd4f0 | (number, userdata) -> ? |
| SetNeedClearResourceCache | 0x19cd6a0 | (boolean) -> ? |
| SetOffScreenShadowEnabled | 0x19cd6d0 | (boolean, userdata) -> ? |
| SetOrientation | 0x19cd720 | (integer, userdata) -> ? |
| SetParticleDynamicBatchEnabled | 0x19cd770 | (boolean, userdata) -> ? |
| SetParticleLodLevel | 0x19cd7c0 | (integer) -> ? |
| SetPlanerShadowEnabled | 0x19cd7f0 | (boolean, userdata) -> ? |
| SetPointLightEnabled | 0x19cd840 | (boolean, userdata) -> ? |
| SetPostProcessEnabled | 0x19cd890 | (boolean) -> ? |
| SetRenderMask | 0x19cd8c0 | (boolean, userdata) -> ? |
| SetRenderQuality | 0x19cd910 | (integer, string) -> ? |
| SetRenderQualityNoCheck | 0x19cd940 | (integer, userdata) -> ? |
| SetResolution | 0x19cd990 | (integer, integer, boolean) -> ? |
| SetRiseLetterCanvasVisible | 0x19cd9c0 | (boolean) -> ? |
| SetSceneMute | 0x19cd9f0 | (boolean) -> ? |
| SetShadowMapSize | 0x19cda20 | (integer, userdata) -> ? |
| SetSimpleSkinningEnabled | 0x19cda70 | (boolean, userdata) -> ? |
| SetSkinType | 0x19cdac0 | (integer) -> ? |
| SetSoundVolume | 0x19cdaf0 | (integer) -> ? |
| SetSoundVolumeByClass | 0x19cdb20 | (string, integer) -> ? |
| SetStringOption | 0x19cdb50 | (string, string) -> ? |
| SetUseCluster | 0x19cdb80 | (boolean, userdata) -> ? |
| SetUseSyetemCursor | 0x19cdbd0 | (boolean, userdata) -> ? |
| SetValue | 0x19cdc70 | (string, string) -> ? |
| SetVSync | 0x19cdc20 | (boolean, userdata) -> ? |
| SetWindowLockWidthHeightRatio | 0x19cdca0 | (integer, integer, userdata) -> ? |
| SetWindowMaximumSize | 0x19cdd10 | (integer, integer, userdata) -> ? |
| SetWindowMinimumSize | 0x19cdd80 | (integer, integer, userdata) -> ? |
| SetWindowPosition | 0x19cddf0 | (integer, integer, userdata) -> ? |
| SetWindowResizable | 0x19cde60 | (boolean, userdata) -> ? |
| Shell | 0x19cdec0 | (string) -> ? |
| show_debug_view | 0x19cd300 | (number) -> ? |
| show_nav | 0x19cd330 | (number) -> ? |
| show_window | 0x19cdef0 | (userdata) -> ? |
| ShowWindow | 0x19cdef0 | (userdata) -> ? |
| snapshot_memory | 0x19cdf20 | (string) -> ? |
| SnapshotMemory | 0x19cdf20 | (string) -> ? |
| stat_sender | 0x19cdf50 | (string, string, string) -> ? |
| StatSender | 0x19cdf50 | (string, string, string) -> ? |
| storage_settings | 0x19cdf80 | () -> ? |
| StorageSettings | 0x19cdf80 | () -> ? |
| string_hash | 0x19cc280 | (string, userdata) -> 1 |
| Test | 0x1983370 | (userdata) -> ? |
| toggle_animation | 0x19cdfb0 | (userdata) -> ? |
| toggle_bg | 0x19cdfe0 | () -> ? |
| toggle_fullscreen | 0x19ce010 | (userdata) -> ? |
| toggle_game_ui | 0x19ce070 | () -> ? |
| toggle_instance | 0x19ce0a0 | (userdata) -> ? |
| toggle_particle | 0x19ce0d0 | () -> ? |
| toggle_postprocess | 0x19ce100 | (userdata) -> ? |
| toggle_shadow | 0x19ce130 | (userdata) -> ? |
| toggle_show_boundingbox | 0x19ce170 | (userdata) -> ? |
| toggle_show_select | 0x19ce1c0 | (userdata) -> ? |
| toggle_show_unit_collision_grid | 0x19ce210 | (userdata) -> ? |
| toggle_show_unit_radius | 0x19ce260 | (userdata) -> ? |
| toggle_terrain | 0x19ce2b0 | () -> ? |
| toggle_ui_scene | 0x19ce2e0 | (boolean, userdata) -> ? |
| toggle_vsync | 0x19ce330 | (userdata) -> ? |
| ToggleAnimation | 0x19cdfb0 | (userdata) -> ? |
| ToggleBG | 0x19cdfe0 | () -> ? |
| ToggleFullscreen | 0x19ce010 | (userdata) -> ? |
| ToggleGameUI | 0x19ce070 | () -> ? |
| ToggleInstance | 0x19ce0a0 | (userdata) -> ? |
| ToggleParticle | 0x19ce0d0 | () -> ? |
| TogglePostProcess | 0x19ce100 | (userdata) -> ? |
| ToggleShadow | 0x19ce130 | (userdata) -> ? |
| ToggleShowBoundingBox | 0x19ce170 | (userdata) -> ? |
| ToggleShowSelect | 0x19ce1c0 | (userdata) -> ? |
| ToggleShowUnitCollisionGrid | 0x19ce210 | (userdata) -> ? |
| ToggleShowUnitRadius | 0x19ce260 | (userdata) -> ? |
| ToggleTerrain | 0x19ce2b0 | () -> ? |
| ToggleUIScene | 0x19ce2e0 | (boolean, userdata) -> ? |
| ToggleVSync | 0x19ce330 | (userdata) -> ? |
| trigger_rdoc_capture | 0x19ce380 | (userdata) -> ? |
| TriggerRenderdocCapture | 0x19ce380 | (userdata) -> ? |
| unlock_scene_view | 0x19ce3b0 | (userdata) -> ? |
| UnlockSceneView | 0x19ce3b0 | (userdata) -> ? |
| unstorage_settings | 0x19ce3e0 | () -> ? |
| UnstorageSettings | 0x19ce3e0 | () -> ? |
| utc_time | 0x19ce410 | (userdata) -> 1 |
| UtcTime | 0x19ce410 | (userdata) -> 1 |
| write_profile_detail | 0x19ce470 | (string, boolean) -> ? |
| WriteProfileDetail | 0x19ce470 | (string, boolean) -> ? |
