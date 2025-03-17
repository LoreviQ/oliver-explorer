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
        for window in &mut self.windows {
            window.update(ctx);
        }
    }
}
