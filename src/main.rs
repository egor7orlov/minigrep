use std::{env, process};
use minigrep::{run};
use minigrep::app_params::InputParams;

fn main() {
    let cli_args = env::args().skip(1).collect::<Vec<String>>();
    let params = InputParams::build(&cli_args)
        .unwrap_or_else(|err_msg| {
            eprintln!("Error while parsing arguments: {err_msg}");
            process::exit(1);
        });

    if let Err(err) = run(params) {
        eprintln!("Error while searching for occurrences: {err}");
        process::exit(1);
    }
}
