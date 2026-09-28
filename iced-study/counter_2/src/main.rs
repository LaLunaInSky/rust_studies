use counter_2::states::counter_state::CounterState;

pub fn main() -> iced::Result {
    iced::application(
        CounterState::default,
        CounterState::update,
        CounterState::view
    )
    .run()
}