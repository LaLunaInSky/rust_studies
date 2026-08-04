pub struct Solution {}

impl Solution {
    pub fn is_valid(
        s: String
    ) -> bool {
        let mut brackets_open: Vec<char> = Vec::new();

        for bracket in s.chars() {
            match bracket {
                '(' | '[' | '{' => brackets_open.push(bracket),
                ')' | ']' | '}' => {
                    match bracket {
                        ')' => {
                            if brackets_open.len() > 0 {
                                if brackets_open[brackets_open.len() - 1] == '(' {
                                    brackets_open.pop();
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        ']' => {
                            if brackets_open.len() > 0 {
                                if brackets_open[brackets_open.len() - 1] == '[' {
                                    brackets_open.pop();
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        '}' => {
                            if brackets_open.len() > 0 {
                                if brackets_open[brackets_open.len() - 1] == '{' {
                                    brackets_open.pop();
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        if brackets_open.len() == 0 {
            return true;
        } else {
            return false;
        }
    }
}