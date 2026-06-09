use exercise_9::Solution;

fn main() {
    let x: i32 = 121;

    let solution = Solution::is_palindrome(x);

    println!(
        "Input: x = {}\nOutput: {}",
        x, solution
    );
}