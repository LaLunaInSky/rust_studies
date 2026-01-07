use std::collections::HashMap;
use crate::analyzer::analyze_the_first_letter_and_relocate_it_within_the_word;

#[derive(Debug)]
pub struct Words {
    words_list: HashMap<String, String>
}

impl Words {
    pub fn new() -> Self {
        let words_list: HashMap<String, String> = HashMap::new();

        Self {
            words_list
        }
    }

    pub fn words_list(
        &self
    ) -> HashMap<String, String> {
        let words_list = self.words_list.clone();

        words_list
    }

    pub fn add_new_word_in_list(
        &mut self,
        word: String
    ) {
        self.words_list.entry(
            word.clone()
        ).or_insert(
            analyze_the_first_letter_and_relocate_it_within_the_word(
                &word
            )
        );
    }
}