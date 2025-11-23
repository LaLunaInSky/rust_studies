fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    let y = plus_one(x);

    println!(
        "The value of x is: {x}\nThe value of y is: {y}"
    );
}

fn plus_one(
    x: i32
) -> i32 {
    x + 1
}