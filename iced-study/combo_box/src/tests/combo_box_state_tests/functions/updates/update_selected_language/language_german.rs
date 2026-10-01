use crate::{
    states::combo_box_state::ComboBoxState,
    messages::combo_box_message::ComboBoxMessage,
    languages::Language,
};

use std::collections::HashMap;

#[test]
fn check_the_value_of_selected_language() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::German
        )
    );

    let expected_result: HashMap<&str, Option<Language>> = HashMap::from([
        ("selected_language", Some(
            Language::German
        ))
    ]);

    assert_eq!(
        combo_box_state.get_value_selected_language(),
        expected_result
    );
}

#[test]
fn check_the_value_of_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::German
        )
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Hallo!"
        ))
    ]);

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}