use std::{
    fs::File,
    io::ErrorKind
};

fn main() {
    println!(
        "\nChapter recoverable-errors-with-result/matching_on_different_errors\n"
    );

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file_create) => file_create,
    //             Err(error_create) => panic!(
    //                 "Problem creating the file? {error_create:?}"
    //             ),
    //         },
    //         _ => panic!(
    //             "Problem opening the file: {error:?}"
    //         ),
    //     }
    // };

    // Or

    let greeting_file = File::open("hello.txt").unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(
                    |error| {
                        panic!(
                            "Problem creating the file: {error:?}"
                        );
                    }
                )
            } else {
                panic!(
                    "Problem opening the file: {error:?}"
                );
            }
        }
    );

    println!(
        "{:#?}",
        greeting_file
    );
}
