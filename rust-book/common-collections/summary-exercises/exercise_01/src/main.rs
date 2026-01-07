use exercise_01::numbers::Numbers;

fn main() {
    println!(
        "\n- The conclusion of 'Exercise 01' -\n"
    );

    let numbers_drawn = Numbers::new();

    println!(
        "The numbers drawn sorted are: {:?}\n",
        numbers_drawn.sorted_list_of_drawn_numbers()
    );

    println!(
        "The median is: {:?}\nThe number of middle positon is: {:?}\nThe values that appears most often: {:#?}",
        numbers_drawn.median(),
        numbers_drawn.number_of_middle_position(),
        numbers_drawn.values_that_appears_most_often(),
    );
}