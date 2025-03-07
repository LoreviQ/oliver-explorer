use crate::ui::config;
use crate::ui::theme;
use crate::ui::types;

use eframe::egui;
use eframe::epaint::Margin;

// Tab bar component
pub struct TabBar<'a> {
    pub active_tab: usize,
    pub tabs: &'a mut Vec<types::Tab>,
}

impl<'a> TabBar<'a> {
    // Create a new tab bar
    pub fn new(active_tab: usize, tabs: &'a mut Vec<types::Tab>) -> Self {
        Self { active_tab, tabs }
    }

    // Show the tab bar returns the index of the active tab
    pub fn show(&mut self, ui: &mut egui::Ui) -> usize {
        let original_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing.x = config::TAB_SPACING;

        ui.horizontal(|ui| {
            // Tab bar height
            ui.set_min_height(config::TAB_BAR_HEIGHT);

            // Tab items
            let tab_width = self.calculate_tab_width(ui);
            for (index, tab) in self.tabs.iter().enumerate() {
                let action = TabItem::new(tab, self.active_tab == index, tab_width).show(ui);

                // Handle the action
                match action {
                    TabAction::Select => {
                        self.active_tab = index;
                    }
                    TabAction::Close => {
                        self.close_tab(index);
                        // Break out of the loop since we modified the tabs collection
                        break;
                    }
                    TabAction::None => {}
                }
            }

            // New tab button
            let plus_button = plus_button(ui);
            if plus_button.clicked() {
                self.tabs.push(types::Tab::new(config::DEFAULT_URL));
                self.active_tab = self.tabs.len() - 1;
            }
        });
        ui.spacing_mut().item_spacing = original_spacing;

        self.active_tab
    }

    // Calculate tab width based on available space
    fn calculate_tab_width(&self, ui: &mut egui::Ui) -> f32 {
        let tab_count = self.tabs.len() as f32;
        let available_width = ui.available_width();
        let plus_button_width = ui.available_size().y;
        let spacing_width = tab_count * config::TAB_SPACING;
        let width_per_tab = (available_width - plus_button_width - spacing_width) / tab_count;
        width_per_tab.min(config::MAX_TAB_WIDTH)
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
    None,
    Select,
    Close,
}

// Tab item component
pub struct TabItem<'a> {
    pub tab: &'a types::Tab,
    pub is_active: bool,
    pub width: f32,
}

impl<'a> TabItem<'a> {
    // Create a new tab item
    pub fn new(tab: &'a types::Tab, is_active: bool, width: f32) -> Self {
        Self {
            tab,
            is_active,
            width,
        }
    }

    // Show the tab item and return the action
    pub fn show(&self, ui: &mut egui::Ui) -> TabAction {
        let mut action = TabAction::None;
        let url = self.tab.get_url();
        let tab_name = url.clone();

        // Get the theme
        let theme = theme::get_theme();

        // Get the background fill and stroke color for the tab
        let (bg_fill, stroke_color) = if self.is_active {
            (theme.active.background, theme.active.text)
        } else {
            (theme.inactive.background, theme.inactive.text)
        };

        // Create a frame for the tab with fixed width and padding
        let frame = egui::Frame::new()
            .fill(bg_fill)
            .inner_margin(Margin::symmetric(config::TAB_PADDING, config::TAB_PADDING));

        let response = frame
            .show(ui, |ui| {
                ui.set_width(self.width - (config::TAB_PADDING as f32 * 2.0));

                // Use horizontal layout to place label and close button side by side
                ui.horizontal(|ui| {
                    // Tab label with truncation - make it take most of the space
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(tab_name)
                                .color(stroke_color)
                                .size(config::TEXT_SIZE),
                        )
                        .truncate(),
                    );

                    // Add flexible space to push the close button to the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Close (X) button with transparent background
                        let close_button = egui::Button::new(
                            egui::RichText::new("×")
                                .color(stroke_color)
                                .size(config::TEXT_SIZE),
                        )
                        .frame(false); // Remove button background/frame

                        let response = ui.add_sized([16.0, 16.0], close_button);

                        // Add hover effect - show background when hovered
                        if response.hovered() {
                            let hover_rect = response.rect;
                            ui.painter().rect_filled(
                                hover_rect,
                                4.0, // Rounded corners radius
                                theme.general.hover,
                            );
                            // Redraw the text on top of the background
                            ui.painter().text(
                                hover_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "×",
                                egui::FontId::proportional(config::TEXT_SIZE),
                                stroke_color,
                            );
                        }

                        if response.clicked() {
                            action = TabAction::Close;
                        }
                    });
                })
            })
            .response;

        // If the tab was clicked and we haven't already set an action, set it to Select
        if response.clicked() && matches!(action, TabAction::None) {
            action = TabAction::Select;
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
