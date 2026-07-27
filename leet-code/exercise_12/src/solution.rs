pub struct Solution {}

impl Solution {
    pub fn in_to_romain(
        num: i32
    ) -> String {
        let mut result = String::new();

        let mut nums: Vec<i32> = Vec::new();

        for (
            index,
            num_char
        ) in num.to_string().chars().rev().enumerate() {
            match index {
                0 => {
                    let number = num_char.to_string().parse::<i32>().unwrap();

                    nums.push(
                        number
                    );
                }
                1 => {
                    let number = num_char.to_string().parse::<i32>().unwrap() * 10;

                    nums.push(
                        number
                    );
                }
                2 => {
                    let number = num_char.to_string().parse::<i32>().unwrap() * 100;
 
                    nums.push(
                        number
                    );
                }
                3 => {
                    let number = num_char.to_string().parse::<i32>().unwrap() * 1000;

                    nums.push(
                        number
                    );
                }
                _ => (),
            }
        }

        fn convert_integer_to_romain(
            integer: &i32,
            romain_string: &mut String
        ) {
            let mut wich_unit_is: Vec<bool> = vec!(
                false, false, false, false
            );

            let mut number: i32 = *integer;

            if *integer % 1000 == 0 {
                number = number / 1000;

                wich_unit_is[0] = true;
            } else if *integer % 100 == 0 {
                number = number / 100;

                wich_unit_is[1] = true;
            } else if *integer % 10 == 0 {
                number = number / 10;

                wich_unit_is[2] = true;
            } else {
                wich_unit_is[3] = true;
            }
            
            if number == 9 || number == 4 {
                match number {
                    9 => {
                        if wich_unit_is[1] {
                            *romain_string += &String::from("CM");
                        } else if wich_unit_is[2] {
                            *romain_string += &String::from("XC");
                        } else if wich_unit_is[3] {
                            *romain_string += &String::from("IX");
                        }
                    },
                    4 => {
                        if wich_unit_is[1] {
                            *romain_string += &String::from("CD");
                        } else if wich_unit_is[2] {
                            *romain_string += &String::from("XL");
                        } else if wich_unit_is[3] {
                            *romain_string += &String::from("IV");
                        }
                    },
                    _ => (),
                }
                
            } else if number == 5 {
                if wich_unit_is[1] {
                    *romain_string += &String::from("D");
                } else if wich_unit_is[2] {
                    *romain_string += &String::from("L");
                } else if wich_unit_is[3] {
                    *romain_string += &String::from("V");
                }

            } else if number > 5 {
                let count_to_i = number - 5;

                if wich_unit_is[1] {
                    *romain_string += &String::from("D");
                } else if wich_unit_is[2] {
                    *romain_string += &String::from("L");
                } else if wich_unit_is[3] {
                    *romain_string += &String::from("V");
                }

                for _ in 0..count_to_i {
                    if wich_unit_is[1] {
                        *romain_string += &String::from("C");
                    } else if wich_unit_is[2] {
                        *romain_string += &String::from("X");
                    } else if wich_unit_is[3] {
                        *romain_string += &String::from("I");
                    }
                }

            } else {
                for _ in 0..number {
                    if wich_unit_is[0] {
                        *romain_string += &String::from("M");
                    } else if wich_unit_is[1] {
                        *romain_string += &String::from("C");
                    } else if wich_unit_is[2] {
                        *romain_string += &String::from("X");
                    } else if wich_unit_is[3] {
                        *romain_string += &String::from("I");
                    }
                }
            }
        }

        for number in nums.iter().rev() {
            convert_integer_to_romain(
                number,
                &mut result
            );
        }

        return result;
    }
}