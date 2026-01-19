pub fn largest(
    list: &[i32]
) -> &i32 {
    let mut item_largest = &list[0];

    for item in list {
        if item > item_largest {
            item_largest = item;
        }
    }

    item_largest
}