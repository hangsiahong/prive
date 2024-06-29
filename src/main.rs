use clap::Parser;

use std::io;
mod modules;
use modules::*;
use state::LoginState;
use state::NoteDatabase;

use crate::clone_repository::clone_repository;
use crate::handle_repository::handle_repository;
use crate::open_note::open_note;
use crate::run_interactive_menu::run_interactive_menu;
use get_github_username::get_github_username;
use login::login;

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

fn main() {
    let args = Args::parse();

    if let Some(note) = &args.open {
        open_note(&note);
    } else if let Some(clone_repo) = &args.clone {
        clone_repository(clone_repo);
    } else {
        let mut login_state = LoginState::load();

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
