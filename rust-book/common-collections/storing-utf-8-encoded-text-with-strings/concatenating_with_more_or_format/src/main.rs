fn main() {
    // Example 01
    let string_01 = String::from("Hello, ");
    let string_02 = String::from("world!");
    let string_03 = string_01.clone() + &string_02;

    println!(
        "Example 01\nThe string_01 is: {string_01}\nThe string_02 is: {string_02}\nThe string_03 is: {string_03}"
    );

    // Example 02
    let string_01 = String::from("tic");
    let string_02 = String::from("tac");
    let string_03 = String::from("toe");
    let string_04 = string_01.clone() + "-" + &string_02 + "-" + &string_03;

    println!(
        "\nExample 02\nThe string_01 is: {string_01}\nThe string_02 is: {string_02}\nThe string_03 is: {string_03}\nThe string_04 is: {string_04}\n"
    );

    // Example 03
    let string_01 = String::from("tic");
    let string_02 = String::from("tac");
    let string_03 = String::from("toe");
    let string_04 = format!(
        "{string_01}-{string_02}-{string_03}"
    );

    println!(
        "\nExample 03\nThe string_01 is: {string_01}\nThe string_02 is: {string_02}\nThe string_03 is: {string_03}\nThe string_04 is: {string_04}\n"
    );
}
