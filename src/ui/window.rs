// UI specific to window level changes

use eframe::egui;

use crate::state;

// Actions to be executed at window level
#[derive(Debug)]
pub enum WindowAction {
    NewTab,
    SelectTab(usize),
    CloseTab(usize),
    Search(usize),
    ToggleMaximize,
    DragWindow,
    CloseWindow,
}

// Add execute method to WindowAction
impl WindowAction {
    pub fn execute(self, window: &mut state::Window, ui: &mut egui::Ui) {
        match self {
            // Create a new tab
            WindowAction::NewTab => {
                window.new_tab();
            }
            // Select the tab with the given id
            WindowAction::SelectTab(tab_id) => {
                window.set_active_tab(tab_id);
            }
            // Close the tab with the given id
            WindowAction::CloseTab(tab_id) => {
                if let Err(e) = window.close_tab(tab_id) {
                    dbg!("Error closing tab: {}", e);
                };
            }
            // Execute a search on the tab with the given id
            WindowAction::Search(tab_id) => {
                if let Err(e) = window.search_tab(tab_id) {
                    dbg!("Error searching: {}", e);
                };
            }
            WindowAction::ToggleMaximize => {
                let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
            }
            WindowAction::DragWindow => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            WindowAction::CloseWindow => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}

// Tab bar component
impl state::Window {
    // Draws all window level elements and executes actions
    pub fn update(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        // Vertical layout without spacing
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

            // Draw title bar and handle action
            if let Some(action) = self.draw_title_bar(ui) {
                action.execute(self, ui);
            }

            // Draw search bar and handle action
            if let Some(action) = self.draw_search_bar(ui) {
                action.execute(self, ui);
            }

            // Draw content and handle action (if any)
            if let Some(action) = self.draw_content(ui) {
                action.execute(self, ui);
            }
        });
    }

    // Draws the title bar and returns the action to be executed
    pub fn draw_title_bar(&mut self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let mut action = None;
        let title_bar_rect = {
            let mut rect = ui.max_rect();
            rect.max.y = rect.min.y + self.settings.theme.frame.toolbar_height;
            rect
        };
        let title_bar_response = ui.interact(
            title_bar_rect,
            egui::Id::new("title_bar"),
            egui::Sense::click_and_drag(),
        );
        if title_bar_response.double_clicked() {
            action = Some(WindowAction::ToggleMaximize);
        }
        if title_bar_response.drag_started_by(egui::PointerButton::Primary) {
            action = Some(WindowAction::DragWindow);
        }

        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(title_bar_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = self.settings.theme.frame.padding;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);
                if let Some(content_action) = self.tab_bar_contents(ui) {
                    action = Some(content_action);
                }
            },
        );
        action
    }

    // Draws the contents of the tab bar
    fn tab_bar_contents(&mut self, ui: &mut egui::Ui) -> Option<WindowAction> {
        let mut action = None;
        action = close_button(ui).or(action);
        action = plus_button(ui).or(action);
        // Tab items
        let tab_width = self.calculate_tab_width(ui);
        for tab in &mut self.tabs {
            if let Some(tab_action) = tab.draw_tab(ui, tab_width) {
                action = Some(tab_action);
            }
        }
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
}

// Plus button component
fn plus_button(ui: &mut egui::Ui) -> Option<WindowAction> {
    let plus_response = ui
        .add_sized(
            [ui.available_size().y, ui.available_size().y],
            egui::Button::new("+"),
        )
        .on_hover_text("New tab");
    if plus_response.clicked() {
        return Some(WindowAction::NewTab);
    }
    None
}

// Close button component
fn close_button(ui: &mut egui::Ui) -> Option<WindowAction> {
    let close_response = ui
        .add_sized(
            [ui.available_size().y, ui.available_size().y],
            egui::Button::new("❌"),
        )
        .on_hover_text("Close the window");
    if close_response.clicked() {
        return Some(WindowAction::CloseWindow);
    }
    None
}
