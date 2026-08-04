use crate::solution::Solution;

#[test]
fn case_1() {
    let x: i32 = 121;

    let solution = Solution::is_palindrome(x);

    assert_eq!(
        solution,
        true
    );
}

#[test]
fn case_2() {
    let x: i32 = -121;

    let solution = Solution::is_palindrome(x);

    assert_eq!(
        solution,
        false
    );
}

#[test]
fn case_3() {
    let x: i32 = 10;

    let solution = Solution::is_palindrome(x);

    assert_eq!(
        solution,
        false
    );
}