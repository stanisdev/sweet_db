use super::args;
use super::file_worker;

// Execute tansfered command
pub fn execute_command(command: &mut args::Command) {
    let mut worker = file_worker::FileWorker::new();
    match Some(command.argument.as_ref()) {

        Some("find") => {
            command.result = match worker.find(&command.parameter) {
                true => Ok("Found. Success"),
                false => Err("String not found"),
            }
        },
        Some("add") => {
            if worker.find(&command.parameter) {
                return command.result = Err("Value already exists");
            }
            worker.add(&command.parameter);
        },
        _ => {},
    }
}
