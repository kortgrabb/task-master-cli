use core::fmt;

use serde::{Serialize, Deserialize};
use crate::storage::{self, Storage};
use colored::*;
use prettytable::{Table, Row, Cell, format};

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
    WIP,
    Done,
}

// allows the status variants to be used in println!
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::WIP => write!(f, "WIP"),
            Status::Done => write!(f, "Completed"),
        }
    }
    
}


pub struct TaskManager {
    pub storage: Storage
}

pub enum TaskAction {
    AddTask(String, Option<String>),
    RemoveTask(String),
    ListTasks(Option<String>),
    MarkTask(usize, String),
    EditTask(usize, String),
    View(usize),
}


fn print_tasks(tasks: Vec<&Task>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("Task ID"),
        Cell::new("Status"),
        Cell::new("Description"),
    ]));

    for task in tasks.clone() {
        let status = match &task.status {
            Status::Done => "Complete".green().to_string(),
            Status::Todo => "Todo".red().to_string(),
            Status::WIP => "Working".yellow().to_string(),
        };

        table.add_row(Row::new(vec![
            Cell::new(&task.id.to_string()),
            Cell::new(&status),
            Cell::new(&task.description),
        ]));
    }

    table.printstd();
    println!("{} tasks found", tasks.len());
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
            TaskAction::AddTask(description, tags) => {

                if description.len() == 0 {
                    println!("You must provide a description for the task");
                    return;
                }

                let mut tags = match tags {
                    Some(tags) => tags.split(",").map(|tag| tag.trim().to_string()).collect::<Vec<String>>(),
                    None => Vec::new(),
                };

                // sort and deduplicate the tags
                tags.sort();
                tags.dedup();

                if tags.len() == 0 {
                    tags.push("none".to_string());
                }

                let task = Task {
                    id: self.storage.tasks.len(),
                    description: description.clone(),

                    // date in the format of Month Day, Year, Hour:Minute AM/PM
                    date: chrono::Local::now().format("%B %d, %Y, %I:%M %p").to_string(),
                    status: Status::Todo,
                    tags: tags.clone()
                };

                self.storage.insert_task(task);

                println!("Task was created with the following information:");   
                println!("Description: {}", description);
                println!("Tags: {}", tags.join(", "));
            },

            TaskAction::RemoveTask(index) => {
                // if the index contains "..", then it is a range of tasks to delete
                let num_tasks_deleted = if index.contains("..") {
                    let indices: Vec<usize> = index.split("..").map(|index| index.parse().unwrap()).collect();

                    let mut num_tasks_deleted = 0;
                    for index in indices[0]..indices[1] + 1 {
                        if self.storage.task_exists(index) {
                            self.storage.tasks.retain(|task| task.id != index);
                            num_tasks_deleted += 1;
                        }
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
                self.storage.load_tasks();

                if self.storage.tasks.len() == 0 {
                    println!("No tasks were found.");
                    return;
                }

                match query {
                    // if there is a query, filter the tasks by the query
                    Some(query) => {

                        let filtered_tasks: Vec<&Task> = if query.starts_with('@') {
                            self.storage.tasks
                            .iter()
                            .filter(|task| task.tags.contains(&query.trim_start_matches('@').to_string()))
                            .collect()
                        } else {
                            self.storage.tasks
                            .iter()
                            .filter(|task| task.description.contains(&query))
                            .collect()
                        };

                        print_tasks(filtered_tasks);
                    },

                    // if there is no query, print all tasks
                    None => {
                        print_tasks(self.storage.tasks.iter().collect());
                    }
                }
            }

            TaskAction::MarkTask(index, status) => {
                
                if !self.storage.task_exists(index) {
                    println!("Task with ID {} does not exist", index);
                    return;
                }

                // see if the status is valid
                let status = match status.as_str() {
                    "todo" | "t" => Status::Todo,
                    "wip" | "w" => Status::WIP,
                    "done" | "d" => Status::Done,
                    _ => {
                        println!("Invalid status: {}", status);
                        println!("Valid statuses are: (todo, t), (wip, w), (done, d)");
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

            TaskAction::EditTask(index, description) => {
                if !self.storage.task_exists(index) {
                    println!("Task with ID {} does not exist", index);
                    return;
                }

                // update the task
                self.storage.tasks.iter_mut().for_each(|task| {
                    if task.id == index {
                        task.description = description.clone();
                    }
                });

                self.storage.update_tasks();
                
                println!("The description of Task with ID {} has been updated to {}", index, description);
            }

            TaskAction::View(index) => {
                if !self.storage.task_exists(index) {
                    println!("Task with ID {} does not exist", index);
                }

                let task = self.storage.get_task_at(index);
                
                println!("{}: {}", "Task ID".green(), task.id);
                println!("{}: {}", "Description".green(), task.description);
                println!("{}: {}", "Date".green(), task.date);
                println!("{}: {}", "Status".green(), task.status);
                println!("{}: {}", "Tags".green(), task.tags.join(", "));
            }
        }
    }
}
