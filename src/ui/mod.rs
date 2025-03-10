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
            for window in &mut self.windows {
                // Tab bar section
                window.draw_tab_bar(ui);

                // Content panel section
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        let active_tab = window.get_active_tab().expect("No active tab found");
                        ui.label(&active_tab.content);
                        // TODO: Add a proper HTML renderer here
                    },
                );
            }
        });
    }
}
