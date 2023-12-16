use std::collections::HashMap;

use crate::taskmanager::{TaskManager, TaskAction};

pub enum Command {
    Add(String, Option<String>),
    Remove(String),
    List(Option<String>),
    Mark(usize, String),
    Edit(usize, String),
    View(usize),
    Help(),
}

fn handle_invalid_args() -> ! {
    println!("Invalid command line arguments");
    print_usage();
    std::process::exit(1);
}

pub fn parse_command(args: &[String]) -> Command {
    if args.is_empty() {
        println!("Invalid command line arguments");
        print_usage();
        std::process::exit(1);
    }

    match args[0].as_str() {
        "list" => {
            let query = args.get(1).map(|s| s.to_string());
            Command::List(query)
        }
        "add" => {
            let task = args.get(1).map(|s| s.to_string());
            let tags = args.get(2).map(|s| s.to_string());
            match task {
                Some(task) => Command::Add(task, tags),
                None => handle_invalid_args(),
            }
        }
        "remove" => {
            let index = args.get(1).map(|s| s.to_string());
            match index {
                Some(index) => Command::Remove(index),
                None => handle_invalid_args(),
            }
        }
        "mark" => {
            let index = args.get(1).and_then(|s| s.parse().ok());
            let status = args.get(2).map(|s| s.to_string());
            match (index, status) {
                (Some(index), Some(status)) => Command::Mark(index, status),
                _ => handle_invalid_args(),
            }
        }
        "edit" => {
            let index = args.get(1).and_then(|s| s.parse().ok());
            let description = args.get(2).map(|s| s.to_string());
            match (index, description) {
                (Some(index), Some(description)) => Command::Edit(index, description),
                _ => handle_invalid_args(),
            }
        }

        "view" => {
            let index = args.get(1).and_then(|s| s.parse().ok());
            match index {
                Some(index) => Command::View(index),
                None => handle_invalid_args(),
            }
        }

        "help" => Command::Help(),
        _ => handle_invalid_args(),
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
        Command::View(index) => tasks.execute(TaskAction::View(index)),
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

