pub fn search_the_query_in_content(
    query: &str,
    content: &str,
    sensitive: bool,
) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content.lines() {
        if !sensitive {
            // case sensitive
            if line.contains(
                &query
            ) {
                results.push(
                    String::from(
                        line
                    )
                );
            }
        } else {
            // case insensitive
            if line.to_lowercase().contains(
                &query.to_lowercase()
            ) {
                results.push(
                    String::from(
                        line
                    )
                )
            }
        }
    }

    results
}

#[test]
fn search_case_sentive() {
    let query = "duct";

    let content = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

    assert_eq!(
        vec![
            "safe, fast, productive."
        ],
        search_the_query_in_content(
            query, content, false
        )
    );
}

#[test]
fn search_case_insensitive() {
    let query = "rUsT";

    let content = "\
Rust:
safe, fast, productve.
Pick three.
Trust me.";

    assert_eq!(
        vec![
            "Rust:",
            "Trust me."
        ],
        search_the_query_in_content(
            query, content, true
        )
    );
}