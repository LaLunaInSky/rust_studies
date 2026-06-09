use exercise_1::Solution;

fn main() {
    let nums: Vec<i32> = vec![
        2, 7, 11, 15
    ];

    let target: i32 = 9;

    let solution = Solution::two_sum(
        nums, target
    );

    println!(
        "{:?}",
        solution
    )
}