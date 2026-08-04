use crate::solution::Solution;

#[test]
fn case_1() {
    let s = String::from(
        "()"
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    assert_eq!(
        solution,
        true
    );
}

#[test]
fn case_2() {
    let s = String::from(
        "()[]{}"
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    assert_eq!(
        solution,
        true
    );
}

#[test]
fn case_3() {
    let s = String::from(
        "(]"
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    assert_eq!(
        solution,
        false
    );
}

#[test]
fn case_4() {
    let s = String::from(
        "([])"
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    assert_eq!(
        solution,
        true
    );
}

#[test]
fn case_5() {
    let s = String::from(
        "([)]"
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    assert_eq!(
        solution,
        false
    );
}

#[test]
fn case_6() {
    let s = String::from(
        "["
    );

    let solution = Solution::is_valid(
        s.clone()
    );

    assert_eq!(
        solution,
        false
    );
}