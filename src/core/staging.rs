//! 暂存目录生成（M5）：项目 → <runtime>/User/debug/<目录名>，对标编辑器 map_starter 的暂存布局。
//! 规则实证：doc/research/scegame-reverse.md §9.2（main.lua 包装 = 模板头 + origin_main_file + ts_module 尾）。

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// 白名单目录（map_starter 的 project_manager.get_project_map_dirs 对应集合）
const DIRS: &[&str] = &[
    "atmosphere", "block", "game_hud", "i18n", "project", "ref", "res", "scene", "script",
    "src", "table", "ui",
];
/// 白名单文件
const FILES: &[&str] = &["config.ini", "libs.json", "project.sce"];

/// 库 main 模块的返回键后缀（官方模板实证；未知库裸 require）
fn lib_require_suffix(name: &str) -> &'static str {
    match name {
        "defaultui" => ".defaultui",
        "lib_common_ai" => ".lib_common_ai",
        "lib_control" => ".lib_control",
        "smallcard_get_items" => ".smallcard_get_items",
        "smallcard_inventory" => ".smallcard_inventory",
        "smallcard_mail" => ".smallcard_mail",
        _ => "",
    }
}

/// 生成暂存目录，返回路径。**增量**：不清目录，按 mtime+size 只更新变化的文件
///（staging 硬链接时代与项目同卷；跨卷回退复制也只补差异——秒开的关键）。
pub fn create(project_root: &Path, runtime_dir: &Path, project_name: &str) -> Result<PathBuf> {
    let dir_name = project_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| anyhow!("项目路径异常"))?;
    let staging = runtime_dir.join("User").join("debug").join(&dir_name);
    create_at(project_root, &staging, project_name)
}

/// 在指定目录生成暂存（create 的布局推导与生成主体分离；CLI `staging create --staging` 直指定目录用）
pub fn create_at(project_root: &Path, staging: &Path, project_name: &str) -> Result<PathBuf> {
    std::fs::create_dir_all(staging)?;

    // 白名单目录增量同步（优先硬链接；目标已存在且同源（size+mtime 一致）则跳过）
    let mut updated = 0u32;
    let mut linked_mode = true;
    for d in DIRS {
        let src = project_root.join(d);
        if src.is_dir() {
            if !sync_dir(&src, &staging.join(d), linked_mode, &mut updated)? {
                if linked_mode {
                    crate::core::logbus::push("[staging] [warn] 硬链接不受支持（跨卷），已降级为复制模式同步".into());
                }
                linked_mode = false; // 硬链接不受支持（跨卷），后续目录走复制
            }
        }
    }
    for f in FILES {
        let src = project_root.join(f);
        if src.is_file() {
            if sync_file(&src, &staging.join(f), linked_mode)? {
                updated += 1;
            }
        }
    }
    if updated > 0 {
        crate::core::logbus::push(format!("[staging] 增量更新 {updated} 个文件（{}）", if linked_mode { "硬链接" } else { "复制" }));
    }

    // 清理 staging 中项目侧已删除/改名的残留（防陈旧文件经 manifest 下发远端客户端）
    let removed = prune_staging(project_root, &staging)?;
    if removed > 0 {
        crate::core::logbus::push(format!("[staging] 清理残留 {removed} 项"));
    }

    // ui/script/main.lua：项目已有（编辑器/bgd 生成过）则沿用；否则按官方规则包装生成
    let ui_script = staging.join("ui").join("script");
    let main_lua = ui_script.join("main.lua");
    if !main_lua.is_file() {
        let origin = staging.join("ui").join("src").join("main.lua");
        let origin_body = std::fs::read_to_string(&origin)
            .map_err(|e| anyhow!("项目缺 ui/src/main.lua（{}）: {e}", origin.display()))?;
        std::fs::create_dir_all(&ui_script)?;
        let wrapped = wrap_client_main(&origin_body, project_root, project_name)?;
        std::fs::write(&main_lua, wrapped)?;
    }

    // 触发器 stub：触编 tstl 产物缺失时补空 stub（bgd 项目走纯 lua，触发器 0.2.x 接官方链）
    for stub in ["trigger_module_main_1.lua", "trigger_validator.lua"] {
        let path = ui_script.join(stub);
        if !path.is_file() {
            std::fs::write(&path, "-- stub by mini-runtime（触发器未编译，0.2.x 接入官方 tstl 链）\n____module = ____module or {}\n")?;
        }
    }

    Ok(staging.to_path_buf())
}

