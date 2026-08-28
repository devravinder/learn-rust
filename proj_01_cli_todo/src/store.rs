// store.rs — data model + JSON file persistence for the todo app.
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// #[derive(Serialize, Deserialize)] wires the struct to serde (JSON <-> Rust).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Store {
    pub tasks: Vec<Task>,
}

impl Store {
    // Load from disk; return an empty store if the file doesn't exist yet.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Store::default());
        }
        let data = fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        let store = serde_json::from_str(&data).context("parsing todo JSON")?;
        Ok(store)
    }

    // Save pretty-printed JSON back to disk.
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("serializing todos")?;
        fs::write(path, json).with_context(|| format!("writing {}", path.display()))?;
        Ok(())
    }

    pub fn add(&mut self, text: String) -> u32 {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.tasks.push(Task { id, text, done: false });
        id
    }

    // Returns true if a task with `id` was found and marked done.
    pub fn complete(&mut self, id: u32) -> bool {
        for t in &mut self.tasks {
            if t.id == id {
                t.done = true;
                return true;
            }
        }
        false
    }

    // Returns true if a task was removed.
    pub fn remove(&mut self, id: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() != before
    }
}

// Default storage location: ./todos.json next to where you run the command.
pub fn default_path() -> PathBuf {
    PathBuf::from("todos.json")
}
