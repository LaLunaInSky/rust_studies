pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(
        s: String
    ) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len_to_chars: usize = chars.len();        

        let mut result = String::new();

        let mut matriz: Vec<Vec<bool>> = Vec::new();

        for index in 0..len_to_chars {
            matriz.push(
                Vec::new()
            );

            for _ in 0..len_to_chars {
                matriz[index].push(
                    true
                );
            }
        }

        let mut start_position: usize = 0;
        let mut max_length_find: usize = 1;

        for first_index in (0..len_to_chars - 1).rev() {
            for next_index in (first_index + 1)..len_to_chars {
                matriz[first_index][next_index] = false;

                if chars[first_index] == chars[next_index] {
                    matriz[first_index][next_index] = matriz[first_index + 1][next_index - 1];

                    if (next_index + 1) - first_index > max_length_find && matriz[first_index][next_index] {
                        start_position = first_index;
                        max_length_find = (next_index + 1) - first_index; 
                    }
                }
                
            }
        }

        for char in chars[start_position..start_position + max_length_find].iter() {
            result.push(*char);
        }

        return result;
    }
}