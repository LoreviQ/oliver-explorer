use crate::ui::config::WIDGET_SIZE;

use iced::border::Radius;
use iced::widget::button;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

pub fn rounded_danger_style(theme: &Theme, status: button::Status) -> button::Style {
    let mut default = button::danger(theme, status);
    default.border.radius = Radius::new(WIDGET_SIZE / 2.0);
    default
}

pub fn title_bar_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(Color {
            r: 0x1C as f32 / 255.0,
            g: 0x1B as f32 / 255.0,
            b: 0x22 as f32 / 255.0,
            a: 1.0,
        })),
        text_color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        border: Border {
            color: Color {
                r: 0x1C as f32 / 255.0,
                g: 0x1B as f32 / 255.0,
                b: 0x22 as f32 / 255.0,
                a: 1.0,
            },
            width: 0.0,
            radius: Radius::new(0).top(5.0),
        },
        shadow: Shadow {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}
