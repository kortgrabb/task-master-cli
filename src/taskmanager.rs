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
            Status::Working => write!(f, "Doing"),
            Status::Complete => write!(f, "Completed"),
        }
    }
    
}

pub struct TaskManager {
    pub storage: Storage
}

pub enum TaskAction {
    AddTask(String),
    RemoveTask(String),
    ListTasks(Option<String>),
    MarkTask(usize, String),
    EditTask(usize, String),
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
                    id: self.storage.tasks.len(),
                    description,
                    // date in the format of Month Day, Year, Hour:Minute AM/PM
                    date: chrono::Local::now().format("%B %d, %Y, %I:%M %p").to_string(),
                    status: Status::Todo,
                    tags: Vec::new(),
                };

                self.storage.insert_task(task);

                println!("Task added successfully!")
            },

            TaskAction::RemoveTask(index) => {

                // if a - is passed between two numbers, remove all tasks between those numbers
                let num_tasks_deleted = if index.contains("..") {
                    let indices: Vec<usize> = index.split("..").map(|index| index.parse().unwrap()).collect();

                    let mut num_tasks_deleted = 0;
                    for index in indices[0]..indices[1] + 1 {
                        self.storage.tasks.retain(|task| task.id != index);
                        num_tasks_deleted += 1;
                    }
                    num_tasks_deleted
                } else {
                    let index: usize = index.parse().unwrap();
                    self.storage.tasks.retain(|task| task.id != index);
                    1
                };

                self.storage.update_tasks();
                println!("{} Task(s) deleted successfully!", num_tasks_deleted);
            },

            TaskAction::ListTasks(query) => {
                let tasks = self.storage.get_tasks();

                if (tasks.len() == 0) {
                    println!("No tasks were found. You can add a task with the command: todo add \"task description\"");
                    return;
                }

                match query {
                    // if there is a query, filter the tasks by the query
                    Some(query) => {
                        let filtered_tasks = tasks
                            .iter()
                            .filter(|task| task.description.contains(&query))
                            .collect::<Vec<&Task>>();
                        for task in filtered_tasks {
                            println!("{}: {}\n{}: {}\n{}: {}\n{}: {}\n----------------------", 
                                "Task ID", task.id, 
                                "Status".bright_blue(), &task.status.to_string().on_color(match &task.status {
                                    Status::Complete => "green",
                                    Status::Todo => "red",
                                    Status::Working => "yellow",
                                }).black(), 
                                "Description".bright_blue(), &task.description, 
                                "Written".bright_blue(), &task.date);
                        }
                    },

                    // if there is no query, print all tasks
                    None => {
                        for task in tasks {
                            println!("{}: {}\n{}: {}\n{}: {}\n{}: {}\n----------------------", 
                                "Task ID", task.id, 
                                "Status".bright_blue(), &task.status.to_string().on_color(match &task.status {
                                    Status::Complete => "green",
                                    Status::Todo => "red",
                                    Status::Working => "yellow",
                                }).black(), 
                                "Description".bright_blue(), &task.description, 
                                "Written".bright_blue(), &task.date);
                        }
                    }
                }
            }

            TaskAction::MarkTask(index, status) => {
                let task_exists = self.storage.tasks.iter().any(|task| task.id == index);

                if !task_exists {
                    println!("Task with ID {} does not exist", index);
                    return;
                }

                // see if the status is valid
                let status = match status.as_str() {
                    "todo" | "t" => Status::Todo,
                    "doing" | "d" => Status::Working,
                    "complete" | "c" => Status::Complete,
                    _ => {
                        println!("Invalid status: {}", status);
                        println!("Valid statuses are (t)odo, (d)oing, and (c)omplete");
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
            TaskAction::EditTask(_, _) => {
                println!("Edit task");
            }
        }
    }
}