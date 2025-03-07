mod egui_ui;
mod html;
mod networking;

pub fn main() -> eframe::Result<()> {
    egui_ui::start_browser()
}
