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
            // Tab bar height
            ui.set_min_height(config::TAB_BAR_HEIGHT);

            // Tab items
            for (index, tab) in self.tabs.iter().enumerate() {
                let tab_item = TabItem::new(tab, self.active_tab == index).show(ui);
                if tab_item.clicked() {
                    self.active_tab = index;
                }
            }

            // New t button
            let plus_button = plus_button(ui);
            if plus_button.clicked() {
                self.tabs.push(types::Tab::new(config::DEFAULT_URL));
                self.active_tab = self.tabs.len() - 1;
            }
        });

        self.active_tab
    }
}

pub struct TabItem<'a> {
    pub tab: &'a types::Tab,
    pub is_active: bool,
}

impl<'a> TabItem<'a> {
    pub fn new(tab: &'a types::Tab, is_active: bool) -> Self {
        Self { tab, is_active }
    }

    pub fn show(&self, ui: &mut egui::Ui) -> egui::Response {
        let url = self.tab.get_url();
        let tab_name = if url.len() > 20 {
            format!("{}...", &url[..20])
        } else {
            url.clone()
        };

        ui.add_sized(
            [0.0, ui.available_size().y],
            egui::SelectableLabel::new(self.is_active, tab_name),
        )
    }
}

fn plus_button(ui: &mut egui::Ui) -> egui::Response {
    ui.add_sized(
        [ui.available_size().y, ui.available_size().y],
        egui::Button::new("+"),
    )
}
