# todo-cli

This is a command-line application written in Rust for managing tasks. It allows you to add, remove, and list tasks.

## Project Structure

The project is organized into several modules:

- [`cli`](command:_github.copilot.openSymbolInFile?%5B%22src%2Fcli.rs%22%2C%22cli%22%5D "src/cli.rs"): This module handles parsing and running commands from the command line.
- [`taskmanager`](command:_github.copilot.openSymbolInFile?%5B%22src%2Ftaskmanager.rs%22%2C%22taskmanager%22%5D "src/taskmanager.rs"): This module manages tasks, including adding, removing, and listing tasks.
- [`storage`](command:_github.copilot.openSymbolInFile?%5B%22src%2Fstorage.rs%22%2C%22storage%22%5D "src/storage.rs"): This module handles reading from and writing to the [`tasks.json`](command:_github.copilot.openSymbolInFile?%5B%22src%2Fstorage.rs%22%2C%22tasks.json%22%5D "src/storage.rs") file.

## Usage

To use this application, you can run the following commands:

- `add <task>`: Add a new task.
- `remove <index>`: Remove a task by its index.
- `list [query]`: List all tasks or tasks containing the query.

## Building the Project

To build this project, you need to have Rust installed. You can then use the following command:

```sh
cargo build
```

This will create an executable file in the [`target/debug`](command:_github.copilot.openRelativePath?%5B%22target%2Fdebug%22%5D "target/debug") directory. You can run the application with:

```sh
cargo run
```

## Dependencies

This project uses the following dependencies:

- `serde`: For serializing and deserializing data.
- `serde_json`: For handling JSON data.
- `chrono`: For handling dates and times.

These dependencies are specified in the [`Cargo.toml`](command:_github.copilot.openSymbolInFile?%5B%22Cargo.toml%22%2C%22Cargo.toml%22%5D "Cargo.toml") file.

## Contributing

Contributions are welcome. Please feel free to open an issue or submit a pull request.

## License

This project is open source and available under the MIT License.