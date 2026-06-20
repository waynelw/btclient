use crate::domain::task::{TaskId, TaskRecord, TaskStatus};

pub fn select_next_active_task(tasks: &[TaskRecord]) -> Option<TaskId> {
    let has_active = tasks
        .iter()
        .any(|task| matches!(task.status, TaskStatus::Checking | TaskStatus::Downloading));

    if has_active {
        return None;
    }

    tasks
        .iter()
        .find(|task| task.status == TaskStatus::Queued)
        .map(|task| task.id.clone())
}
