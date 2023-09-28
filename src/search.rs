pub struct SearchParams<'a> {
    pub query: &'a str,
    pub contents: &'a str,
    pub ignore_case: bool,
}

pub fn search(search_params: SearchParams) -> Vec<&str> {
    let ignore_case = search_params.ignore_case;
    let query = &(if ignore_case {
        search_params.query.to_lowercase()
    } else {
        search_params.query.to_string()
    });

    search_params.contents.lines().filter(|&line| {
        let line_normalized = &(if ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        });

        line_normalized.contains(query)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_one_line_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let expected_result = vec!["safe, fast, productive."];
        let execution_result = search(SearchParams {
            query,
            contents,
            ignore_case: false,
        });

        assert_eq!(expected_result, execution_result);
    }

    #[test]
    fn should_find_one_line_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let expected_result = vec!["Rust:", "Trust me."];
        let execution_result = search(SearchParams {
            query,
            contents,
            ignore_case: true,
        });

        assert_eq!(expected_result, execution_result);
    }
}