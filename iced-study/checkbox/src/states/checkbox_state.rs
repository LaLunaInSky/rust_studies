use std::collections::HashMap;

use crate::{
    messages::checkbox_message::CheckboxMessage,
    fonts::icon_font::ICON_FONT
};

use iced::{
    Element,
    widget::{
        checkbox,
        Checkbox,
        column,
        Column,
        container,
        row,
        Row,
        text
    },
    Fill,
    Alignment
};

const TITLE_FONT_SIZE: u32 = 30;

#[derive(Default, Debug)]
pub struct CheckboxState {
    default: bool,
    custom: bool,
    styled: bool
}

impl CheckboxState {
    pub fn get_values(
        &self
    ) -> HashMap<&str, bool> {
        HashMap::from([
            ("default", self.default),
            ("custom", self.custom),
            ("styled", self.styled)
        ])
    }

    pub fn update(
        &mut self,
        message: CheckboxMessage
    ) {
        match message {
            CheckboxMessage::DefaultToggled(default) => {
                self.default = default;

                print!(
                    "checkbox_input_default: "
                );

                match self.get_values().get("default") {
                    Some(val) => {
                        print!(
                            "{val}\n"
                        );
                    }
                    None => {
                        print!(
                            "None\n"
                        );
                    }
                }
            }
            CheckboxMessage::CustomToggled(custom) => {
                self.custom = custom;
                
                print!(
                    "checkbox_input_custom: "
                );

                match self.get_values().get("custom") {
                    Some(val) => {
                        print!(
                            "{val}\n"
                        );
                    }
                    None => {
                        print!(
                            "None\n"
                        );
                    }
                }
            }
            CheckboxMessage::Styledtoggled(styled) => {
                self.styled = styled;

                print!(
                    "checkbox_input_styled: "
                );

                match self.get_values().get("styled") {
                    Some(val) => {
                        print!(
                            "{val}\n"
                        );
                    }
                    None => {
                        print!(
                            "None\n"
                        );
                    }
                }
            }
        }
    }

    fn checkbox_input_default(
        &self
    ) -> Checkbox<'_, CheckboxMessage> {
        checkbox(self.default)
        .label(
            "Default"
        )
        .on_toggle(
            CheckboxMessage::DefaultToggled
        )
    }

    fn column_checkbox_default(
        &self
    ) -> Column<'_, CheckboxMessage> {
        column![
            text(
                "CheckBox Default"
            ).size(TITLE_FONT_SIZE),
            self.checkbox_input_default()
        ]
        .spacing(10)
        .align_x(
            Alignment::Center
        )
    }

    fn checkbox_input_custom(
        &self
    ) -> Checkbox<'_, CheckboxMessage> {
        checkbox(self.custom)
        .label(
            "Custom"
        )
        .on_toggle(
            CheckboxMessage::CustomToggled
        )
        .icon(
            checkbox::Icon {
                font: ICON_FONT,
                code_point: '\u{e901}',
                size: None,
                line_height: text::LineHeight::Relative(1.0),
                shaping: text::Shaping::Basic,
            }
        )
    }

    fn column_checkbox_custom(
        &self
    ) -> Column<'_, CheckboxMessage> {
        column![
            text(
                "CheckBox Custom"
            ).size(TITLE_FONT_SIZE),
            self.checkbox_input_custom()
        ]
        .spacing(10)
        .align_x(
            Alignment::Center
        )
    }

    fn checkbox_input_styled<'a>(
        &self,
        text: &'a str
    ) -> Checkbox<'a, CheckboxMessage> {
        checkbox(self.styled)
        .label(text)
        .on_toggle_maybe(
            self.default.then_some(
                CheckboxMessage::Styledtoggled
            )
        )
    }

    fn checkboxes_input_styled(
        &self
    ) -> Row<'_, CheckboxMessage> {
        row![
            self.checkbox_input_styled(
                "Primary"
            ).style(
                checkbox::primary
            ),
            self.checkbox_input_styled(
                "Secondary"
            ).style(
                checkbox::secondary
            ),
            self.checkbox_input_styled(
                "Success"
            ).style(
                checkbox::success
            ),
            self.checkbox_input_styled(
                "Danger"
            ).style(
                checkbox::danger
            )
        ]
        .spacing(20)
    }

    fn column_checkboxes_styled(
        &self
    ) -> Column<'_, CheckboxMessage> {
        column![
            text(
                "CheckBox Styled"
            ).size(TITLE_FONT_SIZE),
            self.checkboxes_input_styled()
        ]
        .spacing(10)
        .align_x(
            Alignment::Center
        )
    }

    fn checkbox_inputs_content(
        &self
    ) -> Column<'_, CheckboxMessage> {
        column![
            self.column_checkbox_default(),
            self.column_checkboxes_styled(),
            self.column_checkbox_custom()
        ]
        .spacing(30)
        .padding(20)
        .align_x(
            Alignment::Center
        )
        .into()
    }

    pub fn view(
        &self
    ) -> Element<'_, CheckboxMessage> {
        container(
            self.checkbox_inputs_content()
        )
        .center_x(Fill)
        .into()
    }
}