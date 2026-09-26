use crate::solution::Solution;

#[test]
fn case_1() {
    let n: i32 = 1;
    
    let result: Vec<String> = vec!(
        String::from("()")
    );

    let solution = Solution::generate_parenthesis(
        n
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let n: i32 = 2;
    
    let result: Vec<String> = vec!(
        String::from("(())"),
        String::from("()()")
    );

    let solution = Solution::generate_parenthesis(
        n
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let n: i32 = 3;
    
    let result: Vec<String> = vec!(
        String::from("((()))"),
        String::from("(()())"),
        String::from("(())()"),
        String::from("()(())"),
        String::from("()()()")
    );

    let solution = Solution::generate_parenthesis(
        n
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let n: i32 = 4;
    
    let result: Vec<String> = vec!(
        String::from("(((())))"),
        String::from("((()()))"),
        String::from("((())())"),
        String::from("((()))()"),
        String::from("(()(()))"),
        String::from("(()()())"),
        String::from("(()())()"),
        String::from("(())(())"),
        String::from("(())()()"),
        String::from("()((()))"),
        String::from("()(()())"),
        String::from("()(())()"),
        String::from("()()(())"),
        String::from("()()()()")
    );

    let solution = Solution::generate_parenthesis(
        n
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_5() {
    let n: i32 = 5;
    
    let result: Vec<String> = vec!(
        String::from("((((()))))"),
        String::from("(((()())))"),
        String::from("(((())()))"),
        String::from("(((()))())"),
        String::from("(((())))()"),
        String::from("((()(())))"),
        String::from("((()()()))"),
        String::from("((()())())"),
        String::from("((()()))()"),
        String::from("((())(()))"),
        String::from("((())()())"),
        String::from("((())())()"),
        String::from("((()))(())"),
        String::from("((()))()()"),
        String::from("(()((())))"),
        String::from("(()(()()))"),
        String::from("(()(())())"),
        String::from("(()(()))()"),
        String::from("(()()(()))"),
        String::from("(()()()())"),
        String::from("(()()())()"),
        String::from("(()())(())"),
        String::from("(()())()()"),
        String::from("(())((()))"),
        String::from("(())(()())"),
        String::from("(())(())()"),
        String::from("(())()(())"),
        String::from("(())()()()"),
        String::from("()(((())))"),
        String::from("()((()()))"),
        String::from("()((())())"),
        String::from("()((()))()"),
        String::from("()(()(()))"),
        String::from("()(()()())"),
        String::from("()(()())()"),
        String::from("()(())(())"),
        String::from("()(())()()"),
        String::from("()()((()))"),
        String::from("()()(()())"),
        String::from("()()(())()"),
        String::from("()()()(())"),
        String::from("()()()()()")
    );

    let solution = Solution::generate_parenthesis(
        n
    );

    assert_eq!(
        solution,
        result
    );
}