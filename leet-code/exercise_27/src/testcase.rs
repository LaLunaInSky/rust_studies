use crate::solution::Solution;

#[test]
fn case_1() {
    let mut nums: Vec<i32> = vec!(
        3, 2, 2, 3
    );

    let val: i32 = 3;

    let solution = Solution::remove_element(
        &mut nums,
        val
    );

    assert_eq!(
        solution,
        2
    );
}

#[test]
fn case_2() {
    let mut nums: Vec<i32> = vec!(
        0, 1, 2, 2, 3, 0, 4, 2
    );

    let val: i32 = 2;

    let solution = Solution::remove_element(
        &mut nums,
        val
    );

    assert_eq!(
        solution,
        5
    );
}