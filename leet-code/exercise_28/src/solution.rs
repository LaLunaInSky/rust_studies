pub struct Solution {}

impl Solution {
    pub fn str_str(
        haystack: String,
        needle: String
    ) -> i32 {
        let mut result: i32 = -1;
        
        match haystack.find(&needle) {
            Some(x) => result = x.try_into().unwrap(),
            None => (),
        }

        result
    }
}