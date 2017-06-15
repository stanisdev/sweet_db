use std::env;

pub struct Command {
    argument: String,
    parameter: String,
}

impl Command {

    // Parse command line arguments
    pub fn new() -> Option<Command> {
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
                return Some(Command {
                    argument: parsed[0].to_owned(),
                    parameter: parsed[1].to_owned()
                });
            },
        };
    }

    // Check argument existence
    pub fn execute(&self) -> Result<(), &'static str> {
        let args = vec!["add", "remove", "exists", "find"];
        let found = args.into_iter().filter(|&arg| arg == self.argument).collect::<Vec<&str>>();
        if found.len() < 1 {
            return Err("Command not found");
        }

        // let worker = file_worker::FileWorker.new();
        // match self.argument {
        //     "add".to_string() => {
        //         let worker = file_worker.new();
        //     }
        // }
        return Ok(());
    }
}
