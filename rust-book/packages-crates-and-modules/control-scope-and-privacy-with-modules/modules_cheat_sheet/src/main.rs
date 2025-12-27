use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!(
        "\nChapter control-scope-and-privacy-with-modules/modules_cheat_sheet\n"
    );

    let plant = Asparagus {};

    println!(
        "I'm growing {plant:?}!"
    );
}
