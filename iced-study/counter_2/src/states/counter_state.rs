use crate::{
    states::main_state::MainState,
    messages::counter_message::CounterMessage
};


use iced::{
    Element,
    widget::{
        button,
        text,
        row
    },
    Center
};

#[derive(Default)]
pub struct CounterState {
    value: i64
}

impl CounterState {
    pub fn update(
        &mut self,
        message: CounterMessage
    ) {
        match message {
            CounterMessage::Increment => {
                self.value += 1;
            }
            CounterMessage::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn get_value(
        &self
    ) -> i64 {
        self.value
    }

    pub fn view(
        &self
    ) -> Element<'_, CounterMessage> {
        MainState::view(
            row![
                button("+").on_press(
                    CounterMessage::Increment
                ),
                text(self.value)
                .size(30),
                button("-").on_press(
                    CounterMessage::Decrement
                )
            ]
            .spacing(10)
            .padding(10)
            .align_y(Center)
            .into()
        )
    }
}