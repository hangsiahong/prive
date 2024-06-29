use crate::run_cmd::run_cmd;

/// Handles the login process
pub fn login() -> bool {
    println!("Logging in to GitHub...");
    // Implement GitHub login logic here
    let login_result: bool = run_cmd("gh auth login");

    // Assuming run_cmd returns true if successful, false otherwise
    return login_result;
}
