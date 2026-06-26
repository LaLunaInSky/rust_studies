pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(
        s: String
    ) -> String {
        let chars: Vec<char> = s.chars().collect();

        let mut result = String::new();
        
        let mut substring_palindrome: Vec<char> = Vec::new();
        
        for initial_index in 0..chars.len() {
            let mut substring: Vec<char> = Vec::new();

            for index in initial_index..chars.len() {
                substring.push(
                    chars[index]
                );
                
                if substring.len() > substring_palindrome.len() {
                    let mut subtring_reverse = substring.clone();
                    
                    subtring_reverse.reverse();
                    
                    if *substring == *subtring_reverse {
                        if substring_palindrome.len() < substring.len() {
                            substring_palindrome = substring.clone();
                        }
                    }
                }
            }
        }
        
        for char in substring_palindrome.iter() {
            result.push(*char);
        }

        return result;
    }
}