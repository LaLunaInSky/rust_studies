pub struct Solution {}

impl Solution {
    pub fn max_area(
        height: Vec<i32>
    ) -> i32 {
        let height_len: usize = height.len();
        
        let mut largest_area_found: i32 = 0;
        
        let mut left_point: usize = 0;
        let mut right_point:usize = height_len - 1;

        while left_point < right_point {
            let base = right_point - left_point;

            let mut height_to_calculate = 0;

            if height[left_point] < height[right_point] {
                height_to_calculate = height[left_point];
            } else {
                height_to_calculate = height[right_point];
            }

            let area = base as i32 * height_to_calculate;

            if area > largest_area_found {
                largest_area_found = area;
            }

            if height[left_point] < height[right_point] {
                left_point += 1;
            } else {
                right_point -= 1;
            }
        }
        return largest_area_found;
    }
}