// proj_01_cli_todo — a JSON-backed todo CLI (clap + serde).
// Examples:
//   cargo run --bin proj_01_cli_todo -- add "buy milk"
//   cargo run --bin proj_01_cli_todo -- list
//   cargo run --bin proj_01_cli_todo -- done 1
//   cargo run --bin proj_01_cli_todo -- remove 1
mod store;

use anyhow::Result;
use clap::{Parser, Subcommand};
use store::{Store, default_path};

// clap derive builds the parser from these structs (like decorators in TS).
#[derive(Parser)]
#[command(name = "todo", about = "A tiny JSON-backed todo list")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new task
    Add { text: String },
    /// List all tasks
    List,
    /// Mark a task done by id
    Done { id: u32 },
    /// Remove a task by id
    Remove { id: u32 },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = default_path();
    let mut store = Store::load(&path)?;

    match cli.command {
        Command::Add { text } => {
            let id = store.add(text);
            store.save(&path)?;
            println!("added #{id}");
        }
        Command::List => {
            if store.tasks.is_empty() {
                println!("(no tasks)");
            }
            for t in &store.tasks {
                let mark = if t.done { "x" } else { " " };
                println!("[{mark}] #{} {}", t.id, t.text);
            }
        }
        Command::Done { id } => {
            if store.complete(id) {
                store.save(&path)?;
                println!("completed #{id}");
            } else {
                println!("no task #{id}");
            }
        }
        Command::Remove { id } => {
            if store.remove(id) {
                store.save(&path)?;
                println!("removed #{id}");
            } else {
                println!("no task #{id}");
            }
        }
    }
    Ok(())
}
