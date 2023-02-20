fn main() {
    let mut args = std::env::args();
    args.next(); // skip the program name

    if let Some(command) = args.next() {
        match command.as_str() {
            "add" => todo_list::add_item(args),
            "remove" => todo_list::remove_item(args),
            "list" => todo_list::list_items(),
            _ => todo_list::print_usage(),
        }
    } else {
        todo_list::print_usage();
    }
}
