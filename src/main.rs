use std::{env, process};
use minigrep::{InputArgs, run};

fn main() {
    let arguments_vec = env::args().skip(1).collect::<Vec<String>>();
    let arguments_parsed = InputArgs::from_slice(&arguments_vec)
        .unwrap_or_else(|err_msg| {
            println!("Error while parsing arguments: {}", err_msg);
            process::exit(1);
        });

    if let Err(err) = run(arguments_parsed) {
        println!("Error while searching for occurrences: {}", err);
        process::exit(1);
    }
}
