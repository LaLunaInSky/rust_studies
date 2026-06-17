pub mod listnode;

use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut list3: Option<Box<ListNode>> = None;
        let mut values: Vec<i32> = Vec::new();

        let mut next_node_list1 = list1.clone();
        
        loop {
            match next_node_list1 {
                Some(x) => {
                    let value = x.val;

                    values.push(value);
                    
                    next_node_list1 = x.next;
                }
                None => break
            }
        }

        let mut next_node_list2 = list2.clone();

        loop {        
            match next_node_list2 {
                Some(x) => {
                    let value = x.val;

                    values.push(value);
                    
                    next_node_list2 = x.next;
                }
                None => break
            }
        }

        values.sort();
        values.reverse();

        for value in values {
            match list3.clone() {
                Some(x) => {
                    list3 = Some(Box::new(
                        ListNode {
                            val: value,
                            next: list3
                        }
                    ));
                }
                None => {
                    list3 = Some(Box::new(
                        ListNode {
                            val: value,
                            next: None
                        }
                    ));
                }
            }
        }
        
        return list3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listnode::ListNode;

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
}
