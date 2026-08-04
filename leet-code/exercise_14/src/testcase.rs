use crate::solution::Solution;

#[test]
fn case_1() {
    let strs: Vec<String> = vec!(
        String::from(
            "flower"
        ),
        String::from(
            "flow"
        ),
        String::from(
            "flight"
        )
    );

    let solution = Solution::longest_common_prefix(
        strs.clone()
    );

    assert_eq!(
        solution,
        "fl"
    );
}

#[test]
fn case_2() {
    let strs: Vec<String> = vec!(
        String::from(
            "dog"
        ),
        String::from(
            "racecar"
        ),
        String::from(
            "car"
        )
    );

    let solution = Solution::longest_common_prefix(
        strs.clone()
    );

    assert_eq!(
        solution,
        ""
    );
}

#[test]
fn case_3() {
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

    assert_eq!(
        solution,
        "c"
    );
}