use std::env;

use crate::run_cmd::run_cmd;

/// Creates a new repository
pub fn create_repository(repo_path: &str, github_username: &str) {
    println!("Creating repository at {}", repo_path);
    run_cmd(&format!(
        "gh repo create {}/prive-note --private",
        github_username
    ));

    let target_dir = format!("{}/.prive/", env::var("HOME").unwrap());
    println!("Cloning repository to {}", target_dir);
    run_cmd(&format!(
        "gh repo clone {}/prive-note {}/{}",
        github_username, target_dir, github_username
    ));
}
