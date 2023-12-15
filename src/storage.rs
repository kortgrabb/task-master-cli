
use std::{fs::File, path::Path, io::{self, Read, Write}};

use serde::{Deserialize, Serialize};

use crate::tasks::Task;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub tasks: Vec<Task>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            tasks: Vec::new(),
        }
    }

    pub fn read_from_file(file_path: &Path) -> io::Result<Storage> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        
        // read the file contents into the contents variable
        file.read_to_string(&mut contents)?;
        
        // deserialize the contents into a Storage struct
        let tasks: Storage = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    pub fn write_to_file(&self, file_path: &Path) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        let contents = serde_json::to_string(&self)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    
    pub fn insert_task(&mut self, task: Task) {
        // add the task to the tasks vector
        self.tasks.push(task);

        // write the updated tasks to the file
        self.write_to_file(Path::new("./tasks.json"))
            .expect("Error writing to file");
    }

    pub fn remove_task(&mut self, index: &usize) {
        // remove the task from the tasks vector
        self.tasks.remove(*index);

        // write the updated tasks to the file
        self.write_to_file(Path::new("./tasks.json"))
            .expect("Error writing to file");
    }
}