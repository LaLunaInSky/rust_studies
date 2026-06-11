pub struct Solution {}

impl Solution {
    pub fn romain_to_int(
        s: String
    ) -> i32 {
        let simbols = vec![
            'I', 'V', 'X', 'L', 'C', 'D', 'M'
        ];

        let values = vec![
            1, 5, 10, 50, 100, 500, 1000
        ];

        let mut numbers: Vec<i32> = Vec::new();

        let mut number = 0;

        for char in s.chars() {
            for (
                index,
                simbol
            ) in simbols.iter().enumerate() {
                if *simbol == char {
                    if numbers.len() >= 1 {
                        if numbers[
                            numbers.len() -1
                        ] == 1  {
                            if index == 1 || index == 2 {
                                let new_number = values[index] - numbers[
                                    numbers.len() -1
                                ];

                                numbers.pop();

                                numbers.push(
                                    new_number
                                );
                            } else {
                                numbers.push(
                                    values[index]
                                );
                            }
                        } else if numbers[
                            numbers.len() -1
                        ] == 10  {
                            if index == 3 || index == 4 {
                                let new_number = values[index] - numbers[
                                    numbers.len() -1
                                ];

                                numbers.pop();

                                numbers.push(
                                    new_number
                                );
                            } else {
                                numbers.push(
                                    values[index]
                                );
                            }
                        } else if numbers[
                            numbers.len() -1
                        ] == 100  {
                            if index == 5 || index == 6 {
                                let new_number = values[index] - numbers[
                                    numbers.len() -1
                                ];

                                numbers.pop();

                                numbers.push(
                                    new_number
                                );
                            } else {
                                numbers.push(
                                    values[index]
                                );
                            }
                        } else {
                            numbers.push(
                                values[index]
                            );
                        }
                    } else if numbers.len() == 0 {
                        numbers.push(
                            values[index]
                        );
                    }

                }
            }
        }

        for num in numbers.iter() {
            number += num;
        }

        return number;
    }
}


#[cfg(test)]
mod cases {
    use super::*;

    #[test]
    fn case_1() {
        let s = String::from(
            "III"
        );

        let solution = Solution::romain_to_int(
            s.clone()
        );

        assert_eq!(
            solution,
            3
        );
    }

    #[test]
    fn case_2() {
        let s = String::from(
            "LVIII"
        );

        let solution = Solution::romain_to_int(
            s.clone()
        );

        assert_eq!(
            solution,
            58
        );
    }

    #[test]
    fn case_3() {
        let s = String::from(
            "MCMXCIV"
        );

        let solution = Solution::romain_to_int(
            s.clone()
        );

        assert_eq!(
            solution,
            1994
        );
    }
}