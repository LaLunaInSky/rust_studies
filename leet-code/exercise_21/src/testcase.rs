use crate::{
    listnode::ListNode,
    solution::Solution
};

#[test]
fn case_1() {
    let list1: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode {
                            val: 4,
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let list2: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 3,
                    next: Some(Box::new(
                        ListNode {
                            val: 4,
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let result: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 1,
                    next: Some(Box::new(
                        ListNode {
                            val: 2,
                            next: Some(Box::new(
                                ListNode {
                                    val: 3,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 4,
                                            next: Some(Box::new(
                                                ListNode {
                                                    val: 4,
                                                    next: None
                                                }
                                            ))
                                        }
                                    ))
                                }
                            ))
                        }
                    ))
                }
            ))
        }
    ));

    let solution = Solution::merge_two_lists(
        list1.clone(),
        list2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let list1: Option<Box<ListNode>> = None;

    let list2: Option<Box<ListNode>> = None;

    let result: Option<Box<ListNode>> = None;

    let solution = Solution::merge_two_lists(
        list1.clone(),
        list2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let list1: Option<Box<ListNode>> = None;

    let list2: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 0,
            next: None
        }
    ));

    let result: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 0,
            next: None
        }
    ));

    let solution = Solution::merge_two_lists(
        list1.clone(),
        list2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}