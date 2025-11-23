fn main() {
    another_function(5);

    print_labeled_measurment(
        5, 'h'
    );
}

fn another_function(
    x: i32
) {
    println!(
        "The value of x is: {x}."
    );
}

fn print_labeled_measurment(
    value: i32,
    unit_label: char
) {
    println!(
        "\nThe measurement is: {value}{unit_label}."
    );
}