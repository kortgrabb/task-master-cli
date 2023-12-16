use std::collections::HashMap;

use crate::taskmanager::{TaskManager, TaskAction};

pub enum Command {
    Add(String, Option<String>),
    Remove(String),
    List(Option<String>),
    Mark(usize, String),
    Edit(usize, String),
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
                if let Some(tags) = args.get(2) {
                    Command::Add(task.to_string(), Some(tags.to_string()))
                } else {
                    Command::Add(task.to_string(), None)
                }
            } else {
                println!("Invalid command line arguments");
                print_usage();
                std::process::exit(1);
            }
        }
        "remove" => {
            if let Some(index) = args.get(1) {
                Command::Remove(index.to_string())
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
        "edit" => {
            if let Some(index) = args.get(1) {
                if let Some(description) = args.get(2) {
                    Command::Edit(index.parse().expect("Error parsing index"), description.to_string())
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
        Command::Add(task, tags) => tasks.execute(TaskAction::AddTask(task, tags)),
        Command::Remove(index) => tasks.execute(TaskAction::RemoveTask(index)),
        Command::List(query) => tasks.execute(TaskAction::ListTasks(query)),
        Command::Help() => print_usage(),
        Command::Mark(index, status) => tasks.execute(TaskAction::MarkTask(index, status)),
        Command::Edit(index, description) => tasks.execute(TaskAction::EditTask(index, description)),
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

