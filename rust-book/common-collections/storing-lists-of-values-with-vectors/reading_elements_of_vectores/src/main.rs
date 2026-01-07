fn main() {
    let v = vec![
        1, 2, 3, 4, 5
    ];

    // Example 1
    let third: &i32 = &v[2];
    println!(
        "The third element is {third}"
    );

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!(
            "The third element is {third}"
        ),
        None => println!(
            "There is no third element"
        ),
    }

    // Example 2
    // let hundredth = &v[100];
    // println!(
    //     "The hundredth element is {hundredth}"
    // );

    let hundredth: Option<&i32> = v.get(100);
    match hundredth {
        Some(hundredth) => println!(
            "The hundredth element is {hundredth}"
        ),
        None => println!(
            "There is no hundredth element"
        ),
    }
}