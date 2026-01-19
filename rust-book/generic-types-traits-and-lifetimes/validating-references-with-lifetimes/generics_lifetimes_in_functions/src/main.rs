use generics_lifetimes_in_functions::functions::longest;

fn main() {
    println!(
        "\nChapter validating-references-with-lifetimes/generics_lifetimes_in_functions\n"
    );

    let string_01 = String::from("abcd");

    let string_02 = "xyz";

    let result = longest(
        &string_01.as_str(),
        &string_02
    );

    println!(
        "The longest is '{result}'"
    );
}