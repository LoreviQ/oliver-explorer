// UI specific to tab level changes

use eframe::egui;
use eframe::epaint::Margin;

use crate::state;
use crate::ui::window::WindowAction;

impl state::Tab {
    pub fn draw_tab(&self, ui: &mut egui::Ui, width: f32) -> Option<WindowAction> {
        let mut action = None;

        let tab_name = self.url.clone();

        // Get the background fill and stroke color for the tab
        let (bg_fill, stroke_color) = if self.is_active() {
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
                let content_response = ui
                    .horizontal(|ui| {
                        // Tab label with truncation - make it take most of the space
                        let label_response = ui.add(
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
                                action = Some(WindowAction::CloseTab(self.id));
                            }
                        });

                        // Return the label response so we can check if it was clicked
                        label_response
                    })
                    .inner;

                // If the label was clicked, set the action to SelectTab
                if content_response.clicked() {
                    action = Some(WindowAction::SelectTab(self.id));
                }
            })
            .response;

        // If the tab was clicked and we haven't already set an action, set it to Select
        if response.clicked() && matches!(action, None) {
            action = Some(WindowAction::SelectTab(self.id));
        }
        action
    }
}
