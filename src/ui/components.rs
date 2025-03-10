use eframe::egui;
use eframe::epaint::Margin;

use crate::state;

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

// Define an enum for tab actions
pub enum TabAction {
    Select,
    Close,
}

impl state::Tab {
    pub fn draw_tab(&self, ui: &mut egui::Ui, is_active: bool, width: f32) -> Option<TabAction> {
        let mut action = None;

        let tab_name = self.url.clone();

        // Get the background fill and stroke color for the tab
        let (bg_fill, stroke_color) = if is_active {
            (
                self.settings.theme.active.background,
                self.settings.theme.active.text,
            )
        } else {
            (
                self.settings.theme.inactive.background,
                self.settings.theme.inactive.text,
            )
        };

        // Create a frame for the tab with fixed width and padding
        let frame = egui::Frame::new()
            .fill(bg_fill)
            .inner_margin(Margin::symmetric(
                self.settings.theme.frame.padding as i8,
                self.settings.theme.frame.padding as i8,
            ));

        let response = frame
            .show(ui, |ui| {
                ui.set_width(width - (self.settings.theme.frame.padding * 2.0));

                // Use horizontal layout to place label and close button side by side
                ui.horizontal(|ui| {
                    // Tab label with truncation - make it take most of the space
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(tab_name)
                                .color(stroke_color)
                                .size(self.settings.theme.frame.text_size),
                        )
                        .truncate(),
                    );

                    // Add flexible space to push the close button to the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Close (X) button with transparent background
                        let close_button = egui::Button::new(
                            egui::RichText::new("×")
                                .color(stroke_color)
                                .size(self.settings.theme.frame.text_size),
                        )
                        .frame(false); // Remove button background/frame

                        let response = ui.add_sized([16.0, 16.0], close_button);

                        // Add hover effect - show background when hovered
                        if response.hovered() {
                            let hover_rect = response.rect;
                            ui.painter().rect_filled(
                                hover_rect,
                                4.0, // Rounded corners radius
                                self.settings.theme.general.hover,
                            );
                            // Redraw the text on top of the background
                            ui.painter().text(
                                hover_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "×",
                                egui::FontId::proportional(self.settings.theme.frame.text_size),
                                stroke_color,
                            );
                        }

                        if response.clicked() {
                            action = Some(TabAction::Close);
                        }
                    });
                })
            })
            .response;

        // If the tab was clicked and we haven't already set an action, set it to Select
        if response.clicked() && matches!(action, None) {
            action = Some(TabAction::Select);
        }

        action
    }
}

// Plus button component
fn plus_button(ui: &mut egui::Ui) -> egui::Response {
    ui.add_sized(
        [ui.available_size().y, ui.available_size().y],
        egui::Button::new("+"),
    )
}
