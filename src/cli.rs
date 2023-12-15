use std::collections::HashMap;

use crate::tasks::Tasks;

pub enum Command {
    Add(String),
    Remove(usize),
    List(Option<String>),
    Help(),
}

// takes a slice of String values and returns a Command enum variant
pub fn parse_command(args: &[String]) -> Command {
    match args {
        [command, query] if command == "list" => Command::List(Some(query.to_string())),
        [command] if command == "list" => Command::List(None),
        [command, task] if command == "add" => Command::Add(task.to_string()),
        [command, index] if command == "remove" => Command::Remove(index.parse().expect("Error parsing index")),
        [command] if command == "help" => Command::Help(),
        _ => {
            println!("Invalid command line arguments");
            print_usage();
            std::process::exit(1);
        }
    }
}

// takes a Command enum variant and prints a message to the console
pub fn run_command(command: Command) {
    match command {
        Command::Add(task) => {
            Tasks::Insert(task);
        }
        Command::Remove(index) => {
            Tasks::Remove(index);
        }
        Command::List(query) => {
            Tasks::List(query);
        }
        Command::Help() => {
            print_usage();
        }
    }
}

pub fn print_usage() {
    let mut commands = HashMap::new();
    commands.insert("add <task>", "Add a new task");
    commands.insert("remove <index>", "Remove a task by its index");
    commands.insert("list [query]", "List all tasks or tasks containing the query");

    println!("Usage:\n");
    
    for (command, description) in commands {
        println!("  {} {}", command, description);
    }
}

