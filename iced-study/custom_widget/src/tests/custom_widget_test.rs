use crate::{
    states::custom_widget_state::CustomWidgetState,
    messages::custom_widget_message::CustomWidgetMessage,
};

#[test]
fn custom_widget_state_default_radius() {
    let custom_widget_state = CustomWidgetState::default();

    assert_eq!(
        custom_widget_state.get_radius(),
        50.0
    );
}

#[test]
fn custom_widget_state_update_radius() {
    let mut custom_widget_state = CustomWidgetState::default();

    custom_widget_state.update(
        CustomWidgetMessage::RadiusChanged(
            80.0
        )
    );

    assert_eq!(
        custom_widget_state.get_radius(),
        80.0
    );
}