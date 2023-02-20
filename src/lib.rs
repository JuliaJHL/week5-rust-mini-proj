use std::fs;
use std::io::Write;

pub fn print_usage() {
    println!("Usage:");
    println!("  todo add <item>: cargo run -- add 'item'");
    println!("  todo remove <index>: cargo run -- remove '1'");
    println!("  todo list: cargo run -- list");
}

pub fn add_item(mut args: std::env::Args) {
    let mut todo_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")
        .unwrap();

    if let Some(item) = args.next() {
        writeln!(todo_file, "{item}").unwrap();
        println!("Successfully Added item to todo list");
    } else {
        println!("Error: Missing item");
    }
}

pub fn remove_item(mut args: std::env::Args) {
    // parse the index in args
    let index = match args.next() {
        Some(arg) => arg,
        None => {
            println!("Error: Missing index");
            return;
        }
    };
    // parse the index to a number
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Index is not a number");
            return;
        }
    };
    // read the file
    let todo_list = fs::read_to_string("todo.txt").unwrap();
    // split the file into lines
    let mut items: Vec<&str> = todo_list.lines().collect::<Vec<_>>();
    if index < 1 || index > items.len() {
        println!("Error: Index out of bounds");
    } else {
        items.remove(index - 1);
        let new_todo_list = items.join("\n");
        fs::write("todo.txt", new_todo_list).unwrap();
        println!("Successfully removed item from todo list");
    }
}

pub fn list_items() {
    let todo_list = fs::read_to_string("todo.txt").unwrap();
    if todo_list.trim().is_empty() {
        println!("Todo list is empty");
    } else {
        for (i, item) in todo_list.lines().enumerate() {
            println!("{}. {}", i + 1, item);
        }
    }
}
