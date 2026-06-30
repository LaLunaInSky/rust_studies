use crate::solution::Solution;

#[test]
fn case_1() {
    let x: i32 = 123;

    let result: i32 = 321;

    let solution = Solution::reverse(
        x
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let x: i32 = -123;

    let result: i32 = -321;

    let solution = Solution::reverse(
        x
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let x: i32 = 120;

    let result: i32 = 21;

    let solution = Solution::reverse(
        x
    );

    assert_eq!(
        solution,
        result
    );
}