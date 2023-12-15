use std::collections::HashMap;

use crate::taskmanager::{TaskManager, TaskAction};

pub enum Command {
    Add(String),
    Remove(usize),
    List(Option<String>),
    Mark(usize, String),
    Help(),
}

pub fn parse_command(args: &[String]) -> Command {
    if args.is_empty() {
        println!("Invalid command line arguments");
        print_usage();
        std::process::exit(1);
    }

    match args[0].as_str() {
        "list" => {
            if let Some(query) = args.get(1) {
                Command::List(Some(query.to_string()))
            } else {
                Command::List(None)
            }
        }
        "add" => {
            if let Some(task) = args.get(1) {
                Command::Add(task.to_string())
            } else {
                println!("Invalid command line arguments");
                print_usage();
                std::process::exit(1);
            }
        }
        "remove" => {
            if let Some(index) = args.get(1) {
                Command::Remove(index.parse().expect("Error parsing index"))
            } else {
                println!("Invalid command line arguments");
                print_usage();
                std::process::exit(1);
            }
        }
        "mark" => {
            if let Some(index) = args.get(1) {
                if let Some(status) = args.get(2) {
                    Command::Mark(index.parse().expect("Error parsing index"), status.to_string())
                } else {
                    println!("Invalid command line arguments");
                    print_usage();
                    std::process::exit(1);
                }
            } else {
                println!("Invalid command line arguments");
                print_usage();
                std::process::exit(1);
            }
        }
        "help" => Command::Help(),
        _ => {
            println!("Invalid command line arguments");
            print_usage();
            std::process::exit(1);
        }
    }
}

pub fn run_command(command: Command) {
    let mut tasks = TaskManager::new();

    match command {
        Command::Add(task) => tasks.execute(TaskAction::AddTask(task)),
        Command::Remove(index) => tasks.execute(TaskAction::RemoveTask(index)),
        Command::List(query) => tasks.execute(TaskAction::ListTasks(query)),
        Command::Help() => print_usage(),
        Command::Mark(index, status) => tasks.execute(TaskAction::MarkTask(index, status)),
    }
}

// FUCKING OBVIOUS
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

