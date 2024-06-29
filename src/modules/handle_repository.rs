use std::{env, path::Path};

use crate::{create_repository::create_repository, pull_repository::pull_repository};

/// Handles the repository operations
pub fn handle_repository(github_username: &str) {
    let repo_path = format!("{}/.prive/", env::var("HOME").unwrap());
    let repo_exists = Path::new(&repo_path).exists();

    if repo_exists {
        println!("Repository 'prive-note' already exists. Pulling latest changes...");
        pull_repository(&repo_path);
    } else {
        println!("Repository 'prive-note' does not exist. Creating...");
        create_repository(&repo_path, github_username);
    }
}
