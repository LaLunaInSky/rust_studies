use exercise_26::Solution;

fn main() {
    let mut nums: Vec<i32> = vec!(
        0, 1, 1, 2, 2
    );

    let nums_copy = nums.clone();

    let solution = Solution::remove_duplicates(
        &mut nums
    );

    println!(
        "Input: nums = {:?}\nOutput: {}, nums = {:?}",
        nums_copy,
        solution, 
        nums
    );
}