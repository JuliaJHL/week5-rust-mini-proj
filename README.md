# A Rust CLI tool: to-do list management 
This is a Rust CLI tool that manages a to-do list. This tool allows you to add, remove and list items on your to-do list.
* You can use `cargo run -- add 'buy food'` to add an item to the to-do list.
* You can use `cargo run -- remove 2` to remove the second item on the list.
* You can use `cargo run -- list` to list all items on the list.

## Project Setup
1. clone the repo:
```
git clone https://github.com/JuliaJHL/week5-rust-mini-proj.git
```
2. cd into the project:
```
cd week5-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run 
```

## examples
When you run the project, it will prompt you the usage:
```
Usage:
  todo add <item>: cargo run -- add 'item'
  todo remove <index>: cargo run -- remove '1'
  todo list: cargo run -- list
```
Add three items to the to-do list:
![add](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/add.png)
Check the items on the list:
![addcheck](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/addcheck.png)
Remove the second item on the list:
![remove](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/remove.png)
Check the items on the list after removing:
![removecheck](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/removehceck.png)
You can also check the result in  `todo.txt` file.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
