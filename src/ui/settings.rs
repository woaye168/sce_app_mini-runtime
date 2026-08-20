//! 设置标签页

use crate::App;

impl App {
    pub(crate) fn ui_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("设置");
        ui.label("在这里实现设置项。");
        if let Some(p) = &self.project_root {
            ui.add_space(8.0);
            ui.label(format!("当前项目：{}", p.display()));
        }
    }
}
