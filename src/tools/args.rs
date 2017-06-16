use std::env;
use std::io;
use super::core;

pub struct Command {
    pub argument: String,
    pub parameter: String,
    pub result: Result<&'static str, &'static str>,
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
                    parameter: parsed[1].to_owned(),
                    result: Ok("")
                });
            },
        };
    }

    // Check argument existence
    pub fn run(&mut self) -> Result<&'static str, &'static str> {
        let args = vec!["add", "remove", "exists", "find"];
        let found = args.into_iter().filter(|&arg| arg == self.argument).collect::<Vec<&str>>();
        if found.len() < 1 {
            return Err("Command not found");
        }
        core::execute_command(self);
        return self.result;
    }
}
