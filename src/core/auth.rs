//! 凭证管理：读取编辑器 user_info-*.json、凭证库（多账号存档/切换/收割）。
//! 凭证库落盘：<exe 旁>/sce_app_mini-runtime.credentials.json（appsdk 应用配置旁路）。

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 编辑器凭证文件（User/user_info-<env>.json）的关键字段
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub guest_id: String,
    #[serde(default)]
    pub login: i64,
    #[serde(default)]
    pub login_token: String,
    #[serde(default)]
    pub login_token_secret: String,
    #[serde(default)]
    pub login_type: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub token_type: i64,
    #[serde(default)]
    pub version: i64,
}

impl UserInfo {
    /// token 是否有效（官方 token_valid()：token 非空且 token_type ∈ [11,14]）
    pub fn token_valid(&self) -> bool {
        !self.token.is_empty() && (11..=14).contains(&self.token_type)
    }
    /// HTTP 签名对是否齐（login_token + secret 非空）
    pub fn can_sign(&self) -> bool {
        !self.login_token.is_empty() && !self.login_token_secret.is_empty()
    }
    pub fn token_type_name(&self) -> &'static str {
        match self.token_type {
            11 => "编辑器TapTap",
            13 => "手机TapTap",
            14 => "安卓容器",
            999 => "游客",
            _ => "未知",
        }
    }
}

/// 读凭证文件
pub fn read_user_info(path: &Path) -> Result<UserInfo> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow!("读取凭证失败 {}: {e}", path.display()))?;
    serde_json::from_str(&content).map_err(|e| anyhow!("解析凭证失败: {e}"))
}

/// 写凭证文件
pub fn write_user_info(path: &Path, info: &UserInfo) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(info)?;
    std::fs::write(path, content).map_err(|e| anyhow!("写凭证失败: {e}"))
}

/// 凭证库条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    /// 账号备注（用户填）
    pub label: String,
    /// 环境域名（如 editor-pd.spark.xd.com）
    pub env_domain: String,
    /// 凭证内容
    pub info: UserInfo,
}

/// 凭证库（按 label 索引）
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CredentialStore {
    #[serde(default)]
    pub items: BTreeMap<String, Credential>,
    #[serde(default)]
    pub active_label: Option<String>,
}

fn store_path() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe.with_file_name("sce_app_mini-runtime.credentials.json")
}

impl CredentialStore {
    pub fn load() -> Self {
        let path = store_path();
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(store_path(), content).map_err(|e| anyhow!("写凭证库失败: {e}"))
    }

    /// 收割：把编辑器当前凭证收进库（label 不存在则新建）
    pub fn harvest(&mut self, label: &str, env_domain: &str, info: UserInfo) {
        self.items.insert(
            label.to_string(),
            Credential {
                label: label.to_string(),
                env_domain: env_domain.to_string(),
                info,
            },
        );
    }

    /// 切换：把库中指定凭证写到编辑器凭证文件
    pub fn apply(&mut self, label: &str, user_info_path: &Path) -> Result<()> {
        let cred = self
            .items
            .get(label)
            .ok_or_else(|| anyhow!("凭证不存在: {label}"))?;
        write_user_info(user_info_path, &cred.info)?;
        self.active_label = Some(label.to_string());
        self.save()?;
        Ok(())
    }
}
