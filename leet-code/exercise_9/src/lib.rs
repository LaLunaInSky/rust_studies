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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let x: i32 = 121;

        let solution = Solution::is_palindrome(x);

        assert_eq!(
            solution,
            true
        );
    }

    #[test]
    fn case_2() {
        let x: i32 = -121;

        let solution = Solution::is_palindrome(x);

        assert_eq!(
            solution,
            false
        );
    }

    #[test]
    fn case_3() {
        let x: i32 = 10;

        let solution = Solution::is_palindrome(x);

        assert_eq!(
            solution,
            false
        );
    }
}
