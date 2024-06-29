use std::{env, fs, io, path::Path};

use crate::create_note_in_repo::create_note_in_repo;

pub fn create_note() {
    let prive_dir = format!("{}/.prive", env::var("HOME").unwrap());

    // Check if .prive directory exists
    if !Path::new(&prive_dir).exists() {
        println!("Error: The .prive directory doesn't exist.");
        return;
    }

    // List available repositories
    println!("Choose a repository to create the note in:");
    let mut repos = Vec::new();
    if let Ok(entries) = fs::read_dir(&prive_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name != "note-db.json" {
                        repos.push(file_name.to_string());
                    }
                }
            }
        }
    }

    // Display available repositories
    for (index, repo) in repos.iter().enumerate() {
        println!("{}. {}", index + 1, repo);
    }

    // Prompt user to choose a repository
    let mut choice = String::new();
    if let Ok(_) = io::stdin().read_line(&mut choice) {
        if let Ok(repo_index) = choice.trim().parse::<usize>() {
            if repo_index > 0 && repo_index <= repos.len() {
                let selected_repo = &repos[repo_index - 1];

                // Proceed to create the note in the selected repository
                create_note_in_repo(selected_repo);
                return;
            }
        }
    }

    println!("Invalid choice. Exiting...");
}
