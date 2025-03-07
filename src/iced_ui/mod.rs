mod components;
mod config;
mod icons;
mod styles;

use components::title_bar_button;

use iced::widget::{column, container, row, text, Column};
use iced::{Length, Subscription, Task, Theme};

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
    window_id: iced::window::Id,
    title: String,
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    // Placeholder for future browser actions
    Close,
    DragWindow,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        Self::new()
    }
}

impl OliverExplorer {
    fn new() -> OliverExplorer {
        OliverExplorer {
            window_id: iced::window::Id::new(1),
            title: String::from("Oliver Browser"),
            content: String::from(
                "<html><body><h1>Hello, World!</h1><p>Welcome to Oliver Explorer</p></body></html>",
            ),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => {
                // This will exit the application
                std::process::exit(0);
            }
            Message::DragWindow => {
                // This will drag the window
                iced::window::drag(self.window_id)
            }
        }
    }

    fn view(&self) -> Column<Message> {
        // Title bar
        let title_bar_content = row![text(&self.title).size(16).width(Length::Fill)];
        let title_bar = title_bar_button(title_bar_content);

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
