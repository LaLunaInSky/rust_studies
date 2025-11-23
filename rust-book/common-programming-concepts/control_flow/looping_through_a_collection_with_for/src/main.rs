fn main() {
    let a = [
        10, 20, 30, 40, 50, 60
    ];

    let a_len = a.len();

    let mut index = 0;

    while index < a_len {
        println!(
            "The value is: {}",
            a[index]
        );

        index += 1;
    }

    // or 
    println!();

    for element in a {
        println!(
            "The value is: {element}"
        );
    }

    // Countdown
    println!();

    for number in (1..4).rev() {
        println!(
            "{number}"
        );
    }

    println!(
        "LIFTOFF!!!"
    );
}
