use taskmaster::cli;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = cli::parse_command(&args);
    
    cli::run_command(command);
}

#[cfg(test)]
mod tests {
    use super::cli;

    #[test]
    fn test_parse_command_add() {
        let args = vec![String::from("add"), String::from("Finish the report"), String::from("work,urgent")];
        match cli::parse_command(&args) {
            cli::Command::Add(task, tags) => {
                assert_eq!(task, "Finish the report");
                assert_eq!(tags, Some(String::from("work,urgent")));
            },
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_parse_command_remove() {
        let args = vec![String::from("remove"), String::from("1")];
        match cli::parse_command(&args) {
            cli::Command::Remove(index) => {
                assert_eq!(index, "1");
            },
            _ => panic!("Expected Remove command"),
        }
    }

    #[test]
    fn test_parse_command_list() {
        let args = vec![String::from("list"), String::from("report")];
        match cli::parse_command(&args) {
            cli::Command::List(query) => {
                assert_eq!(query, Some(String::from("report")));
            },
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_parse_command_mark() {
        let args = vec![String::from("mark"), String::from("2"), String::from("doing")];
        match cli::parse_command(&args) {
            cli::Command::Mark(index, status) => {
                assert_eq!(index, 2);
                assert_eq!(status, "doing");
            },
            _ => panic!("Expected Mark command"),
        }
    }

    #[test]
    fn test_parse_command_edit() {
        let args = vec![String::from("edit"), String::from("3"), String::from("Attend team meeting at 10 AM")];
        match cli::parse_command(&args) {
            cli::Command::Edit(index, description) => {
                assert_eq!(index, 3);
                assert_eq!(description, "Attend team meeting at 10 AM");
            },
            _ => panic!("Expected Edit command"),
        }
    }

    #[test]
    fn test_parse_command_help() {
        let args = vec![String::from("help")];
        match cli::parse_command(&args) {
            cli::Command::Help() => {},
            _ => panic!("Expected Help command"),
        }
    }
}