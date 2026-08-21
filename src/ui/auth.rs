//! 凭证标签页：编辑器当前凭证 / 凭证库（收割/切换/删除/验证）/ 扫码自登录

use crate::core::{auth, login, verify};
use crate::App;

impl App {
    pub(crate) fn ui_auth(&mut self, ui: &mut egui::Ui) {
        ui.heading("凭证");

        let Some(locate) = self.locate.clone() else {
            ui.label(if self.locate_err.is_empty() {
                "请先在宿主中打开项目".to_string()
            } else {
                format!("编辑器定位失败：{}", self.locate_err)
            });
            return;
        };
        let user_info_path = locate.user_info_file();

        // ---- 当前编辑器凭证 ----
        ui.group(|ui| {
            ui.label(format!("凭证文件：{}", user_info_path.display()));
            match auth::read_user_info(&user_info_path) {
                Ok(info) => {
                    ui.label(format!(
                        "状态：{}  token_type={}({})  签名对={}",
                        if info.token_valid() { "有效" } else { "无效/游客" },
                        info.token_type,
                        info.token_type_name(),
                        info.can_sign()
                    ));
                    ui.horizontal(|ui| {
                        ui.label("导入为：");
                        ui.text_edit_singleline(&mut self.cred_new_label);
                        if ui.button("导入编辑器凭证").clicked() && !self.cred_new_label.is_empty() {
                            // 只读复制一份进凭证库，不写回——编辑器原凭证不动，不会踢下线
                            self.cred_store.harvest(
                                &self.cred_new_label.clone(),
                                &locate.env_domain,
                                info,
                            );
                            let _ = self.cred_store.save();
                            self.status = format!("已导入凭证：{}（编辑器原凭证未动）", self.cred_new_label);
                            self.cred_new_label.clear();
                        }
                    });
                }
                Err(e) => {
                    ui.label(format!("读取失败：{e}（编辑器可能从未登录）"));
                }
            }
        });

        ui.add_space(8.0);

        // ---- 凭证库 ----
        ui.label("凭证库：");
        let labels: Vec<String> = self.cred_store.items.keys().cloned().collect();
        for label in labels {
            let cred = self.cred_store.items[&label].clone();
            ui.horizontal(|ui| {
                let active = self.cred_store.active_label.as_deref() == Some(label.as_str());
                ui.label(format!(
                    "{}{}  [{}]  {}({})",
                    if active { "★ " } else { "" },
                    label,
                    cred.env_domain,
                    cred.info.token_type,
                    cred.info.token_type_name()
                ));
                if ui.button("切换").clicked() {
                    match self.cred_store.apply(&label, &user_info_path) {
                        Ok(_) => self.status = format!("已切换到凭证：{label}"),
                        Err(e) => self.status = format!("切换失败：{e}"),
                    }
                }
                if ui.button("验证").clicked() {
                    self.status = format!("验证中：{label}...");
                    match verify::verify(&cred.info, &cred.env_domain) {
                        Ok(text) => {
                            self.verify_result = format!("{label} 验证通过：{}", &text[..text.len().min(120)]);
                        }
                        Err(e) => {
                            self.verify_result = format!("{label} 验证失败：{e}");
                        }
                    }
                }
                if ui.button("删除").clicked() {
                    self.cred_store.items.remove(&label);
                    let _ = self.cred_store.save();
                }
            });
        }
        if !self.verify_result.is_empty() {
            ui.label(&self.verify_result);
        }

        ui.add_space(12.0);
        ui.separator();

        // ---- 扫码自登录 ----
        ui.label("扫码自登录（TapTap）：");
        if self.login_grant.is_none() && self.login_rx.is_none() {
            if ui.button("申请登录二维码").clicked() {
                match login::request_device_code(&locate.env_domain) {
                    Ok(grant) => {
                        match login::qrcode_png(&grant.qrcode_url, 200) {
                            Ok(png) => {
                                // 二维码落盘（用户可直接打开扫码）
                                let qr_path = std::env::current_exe()
                                    .map(|e| e.with_file_name("login_qrcode.png"))
                                    .unwrap_or_else(|_| "login_qrcode.png".into());
                                let _ = std::fs::write(&qr_path, &png);
                                self.status = format!("二维码已保存：{}", qr_path.display());
                                let img = egui::ColorImage::from_rgba_unmultiplied(
                                    [200, 200],
                                    &rgba_of(&png),
                                );
                                self.login_qr = Some(ui.ctx().load_texture(
                                    "login_qr",
                                    img,
                                    egui::TextureOptions::LINEAR,
                                ));
                            }
                            Err(e) => self.status = format!("二维码生成失败：{e}"),
                        }
                        self.login_state = Some(login::LoginState::Pending);
                        // 后台线程轮询，结果经 channel 回来，不卡 UI
                        let (tx, rx) = std::sync::mpsc::channel();
                        let env = locate.env_domain.clone();
                        let g = grant.clone();
                        std::thread::spawn(move || {
                            let st = login::poll_device_token(&env, &g, |_| {});
                            let _ = tx.send(st);
                        });
                        self.login_rx = Some(rx);
                        self.login_grant = Some(grant);
                    }
                    Err(e) => self.status = format!("申请失败：{e}"),
                }
            }
        } else {
            if let Some(tex) = &self.login_qr {
                ui.image((tex.id(), egui::vec2(200.0, 200.0)));
            }
            let state_text = match &self.login_state {
                Some(login::LoginState::Pending) => "等待扫码...".to_string(),
                Some(login::LoginState::WaitingConfirm) => "已扫码，请在手机上确认...".to_string(),
                Some(login::LoginState::Denied) => "已拒绝".to_string(),
                Some(login::LoginState::Expired) => "二维码已过期".to_string(),
                Some(login::LoginState::Failed(e)) => format!("失败：{e}"),
                Some(login::LoginState::Done(_)) => "登录成功".to_string(),
                None => String::new(),
            };
            ui.label(state_text);

            // 收后台轮询结果
            let mut done: Option<login::LoginState> = None;
            if let Some(rx) = &self.login_rx {
                if let Ok(st) = rx.try_recv() {
                    done = Some(st);
                }
            }
            if let Some(st) = done {
                if let login::LoginState::Done(info) = &st {
                    let _ = auth::write_user_info(&user_info_path, info);
                    self.status = "登录成功，凭证已落盘（可命名导入凭证库）".to_string();
                    self.login_grant = None;
                    self.login_qr = None;
                }
                self.login_state = Some(st);
                self.login_rx = None;
            }
            if ui.button("取消登录").clicked() {
                self.login_grant = None;
                self.login_qr = None;
                self.login_state = None;
                self.login_rx = None;
            }
        }
    }
}

// 把 PNG 解码为 RGBA 字节（egui ColorImage 用）
fn rgba_of(png: &[u8]) -> Vec<u8> {
    image::load_from_memory_with_format(png, image::ImageFormat::Png)
        .map(|i| i.to_rgba8().into_raw())
        .unwrap_or_default()
}
