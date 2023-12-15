use serde::{Serialize, Deserialize};

use crate::storage::{self, Storage};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,

    pub status: Status,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Todo,
    Working,
    Complete,
}

pub struct Tasks {
    pub action: TaskAction,
    pub storage: Storage
}

pub enum TaskAction {
    AddTask(String),
    RemoveTask(usize),
    ListTasks(Option<String>),
}

impl Tasks {
    pub fn new(action: TaskAction) -> Tasks {
        let storage = storage::Storage::new();
        Tasks {
            action,
            storage,
        }
    }

    pub fn execute(&mut self) {
        match &self.action {
            TaskAction::AddTask(task) => {
                let new_task = Task {
                    description: task.clone(),
                    status: Status::Todo,
                    tags: Vec::new(),
                };
                self.storage.insert_task(new_task);
            }
            TaskAction::RemoveTask(index) => {
                self.storage.remove_task(index);
            }
            TaskAction::ListTasks(query) => {
                let tasks = self.storage.get_tasks();

                for (index, task) in tasks.iter().enumerate() {
                    match query {
                        Some(query) => {
                            if task.description.contains(query) {
                                println!("{}: {}", index, task.description);
                            }
                        }
                        None => {
                            println!("{}: {}", index, task.description);
                        }
                    }
                }
            }
        }
    }
}