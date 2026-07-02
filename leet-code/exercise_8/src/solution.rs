pub struct Solution {}

impl Solution {
    pub fn my_atoi(
        s: String
    ) -> i32 {
        use std::num::IntErrorKind::{
            PosOverflow,
            NegOverflow
        };

        let mut chars: Vec<char> = s.trim().chars().collect();
        
        let mut string_to_number = String::new();
        
        let mut result: i32 = 0;

        if chars.len() != 0 {
            if chars[0] == '-' || chars[0] == '+' {
                if chars[0] == '-' {
                    string_to_number.push(
                        chars[0]
                    );
                }
    
                chars.remove(0);
            }

            let len_chars = chars.len();
    
            if len_chars > 0 {
                for index in 0..len_chars {
                    if chars[index].is_digit(10) {
                        string_to_number.push(
                            chars[index]
                        );
                    } else {
                        break;
                    }
                }
        
                match string_to_number.parse::<i32>() {
                    Ok(number) => {
                        result = number;
                    }
                    Err(error) => {
                        match error.kind() {
                            PosOverflow => result = i32::MAX,
                            NegOverflow => result = i32::MIN,
                            _ => (),
                        }
                    }
                }
            }
        }

        return result;
    }
}