// UI specific to tab level changes

use eframe::egui;
use eframe::epaint::Margin;

use crate::state;
use crate::ui::components;
use crate::ui::window::WindowAction;

impl state::Tab {
    pub fn draw_tab(&self, ui: &mut egui::Ui, width: f32) -> WindowAction {
        let mut action = WindowAction::None;
        let tab_name = self.url.clone();

        // Get the background fill and stroke color for the tab
        let (bg_fill, stroke_color) = match self.is_active() {
            true => (
                self.settings.theme.accent.background,
                self.settings.theme.accent.text,
            ),
            false => (
                self.settings.theme.general.background,
                self.settings.theme.general.text,
            ),
        };

        // Create a frame for the tab with fixed width and padding
        let frame = egui::Frame::new()
            .fill(bg_fill)
            .inner_margin(Margin::symmetric(
                self.settings.theme.frame.padding as i8,
                self.settings.theme.frame.padding as i8,
            ));

        let label = egui::Label::new(
            egui::RichText::new(tab_name)
                .color(stroke_color)
                .size(self.settings.theme.frame.text_size),
        )
        .truncate();

        frame.show(ui, |ui| {
            ui.set_width(width - (self.settings.theme.frame.padding * 2.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                action = components::close_button(
                    ui,
                    egui::Vec2::new(16.0, 16.0),
                    WindowAction::CloseTab(self.id),
                );
                let label_response = ui
                    .with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.set_width(ui.available_width());
                        ui.add(label)
                    })
                    .inner;
                if label_response.clicked() {
                    action = WindowAction::SelectTab(self.id);
                }
            });
            if ui.interact_bg(egui::Sense::click()).clicked() {
                action = WindowAction::SelectTab(self.id);
            }
        });
        action
    }
}
