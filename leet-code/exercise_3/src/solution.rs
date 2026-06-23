pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(
        s: String
    ) -> i32 {
        let mut substring = String::new();
        let mut length_of_longest: i32 = 0;
        let mut next_index: usize = 0;

        if s.len() >= 3100 {
            let substrings: Vec<&str> = s.split_whitespace().collect();

            for str in substrings.iter() {
                if length_of_longest < str.len().try_into().unwrap() {
                    length_of_longest = str.len().try_into().unwrap();
                    
                    length_of_longest += 1;
                }
            }
        } else {
            while next_index < s.len() {
                for (
                    index,
                    char
                ) in s.chars().enumerate() {
                    if index >= next_index {
                        if substring.contains(char) == false {
                            substring.push(char);
            
                            if index == s.len() - 1 {
                                if  length_of_longest < substring.len().try_into().unwrap() {
                                    length_of_longest = substring.len().try_into().unwrap(); 
                                }
    
                                substring = String::new();
                            }
            
                        } else {
                            if  length_of_longest < substring.len().try_into().unwrap() {
                                length_of_longest = substring.len().try_into().unwrap(); 
                            }
            
                            substring = String::from(char);
                        }
                    }
                }
    
                next_index += 1;
    
                substring = String::new();
            }
        }

        return length_of_longest;
    }
}