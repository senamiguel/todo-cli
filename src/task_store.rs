use std::fs::File;
use std::collections::{BTreeMap};
use crate::task::{Task};
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskStore{
    tasks: BTreeMap<usize,Task>,
    file_path:String,
    next_id: usize
}
impl TaskStore {
    pub fn new() -> Self{
        let file_path = "data.json".to_owned();
        let tasks:BTreeMap<usize,Task> = if let Ok(file) = File::open(&file_path){
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_default()
        }else{
            BTreeMap::new()
        };
        let next_id = tasks.keys().max().map(|k| k + 1).unwrap_or(1);
        Self { 
            tasks,
            file_path,
            next_id
        }
    }
    pub fn remove(&mut self, id:usize)-> bool{
        if !self.check_task_exists(id) {
            return false;
        }
        self.tasks.remove(&id);
        self.save_to_file();
        true
    }
    pub fn toggle_status(&mut self, id: usize) -> bool{
        if let Some(task) = self.tasks.get_mut(&id) {
            task.toggle();
            self.save_to_file();
            return true;
        }false
    }
    pub fn get_tasks(&self)-> Vec<&Task>{
        self.tasks.values().collect()
    }
    pub fn create_task(&mut self,description: String)-> bool{
        let task = Task::new(self.next_id, description);
        self.next_id += 1;
        self.tasks.entry(task.id).or_insert(task);
        let file = File::create(&self.file_path).expect("Failed to open/reset file");
        serde_json::to_writer_pretty(file, &self.tasks).unwrap();
        true
    }
    pub fn set_description(&mut self, id: usize, desc: String)-> bool {
        if let Some(task) = self.tasks.get_mut(&id){
            task.set_description(desc);
            self.save_to_file();
            return true;
        }false
    }
    pub fn check_task_exists(&self, id: usize) -> bool {
        self.tasks.contains_key(&id)
    }
    fn save_to_file(&self) {
        let file = File::create(&self.file_path).expect("Falha ao abrir/limpar arquivo");
        serde_json::to_writer_pretty(file, &self.tasks).expect("Falha ao escrever JSON");
    }
}