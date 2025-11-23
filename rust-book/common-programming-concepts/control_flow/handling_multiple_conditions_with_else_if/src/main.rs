use std::io::stdin;

fn main() {
    let number = get_the_input_of_a_positive_interger();

    if number% 4 == 0 {
        println!(
            "Number {number} is divisible by 4"
        );
    } else if number % 3 == 0 {
        println!(
            "Number {number} is divisible by 3"
        );
    } else if number % 2 == 0 {
        println!(
            "Number {number} is divisible by 2"
        );
    } else {
        println!(
            "Number {number} is not divisible by 4, 3, or 2"
        );
    }
}

fn get_the_input_of_a_positive_interger() -> u32 {
    println!(
        "Enter an positive interger:"
    );

    let mut input = String::new();

    stdin().read_line(&mut input).expect(
        "Failed to read line!"
    );

    let positive_number_entered: u32 = input.trim().parse().expect(
        "The input not an positive number"
    );

    positive_number_entered
}
