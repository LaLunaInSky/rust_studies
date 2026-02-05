pub fn add(
    integer_01: u64,
    integer_02: u64
) -> u64 {
    integer_01 + integer_02
}

#[test]
fn test_add() {
    let result = add(2, 2);

    assert_eq!(result, 4);
}