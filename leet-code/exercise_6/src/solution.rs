pub struct Solution {}

impl Solution {
    pub fn convert(
        s: String,
        num_rows: i32
    ) -> String {
        let mut matriz: Vec<Vec<Option<usize>>> = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        let len_chars = chars.len();
        let mut index: usize = 0;
        let mut next_increment: usize = 0;

        for _ in 0..num_rows {
            matriz.push(
                Vec::new()
            );
        }

        loop {
            for index_matriz in 0..num_rows as usize {
                if next_increment ==  0{
                    matriz[index_matriz].push(
                        Some(index)
                    );

                    index += 1;
                } else {
                    if index_matriz == next_increment {
                        matriz[index_matriz].push(
                            Some(index)
                        );

                        index += 1;
                    } else {
                        matriz[index_matriz].push(
                            None
                        );
                    }
                }
            }

            if index < len_chars {
                if next_increment == 0 {
                    if num_rows > 1 {
                        next_increment = num_rows as usize - 2;
                    }

                } else {
                    next_increment -= 1;
                }
            } else {
                break;
            }
        }

        let mut result = String::new();

        for line in matriz.iter() {
            for content in line.iter() {
                match content {
                    Some(x) => {
                        if *x < len_chars {
                            result.push(
                                chars[*x]
                            );
                        }
                    }
                    None => continue,
                }
            }
        }

        return result;
    }
}