use crate::listnode::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut l3: Option<Box<ListNode>> = None;

        let lists: Vec<Option<Box<ListNode>>> = vec!(
            l1.clone(),
            l2.clone()
        );

        let mut numbers: Vec<Vec<i32>> = Vec::new();

        for list in lists.iter() {
            let mut next_node = list.clone();

            let mut nums: Vec<i32> = Vec::new();

            loop {
                match next_node {
                    Some(ref x) => {
                        let value = x.val;
                    
                        nums.push(value);
    
                        next_node = next_node.unwrap().next;
                    }
                    None => break
                }
            }

            numbers.push(
                nums.clone()
            );
        }

        let mut last_index: Vec<usize> = vec!(0,0);
        let mut number_to_add: i32 = 0;
        let mut rest: i32 = 0;
        
        numbers.push(Vec::new());
        
        loop {
            for (
                index,
                list_number
            ) in numbers.clone().iter().enumerate() {
                if index < 2 {
                    for (
                        index_num,
                        num
                    ) in list_number.iter().enumerate() {
                        if index_num < list_number.len() - 1 {
                            if index_num == last_index[index] {
                                number_to_add += num;
                                
                                last_index[index] = index_num + 1;
    
                                break;
                            }
                        } else {
                            if index_num == last_index[index] {
                                number_to_add += num;
                                
                                last_index[index] = index_num + 1;
    
                                break;
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    number_to_add += rest;
                    
                    let number = number_to_add;

                    rest = 0;
                    // number_to_add = 0;

                    if number > 9 {
                        number_to_add = number % 10;
                     
                        rest = number / 10;
                    } else {
                        number_to_add = number;
                    }
                }
            }
    
            numbers[2].push(number_to_add);

            number_to_add = 0;
        
            if last_index[0] == numbers[0].len() && last_index[1] == numbers[1].len() {
                if rest > 0 {
                    numbers[2].push(rest);
                }

                break;
            }
        }

        numbers[2].reverse();

        for (
            index,
            num
        ) in numbers[2].clone().into_iter().enumerate() {
            if index == 0 {
                l3 = Some(Box::new(
                    ListNode {
                        val: num as i32,
                        next: None
                    }
                ));
            } else {
                l3 = Some(Box::new(
                    ListNode {
                        val: num as i32,
                        next: l3
                    }
                ));
            }
        }        

        return l3;
    }
}