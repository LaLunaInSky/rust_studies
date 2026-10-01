use crate::{
    states::combo_box_state::ComboBoxState,
    languages::Language
};

use std::collections::HashMap;

#[test]
fn default_value_text() {
    let combo_box_state = ComboBoxState::default();

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(""))
    ]);

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn default_value_selected_language() {
    let combo_box_state = ComboBoxState::default();

    let expected_result: HashMap<&str, Option<Language>> = HashMap::from([
        ("selected_language", None)
    ]);

    assert_eq!(
        combo_box_state.get_value_selected_language(),
        expected_result
    );
}

#[test]
fn default_value_languages() {
    let combo_box_state = ComboBoxState::default();

    let expected_result: HashMap<&str, Vec<Language>> = HashMap::from([
        ("languages", Language::ALL.to_vec())
    ]);

    assert_eq!(
        combo_box_state.get_value_languages(),
        expected_result
    );
}