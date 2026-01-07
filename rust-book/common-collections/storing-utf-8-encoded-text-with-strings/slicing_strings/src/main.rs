fn main() {
    println!("\nChapter storing-utf-8-enconded-text-with-strings/slicing_strings\n");

    // Example 01
    let hello = "Здравствуйте";

    println!(
        "{} and {:?} -- Example 01\n",
        hello,
        search_and_return_x_letters(
            hello,
            4
        )
    );
}

// My test
fn search_and_return_x_letters(
    word: &str,
    how_many_return_letters: usize
) -> Vec<char> {
    let mut list_of_return_letters: Vec<char> = Vec::new();

    for (
        index, letter
    ) in word.chars().enumerate() {
        if index < how_many_return_letters {
            list_of_return_letters.push(letter);
        }
    }

    return list_of_return_letters;
}