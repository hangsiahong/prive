use std::env;

use crate::run_cmd::run_cmd;

/// Pulls the latest changes from the repository
pub fn pull_repository(repo_path: &str) {
    if let Err(_) = env::set_current_dir(repo_path) {
        println!("Failed to change directory to {}", repo_path);
        return;
    }

    println!("Pulling repository from {}", repo_path);
    run_cmd("git pull");
}
