use crate::{
    solution::Solution,
    listnode::ListNode
};

fn add_in_listnode(
    listnode: &mut Option<Box<ListNode>>, 
    val: i32
) {
    
    match listnode {
        Some(n) => {
            let old_listnode = n.clone();
            
            *listnode = Some(Box::new(
                ListNode {
                    val,
                    next: Some(old_listnode)
                }
            ));
        }
        None => {
            *listnode = Some(Box::new(
                ListNode {
                    val,
                    next: None
                }
            ));
        }
    }
}

#[test]
fn case_1() {
    let head_vec: Vec<i32> = vec!(
        1, 2, 3, 4, 5
    );

    let mut head: Option<Box<ListNode>> = None;

    for num in head_vec.iter().rev() {
        add_in_listnode(
            &mut head,
            *num
        );
    }

    let n: i32 = 2;

    let result_vec: Vec<i32> = vec!(
        1, 2, 3, 5
    );

    let mut result: Option<Box<ListNode>> = None;

    for num in result_vec.iter().rev() {
        add_in_listnode(
            &mut result,
            *num
        );
    }

    let solution = Solution::remove_nth_from_end(
        head.clone(),
        n
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let head_vec: Vec<i32> = vec!(
        1
    );

    let mut head: Option<Box<ListNode>> = None;

    for num in head_vec.iter().rev() {
        add_in_listnode(
            &mut head,
            *num
        );
    }

    let n: i32 = 1;

    let result_vec: Vec<i32> = vec!();

    let mut result: Option<Box<ListNode>> = None;

    for num in result_vec.iter().rev() {
        add_in_listnode(
            &mut result,
            *num
        );
    }

    let solution = Solution::remove_nth_from_end(
        head.clone(),
        n
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let head_vec: Vec<i32> = vec!(
        1, 2
    );

    let mut head: Option<Box<ListNode>> = None;

    for num in head_vec.iter().rev() {
        add_in_listnode(
            &mut head,
            *num
        );
    }

    let n: i32 = 1;

    let result_vec: Vec<i32> = vec!(
        1
    );

    let mut result: Option<Box<ListNode>> = None;

    for num in result_vec.iter().rev() {
        add_in_listnode(
            &mut result,
            *num
        );
    }

    let solution = Solution::remove_nth_from_end(
        head.clone(),
        n
    );

    assert_eq!(
        solution,
        result
    );
}