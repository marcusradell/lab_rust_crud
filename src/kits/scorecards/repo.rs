use std::collections::HashMap;

use super::scorecard::Scorecard;

#[derive(Clone)]
pub struct Repo {
    data: HashMap<String, Scorecard>,
}

impl Repo {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn list(&self) -> Vec<Scorecard> {
        self.data.values().cloned().collect()
    }

    pub fn create(&mut self, scorecard: Scorecard) {
        self.data.insert(scorecard.title.clone(), scorecard);
    }
}
