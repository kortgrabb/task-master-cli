
use std::{fs::OpenOptions, path::Path, io::{self, Read, Write}};

use serde::{Deserialize, Serialize};

use crate::taskmanager::Task;

#[derive(Serialize, Deserialize, Clone)]
pub struct Storage {
    pub tasks: Vec<Task>,
}

impl Storage {
    pub fn new() -> Storage {
        // load the tasks from the file if it exists
        let tasks = match Storage::read_from_file(Path::new("./tasks.json")) {
            Ok(storage) => storage.tasks,
            Err(_) => Vec::new(),
        };

        Storage { tasks }
    }

    fn read_from_file(file_path: &Path) -> io::Result<Storage> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let tasks = match serde_json::from_str(&contents) {
            Ok(storage) => storage,
            Err(_) => Storage { tasks: Vec::new() },
        };

        Ok(tasks)
    }

    fn write_to_file(&self, file_path: &Path) -> io::Result<()> {
        // open the file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        // serialize the Storage struct into a string
        let contents = serde_json::to_string(&self)?;

        // overwrite the existing file with the new contents
        file.set_len(0)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }

    pub fn update_tasks(&mut self) {

        // fix the ids of the tasks
        for (index, task) in self.tasks.iter_mut().enumerate() {
            task.id = index;
        }

        self.write_to_file(Path::new("./tasks.json"))
            .expect("Error writing to file");
    }

    pub fn get_tasks(&self) -> Vec<Task> {

        // use the read_from_file function to read the tasks from the file
        let tasks = Storage::read_from_file(Path::new("./tasks.json"))
            .expect("Error reading from file").tasks;

        tasks
    }

    pub fn insert_task(&mut self, task: Task) {
        // add the task to the tasks vector
        self.tasks.push(task);

        // write the updated tasks to the file
        self.write_to_file(Path::new("./tasks.json"))
            .expect("Error writing to file");
    }

    pub fn remove_task(&mut self, index: &usize) {
        // remove the task at the given index
        self.tasks.remove(*index);

        println!("Task {} removed", index);

        // write the updated tasks to the file
        self.write_to_file(Path::new("./tasks.json"))
            .expect("Error writing to file");
    }
}