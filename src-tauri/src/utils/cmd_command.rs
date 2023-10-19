use std::path::Path;
use std::process::{Command, Output};

#[derive(Debug)]
pub enum WindowType {
    Default,
    WithWindow,
    WithWindowKeepOpen,
}

pub fn run_command(path: &Path, command: &str, window_type: WindowType) -> Output {
    let mut final_command: Vec<&str> = vec![];
    final_command.push("/c");

    match window_type {
        WindowType::Default => {}
        WindowType::WithWindow => {
            final_command.push("start");
            final_command.push("cmd");
            final_command.push("/c");
        }
        WindowType::WithWindowKeepOpen => {
            final_command.push("start");
            final_command.push("cmd");
            final_command.push("/k");
        }
    }
    final_command.push(command);

    Command::new("cmd")
        .current_dir(path)
        .args(&final_command)
        .output()
        .unwrap()
}