// UI specific to tab level changes

use eframe::egui;

use crate::state;
use crate::ui::components;
use crate::ui::window::WindowAction;

impl state::Tab {
    pub fn draw_tab(&self, ui: &mut egui::Ui, width: f32, first: bool) -> WindowAction {
        let mut action = WindowAction::None;

        // Get the background fill for the tab
        let bg_fill = match self.is_active() {
            true => ui.visuals().widgets.active.bg_fill,
            false => ui.visuals().widgets.inactive.bg_fill,
        };
        let corner_radius = match first {
            true => egui::CornerRadius {
                nw: 10,
                ne: 0,
                sw: 0,
                se: 0,
            },
            false => egui::CornerRadius::default(),
        };

        // Create a frame for the tab with fixed width and padding
        let outer_frame = egui::Frame::new()
            .fill(bg_fill)
            .corner_radius(corner_radius);
        let inner_frame = egui::Frame::new().inner_margin(ui.spacing().window_margin);

        let label = egui::Label::new(egui::RichText::new(&self.url)).truncate();

        outer_frame.show(ui, |ui| {
            ui.set_width(width);
            inner_frame.show(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    action = components::button(
                        ui,
                        components::ButtonParams {
                            content: "❌".to_string(),
                            hover_text: "Close the tab".to_string(),
                            action: WindowAction::CloseTab(self.id),
                            size: egui::Vec2::new(24.0, 24.0),
                            corner_radius: egui::CornerRadius {
                                nw: 12,
                                ne: 12,
                                sw: 12,
                                se: 12,
                            },
                            ..Default::default()
                        },
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
            });
            if ui.interact_bg(egui::Sense::click()).clicked() {
                action = WindowAction::SelectTab(self.id);
            }
        });
        action
    }
}
