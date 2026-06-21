mod task;
mod task_store;

use std::io::{self};
use crate::{task_store::TaskStore};

fn main() {
    let mut store = TaskStore::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        show_tasks(&store);
        println!("\n1. Add");
        println!("2. Toggle status");
        println!("3. Edit description");
        println!("4. Remove");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");

        match choice.trim() {
            "1" => {
                let id = read_id();
                let mut desc = String::new();

                println!("Description:");
                io::stdin().read_line(&mut desc).expect("Failed to read"); 
                print_feedback(
                    store.create_task(id, desc.trim().to_string()),
                    "Task created successfully!",
                    "A task with this ID already exists."
                );
            }

            "2" => {
                let id = read_id();
                print_feedback(
                    store.toggle_status(id),
                    "Task status changed!",
                    "Task not found."
                );
            }

            "3" => {
                let id = read_id();
                let mut desc = String::new();

                println!("New description:");
                io::stdin().read_line(&mut desc).expect("Failed to read");

                print_feedback(
                    store.set_description(id, desc.trim().to_string()),
                    "Description changed!",
                    "Task not found."
                );
            }

            "4" => {
                let id = read_id();
                print_feedback(
                    store.remove(id),
                    "Task removed successfully!",
                    "Task not found."
                );
            }

            "5" => break,

            _ => println!("Invalid option"),
        }

        println!("\nPress Enter to continue...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}

fn print_feedback(success: bool, msg_ok: &str, msg_err: &str) {
    if success {
        println!("Success: {}", msg_ok);
    } else {
        println!("ERROR: {}", msg_err);
    }
}

fn read_id() -> u8 {
    let mut id = String::new();
    println!("ID:");
    io::stdin().read_line(&mut id).expect("Failed to read");
    id.trim().parse().unwrap_or(0)
}

fn show_tasks(store: &TaskStore) {
    let tasks = store.get_tasks();
    if tasks.is_empty() {
        println!("Your task list is empty.");
    }
    for element in &tasks {
        println!("[{}] - {} - {}", set_icon(element.is_done), element.id, element.description);
    }
}

fn set_icon(is_done: bool) -> &'static str {
    if is_done { "✔️" } else { "❌" }
}