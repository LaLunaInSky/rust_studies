use std::collections::HashMap;

fn main() {
    println!(
        "\nChapter storing-keys-with-associated-in-hash-maps/creating_a_new_hash_map\n"
    );

    let mut scores: HashMap<String, u8> = HashMap::new();

    scores.insert(
        String::from("Blue"),
        10
    );

    scores.insert(
        String::from("Yellow"),
        50
    );

    println!(
        "{:#?}",
        scores
    );
}