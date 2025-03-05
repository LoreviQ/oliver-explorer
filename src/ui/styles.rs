use iced::border::Radius;
use iced::widget::button;
use iced::Theme;

pub fn rounded_danger_style(theme: &Theme, status: button::Status) -> button::Style {
    let mut default = button::danger(theme, status);
    default.border.radius = Radius::new(10.0);
    default
}
