use std::process::Command;
use std::str;

// Function to get the GitHub username from the `gh auth status` command
pub fn get_github_username() -> String {
    let output = Command::new("gh")
        .arg("auth")
        .arg("status")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
        // Parse the output to extract the username
        for line in stdout.lines() {
            if line.contains("Logged in to github.com account") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(username) = parts.get(6) {
                    return username.to_string();
                }
            }
        }
    }

    // Return an empty string if the username cannot be extracted
    String::new()
}
