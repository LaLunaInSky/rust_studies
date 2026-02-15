use crate::{
    research,
    display
};

use std::{
    fs,
    process
};

pub fn read_the_file_path_and_pass_the_query(
    query: String,
    file_path: String,
    ignore_case: bool,
) {    
    let contents = fs::read_to_string(
        file_path
    );

    match contents {
        Ok(content) => {
            let results_content = research::search_the_query_in_content(
                &query,
                &content,
                ignore_case
            );

            display::result_display(
                &results_content
            );
        }
        Err(_) => {
            println!(
                "Error: The file does not exist or could not be read\n"
            );

            process::exit(1);
        }
    }
}