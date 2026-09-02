//! 「本地服务器」标签页（0.5.0 R5）：真本地 host + 本地账号库（SQLite）。
//! 账号创建/删除；每账号「启动」拉起独立客户端（多开多人同局）；局未起时首个启动自动上传起局。

use crate::core::local_accounts::LocalAccount;
use std::collections::HashMap;
use std::path::PathBuf;

impl crate::App {
    pub(crate) fn ui_local_server(&mut self, ui: &mut egui::Ui) {
        // 状态刷新：账号列表（首次/有变更标记时重载）
        if self.ls_need_reload {
            self.ls_accounts = crate::core::local_accounts::list().unwrap_or_default();
            self.ls_need_reload = false;
        }

        // host / 局状态
        let host_running = crate::core::game_host::control_state().is_some();
        let game = crate::core::local_play::game_active();
        ui.horizontal(|ui| {
            ui.label(if host_running { "host：运行中（127.0.0.1:5003）" } else { "host：未运行（首个「启动」自动拉起）" });
        });
        ui.horizontal(|ui| {
            ui.label(match &game {
                Some(g) => format!("局：{g}"),
                None => "局：未起（首个「启动」自动上传起局）".to_string(),
            });
            if game.is_some() && ui.button("停止本局").clicked() {
                crate::core::local_play::stop_game();
                // 停局后 host 可接下一局（客户端各自断开）
            }
        });
        ui.separator();

        // 创建账号
        ui.horizontal(|ui| {
            ui.label("账号名:");
            ui.text_edit_singleline(&mut self.ls_new_name);
            if ui.button("创建").clicked() {
                match crate::core::local_accounts::create(&self.ls_new_name) {
                    Ok(acc) => {
                        self.status = format!("已创建账号 {}（userid={}）", acc.name, acc.userid);
                        self.ls_new_name.clear();
                        self.ls_need_reload = true;
                    }
                    Err(e) => self.status = format!("创建失败：{e}"),
                }
            }
        });
        ui.separator();

        // 账号列表
        let accounts: Vec<LocalAccount> = self.ls_accounts.clone();
        if accounts.is_empty() {
            ui.label("暂无账号——先创建一个（本地服务器账号与官方凭证无关，纯本地直通）");
        }
        let mut launch_acc: Option<LocalAccount> = None;
        let mut remove_id: Option<i64> = None;
        egui::Grid::new("local_accounts_grid").num_columns(4).show(ui, |ui| {
            for acc in &accounts {
                ui.label(&acc.name);
                ui.label(format!("userid={}", acc.userid));
                let running = self.ls_clients.get(&acc.id).copied().unwrap_or(0);
                let alive = running != 0 && process_alive(running);
                if running != 0 && !alive {
                    self.ls_clients.remove(&acc.id);
                }
                if alive {
                    ui.label(format!("运行中 pid={running}"));
                    if ui.button("停止").clicked() {
                        kill_pid(running);
                        self.ls_clients.remove(&acc.id);
                    }
                } else if ui.button("启动").clicked() {
                    launch_acc = Some(acc.clone());
                }
                if ui.button("删除").clicked() {
                    remove_id = Some(acc.id);
                }
                ui.end_row();
            }
        });
        if let Some(id) = remove_id {
            if let Err(e) = crate::core::local_accounts::remove(id) {
                self.status = format!("删除失败：{e}");
            }
            self.ls_need_reload = true;
        }

        // 启动（工作线程：上传起局可能耗时）
        if let Some(acc) = launch_acc {
            let busy = self.ls_launch_rx.is_some();
            if !busy {
                let Some(project) = self.project_root.clone() else {
                    self.status = "无项目（宿主启动需带 --project-path）".into();
                    return;
                };
                let runtime_dir = PathBuf::from(self.debug_runtime_input.trim());
                let runtime_dir = if runtime_dir.as_os_str().is_empty() {
                    std::env::current_exe()
                        .map(|e| e.with_file_name("runtime"))
                        .unwrap_or_else(|_| PathBuf::from("runtime"))
                } else {
                    runtime_dir
                };
                let env_domain = "editor-pd.spark.xd.com".to_string();
                let (tx, rx) = std::sync::mpsc::channel::<Result<(i64, u32), String>>();
                let acc_id = acc.id;
                let acc_name = acc.name.clone();
                std::thread::spawn(move || {
                    let r = crate::core::local_play::launch(
                        &crate::core::local_play::LocalPlayParams {
                            project_root: project,
                            runtime_dir,
                            env_domain,
                            account: acc,
                        },
                        &mut |msg| println!("[local-server] {msg}"),
                    )
                    .map(|pid| (acc_id, pid))
                    .map_err(|e| e.to_string());
                    let _ = tx.send(r);
                });
                self.ls_launch_rx = Some(rx);
                self.status = format!("启动中（{acc_name}）...");
            }
        }
        // 收启动结果
        if let Some(rx) = &self.ls_launch_rx {
            if let Ok(r) = rx.try_recv() {
                match r {
                    Ok((acc_id, pid)) => {
                        self.ls_clients.insert(acc_id, pid);
                        self.status = format!("客户端已拉起 pid={pid}");
                    }
                    Err(e) => self.status = format!("启动失败：{e}"),
                }
                self.ls_launch_rx = None;
            }
        }
        if self.ls_launch_rx.is_some() {
            ui.label("启动中（首启动含上传起局，约几十秒）...");
        }
    }
}

/// 进程存活检查（tasklist 查 pid）
fn process_alive(pid: u32) -> bool {
    crate::core::silent_command("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/NH"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
        .unwrap_or(false)
}

fn kill_pid(pid: u32) {
    let _ = crate::core::silent_command("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .status();
}

/// 账号 id → 客户端 pid 表类型别名（main.rs App 字段用）
pub type ClientMap = HashMap<i64, u32>;
