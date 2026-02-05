pub fn add_two(
    a: u64
) -> u64 {
    a + 2
}

#[test]
fn add_two_and_two() {
    let result = add_two(2);

    assert_eq!(result, 4);
}

#[test]
fn add_three_and_two() {
    let result = add_two(3);

    assert_eq!(result, 5);
}

#[test]
#[ignore]
fn add_one_hundred_and_two() {
    let result = add_two(100);

    assert_eq!(result, 102);
}