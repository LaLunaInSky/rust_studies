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
            }
        }

        let result: i32 = nums.len().try_into().unwrap();

        return result;
    }
}