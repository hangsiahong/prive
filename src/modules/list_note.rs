use std::{env, fs, io};

use crate::list_notes_in_repository::list_notes_in_repository;

pub fn list_notes() {
    let prive_dir = format!("{}/.prive", env::var("HOME").unwrap());

    // List repositories in the .prive directory
    match fs::read_dir(&prive_dir) {
        Ok(dir_contents) => {
            let repositories: Vec<String> = dir_contents
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if let Some(name) = e.file_name().to_str() {
                            Some(name.to_owned())
                        } else {
                            None
                        }
                    })
                })
                .collect();

            if repositories.is_empty() {
                println!("No repositories found in ~/.prive.");
                return;
            }

            println!("Select a repository to view notes:");

            for (index, repo) in repositories.iter().enumerate() {
                println!("{}. {}", index + 1, repo);
            }

            let mut choice = String::new();

            if let Ok(_) = io::stdin().read_line(&mut choice) {
                if let Ok(choice) = choice.trim().parse::<usize>() {
                    if choice > 0 && choice <= repositories.len() {
                        let selected_repo = &repositories[choice - 1];
                        println!("Listing notes for repository: {}", selected_repo);
                        list_notes_in_repository(selected_repo);
                    } else {
                        println!(
                            "Invalid choice. Please enter a number between 1 and {}.",
                            repositories.len()
                        );
                    }
                } else {
                    println!("Invalid input. Please enter a number.");
                }
            } else {
                println!("Error reading input.");
            }
        }
        Err(_) => {
            println!(
                "Failed to list repositories. The directory may not exist or is inaccessible."
            );
        }
    }
}
