pub fn search_the_query_in_content(
    query: &str,
    content: &str
) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(
                String::from(line)
            );
        }
    }

    results
}

#[test]
fn test_one_result() {
    let query = "duct";

    let content = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec![
            "safe, fast, productive."
        ],
        search_the_query_in_content(
            query, content
        )
    );
}