use iced::widget::{button, container, row, svg, text, Button, Container};
use iced::{Alignment, Center, Length};

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

pub fn title_bar_container() -> Container<'static, Message> {
    container(
        row![
            // Left side: Title with padding
            text("Oliver Explorer").size(16).width(Length::Fill),
            // Right side: Close button
            close_button()
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .align_y(Center),
    )
    .width(Length::Fill)
    .style(container::dark)
}
