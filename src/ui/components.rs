// Reusable components used across the app

use crate::ui::window::WindowAction;
use eframe::egui;

pub struct ButtonParams {
    pub content: String,
    pub hover_text: String,
    pub size: egui::Vec2,
    pub corner_radius: egui::CornerRadius,
    pub action: WindowAction,
    pub stroke: egui::Stroke,
    pub bg_fill: egui::Color32,
}

impl Default for ButtonParams {
    fn default() -> Self {
        Self {
            content: "".to_string(),
            hover_text: "".to_string(),
            size: egui::Vec2::new(16.0, 16.0),
            corner_radius: egui::CornerRadius {
                nw: 0,
                ne: 0,
                sw: 0,
                se: 0,
            },
            action: WindowAction::None,
            stroke: egui::Stroke::new(0.0, egui::Color32::TRANSPARENT),
            bg_fill: egui::Color32::TRANSPARENT,
        }
    }
}

// Close button component
pub fn button(ui: &mut egui::Ui, params: ButtonParams) -> WindowAction {
    let original_bg_fill = ui.visuals().widgets.inactive.weak_bg_fill;
    ui.visuals_mut().widgets.inactive.weak_bg_fill = params.bg_fill;
    let response = ui
        .add_sized(
            params.size,
            egui::Button::new(params.content)
                .corner_radius(params.corner_radius)
                .stroke(params.stroke),
        )
        .on_hover_text(params.hover_text);
    ui.visuals_mut().widgets.inactive.weak_bg_fill = original_bg_fill;
    if response.clicked() {
        return params.action;
    }
    WindowAction::None
}
