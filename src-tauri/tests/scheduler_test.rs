use btclient::domain::task::{TaskId, TaskRecord, TaskStatus};
use btclient::scheduler::single_active_scheduler::select_next_active_task;

fn task(id: &str, status: TaskStatus) -> TaskRecord {
    let mut task = TaskRecord::new(
        TaskId::new(id),
        id,
        format!("/tmp/{id}.torrent"),
        "/tmp/downloads",
        1024,
    );
    task.status = status;
    task
}

#[test]
fn no_queued_task_returns_none() {
    let tasks = vec![
        task("paused", TaskStatus::Paused),
        task("completed", TaskStatus::Completed),
    ];

    assert_eq!(select_next_active_task(&tasks), None);
}

#[test]
fn existing_checking_task_prevents_new_activation() {
    let tasks = vec![
        task("active", TaskStatus::Checking),
        task("queued", TaskStatus::Queued),
    ];

    assert_eq!(select_next_active_task(&tasks), None);
}

#[test]
fn existing_downloading_task_prevents_new_activation() {
    let tasks = vec![
        task("active", TaskStatus::Downloading),
        task("queued", TaskStatus::Queued),
    ];

    assert_eq!(select_next_active_task(&tasks), None);
}

#[test]
fn oldest_queued_task_is_selected() {
    let tasks = vec![
        task("paused", TaskStatus::Paused),
        task("first", TaskStatus::Queued),
        task("second", TaskStatus::Queued),
    ];

    assert_eq!(select_next_active_task(&tasks), Some(TaskId::new("first")));
}

#[test]
fn paused_error_and_completed_tasks_are_skipped() {
    let tasks = vec![
        task("paused", TaskStatus::Paused),
        task("error", TaskStatus::Error),
        task("completed", TaskStatus::Completed),
        task("queued", TaskStatus::Queued),
    ];

    assert_eq!(select_next_active_task(&tasks), Some(TaskId::new("queued")));
}
