use core::fmt;

use serde::{Serialize, Deserialize};
use crate::storage::{self, Storage};
use colored::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub date: String,
    pub status: Status,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]

pub enum Status {
    Todo,
    Working,
    Complete,
}

// allows the status variants to be used in println!
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::Working => write!(f, "Working"),
            Status::Complete => write!(f, "Complete"),
        }
    }
    
}

pub struct TaskManager {
    pub storage: Storage
}

pub enum TaskAction {
    AddTask(String),
    RemoveTask(usize),
    ListTasks(Option<String>),
    MarkTask(usize, String),
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let storage = storage::Storage::new();
        TaskManager {
            storage,
        }
    }

    pub fn execute(&mut self, action: TaskAction) {
        match action {

            // add a new task to the storage
            TaskAction::AddTask(description) => {
                let task = Task {
                    id: self.storage.tasks.len() + 0,
                    description,
                    // date in the format of Month Day, Year, Hour:Minute AM/PM
                    date: chrono::Local::now().format("%B %d, %Y, %I:%M %p").to_string(),
                    status: Status::Todo,
                    tags: Vec::new(),
                };

                self.storage.insert_task(task);

                println!("Task added successfully!")
            },

            // remove the task where task.id == index
            TaskAction::RemoveTask(index) => {
                // retain() is the opposite of filter()

                // see if the task exists
                let task_exists = self.storage.tasks.iter().any(|task| task.id == index);
                if !task_exists {
                    println!("Task with ID {} does not exist", index);
                    return;
                }

                self.storage.tasks.retain(|task| task.id != index);
                self.storage.update_tasks();

                println!("Task with ID {} has been removed", index);
            },

            // list all tasks
            // list all tasks
            TaskAction::ListTasks(query) => {
                let tasks = self.storage.get_tasks();
                match query {
                    // if there is a query, filter the tasks by the query
                    Some(query) => {
                        let filtered_tasks = tasks
                            .iter()
                            .filter(|task| task.description.contains(&query))
                            .collect::<Vec<&Task>>();
                        for task in filtered_tasks {
                            println!("{}: {}\n{}: {}\n{}: {}\n{}: {}\n----------------------", 
                                "Task ID".cyan(), task.id, 
                                "Status".bright_blue(), &task.status.to_string().on_color(match &task.status {
                                    Status::Complete => "green",
                                    Status::Todo => "red",
                                    Status::Working => "yellow",
                                }), 
                                "Description".bright_blue(), &task.description, 
                                "Date".bright_blue(), &task.date);
                        }
                    },

                    // if there is no query, print all tasks
                    None => {
                        for task in tasks {
                            println!("{}: {}\n{}: {}\n{}: {}\n{}: {}\n----------------------", 
                                "Task ID".on_cyan(), task.id, 
                                "Status".bright_blue(), &task.status.to_string().on_color(match &task.status {
                                    Status::Complete => "green",
                                    Status::Todo => "red",
                                    Status::Working => "yellow",
                                }), 
                                "Description".bright_blue(), &task.description, 
                                "Date".bright_blue(), &task.date);
                        }
                    }
                }
            }

            // set the status to the given string
            TaskAction::MarkTask(index, status) => {
                // see if the task exists
                let task_exists = self.storage.tasks.iter().any(|task| task.id == index);

                if !task_exists {
                    println!("Task with ID {} does not exist", index);
                    return;
                }

                // see if the status is valid
                let status = match status.as_str() {
                    "todo" | "t" => Status::Todo,
                    "working" | "w" => Status::Working,
                    "complete" | "c" => Status::Complete,
                    _ => {
                        println!("Invalid status: {}", status);
                        println!("Valid statuses are (t)odo, (w)orking, and (c)omplete");
                        return;
                    }
                };

                // update the task
                self.storage.tasks.iter_mut().for_each(|task| {
                    if task.id == index {
                        task.status = status.clone();
                    }
                });

                self.storage.update_tasks();
                
                println!("Task with ID {} has been marked as {}", index, status);
            }
        }
    }
}