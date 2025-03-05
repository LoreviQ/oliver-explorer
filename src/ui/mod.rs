mod styles;
use styles::rounded_danger_style;

use iced::widget::{button, column, container, row, text, Column};
use iced::{Center, Length, Theme};

pub fn start_browser() -> iced::Result {
    iced::application(
        "Oliver Explorer",
        OliverExplorer::update,
        OliverExplorer::view,
    )
    .window(iced::window::Settings {
        decorations: false,
        ..Default::default()
    })
    .theme(|_| Theme::Dark)
    .run()
}

struct OliverExplorer {
    title: String,
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    // Placeholder for future browser actions
    Reload,
    Close,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        Self::new()
    }
}

impl OliverExplorer {
    fn new() -> OliverExplorer {
        OliverExplorer {
            title: String::from("Oliver Browser"),
            content: String::from(
                "<html><body><h1>Hello, World!</h1><p>Welcome to Oliver Explorer</p></body></html>",
            ),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Reload => {
                // For now, this doesn't do anything
            }
            Message::Close => {
                // This will exit the application
                std::process::exit(0);
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // Title bar
        let title_bar = container(
            row![
                // Left side: Title with padding
                text(&self.title).size(16).width(Length::Fill),
                // Right side: Close button
                button(text("X").size(8))
                    .on_press(Message::Close)
                    .width(Length::Fixed(20.0))
                    .height(Length::Fixed(20.0))
                    .style(rounded_danger_style)
            ]
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .align_y(Center),
        )
        .width(Length::Fill)
        .style(container::dark);

        // Content panel (just showing raw HTML for now)
        let content_panel = container(text(&self.content).width(Length::Fill))
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill);

        column![title_bar, content_panel,]
            .spacing(1)
            .width(Length::Fill)
            .height(Length::Fill)
    }
}
