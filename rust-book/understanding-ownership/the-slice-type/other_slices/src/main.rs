fn main() {
    let a = [
        1, 2, 3, 4, 5
    ];

    let a_slice = &a[1..3];

    println!(
        "The value of a is: {a:?}\nThe value of slice a is: {a_slice:?}"
    );

    assert_eq!(
        a_slice, [2, 3]
    );    
}
