use std::process;

fn check_if_have_all_the_arguments(
    args: &Vec<String>
) -> Result<(), &'static str> {    
    if args.len() == 1 {
        return Err(
            "Error: The \'QUERY\' and \'FILE\' arguments are missing"
        );
    } else if args.len() == 2 {
        return Err(
            "Error: The \'FILE\' argument is missing"
        );
    } else if args.len() > 3 {
        return Err(
            "Error: There are more arguments than necessary"
        );
    }

    Ok(())
}

fn get_the_query(
    args: &Vec<String>
) -> String {
    let query = String::from(
        &args[1]
    );

    println!(
        "\nSearching for {query}"
    );

    query
}

fn get_the_file_path(
    args: &Vec<String>
) -> String {
    let file_path = String::from(
        &args[2]
    );

    println!(
        "In the {file_path}\n"
    );

    file_path
}

pub fn get_the_arguments(
    args: &Vec<String>
) -> (String, String) {
    check_if_have_all_the_arguments(
        &args
    ).unwrap_or_else(|err| {
        println!(
            "\n{err}\n"
        );

        process::exit(1);
    });

    let args_01 = get_the_query(
        &args
    );

    let args_02 = get_the_file_path(
        &args
    );

    (args_01, args_02)
}