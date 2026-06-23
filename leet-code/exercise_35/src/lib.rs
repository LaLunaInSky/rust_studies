pub struct Solution {}

impl Solution {
    pub fn search_insert(
        nums: Vec<i32>,
        target: i32
    ) -> i32 {
        let mut index_to_add: i32 = nums.len().try_into().unwrap();

        for (
            index,
            num
        ) in nums.iter().enumerate() {
            if *num == target {
                index_to_add = index.try_into().unwrap();

                break;
            } else if *num < target {
                index_to_add = (index + 1).try_into().unwrap();
            } else if *num > target {
                index_to_add = index.try_into().unwrap();

                break;
            }
        }

        return index_to_add;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums: Vec<i32> = vec!(
            1, 3, 5, 6
        );

        let target: i32 = 5;

        let solution = Solution::search_insert(
            nums.clone(),
            target
        );

        assert_eq!(
            solution,
            2
        );
    }

    #[test]
    fn case_2() {
        let nums: Vec<i32> = vec!(
            1, 3, 5, 6
        );

        let target: i32 = 2;

        let solution = Solution::search_insert(
            nums.clone(),
            target
        );

        assert_eq!(
            solution,
            1
        );
    }

    #[test]
    fn case_3() {
        let nums: Vec<i32> = vec!(
            1, 3, 5, 6
        );

        let target: i32 = 7;

        let solution = Solution::search_insert(
            nums.clone(),
            target
        );

        assert_eq!(
            solution,
            4
        );
    }

    #[test]
    fn case_4() {
        let nums: Vec<i32> = vec!(
            1, 3, 5, 6
        );

        let target: i32 = 0;

        let solution = Solution::search_insert(
            nums.clone(),
            target
        );

        assert_eq!(
            solution,
            0
        );
    }
}
