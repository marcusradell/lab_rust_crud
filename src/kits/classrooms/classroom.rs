use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Classroom {
    title: String,
}

impl Classroom {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
