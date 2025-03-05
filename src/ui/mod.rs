use iced::widget::{button, column, container, row, text, Column};
use iced::{Length, Theme};

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
        }
    }

    fn view(&self) -> Column<Message> {
        // Title bar
        let title_bar = row![
            text(&self.title).size(24),
            button("Reload").on_press(Message::Reload),
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill);

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
