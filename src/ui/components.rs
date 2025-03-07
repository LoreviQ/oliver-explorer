use crate::ui::config;
use crate::ui::types;
use eframe::egui;

pub struct TabBar<'a> {
    pub active_tab: usize,
    pub tabs: &'a mut Vec<types::Tab>,
}

impl<'a> TabBar<'a> {
    pub fn new(active_tab: usize, tabs: &'a mut Vec<types::Tab>) -> Self {
        Self { active_tab, tabs }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> usize {
        ui.horizontal(|ui| {
            for (index, tab) in self.tabs.iter().enumerate() {
                let url = tab.get_url();
                let tab_name = if url.len() > 20 {
                    format!("{}...", &url[..20])
                } else {
                    url.clone()
                };

                if ui
                    .selectable_label(self.active_tab == index, tab_name)
                    .clicked()
                {
                    self.active_tab = index;
                }
            }

            if ui.button("+").clicked() {
                self.tabs.push(types::Tab::new(config::DEFAULT_URL));
                self.active_tab = self.tabs.len() - 1;
            }
        });

        self.active_tab
    }
}
