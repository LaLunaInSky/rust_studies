fn main() {
    // addition
    let sum = 5 + 10;

    println!(
        "The addition of \'5 + 10\' is: {sum}"
    );  

    // subtraction
    let difference = 95.5 - 4.3;

    println!(
        "The subtraction of \'95.5 - 4.3\' is: {difference}"
    );

    // division
    let quotient = 56.7 / 32.2;

    println!(
        "The division of \'56.7 / 32.2\' is: {quotient:.2}"
    );

    let truncated = -5 / 3;

    println!(
        "The division of \'-5 / 3\' is: {truncated}"
    );

    // remainder
    let remainder = 43 % 5;

    println!(
        "The remainder of \'43 % 5\' is: {remainder}"
    );
}