# Task Master CLI

A command line application written purely in Rust for managing tasks.

## Project Structure

The project is organized into several modules:

- `cli`: This module handles parsing and running commands from the command line.
- `taskmanager`: This module manages tasks, including adding, removing, and listing tasks.
- `storage`: This module handles reading from and writing to the `tasks.json` file.
- The tasks are all stored in a `tasks.json` file.

## Usage

To use this application, you can run the following commands:

Based on the Rust code provided, here's a comprehensive guide for the command usage of your Task Management application:

### Command Usage Guide

#### 1. Add a New Task
- **Command:** `add <task_description> [tags]`
- **Description:** Adds a new task with the specified description. Optionally, you can include tags separated by commas.
- **Example:** 
  - `add "Finish the report" work,urgent`
  - This adds a task with the description "Finish the report" and tags "work" and "urgent".

#### 2. Remove a Task
- **Command:** `remove <task_id> | <range>`
- **Description:** Removes a task by its ID. You can also specify a range of IDs to remove multiple tasks.
- **Example:** 
  - `remove 3` 
  - Removes the task with ID 3.
  - `remove 1..4` 
  - Removes tasks with IDs from 1 to 4.

#### 3. List Tasks
- **Command:** `list [query]`
- **Description:** Lists all tasks. If a query is provided, it lists tasks that contain the query in their description.
- **Example:** 
  - `list`
  - Lists all tasks.
  - `list report`
  - Lists tasks containing the word "report".

#### 4. Mark a Task
- **Command:** `mark <task_id> <status>`
- **Description:** Marks a task with the specified ID as either "todo", "doing", or "complete".
- **Example:** 
  - `mark 2 doing`
  - Marks task ID 2 as "doing".

#### 5. Edit a Task
- **Command:** `edit <task_id> <new_description>`
- **Description:** Edits the description of the task with the specified ID.
- **Example:** 
  - `edit 3 "Attend team meeting at 10 AM"`
  - Changes the description of task ID 3 to "Attend team meeting at 10 AM".

#### 6. Display Help
- **Command:** `help`
- **Description:** Displays the usage guide for all commands.
- **Example:** 
  - `help`
  - Displays the help information.

### Note:
- `<task_id>` refers to the numerical ID of the task.
- `<range>` refers to a range of task IDs, formatted as `start..end`.
- `[tags]` and `[query]` are optional parameters.

This command usage guide provides a clear understanding of how to interact with your Task Manager application via the command line. It's recommended to include this guide in your project's README or documentation for easy reference.

## Building the Project

To build this project, you need to have Rust installed. Use the following command:

```sh
cargo build --release
```

This will create an executable file in the `target/debug` directory. Run the application with:

## Dependencies

This project uses the following dependencies:

- `serde`: For serializing and deserializing data.
- `serde_json`: For handling JSON data.
- `chrono`: For handling dates and times.

These dependencies are specified in the `Cargo.toml` file.

## Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull request.

## License

This project is open source and available under the MIT License.
