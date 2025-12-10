fn main() {
    let x = 5;

    let y = x;

    let s1 = String::from("hello");

    let s2 = &s1;

    println!(
        "The value x is: {x}\nThe value y is: {y}\nThe value s1 is: {s1}\nThe value s2 is: {s2}"
    );
}
