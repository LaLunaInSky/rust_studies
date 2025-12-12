#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    println!("chapter the-match-control-flow-construct/patterns_that_bind_to_values\n");

    println!(
        "{}",
        value_in_cents(
            Coin::Quarter(
                UsState::Alaska
            )
        )
    );
}

fn value_in_cents(
    coin: Coin
) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!(
                "The quarter state is: {:?}!",
                state
            );

            25
        }
    }
}