use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct LoginState {
    pub logged_in: bool,
}

impl LoginState {
    pub fn load() -> Self {
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

    pub fn save(&self) {
        let config_dir = format!("{}/.prive-note", env::var("HOME").unwrap());
        let config_file = format!("{}/login_state.json", config_dir);

        fs::create_dir_all(&config_dir).unwrap_or_else(|_| ());

        let json = serde_json::to_string(self).unwrap();
        let mut file = File::create(&config_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct NoteDatabase {
    pub password_hints: HashMap<String, String>,
}

impl NoteDatabase {
    pub fn load(repo_name: &str) -> Self {
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

    pub fn save(&self, repo_name: &str) {
        let db_file = format!(
            "{}/.prive/{}/note-db.json",
            env::var("HOME").unwrap(),
            repo_name
        );
        let serialized = serde_json::to_string(self).unwrap();
        fs::write(db_file, serialized).unwrap();
    }

    pub fn get_password_hint(&self, file: &str) -> Option<String> {
        self.password_hints.get(file).cloned()
    }

    pub fn get_password_hint_with_default(&self, file: &str) -> String {
        self.get_password_hint(file)
            .map(String::from)
            .unwrap_or_else(|| "No hint".to_string())
    }

    pub fn set_password_hint(&mut self, file: &str, hint: String) {
        self.password_hints.insert(file.to_string(), hint);
    }
}