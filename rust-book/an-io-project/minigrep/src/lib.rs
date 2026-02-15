pub mod analyzer;
pub mod reader;
pub mod arguments;
pub mod research;
pub mod display;

use reader::read_the_file_path_and_pass_the_query;
use arguments::Arguments;

pub fn run_minigrep(
    args: &Vec<String>
) {
    let arguments = Arguments::build(
        &args
    );

    read_the_file_path_and_pass_the_query(
        arguments.query(),
        arguments.file_path()
    );
}