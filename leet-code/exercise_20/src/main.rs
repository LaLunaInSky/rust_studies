use exercise_20::Solution;

fn main() {
    let s = String::from(
        "]"
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    println!(
        "Input: {}\nOutput: {}\n",
        s, solution
    );
}