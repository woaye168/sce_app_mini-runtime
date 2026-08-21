//! 调试标签页：方案 B 自托管启动（assign_host→上传→起局→拉起客户端），工作线程执行不阻塞 UI

use crate::core::debug::{DebugParams, DebugSession, DebugStatus};
use crate::App;
use std::path::PathBuf;

/// 启动结果（工作线程 → UI）
pub(crate) struct StartOutcome {
    pub result: std::result::Result<DebugSession, String>,
}

impl App {
    pub(crate) fn ui_debug(&mut self, ui: &mut egui::Ui) {
        ui.heading("脱机调试（方案 B · 自托管）");

        let Some(project) = self.project_root.clone() else {
            ui.label("请先在宿主中打开项目");
            return;
        };
        ui.label(format!("项目：{}", project.display()));

        // 凭证
        let active = self.cred_store.active_label.clone();
        match &active {
            Some(label) => { ui.label(format!("凭证：{label}")); }
            None => { ui.label("无激活凭证（先到「凭证」页导入/登录）"); }
        }

        // 参数区
        if self.debug_runtime_input.is_empty() {
            self.debug_runtime_input = std::env::current_exe()
                .map(|e| e.with_file_name("runtime").display().to_string())
                .unwrap_or_default();
        }
        ui.horizontal(|ui| {
            ui.label("载荷目录:");
            ui.text_edit_singleline(&mut self.debug_runtime_input);
        });
        ui.horizontal(|ui| {
            ui.label("暂存目录:");
            ui.text_edit_singleline(&mut self.debug_staging_input);
        });
        if self.debug_staging_input.is_empty() {
            ui.label("暂存留空 = 自动生成（白名单复制+入口包装）");
        }
        ui.horizontal(|ui| {
            ui.label("userid:");
            ui.text_edit_singleline(&mut self.debug_userid_input);
        });
        ui.add_space(6.0);

        // 启动结果回收
        if let Some(rx) = &self.debug_start_rx {
            if let Ok(outcome) = rx.try_recv() {
                match outcome.result {
                    Ok(session) => {
                        self.status = format!("调试局已起 session={} pid={}", session.session_id, session.pid());
                        self.debug_session = Some(session);
                        self.debug_status = Some(DebugStatus::Running);
                    }
                    Err(e) => self.status = format!("启动失败：{e}"),
                }
                self.debug_start_rx = None;
            }
        }

        if self.debug_session.is_none() {
            let busy = self.debug_start_rx.is_some();
            ui.add_enabled_ui(!busy, |ui| {
                if ui.button("启动调试").clicked() {
                    let Some(label) = active else {
                        self.status = "无激活凭证".into();
                        return;
                    };
                    let Some(cred) = self.cred_store.items.get(&label) else {
                        self.status = format!("凭证不存在: {label}");
                        return;
                    };
                    let userid: i64 = match self.debug_userid_input.trim().parse() {
                        Ok(v) => v,
                        Err(_) => {
                            self.status = "userid 必须是数字".into();
                            return;
                        }
                    };
                    let staging_text = self.debug_staging_input.trim().to_string();
                    let params = DebugParams {
                        project_root: project.clone(),
                        runtime_dir: PathBuf::from(self.debug_runtime_input.trim()),
                        staging_dir: if staging_text.is_empty() { None } else { Some(PathBuf::from(staging_text)) },
                        cred: cred.info.clone(),
                        userid,
                        env_domain: cred.env_domain.clone(),
                    };
                    let (tx, rx) = std::sync::mpsc::channel();
                    std::thread::spawn(move || {
                        let result = DebugSession::start(&params).map_err(|e| e.to_string());
                        let _ = tx.send(StartOutcome { result });
                    });
                    self.debug_start_rx = Some(rx);
                    self.status = "启动中（assign_host→上传→起局）...".into();
                }
            });
            if busy {
                ui.label("启动中（assign_host→上传→起局）...");
            }
        } else {
            if let Some(session) = &mut self.debug_session {
                let st = session.poll();
                self.debug_status = Some(st.clone());
                let text = match &st {
                    DebugStatus::Starting => "启动中...".to_string(),
                    DebugStatus::Running => format!("调试运行中（session={}）", session.session_id),
                    DebugStatus::Exited(code) => format!("已退出，退出码={code}"),
                    DebugStatus::Failed(e) => format!("失败：{e}"),
                };
                ui.label(text);
                // host 服务端日志尾部
                if let Some(ctl) = &session.ctl {
                    if !ctl.host_logs.is_empty() {
                        ui.separator();
                        ui.label("host 服务端日志：");
                        egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                            let start = ctl.host_logs.len().saturating_sub(50);
                            for line in &ctl.host_logs[start..] {
                                ui.monospace(line);
                            }
                        });
                    }
                }
                if matches!(st, DebugStatus::Exited(_) | DebugStatus::Failed(_)) {
                    self.debug_session = None;
                }
            }
            if ui.button("停止调试").clicked() {
                if let Some(session) = &mut self.debug_session {
                    session.stop();
                }
                self.debug_session = None;
                self.debug_status = None;
                self.status = "已停止".to_string();
            }
        }
    }
}
