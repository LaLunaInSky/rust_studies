use std::env;

use minigrep::run_minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    run_minigrep(
        &args
    );
}