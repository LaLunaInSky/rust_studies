fn main() {
    let a: [u32; 5] = [
        1, 2, 3, 4, 5
    ];

    let months = [
        "January", "Fabruary", "March", "April", "May", "June", "August", "September", "October", "November", "December"
    ];

    let a_2 = [3; 5];

    println!(
        "The array \'a\': {a:?}\nThe array \'months\': {months:?}\nThe array \'a_2\': {a_2:?}\n"
    );

    let first_element_a = a[0];
    let second_element_a = a[1];

    println!(
        "The first elemet of the array \'a\' is: {first_element_a}.\nThe second element of the array \'a\' is: {second_element_a}."
    );
}