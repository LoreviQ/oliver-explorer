// UI specific to window level changes

use eframe::egui;

use crate::state;
use crate::ui::tab::TabAction;

// Tab bar component
impl state::Window {
    // Show the tab bar returns the index of the active tab
    pub fn draw_tab_bar(&mut self, ui: &mut egui::Ui) {
        let original_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing.x = self.settings.theme.frame.spacing;

        ui.horizontal(|ui| {
            // Tab bar height
            ui.set_min_height(self.settings.theme.frame.tab.height);

            // Tab items
            let tab_width = self.calculate_tab_width(ui);
            for (index, tab) in self.tabs.iter().enumerate() {
                let action = tab.draw_tab(ui, index == self.active_tab, tab_width);

                // Handle the action
                match action {
                    Some(TabAction::Select) => {
                        self.active_tab = index;
                    }
                    Some(TabAction::Close) => {
                        self.close_tab(index);
                        // Break out of the loop since we modified the tabs collection
                        break;
                    }
                    None => {}
                }
            }

            // New tab button
            let plus_button = plus_button(ui);
            if plus_button.clicked() {
                self.new_tab();
            }
        });
        ui.spacing_mut().item_spacing = original_spacing;
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

    // Close a tab at the given index
    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() <= 1 {
            // If this is the last tab, request application to close
            std::process::exit(0);
        } else {
            self.tabs.remove(index);

            // Adjust active_tab if necessary
            if index <= self.active_tab {
                if index == self.active_tab {
                    // If we closed the active tab
                    if index == self.tabs.len() {
                        // If it was the last tab, activate the new last tab
                        self.active_tab = self.tabs.len() - 1;
                    }
                    // Otherwise active_tab stays the same (points to the next tab)
                } else {
                    // If we closed a tab before the active tab, decrement active_tab
                    self.active_tab -= 1;
                }
            }
        }
    }
}

// Plus button component
fn plus_button(ui: &mut egui::Ui) -> egui::Response {
    ui.add_sized(
        [ui.available_size().y, ui.available_size().y],
        egui::Button::new("+"),
    )
}
