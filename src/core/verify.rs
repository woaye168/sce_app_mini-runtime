//! 登录态验证：内部 HTTP API 签名（account.lua:412-441 复现）+ updater/api-version 试调。

use crate::core::auth::UserInfo;
use anyhow::{anyhow, Result};
use md5::Digest;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// 生成签名 header（noise/time_str/token/sign）
pub fn sign_headers(info: &UserInfo) -> Result<[(String, String); 4]> {
    if !info.can_sign() {
        return Err(anyhow!(
            "凭证缺 login_token/secret（该凭证是在编辑器未完整登录时收割的）——请到「凭证」页重新「导入编辑器凭证」覆盖，或换一个凭证"
        ));
    }
    let noise = format!("{}", 1000000 + (rand_u32() % 9000000));
    let time_str = format!(
        "{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
    );
    let pre_sign = format!(
        "{noise}\n{time_str}\n\n{}\n{}",
        info.login_token, info.login_token_secret
    );
    let sign = format!("{:x}", md5::Md5::digest(pre_sign.as_bytes()));
    Ok([
        ("noise".into(), noise),
        ("time_str".into(), time_str),
        ("token".into(), info.login_token.clone()),
        ("sign".into(), sign),
    ])
}

// 简单随机（不引 rand crate：用时间戳纳秒搅合）
fn rand_u32() -> u32 {
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    n ^ (n >> 16)
}

/// 服务地址（utility.lua 推导；新域名体系走 443 不带端口——9000/9002/9011 是旧域名时代的端口，
/// 实测 publisher-pd.tapsce.cn 的 443 通、9000+ 不通）
pub fn service_url(service: &str, _port: u16, env_domain: &str) -> String {
    let env = env_domain
        .trim_start_matches("editor-")
        .split('.')
        .next()
        .unwrap_or("pd");
    format!("https://{service}-{env}.tapsce.cn")
}

/// 代理：仅显式设置环境变量 MINI_RUNTIME_PROXY 时走代理（星火官方接口是国内 CDN，直连即可）
pub fn proxy() -> Option<reqwest::Proxy> {
    // v0.6.1 前默认 127.0.0.1:7897 是开发机便利残留——对端机器没有该代理会导致
    // 全部官方请求失败"error sending request"
    let url = std::env::var("MINI_RUNTIME_PROXY")
        .ok()
        .filter(|s| !s.is_empty())?;
    reqwest::Proxy::all(&url).ok()
}

/// 验证登录态：签名调 updater/api-version（最低风险 API）
pub fn verify(info: &UserInfo, env_domain: &str) -> Result<String> {
    let headers = sign_headers(info)?;
    let url = format!("{}/api/map/api-version", service_url("updater", 9002, env_domain));
    let mut builder = reqwest::blocking::Client::builder().timeout(Duration::from_secs(20));
    if let Some(p) = proxy() {
        builder = builder.proxy(p);
    }
    let client = builder.build()?;
    let mut req = client.post(&url);
    for (k, v) in headers {
        req = req.header(k, v);
    }
    let resp = req.send().map_err(|e| anyhow!("请求失败: {e}"))?;
    let status = resp.status();
    let text = resp.text()?;
    if !status.is_success() {
        return Err(anyhow!("HTTP {status}: {text}"));
    }
    Ok(text)
}
