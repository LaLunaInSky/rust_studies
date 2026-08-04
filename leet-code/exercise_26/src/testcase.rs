use crate::solution::Solution;

#[test]
fn case_1() {
    let mut nums: Vec<i32> = vec!(
        1, 1, 2
    );

    let solution = Solution::remove_duplicates(
        &mut nums
    );

    assert_eq!(
        solution,
        2
    );
}

#[test]
fn case_2() {
    let mut nums: Vec<i32> = vec!(
        0, 0, 1, 1, 1, 2, 2, 3, 3, 4
    );

    let solution = Solution::remove_duplicates(
        &mut nums
    );

    assert_eq!(
        solution,
        5
    );
}