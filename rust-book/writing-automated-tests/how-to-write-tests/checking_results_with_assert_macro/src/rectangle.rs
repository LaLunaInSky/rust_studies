#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn new(
        width: u32,
        height: u32
    ) -> Self {
        Self {
            width,
            height
        }
    }

    pub fn width(
        &self
    ) -> u32 {
        self.width
    }

    pub fn height(
        &self
    ) -> u32 {
        self.height
    }

    pub fn can_hold(
        &self,
        other_rectangle: &Rectangle
    ) -> bool {
        self.width > other_rectangle.width() && self.height > other_rectangle.height()
    }
}

#[test]
fn largest_can_hold_smaller() {
    let largest = Rectangle::new(
        8, 7
    );

    let smaller = Rectangle::new(
        5, 1
    );

    assert!(
        largest.can_hold(&smaller)
    );
}

#[test]
fn smaller_cannot_hold_largest() {
    let largest = Rectangle::new(
        8, 7
    );

    let smaller = Rectangle::new(
        5, 1
    );

    assert!(
        !smaller.can_hold(&largest)
    );
}