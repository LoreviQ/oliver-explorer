mod config;

use crate::html;
use crate::networking;

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
    url: String,
    html_content: String,
    content: String,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        let url = config::DEFAULT_URL;
        let html_content = networking::fetch_url(&url).unwrap();
        let content = html::parse_html(&html_content).unwrap();
        Self {
            url: url.to_string(),
            html_content,
            content,
        }
    }
}

impl eframe::App for OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title bar section
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), config::TITLE_BAR_HEIGHT),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.heading(config::TITLE);
                    // Add any other title bar elements here
                },
            );

            // Content panel section
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    // Display the HTML content
                    ui.label(&self.content);
                    // Later you might want to use a proper HTML renderer here
                },
            );
        });
    }
}
