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

impl state::OliverExplorer {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = Self::default();
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx
            .set_style_of(egui::Theme::Dark, state::default_style());
        state
    }
}

impl eframe::App for state::OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_frame = egui::Frame::new()
            .fill(ctx.style().visuals.window_fill())
            .corner_radius(egui::CornerRadius {
                nw: 10,
                ne: 10,
                sw: 0,
                se: 0,
            })
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
