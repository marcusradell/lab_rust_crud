#![cfg(test)]

use uuid::Uuid;

use super::*;

#[tokio::test]
async fn list_scorecards() {
    let kit = ScorecardsKit::new();

    let result = kit.list().await.unwrap();

    assert_eq!(result, vec![]);
}

#[tokio::test]
async fn create_scorecard() {
    let mut kit = ScorecardsKit::new();

    let scorecard = Scorecard {
        id: Uuid::new_v4(),
        full_name: "Marcus Rådell".to_string(),
    };

    kit.create(scorecard.clone()).await.unwrap();

    let result = kit.list().await.unwrap();

    assert_eq!(result, vec![scorecard]);
}
