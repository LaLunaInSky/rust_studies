use std::fs::File;

fn main() {
    println!(
        "\nChapter recoverable-errors-with-result/shortcuts_for_panic_on_error\n"
    );

    // let greeting_file = File::open("hello.txt").unwrap();

    // Or 

    let greeting_file = File::open("hello.txt").expect(
        "hello.txt should be included in this project"
    );

    println!(
        "{:?}",
        greeting_file
    );
}
