use std::process::{Command, Stdio};
use std::str;

/// Runs a command in the shell
pub fn run_cmd(command: &str) -> bool {
    let args: Vec<&str> = command.split_whitespace().collect();
    let result = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                // println!("Command executed successfully.");
                true // Command executed successfully, return true
            } else {
                // println!("Failed to execute command.");
                false // Command execution failed, return false
            }
        }
        Err(_) => {
            println!("Error running command.");
            false // Error running command, return false
        }
    }
}
