use std::{
    fs::File,
    error::Error
};

fn last_char_of_first_line(
    text: &str
) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "\nChapter recoverable-errors-with-result/where_to_use_the_question_mark_operator\n"
    );

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}