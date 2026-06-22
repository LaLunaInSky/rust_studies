pub struct Solution {}

impl Solution {
    pub fn remove_element(
        nums: &mut Vec<i32>,
        val: i32
    ) -> i32 {
        while nums.contains(&val) {
            let mut index_to_remove: usize = usize::MAX;

            for (
                index,
                num
            ) in nums.clone().into_iter().enumerate() {
                if num == val {
                    index_to_remove = index;

                    break;
                }
            }

            if index_to_remove != usize::MAX {
                nums.remove(index_to_remove);

                index_to_remove = usize::MAX;
            }
        }

        let result: i32 = nums.len().try_into().unwrap();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums: Vec<i32> = vec!(
            3, 2, 2, 3
        );

        let val: i32 = 3;

        let solution = Solution::remove_element(
            &mut nums,
            val
        );

        assert_eq!(
            solution,
            2
        );
    }

    #[test]
    fn case_2() {
        let mut nums: Vec<i32> = vec!(
            0, 1, 2, 2, 3, 0, 4, 2
        );

        let val: i32 = 2;

        let solution = Solution::remove_element(
            &mut nums,
            val
        );

        assert_eq!(
            solution,
            5
        );
    }
}
