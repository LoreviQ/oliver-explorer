// UI specific to window level changes

use eframe::egui;

use crate::state;

// Actions to be executed at window level
#[derive(Debug)]
pub enum WindowAction {
    SelectTab(usize),
    CloseTab(usize),
}

// Tab bar component
impl state::Window {
    // Draws all window level elements and executes actions
    pub fn update(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        // Draw content and collect actions
        let mut action = None;
        action = self.draw_tab_bar(ui).or(action);
        action = self.draw_search_bar(ui).or(action);
        action = self.draw_content(ui).or(action);

        // Execute actions
        if let Some(action) = action {
            self.execute_action(action);
        }
    }
    // Show the tab bar returns the index of the active tab
    pub fn draw_tab_bar(&mut self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let mut action = None;

        let original_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing.x = self.settings.theme.frame.spacing;

        // Apply general background color to tab bar
        let tab_bar_rect = egui::Rect::from_min_size(
            ui.cursor().min,
            egui::vec2(ui.available_width(), self.settings.theme.frame.tab.height),
        );
        ui.painter().rect_filled(
            tab_bar_rect,
            0.0, // No rounding
            self.settings.theme.general.background,
        );

        ui.horizontal(|ui| {
            // Tab bar height
            ui.set_min_height(self.settings.theme.frame.tab.height);

            // Tab items
            let tab_width = self.calculate_tab_width(ui);
            for tab in &mut self.tabs {
                if let Some(tab_action) = tab.draw_tab(ui, tab_width) {
                    action = Some(tab_action);
                }
            }

            // New tab button
            let plus_button = plus_button(ui);
            if plus_button.clicked() {
                self.new_tab();
            }
        });
        ui.spacing_mut().item_spacing = original_spacing;

        // Return the action
        action
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

    // Draw the search bar
    fn draw_search_bar(&mut self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let action = None;
        let theme = self.settings.theme.clone();
        let active_tab = self.get_active_tab_mut().expect("No active tab found");

        // Apply accent background color to search bar area
        let search_bar_height = 30.0;
        let search_bar_rect = egui::Rect::from_min_size(
            ui.cursor().min,
            egui::vec2(ui.available_width(), search_bar_height),
        );
        ui.painter().rect_filled(
            search_bar_rect,
            0.0, // No rounding
            theme.accent.background,
        );

        // Full width container
        ui.horizontal(|ui| {
            ui.set_min_height(search_bar_height);

            // Calculate width for the centered search input
            let available_width = ui.available_width();
            let search_width = available_width * 0.6; // Use 60% of available width
            let padding = (available_width - search_width) / 2.0;

            // Add left padding
            ui.add_space(padding);

            // Add the search text field with rounded corners
            let text_edit = egui::TextEdit::singleline(&mut active_tab.search_buffer)
                .hint_text("Search...")
                .desired_width(search_width)
                .text_color(theme.general.text)
                .background_color(theme.general.background);

            let _ = ui.add(text_edit);

            // Add right padding to complete the layout
            ui.add_space(padding);
        });

        action
    }

    // Draws the content of the active tab
    fn draw_content(&self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let action = None;

        // Apply general background color to content area
        let content_rect = egui::Rect::from_min_size(
            ui.cursor().min,
            egui::vec2(ui.available_width(), ui.available_height()),
        );
        ui.painter().rect_filled(
            content_rect,
            0.0, // No rounding
            self.settings.theme.general.background,
        );

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), ui.available_height()),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                // Set text color for content
                let text_color = self.settings.theme.general.text;
                ui.visuals_mut().override_text_color = Some(text_color);

                let active_tab = self.get_active_tab().expect("No active tab found");
                ui.label(&active_tab.content);
                // TODO: Add a proper HTML renderer here
            },
        );
        action
    }

    // Executes the action at the window level
    fn execute_action(&mut self, action: WindowAction) {
        match action {
            WindowAction::SelectTab(tab_id) => {
                self.set_active_tab(tab_id);
            }
            WindowAction::CloseTab(tab_id) => {
                if let Err(e) = self.close_tab(tab_id) {
                    eprintln!("Error closing tab: {}", e);
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
