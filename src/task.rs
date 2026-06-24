#[derive(serde::Serialize, serde::Deserialize)]
pub struct Task{
    pub id: usize,
    pub description:String,
    pub is_done:bool
}
impl Task {
    pub fn new(id:usize,description:String) -> Task{
        Self{
            id:id,
            description:description,
            is_done:false
        }
    }
    pub fn toggle(&mut self){
        self.is_done = !self.is_done;
    }
    pub fn set_description(&mut self, desc:String){
        self.description = desc;
    }
}
