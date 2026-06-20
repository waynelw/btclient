use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use btclient::domain::task::{TaskId, TaskStatus};
use btclient::engine::memory_engine::MemoryEngine;
use btclient::persistence::task_repository::TaskRepository;
use btclient::service::task_service::TaskService;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn repo_path(name: &str) -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let root = std::env::temp_dir().join(format!(
        "btclient-service-{name}-{}-{id}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root.join("tasks.json")
}

fn service(name: &str) -> TaskService<MemoryEngine> {
    TaskService::new(
        TaskRepository::new(repo_path(name)),
        MemoryEngine::default(),
    )
    .unwrap()
}

#[test]
fn adding_first_task_persists_and_starts_it() {
    let mut service = service("first-starts");

    let task = service
        .add_torrent(
            TaskId::new("task-1"),
            "ubuntu",
            "/tmp/ubuntu.torrent",
            "/tmp/downloads",
            4096,
        )
        .unwrap();

    assert_eq!(task.status, TaskStatus::Downloading);
    assert_eq!(service.tasks()[0].status, TaskStatus::Downloading);
    assert_eq!(
        service.reload_from_disk().unwrap()[0].status,
        TaskStatus::Downloading
    );
}

#[test]
fn adding_second_task_keeps_it_queued_while_first_downloads() {
    let mut service = service("second-queued");

    service
        .add_torrent(
            TaskId::new("task-1"),
            "ubuntu",
            "/tmp/ubuntu.torrent",
            "/tmp/downloads",
            4096,
        )
        .unwrap();
    let second = service
        .add_torrent(
            TaskId::new("task-2"),
            "debian",
            "/tmp/debian.torrent",
            "/tmp/downloads",
            2048,
        )
        .unwrap();

    assert_eq!(second.status, TaskStatus::Queued);
    assert_eq!(service.tasks()[0].status, TaskStatus::Downloading);
    assert_eq!(service.tasks()[1].status, TaskStatus::Queued);
}

#[test]
fn pausing_active_task_schedules_next_queued_task() {
    let mut service = service("pause-schedules-next");
    service
        .add_torrent(
            TaskId::new("task-1"),
            "ubuntu",
            "/tmp/ubuntu.torrent",
            "/tmp/downloads",
            4096,
        )
        .unwrap();
    service
        .add_torrent(
            TaskId::new("task-2"),
            "debian",
            "/tmp/debian.torrent",
            "/tmp/downloads",
            2048,
        )
        .unwrap();

    service.pause_task(&TaskId::new("task-1")).unwrap();

    assert_eq!(service.tasks()[0].status, TaskStatus::Paused);
    assert_eq!(service.tasks()[1].status, TaskStatus::Downloading);
}

#[test]
fn retrying_error_task_schedules_when_idle() {
    let mut service = service("retry-error");
    service
        .add_torrent(
            TaskId::new("task-1"),
            "ubuntu",
            "/tmp/ubuntu.torrent",
            "/tmp/downloads",
            4096,
        )
        .unwrap();
    service
        .fail_task(&TaskId::new("task-1"), "network unavailable")
        .unwrap();

    service.retry_task(&TaskId::new("task-1")).unwrap();

    assert_eq!(service.tasks()[0].status, TaskStatus::Downloading);
}

#[test]
fn removing_task_persists_removal() {
    let mut service = service("remove");
    service
        .add_torrent(
            TaskId::new("task-1"),
            "ubuntu",
            "/tmp/ubuntu.torrent",
            "/tmp/downloads",
            4096,
        )
        .unwrap();

    service.remove_task(&TaskId::new("task-1"), false).unwrap();

    assert!(service.tasks().is_empty());
    assert!(service.reload_from_disk().unwrap().is_empty());
}
