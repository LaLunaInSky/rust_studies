use in_functions_definitions::largest::largest;

fn main() {
    println!(
        "\nChapter generic-data-types/in_functions_definitions\n"
    );

    let number_list = vec![
        34, 50, 25, 100, 65
    ];

    let result = largest(&number_list);
    
    println!(
        "The largest number is {result}"
    );

    let char_list = vec![   
        'y', 'm', 'a', 'q'
    ];

    let result = largest(&char_list);

    println!(
        "The largest char is {result}"
    );
}
