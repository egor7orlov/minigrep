use std::env;

pub struct InputParams {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl InputParams {
    pub fn build(args: &[String]) -> Result<InputParams, &'static str> {
        if args.len() < 2 {
            return Err("2 input arguments are required");
        }

        Ok(InputParams {
            query: args.get(0).unwrap().to_string(),
            file_path: args.get(1).unwrap().to_string(),
            ignore_case: env::var("IGNORE_CASE").is_ok_and(|val| val == "1"),
        })
    }
}
