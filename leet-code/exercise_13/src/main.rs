use exercise_13::Solution;

fn main() {
    // MCMXCIV

    let s = String::from(
        "MCMXCIV"
    );

    let solution = Solution::romain_to_int(s.clone());

    println!(
        "Input: {}\nOutput: {}\n",
        s, solution
    );
}
