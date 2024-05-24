use clap::Parser;
use rustyline::DefaultEditor;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use std::{env, io};

use std::error::Error;
use std::fs::OpenOptions;
use std::time::Duration;

/// Command-line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    open: Option<String>,

    #[arg(short, long)]
    /// Clone private access note from other Repository
    clone: Option<String>,
}

/// Struct to represent the login state
#[derive(Serialize, Deserialize)]
struct LoginState {
    logged_in: bool,
}

impl LoginState {
    /// Loads the login state from a configuration file
    fn load() -> Self {
        let config_dir = format!("{}/.prive-note", env::var("HOME").unwrap());
        let config_file = format!("{}/login_state.json", config_dir);

        if let Ok(mut file) = File::open(&config_file) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap_or(LoginState { logged_in: false })
        } else {
            LoginState { logged_in: false }
        }
    }

    /// Saves the login state to a configuration file
    fn save(&self) {
        let config_dir = format!("{}/.prive-note", env::var("HOME").unwrap());
        let config_file = format!("{}/login_state.json", config_dir);

        // Create the directory if it doesn't exist
        fs::create_dir_all(&config_dir).unwrap_or_else(|_| ());

        let json = serde_json::to_string(self).unwrap();
        let mut file = File::create(&config_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
struct NoteDatabase {
    password_hints: HashMap<String, String>,
}

impl NoteDatabase {
    /// Loads the note database from a configuration file
    fn load(repo_name: &str) -> Self {
        let db_file = format!(
            "{}/.prive/{}/note-db.json",
            env::var("HOME").unwrap(),
            repo_name
        );

        if let Ok(mut file) = File::open(&db_file) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap_or(NoteDatabase {
                password_hints: HashMap::new(),
            })
        } else {
            NoteDatabase {
                password_hints: HashMap::new(),
            }
        }
    }

    /// Saves the note database to a configuration file
    fn save(&self, repo_name: &str) {
        let db_file = format!(
            "{}/.prive/{}/note-db.json",
            env::var("HOME").unwrap(),
            repo_name
        );
        let serialized = serde_json::to_string(self).unwrap();
        fs::write(db_file, serialized).unwrap();
    }

    // Adjust the return type to Option<String> instead of Option<&str>
    fn get_password_hint(&self, file: &str) -> Option<String> {
        self.password_hints.get(file).cloned()
    }

    fn get_password_hint_with_default(&self, file: &str) -> String {
        self.get_password_hint(file)
            .map(String::from)
            .unwrap_or_else(|| "No hint".to_string())
    }

    fn set_password_hint(&mut self, file: &str, hint: String) {
        self.password_hints.insert(file.to_string(), hint);
    }
}

fn main() {
    let args = Args::parse();

    if let Some(note) = &args.open {
        // Dereference the String reference to get a &str
        open_note(&note);
    } else if let Some(clone_repo) = &args.clone {
        clone_repository(clone_repo);
    } else {
        let mut login_state = LoginState::load();

        // Check if the user is already logged in to GitHub
        if !login_state.logged_in {
            println!("Are you already logged in to GitHub? (yes/no)");
            let mut choice = String::new();
            if let Ok(_) = io::stdin().read_line(&mut choice) {
                match choice.trim().to_lowercase().as_str() {
                    "yes" => {
                        let github_username = get_github_username();
                        handle_repository(&github_username);
                        login_state.logged_in = true;
                        login_state.save();
                    }
                    "no" => {
                        if login() {
                            println!("Login successful. Proceeding to handle repositories...");
                            let github_username = get_github_username();
                            handle_repository(&github_username);
                            login_state.logged_in = true;
                            login_state.save();
                        } else {
                            println!("Login failed.");
                            return;
                        }
                    }
                    _ => {
                        println!("Invalid choice. Exiting...");
                        return;
                    }
                }
            } else {
                println!("Error reading input.");
                return;
            }
        }

        run_interactive_menu();
    }
}

/// Runs the interactive menu for the CLI application
fn run_interactive_menu() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut login_state = LoginState::load();

    loop {
        println!("Choose an option:");
        if !login_state.logged_in {
            // Display login options if not logged in
            println!("1. Login");
        } else {
            // Display other options if logged in
            println!("1. Create Note");
            println!("2. List All Notes");
            println!("3. Delete Note");
        }
        println!("4. Exit");

        if let Ok(line) = rl.readline("> ") {
            match line.trim().parse::<u32>() {
                Ok(choice) => match choice {
                    1 if !login_state.logged_in => {
                        if login() {
                            println!("Login successful. Proceeding to handle repositories...");
                            let github_username = get_github_username();
                            handle_repository(&github_username);
                            login_state.logged_in = true; // Set logged_in to true after successful login
                            login_state.save(); // Save the login state
                        } else {
                            println!("Login failed.");
                        }
                    }
                    1 if login_state.logged_in => create_note(),
                    2 if login_state.logged_in => list_notes(),
                    3 if login_state.logged_in => delete_note(),
                    4 => {
                        println!("Exiting...");
                        break;
                    }
                    _ => println!("Invalid option, please choose again."),
                },
                Err(_) => println!("Invalid input, please enter a number."),
            }
        } else {
            println!("Error reading input, please try again.");
        }
    }
}

/// Handles the login process
fn login() -> bool {
    println!("Logging in to GitHub...");
    // Implement GitHub login logic here
    let login_result: bool = run_cmd("gh auth login");

    // Assuming run_cmd returns true if successful, false otherwise
    return login_result;
}

