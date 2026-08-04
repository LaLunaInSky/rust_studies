use crate::solution::Solution;
use crate::listnode::ListNode;

#[test]
fn case_1() {
    let l1: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 2,
            next: Some(Box::new(
                ListNode {
                    val: 4,
                    next: Some(Box::new(
                        ListNode {
                            val: 3,
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let l2: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 5,
            next: Some(Box::new(
                ListNode {
                    val: 6,
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
            val: 7,
            next: Some(Box::new(
                ListNode {
                    val: 0,
                    next: Some(Box::new(
                        ListNode {
                            val: 8,
                            next: None
                        }
                    ))
                }
            ))
        }
    ));

    let solution = Solution::add_two_numbers(
        l1.clone(),
        l2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_2() {
    let l1: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 0,
            next: None
        }
    ));

    let l2: Option<Box<ListNode>> = Some(Box::new(
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

    let solution = Solution::add_two_numbers(
        l1.clone(),
        l2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_3() {
    let l1: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 9,
            next: Some(Box::new(
                ListNode {
                    val: 9,
                    next: Some(Box::new(
                        ListNode {
                            val: 9,
                            next: Some(Box::new(
                                ListNode {
                                    val: 9,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 9,
                                            next: Some(Box::new(
                                                ListNode {
                                                    val: 9,
                                                    next: Some(Box::new(
                                                        ListNode {
                                                            val: 9,
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
            ))
        }
    ));

    let l2: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 9,
            next: Some(Box::new(
                ListNode {
                    val: 9,
                    next: Some(Box::new(
                        ListNode {
                            val: 9,
                            next: Some(Box::new(
                                ListNode {
                                    val: 9,
                                    next: None
                                }
                            ))
                        }
                    ))
                }
            ))
        }
    ));

    let result: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 8,
            next: Some(Box::new(
                ListNode {
                    val: 9,
                    next: Some(Box::new(
                        ListNode {
                            val: 9,
                            next: Some(Box::new(
                                ListNode {
                                    val: 9,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 0,
                                            next: Some(Box::new(
                                                ListNode {
                                                    val: 0,
                                                    next: Some(Box::new(
                                                        ListNode {
                                                            val: 0,
                                                            next: Some(Box::new(
                                                                ListNode {
                                                                    val: 1,
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
                    ))
                }
            ))
        }
    ));

    let solution = Solution::add_two_numbers(
        l1.clone(),
        l2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}

#[test]
fn case_4() {
    let l1: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 9,
            next: None,
        }
    ));

    let l2: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 9,
                    next: Some(Box::new(
                        ListNode {
                            val: 9,
                            next: Some(Box::new(
                                ListNode {
                                    val: 9,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 9,
                                            next: Some(Box::new(
                                                ListNode {
                                                    val: 9,
                                                    next: Some(Box::new(
                                                        ListNode {
                                                            val: 9,
                                                            next: Some(Box::new(
                                                                ListNode {
                                                                    val: 9,
                                                                    next: Some(Box::new(
                                                                        ListNode {
                                                                            val: 9,
                                                                            next: Some(Box::new(
                                                                                ListNode {
                                                                                    val: 9,
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
                                    ))
                                }
                            ))
                        }
                    ))
                }
            ))
        }
    ));

    let result: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 0,
            next: Some(Box::new(
                ListNode {
                    val: 0,
                    next: Some(Box::new(
                        ListNode {
                            val: 0,
                            next: Some(Box::new(
                                ListNode {
                                    val: 0,
                                    next: Some(Box::new(
                                        ListNode {
                                            val: 0,
                                            next: Some(Box::new(
                                                ListNode {
                                                    val: 0,
                                                    next: Some(Box::new(
                                                        ListNode {
                                                            val: 0,
                                                            next: Some(Box::new(
                                                                ListNode {
                                                                    val: 0,
                                                                    next: Some(Box::new(
                                                                        ListNode {
                                                                            val: 0,
                                                                            next: Some(Box::new(
                                                                                ListNode {
                                                                                    val: 0,
                                                                                    next: Some(Box::new(
                                                                                        ListNode {
                                                                                            val: 1,
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

    let solution = Solution::add_two_numbers(
        l1.clone(),
        l2.clone()
    );

    assert_eq!(
        solution,
        result
    );
}