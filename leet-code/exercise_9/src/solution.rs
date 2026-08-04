pub struct Solution {}

impl Solution {
    pub fn is_palindrome(
        x: i32
    ) -> bool {
        let x_string = format!(
            "{}",
            x
        );

        let mut x_string_reverse = String::new();

        for char in x_string.chars().rev() {
            x_string_reverse.push(char);
        }

        return x_string == x_string_reverse;
    }
}