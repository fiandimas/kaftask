use std::{env, process};

mod kaftask;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        println!("usage kaftask <command> <action>");
        process::exit(1);
    }

    let command = &args[1];
    let action = &args[2];

    kaftask::execute(command, action);
}
