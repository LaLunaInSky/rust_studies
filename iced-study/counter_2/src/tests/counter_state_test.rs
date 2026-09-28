use crate::{
    states::counter_state::CounterState,
    messages::counter_message::CounterMessage
};

#[test]
fn counter_state_default_value() {
    let counter_state = CounterState::default();

    assert_eq!(
        counter_state.get_value(),
        0
    );
}

#[test]
fn counter_state_update_increment_value() {
    let mut counter_state = CounterState::default();

    counter_state.update(
        CounterMessage::Increment
    );

    assert_eq!(
        counter_state.get_value(),
        1
    );
}

#[test]
fn counter_state_update_decrement_value() {
    let mut counter_state = CounterState::default();

    counter_state.update(
        CounterMessage::Decrement
    );

    assert_eq!(
        counter_state.get_value(),
        -1
    );
}