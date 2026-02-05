use super::*;

#[test]
fn internal() {
    let result = internal_adder(2, 2);

    assert_eq!(result, 4);
}