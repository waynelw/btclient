use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use btclient::commands::task_commands::{
    add_torrent_handler, get_app_snapshot_handler, pause_task_handler, remove_task_handler,
    start_task_handler,
};
use btclient::domain::task::{TaskId, TaskStatus};
use btclient::engine::memory_engine::MemoryEngine;
use btclient::persistence::task_repository::TaskRepository;
use btclient::service::task_service::TaskService;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn service(name: &str) -> TaskService<MemoryEngine> {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let root = std::env::temp_dir().join(format!(
        "btclient-command-{name}-{}-{id}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    TaskService::new(
        TaskRepository::new(root.join("tasks.json")),
        MemoryEngine::default(),
    )
    .unwrap()
}

fn sample_paths(name: &str) -> (String, String) {
    let torrent = PathBuf::from(format!("/tmp/{name}.torrent"));
    let download = PathBuf::from(format!("/tmp/{name}-downloads"));
    (
        torrent.to_string_lossy().to_string(),
        download.to_string_lossy().to_string(),
    )
}

#[test]
fn snapshot_handler_returns_current_tasks() {
    let mut service = service("snapshot");
    let (torrent_path, download_dir) = sample_paths("ubuntu");
    add_torrent_handler(
        &mut service,
        "task-1".to_string(),
        "ubuntu".to_string(),
        torrent_path,
        download_dir,
        4096,
    )
    .unwrap();

    let snapshot = get_app_snapshot_handler(&service);

    assert_eq!(snapshot.tasks.len(), 1);
    assert_eq!(snapshot.tasks[0].id, "task-1");
    assert_eq!(snapshot.tasks[0].status, "downloading");
    assert_eq!(snapshot.settings.default_download_dir, "~/Downloads");
}

#[test]
fn snapshot_handler_uses_engine_stats_for_active_tasks() {
    let mut service = service("snapshot-stats");
    let (torrent_path, download_dir) = sample_paths("ubuntu");
    add_torrent_handler(
        &mut service,
        "task-1".to_string(),
        "ubuntu".to_string(),
        torrent_path,
        download_dir,
        8192,
    )
    .unwrap();

    let snapshot = get_app_snapshot_handler(&service);

    assert_eq!(snapshot.tasks[0].total_bytes, 4096);
    assert_eq!(snapshot.tasks[0].downloaded_bytes, 1024);
    assert_eq!(snapshot.tasks[0].progress, 0.25);
}

#[test]
fn add_torrent_handler_adds_and_starts_first_task() {
    let mut service = service("add");
    let (torrent_path, download_dir) = sample_paths("ubuntu");

    let task = add_torrent_handler(
        &mut service,
        "task-1".to_string(),
        "ubuntu".to_string(),
        torrent_path,
        download_dir,
        4096,
    )
    .unwrap();

    assert_eq!(task.id, "task-1");
    assert_eq!(task.status, "downloading");
    assert_eq!(task.created_at, "1970-01-01T00:00:00Z");
    assert_eq!(task.updated_at, "1970-01-01T00:00:00Z");
    assert_eq!(service.tasks()[0].status, TaskStatus::Downloading);
}

#[test]
fn start_handler_resumes_paused_task() {
    let mut service = service("start");
    let (torrent_path, download_dir) = sample_paths("ubuntu");
    add_torrent_handler(
        &mut service,
        "task-1".to_string(),
        "ubuntu".to_string(),
        torrent_path,
        download_dir,
        4096,
    )
    .unwrap();
    pause_task_handler(&mut service, TaskId::new("task-1")).unwrap();

    let task = start_task_handler(&mut service, TaskId::new("task-1")).unwrap();

    assert_eq!(task.status, "downloading");
}

#[test]
fn pause_handler_returns_updated_task() {
    let mut service = service("pause");
    let (torrent_path, download_dir) = sample_paths("ubuntu");
    add_torrent_handler(
        &mut service,
        "task-1".to_string(),
        "ubuntu".to_string(),
        torrent_path,
        download_dir,
        4096,
    )
    .unwrap();

    let task = pause_task_handler(&mut service, TaskId::new("task-1")).unwrap();

    assert_eq!(task.status, "paused");
}

#[test]
fn remove_handler_removes_task() {
    let mut service = service("remove");
    let (torrent_path, download_dir) = sample_paths("ubuntu");
    add_torrent_handler(
        &mut service,
        "task-1".to_string(),
        "ubuntu".to_string(),
        torrent_path,
        download_dir,
        4096,
    )
    .unwrap();

    remove_task_handler(&mut service, TaskId::new("task-1"), false).unwrap();

    assert!(service.tasks().is_empty());
}
