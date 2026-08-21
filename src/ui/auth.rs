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
            ui.label(format!("凭证文件：{}", crate::core::disp(&user_info_path)));

        // ---- 后台任务回收（验证/刷新登录态都在工作线程，不卡 UI）----
        if let Some(rx) = &self.verify_rx {
            if let Ok((label, result)) = rx.try_recv() {
                self.verify_result = match result {
                    // 注意按字符截断：按字节切 UTF-8 会切到多字节字符中间 panic（0.3.0 闪退根因）
                    Ok(text) => format!("{label} 验证通过：{}", text.chars().take(120).collect::<String>()),
                    Err(e) => format!("{label} 验证失败：{e}"),
                };
                self.verify_rx = None;
            }
        }
        let mut refreshed: Option<(String, i64, Option<String>)> = None;
        if let Some(rx) = &self.refresh_rx {
            if let Ok((label, result)) = rx.try_recv() {
                match result {
                    Ok((uid, name)) => refreshed = Some((label, uid, name)),
                    Err(e) => self.verify_result = format!("{label} 登录态获取失败：{e}"),
                }
                self.refresh_rx = None;
            }
        }
        if let Some((label, uid, name)) = refreshed {
            match self.cred_store.update_identity(&label, uid, name.clone()) {
                Ok(_) => {
                    self.verify_result = format!(
                        "{label} 登录态已刷新：userid={uid} 昵称={}",
                        name.unwrap_or_else(|| "（无）".into())
                    );
                    // 顺手填到调试页（输入框为空时），免得手抄
                    if self.debug_userid_input.trim().is_empty() {
                        self.debug_userid_input = uid.to_string();
                    }
                }
                Err(e) => self.verify_result = format!("写回凭证库失败：{e}"),
            }
        }

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
                        if ui.button("导入编辑器凭证").clicked() {
                            // 名称为空时按 {userid}_{时间}_{env}_{token_type} 自动命名，重名加序号
                            let base = if self.cred_new_label.trim().is_empty() {
                                auth::make_label(&info, &locate.env_domain)
                            } else {
                                self.cred_new_label.trim().to_string()
                            };
                            let mut label = base.clone();
                            let mut n = 2;
                            while self.cred_store.items.contains_key(&label) {
                                label = format!("{base}-{n}");
                                n += 1;
                            }
                            // 只读复制一份进凭证库，不写回——编辑器原凭证不动，不会踢下线
                            self.cred_store
                                .harvest(&label, &locate.env_domain, info.clone());
                            // 新导入的凭证直接设为当前激活（调试页立即可用）
                            self.cred_store.active_label = Some(label.clone());
                            let _ = self.cred_store.save();
                            self.status = if info.can_sign() {
                                format!("已导入并激活凭证：{label}（编辑器原凭证未动）")
                            } else {
                                format!("已导入并激活凭证：{label}（注意：无 HTTP 签名对，调试会失败——请确认编辑器已完整登录后重新导入）")
                            };
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
        ui.label("凭证库（点击条目选中为当前凭证）：");
        let labels: Vec<String> = self.cred_store.items.keys().cloned().collect();
        for label in labels {
            let cred = self.cred_store.items[&label].clone();
            let active = self.cred_store.active_label.as_deref() == Some(label.as_str());
            // 两行布局：第一行凭证信息（点击选中），第二行操作按钮——避免长名字把按钮挤出屏幕
            ui.group(|ui| {
                let uid_text = cred
                    .info
                    .userid
                    .map(|u| u.to_string())
                    .unwrap_or_else(|| "无userid".into());
                let sign_text = if cred.info.can_sign() { "签名对✓" } else { "签名对✗" };
                if ui
                    .selectable_label(
                        active,
                        format!(
                            "{}  [{}]  {}({})  uid={}  {}",
                            label, cred.env_domain, cred.info.token_type,
                            cred.info.token_type_name(), uid_text, sign_text
                        ),
                    )
                    .clicked()
                {
                    self.cred_store.active_label = Some(label.clone());
                    let _ = self.cred_store.save();
                    self.status = format!("当前凭证：{label}");
                }
                ui.horizontal(|ui| {
                    if ui.button("回写编辑器").clicked() {
                        match self.cred_store.apply(&label, &user_info_path) {
                            Ok(_) => self.status = format!("已回写编辑器并设为当前：{label}"),
                            Err(e) => self.status = format!("回写失败：{e}"),
                        }
                    }
                    let busy = self.verify_rx.is_some() || self.refresh_rx.is_some();
                    ui.add_enabled_ui(!busy, |ui| {
                        if ui.button("验证").clicked() {
                            self.status = format!("验证中：{label}...");
                            let (tx, rx) = std::sync::mpsc::channel();
                            let info = cred.info.clone();
                            let env = cred.env_domain.clone();
                            let label2 = label.clone();
                            std::thread::spawn(move || {
                                let r = verify::verify(&info, &env).map_err(|e| e.to_string());
                                let _ = tx.send((label2, r));
                            });
                            self.verify_rx = Some(rx);
                        }
                        if ui.button("刷新登录态").clicked() {
                            self.status = format!("刷新登录态：{label}（起脱机客户端真实登录，约 1 分钟）...");
                            let (tx, rx) = std::sync::mpsc::channel();
                            let info = cred.info.clone();
                            let env = cred.env_domain.clone();
                            let label2 = label.clone();
                            let runtime_dir = std::env::current_exe()
                                .map(|e| e.with_file_name("runtime"))
                                .unwrap_or_else(|_| std::path::PathBuf::from("runtime"));
                            std::thread::spawn(move || {
                                let r = crate::core::login_state::fetch_identity(
                                    &runtime_dir,
                                    &info,
                                    &env,
                                    std::time::Duration::from_secs(120),
                                )
                                .map_err(|e| e.to_string())
                                .and_then(|id| {
                                    id.userid_i64()
                                        .map(|uid| (uid, id.user_name_opt()))
                                        .ok_or_else(|| "登录响应无 userid".to_string())
                                });
                                let _ = tx.send((label2, r));
                            });
                            self.refresh_rx = Some(rx);
                        }
                    });
                    if ui.button("删除").clicked() {
                        // store.remove：删的是当前激活项时自动切到剩余第一条
                        self.cred_store.remove(&label);
                        self.status = format!("已删除：{label}");
                    }
                });
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
                                // 注意：min_dimensions 是最小尺寸，实际渲染可能 >200，
                                // 尺寸必须与像素缓冲一致，硬编码 200x200 会 assert 崩溃
                                match color_image_of(&png) {
                                    Some(img) => {
                                        self.login_qr = Some(ui.ctx().load_texture(
                                            "login_qr",
                                            img,
                                            egui::TextureOptions::LINEAR,
                                        ));
                                    }
                                    None => self.status = "二维码解码失败".into(),
                                }
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

// 把 PNG 解码为 egui ColorImage（尺寸取图片真实宽高，防 assert 崩溃）
fn color_image_of(png: &[u8]) -> Option<egui::ColorImage> {
    let img = image::load_from_memory_with_format(png, image::ImageFormat::Png).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = (rgba.width() as usize, rgba.height() as usize);
    Some(egui::ColorImage::from_rgba_unmultiplied([w, h], &rgba.into_raw()))
}
