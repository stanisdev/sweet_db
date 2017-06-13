use std::env;

/**
 * Parse command line arguments
 */
pub fn init() -> Option<(String, String)> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => {
            return None;
        },
        Some(arg) => {
            let parsed = arg.split(":").collect::<Vec<&str>>();
            if parsed.len() != 2 || parsed[1].len() < 1 {
                return None;
            }
            return Some((parsed[0].to_owned(), parsed[1].to_owned()));
        },
    };
}
