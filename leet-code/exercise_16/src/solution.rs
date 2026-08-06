pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(
        nums: Vec<i32>,
        target: i32
    ) -> i32 {
        let mut result: i32 = 0;

        let mut nums = nums;

        nums.sort();

        let len_of_nums = nums.len();

        let mut shortest_distance: i32 = i32::MAX;

        if len_of_nums >= 3 {
            for first_index in 0..(len_of_nums - 2) {
                for second_index in (first_index + 1)..(len_of_nums - 1) {
                    for third_index in (second_index + 1)..len_of_nums {
                        let sum = nums[first_index] + nums[second_index] + nums[third_index];
                        
                        let mut distance_of_target = sum - target;
                        
                        if distance_of_target < 0 {
                            distance_of_target = -distance_of_target;
                        }

                        println!(
                            "{} - {}",
                            distance_of_target,
                            sum
                        );

                        if distance_of_target < shortest_distance {
                            shortest_distance = distance_of_target;
                            
                            result = sum;
                        }

                    }
                }
            }
        } else {
            for num in nums.iter() {
                result += num;
            }
        }

        result
    }
}