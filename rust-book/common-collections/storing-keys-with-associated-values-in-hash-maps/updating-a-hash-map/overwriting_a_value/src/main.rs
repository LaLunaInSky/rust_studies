use std::collections::HashMap;

fn main() {
    println!(
        "\nChapter storing-keys-associated-values-in-hash-maps/updating-a-hash-map/overwriting_a_value\n"
    );

    let mut scores: HashMap<String, u8> = HashMap::new();

    scores.insert(
        String::from("Blue"),
        10
    );

    println!(
        "{:#?}",
        scores
    );

    scores.insert(
        String::from("Blue"),
        25
    );

    println!(
        "{:#?}",
        scores
    );
}   