use serde::{Serialize, Deserialize};

use crate::storage::{self, Storage};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
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

pub struct TaskManager {
    pub storage: Storage
}

pub enum TaskAction {
    AddTask(String),
    RemoveTask(usize),
    ListTasks(Option<String>),
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
            TaskAction::AddTask(description) => {
                let task = Task {
                    description,
                    date: chrono::Local::now().to_string(),
                    status: Status::Todo,
                    tags: Vec::new(),
                };

                self.storage.insert_task(task);
            },
            TaskAction::RemoveTask(index) => {
                self.storage.remove_task(&index);
            },
            TaskAction::ListTasks(query) => {
                let tasks = self.storage.get_tasks();
                match query {
                    Some(query) => {
                        let filtered_tasks = tasks
                            .iter()
                            .filter(|task| task.description.contains(&query))
                            .collect::<Vec<&Task>>();
                        for task in filtered_tasks {
                            println!("{}", task.description);
                        }
                    },
                    None => {
                        for task in tasks {
                            println!("{}", task.description);
                        }
                    }
                }
            }
        }
    }
}