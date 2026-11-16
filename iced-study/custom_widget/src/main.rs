use custom_widget::states::custom_widget_state::CustomWidgetState;

pub fn main() -> iced::Result {
    iced::application(
        CustomWidgetState::default,
        CustomWidgetState::update,
        CustomWidgetState::view,
    ).run()
}