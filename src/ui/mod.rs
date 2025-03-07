mod components;
mod config;
mod theme;
mod types;

use eframe::egui;

pub fn start_browser() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        config::TITLE,
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<OliverExplorer>::default())
        }),
    )
}

struct OliverExplorer {
    tabs: Vec<types::Tab>,
    active_tab: usize,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        let default_tab = types::Tab::new(config::DEFAULT_URL);
        Self {
            tabs: vec![default_tab],
            active_tab: 0,
        }
    }
}

impl eframe::App for OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab bar section
            self.active_tab = {
                let mut tab_bar = components::TabBar::new(self.active_tab, &mut self.tabs);
                tab_bar.show(ui)
            };

            // Content panel section
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    if let Some(active_tab) = self.tabs.get(self.active_tab) {
                        // Display the HTML content of the active tab
                        ui.label(&active_tab.content);
                        // Later you might want to use a proper HTML renderer here
                    }
                },
            );
        });
    }
}
