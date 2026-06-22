use exercise_27::Solution;

fn main() {
    let mut nums: Vec<i32> = vec!(
        3, 2, 2, 3
    );

    let nums_copy: Vec<i32> = nums.clone();

    let val: i32 = 3;

    let solution = Solution::remove_element(
        &mut nums,
        val
    );

    println!(
        "Input: num = {:?}, val = {}\nOutput: {}, nums = {:?}\n",
        nums_copy, val, solution, nums
    );
}