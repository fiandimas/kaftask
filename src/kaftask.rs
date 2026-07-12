use crate::file::file;

const LIST: &str = "list";
const ADD: &str = "add";
const DELETE: &str = "delete";

pub fn list() {
    match file::read() {
        Ok(tasks) => {
            if tasks == "" {
                println!("no tasks to display");
                return;
            }

            tasks.split("|")
                .filter(|f| !f.is_empty())
                .for_each(|f| {
                    let items: Vec<&str> = f.split(",").collect();

                    println!("{} {}", items[0], items[1]);
                });
        },
        Err(e) => eprintln!("failed to display task: {e}"),
    }
}

pub fn add(action: &str) {
    match file::read() {
        Ok(tasks) => {
            let mut max: Option<u8> = tasks.split("|")
                .filter(|f| !f.is_empty())
                .map(|f| f.split(",").next().unwrap().parse::<u8>().unwrap())
                .max();

            if let None = max {
                max = Some(1);
            } else {
                max = Some(max.unwrap() + 1);
            }

            let task = format!("{},{}", max.unwrap(), action);
            let is_first = max.unwrap() == 1;

            let result = file::append(is_first, task);
            if let Err(e) = result {
                eprintln!("failed to write task: {e}");
                return;
            }

            println!("success add task");
        },
        Err(e) => eprintln!("failed to add task: {e}"),
    }

}

pub fn delete(id: &str) {
    match file::read() {
        Ok(tasks) => {
            let mapped  = tasks.split("|")
                .filter(|f| !f.is_empty())
                .filter(|f| f.split(",").next().unwrap() != id)
                .collect::<Vec<&str>>()
                .join("|");

            let result = file::overwrite(mapped);
            if let Err(e) = result {
                eprintln!("failed to delete task: {e}");
                return;
            }

            println!("success delete task");
        },
        Err(e) => eprintln!("failed to delete task: {e}"),
    }
}

pub fn execute(command: &str, action: &str) {
    match command {
        LIST => list(),
        ADD => add(action),
        DELETE => delete(action),
        _ => println!("usage kafka <list|add|delete> <action>"),
    }
}


