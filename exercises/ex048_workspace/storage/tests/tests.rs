use ex048_core::{count_pending, Task};
use ex048_storage::MemoryStore;

#[test]
fn task_can_be_completed() {
    let mut t = Task::new(1, "write tests");
    assert!(!t.done);
    t.complete();
    assert!(t.done);
}

#[test]
fn count_pending_counts_unfinished_tasks() {
    let mut a = Task::new(1, "a");
    let b = Task::new(2, "b");
    a.complete();
    assert_eq!(count_pending(&[a, b]), 1);
}

#[test]
fn store_adds_and_finds_tasks() {
    let mut store = MemoryStore::new();
    store.add(Task::new(1, "first"));
    store.add(Task::new(2, "second"));

    assert_eq!(store.get(2).map(|t| t.title.as_str()), Some("second"));
    assert!(store.get(99).is_none());
}

#[test]
fn store_reports_pending_count_via_core() {
    let mut store = MemoryStore::new();
    let mut done = Task::new(1, "done");
    done.complete();
    store.add(done);
    store.add(Task::new(2, "pending"));

    assert_eq!(store.pending_count(), 1);
}
