mod task;
mod task_store;

use std::io::{self};
use crate::{task_store::TaskStore};

fn main() {
    let mut store = TaskStore::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        show_tasks(&store);
        println!("\n1. Adicionar");
        println!("2. Alternar status");
        println!("3. Alterar descrição");
        println!("4. Remover");
        println!("5. Sair");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Falha ao ler");

        match choice.trim() {
            "1" => {
                let mut id = String::new();

                println!("Id:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");
                let id = match id.trim().parse(){
                    Ok(value) => value,
                    Err(_) => {println!("ERRO: Digite um número..."); continue},
                };
                let mut desc = String::new();

                println!("Descrição:");
                io::stdin().read_line(&mut desc).expect("Falha ao ler");
                store.create_task(id,desc.trim().to_string());
            }

            "2" => {
                let mut id = String::new();

                println!("ID:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");

                let id: u8 = id.trim().parse().unwrap_or(0);
                store.toggle_status(id);
            }

            "3" => {
                let mut id = String::new();
                let mut desc = String::new();

                println!("ID:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");

                println!("Nova descrição:");
                io::stdin().read_line(&mut desc).expect("Falha ao ler");

                let id: u8 = id.trim().parse().unwrap_or(0);
                store.set_description(id, desc.trim().to_string());
            }

            "4" => {
                let mut id = String::new();

                println!("ID:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");

                let id: u8 = id.trim().parse().unwrap_or(0);
                store.remove(id);
            }

            "5" => break,

            _ => println!("Opção inválida"),
        }
    }

}

fn show_tasks(store: &TaskStore){
    let tasks = store.get_tasks();
    if tasks.is_empty(){
        println!("Sua lista de tarefas está vazia.");
    }
    for element in &tasks{
        println!("[{}] - {} - {}", set_icon(element.is_done), element.id, element.description);
    }
}
fn set_icon(is_done: bool) -> &'static str {
    if is_done { "✔️" } else { "❌" }
}