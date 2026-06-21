use std::collections::{HashMap};
use crate::task::{Task};

pub struct TaskStore{
    tasks: HashMap<u8,Task>
}
impl TaskStore {
    pub fn new() -> Self{
        Self { tasks: HashMap::new() }
    }
    pub fn remove(&mut self, id:u8)-> bool{
        if !self.check_task_exists(id) {
            return false;
        }
        self.tasks.remove(&id);
        true
    }
    pub fn toggle_status(&mut self, id: u8) -> bool{
        if let Some(task) = self.tasks.get_mut(&id) {
            task.toggle();
            return true;
        }false
    }
    pub fn get_tasks(&self)-> Vec<&Task>{
        let mut tasks_list = Vec::new();
        for element in self.tasks.iter(){
            tasks_list.push(element.1)
        }
        tasks_list
    }
    pub fn create_task(&mut self, id: u8, description: String)-> bool{
        let task = Task::new(id, description);
        if self.check_task_exists(id) {
            return false;
        }
        self.tasks.entry(task.id).or_insert(task);
        true
    }
    pub fn set_description(&mut self, id: u8, desc: String)-> bool {
        if let Some(task) = self.tasks.get_mut(&id){
            task.set_description(desc);
            return true;
        }false
    }
    pub fn check_task_exists(&self, id: u8) -> bool {
        self.tasks.contains_key(&id)
    }

}