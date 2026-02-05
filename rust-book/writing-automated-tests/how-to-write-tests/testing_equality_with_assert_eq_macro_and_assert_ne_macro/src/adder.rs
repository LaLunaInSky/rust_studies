pub fn add_two(
    integer: u64 
) -> u64 {
    integer + 2
}

#[test]
fn test_add_two() {
    let result = add_two(2);

    assert_eq!(result, 4);
}