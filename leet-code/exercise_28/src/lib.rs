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

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let haystack = String::from(
            "sadbutsad"
        );

        let needle = String::from(
            "sad"
        );

        let solution = Solution::str_str(
            haystack.clone(),
            needle.clone()
        );

        assert_eq!(
            solution,
            0
        );
    }

    #[test]
    fn case_2() {
        let haystack = String::from(
            "leetcode"
        );

        let needle = String::from(
            "leeto"
        );

        let solution = Solution::str_str(
            haystack.clone(),
            needle.clone()
        );

        assert_eq!(
            solution,
            -1
        );
    }
}
