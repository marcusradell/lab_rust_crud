#![cfg(test)]

use super::dto::Scorecard;
use super::*;
use create::Create as _;
use list::List as _;
use uuid::Uuid;

#[tokio::test]
async fn list_scorecards() {
    let db = Db::new().await;
    let kit = Kit::new(db);

    let result = kit.list().await.unwrap();

    assert_eq!(result, vec![]);
}

#[tokio::test]
async fn create_scorecard() {
    let db = Db::new().await;
    let kit = Kit::new(db);

    let scorecard = Scorecard {
        id: Uuid::new_v4(),
        full_name: "Marcus Rådell".to_string(),
    };

    kit.create(scorecard.clone()).await.unwrap();

    let result = kit.list().await.unwrap();

    assert_eq!(result, vec![scorecard]);
}
