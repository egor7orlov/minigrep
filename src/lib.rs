use std::{env, fs};
use std::error::Error;

pub struct InputParams {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl InputParams {
    pub fn build(args: &[String]) -> Result<InputParams, &'static str> {
        if args.len() < 2 {
            return Err("2 input arguments are required");
        }

        Ok(InputParams {
            query: args.get(0).unwrap().to_string(),
            file_path: args.get(1).unwrap().to_string(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }

    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }
}

pub fn run(params: InputParams) -> Result<(), Box<dyn Error>> {
    let query = params.get_query();
    let ignore_case = params.get_ignore_case();
    let file_path = params.get_file_path();
    let file_contents = fs::read_to_string(file_path)?;
    let found_lines = if ignore_case {
        search_case_insensitive(query, &file_contents)
    } else {
        search_case_sensitive(query, &file_contents)
    };

    // TODO: return Err with message "-- No lines containing \"{query}\" substring found --"
    //  if there are no matches for query
    // if found_lines.is_empty() {
    // }

    for line_with_occurrence in found_lines {
        println!("{}", line_with_occurrence);
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
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
        let execution_result = search_case_sensitive(query, contents);

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
        let execution_result = search_case_insensitive(query, contents);

        assert_eq!(expected_result, execution_result);
    }
}
