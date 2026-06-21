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
                let id = read_id();
                let mut desc = String::new();

                println!("Descrição:");
                io::stdin().read_line(&mut desc).expect("Falha ao ler"); 
                print_feedback(
                    store.create_task(id, desc.trim().to_string()),
                    "Tarefa criada com sucesso!",
                    "Já existe uma tarefa com esse ID."
                );
            }

            "2" => {
                let id = read_id();
                print_feedback(
                    store.toggle_status(id),
                    "Status da tarefa alterado!",
                    "Tarefa não encontrada."
                );
            }

            "3" => {
                let id = read_id();
                let mut desc = String::new();

                println!("Nova descrição:");
                io::stdin().read_line(&mut desc).expect("Falha ao ler");

                print_feedback(
                    store.set_description(id, desc.trim().to_string()),
                    "Descrição alterada!",
                    "Tarefa não encontrada."
                );
            }

            "4" => {
                let id = read_id();
                print_feedback(
                    store.remove(id),
                    "Tarefa removida com sucesso!",
                    "Tarefa não encontrada."
                );
            }

            "5" => break,

            _ => println!("Opção inválida"),
        }

        println!("\nPressione Enter para continuar...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}

    fn print_feedback(success: bool, msg_ok: &str, msg_err: &str) {
    if success {
        println!("Sucesso: {}", msg_ok);
    } else {
        println!("ERRO: {}", msg_err);
    }
}

fn read_id() -> u8 {
    let mut id = String::new();
    println!("ID:");
    io::stdin().read_line(&mut id).expect("Falha ao ler");
    id.trim().parse().unwrap_or(0)
}

fn show_tasks(store: &TaskStore) {
    let tasks = store.get_tasks();
    if tasks.is_empty() {
        println!("Sua lista de tarefas está vazia.");
    }
    for element in &tasks {
        println!("[{}] - {} - {}", set_icon(element.is_done), element.id, element.description);
    }
}

fn set_icon(is_done: bool) -> &'static str {
    if is_done { "✔️" } else { "❌" }
}