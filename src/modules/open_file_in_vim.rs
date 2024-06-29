use std::fs;
use std::io;
use std::process::Command;
use std::str;

use crate::run_cmd::run_cmd;
use crate::save_changes::save_changes;
use crate::save_changes_local::save_changes_local;

/// Opens a file in Vim for editing
pub fn open_file_in_vim(note_dir: &str, file_name: &str) -> bool {
    let secured_file_path = format!("{}/{}", note_dir, file_name);
    let decrypted_file_path = format!("{}/{}", note_dir, &file_name[..file_name.len() - 8]); // Remove ".secured" from file name

    // Decrypt the secured file
    if !run_cmd(&format!("secured decrypt {}", secured_file_path)) {
        println!("Error: Failed to decrypt the file.");
        return false;
    }

    // Open the decrypted file in Vim
    match Command::new("nvim").arg(&decrypted_file_path).status() {
        Ok(status) => {
            if !status.success() {
                println!("Failed to open the file in Vim.");
                return false;
            }
        }
        Err(err) => {
            println!("Error opening note: {}", err);
            return false;
        }
    }

    println!("Editing session finished. Do you want to save changes?");
    println!("1. Save changes and push to github");
    println!("2. Save on local only");

    let mut choice = String::new();
    if let Ok(_) = io::stdin().read_line(&mut choice) {
        match choice.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => {
                    save_changes(&decrypted_file_path, &note_dir);
                    true
                }
                2 => {
                    save_changes_local(&decrypted_file_path, &note_dir);
                    true
                }
                _ => {
                    // Delete the original file after encryption
                    if let Err(err) = fs::remove_file(&decrypted_file_path) {
                        println!("Failed to delete original file: {}", err);
                        return false;
                    }
                    println!("Invalid choice, please enter 1 or 2.");
                    false
                }
            },
            Err(_) => {
                println!("Invalid input, please enter a number.");
                false
            }
        }
    } else {
        println!("Error reading input.");
        false
    }
}
