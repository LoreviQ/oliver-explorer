// UI specific to window level changes

use eframe::egui;

use crate::state;

// Actions to be executed at window level
#[derive(Debug)]
pub enum WindowAction {
    SelectTab(usize),
    CloseTab(usize),
    Search(usize),
}

// Tab bar component
impl state::Window {
    // Draws all window level elements and executes actions
    pub fn update(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        // Vertical layout without spacing
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            self.draw_and_execute(ui, |s, ui| s.draw_tab_bar(ui));
            self.draw_and_execute(ui, |s, ui| s.draw_search_bar(ui));
            self.draw_and_execute(ui, |s, ui| s.draw_content(ui));
        });
    }

    // Show the tab bar returns the index of the active tab
    pub fn draw_tab_bar(&mut self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let mut action = None;
        let frame = egui::Frame::new()
            .fill(self.settings.theme.general.background)
            .inner_margin(egui::Margin::symmetric(
                self.settings.theme.frame.padding as i8,
                self.settings.theme.frame.padding as i8,
            ));

        frame.show(ui, |ui| {
            // Fixed height for the tab bar
            ui.set_min_height(self.settings.theme.frame.toolbar_height);
            ui.set_max_height(self.settings.theme.frame.toolbar_height);
            ui.set_min_width(ui.available_width());

            // Horizontal layout of tabs and new tab button
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = self.settings.theme.frame.padding;

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
        });

        // Return the action
        action
    }

    // Calculate tab width based on available space
    fn calculate_tab_width(&self, ui: &mut egui::Ui) -> f32 {
        let tab_count = self.tabs.len() as f32;
        let available_width = ui.available_width();
        let plus_button_width = ui.available_size().y;
        let spacing_width = tab_count * self.settings.theme.frame.padding;
        let width_per_tab = (available_width - plus_button_width - spacing_width) / tab_count;
        width_per_tab.min(self.settings.theme.frame.tab.width.max)
    }

    // Draw the search bar
    fn draw_search_bar(&mut self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let mut action = None;
        egui::Frame::new()
            .fill(self.settings.theme.accent.background)
            .inner_margin(egui::Margin::ZERO)
            .show(ui, |ui| {
                // Fixed height for search bar
                ui.set_min_height(self.settings.theme.frame.toolbar_height);
                ui.set_max_height(self.settings.theme.frame.toolbar_height);
                ui.set_min_width(ui.available_width());

                // Full width container
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Calculate width for the centered search input
                    let available_width = ui.available_width();
                    let search_width = available_width * 0.6; // Use 60% of available width
                    let search_height = self.settings.theme.frame.toolbar_height
                        - self.settings.theme.frame.padding * 2.0;
                    let padding = (available_width - search_width) / 2.0;
                    let text_color = self.settings.theme.general.text;
                    let bg_color = self.settings.theme.general.background;
                    let active_tab = self.get_active_tab_mut().expect("No active tab found");

                    // layout elements
                    ui.add_space(padding);
                    let search = egui::TextEdit::singleline(&mut active_tab.search_buffer)
                        .hint_text("Search...")
                        .desired_width(search_width)
                        .text_color(text_color)
                        .background_color(bg_color);

                    let search_response = ui.add_sized([search_width, search_height], search);
                    ui.add_space(padding);

                    // Handle responses
                    // Check if Enter key was pressed while the search box is focused
                    if search_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        action = Some(WindowAction::Search(active_tab.id));
                    }
                });
            });
        action
    }

    // Draws the content of the active tab
    fn draw_content(&self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let action = None;

        // Use Frame for consistent styling
        egui::Frame::new()
            .fill(self.settings.theme.general.background)
            .inner_margin(egui::Margin::ZERO)
            .show(ui, |ui| {
                // Take all available remaining height
                ui.set_min_height(ui.available_height());
                ui.set_min_width(ui.available_width());

                // Set text color for content
                let text_color = self.settings.theme.general.text;
                ui.visuals_mut().override_text_color = Some(text_color);

                let active_tab = self.get_active_tab().expect("No active tab found");
                ui.label(&active_tab.content);
                // TODO: Add a proper HTML renderer here
            });

        action
    }

    // draws content from the draw_fn and executes any resulting actions
    fn draw_and_execute<F>(&mut self, ui: &mut egui::Ui, draw_fn: F)
    where
        F: FnOnce(&mut Self, &mut egui::Ui) -> Option<WindowAction>,
    {
        let Some(action) = draw_fn(self, ui) else {
            return;
        };

        match action {
            // Select the tab with the given id
            WindowAction::SelectTab(tab_id) => {
                self.set_active_tab(tab_id);
            }
            // Close the tab with the given id
            WindowAction::CloseTab(tab_id) => {
                if let Err(e) = self.close_tab(tab_id) {
                    eprintln!("Error closing tab: {}", e);
                }
            }
            // Execute a search on the tab with the given id
            WindowAction::Search(tab_id) => {
                let tab = match self.get_tab_mut(tab_id) {
                    Ok(tab) => tab,
                    Err(e) => {
                        eprintln!("Error getting tab: {}", e);
                        return;
                    }
                };
                if let Err(e) = tab.search() {
                    eprintln!("Error searching: {}", e);
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
