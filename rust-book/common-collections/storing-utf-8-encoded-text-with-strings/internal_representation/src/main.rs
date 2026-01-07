fn main() {
    println!(
        "\nChapter stortin-utf-8-enconded-text-with-strings/internal_representation\n"
    );

    // Example 01
    let hello = String::from("Hola");

    println!(
        "{} has {} letters -- Example 01\n",
        hello,
        hello.len()
    );

    // Example 02
    let hello = String::from("Здравствуйте");

    println!(
        "{} has {} letters -- Example 02\n",
        hello,
        hello.len()
    );

    // Example 03
    let hello = "Здравствуйте";
    let answer = &hello.bytes().nth(0);

    println!(
        "{} and {:?} -- Example 03\n",
        hello,
        answer
    );
}
