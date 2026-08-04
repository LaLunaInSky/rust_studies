pub struct Solution {}

impl Solution {
    pub fn two_sum(
        nums: Vec<i32>,
        target: i32
    ) -> Vec<i32> {
        let mut return_vector: Vec<i32> = Vec::new();
        let mut index_initial: usize = 0;

        while return_vector.len() < 2 {
            if index_initial <= nums.len() -2 {
                for (
                    index, 
                    number
                ) in nums.iter().enumerate() {
                    if return_vector.len() == 0 {
                        if index == index_initial {
                            return_vector.push(
                                index.try_into().unwrap()
                            );
                        }
                    }
    
                    if index > index_initial {
                        if (nums[index_initial] + number) == target {
                            return_vector.push(
                                index.try_into().unwrap()
                            );

                            break;
                        }
                    }
                }
            
                if return_vector.len() != 2 {
                    return_vector.clear();
                    index_initial += 1;
                }
            } else {
                break;
            }
        }

        return return_vector;
    }
}
