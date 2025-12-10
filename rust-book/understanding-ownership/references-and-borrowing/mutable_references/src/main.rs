fn main() {
    let mut  string_01 = String::from("hello");

    change_a_string(
        &mut string_01,
        ", world!"
    );

    println!(
        "{string_01}"
    );
}


fn change_a_string(
    reference_of_a_mutable_string: &mut String,
    additional_text_for_the_string: &str
) {
    reference_of_a_mutable_string.push_str(
        &additional_text_for_the_string
    );
}