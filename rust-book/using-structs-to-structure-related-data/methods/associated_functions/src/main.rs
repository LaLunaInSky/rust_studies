#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create_a_square(
        dimension: u32
    ) -> Self {
        Self {
            width: dimension,
            height: dimension
        }
    }
}

fn main() {
    let rectangle_01 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle_square_01 = Rectangle::create_a_square(40);

    dbg!(rectangle_01);
    dbg!(rectangle_square_01);
}