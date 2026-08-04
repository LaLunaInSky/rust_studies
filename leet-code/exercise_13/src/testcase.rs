use crate::solution::Solution;

#[test]
fn case_1() {
    let s = String::from(
        "III"
    );

    let solution = Solution::romain_to_int(
        s.clone()
    );

    assert_eq!(
        solution,
        3
    );
}

#[test]
fn case_2() {
    let s = String::from(
        "LVIII"
    );

    let solution = Solution::romain_to_int(
        s.clone()
    );

    assert_eq!(
        solution,
        58
    );
}

#[test]
fn case_3() {
    let s = String::from(
        "MCMXCIV"
    );

    let solution = Solution::romain_to_int(
        s.clone()
    );

    assert_eq!(
        solution,
        1994
    );
}