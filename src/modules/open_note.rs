use std::fs;

use std::env;
use std::path::Path;
use std::process::Command;
use std::str;

/// Opens a note in Vim, encrypts it back after editing and exiting Vim, and deletes the original note file
pub fn open_note(note: &str) {
    let note_path = format!("{}/.prive-note/{}", env::var("HOME").unwrap(), note);
    let encrypted_note_path = format!("{}.secured", note_path);

    // Check if the original note file exists
    if Path::new(&note_path).exists() {
        // Open the note in Vim
        if let Err(err) = Command::new("vim").arg(&note_path).status() {
            eprintln!("Error opening note: {}", err);
            return;
        }

        // After exiting Vim, encrypt the note back
        if let Err(err) = Command::new("secured")
            .args(&["encrypt", &note_path])
            .status()
        {
            eprintln!("Error encrypting note: {}", err);
        }

        // Delete the original note file
        if let Err(err) = fs::remove_file(&note_path) {
            eprintln!("Error deleting original note: {}", err);
        }
    } else {
        // If the original note file doesn't exist, check if the corresponding encrypted note exists
        if Path::new(&encrypted_note_path).exists() {
            // Encrypted note exists, remove it
            if let Err(err) = fs::remove_file(&encrypted_note_path) {
                eprintln!("Error removing encrypted note: {}", err);
            }
        }
    }
}
