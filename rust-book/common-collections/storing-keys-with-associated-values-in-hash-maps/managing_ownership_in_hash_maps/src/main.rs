use std::collections::HashMap;

fn main() {
    println!(
        "\nChapter storing-keys-associated-values-in-hash-maps/managing_ownership_in_hash_maps\n"
    );

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(
        field_name,
        field_value
    );

    println!(
        "{:#?}",
        map
    );
}
