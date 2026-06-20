use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use btclient::domain::errors::{AppError, AppErrorCode};
use btclient::domain::task::{TaskId, TaskRecord, TaskStatus};
use btclient::persistence::paths::validate_torrent_relative_path;
use btclient::persistence::task_repository::TaskRepository;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_dir(name: &str) -> PathBuf {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = std::env::temp_dir().join(format!("btclient-{name}-{}-{id}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn sample_task(id: &str, name: &str) -> TaskRecord {
    let mut task = TaskRecord::new(
        TaskId::new(id),
        name,
        format!("/tmp/{name}.torrent"),
        "/tmp/downloads",
        4096,
    );
    task.transition_to(TaskStatus::Checking).unwrap();
    task
}

#[test]
fn repository_creates_parent_directories() {
    let root = temp_dir("creates-parent");
    let repo_path = root.join("nested/state/tasks.json");
    let repo = TaskRepository::new(repo_path.clone());

    repo.save(&[sample_task("task-1", "ubuntu")]).unwrap();

    assert!(repo_path.exists());
}

#[test]
fn save_then_load_preserves_order_and_fields() {
    let root = temp_dir("roundtrip");
    let repo = TaskRepository::new(root.join("tasks.json"));
    let first = sample_task("task-1", "ubuntu");
    let second = sample_task("task-2", "debian");

    repo.save(&[first.clone(), second.clone()]).unwrap();
    let loaded = repo.load().unwrap();

    assert_eq!(loaded, vec![first, second]);
}

#[test]
fn corrupt_json_returns_persistence_error() {
    let root = temp_dir("corrupt");
    let repo_path = root.join("tasks.json");
    fs::write(&repo_path, "{not json").unwrap();
    let repo = TaskRepository::new(repo_path);

    let err = repo.load().unwrap_err();

    assert!(matches!(err, AppError::Persistence { .. }));
    assert_eq!(err.code(), AppErrorCode::Persistence);
}

#[test]
fn torrent_relative_path_policy_rejects_unsafe_paths() {
    assert!(validate_torrent_relative_path("folder/file.txt").is_ok());

    for path in ["", "/absolute", "../escape", "safe/../escape", "safe//file"] {
        let err = validate_torrent_relative_path(path).unwrap_err();
        assert_eq!(err.code(), AppErrorCode::InvalidPath, "{path}");
    }
}

#[test]
fn overwrite_keeps_valid_json() {
    let root = temp_dir("overwrite");
    let repo = TaskRepository::new(root.join("tasks.json"));
    repo.save(&[sample_task("task-1", "ubuntu")]).unwrap();

    let replacement = sample_task("task-2", "fedora");
    repo.save(&[replacement.clone()]).unwrap();
    let loaded = repo.load().unwrap();

    assert_eq!(loaded, vec![replacement]);
}
