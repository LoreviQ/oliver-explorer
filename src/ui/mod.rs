mod tab;
mod window;

use crate::state;
use eframe::egui;

pub fn start_browser() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        state::AppSettings::default().title.as_str(),
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<state::OliverExplorer>::default())
        }),
    )
}

impl eframe::App for state::OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab bar section
            self.windows[0].draw_tab_bar(ui);

            // Content panel section
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    if let Some(active_tab) = self.windows[0].tabs.get(self.windows[0].active_tab) {
                        // Display the HTML content of the active tab
                        ui.label(&active_tab.content);
                        // Later you might want to use a proper HTML renderer here
                    }
                },
            );
        });
    }
}
