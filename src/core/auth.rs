//! 凭证管理：读取编辑器 user_info-*.json、凭证库（多账号存档/切换/收割）。
//! 凭证库落盘：<exe 旁>/sce_app_mini-runtime.credentials.json（appsdk 应用配置旁路）。

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 按用户要求的凭证名格式生成：{userid}_{YYYYMMDD_HHMMSS}_{env}_{token_type}
/// userid 缺（旧凭证没有字段）时用 `no-uid` 占位。
pub fn make_label(info: &UserInfo, env_domain: &str) -> String {
    let uid = info
        .userid
        .map(|u| u.to_string())
        .unwrap_or_else(|| "no-uid".into());
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // 转本地时间 YYYYMMDD_HHMMSS（UTC+8）
    let t = now + 8 * 3600;
    let (y, mo, d, h, mi, s) = epoch_to_ymd(t);
    format!(
        "{uid}_{y:04}{mo:02}{d:02}_{h:02}{mi:02}{s:02}_{}_{}",
        env_domain, info.token_type
    )
}

/// unix 秒（UTC+8 已偏移）→ (年,月,日,时,分,秒)，civil-from-days 算法
fn epoch_to_ymd(t: u64) -> (i64, u32, u32, u32, u32, u32) {
    let days = (t / 86400) as i64;
    let secs = t % 86400;
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if mo <= 2 { y + 1 } else { y };
    (
        y,
        mo,
        d,
        (secs / 3600) as u32,
        (secs % 3600 / 60) as u32,
        (secs % 60) as u32,
    )
}

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
    /// 账号数字 userid（最新登录态写盘；旧凭证可能没有该字段）
    #[serde(default)]
    pub userid: Option<i64>,
    /// 账号昵称（展示用，登录态提供）
    #[serde(default)]
    pub user_name: Option<String>,
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

    /// 账号数字 userid：读凭证字段（最新登录态写盘；旧凭证没有则 None）。
    /// 注意：token 的 kid 段是 opaque 随机字节，**不是** protobuf，不能从中解 userid（0.2.1 探针实证）。
    pub fn userid(&self) -> Option<i64> {
        self.userid
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

    /// 刷新某条目的 userid/user_name（登录态字段），并落盘
    pub fn update_identity(&mut self, label: &str, userid: i64, user_name: Option<String>) -> Result<()> {
        let cred = self
            .items
            .get_mut(label)
            .ok_or_else(|| anyhow!("凭证不存在: {label}"))?;
        cred.info.userid = Some(userid);
        if user_name.is_some() {
            cred.info.user_name = user_name;
        }
        self.save()
    }
}
