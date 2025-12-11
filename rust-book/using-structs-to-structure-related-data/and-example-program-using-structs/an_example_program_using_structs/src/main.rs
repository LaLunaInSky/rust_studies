fn main() {
    let rectangle_01 = Rectangle {
        width: 30,
        height: 50
    };

    dbg!(&rectangle_01);

    println!(
        "The area of the rectangle with width = {} and height = {} is {} square pixels.",
        rectangle_01.width,
        rectangle_01.height,
        calculate_the_area_of_an_rectangle(
            &rectangle_01
        )
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn calculate_the_area_of_an_rectangle(
    dimensions: &Rectangle
) -> u32 {
    let Rectangle{width, height} = dimensions;

    dbg!(width * height)
}