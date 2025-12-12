fn main() {
    println!("\nChapter the-match-control-flow-construct/the_option_match_pattern\n");

    let five = Some(5);
    let six = plus_one(five);

    println!(
        "the '5' with plus_one is: {:?}!",
        six
    );

    let none = plus_one(None);

    println!(
        "The 'none' with plus_one is: {:?}!",
        none
    );
}

fn plus_one(
    x: Option<i32>
) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}