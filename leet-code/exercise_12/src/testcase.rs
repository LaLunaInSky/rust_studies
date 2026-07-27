use crate::solution::Solution;

#[test]
fn case_1() {
    let num: i32 = 3749;

    let result = String::from(
        "MMMDCCXLIX"
    );

    let solution = Solution::in_to_romain(
        num
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let num: i32 = 58;

    let result = String::from(
        "LVIII"
    );

    let solution = Solution::in_to_romain(
        num
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let num: i32 = 1994;

    let result = String::from(
        "MCMXCIV"
    );

    let solution = Solution::in_to_romain(
        num
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let num: i32 = 3333;

    let result = String::from(
        "MMMCCCXXXIII"
    );

    let solution = Solution::in_to_romain(
        num
    );

    assert_eq!(
        solution,
        result
    );
}