use exercise_14::Solution;

fn main() {
    // let strs: Vec<String> = vec!(
    //     String::from(
    //         "flower"
    //     ),
    //     String::from(
    //         "flow"
    //     ),
    //     String::from(
    //         "flight"
    //     )
    // );

    // let strs: Vec<String> = vec!(
    //     String::from(
    //         "dog"
    //     ),
    //     String::from(
    //         "racecar"
    //     ),
    //     String::from(
    //         "car"
    //     )
    // );

    let strs: Vec<String> = vec!(
        String::from(
            "cir"
        ),
        String::from(
            "car"
        )
    );

    let solution = Solution::longest_common_prefix(
        strs.clone()
    );

    println!(
        "Input: strs = {:?}\nOutput: {}",
        strs,
        solution
    );
}