/// 客户端入口包装：官方模板头 + `---origin_main_file---` + 源 ui/src/main.lua 原文 + `---ts_module---` 尾
fn wrap_client_main(origin_body: &str, project_root: &Path, project_name: &str) -> Result<String> {
    // libs 段按 libs.json 生成
    let libs_path = project_root.join("libs.json");
    let mut libs_section = String::new();
    if let Ok(content) = std::fs::read_to_string(&libs_path) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(obj) = v.as_object() {
                for name in obj.keys() {
                    libs_section.push_str(&format!(
                        "{name} = require\"@{name}.main\"{}\n",
                        lib_require_suffix(name)
                    ));
                }
            }
        }
    }

    Ok(format!(
        r#"---require_common---
require"@common.base"
require"@global_default.lua_declare"
---load localization file---
base.i18n.load_map("{project_name}")
---scene_folder---
xpcall(require_folder, function(err) log.info(string.format("调用失败：%s", err)) end, "scene")
---init data object cache---
if base.eff.has_cache_init() then
else
	base.eff.init_cache()
end
---new struct creater---

base.new_struct_creater = {{}}
function base.proto.__server_custom_event_struct_creater(msg)
    if msg.struct_name and base.new_struct_creater then
        base.new_struct_creater[msg.struct_name] = function()
            return msg.struct
        end
    end
end
function _send_custom_event_struct_creater(param_name, param_struct)
    base.game:server'__client_custom_event_struct_creater'{{
        struct_name = param_name,
        struct = param_struct,
    }}
end

---require libs---
{libs_section}
---gui---
do
    local res, page = xpcall(require, function(err) end, "gui.page")
    if res and page then
        local MainPage = page.MainPage
        if MainPage then
            local main_page = MainPage:new()
            _ENV.page_components = page
            _ENV.main_page = main_page
            _G.__main_page = main_page
        end
    end
end
---origin_main_file---
{origin_body}
---ts_module---
require "trigger_module_main_1"
require "trigger_validator"
local ret = {{["{project_name}"] = {project_name}}}
for k, v in pairs(____module or {{}}) do ret["{project_name}"][k] = v end
for k, v in pairs(____return or {{}}) do ret[k] = v end
return ret
"#
    ))
}

/// 单文件增量同步（根级 FILES 用）：内容一致跳过，否则硬链接/复制
fn sync_file(src: &Path, dst: &Path, linked_mode: bool) -> Result<bool> {
    if files_equal(src, dst) {
        return Ok(false);
    }
    if dst.exists() {
        let _ = std::fs::remove_file(dst);
    }
    if let Some(p) = dst.parent() {
        std::fs::create_dir_all(p)?;
    }
    if linked_mode && std::fs::hard_link(src, dst).is_ok() {
        return Ok(true);
    }
    std::fs::copy(src, dst)?;
    Ok(true)
}