/// Handles the repository operations
fn handle_repository(github_username: &str) {
    let repo_path = format!("{}/.prive/", env::var("HOME").unwrap());
    let repo_exists = Path::new(&repo_path).exists();

    if repo_exists {
        println!("Repository 'prive-note' already exists. Pulling latest changes...");
        pull_repository(&repo_path);
    } else {
        println!("Repository 'prive-note' does not exist. Creating...");
        create_repository(&repo_path, github_username);
    }
}

/// Pulls the latest changes from the repository
fn pull_repository(repo_path: &str) {
    if let Err(_) = env::set_current_dir(repo_path) {
        println!("Failed to change directory to {}", repo_path);
        return;
    }

    println!("Pulling repository from {}", repo_path);
    run_cmd("git pull");
}

/// Creates a new repository
fn create_repository(repo_path: &str, github_username: &str) {
    println!("Creating repository at {}", repo_path);
    run_cmd(&format!(
        "gh repo create {}/prive-note --private",
        github_username
    ));

    let target_dir = format!("{}/.prive/", env::var("HOME").unwrap());
    println!("Cloning repository to {}", target_dir);
    run_cmd(&format!(
        "gh repo clone {}/prive-note {}/{}",
        github_username, target_dir, github_username
    ));
}

/// Runs a command in the shell
fn run_cmd(command: &str) -> bool {
    let args: Vec<&str> = command.split_whitespace().collect();
    let result = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                // println!("Command executed successfully.");
                true // Command executed successfully, return true
            } else {
                // println!("Failed to execute command.");
                false // Command execution failed, return false
            }
        }
        Err(_) => {
            println!("Error running command.");
            false // Error running command, return false
        }
    }
}

/// Runs a command in the shell
fn run_cmd_result(command: &str) -> Result<(), Box<dyn Error>> {
    let args: Vec<&str> = command.split_whitespace().collect();
    let result = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if result.status.success() {
        // println!("Command executed successfully.");
        Ok(())
    } else {
        println!("Failed to execute command.");
        Err("Command execution failed.".into())
    }
}

fn list_notes() {
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

// Helper function to list notes in a specific repository
fn list_notes_in_repository(repo_name: &str) {
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
/// Opens a file in Vim for editing
fn open_file_in_vim(note_dir: &str, file_name: &str) -> bool {
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
    println!("1. Save changes");
    println!("2. Discard changes");

    let mut choice = String::new();
    if let Ok(_) = io::stdin().read_line(&mut choice) {
        match choice.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => {
                    save_changes(&decrypted_file_path, &note_dir);
                    true
                }
                2 => {
                    println!("Changes discarded.");
                    false
                }
                _ => {
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

/// Saves changes made to a file
fn save_changes(file_path: &str, target_dir: &str) {
    // Ensure error handling for changing directory
    if let Err(_) = env::set_current_dir(target_dir) {
        println!("Failed to change directory to {}", target_dir);
        return;
    }

    // Encrypt the file with password verification
    let encrypted_file_secure = format!("{}.secured", file_path);
    loop {
        println!("Enter a password to encrypt the note:");
        let mut password = String::new();
        if let Ok(_) = io::stdin().read_line(&mut password) {
            password = password.trim().to_string();
            if run_cmd(&format!("secured encrypt {} -p {}", &file_path, &password)) {
                break;
            } else {
                println!("Error: Incorrect password. Please try again.");
            }
        } else {
            println!("Error reading input.");
            return;
        }
    }

    // Delete the original file after encryption
    if let Err(err) = fs::remove_file(&file_path) {
        println!("Failed to delete original file: {}", err);
        return;
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

fn create_note() {
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

fn create_note_in_repo(repo_name: &str) {
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
            let encrypted_file_path = format!("{}.secured", file_path);
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

/// Deletes a note from a selected repository
fn delete_note() {
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

            println!("Select a repository to delete notes from:");

            for (index, repo) in repositories.iter().enumerate() {
                println!("{}. {}", index + 1, repo);
            }

            let mut choice = String::new();

            if let Ok(_) = io::stdin().read_line(&mut choice) {
                if let Ok(choice) = choice.trim().parse::<usize>() {
                    if choice > 0 && choice <= repositories.len() {
                        let selected_repo = &repositories[choice - 1];
                        println!("Deleting notes for repository: {}", selected_repo);
                        delete_notes_in_repository(selected_repo);
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

fn delete_notes_in_repository(repo_name: &str) {
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
                        env::set_current_dir(note_dir);

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

/// Opens a note in Vim, encrypts it back after editing and exiting Vim, and deletes the original note file
fn open_note(note: &str) {
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

// Function to get the GitHub username from the `gh auth status` command
fn get_github_username() -> String {
    let output = Command::new("gh")
        .arg("auth")
        .arg("status")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
        // Parse the output to extract the username
        for line in stdout.lines() {
            if line.contains("Logged in to github.com account") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(username) = parts.get(6) {
                    return username.to_string();
                }
            }
        }
    }

    // Return an empty string if the username cannot be extracted
    String::new()
}

fn clone_repository(repo: &str) {
    let repo_path = format!("{}/.prive/{}", env::var("HOME").unwrap(), repo);

    println!("Cloning repository from {}", repo);

    run_cmd(&format!("gh repo clone {}/prive-note {}", repo, repo_path));
}
