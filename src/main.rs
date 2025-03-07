mod html;
mod networking;
mod ui;

pub fn main() -> eframe::Result<()> {
    ui::start_browser()
}
