use crate::{
    listnode::ListNode,
    solution::Solution,
};

fn created_a_listnode_based_on_an_vector(
    numbers: &Vec<i32>
) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = None;

    for number in numbers.iter().rev() {
        match result {
            Some(ref _value) => { 
                result = Some(
                    Box::new(
                        ListNode {
                            val: * number,
                            next: result
                        }
                    )
                );
            }
            None => {
                result = Some(
                    Box::new(
                        ListNode::new(*number)
                    )
                );

            }
        }  
    }

    result
}

#[test]
fn case_1() {
    let head: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![
            1, 2, 3, 4
        ]
    );

    let solution: Option<Box<ListNode>> = Solution::swap_pairs(
        head.clone()
    );

    let expected_result: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![
            2, 1, 4, 3
        ]
    );

    assert_eq!(
        solution,
        expected_result
    );
}

#[test]
fn case_2() {
    let head: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![]
    );

    let solution: Option<Box<ListNode>> = Solution::swap_pairs(
        head.clone()
    );

    let expected_result: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![]
    );

    assert_eq!(
        solution,
        expected_result
    );
}

#[test]
fn case_3() {
    let head: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![
            1
        ]
    );

    let solution: Option<Box<ListNode>> = Solution::swap_pairs(
        head.clone()
    );

    let expected_result: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![
            1
        ]
    );

    assert_eq!(
        solution,
        expected_result
    );
}

#[test]
fn case_4() {
    let head: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![
            1, 2, 3
        ]
    );

    let solution: Option<Box<ListNode>> = Solution::swap_pairs(
        head.clone()
    );

    let expected_result: Option<Box<ListNode>> = created_a_listnode_based_on_an_vector(
        &vec![
            2, 1, 3
        ]
    );

    assert_eq!(
        solution,
        expected_result
    );
}