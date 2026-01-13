fn main() {
    println!(
        "\nChapter unrecovable-errors-with-panic/unrecovable_errors_with_panic_macro\n"
    );

    // panic!(
    //     "crash and burn"
    // );

    let v = vec![
        1, 2, 3
    ];

    v[99];
}
