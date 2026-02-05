pub fn greeting(
    name: &str
) -> String {
    format!(
        "Hello {name}!"
    )

    // String::from("Hello!")
}

#[test]
pub fn test_greeting() {
    let result = greeting("Carol");

    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}