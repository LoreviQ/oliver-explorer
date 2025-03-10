// UI specific to window level changes

use eframe::egui;

use crate::state;
use crate::ui::tab::TabAction;

// Tab bar component
impl state::Window {
    // Show the tab bar returns the index of the active tab
    pub fn draw_tab_bar(&mut self, ui: &mut egui::Ui) {
        let mut action_and_tab = None;

        let original_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing.x = self.settings.theme.frame.spacing;
        ui.horizontal(|ui| {
            // Tab bar height
            ui.set_min_height(self.settings.theme.frame.tab.height);

            // Tab items
            let tab_width = self.calculate_tab_width(ui);
            for tab in &mut self.tabs {
                let action = tab.draw_tab(ui, tab_width);
                if let Some(action) = action {
                    action_and_tab = Some((action, tab.id));
                }
            }

            // New tab button
            let plus_button = plus_button(ui);
            if plus_button.clicked() {
                self.new_tab();
            }
        });
        ui.spacing_mut().item_spacing = original_spacing;

        // Handle window level action
        if let Some((action, tab_id)) = action_and_tab {
            match action {
                TabAction::Select => {
                    self.set_active_tab(tab_id);
                }
                TabAction::Close => {
                    if let Err(e) = self.close_tab(tab_id) {
                        eprintln!("Error closing tab: {}", e);
                    }
                }
            }
        }
    }

    // Calculate tab width based on available space
    fn calculate_tab_width(&self, ui: &mut egui::Ui) -> f32 {
        let tab_count = self.tabs.len() as f32;
        let available_width = ui.available_width();
        let plus_button_width = ui.available_size().y;
        let spacing_width = tab_count * self.settings.theme.frame.spacing;
        let width_per_tab = (available_width - plus_button_width - spacing_width) / tab_count;
        width_per_tab.min(self.settings.theme.frame.tab.width.max)
    }
}

// Plus button component
fn plus_button(ui: &mut egui::Ui) -> egui::Response {
    ui.add_sized(
        [ui.available_size().y, ui.available_size().y],
        egui::Button::new("+"),
    )
}
