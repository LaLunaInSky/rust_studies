use crate::analyzer::get_the_arguments;

pub struct Arguments {
    query: String,
    file_path: String,
}

impl Arguments {
    pub fn build(
        args: &Vec<String>
    ) -> Arguments {
        let (query, file_path) = get_the_arguments(
            &args
        );

        Arguments {
            query, 
            file_path
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
}