use crate::solution::Solution;

#[test]
fn case_1() {
    let s = String::from(
        "42"
    );

    let result: i32 = 42;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let s = String::from(
        " -042"
    );

    let result: i32 = -42;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let s = String::from(
        "1337c0d3"
    );

    let result: i32 = 1337;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let s = String::from(
        "0-1"
    );

    let result: i32 = 0;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_5() {
    let s = String::from(
        "words and 987"
    );

    let result: i32 = 0;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_6() {
    let s = String::from(
        "3147483660"
    );

    let result: i32 = 2147483647;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_7() {
    let s = String::from(
        "-3147483660"
    );

    let result: i32 = -2147483648;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_8() {
    let s = String::from(
        ""
    );

    let result: i32 = 0;

    let solution = Solution::my_atoi(
        s.clone()
    );

    assert_eq!(
        solution,
        result
    );
}