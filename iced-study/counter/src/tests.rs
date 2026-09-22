use crate::{
    states::Counter,
    messages::Message
};

#[test]
fn test_1() {
    // let mut counter = Counter {
    //     value: 0
    // };

    let mut counter = Counter::default();

    counter.update(
        Message::Increment
    );

    counter.update(
        Message::Increment
    );

    counter.update(
        Message::Decrement
    );

    assert_eq!(
        counter.value,
        1
    )
}