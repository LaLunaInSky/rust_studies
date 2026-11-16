use crate::{
    messages::custom_widget_message::CustomWidgetMessage,
    states::{
        main_state::MainState,
        circle_state::circle_state
    },
};

use iced::{
    Element,
    widget::{
        column,
        text,
        slider,
    },
    Center
};
pub struct CustomWidgetState {
    radius: f32,
}

impl CustomWidgetState {
    pub fn new() -> Self {
        CustomWidgetState {
            radius: 50.0
        }
    }

    pub fn update(
        &mut self,
        message: CustomWidgetMessage
    ) {
        match message {
            CustomWidgetMessage::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    pub fn get_radius(
        &self
    ) -> f32 {
        self.radius
    }

    pub fn view(
        &self
    ) -> Element<'_, CustomWidgetMessage> {
        MainState::view(
            column![
                circle_state(self.radius),
                text!(
                    "Radius: {:.2}",
                    self.radius
                )
                .width(110),
                slider(
                    1.0..=100.0,
                    self.radius,
                    CustomWidgetMessage::RadiusChanged
                ).step(0.01_f32),
            ]
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_x(Center)
            .into()
        )
    }
}

impl Default for CustomWidgetState {
    fn default() -> Self {
        CustomWidgetState::new()
    }
}