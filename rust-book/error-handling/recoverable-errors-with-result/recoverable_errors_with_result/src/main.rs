use std::fs::File;

fn main() {
    println!(
        "\nChapter recoverable-errors-with-result/recoverable_errors_with_result\n"
    );

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!(
            "Problem open the file: {error:?}"
        ),
    };

    println!(
        "{greeting_file:?}"
    );
}