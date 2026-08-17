use rust_tdd_learning::{Score, TodoList};

#[test]
fn learner_can_build_a_todo_workflow_from_public_api() {
    let mut list = TodoList::default();
    list.add("learn ownership").unwrap();
    list.add("practice borrowing").unwrap();
    list.complete(0).unwrap();

    assert_eq!(list.pending_titles(), vec!["practice borrowing"]);
    assert!(Score::new(60).unwrap().passed());
}
