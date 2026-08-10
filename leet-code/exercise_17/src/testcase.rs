use crate::solution::Solution;

#[test]
fn case_1() {
    let digits: String = String::from(
        "23"
    );

    let result: Vec<String> = vec!(
        String::from("ad"),
        String::from("ae"),
        String::from("af"),
        String::from("bd"),
        String::from("be"),
        String::from("bf"),
        String::from("cd"),
        String::from("ce"),
        String::from("cf")
    );

    let solution = Solution::letter_combinations(
        digits.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let digits: String = String::from(
        "2"
    );

    let result: Vec<String> = vec!(
        String::from("a"),
        String::from("b"),
        String::from("c")
    );

    let solution = Solution::letter_combinations(
        digits.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let digits: String = String::from(
        "3"
    );

    let result: Vec<String> = vec!(
        String::from("d"),
        String::from("e"),
        String::from("f")
    );

    let solution = Solution::letter_combinations(
        digits.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let digits: String = String::from(
        "234"
    );

    let result: Vec<String> = vec!(
        String::from("adg"),
        String::from("adh"),
        String::from("adi"),
        String::from("aeg"),
        String::from("aeh"),
        String::from("aei"),
        String::from("afg"),
        String::from("afh"),
        String::from("afi"),
        String::from("bdg"),
        String::from("bdh"),
        String::from("bdi"),
        String::from("beg"),
        String::from("beh"),
        String::from("bei"),
        String::from("bfg"),
        String::from("bfh"),
        String::from("bfi"),
        String::from("cdg"),
        String::from("cdh"),
        String::from("cdi"),
        String::from("ceg"),
        String::from("ceh"),
        String::from("cei"),
        String::from("cfg"),
        String::from("cfh"),
        String::from("cfi")
    );

    let solution = Solution::letter_combinations(
        digits.clone()
    );

    assert_eq!(
        solution,
        result
    );
}