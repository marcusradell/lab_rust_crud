use super::scorecard::Scorecard;
use std::collections::HashMap;

#[derive(Clone)]
pub struct InMemoryDb {
    pub data: HashMap<String, Scorecard>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
