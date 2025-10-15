use iced::Length::Fill;
use iced::widget::{button, column, text};
use iced::{Element, Task};

pub struct App;

#[derive(Debug, Clone)]
pub enum Message {
    Update,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (App, Task::none())
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Hello, Iced!").size(50),
            button("Click Me").on_press(Message::Update)
        ]
        .padding(20)
        .width(Fill)
        .height(Fill)
        .into()
    }
}
