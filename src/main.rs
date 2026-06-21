mod task;
mod task_store;

use std::io::{self, Write};
use crate::{task_store::TaskStore};
use colored::*;


fn main() {
    let mut store = TaskStore::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        show_tasks(&store);
        println!("\n{}", "--- TODO CLI ---".bright_cyan().bold());
        println!("{}", "1. Add".bright_blue());
        println!("{}", "2. Toggle status".bright_blue());
        println!("{}", "3. Edit description".bright_blue());
        println!("{}", "4. Remove".bright_blue());
        println!("{}", "5. Exit".bright_blue());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");

        match choice.trim() {
            "1" => {
                let id = match get_valid_id() {
                    Some(val) => val,
                    None => continue,
                };
                let mut desc = String::new();

                print!("Description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut desc).expect("Failed to read"); 
                print_feedback(
                    store.create_task(id, desc.trim().to_string()),
                    "Task created successfully!",
                    "A task with this ID already exists."
                );
            }

            "2" => {
                let id = match get_valid_id() {
                    Some(val) => val,
                    None => continue,
                };
                print_feedback(
                    store.toggle_status(id),
                    "Task status changed!",
                    "Task not found."
                );
            }

            "3" => {
                let id = match get_valid_id() {
                    Some(val) => val,
                    None => continue,
                };
                let mut desc = String::new();

                print!("New description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut desc).expect("Failed to read");

                print_feedback(
                    store.set_description(id, desc.trim().to_string()),
                    "Description changed!",
                    "Task not found."
                );
            }

            "4" => {
                let id = match get_valid_id() {
                    Some(val) => val,
                    None => continue,
                };
                print_feedback(
                    store.remove(id),
                    "Task removed successfully!",
                    "Task not found."
                );
            }

            "5" => break,

            _ => println!("{}", "Invalid option".red()),
        }

        println!("\nPress Enter to continue...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}

fn print_feedback(success: bool, msg_ok: &str, msg_err: &str) {
    if success {
        println!("{}", format!("Success: {}", msg_ok).green());
    } else {
        println!("{}", format!("ERROR: {}", msg_err).red());
    }
}

fn read_id() -> Option<u8> {
    let mut id = String::new();
    print!("ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).expect("Failed to read");
    id.trim().parse().ok()
}

fn get_valid_id() -> Option<u8> {
    let id = read_id();
    if id.is_none() {
        println!("{}", "Please enter a valid number.".red());
        println!("\nPress Enter to continue...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
    id
}

fn show_tasks(store: &TaskStore) {
    let tasks = store.get_tasks();
    if tasks.is_empty() {
        println!("{}", "Your task list is empty.".bright_yellow());
    } else {
        println!("{}", "=== My Tasks ===".bright_cyan().bold());
        for element in &tasks {
            let line = format!("[{}] {} - {}", set_icon(element.is_done), element.id, element.description);
            if element.is_done {
                println!("{}", line.green().strikethrough());
            } else {
                println!("{}", line.yellow());
            }
        }
    }
}

fn set_icon(is_done: bool) -> &'static str {
    if is_done { "✔️" } else { "❌" }
}