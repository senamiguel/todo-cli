mod task;
mod task_store;

use std::io::{self};
use crate::task_store::TaskStore;

fn main() {
    let mut store = TaskStore::new();

    loop {
        println!("\n1. Adicionar");
        println!("2. Listar");
        println!("3. Alternar status");
        println!("4. Alterar descrição");
        println!("5. Remover");
        println!("6. Sair");

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
                store.list_tasks();
                println!("Aperte enter para sair.");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }

            "3" => {
                let mut id = String::new();

                println!("ID:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");

                let id: u8 = id.trim().parse().unwrap_or(0);
                store.toggle_status(id);
            }

            "4" => {
                let mut id = String::new();
                let mut desc = String::new();

                println!("ID:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");

                println!("Nova descrição:");
                io::stdin().read_line(&mut desc).expect("Falha ao ler");

                let id: u8 = id.trim().parse().unwrap_or(0);
                store.set_description(id, desc.trim().to_string());
            }

            "5" => {
                let mut id = String::new();

                println!("ID:");
                io::stdin().read_line(&mut id).expect("Falha ao ler");

                let id: u8 = id.trim().parse().unwrap_or(0);
                store.remove(id);
            }

            "6" => break,

            _ => println!("Opção inválida"),
        }
        print!("\x1B[2J\x1B[1;1H");
    }
}