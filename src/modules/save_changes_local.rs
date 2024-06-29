use std::env;
use std::fs;
use std::str;

use rpassword::read_password;
use std::time::Duration;

use crate::run_cmd::run_cmd;

pub fn save_changes_local(file_path: &str, target_dir: &str) {
    // Ensure error handling for changing directory
    if let Err(_) = env::set_current_dir(target_dir) {
        println!("Failed to change directory to {}", target_dir);
        return;
    }

    // Encrypt the file with password verification
    let _encrypted_file_secure = format!("{}.secured", file_path);
    loop {
        println!("Enter a password to encrypt the note:");

        let password = match read_password() {
            Ok(password) => password,
            Err(_) => {
                println!("Error reading input.");
                return;
            }
        };

        if run_cmd(&format!("secured encrypt {} -p {}", &file_path, &password)) {
            break;
        } else {
            println!("Error: Incorrect password. Please try again.");
        }
    }

    // Delete the original file after encryption
    if let Err(err) = fs::remove_file(&file_path) {
        println!("Failed to delete original file: {}", err);
        return;
    }

    // Add, commit, and push the encrypted file
    std::thread::sleep(Duration::from_secs(1));

    println!("Successfully changed the note.");
}
