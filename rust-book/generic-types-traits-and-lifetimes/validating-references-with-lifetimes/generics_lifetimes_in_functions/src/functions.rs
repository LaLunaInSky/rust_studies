pub fn longest<'a>(
    first_word: &'a str,
    second_word: &'a str
) -> &'a str {
    if first_word.len() > second_word.len() {
        first_word
    } else {
        second_word
    }
}