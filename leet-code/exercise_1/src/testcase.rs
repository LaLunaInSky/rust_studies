use crate::solution::Solution;

#[test]
fn case_1() {
    let nums: Vec<i32> = vec![
        2, 7, 11, 15
    ];

    let target: i32 = 9;

    let solution = Solution::two_sum(
        nums, target
    );

    assert_eq!(
        solution,
        vec![0, 1]
    );
}

#[test]
fn case_2() {
    let nums: Vec<i32> = vec![
        3, 2, 4
    ];

    let target: i32 = 6;

    let solution = Solution::two_sum(
        nums, target
    );

    assert_eq!(
        solution,
        vec![1, 2]
    );
}

#[test]
fn case_3() {
    let nums: Vec<i32> = vec![
        3, 3
    ];

    let target: i32 = 6;

    let solution = Solution::two_sum(
        nums, target
    );

    assert_eq!(
        solution,
        vec![0, 1]
    );
}