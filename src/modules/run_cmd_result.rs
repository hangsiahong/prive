use std::error::Error;
use std::process::{Command, Stdio};

/// Runs a command in the shell
pub fn run_cmd_result(command: &str) -> Result<(), Box<dyn Error>> {
    let args: Vec<&str> = command.split_whitespace().collect();
    let result = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if result.status.success() {
        // println!("Command executed successfully.");
        Ok(())
    } else {
        println!("Failed to execute command.");
        Err("Command execution failed.".into())
    }
}
