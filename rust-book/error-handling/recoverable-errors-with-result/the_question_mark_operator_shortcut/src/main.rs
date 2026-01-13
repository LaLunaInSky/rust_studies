use std::{
    fs::{
        self,
        File
    },
    io::{
        self,
        Read
    }
};

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;

    // let mut username = String::new();
    
    // username_file.read_to_string(
    //     &mut username
    // )?;

    // Or
    // let mut username = String::new();

    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Ok(username)

    // Or
    fs::read_to_string("hello.txt")
}

fn main() {
    println!(
        "\nChapter recoverable-errors-with-result/the_question_mark_operator_shortcut\n"
    );

    let username = read_username_from_file();
    
    println!(
        "{:?}",
        username
    );
}