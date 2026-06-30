use crate::solution::Solution;

#[test]
fn case_1() {
    let s = String::from(
        "PAYPALISHIRING"
    );

    let num_rows: i32 = 3;

    let result = String::from(
        "PAHNAPLSIIGYIR"
    );

    let solution = Solution::convert(
        s.clone(),
        num_rows
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let s = String::from(
        "PAYPALISHIRING"
    );

    let num_rows: i32 = 4;

    let result = String::from(
        "PINALSIGYAHRPI"
    );

    let solution = Solution::convert(
        s.clone(),
        num_rows
    );
    
    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let s = String::from(
        "A"
    );

    let num_rows: i32 = 1;

    let result = String::from(
        "A"
    );

    let solution = Solution::convert(
        s.clone(),
        num_rows
    );
    
    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let s = String::from(
        "AB"
    );

    let num_rows: i32 = 1;

    let result = String::from(
        "AB"
    );

    let solution = Solution::convert(
        s.clone(),
        num_rows
    );
    
    assert_eq!(
        solution,
        result
    );
}