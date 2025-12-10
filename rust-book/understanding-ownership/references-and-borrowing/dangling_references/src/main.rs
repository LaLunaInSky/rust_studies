fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let string = String::from("hello");

    string
}