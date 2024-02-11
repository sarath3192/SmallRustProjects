#[allow(unused)]
use std::collections::HashMap;

struct passwordManager{
    Master_Password: String,
    NewPassword: HashMap<String, String>,
}

impl passwordManager {
    pub fn new(master_password: String) -> Self{
        passwordManager{
            NewPassword: Default::default(),
            Master_Password: master_password,   
        }
    }
    pub fn unlock(&mut self, password: String){
        todo!()
    }
    pub fn lock(&self){
        todo!()
    }
    pub fn add_password(&mut self, acc: String, pass: String){
        todo!()
    }
    pub fn list_passwords(&mut self)-> &HashMap<String, String>{
        todo!()
    }
    pub fn encryption(&mut self){
        todo!()
    }
    pub fn version(&mut self) -> String {
        todo!()
    }
}

fn main() {
    
}
