use combo_box::states::combo_box_state::ComboBoxState;

pub fn main() -> iced::Result {
    iced::application(
        ComboBoxState::default,
        ComboBoxState::update,
        ComboBoxState::view
    )
    .run()
}