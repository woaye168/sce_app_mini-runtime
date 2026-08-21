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

/// 生成暂存目录，返回路径。幂等（先清后建）。
pub fn create(project_root: &Path, runtime_dir: &Path, project_name: &str) -> Result<PathBuf> {
    let dir_name = project_root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| anyhow!("项目路径异常"))?;
    let staging = runtime_dir.join("User").join("debug").join(&dir_name);
    if staging.exists() {
        // 清内容留目录（对齐 clear_folder 语义）
        for entry in std::fs::read_dir(&staging)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                std::fs::remove_dir_all(&p)?;
            } else {
                std::fs::remove_file(&p)?;
            }
        }
    } else {
        std::fs::create_dir_all(&staging)?;
    }

    // 白名单复制
    for d in DIRS {
        let src = project_root.join(d);
        if src.is_dir() {
            copy_dir(&src, &staging.join(d))?;
        }
    }
    for f in FILES {
        let src = project_root.join(f);
        if src.is_file() {
            std::fs::copy(&src, staging.join(f))?;
        }
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

    Ok(staging)
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

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let s = entry.path();
        let d = dst.join(entry.file_name());
        if s.is_dir() {
            copy_dir(&s, &d)?;
        } else {
            std::fs::copy(&s, &d)?;
        }
    }
    Ok(())
}
