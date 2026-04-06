#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue       
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(
        &self,
        user_preference: Option<ShirtColor>
    ) -> ShirtColor {
        user_preference.unwrap_or_else(
            || self.most_stocked()
        )
    }

    fn most_stocked(
        &self
    ) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    println!(
        "\nChapter functional-features/capturing_the_environment\n"
    );

    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue
        ]
    };

    let user_preference_01 = Some(ShirtColor::Red);

    let giveaway_01 = store.giveaway(
        user_preference_01
    );

    println!(
        "The user with preference {:?} gets {:?}",
        user_preference_01,
        giveaway_01
    );

    let user_preference_02 = None;

    let giveaway_02 = store.giveaway(
        user_preference_02
    );

    println!(
        "The user with preference {:?} gets {:?}",
        user_preference_02,
        giveaway_02
    );
}