use crate::solution::Solution;

#[test]
fn case_1() {
    let haystack = String::from(
        "sadbutsad"
    );

    let needle = String::from(
        "sad"
    );

    let solution = Solution::str_str(
        haystack.clone(),
        needle.clone()
    );

    assert_eq!(
        solution,
        0
    );
}

#[test]
fn case_2() {
    let haystack = String::from(
        "leetcode"
    );

    let needle = String::from(
        "leeto"
    );

    let solution = Solution::str_str(
        haystack.clone(),
        needle.clone()
    );

    assert_eq!(
        solution,
        -1
    );
}