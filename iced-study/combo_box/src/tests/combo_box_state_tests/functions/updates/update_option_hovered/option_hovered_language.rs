use crate::{
    languages::Language,
    messages::combo_box_message::ComboBoxMessage,
    states::combo_box_state::ComboBoxState,
};

use std::collections::HashMap;

#[test]
fn language_danish() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Danish
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Halloy!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_english() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Danish
        )
    );

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::English
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Hello!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_french() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::French
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Salut!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_german() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::German
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Hallo!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_italian() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Italian
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Ciao!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_japanese() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Japanese
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "こんにちは!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_portuguese() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Portuguese
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "Olá!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_spanish() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Spanish
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "¡Hola!"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}

#[test]
fn language_other() {
    let mut combo_box_state = ComboBoxState::default();

    combo_box_state.update(
        ComboBoxMessage::OptionHovered(
            Language::Other
        )
    );

    let expectec_result: HashMap<&str, String> = HashMap::from([
        ("text", String::from(
            "... hello?"
        ))
    ]); 

    assert_eq!(
        combo_box_state.get_value_text(),
        expectec_result
    );
}