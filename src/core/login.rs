//! TapTap OAuth2 device flow 自登录（复现 xdeditor ui/login.lua，纯 HTTPS）。
//! 二维码以 PNG 字节返回，UI 层渲染。

use crate::core::auth::UserInfo;
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// 各环境 client_id（client_base base/client_id.lua 实证）
pub fn client_id(env_domain: &str) -> &'static str {
    match env_domain {
        "editor-alpha.spark.xd.com" | "editor-fj-review.spark.xd.com" => "qllpthuj3ep4aywb0o",
        "editor-beta.spark.xd.com" => "mntdx77vu0jk07fatr",
        "editor-pd.spark.xd.com" => "YGySNSXKlgec6kROit",
        "editor-intl.spark.xd.com" | "editor-intl-beta.spark.xd.com" => "yrhvhmeeevciti1cl5",
        _ => "5csnnikv4bp68x6qxz",
    }
}

const OAUTH_BASE: &str = "https://www.taptap.com";

fn http() -> reqwest::blocking::Client {
    let mut builder = reqwest::blocking::Client::builder().timeout(Duration::from_secs(30));
    if let Some(p) = crate::core::verify::proxy() {
        builder = builder.proxy(p);
    }
    builder.build().expect("reqwest client")
}

/// device/code 申请结果
#[derive(Debug, Clone)]
pub struct DeviceGrant {
    pub device_code: String,
    pub qrcode_url: String,
    pub interval: u64,
    pub expires_in: u64,
}

/// 登录状态（供 UI 轮询展示）
#[derive(Debug, Clone, PartialEq)]
pub enum LoginState {
    Pending,          // 等待扫码
    WaitingConfirm,   // 已扫待确认
    Done(UserInfo),   // 成功
    Denied,           // 拒绝
    Expired,          // 超时
    Failed(String),   // 失败
}

/// 第一步：申请 device_code + qrcode_url
pub fn request_device_code(env_domain: &str) -> Result<DeviceGrant> {
    let resp = http()
        .post(format!("{OAUTH_BASE}/oauth2/v1/device/code"))
        .form(&[
            ("client_id", client_id(env_domain)),
            ("response_type", "device_code"),
            ("scope", "public_profile"),
            ("version", "1.0"),
            ("platform", "nodejs"),
            ("info", r#"{"device_id":"PC"}"#),
        ])
        .send()
        .map_err(|e| anyhow!("device/code 请求失败: {e}"))?;
    let text = resp.text()?;
    let j: Value = serde_json::from_str(&text)?;
    if j["success"].as_bool() != Some(true) {
        return Err(anyhow!("device/code 失败: {text}"));
    }
    let d = &j["data"];
    Ok(DeviceGrant {
        device_code: d["device_code"].as_str().unwrap_or_default().to_string(),
        qrcode_url: d["qrcode_url"].as_str().unwrap_or_default().to_string(),
        interval: d["interval"].as_u64().unwrap_or(2),
        expires_in: d["expires_in"].as_u64().unwrap_or(300),
    })
}

/// 第二步：轮询授权结果（阻塞，UI 应放线程里跑）
/// cancel 置位后下一轮即退出（取消登录用，防轮询线程泄漏到二维码自然过期）
pub fn poll_device_token(env_domain: &str, grant: &DeviceGrant, on_state: impl Fn(LoginState), cancel: Option<&AtomicBool>) -> LoginState {
    let deadline = Instant::now() + Duration::from_secs(grant.expires_in);
    let client = http();
    loop {
        if cancel.is_some_and(|c| c.load(Ordering::Relaxed)) {
            return LoginState::Failed("已取消".to_string());
        }
        if Instant::now() >= deadline {
            return LoginState::Expired;
        }
        std::thread::sleep(Duration::from_secs(grant.interval.max(1)));
        let resp = client
            .post(format!("{OAUTH_BASE}/oauth2/v1/token"))
            .form(&[
                ("grant_type", "device_token"),
                ("client_id", client_id(env_domain)),
                ("secret_type", "hmac-sha-1"),
                ("code", grant.device_code.as_str()),
                ("version", "1.0"),
                ("platform", "unity"),
                ("info", r#"{"device_id": "PC"}"#),
            ])
            .send();
        let Ok(resp) = resp else { continue };
        let Ok(text) = resp.text() else { continue };
        let Ok(j) = serde_json::from_str::<Value>(&text) else { continue };
        if j["success"].as_bool() == Some(true) {
            let d = &j["data"];
            let kid = d["kid"].as_str().unwrap_or_default();
            let mac_key = d["mac_key"].as_str().unwrap_or_default();
            let access_token = d["access_token"].as_str().unwrap_or_default();
            let info = UserInfo {
                access_token: access_token.to_string(),
                token: format!("{mac_key}${kid}"),
                token_type: 11,
                version: 1,
                ..Default::default()
            };
            let st = LoginState::Done(info);
            on_state(st.clone());
            return st;
        }
        let err = j["data"]["error"].as_str().unwrap_or_default();
        let st = match err {
            "authorization_waiting" => LoginState::WaitingConfirm,
            "authorization_pending" => LoginState::Pending,
            "access_denied" => {
                on_state(LoginState::Denied);
                return LoginState::Denied;
            }
            other => {
                // 未知错误（expired_token/slow_down/未来新增错误码）：立即返回真实原因，
                // 不再继续轮询到超时把失败掩盖成 Expired（slow_down 继续原间隔轮询还会加重限流）
                let st = LoginState::Failed(format!("未知状态: {other}"));
                on_state(st.clone());
                return st;
            }
        };
        on_state(st);
    }
}

/// 二维码 URL → PNG 字节
pub fn qrcode_png(url: &str, size: u32) -> Result<Vec<u8>> {
    let code = qrcode::QrCode::new(url.as_bytes())?;
    let img = code.render::<image::Luma<u8>>().min_dimensions(size, size).build();
    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}
