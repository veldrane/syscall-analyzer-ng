use crate::fd_table::Descs;
use std::collections::HashMap;
use std::ops::{ Deref, DerefMut };


pub struct Archive(HashMap<i32, Descs>);

impl Archive {
    pub fn new() -> Self {
        Archive(HashMap::new())
    }

    pub fn add_descs(&mut self, pid: i32, desc: Descs) {
        self.0.insert(pid, desc);
    }

    pub fn get_descs(&self, pid: i32) -> Option<Descs> {
        self.0.get(&pid).cloned()
    }
}

impl Deref for Archive {
    
    type Target = HashMap<i32, Descs>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Archive {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}