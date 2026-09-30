#[derive(Debug, Clone, Copy)]
pub enum CheckboxMessage {
    DefaultToggled(bool),
    CustomToggled(bool),
    Styledtoggled(bool)
}