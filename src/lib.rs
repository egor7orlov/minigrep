use std::error::Error;
use std::fs;

pub struct InputArgs {
    query: String,
    file_path: String,
}

impl InputArgs {
    pub fn from_slice(args: &[String]) -> Result<InputArgs, &'static str> {
        if args.len() < 2 {
            return Err("2 input arguments are required");
        }

        Ok(InputArgs {
            query: args.get(0).unwrap().to_string(),
            file_path: args.get(1).unwrap().to_string(),
        })
    }

    pub fn get_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path.as_str()
    }
}

pub fn run(arguments: InputArgs) -> Result<(), Box<dyn Error>> {
    let query = arguments.get_query();
    let file_path = arguments.get_file_path();
    let file_contents = fs::read_to_string(file_path)?;
    let found_lines = search(query, &file_contents);

    if found_lines.is_empty() {
        println!("-- No lines containing \"{}\" substring found --", query);
        return Ok(());
    }

    for line_with_occurrence in found_lines {
        println!("{}", line_with_occurrence);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_one_line() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
