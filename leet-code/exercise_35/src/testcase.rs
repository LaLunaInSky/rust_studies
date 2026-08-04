use crate::solution::Solution;

#[test]
fn case_1() {
    let nums: Vec<i32> = vec!(
        1, 3, 5, 6
    );

    let target: i32 = 5;

    let solution = Solution::search_insert(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        2
    );
}

#[test]
fn case_2() {
    let nums: Vec<i32> = vec!(
        1, 3, 5, 6
    );

    let target: i32 = 2;

    let solution = Solution::search_insert(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        1
    );
}

#[test]
fn case_3() {
    let nums: Vec<i32> = vec!(
        1, 3, 5, 6
    );

    let target: i32 = 7;

    let solution = Solution::search_insert(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        4
    );
}

#[test]
fn case_4() {
    let nums: Vec<i32> = vec!(
        1, 3, 5, 6
    );

    let target: i32 = 0;

    let solution = Solution::search_insert(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        0
    );
}