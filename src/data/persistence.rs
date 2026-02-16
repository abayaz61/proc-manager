use std::collections::HashSet;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProcessState {
    #[serde(default)]
    pub pinned: Vec<String>,
    #[serde(default)]
    pub hidden: Vec<String>,
}

impl ProcessState {
    pub fn load() -> Self {
        if let Some(path) = state_file_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(state) = toml::from_str(&content) {
                        return state;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = state_file_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(content) = toml::to_string_pretty(self) {
                let _ = std::fs::write(&path, content);
            }
        }
    }

    pub fn pinned_set(&self) -> HashSet<String> {
        self.pinned.iter().cloned().collect()
    }

    pub fn hidden_set(&self) -> HashSet<String> {
        self.hidden.iter().cloned().collect()
    }
}

fn state_file_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "proc-manager", "proc-manager")
        .map(|dirs| dirs.data_dir().join("state.toml"))
}
