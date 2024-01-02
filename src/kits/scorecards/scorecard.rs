use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Scorecard {
    pub id: Uuid,
    pub full_name: String,
}
