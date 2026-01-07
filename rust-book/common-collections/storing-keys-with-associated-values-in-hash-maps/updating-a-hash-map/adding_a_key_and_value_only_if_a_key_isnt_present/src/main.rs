use std::collections::HashMap;

fn main() {
    println!(
        "\nChapter storing-keys-associated-values-in-hash-maps/updating-a-hash-map/adding_a_key_and_value_only_if_a_key_isnt_present\n"
    );

    let mut scores: HashMap<String, u8> = HashMap::new();

    scores.insert(
        String::from("Blue"),
        10
    );

    scores.entry(
        String::from("Yellow")
    ).or_insert(50);

    scores.entry(
        String::from("Blue")
    ).or_insert(50);

    println!(
        "{:#?}",
        scores
    );
}
