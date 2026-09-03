//! 编辑器根定位：项目 map_settings.json api_version → tsconfig.json typeRoots 推编辑器根
//! → 上两级为运行根（含 version-<api>）。凭证/调试都从这里推。

use anyhow::{anyhow, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// 编辑器定位结果
#[derive(Debug, Clone)]
pub struct EditorLocate {
    /// 编辑器 api 版本（如 "13"）
    pub api_version: String,
    /// 编辑器根（update/<env> 目录，如 D:/sce_online/update/editor-pd.spark.xd.com）
    pub editor_root: PathBuf,
    /// 运行根（editor_root 上两级，如 D:/sce_online）
    pub engine_root: PathBuf,
    /// 环境域名（editor_root 末段，如 editor-pd.spark.xd.com）
    pub env_domain: String,
}

impl EditorLocate {
    /// 版本目录 <运行根>/version-<api>
    pub fn version_dir(&self) -> PathBuf {
        self.engine_root.join(format!("version-{}", self.api_version))
    }
    /// 凭证文件 <运行根>/User/user_info-<env>.json
    pub fn user_info_file(&self) -> PathBuf {
        self.engine_root
            .join("User")
            .join(format!("user_info-{}.json", self.env_domain))
    }
    /// 编辑器 exe <运行根>/星火编辑器.exe
    pub fn editor_exe(&self) -> PathBuf {
        self.engine_root.join("星火编辑器.exe")
    }
}

/// 从项目根定位编辑器。项目根需含 map_settings.json + script/tsconfig.json。
pub fn locate(project_root: &Path) -> Result<EditorLocate> {
    let api_version = read_api_version(project_root)?;
    let editor_root = find_editor_root(project_root)?;
    let engine_root = editor_root
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow!("编辑器根无法上溯两级: {}", editor_root.display()))?
        .to_path_buf();
    if !engine_root
        .join(format!("version-{}", api_version))
        .is_dir()
    {
        return Err(anyhow!(
            "运行根 {} 下不存在 version-{} 目录",
            engine_root.display(),
            api_version
        ));
    }
    let env_domain = editor_root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    Ok(EditorLocate {
        api_version,
        editor_root,
        engine_root,
        env_domain,
    })
}

/// map_settings.json 的 api_version（兼容字符串/数字）
fn read_api_version(project_root: &Path) -> Result<String> {
    let candidates = [
        project_root.join("project").join("map_settings.json"),
        project_root.join("map_settings.json"),
    ];
    for path in candidates {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let json: Value = serde_json::from_str(&content)
                .map_err(|e| anyhow!("解析 {} 失败: {e}", path.display()))?;
            if let Some(v) = json.get("api_version") {
                return Ok(match v {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    // 新版格式：{"api_version": {"api_version": 13, "show_name": "13"}}
                    Value::Object(o) => o
                        .get("api_version")
                        .and_then(|n| n.as_u64().map(|u| u.to_string()).or_else(|| n.as_str().map(|s| s.to_string())))
                        .or_else(|| o.get("show_name").and_then(|s| s.as_str()).map(|s| s.to_string()))
                        .ok_or_else(|| anyhow!("api_version 对象缺 api_version/show_name"))?,
                    _ => return Err(anyhow!("api_version 类型异常")),
                });
            }
        }
    }
    Err(anyhow!("找不到 map_settings.json 的 api_version"))
}

/// tsconfig.json typeRoots 推编辑器根（`/Res/_m/` 之前的部分）
fn find_editor_root(project_root: &Path) -> Result<PathBuf> {
    let path = project_root.join("script").join("tsconfig.json");
    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("读取 {} 失败: {e}", path.display()))?;
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| anyhow!("解析 {} 失败: {e}", path.display()))?;
    let roots = json
        .get("compilerOptions")
        .and_then(|c| c.get("typeRoots"))
        .or_else(|| json.get("typeRoots"))
        .and_then(|r| r.as_array())
        .ok_or_else(|| anyhow!("tsconfig.json 缺少 compilerOptions.typeRoots"))?;
    for root in roots {
        if let Some(s) = root.as_str() {
            // typeRoots 条目形如 <编辑器根>/Res/_m/... 或含 /res/_m/
            // 先 replace 得 owned 串再 lowercase 匹配（lowercase 对非 ASCII 可能改变字节长度，
            // 其下标不能回切原始串；切的是同一 owned 串，与 payload.rs derive_editor_paths 一致）
            let norm = s.replace('\\', "/");
            let lower = norm.to_lowercase();
            if let Some(idx) = lower.find("/res/_m/") {
                let editor_root = PathBuf::from(&norm[..idx]);
                if editor_root.is_dir() {
                    return Ok(editor_root);
                }
            }
        }
    }
    Err(anyhow!("tsconfig.json typeRoots 无法推编辑器根"))
}
