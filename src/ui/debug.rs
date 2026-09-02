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
        ui.label(format!("项目：{}", crate::core::disp(&project)));

        // 凭证
        let active = self.cred_store.active_label.clone();
        match &active {
            Some(label) => { ui.label(format!("凭证：{label}")); }
            None => { ui.label("无激活凭证（先到「凭证」页导入/登录）"); }
        }

        // 运行时选择（编辑器-api 为默认；对战平台测试/正式为既有 scegame 链路）
        let project_api = crate::core::debug::read_map_settings(&project)
            .map(|(_, api)| api)
            .unwrap_or(13);
        let kind_options = [
            format!("星火编辑器-{project_api} 运行时（默认）"),
            "星火对战平台 测试环境运行时".to_string(),
            "星火对战平台 正式环境运行时".to_string(),
        ];
        let selected_kind = match self.debug_kind_sel {
            1 => Some(crate::core::runtimes::RuntimeKind::TesterTest),
            2 => Some(crate::core::runtimes::RuntimeKind::TesterProd),
            _ => None, // 默认编辑器-api
        };
        egui::ComboBox::from_label("运行时")
            .selected_text(&kind_options[self.debug_kind_sel.min(2)])
            .show_ui(ui, |ui| {
                for (i, name) in kind_options.iter().enumerate() {
                    ui.selectable_value(&mut self.debug_kind_sel, i, name);
                }
            });

        // host 模式三态（0.5.0）：云端直连（默认）/ 本地中继（观测抓包）/ 真本地（脱机 lua 服务端）
        // 注意：0.4.x 的「本地」是中继，0.5.0 起 local=真本地、中继由 relay 承接（语义切换）
        let host_options = [
            "云端 host（直连，默认）",
            "本地中继 host（观测抓包，127.0.0.1:5003 → 云端）",
            "真本地 host（完全脱机，内嵌 lua 服务端）",
        ];
        egui::ComboBox::from_label("host 模式")
            .selected_text(host_options[self.debug_host_sel.min(2)])
            .show_ui(ui, |ui| {
                for (i, name) in host_options.iter().enumerate() {
                    ui.selectable_value(&mut self.debug_host_sel, i, *name);
                }
            });

        // 附加客户端多开（0.5.0）：按凭证库顺序取（排除当前凭证），每个须有登录态
        let extra_options = ["不加（单客户端）", "+1（双人）", "+2（三人）", "+3（四人）"];
        egui::ComboBox::from_label("附加客户端")
            .selected_text(extra_options[self.debug_extra_clients_sel.min(3)])
            .show_ui(ui, |ui| {
                for (i, name) in extra_options.iter().enumerate() {
                    ui.selectable_value(&mut self.debug_extra_clients_sel, i, *name);
                }
            });

        // 参数区
        if self.debug_runtime_input.is_empty() {
            self.debug_runtime_input = std::env::current_exe()
                .map(|e| crate::core::disp(&e.with_file_name("runtime")))
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
        // userid 自动填充：输入框空且激活凭证已有登录态时自动带入（可手改覆盖）
        if self.debug_userid_input.trim().is_empty() {
            if let Some(label) = &active {
                if let Some(cred) = self.cred_store.items.get(label) {
                    if let Some(uid) = cred.info.userid {
                        self.debug_userid_input = uid.to_string();
                    }
                }
            }
        }
        ui.horizontal(|ui| {
            ui.label("userid:");
            ui.text_edit_singleline(&mut self.debug_userid_input);
            if self.debug_userid_input.trim().is_empty() {
                ui.label("（留空=取凭证登录态；无则到「凭证」页点「刷新登录态」）");
            }
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
                self.debug_progress_rx = None;
            }
        }
        // payload sync 进度展示
        if let Some(rx) = &self.debug_progress_rx {
            let mut last = None;
            while let Ok(msg) = rx.try_recv() {
                last = Some(msg);
            }
            if let Some(msg) = last {
                self.status = msg;
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
                    let host_mode = match self.debug_host_sel {
                        1 => crate::core::debug::HostMode::Relay,
                        2 => crate::core::debug::HostMode::Local,
                        _ => crate::core::debug::HostMode::Cloud,
                    };
                    // 前置校验：缺 HTTP 签名对时 assign_host 必失败，提前给明确引导（真本地不联云端，免检）
                    if host_mode != crate::core::debug::HostMode::Local && !cred.info.can_sign() {
                        self.status = "凭证缺 login_token/secret——请到「凭证」页重新「导入编辑器凭证」覆盖，或点击换一个凭证".into();
                        return;
                    }
                    // userid：输入框优先，空则取凭证登录态
                    let userid: i64 = match self.debug_userid_input.trim().parse() {
                        Ok(v) => v,
                        Err(_) => match cred.info.userid {
                            Some(v) => v,
                            None => {
                                self.status = "userid 为空且凭证无登录态——先到「凭证」页点「刷新登录态」".into();
                                return;
                            }
                        },
                    };
                    let staging_text = self.debug_staging_input.trim().to_string();
                    let kind = selected_kind;
                    let env_domain = kind
                        .map(|k| k.env_domain().to_string())
                        .unwrap_or_else(|| cred.env_domain.clone());
                    let params = DebugParams {
                        project_root: project.clone(),
                        runtime_dir: PathBuf::from(self.debug_runtime_input.trim()),
                        staging_dir: if staging_text.is_empty() { None } else { Some(PathBuf::from(staging_text)) },
                        cred: cred.info.clone(),
                        userid,
                        env_domain,
                        runtime_kind: kind,
                        host_mode,
                        extra_clients: {
                            // 多开：按凭证库顺序取（排除当前凭证），须有登录态 userid
                            let mut v = Vec::new();
                            for (l, c) in &self.cred_store.items {
                                if v.len() >= self.debug_extra_clients_sel {
                                    break;
                                }
                                if *l == label {
                                    continue;
                                }
                                if let Some(uid2) = c.info.userid {
                                    v.push((c.info.clone(), uid2));
                                }
                            }
                            v
                        },
                    };
                    let (tx, rx) = std::sync::mpsc::channel();
                    let (prog_tx, prog_rx) = std::sync::mpsc::channel::<String>();
                    std::thread::spawn(move || {
                        // 引擎未就绪时自动 payload sync（用户报过的「运行时没下载」痛点）
                        let kind_eff = params
                            .runtime_kind
                            .unwrap_or(crate::core::runtimes::RuntimeKind::EditorApi(project_api));
                        if !kind_eff.engine_ready(&params.runtime_dir) {
                            let _ = prog_tx.send(format!(
                                "运行时 {} 未就绪，自动下载载荷中（首次约 150MB）...",
                                kind_eff.display_name()
                            ));
                            let project_libs = std::fs::read_to_string(params.project_root.join("libs.json"))
                                .ok()
                                .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
                                .and_then(|v| v.as_object().map(|o| o.keys().cloned().collect()))
                                .unwrap_or_else(Vec::new);
                            let sync_params = crate::core::payload::SyncParams {
                                runtime_dir: params.runtime_dir.clone(),
                                env_domain: params.env_domain.clone(),
                                api_version: kind_eff.api_version(project_api),
                                project_libs,
                                project_root: Some(params.project_root.clone()),
                                dry_run: false,
                                runtime_kind: Some(kind_eff),
                            };
                            let prog_tx2 = prog_tx.clone();
                            let mut log = move |msg: String| {
                                let _ = prog_tx2.send(format!("[载荷] {msg}"));
                            };
                            if let Err(e) = crate::core::payload::sync(&sync_params, &mut log) {
                                let _ = tx.send(StartOutcome { result: Err(format!("载荷同步失败: {e}")) });
                                return;
                            }
                            let _ = prog_tx.send("载荷同步完成，启动调试中...".to_string());
                        }
                        let result = DebugSession::start(&params).map_err(|e| e.to_string());
                        let _ = tx.send(StartOutcome { result });
                    });
                    self.debug_start_rx = Some(rx);
                    self.debug_progress_rx = Some(prog_rx);
                    self.status = "启动中（assign_host→上传→起局）...".into();
                }
            });
            if busy {
                ui.label("启动中（见状态栏进度）...");
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
