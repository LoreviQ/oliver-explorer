mod components;
mod tab;
mod window;

use crate::state;
use eframe::egui;

pub enum AppAction {
    CloseWindow,
    None,
    CloseApp,
}

impl eframe::App for state::OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_frame = egui::Frame::new()
            .fill(ctx.style().visuals.window_fill())
            .corner_radius(10)
            .stroke(ctx.style().visuals.widgets.noninteractive.fg_stroke)
            .outer_margin(1); // so the stroke is within the bounds

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                for window in &mut self.windows {
                    window.update(ctx, ui);
                }
            });
    }
}
