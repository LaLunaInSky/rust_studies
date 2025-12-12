fn main() {
    println!("\nchapter the-match-control-flow-construct/catch_all_patterns_and_the_placeholder\n");

    let dice_roll: u8 = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(
//     num_spaces: u8
// ) {}
// fn reroll() {}