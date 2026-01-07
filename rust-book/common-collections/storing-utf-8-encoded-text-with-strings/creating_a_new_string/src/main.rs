fn main() {
    // Example 01
    let mut s = String::new();

    println!(
        "{s} -- Example 01"
    );

    // Example 02
    let data = "initial contents";
    let s = data.to_string();

    println!(
        "{s} -- Example 02"
    );

    // The method also works on a literal directly
    let s = "initial contents".to_string();

    println!(
        "{s} -- Example 02"
    );

    // Example 03
    let s = String::from("inital contents");

    println!(
        "{s} -- Example 03\n"
    );

    // Example 04
    let hello_in_various_languages = vec![
        "السلام عليكم", "Dobrý den", "Hello", "שלום", "नमस्ते", "こんにちは", "안녕하세요", "你好", "Olá", "Здравствуйте", "Hola"
    ];

    for hello_value in hello_in_various_languages {
        let hello = String::from(hello_value);

        println!(
            "{hello} -- Example 04"
        );
    }
}
