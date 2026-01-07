fn main() {
    let mut v = vec![
        100, 32, 57
    ];

    for count in 1..4 {
        if count % 2 != 0 {
            for value in &v {
                println!(
                    "{value}"
                );
            }
            println!();
        } else {
            for value in &mut v {
                *value += 50;
            }
        }
    }
}