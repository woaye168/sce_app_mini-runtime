//! 脱机运行时（sce_app_mini-runtime）：基于 bgd_appsdk 的标准应用骨架
//!
//! 本文件为入口聚合：应用状态 + ShellApp 壳实现（ui_tab 只做分发）；
//! 标签页 UI 分散在 src/ui/ 各页面文件（impl App）。
//!
//! 公共逻辑（CLI 分发 --quit/notify、单实例、看守线程、--background、项目解析、窗口壳）
//! 由 bgd_appsdk::app::run 全托管——业务只需实现 ShellApp（标签页渲染）。

// Windows 下不弹出黑色控制台窗口
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod ui;

use std::path::PathBuf;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = "脱机运行时";

fn main() -> eframe::Result<()> {
    bgd_appsdk::app::run(
        bgd_appsdk::app::AppOptions {
            app_name: APP_NAME,
            inner_size: [720.0, 560.0],
            min_size: [600.0, 480.0],
            // 单实例/信号前缀一律由 appsdk 按 exe 名推导，禁止硬编码
            si_prefix: None,
            is_valid_project: Some(|p| p.join(".bgd").is_dir()),
            app: App::default(),
        },
        APP_VERSION,
    )
}

/// 应用状态（壳只负责框架，业务状态都放这里）
#[derive(Default)]
struct App {
    /// 当前项目根（on_project_changed 回调维护）
    project_root: Option<PathBuf>,
    /// 状态栏文本
    status: String,
}

const TABS: &[bgd_appsdk::ui::ShellTab] = &[
    bgd_appsdk::ui::ShellTab { id: "main", label: "主页" },
    bgd_appsdk::ui::ShellTab { id: "settings", label: "设置" },
];

impl bgd_appsdk::ui::ShellApp for App {
    fn app_title(&self) -> &'static str {
        APP_NAME
    }

    fn tabs(&self) -> &[bgd_appsdk::ui::ShellTab] {
        TABS
    }

    fn ui_tab(&mut self, ui: &mut egui::Ui, tab: &str) {
        match tab {
            "main" => self.ui_main(ui),
            "settings" => self.ui_settings(ui),
            _ => {}
        }
    }

    fn on_project_changed(&mut self, project: Option<&std::path::Path>) {
        self.project_root = project.map(|p| p.to_path_buf());
        if let Some(p) = project {
            self.status = format!("当前项目: {}", p.display());
        }
    }

    fn status_text(&self) -> String {
        self.status.clone()
    }
}
