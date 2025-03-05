use iced::widget::{button, column, text, Column};

pub fn start_browser() -> iced::Result {
    iced::application(
        "Oliver Explorer",
        OliverExplorer::update,
        OliverExplorer::view,
    )
    .run()
}

struct OliverExplorer {
    counter: u64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Default for OliverExplorer {
    fn default() -> Self {
        Self::new()
    }
}

impl OliverExplorer {
    fn new() -> OliverExplorer {
        OliverExplorer { counter: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
        }
    }

    fn view(&self) -> Column<Message> {
        column![text(self.counter), button("+").on_press(Message::Increment),]
    }
}
