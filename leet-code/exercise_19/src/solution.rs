use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(
        head: Option<Box<ListNode>>,
        n: i32
    ) -> Option<Box<ListNode>> {
        // first function
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

        // second function
        fn created_vec_from_the_listnode(
            listnode: &Option<Box<ListNode>>
        ) -> Vec<i32> {
            let mut result_vec: Vec<i32> = Vec::new();

            let mut next_node = listnode.clone();

            loop {
                match next_node {
                    Some(ref node) => {
                        let value = node.val;

                        result_vec.push(value);

                        next_node = next_node.unwrap().next;
                    }
                    None => break
                }
            }

            result_vec
        }

        // third function
        fn remove_nth_from_list(
            list: &mut Vec<i32>,
            n: i32
        ) {
            if list.len() >= n as usize {
                if n > 0 {
                    list.remove(n as usize - 1);
                }
            }
        }

        let mut result: Option<Box<ListNode>> = None;

        let mut nums = created_vec_from_the_listnode(
            &head
        );

        nums.reverse();

        remove_nth_from_list(
            &mut nums, 
            n
        );

        for num in nums.iter() {
            add_in_listnode(
                &mut result,
                *num
            );
        }

        result
    }
}