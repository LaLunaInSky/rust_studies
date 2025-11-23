fn main() {
    let tup: (
        u32, f64, u8
    ) = (
        500, 6.4, 1
    );

    let (
        x, y, z
    ) = tup;

    println!(
        "The value of x is: {x}\nThe value of y is: {y}\nThe value of z is: {z}"
    );

    // or
    println!(
        "The value of tup.0 is: {0}\nThe value of tup.1 is: {1}\nThe value of tup.2 is: {2}",
        tup.0, tup.1, tup.2
    );
}