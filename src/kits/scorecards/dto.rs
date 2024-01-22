use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, FromRow, PartialEq, Debug)]
pub struct Scorecard {
    pub id: Uuid,
    pub full_name: String,
}
