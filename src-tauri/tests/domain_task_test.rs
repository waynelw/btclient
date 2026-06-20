use btclient::domain::errors::{AppError, AppErrorCode};
use btclient::domain::task::{TaskId, TaskRecord, TaskStatus};

fn sample_task() -> TaskRecord {
    TaskRecord::new(
        TaskId::new("task-1"),
        "ubuntu.iso",
        "/tmp/ubuntu.torrent",
        "/tmp/downloads",
        1024,
    )
}

#[test]
fn new_task_starts_queued() {
    let task = sample_task();

    assert_eq!(task.status, TaskStatus::Queued);
    assert_eq!(task.id.as_str(), "task-1");
    assert_eq!(task.name, "ubuntu.iso");
}

#[test]
fn task_can_move_from_queued_to_completed_path() {
    let mut task = sample_task();

    task.transition_to(TaskStatus::Checking).unwrap();
    task.transition_to(TaskStatus::Downloading).unwrap();
    task.transition_to(TaskStatus::Completed).unwrap();

    assert_eq!(task.status, TaskStatus::Completed);
}

#[test]
fn paused_task_can_resume_to_queue() {
    let mut task = sample_task();

    task.transition_to(TaskStatus::Checking).unwrap();
    task.transition_to(TaskStatus::Downloading).unwrap();
    task.transition_to(TaskStatus::Paused).unwrap();
    task.transition_to(TaskStatus::Queued).unwrap();

    assert_eq!(task.status, TaskStatus::Queued);
}

#[test]
fn errored_task_can_retry_to_queue() {
    let mut task = sample_task();

    task.transition_to(TaskStatus::Checking).unwrap();
    task.transition_to(TaskStatus::Error).unwrap();
    task.transition_to(TaskStatus::Queued).unwrap();

    assert_eq!(task.status, TaskStatus::Queued);
}

#[test]
fn invalid_transition_returns_typed_error() {
    let mut task = sample_task();

    let err = task.transition_to(TaskStatus::Completed).unwrap_err();

    assert!(matches!(err, AppError::InvalidTransition { .. }));
    assert_eq!(err.code(), AppErrorCode::InvalidTransition);
}
