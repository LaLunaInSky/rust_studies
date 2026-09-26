pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(
        n: i32
    ) -> Vec<String> {
        // first step
        let mut last_expected_occurrence: Vec<char> = Vec::new();
        
        for _ in 0..n {
            last_expected_occurrence.push(
                '('
            );

            last_expected_occurrence.push(
                ')'
            );
        }

        // second step
        let mut possibilities_found: Vec<Vec<char>> = Vec::new();

        let mut new_occurrence: Vec<char> = Vec::new();

        while new_occurrence != last_expected_occurrence {
            match possibilities_found.len() {
                0 => {
                    for _ in 0..n {
                        new_occurrence.push('(');
                    }

                    for occurrence in new_occurrence.clone().iter() {
                        match occurrence {
                            '(' => new_occurrence.push(')'),
                            _ => (),
                        }
                    }

                    if new_occurrence != last_expected_occurrence {
                        possibilities_found.push(new_occurrence.clone());

                        new_occurrence.clear();
                    }
                }
                _ => {
                    eprintln!(
                        "Start"
                    );

                    new_occurrence = possibilities_found[
                        possibilities_found.len() - 1
                    ].clone();

                    eprintln!(
                        "\n{:?} - new_occurrence / first occurrence\n",
                        new_occurrence
                    );

                    let mut number_of_consecutive_close_parentheses: u8 = 0;

                    let mut index_for_the_change_of_parentheses: usize = 0;

                    for (
                        index_occurrence,
                        occurrence
                    ) in new_occurrence.iter().enumerate().rev() {
                        if *occurrence == ')' {
                            number_of_consecutive_close_parentheses += 1;

                            index_for_the_change_of_parentheses = index_occurrence;
                        } else {
                            break;
                        }
                    }

                    if number_of_consecutive_close_parentheses > 1 {
                        new_occurrence[
                            index_for_the_change_of_parentheses
                        ] = '(';

                        new_occurrence[
                            index_for_the_change_of_parentheses - 1
                        ] = ')';
                    } else {
                        eprintln!(
                            "Start 2"
                        );

                        eprintln!(
                            "\n{:?} - new_occurrence / third occurrence\n",
                            new_occurrence
                        );

                        number_of_consecutive_close_parentheses = 0;

                        while number_of_consecutive_close_parentheses <= 1 {
                            number_of_consecutive_close_parentheses = 0;


                            for (
                                index_occurrence,
                                occurrence
                            ) in new_occurrence.iter().enumerate().rev() {
                                if index_occurrence <= index_for_the_change_of_parentheses - 2 {
                                    if *occurrence == ')' {
                                        number_of_consecutive_close_parentheses += 1;

                                        // eprintln!(
                                        //     "\n{} occurrence - {} index",
                                        //     occurrence,
                                        //     index_occurrence
                                        // );
                                    } else {
                                        index_for_the_change_of_parentheses = index_occurrence + 1;

                                        // eprintln!(
                                        //     "\n{} index_for_the_change_of_parentheses\n{} occurrence - {} index",
                                        //     index_for_the_change_of_parentheses,
                                        //     occurrence,
                                        //     index_occurrence
                                        // );

                                        break;
                                    }

                                    // eprintln!(
                                    //     "{} - {} index",
                                    //     occurrence,
                                    //     index_occurrence
                                    // );
                                }
                            }
                        }

                        eprintln!(
                            "\n{} number_of_consecutive_close_parentheses\n{} index_for_the_change_of_parentheses\n",
                            number_of_consecutive_close_parentheses,
                            index_for_the_change_of_parentheses
                        );

                        new_occurrence[index_for_the_change_of_parentheses] = '(';

                        new_occurrence[index_for_the_change_of_parentheses - 1] = ')';

                        let mut remaining_number_of_open_parentheses = n;

                        for (
                            index_occurrence,
                            occurrence
                        ) in new_occurrence.iter().enumerate() {
                            // eprintln!(
                            //     "{} - {} index\n{} remaining_number_of_open_parentheses\n{} index_for_the_change_of_parentheses\n",
                            //     occurrence, 
                            //     index_occurrence,
                            //     remaining_number_of_open_parentheses,
                            //     index_for_the_change_of_parentheses
                            // );

                            
                            if index_occurrence >= index_for_the_change_of_parentheses {
                                break;
                            }

                            if *occurrence == '(' {
                                remaining_number_of_open_parentheses -= 1;
                            }
                        }

                        for count in 0..remaining_number_of_open_parentheses {
                            let index_to_change: usize = (index_for_the_change_of_parentheses as i32 + count).try_into().unwrap();

                            new_occurrence[index_to_change] = '(';

                            // eprintln!(
                            //     "{} index_to_change\n{} remaining_number_of_open_parentheses\n",
                            //     index_to_change,
                            //     remaining_number_of_open_parentheses
                            // );

                            if count == remaining_number_of_open_parentheses - 1 {
                                index_for_the_change_of_parentheses = index_to_change + 1;
                            }
                        }

                        // eprintln!(
                        //     "\n{:?} - new_occurrence / fourth third occurrence\n{} index_for_the_change_of_parentheses\n",
                        //     new_occurrence,
                        //     index_for_the_change_of_parentheses
                        // );

                        for index in index_for_the_change_of_parentheses..new_occurrence.len() {
                            // eprintln!(
                            //     "{} index",
                            //     index
                            // );

                            new_occurrence[index] = ')';
                        }
                    }

                    if new_occurrence != last_expected_occurrence {
                        possibilities_found.push(
                            new_occurrence.clone()
                        );

                        new_occurrence.clear();
                    }

                    // if possibilities_found.len() >= 29 {
                    //     break;
                    // }
                }
            }
        }

        possibilities_found.push(new_occurrence.clone());

        new_occurrence.clear();

        // last step
        let mut result: Vec<String> = Vec::new();

        for found_occurrence in possibilities_found.iter() {
            let mut occurrence_string: String = String::new();

            for occurrence in found_occurrence.iter() {
                occurrence_string.push(*occurrence);
            }

            result.push(occurrence_string.clone());
        }

        result 
    }
}