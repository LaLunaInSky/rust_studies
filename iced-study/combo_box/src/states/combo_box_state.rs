use iced::{
    widget::{
        combo_box,
        Container,
        container,
        column,
        text,
    },
    Fill,
};

use std::collections::HashMap;

use crate::{
    languages::Language,
    messages::combo_box_message::ComboBoxMessage,
};

pub struct ComboBoxState {
    languages: combo_box::State<Language>,
    selected_language: Option<Language>,
    text: String,
}

impl ComboBoxState {
    fn new() -> Self {
        Self {
            languages: combo_box::State::new(
                Language::ALL.to_vec()
            ),
            selected_language: None,
            text: String::new(),
        }
    }

    pub fn get_value_languages(
        &self
    ) -> HashMap<&str, Vec<Language>> {
        HashMap::from([
            ("languages", self.languages.clone().into_options())
        ])
    }

    pub fn get_value_selected_language(
        &self
    ) -> HashMap<&str, Option<Language>> {
        HashMap::from([
            ("selected_language", self.selected_language)
        ])
    }

    pub fn get_value_text(
        &self
    ) -> HashMap<&str, String>{
        HashMap::from([
            ("text", self.text.clone())
        ])
    }

    pub fn update(
        &mut self,
        message: ComboBoxMessage
    ) {
        match message {
            ComboBoxMessage::Selected(language) => {
                self.selected_language = Some(language);

                self.text = language.greeting().to_string();

                println!(
                    "Selected_language: {:#?}\nText: {}",
                    self.selected_language,
                    self.text
                );
            }
            ComboBoxMessage::OptionHovered(language) => {
                self.text = language.greeting().to_string();

                println!(
                    "Text: {}",
                    self.text
                );
            }
            ComboBoxMessage::Closed => {
                self.text = self.selected_language.map(
                    |language| language.greeting().to_string()
                ).unwrap_or_default();

                println!(
                    "Text: {}",
                    self.text
                );
            }
        }
    }

    pub fn view(
        &self
    ) -> Container<'_, ComboBoxMessage> {
        container(
            column![
                text("x")
            ]
        )
        .padding(20)
        .center_x(Fill)
        .into()
    }
}

impl Default for ComboBoxState {
    fn default() -> Self {
        ComboBoxState::new()
    }
}