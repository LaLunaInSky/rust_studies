use counter::states::Counter;

pub fn main() -> iced::Result {
    iced::run(
        Counter::update,
        Counter::view
    )
}