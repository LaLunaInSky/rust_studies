use std::cmp::PartialOrd;

pub fn largest<T: PartialOrd>(
    list: &[T]
) -> &T {
    let mut item_largest = &list[0];

    for item in list {
        if item > item_largest {
            item_largest = item;
        }
    }

    item_largest
}