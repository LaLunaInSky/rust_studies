use in_method_definitions::point::{
    Point,
    point_mixup::PointMixup
};

fn main() {
    println!(
        "\nChapter generic-data-types/in_method_definitions\n"
    );

    let p = Point::new(5, 10);

    println!(
        "p.x = {}",
        p.x()
    );

    let point_f32 = Point::new(5.0, 1.0);

    println!(
        "The distance from origin is {}",
        point_f32.distance_from_origin()
    );

    let point_mixup_01 = PointMixup::new(5, 10.4);
    let point_mixup_02 = PointMixup::new("Hello", 'c');

    let point_mixup_03 = point_mixup_01.mixup(
        point_mixup_02.clone()
    );

    println!(
        "\n{:#?}\n{:#?}\n{:#?}",
        point_mixup_01,
        point_mixup_02,
        point_mixup_03
    );
}