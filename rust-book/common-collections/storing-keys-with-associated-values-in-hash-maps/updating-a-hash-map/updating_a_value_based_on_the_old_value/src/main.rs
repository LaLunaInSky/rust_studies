use std::collections::HashMap;

fn main() {
    println!(
        "\nChapter storing-keys-associated-values-in-hash-maps/updating-a-hash-map/updating_a_value_based_on_the_old_values\n"
    );

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!(
        "{:#?}",
        map
    );
}
