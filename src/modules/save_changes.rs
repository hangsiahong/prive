use std::{env, time::Duration};

use rpassword::read_password;

use crate::run_cmd::run_cmd;

/// Saves changes made to a file
pub fn save_changes(file_path: &str, target_dir: &str) {
    // Ensure error handling for changing directory
    if let Err(_) = env::set_current_dir(target_dir) {
        println!("Failed to change directory to {}", target_dir);
        return;
    }

    // Encrypt the file with password verification
    let encrypted_file_secure = format!("{}.secured", file_path);
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
        // Add, commit, and push the encrypted file
        std::thread::sleep(Duration::from_secs(1));

        if !run_cmd(&format!("git add {}", encrypted_file_secure)) {
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
    }
}
