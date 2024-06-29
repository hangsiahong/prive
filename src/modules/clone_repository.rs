use std::env;

use crate::run_cmd::run_cmd;

pub fn clone_repository(repo: &str) {
    let repo_path = format!("{}/.prive/{}", env::var("HOME").unwrap(), repo);

    println!("Cloning repository from {}", repo);

    run_cmd(&format!("gh repo clone {}/prive-note {}", repo, repo_path));
}
