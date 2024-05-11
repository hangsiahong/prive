use clap::Parser;
use rustyline::DefaultEditor;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, io};

/// Command-line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    open: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(note) = args.open {
        open_note(&note);
    } else {
        run_interactive_menu();
    }
}

/// Runs the interactive menu for the CLI application
fn run_interactive_menu() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        println!("Choose an option:");
        println!("1. Login");
        println!("2. Already logged in");
        println!("3. List notes");
        println!("4. Create note");
        println!("5. Exit");

        if let Ok(line) = rl.readline("> ") {
            match line.trim().parse::<u32>() {
                Ok(choice) => match choice {
                    1 => login(),
                    2 => {
                        println!("You are already logged in.");
                        handle_repository();
                        break;
                    }
                    3 => list_notes(),
                    4 => create_note(),
                    5 => {
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
fn login() {
    println!("Logging in to GitHub...");
    // Implement GitHub login logic here
}

/// Handles the repository operations
fn handle_repository() {
    let repo_path = format!("{}/.prive-note", env::var("HOME").unwrap());
    let repo_exists = Path::new(&repo_path).exists();

    if repo_exists {
        println!("Repository 'prive-note' already exists. Pulling latest changes...");
        pull_repository(&repo_path);
    } else {
        println!("Repository 'prive-note' does not exist. Creating...");
        create_repository(&repo_path);
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
fn create_repository(repo_path: &str) {
    println!("Creating repository at {}", repo_path);
    run_cmd("gh repo create prive-note --private");

    let target_dir = format!("{}/.prive-note", env::var("HOME").unwrap());
    println!("Cloning repository to {}", target_dir);
    run_cmd(&format!("gh repo clone prive-note {}", target_dir));
}

/// Runs a command in the shell
fn run_cmd(command: &str) {
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
                println!("Command executed successfully.");
            } else {
                println!("Failed to execute command.");
            }
        }
        Err(_) => {
            println!("Error running command.");
        }
    }
}

fn list_notes() {
    let note_dir = format!("{}/.prive-note", env::var("HOME").unwrap());

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
                println!("No secured notes found in ~/.prive-note.");
                return;
            }

            println!("Select a secured note to view:");
            for (index, file) in secured_files.iter().enumerate() {
                println!("{}. {}", index + 1, file);
            }

            let mut choice = String::new(); // Change to String

            if let Ok(_) = io::stdin().read_line(&mut choice) {
                // Read into String
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
fn open_file_in_vim(note_dir: &str, file_name: &str) {
    let secured_file_path = format!("{}/{}", note_dir, file_name);
    let decrypted_file_path = format!("{}/{}", note_dir, &file_name[..file_name.len()-8]); // Remove ".secured" from file name

    // Decrypt the secured file
    run_cmd(&format!("secured decrypt {}", secured_file_path));

    // Open the decrypted file in Vim
    match Command::new("vim")
        .arg(&decrypted_file_path)
        .status()
    {
        Ok(status) => {
            if !status.success() {
                println!("Failed to open the file in Vim.");
            }
        }
        Err(err) => {
            println!("Error opening note: {}", err);
        }
    }

    println!("Editing session finished. Do you want to save changes?");
    println!("1. Save changes");
    println!("2. Discard changes");

    let mut choice = String::new();
    if let Ok(_) = io::stdin().read_line(&mut choice) {
        match choice.trim().parse::<u32>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        save_changes(&decrypted_file_path);
                    }
                    2 => {
                        println!("Changes discarded.");
                    }
                    _ => println!("Invalid choice, please enter 1 or 2."),
                }
            }
            Err(_) => println!("Invalid input, please enter a number."),
        }
    } else {
        println!("Error reading input.");
    }
}

/// Saves changes made to a file
fn save_changes(file_path: &str) {
    let target_dir = format!("{}/.prive-note/", env::var("HOME").unwrap());

    if let Err(_) = env::set_current_dir(&target_dir) {
        println!("Failed to change directory to {}", target_dir);
        return;
    }

    // Encrypt the file
    let encrypted_file = format!("{}", file_path);
    let encrypted_file_secure = format!("{}.secured", file_path);
    run_cmd(&format!("secured encrypt {}", &file_path));
    // Delete the original after encrypt
    run_cmd(&format!("rm -rf {}", &file_path));

    // Add, commit, and push the encrypted file
    run_cmd(&format!("git add {}", encrypted_file_secure));
    run_cmd("git commit -m 'update'");
    run_cmd("git push origin main");

    println!("Changes committed and pushed successfully.");
}
/// Creates a new note
fn create_note() {
    let note_dir = format!("{}/.prive-note", env::var("HOME").unwrap());

    if !Path::new(&note_dir).exists() {
        println!("Error: The note directory doesn't exist.");
        return;
    }

    println!("Enter the name of the new note:");
    let mut note_name = String::new();
    if let Ok(_) = io::stdin().read_line(&mut note_name) {
        let note_name = note_name.trim();
        if note_name.is_empty() {
            println!("Error: Note name cannot be empty.");
            return;
        }

        let file_path = format!("{}/{}.secured", note_dir, note_name);
        match File::create(&file_path) {
            Ok(_) => println!("Note '{}' created successfully.", note_name),
            Err(e) => println!("Failed to create note: {}", e),
        }
    } else {
        println!("Failed to read input.");
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
        if let Err(err) = Command::new("secured").args(&["encrypt", &note_path]).status() {
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
