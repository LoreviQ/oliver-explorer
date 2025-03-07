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

// New Tab struct to hold tab-specific data
struct Tab {
    url: String,
    html_content: String,
    content: String,
}

impl Tab {
    fn new(url: &str) -> Self {
        let html_content = networking::fetch_url(url).unwrap_or_default();
        let content = html::parse_html(&html_content).unwrap_or_default();
        Self {
            url: url.to_string(),
            html_content,
            content,
        }
    }
}

struct OliverExplorer {
    tabs: Vec<Tab>,
    active_tab: usize,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        let default_tab = Tab::new(config::DEFAULT_URL);
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
            ui.horizontal(|ui| {
                for (index, tab) in self.tabs.iter().enumerate() {
                    let tab_name = if tab.url.len() > 20 {
                        format!("{}...", &tab.url[..20])
                    } else {
                        tab.url.clone()
                    };

                    if ui
                        .selectable_label(self.active_tab == index, tab_name)
                        .clicked()
                    {
                        self.active_tab = index;
                    }
                }

                if ui.button("+").clicked() {
                    self.tabs.push(Tab::new(config::DEFAULT_URL));
                    self.active_tab = self.tabs.len() - 1;
                }
            });

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
