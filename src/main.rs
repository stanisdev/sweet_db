extern crate sweet_db;

use sweet_db::tools::args;

fn main() {
    let mut commands = match args::Command::new() {
        None => {
            println!("Any commands was not entered");
            return;
        },
        Some(instance) => instance,
    };
    match commands.run() {
        Err(mess) => println!("{}", mess),
        Ok(mess) => {
            if mess.len() > 0 {
                println!("{}", mess);
            }
        }
    }
}
