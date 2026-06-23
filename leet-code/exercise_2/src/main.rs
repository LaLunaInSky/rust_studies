use exercise_2::{
    listnode::ListNode,
    Solution
};

fn main() {
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

    let solution = Solution::add_two_numbers(
        l1.clone(),
        l2.clone()
    );

    println!(
        "Input: l1 = {:?}, l2 = {:?}\nOutput: {:?}",
        l1, l2, solution
    );
}