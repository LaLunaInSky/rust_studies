pub fn add(
    left: u64,
    right: u64
) -> u64 {
    left + right
}

#[test]
fn add_result_four() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(
            String::from(
                "two plus two does not equal four"
            )
        )
    }
}