/// 内容级一致判定：尺寸不等先快速否；同尺寸逐字节比对（分块读，不大内存）
fn files_equal(a: &Path, b: &Path) -> bool {
    let (Ok(ma), Ok(mb)) = (std::fs::metadata(a), std::fs::metadata(b)) else {
        return false;
    };
    if ma.len() != mb.len() {
        return false;
    }
    let (Ok(mut fa), Ok(mut fb)) = (std::fs::File::open(a), std::fs::File::open(b)) else {
        return false;
    };
    use std::io::Read;
    let mut ba = [0u8; 65536];
    let mut bb = [0u8; 65536];
    loop {
        let na = fa.read(&mut ba).unwrap_or(0);
        let nb = fb.read(&mut bb).unwrap_or(0);
        if na != nb {
            return false;
        }
        if na == 0 {
            return true;
        }
        if ba[..na] != bb[..nb] {
            return false;
        }
    }
}

/// 目录树增量同步：递归逐文件判定。**硬链接仅在从未成功过的目录尝试**（hard_link 失败一次
/// 即整体转复制模式，避免"内容判定后 remove+hard_link 失败"丢文件），返回 false 由调用方转复制。
/// changed 全局记忆：父目录发生过更新后，其余兄弟目录一律走复制模式（防 linked 残留判定漏拷）。
/// 降级不中断遍历：hard_link 失败的本文件复制落位后继续同步同目录剩余条目，仅在返回时上报降级。
fn sync_dir(src: &Path, dst: &Path, linked_mode: bool, updated: &mut u32) -> Result<bool> {
    std::fs::create_dir_all(dst)?;
    let mut linked = linked_mode;
    let mut degraded = false; // 本目录树发生过硬链接降级（跨卷），返回 false 让后续目录直接复制
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let s = entry.path();
        let d = dst.join(entry.file_name());
        if s.is_dir() {
            if !sync_dir(&s, &d, linked, updated)? {
                // 子目录降级：本目录剩余条目继续用复制模式走完，不中断遍历
                linked = false;
                degraded = true;
            }
        } else if files_equal(&s, &d) {
            continue;
        } else if linked && !d.exists() {
            // 快路径：目标不存在（首装）直接硬链接；失败 = 跨卷，本文件降级复制并继续遍历
            if std::fs::hard_link(&s, &d).is_err() {
                std::fs::copy(&s, &d)?;
                linked = false;
                degraded = true;
            }
            *updated += 1;
        } else {
            // 复制模式（或 linked 但目标已存在）：remove+hard_link 有"先删后失败"丢文件风险，
            // 一律覆盖复制（硬链接的收益只在首装）
            std::fs::copy(&s, &d)?;
            *updated += 1;
        }
    }
    Ok(!degraded)
}

/// 清理 staging 白名单范围内项目侧已不存在的条目（删除/改名残留会随 manifest 下发远端）。
/// 仅清理白名单目录/文件；ui/script 为工具自生成目录（包装入口 + 触发器 stub），整目录跳过。
fn prune_staging(project_root: &Path, staging: &Path) -> Result<u32> {
    let mut removed = 0u32;
    let ui_script = staging.join("ui").join("script");
    for d in DIRS {
        prune_dir(&project_root.join(d), &staging.join(d), &ui_script, &mut removed)?;
    }
    for f in FILES {
        let d = staging.join(f);
        if d.is_file() && !project_root.join(f).is_file() {
            std::fs::remove_file(&d)?;
            removed += 1;
        }
    }
    Ok(removed)
}

/// 递归清理 dst 中 src 已不存在的条目
fn prune_dir(src: &Path, dst: &Path, ui_script: &Path, removed: &mut u32) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(dst) else {
        return Ok(()); // dst 不存在（源端本就没有该目录）
    };
    for entry in entries.flatten() {
        let d = entry.path();
        if d == ui_script {
            continue; // 工具自生成目录，不归源端管
        }
        let s = src.join(entry.file_name());
        if !s.exists() {
            if d.is_dir() {
                std::fs::remove_dir_all(&d)?;
            } else {
                std::fs::remove_file(&d)?;
            }
            *removed += 1;
        } else if d.is_dir() && s.is_dir() {
            prune_dir(&s, &d, ui_script, removed)?;
        }
    }
    Ok(())
}
