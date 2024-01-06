use super::scorecard::Scorecard;
use std::collections::HashMap;

pub trait Repo {
    fn list(&self) -> Vec<Scorecard>;
    fn create(&mut self, scorecard: Scorecard);
}

#[derive(Clone)]
pub struct InMemoryDb {
    data: HashMap<String, Scorecard>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Repo for InMemoryDb {
    fn list(&self) -> Vec<Scorecard> {
        self.data.values().cloned().collect()
    }

    fn create(&mut self, scorecard: Scorecard) {
        self.data.insert(scorecard.full_name.clone(), scorecard);
    }
}
