use crate::analyzer::get_the_arguments;

use std::env;

pub struct Arguments {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Arguments {
    pub fn build(
        args: &Vec<String>
    ) -> Arguments {
        let (query, file_path) = get_the_arguments(
            &args
        );

        let ignore_case = env::var(
            "IGNORE_CASE"
        ).is_ok();

        Arguments {
            query, 
            file_path,
            ignore_case,
        }
    }

    pub fn query(
        &self
    ) -> String {
        self.query.clone()
    }

    pub fn file_path(
        &self
    ) -> String {
        self.file_path.clone()
    }

    pub fn ignore_case(
        &self
    ) -> bool {
        self.ignore_case
    }
}