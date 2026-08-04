pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(
        strs: Vec<String>
    ) -> String {
        let mut result = String::new();
        let mut index_initial: usize = 0;
        let mut limit: usize = 500;

        loop {
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

        result
    }
}