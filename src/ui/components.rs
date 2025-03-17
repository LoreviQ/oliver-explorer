// Reusable components used across the app

use crate::ui::window::WindowAction;
use eframe::egui;

pub struct ButtonParams {
    pub size: egui::Vec2,
    pub corner_radius: egui::CornerRadius,
    pub action: WindowAction,
}

impl Default for ButtonParams {
    fn default() -> Self {
        Self {
            size: egui::Vec2::new(16.0, 16.0),
            corner_radius: egui::CornerRadius {
                nw: 0,
                ne: 0,
                sw: 0,
                se: 0,
            },
            action: WindowAction::None,
        }
    }
}

// Close button component
pub fn close_button(ui: &mut egui::Ui, params: ButtonParams) -> WindowAction {
    let close_response = ui
        .add_sized(
            params.size,
            egui::Button::new("❌").corner_radius(params.corner_radius),
        )
        .on_hover_text("Close the window");
    if close_response.clicked() {
        return params.action;
    }
    WindowAction::None
}

// Plus button component
pub fn plus_button(ui: &mut egui::Ui, params: ButtonParams) -> WindowAction {
    let plus_response = ui
        .add_sized(
            params.size,
            egui::Button::new("+").corner_radius(params.corner_radius),
        )
        .on_hover_text("New tab");
    if plus_response.clicked() {
        return params.action;
    }

    WindowAction::None
}
