use crate::languages::Language;

#[derive(Debug, Clone, Copy)]
pub enum ComboBoxMessage {
    Selected(Language),
    OptionHovered(Language),
    Closed,
}