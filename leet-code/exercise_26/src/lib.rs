pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(
        nums: &mut Vec<i32>
    ) -> i32 {
        let mut last_num_and_index_to_nums: Vec<i32> = vec!(
            i32::MAX,
            i32::MAX
        );
        
        let mut index_to_remove: usize = usize::MAX;

        'main_loop: loop {
            for (
                index,
                num
            ) in nums.iter().enumerate() {
                if last_num_and_index_to_nums[1] == i32::MAX {
                    last_num_and_index_to_nums = vec!(
                        *num, index.try_into().unwrap()
                    );

                    break;
                } else if index >= last_num_and_index_to_nums[1].try_into().unwrap() {
                    if last_num_and_index_to_nums[0] != *num {
                        last_num_and_index_to_nums = vec!(
                            *num, index.try_into().unwrap()
                        );   
                        
                        index_to_remove = usize::MAX;

                        break;   
                    } else {
                        if index > last_num_and_index_to_nums[1].try_into().unwrap() {
                            index_to_remove = index;

                            break;
                        } else {
                            if index == nums.len() - 1 {
                                break 'main_loop;
                            }
                        }
                    }
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
            1, 1, 2
        );

        let solution = Solution::remove_duplicates(
            &mut nums
        );

        assert_eq!(
            solution,
            2
        );
    }

    #[test]
    fn case_2() {
        let mut nums: Vec<i32> = vec!(
            0, 0, 1, 1, 1, 2, 2, 3, 3, 4
        );

        let solution = Solution::remove_duplicates(
            &mut nums
        );

        assert_eq!(
            solution,
            5
        );
    }
}
