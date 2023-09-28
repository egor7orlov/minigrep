mod search;
pub mod app_params;

use std::{fs};
use std::error::Error;
use search::{search, SearchParams};
use app_params::{InputParams};

pub fn run(params: InputParams) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(params.file_path)?;
    let found_lines = search(SearchParams {
        query: &params.query,
        contents: &file_contents,
        ignore_case: params.ignore_case,
    });

    // TODO: return Err with message "-- No lines containing \"{query}\" substring found --"
    //  if there are no matches for query
    // if found_lines.is_empty() {
    //     return Err(???);
    // }

    for line_with_occurrence in found_lines {
        println!("{}", line_with_occurrence);
    }

    Ok(())
}
