use std::{env, fs, io, time::Duration};

use crate::run_cmd::run_cmd;

pub fn delete_notes_in_repository(repo_name: &str) {
    let note_dir = format!("{}/.prive/{}", env::var("HOME").unwrap(), repo_name);

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

            println!("Select a secured note to delete:");
            for (index, file) in secured_files.iter().enumerate() {
                println!("{}. {}", index + 1, file);
            }

            let mut choice = String::new();

            if let Ok(_) = io::stdin().read_line(&mut choice) {
                if let Ok(choice) = choice.trim().parse::<usize>() {
                    if choice > 0 && choice <= secured_files.len() {
                        let selected_file = &secured_files[choice - 1];
                        println!("Deleting note: {}", selected_file);

                        // Construct the file path correctly
                        let file_to_delete = format!("{}/{}", note_dir, selected_file);
                        println!("File to delete: {}", file_to_delete);

                        // Delete the selected note file
                        if let Err(err) = fs::remove_file(&file_to_delete) {
                            println!("Failed to delete note file: {}", err);
                            return;
                        }

                        // set the current directory
                        env::set_current_dir(note_dir).unwrap();

                        if !run_cmd(&format!("git add {}", file_to_delete)) {
                            println!("Failed to stage changes.");
                            return;
                        }

                        std::thread::sleep(Duration::from_secs(1));
                        if !run_cmd("git commit -m 'update'") {
                            println!("Failed to commit changes.");
                            return;
                        }

                        std::thread::sleep(Duration::from_secs(1));
                        if !run_cmd("git push origin main") {
                            println!("Failed to push changes.");
                            return;
                        }

                        println!("Changes committed and pushed successfully.");

                        println!("Note '{}' deleted successfully.", selected_file);
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
