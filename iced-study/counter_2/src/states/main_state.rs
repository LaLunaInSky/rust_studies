use iced::{
    Element,
    widget::{
        text,
        column,
        container
    },
    Fill,
    Center
};

use crate::messages::counter_message::CounterMessage;

pub struct MainState {}

impl MainState {
    pub fn view(element: Element<'_, CounterMessage>) -> Element<'_, CounterMessage> {
        container(
            column![
                text("Counter").size(30),
                element
            ]
            .spacing(10)
            .align_x(Center)
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}