use crate::raffles;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Numbers {
    total_number_of_drawn: u8,
    list_of_drawn_numbers: Vec<u8>,
}

impl Numbers {
    pub fn new() -> Self {
        let total_number_of_drawn = raffles::drawn_an_integer_number_from_u8();

        let mut list_of_drawn_numbers: Vec<u8> = Vec::new();

        for _count in 0..total_number_of_drawn {
            list_of_drawn_numbers.push(
                raffles::drawn_an_integer_number_from_u8()
            );
        }

        Self {
            total_number_of_drawn,
            list_of_drawn_numbers
        }
    }

    pub fn list_of_drawn_numbers(
        &self
    ) -> Vec<u8> {
        self.list_of_drawn_numbers.clone()
    }

    pub fn sorted_list_of_drawn_numbers(
        &self
    ) -> Vec<u8> {
        let mut sorted_list_of_drawn_numbers = self.list_of_drawn_numbers.clone();
        
        sorted_list_of_drawn_numbers.sort_by(
            |a, b| a.partial_cmp(b).unwrap()
        );

        sorted_list_of_drawn_numbers
    }

    pub fn median(
        &self
    ) -> u32 {
        let mut sum_of_all_numbers: u32 = 0;

        for number in self.sorted_list_of_drawn_numbers() {
            sum_of_all_numbers += u32::from(number);
        }

        sum_of_all_numbers
    }

    pub fn number_of_middle_position(
        &self
    ) -> u8 {
        let index_of_middle_position: usize = usize::from(self.total_number_of_drawn) / 2;

        let number_of_middle_position = self.sorted_list_of_drawn_numbers()[index_of_middle_position];

        number_of_middle_position
    }

    pub fn values_that_appears_most_often(
        &self
    ) -> Vec<String> {
        let mut all_the_number_and_their_appearances: HashMap<u8, u8> = HashMap::new();

        let mut the_quantity_that_times_greater: u8 = 0;
        let mut the_values_that_appear_most_often: Vec<String> = Vec::new();

        for number in self.sorted_list_of_drawn_numbers() {
            let count = all_the_number_and_their_appearances.entry(number).or_insert(1);

            *count += 1;
        } 

        for (
            key, value
        ) in all_the_number_and_their_appearances {
            if the_quantity_that_times_greater < value {
                the_quantity_that_times_greater = value;

                the_values_that_appear_most_often.clear();
            }
            
            if the_quantity_that_times_greater == value {
                the_values_that_appear_most_often.push(
                    format!(
                        "{} appears {} times",
                        key, value
                    )
                )
            }

        }

        the_values_that_appear_most_often
    }
}