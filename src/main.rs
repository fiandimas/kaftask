use std::{env, process};

mod kaftask;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage kaftask <list|add|delete> <action>");
        process::exit(1);
    }

    let command = &args[1];
    let mut action = String::from("");

    if (command == "add" || command == "delete") && args.len() < 3 {
        println!("usage kaftask <list|add|delete> <action>");
        process::exit(1);
    } else if command == "add" || command == "delete" {
        action = String::from(&args[2]);
    }

    kaftask::execute(command, &action);
}
