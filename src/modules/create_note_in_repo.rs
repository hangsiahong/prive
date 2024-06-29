use std::{env, fs::OpenOptions, io, time::Duration};

use crate::{run_cmd::run_cmd, run_cmd_result::run_cmd_result, NoteDatabase};

use std::io::Write;

pub fn create_note_in_repo(repo_name: &str) {
    println!("Creating note in repository: {}", repo_name);

    let note_dir = format!("{}/.prive/{}", env::var("HOME").unwrap(), repo_name);
    println!("Note directory: {}", note_dir);
    let mut note_db = NoteDatabase::load(repo_name);
    println!("Note database loaded");

    // Set current directory to the note directory
    if let Err(_) = env::set_current_dir(&note_dir) {
        println!("Failed to change directory to {}", note_dir);
        return;
    }
    println!("Changed directory to {}", note_dir);

    println!("Enter the name of the new note:");
    let mut note_name = String::new();
    if let Ok(_) = io::stdin().read_line(&mut note_name) {
        println!("Note name entered: {}", note_name);
        let note_name = note_name.trim();
        if note_name.is_empty() {
            println!("Error: Note name cannot be empty.");
            return;
        }

        let file_path = format!("{}/{}", note_dir, note_name);
        // Encrypt note name
        let secured_note_name = format!("{}.secured", note_name);
        // Prompt the user to enter a password
        println!("Enter a password for the note:");
        let mut password = String::new();
        if let Ok(_) = io::stdin().read_line(&mut password) {
            // Remove newline characters from the password
            password = password.trim().to_string();

            println!("Do you want to set a password hint? (yes/no)");
            let mut hint_choice = String::new();
            if let Ok(_) = io::stdin().read_line(&mut hint_choice) {
                if hint_choice.trim().eq_ignore_ascii_case("yes") {
                    println!("Enter the password hint:");
                    let mut password_hint = String::new();
                    if let Ok(_) = io::stdin().read_line(&mut password_hint) {
                        note_db.set_password_hint(
                            &secured_note_name,
                            password_hint.trim().to_string(),
                        );
                        note_db.save(repo_name);
                    }
                }
            }

            // Create the note file with the template content
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&file_path)
            {
                Ok(mut file) => {
                    // Write template content to the file
                    if let Err(e) = writeln!(file, "Title: {}", note_name) {
                        println!("Failed to write template content to file: {}", e);
                        // Delete the file if writing the template content fails
                        if let Err(e) = std::fs::remove_file(&file_path) {
                            println!("Failed to delete file: {}", e);
                        }
                        return;
                    }
                    println!("Note '{}' created successfully.", note_name);
                }
                Err(e) => {
                    println!("Failed to create note: {}", e);
                    return;
                }
            }

            // Encrypt the note file
            let _encrypted_file_path = format!("{}.secured", file_path);
            match run_cmd_result(&format!("secured encrypt {} -p {}", &file_path, &password)) {
                Ok(_) => println!("Note '{}' encrypted successfully.", note_name),
                Err(e) => {
                    println!("Failed to encrypt note file: {}", e);
                    // Delete the unencrypted file if encryption fails
                    if let Err(e) = std::fs::remove_file(&file_path) {
                        println!("Failed to delete unencrypted file: {}", e);
                    }
                }
            }
        } else {
            println!("Failed to read password input.");
        }
    } else {
        println!("Failed to read input.");
    }

    // After encrypting and pushing the encrypted file
    println!("Changes committed and pushed successfully.");

    std::thread::sleep(Duration::from_secs(1));
    // git commit note-db.json to github
    // Add, commit, and push the encrypted file
    run_cmd("git add note-db.json");
    std::thread::sleep(Duration::from_secs(1));
    run_cmd("git commit -m 'update'");
    std::thread::sleep(Duration::from_secs(1));
    run_cmd("git push origin main");
}
