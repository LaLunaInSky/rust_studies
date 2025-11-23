fn main() {
    let a: [u32; 5] = [
        1, 2, 3, 4, 5
    ];

    let months = [
        "January", "Fabruary", "March", "April", "May", "June", "August", "September", "October", "November", "December"
    ];

    let a_2 = [3; 5];

    println!(
        "The array \'a\': {a:?}\nThe array \'months\': {months:?}\nThe array \'a_2\': {a_2:?}"
    );
}
