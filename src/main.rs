use todo_cli::cli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = cli::parse_command(&args);
    
    cli::run_command(command);
}