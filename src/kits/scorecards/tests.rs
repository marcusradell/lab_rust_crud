#![cfg(test)]

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

    kit.create(Scorecard::new("Marcus Rådell"));

    let result = kit.list();

    assert_eq!(result, vec![Scorecard::new("Marcus Rådell")]);
}
