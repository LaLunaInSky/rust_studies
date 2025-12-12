enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(
        &self,
        year: u16,
    ) -> bool {
        UsState::Alabama => year  >= 1819,
        UsState::Alaska => year >= 1959,
        // -- snip --
    }
}

fn main() {
    println!(
        "\nchapter concise-control-flow-with-if-let-and-let-else/staying_on_the_happy_path_with_let_else\n"
    );
}

fn describe_state_quarter(
    coin: Coin
) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(
    //         1900
    //     ) {
    //         Some(
    //             format!(
    //                 "{state:?} is pretyy old, for America!"
    //             )
    //         )
    //     } else {
    //         Some(
    //             format!(
    //                 "{state:?} is relatively new."
    //             )
    //         )
    //     }
    // } else {
    //     None
    // }

    // Or

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    // Or

    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(
        1900
    ) {
        Some(
            format!(
                "{state:?} is pretty old, for America!"
            )
        )
    } else {
        Some(
            format!(
                "{state:?} is relatively new."
            )
        )
    }
}
