use std::collections::HashMap;
use crate::task::{Task};

pub struct TaskStore{
    tasks: HashMap<u8,Task>
}
impl TaskStore {
    pub fn new() -> Self{
        Self { tasks: HashMap::new() }
    }
    pub fn remove(&mut self, id:u8){
        self.tasks.remove(&id);
    }
    pub fn toggle_status(&mut self, id: u8) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.toggle();
        }
    }
    pub fn list_tasks(&self){
        for (id, task) in &self.tasks{
            println!("{}-{} {}",id,Self::set_icon(task.is_done), task.description)
        }
    }
    pub fn create_task(&mut self, id: u8, description: String) {
        let task = Task::new(id, description);
        self.tasks.insert(task.id,task);
    }
    pub fn set_description(&mut self, id: u8, desc: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.set_description(desc);
        }
    }
    fn set_icon(is_done: bool) -> &'static str {
        if is_done { "✔️" } else { "❌" }
    }
}