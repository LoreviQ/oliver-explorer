mod html;
mod networking;
mod state;
mod ui;

pub fn main() -> eframe::Result<()> {
    ui::start_browser()
}
