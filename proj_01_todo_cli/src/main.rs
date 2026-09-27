/*
 Interactive CLI todo app.

 Run from this folder ( rust\learn-rust\proj_01_todo_cli ):
   cargo run                      -> no args, shows interactive menu
   cargo run -- add "Buy milk"    -> runs a subcommand directly
   cargo run -- list
   cargo run -- done 2
   cargo run -- remove 3

 For now every action just prints a message (no persistence yet).
*/

mod store;


use std::io::{self, Write};
use std::path::Path;
use anyhow::Result;
use clap::{Parser, Subcommand};
use store::{Store, default_path};




#[derive(Parser, Debug)]
#[command(name = "todo", about = "A tiny interactive todo list")]
struct Cli {
    /// The command to run. If omitted, an interactive menu is shown.
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Add {
        task: String,
    },
    List,
    Done {
        id: u32,
    },
    Remove {
        id: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = default_path();
    let mut store = Store::load(&path)?;

    match cli.command {
        // Args were passed -> run that command straight away.
        Some(command) => run(command, &mut store, &path),
        // No args -> drop into the interactive menu.
        None => interactive_menu(&mut store, &path),
    }
}

/// Execute a single command against the store, persisting any changes.
fn run(command: Command, store: &mut Store, path: &Path) -> Result<()> {
    match command {
        Command::Add { task } => {
            let id = store.add(task.clone());
            store.save(path)?;
            println!("[add]   added task #{id}: {task}");
        }
        Command::List => {
            if store.tasks.is_empty() {
                println!("[list]   no todos yet — add one!");
            } else {
                println!("[list]   your todos:");
                for t in &store.tasks {
                    let mark = if t.done { "x" } else { " " };
                    println!("  {}. [{mark}] {}", t.id, t.text);
                }
            }
        }
        Command::Done { id } => {
            if store.complete(id) {
                store.save(path)?;
                println!("[done]   marked task #{id} as done");
            } else {
                println!("[done]   no task with id #{id}");
            }
        }
        Command::Remove { id } => {
            if store.remove(id) {
                store.save(path)?;
                println!("[remove] removed task #{id}");
            } else {
                println!("[remove] no task with id #{id}");
            }
        }
    }
    Ok(())
}

/// Interactive loop shown when the program is started without arguments.
fn interactive_menu(store: &mut Store, path: &Path) -> Result<()> {
    println!("Welcome to the interactive todo app!");

    loop {
        println!();
        println!("What would you like to do?");
        println!("  1) Add a task");
        println!("  2) List tasks");
        println!("  3) Mark a task as done");
        println!("  4) Remove a task");
        println!("  5) Quit");
        print!("Enter choice [1-5]: ");
        io::stdout().flush()?;

        let choice = read_line()?;
        println!("");

        match choice.trim() {
            "1" => {
                let task = prompt("Task text: ")?;
                run(Command::Add { task }, store, path)?;
                println!("");
            }
            "2" => {
                run(Command::List, store, path)?;
                println!("");
            }
            "3" => {
                let id = prompt_id("Task id to complete: ")?;
                run(Command::Done { id }, store, path)?;
                println!("");

            }
            "4" => {
                let id = prompt_id("Task id to remove: ")?;
                run(Command::Remove { id }, store, path)?;
                println!("");

            }
            "5" | "q" | "quit" | "exit" => {
                println!("Bye!");
                println!("");
                break;
            }
            other => println!("Unknown choice: '{other}'. Please pick 1-5."),
        }
    }

    Ok(())
}

/// Print a prompt and return the trimmed line the user typed.
fn prompt(label: &str) -> Result<String> {
    print!("{label}");
    io::stdout().flush()?;
    Ok(read_line()?.trim().to_string())
}

/// Prompt until the user enters a valid unsigned number.
fn prompt_id(label: &str) -> Result<u32> {
    loop {
        let input = prompt(label)?;
        match input.parse::<u32>() {
            Ok(id) => return Ok(id),
            Err(_) => println!("'{input}' is not a valid id, try again."),
        }
    }
}

/// Read a single line from stdin.
fn read_line() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}
