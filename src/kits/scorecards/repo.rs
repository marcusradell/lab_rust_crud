use super::{mock_db::InMemoryDb, scorecard::Scorecard};

pub trait Repo {
    fn list(&self) -> Vec<Scorecard>;
    fn create(&mut self, scorecard: Scorecard);
}

impl Repo for InMemoryDb {
    fn list(&self) -> Vec<Scorecard> {
        self.data.values().cloned().collect()
    }

    fn create(&mut self, scorecard: Scorecard) {
        self.data.insert(scorecard.full_name.clone(), scorecard);
    }
}
