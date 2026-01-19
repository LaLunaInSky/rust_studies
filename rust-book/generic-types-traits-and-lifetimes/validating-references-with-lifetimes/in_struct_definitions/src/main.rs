struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    println!(
        "\nChapter validating-references-with-lifetimes/in_struct_definitions\n"
    );

    let novel = String::from(
        "Call me Ishmael. Some years ago..."
    );

    let first_sentence = novel.split(".").next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence
    };
}