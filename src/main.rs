mod args;

fn main() {

    // let commands = args::Command::new();
    let commands = match args::Command::new() {
        None => {
            println!("Any commands was not entered");
            return;
        },
        Some(instance) => instance,
    };
    match commands.execute() {
        Err(mess) => println!("{}", mess),
        Ok(_) => {}
    }
}
