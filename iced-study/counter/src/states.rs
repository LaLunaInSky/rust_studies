use iced::{
    widget::{
        button,
        text,
        column,
        container
    },
    Element,
    Fill,
};

use crate::messages::Message;

#[derive(Default)]
pub struct Counter {
    pub value: i64,
}

impl Counter {
    pub fn update(
        &mut self,
        message: Message
    ) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn view(
        &self
    ) -> Element<'_, Message> {
        container(
            column![
                button("+").on_press(
                    Message::Increment
                ),
                text(self.value),
                button("-").on_press(
                    Message::Decrement
                )
            ]
            .spacing(10)

        )
        .
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}