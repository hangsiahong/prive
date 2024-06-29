use rustyline::DefaultEditor;

use crate::create_note::create_note;
use crate::delete_note::delete_note;
use crate::get_github_username::get_github_username;
use crate::handle_repository::handle_repository;
use crate::list_note::list_notes;
use crate::login::login;
use crate::LoginState;

/// Runs the interactive menu for the CLI application
pub fn run_interactive_menu() {
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
