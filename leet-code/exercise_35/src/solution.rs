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

        index_to_add
    }
}