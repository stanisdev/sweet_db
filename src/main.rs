mod args;
mod file_worker;

fn main() {
    let args = match args::init() {
        None => {
            println!("Commands cannot be found");
            return;
        },
        Some(args) => args,
    };
    let mut worker = file_worker::FileWorker::new();
    worker.read();
}
