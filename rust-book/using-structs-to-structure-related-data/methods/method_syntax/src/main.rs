#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculete_the_area(
        &self
    ) -> u32 {
        dbg!(self);
        dbg!(self.width * self.height)
    }

    fn width(
        &self
    ) -> bool {
        dbg!(self);
        dbg!(self.width > 0)
    }
}

fn main() {
    let rectangle_01 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The 'rectangle_01' with width of {} and height of {} has an area of {} square pixels.\n",
        rectangle_01.width,
        rectangle_01.height,
        rectangle_01.calculete_the_area()
    );

    if rectangle_01.width() {
        println!(
            "The 'rectangle_01' has a nonzero width; it is {}\n",
            rectangle_01.width
        );
    }
}