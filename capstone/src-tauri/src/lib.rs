// Rust core for the capstone Tauri app.
// Three commands exposed to the React frontend, backed by in-memory shared state
// (Arc<Mutex<...>> via tauri::State) — the same ownership concepts from learn_02
// and learn_11 applied in a real app.
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    text: String,
    done: bool,
}

#[derive(Default)]
struct AppState {
    todos: Mutex<Vec<Todo>>,
    next_id: Mutex<u32>,
}

#[tauri::command]
fn list_todos(state: State<AppState>) -> Vec<Todo> {
    state.todos.lock().unwrap().clone()
}

#[tauri::command]
fn add_todo(state: State<AppState>, text: String) {
    let mut id = state.next_id.lock().unwrap();
    *id += 1;
    state.todos.lock().unwrap().push(Todo {
        id: *id,
        text,
        done: false,
    });
}

#[tauri::command]
fn toggle_todo(state: State<AppState>, id: u32) {
    if let Some(t) = state.todos.lock().unwrap().iter_mut().find(|t| t.id == id) {
        t.done = !t.done;
    }
}

// Entry point used by both desktop and mobile targets.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![list_todos, add_todo, toggle_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
