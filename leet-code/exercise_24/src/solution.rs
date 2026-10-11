use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(
        head: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut numbers_found: Vec<i32> = Vec::new();
        
        head.clone().map(
            |mut node| {
                loop {
                    numbers_found.push(
                        node.val
                    );

                    match node.next {
                        Some(value) => {
                            node = value;
                        }
                        None => break,
                    }
                }
            }
        );

        eprintln!(
            "numbers_found: {:?}",
            numbers_found
        );

        numbers_found.clone().into_iter().enumerate().for_each(
            |(
                index,
                number
            )| {
                if index % 2 == 1 {
                    let number_backup = number;

                    numbers_found[index] = numbers_found[index - 1];

                    numbers_found[index - 1] = number_backup;
                }
            }
        );

        eprintln!(
            "numbers_found: {:?}",
            numbers_found
        );

        let mut result: Option<Box<ListNode>> = None;

        numbers_found.clone().into_iter().for_each(
            |number| {
                match result {
                    Some(ref mut node) => {
                        eprintln!(
                            "\nnode: {:?}",
                            node
                        );

                        match node.next.take() {
                            None => {
                                node.next = Some(
                                    Box::new(
                                        ListNode::new(
                                            number
                                        )
                                    )
                                );
                            }
                            Some(ref next_node) => {
                                next_node = Som
                            },
                        }
                    }
                    None => {
                        result = Some(
                            Box::new(
                                ListNode::new(
                                    number
                                )
                            )
                        );
                    }
                }
            }
        );
        
        eprintln!(
            "result: {:?}",
            result
        );

        result
    }
}