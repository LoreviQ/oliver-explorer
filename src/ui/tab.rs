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

        let close_button = egui::Button::new(
            egui::RichText::new("×")
                .color(stroke_color)
                .size(self.settings.theme.frame.text_size),
        )
        .frame(false);

        frame.show(ui, |ui| {
            ui.set_width(width - (self.settings.theme.frame.padding * 2.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let close_response = ui.add_sized([16.0, 16.0], close_button);
                let label_response = ui
                    .with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.set_width(ui.available_width());
                        ui.add(label)
                    })
                    .inner;

                // Handle close response
                if close_response.hovered() {
                    let hover_rect = close_response.rect;
                    ui.painter().rect_filled(
                        hover_rect,
                        4.0, // Rounded corners radius
                        self.settings.theme.general.hover,
                    );
                    ui.painter().text(
                        hover_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "×",
                        egui::FontId::proportional(self.settings.theme.frame.text_size),
                        stroke_color,
                    );
                }
                if close_response.clicked() {
                    dbg!("Closing tab: {}", self.id);
                    action = Some(WindowAction::CloseTab(self.id));
                }
                if label_response.clicked() {
                    action = Some(WindowAction::SelectTab(self.id));
                }
            });
            if ui.interact_bg(egui::Sense::click()).clicked() {
                action = Some(WindowAction::SelectTab(self.id));
            }
        });
        action
    }
}
