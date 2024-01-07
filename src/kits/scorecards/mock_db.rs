use super::scorecard::Scorecard;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct InMemoryDb {
    pub data: Arc<Mutex<HashMap<String, Scorecard>>>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
