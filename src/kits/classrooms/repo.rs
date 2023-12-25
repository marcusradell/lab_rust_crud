use std::collections::HashMap;

use super::classroom::Classroom;

#[derive(Clone)]
pub struct Repo {
    data: HashMap<String, Classroom>,
}

impl Repo {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn list(&self) -> Vec<Classroom> {
        self.data.values().cloned().collect()
    }

    pub fn create(&mut self, classroom: Classroom) {
        self.data.insert(classroom.title().to_string(), classroom);
    }
}
