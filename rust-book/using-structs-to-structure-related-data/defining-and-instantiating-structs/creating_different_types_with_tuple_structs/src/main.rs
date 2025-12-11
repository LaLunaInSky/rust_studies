fn main() {
    let black = Color(0, 0, 0);

    let origin = Point(0, 0, 0);

    let Color(r, g, b) = black;

    let Point(x, y, z) = origin;

    println!(
        "The color is r = {r}, g = {g}, b = {b}\nThe points is x = {x} , y = {y}, z = {z}"
    );
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);