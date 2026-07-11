use crate::file::file;

const LIST: &str = "list";
const ADD: &str = "add";
const UPDATE: &str = "update";
const DELETE: &str = "delete";

pub fn list() {
    let tasks = file::read();

    tasks.split("|").for_each(|f| {
        let items: Vec<&str> = f.split(",").collect();

        println!("{} {}", items[0], items[1]);
    });
}

pub fn add(action: &str) {
    let tasks = file::read();

    let mut max: Option<u8> = tasks.split("|")
        .filter(|f| !f.is_empty())
        .map(|f| f.chars().next().unwrap().to_digit(10).unwrap() as u8)
        .max();

    if let None = max {
        max = Some(1);
    } else {
        max = Some(max.unwrap() + 1);
    }

    let task = format!("{},{}", max.unwrap(), action);
    let is_first = max.unwrap() == 1;
    file::append(is_first, task);
}

pub fn execute(command: &str, action: &str) {
    match command {
        LIST => list(),
        ADD => add(action),
        UPDATE => list(),
        DELETE => list(),
        _ => println!("usage kafka <list|add|update|delete> <action>"),
    }
}


