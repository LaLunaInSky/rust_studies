use crate::solution::Solution;

#[test]
fn case_1() {
    let nums: Vec<i32> = vec!(
        1, 0, -1, 0, -2, 2
    );

    let target: i32 = 0;

    let result: Vec<Vec<i32>> = vec!(
        vec!(-2, -1, 1, 2),
        vec!(-2, 0, 0, 2),
        vec!(-1, 0, 0, 1)
    );

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let nums: Vec<i32> = vec!(
        2, 2, 2, 2, 2
    );

    let target: i32 = 8;

    let result: Vec<Vec<i32>> = vec!(
        vec!(2, 2, 2, 2)
    );

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let nums: Vec<i32> = vec!(
        0
    );

    let target: i32 = 0;

    let result: Vec<Vec<i32>> = vec!();

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let nums: Vec<i32> = vec!(
        -3, -1, 0, 2, 4, 5
    );

    let target: i32 = 2;

    let result: Vec<Vec<i32>> = vec!(
        vec!(-3, -1, 2, 4)
    );

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_5() {
    let nums: Vec<i32> = vec!(
        3, 1, 4, 2, 5, -4, 2, 4, -5
    );

    let target: i32 = -12;

    let result: Vec<Vec<i32>> = vec!();

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_6() {
    let nums: Vec<i32> = vec!(
        -493,-482,-482,-456,-427,-405,-392,-385,-351,-269,-259,-251,-235,-235,-202,-201,-194,-189,-187,-186,-180,-177,-175,-156,-150,-147,-140,-122,-112,-112,-105,-98,-49,-38,-35,-34,-18,20,52,53,57,76,124,126,128,132,142,147,157,180,207,227,274,296,311,334,336,337,339,349,354,363,372,378,383,413,431,471,474,481,49
    );

    let target: i32 = 6189;

    let result: Vec<Vec<i32>> = vec!();

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_7() {
    let nums: Vec<i32> = vec!(
        -493,-470,-464,-453,-451,-446,-445,-407,-406,-393,-328,-312,-307,-303,-259,-253,-252,-243,-221,-193,-126,-126,-122,-117,-106,-105,-101,-71,-20,-12,3,4,20,20,54,84,98,111,148,149,152,171,175,176,211,218,227,331,352,389,410,420,448,485
    );

    let target: i32 = 1057;

    let result: Vec<Vec<i32>> = vec!(
        vec!(-221,410,420,448),
        vec!(-12,211,410,448),
        vec!(3,149,420,485),
        vec!(4,148,420,485),
        vec!(54,98,420,485),
        vec!(84,211,352,410),
        vec!(98,218,331,410),
        vec!(98,218,352,389),
        vec!(171,211,227,448)
    );

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_8() {
    let nums: Vec<i32> = vec!(
        1000000000, 1000000000, 1000000000, 1000000000
    );

    let target: i32 = -294967296;

    let result: Vec<Vec<i32>> = vec!();

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_9() {
    let nums: Vec<i32> = vec!(
        0,0,0,1000000000,1000000000,1000000000,1000000000
    );

    let target: i32 = 1000000000;

    let result: Vec<Vec<i32>> = vec!(
        vec!(
            0,0,0,1000000000
        )
    );

    let solution = Solution::four_sum(
        nums.clone(),
        target
    );

    assert_eq!(
        solution,
        result
    );
}