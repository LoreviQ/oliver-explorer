use iced::widget::{button, container, svg, Button};
use iced::{Alignment, Length};

use crate::ui::config::WIDGET_SIZE;
use crate::ui::icons;
use crate::ui::styles::rounded_danger_style;
use crate::ui::Message;

/// Creates a standardized close button that sends the Close message when pressed
pub fn close_button() -> Button<'static, Message> {
    button(
        container(
            svg(icons::x_icon())
                .width(Length::Fixed(WIDGET_SIZE * 0.7))
                .height(Length::Fixed(WIDGET_SIZE * 0.7)),
        )
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    )
    .on_press(Message::Close)
    .width(Length::Fixed(WIDGET_SIZE))
    .height(Length::Fixed(WIDGET_SIZE))
    .style(rounded_danger_style)
}
