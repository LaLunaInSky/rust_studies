use std::io::stdin;

fn main() {
    let number = get_an_integer_input();

    if number < 5 {
        println!(
            "Condition was true"
        );
    } else {
        println!(
            "Condition was false"
        );
    }
}

fn get_an_integer_input() -> u32 {
    println!(
        "Enter an positive integer: "
    );

    let mut input = String::new();

    stdin().read_line(&mut input).expect(
        "Failed to read line!"
    );

    let integer_input: u32 = input.trim().parse().expect(
        "The input is not an integer"
    );

    integer_input
}