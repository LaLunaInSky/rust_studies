use crate::{
    languages::Language,
    states::combo_box_state::ComboBoxState,
    messages::combo_box_message::ComboBoxMessage,
};

use std::collections::HashMap;

#[test]
fn closed_with_default_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            ""
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_danish_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::Danish
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Halloy!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_english_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::English
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Hello!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_french_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::French
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Salut!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_german_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::German
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
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

#[test]
fn closed_with_language_italian_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::Italian
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Ciao!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_japanese_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::Japanese
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "こんにちは!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_portuguese_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::Portuguese
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Olá!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_spanish_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::Spanish
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "¡Hola!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}

#[test]
fn closed_with_language_other_the_value_of_the_text() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::Selected(
            Language::Other
        )
    );

    combo_box_state.update(
        ComboBoxMessage::Closed
    );

    let expected_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "... hello?"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expected_result
    );
}