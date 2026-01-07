fn main() {
    println!(
        "\nChapter storing-utf-8-text-with-strings/interating_over_strings\n"
    );

    let word = "Зд";

    // Example 01
    for c in word.chars() {
        println!(
            "{c}"
        );
    }

    // Example 02
    for b in word.bytes() {
        println!(
            "{b}"
        );
    }
}