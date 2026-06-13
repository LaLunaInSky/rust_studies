pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(
        strs: Vec<String>
    ) -> String {
        let mut result = String::new();
        let mut index_initial: usize = 0;
        let mut limit: usize = 500;

        while true {
            let mut chars: Vec<char> = Vec::new();

            for str in strs.iter() {
                if str.len() < limit {
                    limit = str.len();
                }

                for (
                    index,
                    char
                ) in str.chars().enumerate() {
                    if index == index_initial {
                        if chars.len() == 0 {
                            chars.push(char);
                        } else if char == chars[
                            chars.len() - 1
                        ] {
                            chars.push(char);
                        }
                    }
                }
            }

            if chars.len() == strs.len() {
                result.push(
                    chars[0]
                );
            } else {
                break;
            }

            if index_initial < limit {
                index_initial += 1;
            } else {
                break;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod cases {
    use super::*;

    #[test]
    fn case_1() {
        let strs: Vec<String> = vec!(
            String::from(
                "flower"
            ),
            String::from(
                "flow"
            ),
            String::from(
                "flight"
            )
        );

        let solution = Solution::longest_common_prefix(
            strs.clone()
        );

        assert_eq!(
            solution,
            "fl"
        );
    }

    #[test]
    fn case_2() {
        let strs: Vec<String> = vec!(
            String::from(
                "dog"
            ),
            String::from(
                "racecar"
            ),
            String::from(
                "car"
            )
        );

        let solution = Solution::longest_common_prefix(
            strs.clone()
        );

        assert_eq!(
            solution,
            ""
        );
    }

    #[test]
    fn case_3() {
        let strs: Vec<String> = vec!(
            String::from(
                "cir"
            ),
            String::from(
                "car"
            )
        );

        let solution = Solution::longest_common_prefix(
            strs.clone()
        );

        assert_eq!(
            solution,
            "c"
        );
    }
}