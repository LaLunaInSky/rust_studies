use exercise_02::{
    words::Words,
    example_words::get_example_words
};

fn main() {
    println!(
        "\n- The conclusion of 'Exercise 02' -\n"
    );

    let mut words_list = Words::new();

    for example_word in get_example_words() {
        words_list.add_new_word_in_list(
            String::from(example_word)
        );
    }

    println!(
        "{:#?}",
        words_list.words_list()
    );
}
