pub struct Solution {}

impl Solution {
    pub fn reverse(
        x: i32
    ) -> i32 {
        let mut string_x_reverse = String::new();
        let mut is_negative = false;

        if x < 0 {
            is_negative = true;
        }

        for char in x.to_string().chars().rev() {
            if char != '-' {
                string_x_reverse.push(char);
            }
        }

        match string_x_reverse.parse::<i32>() {
            Ok(number) => {
                if is_negative {
                    return -number;
                } else {
                    return number
                }
            }
            Err(_) => return 0,
        }
    }
}