use removing_duplication_by_extracting_a_function::largest::largest;

fn main() {
    println!(
        "\nChapter generic-types-traits-and-lifetimes/removing_duplication_by_extracting_a_function\n"
    );

    let number_list = vec![
        34, 50, 25, 100, 65
    ];

    let result = largest(&number_list);

    println!(
        "The largest number is {result}"
    );

    let number_list = vec![
        102, 34, 6000, 89, 54, 2, 43, 8
    ];

    let result = largest(&number_list); 

    println!(
        "The largest number is {result}"
    );
}