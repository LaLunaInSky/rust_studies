pub struct Solution {}

impl Solution {
    pub fn four_sum(
        nums: Vec<i32>,
        target: i32
    ) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut nums = nums.clone();

        nums.sort();

        let len_nums = nums.len();

        if 4 <= len_nums {
            
            for a in 0..len_nums - 3 {

                for b in a + 1..len_nums - 2 {

                    let (mut c, mut d) = (b + 1, len_nums - 1);
    
                    while c < d {
                
                        let nums_sum = vec!(
                            nums[a], nums[b], nums[c], nums[d]
                        );

                        let mut sum: i32 = 0;

                        for num in nums_sum.iter() {
                            if *num >= -10_i32.pow(9) && *num <= 10_i32.pow(9) {
                                if !sum.overflowing_add(*num).1 {
                                    sum += num;
                                }
                            } else {
                                break;
                            }
                        }
                    
                        if sum == target {
                            if !result.contains(& nums_sum) {
                                result.push(nums_sum.clone());
                            }

                            d -= 1;
                            c += 1;
                        } else if sum > target {
                            d -= 1;
                        } else {
                            c += 1;
                        }
                    }
                }
            }
        }

        result.sort();

        result
    }
}