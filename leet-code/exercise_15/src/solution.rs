pub struct Solution {}

impl Solution {
    pub fn three_sum(
        nums: Vec<i32>
    ) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut nums = nums;
        
        nums.sort();

        let len_nums = nums.len();

        for first_index in 0..len_nums {
            let first_num = nums[first_index];

            if first_num > 0 {
                break;
            }

            let (
                mut left_index,
                mut right_index
            ) = (
                first_index + 1,
                len_nums - 1
            );

            while left_index < right_index {   
                let sum = first_num + nums[left_index] + nums[right_index];

                if sum == 0 {
                    let vec_sum: Vec<i32> = vec!(
                        first_num, 
                        nums[left_index],
                        nums[right_index]
                    );

                    if !result.contains(&vec_sum) {
                        result.push(vec_sum);
                    }

                    left_index += 1;
                    right_index -= 1;
                } else if sum > 0 {
                    right_index -= 1;
                } else {
                    left_index += 1;
                }
            }
        }

        result
    }
}