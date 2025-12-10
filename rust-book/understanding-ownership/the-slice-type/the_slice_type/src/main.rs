fn main() {
    let string_01 = String::from("Hello, World!");

    let first_word_string_01 = find_the_first_word(
        &string_01
    );

    println!(
        "In the sentence '{string_01}', the first setence is:  {first_word_string_01}"
    );
}

fn find_the_first_word(
    reference_string: &String
) -> &str {
    let bytes = reference_string.as_bytes();

    for (
        i, &item
    ) in bytes.iter().enumerate() {
        if item == 32 {
            return &reference_string[0..i];
        }
    }

    &reference_string
}