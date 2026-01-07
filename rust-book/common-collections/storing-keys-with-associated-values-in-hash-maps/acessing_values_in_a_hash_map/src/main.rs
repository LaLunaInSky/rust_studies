use std::collections::HashMap;

fn main() {
    println!(
        "\nChapter storing-keys-with-associated-values-in-hash-maps/acessing_values_in_a_hash_map\n"
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
        "{:#?}\n",
        scores
    );

    // Example 01

    let team_name_01 = String::from("Blue");

    let score = scores.get(
        &team_name_01
    ).copied().unwrap_or(0);

    println!(
        "{score} -- Example 01\n"
    );

    // Example 02
    for (
        key,
        value
    ) in &scores {
        println!(
            "{key}: {value} -- Example 02"
        );
    }
}