use std::process;

pub fn result_display(
    result: &Vec<String>
) {
    println!(
        "Result:"
    );

    if result.len() > 0 {
        for (
            index, 
            line
        ) in result.iter().enumerate() {
            println!(
                "{line}"
            );

            if index == result.len() - 1 {
                println!();
            }
        }
    } else {
        println!(
            "NOT MATCHES FOUND\n"
        );

        process::exit(1);
    }
}