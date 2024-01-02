#![cfg(test)]

use uuid::Uuid;

use super::*;

#[tokio::test]
async fn list_scorecards() {
    let kit = ScorecardsKit::new();

    let result = kit.list();

    assert_eq!(result, vec![]);
}

#[tokio::test]
async fn create_scorecard() {
    let kit = ScorecardsKit::new();

    let scorecard = Scorecard {
        id: Uuid::new_v4(),
        full_name: "Marcus Rådell".to_string(),
    };

    kit.create(scorecard.clone());

    let result = kit.list();

    assert_eq!(result, vec![scorecard]);
}
