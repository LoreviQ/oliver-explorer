// Reusable components used across the app

use crate::ui::window::WindowAction;
use eframe::egui;

// Close button component
pub fn close_button(
    ui: &mut egui::Ui,
    size: egui::Vec2,
    action: WindowAction,
) -> Option<WindowAction> {
    let close_response = ui
        .add_sized(size, egui::Button::new("❌"))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        return Some(action);
    }
    None
}

/*
// Hover effect
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
*/
