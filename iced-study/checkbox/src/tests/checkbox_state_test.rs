use crate::{
    states::checkbox_state::CheckboxState,
    messages::checkbox_message::CheckboxMessage
};

use std::collections::HashMap;

#[test]
fn checkbox_state_default_values() {
    let checkbox_state = CheckboxState::default();

    let expected_result = HashMap::from([
        ("default", false),
        ("custom", false),
        ("styled", false)
    ]);

    assert_eq!(
        checkbox_state.get_values(),
        expected_result
    )
}

#[test]
fn checkbox_state_update_default_value_for_true() {
    let mut checkbox_state = CheckboxState::default();

    checkbox_state.update(
        CheckboxMessage::DefaultToggled(
            true
        )
    );

    let expected_result = HashMap::from([
        ("default", true),
        ("custom", false),
        ("styled", false)
    ]);

    assert_eq!(
        checkbox_state.get_values(),
        expected_result
    )
}

#[test]
fn checkbox_state_update_custom_value_for_true() {
    let mut checkbox_state = CheckboxState::default();

    checkbox_state.update(
        CheckboxMessage::CustomToggled(
            true
        )
    );

    let expected_result = HashMap::from([
        ("default", false),
        ("custom", true),
        ("styled", false)
    ]);

    assert_eq!(
        checkbox_state.get_values(),
        expected_result
    )
}

#[test]
fn checkbox_state_update_styled_value_for_true() {
    let mut checkbox_state = CheckboxState::default();

    checkbox_state.update(
        CheckboxMessage::Styledtoggled(
            true
        )
    );

    let expected_result = HashMap::from([
        ("default", false),
        ("custom", false),
        ("styled", true)
    ]);

    assert_eq!(
        checkbox_state.get_values(),
        expected_result
    )
}