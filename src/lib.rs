pub mod html;
pub mod networking;
pub mod state;
pub mod ui;

// Start the browser
pub fn start_browser() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        state::AppSettings::default().title.as_str(),
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<state::OliverExplorer>::default())
        }),
    )
}
