use exercise_21::{
    listnode::ListNode,
    Solution
};

fn main() {
    let list1: Option<Box<ListNode>> = Some(
        Box::new(
            ListNode {
                val: -9,
                next: Some(
                    Box::new(
                        ListNode {
                            val: 3,
                            next: None
                        }
                    )
                )
            },
        )
    );

    let list2: Option<Box<ListNode>> = Some(Box::new(
        ListNode {
            val: 5,
            next: Some(Box::new(
                ListNode {
                    val: 7,
                    next: None
                }
            ))
        }
    ));

    let solution = Solution::merge_two_lists(
        list1.clone(),
        list2.clone()
    );

    println!(
        "Input: list1 = {:?}, list2 = {:?}\nOutput: {:?}",
        list1,
        list2,
        solution
    );
}