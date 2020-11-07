use iced_raqote::{Color, Command, Element, Length, Program, Renderer, Space};

pub struct InteractiveTest {}

#[derive(Debug, Clone)]
pub enum Message {
    #[allow(dead_code)]
    NoOp,
}

impl InteractiveTest {
    pub fn new() -> InteractiveTest {
        InteractiveTest {}
    }

    pub fn background_color(&self) -> Color {
        Color::WHITE
    }
}

impl Program for InteractiveTest {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NoOp => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        Space::new(Length::Fill, Length::Fill).into()
    }
}
