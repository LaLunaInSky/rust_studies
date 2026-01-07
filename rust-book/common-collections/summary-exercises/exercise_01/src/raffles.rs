use rand::random;

pub fn drawn_an_integer_number_from_u8() -> u8 {
    let drawn_number: u8 = random::<u8>();

    drawn_number
}