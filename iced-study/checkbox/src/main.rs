use checkbox::states::checkbox_state::CheckboxState;

pub fn main() -> iced::Result {
    iced::application(
        CheckboxState::default,
        CheckboxState::update,
        CheckboxState::view
    )
    .font(
        include_bytes!(
            "./fonts/icons.ttf"
        )
        .as_slice()
    )
    .run()
}

