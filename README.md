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

- `add <task> <description>`: Add a new task.
- `remove <index> `: Remove a task by its index.
- `remove start..end`: Remove all the tasks start through end
- `list [query]`: List all tasks or tasks containing the query.
- `mark <index> [(t)odo, (d)oing, (c)ompleted]`: Change the status of a task

## Building the Project

To build this project, you need to have Rust installed. Use the following command:

```sh
cargo build
```

This will create an executable file in the `target/debug` directory. Run the application with:

```sh
cargo run
```

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

---

This format should be more suitable for your documentation needs.