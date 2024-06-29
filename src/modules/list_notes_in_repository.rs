use std::{env, fs, io};

use crate::{open_file_in_vim::open_file_in_vim, NoteDatabase};

// Helper function to list notes in a specific repository
pub fn list_notes_in_repository(repo_name: &str) {
    let note_dir = format!("{}/.prive/{}", env::var("HOME").unwrap(), repo_name);
    let note_db = NoteDatabase::load(repo_name);

    match fs::read_dir(&note_dir) {
        Ok(dir_contents) => {
            let secured_files: Vec<String> = dir_contents
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        if let Some(name) = e.file_name().to_str() {
                            if name.ends_with(".secured") {
                                Some(name.to_owned())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                })
                .collect();

            if secured_files.is_empty() {
                println!("No secured notes found in {}.", note_dir);
                return;
            }

            println!("Select a secured note to view:");
            for (index, file) in secured_files.iter().enumerate() {
                let hint = note_db.get_password_hint_with_default(file);
                println!("{}. {} (Password Hint: {})", index + 1, file, hint);
            }

            let mut choice = String::new();

            if let Ok(_) = io::stdin().read_line(&mut choice) {
                if let Ok(choice) = choice.trim().parse::<usize>() {
                    if choice > 0 && choice <= secured_files.len() {
                        let selected_file = &secured_files[choice - 1];
                        println!("Viewing note: {}", selected_file);
                        open_file_in_vim(&note_dir, selected_file);
                    } else {
                        println!(
                            "Invalid choice. Please enter a number between 1 and {}.",
                            secured_files.len()
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
                "Failed to list secured notes. The directory may not exist or is inaccessible."
            );
        }
    }
}
