pub struct Solution {}

impl Solution {
    pub fn three_sum(
        nums: Vec<i32>
    ) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut first_index: usize = 0;
        let mut second_index: usize = 1;
        let mut third_index: usize = 2;

        while first_index < nums.len() - 2 {
            while second_index < nums.len() - 1 {
                while third_index < nums.len() {
                    let sum = nums[first_index] + nums[second_index] + nums[third_index];
        
                    if sum == 0 {
                        let mut nums_to_add: Vec<i32> = vec!(
                            nums[first_index], nums[second_index], nums[third_index]
                        ); 

                        nums_to_add.sort();
                        
                        if result.contains(&nums_to_add) == false {
                            result.push(
                                nums_to_add
                            );
                        }

                    }
    
                    third_index += 1;
                }

                second_index += 1;
                third_index = second_index + 1;    
            }

            first_index += 1;
            second_index = first_index + 1;
            third_index = second_index + 1;
        }

        result.sort();

        result
    }
}