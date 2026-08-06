use crate::solution::Solution;

#[test]
fn case_1() {
    let nums: Vec<i32> = vec!(
        -1, 2, 1, -4
    );

    let target: i32 = 1;

    let result: i32 = 2;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let nums: Vec<i32> = vec!(
        0, 0, 0
    );

    let target: i32 = 1;

    let result: i32 = 0;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let nums: Vec<i32> = vec!(
        5, 0
    );

    let target: i32 = 1;

    let result: i32 = 5;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let nums: Vec<i32> = vec!(
        10, 20, 30, 40, 50, 60, 70, 80, 90
    );

    let target: i32 = 1;

    let result: i32 = 60;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_5() {
    let nums: Vec<i32> = vec!(
        4, 0, 5, -5, 3, 3, 0, -4, -5
    );

    let target: i32 = -2;

    let result: i32 = -2;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_6() {
    let nums: Vec<i32> = vec!(
        558,316,-411,160,801,568,-124,-589,32,897,-33,-767,-528,-180,916,813,351,642,-373,-919,666,973,-165,831,-67,-934,-659,-18,273,201,728,988,-926,409,-573,575,-502,745,724,989,-464,903,975,980,824,-197,-261,-761,966,799,-379,96,9,-680,-15,476,220,-647,365,518,-479,-443,337,-364,968,-617,862,-281,-936,-526,196,829,-191,643,-473,557,-870,553,-506,774,784,-344,-452,510,219,-785,-1,711,-759,-830,10,612,-450,-784,53,976,-314,-908,463,-408,-846,261,689,-856,-687,-949,-163,-621,-233,847,-682,-805,-711,286,40,-831,-12,952,-878,-226,739,11,-342,74,-933,-770,-840,265,702,572,-453,-295,704,-249,-835,191,404,984,-820,321,632,-689,285,-877,-643,-451,-625,84,889,620,-658,861,-397,-952,695,-386,31,827,-539,-350,588,846,-142,314,909,937,625,-230,-553,403,-763,413,904,-994,272,-426,104,-715,-159,-270,716,819,806,891,-47,-100,440,-339,918,-577,508,-554,-478,120,-943,25,-600,-626,336,-567,473,531,195,-259,-267,-883,450,170,733,491,602
    );

    let target: i32 = -8224;

    let result: i32 = -2895;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_7() {
    let nums: Vec<i32> = vec!(
        0,3,97,102,200
    );

    let target: i32 = 300;

    let result: i32 = 300;

    let solution = Solution::three_sum_closest(
        nums.clone(),
        target.clone()
    );

    assert_eq!(
        solution,
        result
    );
}