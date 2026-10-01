use iced::{
    widget::{
        combo_box,
        ComboBox,
        Container,
        container,
        column,
        Column,
        text,
        space,
        scrollable,
        Scrollable,
    },
    Fill,
    Center,
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

    fn view_combo_box_input(
        &self
    ) -> ComboBox<'_, Language, ComboBoxMessage> {
        combo_box(
            &self.languages,
            "Type a language...",
            self.selected_language.as_ref(),
            ComboBoxMessage::Selected
        )
        .on_option_hovered(
            ComboBoxMessage::OptionHovered
        )
        .on_close(
            ComboBoxMessage::Closed
        )
        .menu_height(200)
        .width(250)
    }

    fn view_combo_box_content(
        &self
    ) -> Column<'_, ComboBoxMessage> {
        column![
            text(
                &self.text
            )
            .size(30),
            space()
            .height(180),
            "What is your language?",
            self.view_combo_box_input(),
        ]
        .width(Fill)
        .align_x(Center)
        .spacing(10)
    }

    fn view_combo_box_scrollable(
        &self
    ) -> Scrollable<'_, ComboBoxMessage> {
        scrollable(
            self.view_combo_box_content()
        )
    }

    pub fn view(
        &self
    ) -> Container<'_, ComboBoxMessage> {
        container(
            self.view_combo_box_scrollable()
        )
        .padding(20)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

impl Default for ComboBoxState {
    fn default() -> Self {
        ComboBoxState::new()
    }
}