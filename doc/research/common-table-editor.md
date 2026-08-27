# common 表注册清单 —— 星火编辑器（BuildPC / sceengine.dll）

> 生成：2026-08-26 | 引擎：version-13/sceengine.dll 49MB | 工具：examples/lua_api_dump.rs（注册表精确 + 签名启发式推断）
> 签名由反汇编推断（edx 下标 + lua54 取参调用），可能有漏参/误判；`?` = 未取到；重要函数以官方 lua 调用点复核。

| 注册名 | 函数 RVA | 推断签名 |
| --- | --- | --- |
| add_argv | 0x12eb780 | (string, string) -> ? |
| AddArgv | 0x12eb780 | (string, string) -> ? |
| apply_mode | 0x12eb7b0 | (userdata) -> ? |
| ApplyMode | 0x12eb7b0 | (userdata) -> ? |
| appstore_buy_diamond | 0x1281920 | (userdata) -> ? |
| appstore_buy_diamond_success | 0x1281920 | (userdata) -> ? |
| AppStoreBuyDiamond | 0x1281920 | (userdata) -> ? |
| AppStoreBuyDiamondSuccess | 0x1281920 | (userdata) -> ? |
| baking_shadowmap_once | 0x12eb7e0 | (userdata) -> ? |
| BakingShadowMapOnce | 0x12eb7e0 | (userdata) -> ? |
| begin_ref_stack_info | 0x12eb810 | (userdata) -> ? |
| BeginRefStackInfo | 0x12eb810 | (userdata) -> ? |
| change_editor_api | 0x12eb8b0 | (integer, integer) -> ? |
| ChangeEditorApi | 0x12eb8b0 | (integer, integer) -> ? |
| cheat_codes | 0x12eb8e0 | (string) -> ? |
| CheatCodes | 0x12eb8e0 | (string) -> ? |
| clear_part_shaders | 0x12eb910 | (userdata) -> ? |
| clear_shaders | 0x12eb940 | (userdata) -> ? |
| clear_shaders_from_startup | 0x12eb970 | () -> ? |
| ClearPartShaders | 0x12eb910 | (userdata) -> ? |
| ClearShaders | 0x12eb940 | (userdata) -> ? |
| ClearShadersFromStartup | 0x12eb970 | () -> ? |
| copy_to_clipboard | 0x12eb9a0 | (string) -> ? |
| CopyToClipboard | 0x12eb9a0 | (string) -> ? |
| cpp_break_point | 0x12eb880 | () -> ? |
| CPPBreakPoint | 0x12eb880 | () -> ? |
| create_desktop_short_cut | 0x12eb9d0 | (string, string, string) -> 1 |
| create_shortcut | 0x1281920 | (userdata) -> ? |
| create_texture | 0x12eba00 | (string, integer, integer) -> 1 |
| CreateDesktopShortCut | 0x12eb9d0 | (string, string, string) -> 1 |
| CreateShortcut | 0x1281920 | (userdata) -> ? |
| CreateTexture | 0x12eba00 | (string, integer, integer) -> 1 |
| disconnect_test | 0x12eba30 | (boolean, boolean) -> 192 |
| DisconnectTest | 0x12eba30 | (boolean, boolean) -> 192 |
| dump_allocs_to_file | 0x12eba60 | (userdata) -> ? |
| dump_gpu_resource_memory | 0x12eba90 | (userdata) -> ? |
| dump_simple_allocs_to_file | 0x12ebab0 | (userdata) -> ? |
| DumpAllocsToFile | 0x12eba60 | (userdata) -> ? |
| DumpGpuResourceMemory | 0x12eba90 | (userdata) -> ? |
| DumpSimpleAllocsToFile | 0x12ebab0 | (userdata) -> ? |
| enable_game_lua | 0x12ebae0 | (boolean) -> ? |
| EnableGameLua | 0x12ebae0 | (boolean) -> ? |
| end_ref_stack_info | 0x12ebb10 | (userdata) -> ? |
| EndRefStackInfo | 0x12ebb10 | (userdata) -> ? |
| Exit | 0x12ebb40 | (userdata) -> ? |
| force_exit | 0x12ebb70 | (userdata) -> ? |
| ForceExit | 0x12ebb70 | (userdata) -> ? |
| ForceRegionSelect | 0x12ebb90 | (string) -> ? |
| game_is_start | 0x12ebbc0 | (userdata) -> ? |
| GameIsStart | 0x12ebbc0 | (userdata) -> ? |
| generate_qrcode | 0x12ebc10 | (string, integer?) -> 2 |
| GenerateQRCode | 0x12ebc10 | (string, integer?) -> 2 |
| get_active_bones | 0x12ebc40 | () -> 1 |
| get_active_primitives | 0x12ebc70 | () -> 1 |
| get_ambient_occlusion_type | 0x12ebca0 | (userdata) -> 1 |
| get_android_version | 0x12ebce0 | (userdata) -> 1 |
| get_anti_addict_token_id | 0x1281920 | (userdata) -> ? |
| get_app_dir | 0x12c55a0 | (userdata) -> 1 |
| get_app_env | 0x12ebd10 | () -> 1 |
| get_argv | 0x12ebd40 | (string) -> 1 |
| get_bangs_height | 0x12ebd70 | () -> 1 |
| get_battery_info | 0x12ebda0 | () -> 1 |
| get_binary | 0x12ebdd0 | (userdata) -> 1 |
| get_binary_version | 0x12ebe40 | (userdata) -> 1 |
| get_buff_count | 0x12ebe80 | (userdata) -> 1 |
| get_choose_api_window_times | 0x12ebee0 | () -> 1 |
| get_client_unit_count | 0x12ebf10 | () -> 1 |
| get_clipboard | 0x12ebf40 | () -> 1 |
| get_compute_skinning_enabled | 0x12ebf70 | (userdata) -> 1 |
| get_current_draw_call | 0x12ebfb0 | (userdata) -> 1 |
| get_current_fps | 0x12ebff0 | (userdata) -> ? |
| get_current_memory | 0x12ec030 | (userdata) -> 1 |
| get_current_ping | 0x12ec090 | (userdata) -> 1 |
| get_debug_game_mobile | 0x12ec0f0 | () -> 1 |
| get_default_language | 0x12ec120 | (userdata) -> 1 |
| get_desktop_resolution | 0x12ec190 | (integer) -> 2 |
| get_desktop_workarea | 0x12ec1c0 | (userdata) -> 2 |
| get_detail | 0x12ec210 | (userdata) -> 1 |
| get_device_detail | 0x12ec210 | (userdata) -> 1 |
| get_disk_freespace | 0x12ecf60 | (userdata) -> 1 |
| get_documents_path | 0x12ec260 | (userdata) -> 1 |
| get_draw_shadows | 0x12ec2c0 | (userdata) -> 1 |
| get_effect_emitters_count | 0x12ec300 | () -> 1 |
| get_file_crc32 | 0x12ec330 | (string, boolean) -> 1 |
| get_file_md5 | 0x12ec360 | (string, boolean) -> 1 |
| get_file_sha1 | 0x12ec390 | (string) -> 1 |
| get_full_cmdline | 0x12ec3c0 | (userdata) -> 1 |
| get_fullscreen | 0x12ec420 | (userdata) -> 1 |
| get_git_info_hash | 0x12ec460 | (userdata) -> 1 |
| get_git_info_string | 0x12ec4a0 | (userdata) -> 1 |
| get_jank_count | 0x12ec500 | (userdata) -> 1 |
| get_local_ip | 0x12ec560 | (userdata) -> 1 |
| get_local_mac_address | 0x12ec5b0 | (userdata) -> 1 |
| get_localization_language | 0x12ec600 | () -> 1 |
| get_lock_max_fps | 0x12ec630 | (userdata) -> 1 |
| get_lua_object_ref_info | 0x12ec670 | () -> ? |
| get_malloc_memory_size | 0x12ec740 | () -> 1 |
| get_map_pak_version | 0x12ec770 | (string) -> 1 |
| get_max_fps | 0x12ec7a0 | (userdata) -> 1 |
| get_md5 | 0x12ec6a0 | (string, userdata) -> 1 |
| get_md5_from_http_stream | 0x12ec710 | (userdata) -> 1 |
| get_memory_used | 0x12ec7e0 | (userdata) -> 1 |
| get_min_fps | 0x12ec840 | (userdata) -> 1 |
| get_mouse_screen_pos | 0x12ec880 | (integer) -> 2 |
| get_notch_height | 0x12ec8b0 | (userdata) -> 1 |
| get_option | 0x12ec8e0 | (string) -> 1 |
| get_orientation | 0x12ec910 | (userdata) -> 1 |
| get_package | 0x12ec950 | (userdata) -> 1 |
| get_particle_dynamic_batch_enabled | 0x12ec9b0 | (userdata) -> 1 |
| get_particle_lod_level | 0x12ec9f0 | (userdata) -> 1 |
| get_platform | 0x12eca40 | (userdata) -> 1 |
| get_point_light_enabled | 0x12eca90 | (userdata) -> 1 |
| get_postprocess_enabled | 0x12ecad0 | (userdata) -> 1 |
| get_power_info | 0x12ecb10 | (userdata) -> 3 |
| get_ref_stack_info | 0x12ecb50 | (integer) -> 1 |
| get_ref_stack_info_external | 0x12ecb80 | (number, boolean, boolean) -> ? |
| get_referenced_libs | 0x12ecbb0 | () -> 1 |
| get_render_quality | 0x12ecc10 | (userdata) -> 1 |
| get_renderer_name | 0x12ecc50 | (userdata) -> 1 |
| get_resolution | 0x12ecca0 | () -> 2 |
| get_resolutions | 0x12eccd0 | () -> 1 |
| get_safe_area_insets | 0x12ecd00 | (boolean) -> 4 |
| get_score_call_count | 0x12ecd30 | (userdata) -> 1 |
| get_server_cost | 0x12ecd90 | (userdata) -> 1 |
| get_server_cpu_usage | 0x12ecdf0 | (userdata) -> 1 |
| get_server_GC_count | 0x12ece50 | (userdata) -> 1 |
| get_simple_skinning_enabled | 0x12eceb0 | (userdata) -> 1 |
| get_system_language | 0x12ecf90 | () -> 1 |
| get_system_time | 0x12ecfc0 | () -> 1 |
| get_ticked_unit_count | 0x12ecff0 | () -> 1 |
| get_traffic | 0x12ed020 | (userdata) -> 2 |
| get_unit_count | 0x12ed090 | (userdata) -> 1 |
| get_unit_wait_gc_count | 0x12ed0f0 | (userdata) -> 1 |
| get_value | 0x12ed190 | (string) -> 1 |
| get_vsync | 0x12ed150 | (userdata) -> 1 |
| GetActiveBones | 0x12ebc40 | () -> 1 |
| GetActivePrimitives | 0x12ebc70 | () -> 1 |
| GetAmbientOcclusionType | 0x12ebca0 | (userdata) -> 1 |
| GetAndroidVersion | 0x12ebce0 | (userdata) -> 1 |
| GetAntiAddictTokenAndId | 0x1281920 | (userdata) -> ? |
| GetAppDir | 0x12c55a0 | (userdata) -> 1 |
| GetAppEnv | 0x12ebd10 | () -> 1 |
| GetArgv | 0x12ebd40 | (string) -> 1 |
| GetBangsHeight | 0x12ebd70 | () -> 1 |
| GetBatteryInfo | 0x12ebda0 | () -> 1 |
| GetBinary | 0x12ebdd0 | (userdata) -> 1 |
| GetBinaryVersion | 0x12ebe40 | (userdata) -> 1 |
| GetBuffCount | 0x12ebe80 | (userdata) -> 1 |
| GetChooseAPIWindowTimes | 0x12ebee0 | () -> 1 |
| GetClientUnitCount | 0x12ebf10 | () -> 1 |
| GetClipboard | 0x12ebf40 | () -> 1 |
| GetComputeSkinningEnabled | 0x12ebf70 | (userdata) -> 1 |
| GetCurrentDrawCall | 0x12ebfb0 | (userdata) -> 1 |
| GetCurrentFPS | 0x12ebff0 | (userdata) -> ? |
| GetCurrentMemory | 0x12ec030 | (userdata) -> 1 |
| GetCurrentPing | 0x12ec090 | (userdata) -> 1 |
| GetDebugGameMobile | 0x12ec0f0 | () -> 1 |
| GetDefaultLanguage | 0x12ec120 | (userdata) -> 1 |
| GetDesktopResolution | 0x12ec190 | (integer) -> 2 |
| GetDesktopWorkArea | 0x12ec1c0 | (userdata) -> 2 |
| GetDetail | 0x12ec210 | (userdata) -> 1 |
| GetDeviceDetail | 0x12ec210 | (userdata) -> 1 |
| GetDocumentsPath | 0x12ec260 | (userdata) -> 1 |
| GetDrawShadows | 0x12ec2c0 | (userdata) -> 1 |
| GetEffectEmittersCount | 0x12ec300 | () -> 1 |
| GetFileCrc32 | 0x12ec330 | (string, boolean) -> 1 |
| GetFileMD5 | 0x12ec360 | (string, boolean) -> 1 |
| GetFileSHA1 | 0x12ec390 | (string) -> 1 |
| GetFullCmdline | 0x12ec3c0 | (userdata) -> 1 |
| GetFullscreen | 0x12ec420 | (userdata) -> 1 |
| GetGitInfoHash | 0x12ec460 | (userdata) -> 1 |
| GetGitInfoString | 0x12ec4a0 | (userdata) -> 1 |
| GetIsIpad | 0x12a6750 | (userdata) -> 1 |
| GetIsIphone | 0x12a6750 | (userdata) -> 1 |
| GetIsIphoneX | 0x12a6750 | (userdata) -> 1 |
| GetIsWifi | 0x12ec4d0 | (userdata) -> 1 |
| GetIsWXPaySupported | 0x12a6750 | (userdata) -> 1 |
| GetJankCount | 0x12ec500 | (userdata) -> 1 |
| GetLocalIP | 0x12ec560 | (userdata) -> 1 |
| GetLocalizationLanguage | 0x12ec600 | () -> 1 |
| GetLocalMacAddress | 0x12ec5b0 | (userdata) -> 1 |
| GetLockMaxFps | 0x12ec630 | (userdata) -> 1 |
| GetLuaObjectRefInfo | 0x12ec670 | () -> ? |
| GetMallocMemorySize | 0x12ec740 | () -> 1 |
| GetMapPakVersion | 0x12ec770 | (string) -> 1 |
| GetMaxFps | 0x12ec7a0 | (userdata) -> 1 |
| GetMD5 | 0x12ec6a0 | (string, userdata) -> 1 |
| GetMD5FromHttpStream | 0x12ec710 | (userdata) -> 1 |
| GetMemoryUsed | 0x12ec7e0 | (userdata) -> 1 |
| GetMinFps | 0x12ec840 | (userdata) -> 1 |
| GetMouseScreenPos | 0x12ec880 | (integer) -> 2 |
| GetNotchHeight | 0x12ec8b0 | (userdata) -> 1 |
| GetOption | 0x12ec8e0 | (string) -> 1 |
| GetOrientation | 0x12ec910 | (userdata) -> 1 |
| GetPackage | 0x12ec950 | (userdata) -> 1 |
| GetParticleDynamicBatchEnabled | 0x12ec9b0 | (userdata) -> 1 |
| GetParticleLodLevel | 0x12ec9f0 | (userdata) -> 1 |
| GetPlatform | 0x12eca40 | (userdata) -> 1 |
| GetPointLightEnabled | 0x12eca90 | (userdata) -> 1 |
| GetPostProcessEnabled | 0x12ecad0 | (userdata) -> 1 |
| GetPowerInfo | 0x12ecb10 | (userdata) -> 3 |
| GetReferencedLibs | 0x12ecbb0 | () -> 1 |
| GetRefStackInfo | 0x12ecb50 | (integer) -> 1 |
| GetRefStackInfoExternal | 0x12ecb80 | (number, boolean, boolean) -> ? |
| GetRegionSelect | 0x12ecbe0 | (string) -> 1 |
| GetRendererName | 0x12ecc50 | (userdata) -> 1 |
| GetRenderQuality | 0x12ecc10 | (userdata) -> 1 |
| GetResolution | 0x12ecca0 | () -> 2 |
| GetResolutions | 0x12eccd0 | () -> 1 |
| GetSafeAreaInsets | 0x12ecd00 | (boolean) -> 4 |
| GetScoreCallCount | 0x12ecd30 | (userdata) -> 1 |
| GetServerCost | 0x12ecd90 | (userdata) -> 1 |
| GetServerCpuUsage | 0x12ecdf0 | (userdata) -> 1 |
| GetServerGCCount | 0x12ece50 | (userdata) -> 1 |
| GetSimpleSkinningEnabled | 0x12eceb0 | (userdata) -> 1 |
| GetStringHash | 0x12ecef0 | (string, userdata) -> 1 |
| GetSystemDiskFreeSpace | 0x12ecf60 | (userdata) -> 1 |
| GetSystemLanguage | 0x12ecf90 | () -> 1 |
| GetSystemTime | 0x12ecfc0 | () -> 1 |
| GetTickedUnitCount | 0x12ecff0 | () -> 1 |
| GetTraffic | 0x12ed020 | (userdata) -> 2 |
| GetUnitCount | 0x12ed090 | (userdata) -> 1 |
| GetUnitWaitGCCount | 0x12ed0f0 | (userdata) -> 1 |
| GetValue | 0x12ed190 | (string) -> 1 |
| GetVSync | 0x12ed150 | (userdata) -> 1 |
| has_arg | 0x12ed1c0 | (string) -> 1 |
| has_full_shadercache | 0x12ed1f0 | (userdata) -> 1 |
| HasArg | 0x12ed1c0 | (string) -> 1 |
| HasFullShaderCache | 0x12ed1f0 | (userdata) -> 1 |
| hide_window | 0x12ed230 | (userdata) -> ? |
| HideWindow | 0x12ed230 | (userdata) -> ? |
| init_appstore_product | 0x1281920 | (userdata) -> ? |
| InitAppStoreProduct | 0x1281920 | (userdata) -> ? |
| is_bakedshadow | 0x12ed260 | (userdata) -> 1 |
| is_game_play_in_editor | 0x12ed2a0 | (userdata) -> ? |
| is_ipad | 0x12a6750 | (userdata) -> 1 |
| is_iphone | 0x12a6750 | (userdata) -> 1 |
| is_iphoneX | 0x12a6750 | (userdata) -> 1 |
| is_merge_directional_light_and_point_light | 0x12ed2f0 | (userdata) -> 1 |
| is_off_screen_shadow_enabled | 0x12ed330 | (userdata) -> 1 |
| is_planer_shadow_enabled | 0x12ed370 | (userdata) -> 1 |
| is_use_cluster | 0x12ed3b0 | (userdata) -> 1 |
| is_wifi | 0x12ec4d0 | (userdata) -> 1 |
| is_wxpay_supported | 0x12a6750 | (userdata) -> 1 |
| IsBakedShadow | 0x12ed260 | (userdata) -> 1 |
| IsGamePlayInEditor | 0x12ed2a0 | (userdata) -> ? |
| IsMergeDirectionalLightAndPointLight | 0x12ed2f0 | (userdata) -> 1 |
| IsOffScreenShadowEnabled | 0x12ed330 | (userdata) -> 1 |
| IsPlanerShadowEnabled | 0x12ed370 | (userdata) -> 1 |
| IsUseCluster | 0x12ed3b0 | (userdata) -> 1 |
| json_decode | 0x12ed3f0 | (string) -> ? |
| json_encode | 0x12ed420 | (boolean) -> 1 |
| JsonDecode | 0x12ed3f0 | (string) -> ? |
| JsonEncode | 0x12ed420 | (boolean) -> 1 |
| load_shadercache_and_paks | 0x12ed450 | () -> 192 |
| LoadShaderCacheAndPaks | 0x12ed450 | () -> 192 |
| lock_scene_view | 0x12ed480 | (userdata) -> ? |
| LockSceneView | 0x12ed480 | (userdata) -> ? |
| memory_profiler_begin | 0x12ed4b0 | (string) -> ? |
| memory_profiler_end | 0x12ed4e0 | (userdata) -> ? |
| MemoryProfilerBegin | 0x12ed4b0 | (string) -> ? |
| MemoryProfilerEnd | 0x12ed4e0 | (userdata) -> ? |
| open_and_set_posteffect | 0x12ed500 | (string, integer, number, number, number) -> ? |
| open_url | 0x12ed530 | (string, string) -> ? |
| OpenAndSetPostEffect | 0x12ed500 | (string, integer, number, number, number) -> ? |
| OpenUrl | 0x12ed530 | (string, string) -> ? |
| pack_latest_log | 0x12ed560 | (string, string) -> ? |
| PackLatestLog | 0x12ed560 | (string, string) -> ? |
| process_shader | 0x1281920 | (userdata) -> ? |
| ProcessShader | 0x1281920 | (userdata) -> ? |
| profile_begin_block | 0x12ed590 | (string, userdata) -> ? |
| profile_end_block | 0x12ed5e0 | (userdata) -> ? |
| ProfileBeginBlock | 0x12ed590 | (string, userdata) -> ? |
| ProfileEndBlock | 0x12ed5e0 | (userdata) -> ? |
| raise_window | 0x12ed600 | (userdata) -> ? |
| RaiseWindow | 0x12ed600 | (userdata) -> ? |
| record_stage | 0x12ed630 | (string, string) -> ? |
| record_stage_clear | 0x12ed660 | (string) -> ? |
| RecordStage | 0x12ed630 | (string, string) -> ? |
| RecordStageClear | 0x12ed660 | (string) -> ? |
| register_option | 0x12ed690 | (string) -> ? |
| RegisterOption | 0x12ed690 | (string) -> ? |
| reload_font_map | 0x12ed6c0 | () -> ? |
| reload_pak | 0x12ed6f0 | (string) -> ? |
| reload_shadercache | 0x12ed720 | () -> ? |
| ReloadFontMap | 0x12ed6c0 | () -> ? |
| ReloadPak | 0x12ed6f0 | (string) -> ? |
| ReloadShaderCache | 0x12ed720 | () -> ? |
| remove_argv | 0x12ed750 | (string) -> ? |
| remove_posteffect | 0x12ed780 | (string) -> ? |
| RemoveArgv | 0x12ed750 | (string) -> ? |
| RemovePostEffect | 0x12ed780 | (string) -> ? |
| report_game_size | 0x1281920 | (userdata) -> ? |
| report_uninstall_progress | 0x1281920 | (userdata) -> ? |
| report_uninstall_result | 0x1281920 | (userdata) -> ? |
| ReportGameSize | 0x1281920 | (userdata) -> ? |
| ReportUninstallProgress | 0x1281920 | (userdata) -> ? |
| ReportUninstallResult | 0x1281920 | (userdata) -> ? |
| request_sdk_exit | 0x12ed7e0 | (userdata) -> ? |
| RequestRegionSelect | 0x12ed7b0 | (string, string) -> ? |
| RequestSDKExit | 0x12ed7e0 | (userdata) -> ? |
| reset_game_network | 0x1281920 | (userdata) -> ? |
| ResetGameNetwork | 0x1281920 | (userdata) -> ? |
| save_boolean_option | 0x12ed820 | (string, boolean, boolean) -> ? |
| save_float_option | 0x12ed850 | (string, number, boolean) -> ? |
| save_replay_next_game | 0x12ed880 | (userdata) -> ? |
| save_string_option | 0x12ed8c0 | (string, string, boolean) -> ? |
| SaveBoolOption | 0x12ed820 | (string, boolean, boolean) -> ? |
| SaveFloatOption | 0x12ed850 | (string, number, boolean) -> ? |
| SaveReplayNextGame | 0x12ed880 | (userdata) -> ? |
| SaveStringOption | 0x12ed8c0 | (string, string, boolean) -> ? |
| send_app_record | 0x12ed8f0 | (string, string) -> ? |
| send_autotest_log | 0x12ed920 | (string, string, string, string) -> ? |
| send_broadcast | 0x12ed950 | (string) -> ? |
| send_error_stat | 0x12ed980 | (string, integer, string) -> ? |
| send_http_user_stat | 0x12ed9b0 | (string, string, string) -> ? |
| send_profile_detail | 0x12ed9e0 | (string) -> ? |
| send_record_stage | 0x12eda10 | (string) -> ? |
| send_user_stat | 0x12eda40 | (string, string) -> ? |
| SendAppRecord | 0x12ed8f0 | (string, string) -> ? |
| SendAutotestLog | 0x12ed920 | (string, string, string, string) -> ? |
| SendBroadcast | 0x12ed950 | (string) -> ? |
| SendErrorStat | 0x12ed980 | (string, integer, string) -> ? |
| SendHttpUserStat | 0x12ed9b0 | (string, string, string) -> ? |
| SendProfileDetail | 0x12ed9e0 | (string) -> ? |
| SendRecordStage | 0x12eda10 | (string) -> ? |
| SendUserStat | 0x12eda40 | (string, string) -> ? |
| set_ambient_occlusion_type | 0x12eda70 | (integer, userdata) -> ? |
| set_background_texture_path | 0x1281920 | (userdata) -> ? |
| set_background_texture_uv | 0x1281920 | (userdata) -> ? |
| set_bakedshadow | 0x12edac0 | (boolean, userdata) -> ? |
| set_bangs_height | 0x12edb10 | (number) -> ? |
| set_bloodstrip_canvas_visible | 0x12edb40 | (boolean) -> ? |
| set_boolean_option | 0x12edb70 | (string, boolean) -> ? |
| set_callstack_memory_profiler_enable | 0x12edba0 | (boolean) -> ? |
| set_choose_api_window_times | 0x12edbd0 | (integer) -> ? |
| set_compute_skinning_enabled | 0x12edc00 | (boolean, userdata) -> ? |
| set_current_game | 0x12edc50 | (string) -> ? |
| set_cursor_shape | 0x12edc80 | (string, string) -> ? |
| set_cursor_visible | 0x12edcb0 | (boolean, userdata) -> ? |
| set_custom_binary | 0x12edd00 | (string) -> ? |
| set_default_boolean_option | 0x12edd30 | (string, boolean) -> ? |
| set_default_float_option | 0x12edd60 | (string, number) -> ? |
| set_default_string_option | 0x12edd90 | (string, string) -> ? |
| set_direct_connect_host_mode | 0x12eddc0 | (boolean, userdata) -> ? |
| set_draw_shadows | 0x12ede00 | (boolean, userdata) -> ? |
| set_float_option | 0x12ede50 | (string, number) -> ? |
| set_fullscreen | 0x12ede80 | (boolean, userdata) -> ? |
| set_game_speed | 0x12edf10 | (number) -> ? |
| set_game_world_lowfps_enable | 0x12edf40 | (boolean) -> ? |
| set_gameplay_canvas_visible | 0x12edee0 | (boolean) -> ? |
| set_gameworld_lowfps_enable | 0x12edf40 | (boolean) -> ? |
| set_global_material_pixel_excludes | 0x12edfd0 | (string) -> ? |
| set_global_material_vertex_excludes | 0x12ee000 | (string) -> ? |
| set_home_indicator | 0x1281920 | (userdata) -> ? |
| set_lag_thresholds | 0x12ee030 | (integer, integer, userdata) -> ? |
| set_landscape | 0x12a6750 | (userdata) -> 1 |
| set_localization_language | 0x12ee0a0 | (string) -> ? |
| set_lock_max_fps | 0x12ee0d0 | (integer, userdata) -> ? |
| set_logic_view | 0x12ee120 | (integer, integer, userdata) -> ? |
| set_map_pak_version | 0x12ee1c0 | (string, string, integer) -> ? |
| set_max_fps | 0x12ee1f0 | (integer, userdata) -> ? |
| set_merge_directional_light_and_point_light | 0x12ee240 | (boolean, userdata) -> ? |
| set_min_fps | 0x12ee290 | (integer, userdata) -> ? |
| set_minimap_canvas_visible | 0x12ee2e0 | (boolean) -> ? |
| set_msaa | 0x12ee160 | (number, userdata) -> ? |
| set_need_clear_resource_cache | 0x12ee310 | (boolean) -> ? |
| set_off_screen_shadow_enabled | 0x12ee340 | (boolean, userdata) -> ? |
| set_orientation | 0x12ee390 | (integer, userdata) -> ? |
| set_particle_dynamic_batch_enabled | 0x12ee3e0 | (boolean, userdata) -> ? |
| set_particle_lod_level | 0x12ee430 | (integer) -> ? |
| set_planer_shadow_enabled | 0x12ee460 | (boolean, userdata) -> ? |
| set_point_light_enabled | 0x12ee4b0 | (boolean, userdata) -> ? |
| set_postprocess_enabled | 0x12ee500 | (boolean) -> ? |
| set_render_mask | 0x12ee530 | (boolean, userdata) -> ? |
| set_render_quality | 0x12ee580 | (integer, string) -> ? |
| set_render_quality_no_check | 0x12ee5b0 | (integer, userdata) -> ? |
| set_resolution | 0x12ee600 | (integer, integer, boolean) -> ? |
| set_riseletter_canvas_visible | 0x12ee630 | (boolean) -> ? |
| set_scene_mute | 0x12ee660 | (boolean) -> ? |
| set_shadowmap_size | 0x12ee690 | (integer, userdata) -> ? |
| set_simple_skinning_enabled | 0x12ee6e0 | (boolean, userdata) -> ? |
| set_skin_type | 0x12ee730 | (integer) -> ? |
| set_sound_volume | 0x12ee760 | (integer) -> ? |
| set_sound_volume_by_class | 0x12ee790 | (string, integer) -> ? |
| set_string_option | 0x12ee7c0 | (string, string) -> ? |
| set_use_cluster | 0x12ee7f0 | (boolean, userdata) -> ? |
| set_use_system_cursor | 0x12ee840 | (boolean, userdata) -> ? |
| set_value | 0x12ee8e0 | (string, string) -> ? |
| set_vsync | 0x12ee890 | (boolean, userdata) -> ? |
| set_window_maximum_size | 0x12ee980 | (integer, integer, userdata) -> ? |
| set_window_minimum_size | 0x12ee9f0 | (integer, integer, userdata) -> ? |
| set_window_position | 0x12eea60 | (integer, integer, userdata) -> ? |
| set_window_resizable | 0x12eead0 | (boolean, userdata) -> ? |
| set_window_width_height_ratio | 0x12ee910 | (integer, integer, userdata) -> ? |
| SetAmbientOcclusionType | 0x12eda70 | (integer, userdata) -> ? |
| SetBackgroundTexturePath | 0x1281920 | (userdata) -> ? |
| SetBackgroundTextureUV | 0x1281920 | (userdata) -> ? |
| SetBakedShadow | 0x12edac0 | (boolean, userdata) -> ? |
| SetBangsHeight | 0x12edb10 | (number) -> ? |
| SetBloodStripCanvasVisible | 0x12edb40 | (boolean) -> ? |
| SetBoolOption | 0x12edb70 | (string, boolean) -> ? |
| SetCallstackMemoryProfilerEnable | 0x12edba0 | (boolean) -> ? |
| SetChooseAPIWindowTimes | 0x12edbd0 | (integer) -> ? |
| SetComputeSkinningEnabled | 0x12edc00 | (boolean, userdata) -> ? |
| SetCurrentGame | 0x12edc50 | (string) -> ? |
| SetCursorShape | 0x12edc80 | (string, string) -> ? |
| SetCursorVisible | 0x12edcb0 | (boolean, userdata) -> ? |
| SetCustomBinary | 0x12edd00 | (string) -> ? |
| SetDefaultBoolOption | 0x12edd30 | (string, boolean) -> ? |
| SetDefaultFloatOption | 0x12edd60 | (string, number) -> ? |
| SetDefaultStringOption | 0x12edd90 | (string, string) -> ? |
| SetDirectConnectHostMode | 0x12eddc0 | (boolean, userdata) -> ? |
| SetDrawShadows | 0x12ede00 | (boolean, userdata) -> ? |
| SetFloatOption | 0x12ede50 | (string, number) -> ? |
| SetFullscreen | 0x12ede80 | (boolean, userdata) -> ? |
| SetGamePlayCanvasVisible | 0x12edee0 | (boolean) -> ? |
| SetGameSpeed | 0x12edf10 | (number) -> ? |
| SetGameWorldLowFPSEnable | 0x12edf40 | (boolean) -> ? |
| SetGameWorldShowDebugView | 0x12edf70 | (number) -> ? |
| SetGameWorldShowNavMesh | 0x12edfa0 | (number) -> ? |
| SetGlobalMaterialPixelExcludes | 0x12edfd0 | (string) -> ? |
| SetGlobalMaterialVertexExcludes | 0x12ee000 | (string) -> ? |
| SetHomeIndicator | 0x1281920 | (userdata) -> ? |
| SetLagThresholds | 0x12ee030 | (integer, integer, userdata) -> ? |
| SetLandscape | 0x12a6750 | (userdata) -> 1 |
| SetLocalizationLanguage | 0x12ee0a0 | (string) -> ? |
| SetLockMaxFps | 0x12ee0d0 | (integer, userdata) -> ? |
| SetLogicView | 0x12ee120 | (integer, integer, userdata) -> ? |
| SetMapPakVersion | 0x12ee1c0 | (string, string, integer) -> ? |
| SetMaxFps | 0x12ee1f0 | (integer, userdata) -> ? |
| SetMergeDirectionalLightAndPointLight | 0x12ee240 | (boolean, userdata) -> ? |
| SetMinFps | 0x12ee290 | (integer, userdata) -> ? |
| SetMiniMapCanvasVisible | 0x12ee2e0 | (boolean) -> ? |
| SetMSAA | 0x12ee160 | (number, userdata) -> ? |
| SetNeedClearResourceCache | 0x12ee310 | (boolean) -> ? |
| SetOffScreenShadowEnabled | 0x12ee340 | (boolean, userdata) -> ? |
| SetOrientation | 0x12ee390 | (integer, userdata) -> ? |
| SetParticleDynamicBatchEnabled | 0x12ee3e0 | (boolean, userdata) -> ? |
| SetParticleLodLevel | 0x12ee430 | (integer) -> ? |
| SetPlanerShadowEnabled | 0x12ee460 | (boolean, userdata) -> ? |
| SetPointLightEnabled | 0x12ee4b0 | (boolean, userdata) -> ? |
| SetPostProcessEnabled | 0x12ee500 | (boolean) -> ? |
| SetRenderMask | 0x12ee530 | (boolean, userdata) -> ? |
| SetRenderQuality | 0x12ee580 | (integer, string) -> ? |
| SetRenderQualityNoCheck | 0x12ee5b0 | (integer, userdata) -> ? |
| SetResolution | 0x12ee600 | (integer, integer, boolean) -> ? |
| SetRiseLetterCanvasVisible | 0x12ee630 | (boolean) -> ? |
| SetSceneMute | 0x12ee660 | (boolean) -> ? |
| SetShadowMapSize | 0x12ee690 | (integer, userdata) -> ? |
| SetSimpleSkinningEnabled | 0x12ee6e0 | (boolean, userdata) -> ? |
| SetSkinType | 0x12ee730 | (integer) -> ? |
| SetSoundVolume | 0x12ee760 | (integer) -> ? |
| SetSoundVolumeByClass | 0x12ee790 | (string, integer) -> ? |
| SetStringOption | 0x12ee7c0 | (string, string) -> ? |
| SetUseCluster | 0x12ee7f0 | (boolean, userdata) -> ? |
| SetUseSyetemCursor | 0x12ee840 | (boolean, userdata) -> ? |
| SetValue | 0x12ee8e0 | (string, string) -> ? |
| SetVSync | 0x12ee890 | (boolean, userdata) -> ? |
| SetWindowLockWidthHeightRatio | 0x12ee910 | (integer, integer, userdata) -> ? |
| SetWindowMaximumSize | 0x12ee980 | (integer, integer, userdata) -> ? |
| SetWindowMinimumSize | 0x12ee9f0 | (integer, integer, userdata) -> ? |
| SetWindowPosition | 0x12eea60 | (integer, integer, userdata) -> ? |
| SetWindowResizable | 0x12eead0 | (boolean, userdata) -> ? |
| Shell | 0x12eeb30 | (string) -> ? |
| show_debug_view | 0x12edf70 | (number) -> ? |
| show_nav | 0x12edfa0 | (number) -> ? |
| show_window | 0x12eeb60 | (userdata) -> ? |
| ShowWindow | 0x12eeb60 | (userdata) -> ? |
| snapshot_memory | 0x12eeb90 | (string) -> ? |
| SnapshotMemory | 0x12eeb90 | (string) -> ? |
| stat_sender | 0x12eebc0 | (string, string, string) -> ? |
| StatSender | 0x12eebc0 | (string, string, string) -> ? |
| storage_settings | 0x12eebf0 | () -> ? |
| StorageSettings | 0x12eebf0 | () -> ? |
| string_hash | 0x12ecef0 | (string, userdata) -> 1 |
| Test | 0x1281920 | (userdata) -> ? |
| toggle_animation | 0x12eec20 | (userdata) -> ? |
| toggle_bg | 0x12eec50 | () -> ? |
| toggle_fullscreen | 0x12eec80 | (userdata) -> ? |
| toggle_game_ui | 0x12eece0 | () -> ? |
| toggle_instance | 0x12eed10 | (userdata) -> ? |
| toggle_particle | 0x12eed40 | () -> ? |
| toggle_postprocess | 0x12eed70 | (userdata) -> ? |
| toggle_shadow | 0x12eeda0 | (userdata) -> ? |
| toggle_show_boundingbox | 0x12eede0 | (userdata) -> ? |
| toggle_show_select | 0x12eee30 | (userdata) -> ? |
| toggle_show_unit_collision_grid | 0x12eee80 | (userdata) -> ? |
| toggle_show_unit_radius | 0x12eeed0 | (userdata) -> ? |
| toggle_terrain | 0x12eef20 | () -> ? |
| toggle_ui_scene | 0x12eef50 | (boolean, userdata) -> ? |
| toggle_vsync | 0x12eefa0 | (userdata) -> ? |
| ToggleAnimation | 0x12eec20 | (userdata) -> ? |
| ToggleBG | 0x12eec50 | () -> ? |
| ToggleFullscreen | 0x12eec80 | (userdata) -> ? |
| ToggleGameUI | 0x12eece0 | () -> ? |
| ToggleInstance | 0x12eed10 | (userdata) -> ? |
| ToggleParticle | 0x12eed40 | () -> ? |
| TogglePostProcess | 0x12eed70 | (userdata) -> ? |
| ToggleShadow | 0x12eeda0 | (userdata) -> ? |
| ToggleShowBoundingBox | 0x12eede0 | (userdata) -> ? |
| ToggleShowSelect | 0x12eee30 | (userdata) -> ? |
| ToggleShowUnitCollisionGrid | 0x12eee80 | (userdata) -> ? |
| ToggleShowUnitRadius | 0x12eeed0 | (userdata) -> ? |
| ToggleTerrain | 0x12eef20 | () -> ? |
| ToggleUIScene | 0x12eef50 | (boolean, userdata) -> ? |
| ToggleVSync | 0x12eefa0 | (userdata) -> ? |
| trigger_rdoc_capture | 0x12eeff0 | (userdata) -> ? |
| TriggerRenderdocCapture | 0x12eeff0 | (userdata) -> ? |
| unlock_scene_view | 0x12ef020 | (userdata) -> ? |
| UnlockSceneView | 0x12ef020 | (userdata) -> ? |
| unstorage_settings | 0x12ef050 | () -> ? |
| UnstorageSettings | 0x12ef050 | () -> ? |
| utc_time | 0x12ef080 | (userdata) -> 1 |
| UtcTime | 0x12ef080 | (userdata) -> 1 |
| write_profile_detail | 0x12ef0e0 | (string, boolean) -> ? |
| WriteProfileDetail | 0x12ef0e0 | (string, boolean) -> ? |
