use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Scorecard {
    pub title: String,
}

impl Scorecard {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
