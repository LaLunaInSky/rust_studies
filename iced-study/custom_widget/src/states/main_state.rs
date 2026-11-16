use iced::{
    Element,
    widget::{
        container,
        text,
        column,
    },
    Center,
    Fill
};

use crate::messages::custom_widget_message::CustomWidgetMessage;

pub struct MainState {}

impl MainState {
    pub fn view(
        element: Element<'_, CustomWidgetMessage>
    ) -> Element<'_, CustomWidgetMessage> {
        container(
            column![
                text("Custom Widget").size(30),
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