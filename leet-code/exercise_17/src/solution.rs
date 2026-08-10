pub struct Solution {}

impl Solution {
    pub fn letter_combinations(
        digits: String 
    ) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let digits_of_letter: Vec<char> = vec!(
            '2', '3', '4', '5', '6', '7', '8', '9'
        );

        let letters: Vec<Vec<char>> = vec!(
            vec!(
                'a', 'b', 'c'
            ),
            vec!(
                'd', 'e', 'f'
            ),
            vec!(
                'g', 'h', 'i'
            ),
            vec!(
                'j', 'k', 'l'
            ),
            vec!(
                'm', 'n', 'o'
            ),
            vec!(
                'p', 'q', 'r', 's'
            ),
            vec!(
                't', 'u', 'v'
            ),
            vec!(
                'w', 'x', 'y', 'z'
            ),
        );

        let mut index_founds: Vec<usize> = Vec::new();

        for digit in digits.chars() {
            for index_digit_of_letter in 0..digits_of_letter.len() {
                if digits_of_letter[index_digit_of_letter] == digit {
                    index_founds.push(index_digit_of_letter);
                }
            }
        }

        for next_index in (0..index_founds.len()).rev() {
            if next_index == index_founds.len() - 1 {
                for letter in letters[
                    index_founds[next_index]
                ].iter() {
                    result.push(
                        String::from(*letter)
                    );
                }
            } else {
                let mut new_result: Vec<String> = Vec::new();

                for letter in letters[
                    index_founds[next_index]
                ].iter() {
                    for letter_result in result.iter() {
                        let mut string_letter = String::from(*letter);

                        string_letter += letter_result;

                        new_result.push(string_letter);
                    }
                }

                result.clear();

                result = new_result.clone();
            }
        }
        
        result 
    }
}