mod tab;
mod window;

use crate::state;
use eframe::egui;

impl eframe::App for state::OliverExplorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show(ctx, |ui| {
                for window in &mut self.windows {
                    window.update(ctx, ui);
                }
            });
    }
}
