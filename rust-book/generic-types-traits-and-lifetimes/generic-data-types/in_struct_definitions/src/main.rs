use in_struct_definitions::point::Point;

fn main() {
    println!(
        "\nChapter generic-data-types/in_struct_definitions\n"
    );

    let interger_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    let integer_and_float_point = Point::new(5, 4.0);

    println!(
        "{:#?}\n{:#?}\n{:#?}",
        interger_point,
        float_point,
        integer_and_float_point,
    );
}
