fn main() {
    let string_01 = String::from("hello");

    let string_01_len = calculate_the_length_of_a_string(
        string_01
    );

    println!(
        "The length of '{string_01}' is {string_01_len}."
    );
}

fn calculate_the_length_of_a_string(
    reference_string: &String
) -> usize {
    reference_string.len()
}