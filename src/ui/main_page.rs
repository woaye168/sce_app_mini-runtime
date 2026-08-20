//! 主页标签页

use crate::App;

impl App {
    pub(crate) fn ui_main(&mut self, ui: &mut egui::Ui) {
        ui.heading("脱机运行时");
        ui.label("基于 bgd_appsdk 的标准应用骨架。在这里实现你的功能。");
        ui.add_space(8.0);
        if ui.button("点我").clicked() {
            self.status = "按钮被点击了".to_string();
        }
    }
}
