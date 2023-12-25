#![cfg(test)]

use super::*;

#[tokio::test]
async fn list_classrooms() {
    let kit = ClassroomsKit::new();

    let result = kit.list();

    assert_eq!(result, vec![]);
}

#[tokio::test]
async fn create_classroom() {
    let kit = ClassroomsKit::new();

    kit.create(Classroom::new("Fullstack Rust"));

    let result = kit.list();

    assert_eq!(result, vec![Classroom::new("Fullstack Rust")]);
}
