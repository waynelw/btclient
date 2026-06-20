use btclient::domain::errors::{AppError, AppErrorCode};
use btclient::domain::task::{TaskId, TaskStatus};
use btclient::engine::memory_engine::MemoryEngine;
use btclient::engine::models::EngineTaskStatus;
use btclient::engine::EngineAdapter;

#[test]
fn adding_task_returns_engine_reference() {
    let mut engine = MemoryEngine::default();

    let reference = engine
        .add_torrent(TaskId::new("task-1"), "/tmp/a.torrent", "/tmp/downloads")
        .unwrap();

    assert_eq!(reference.task_id, TaskId::new("task-1"));
}

#[test]
fn starting_task_changes_stats_to_downloading() {
    let mut engine = MemoryEngine::default();
    engine
        .add_torrent(TaskId::new("task-1"), "/tmp/a.torrent", "/tmp/downloads")
        .unwrap();

    engine.start(&TaskId::new("task-1")).unwrap();
    let stats = engine.stats(&TaskId::new("task-1")).unwrap();

    assert_eq!(stats.status, EngineTaskStatus::Downloading);
    assert_eq!(stats.download_speed_bps, 1024);
}

#[test]
fn pausing_task_changes_stats_to_paused() {
    let mut engine = MemoryEngine::default();
    engine
        .add_torrent(TaskId::new("task-1"), "/tmp/a.torrent", "/tmp/downloads")
        .unwrap();
    engine.start(&TaskId::new("task-1")).unwrap();

    engine.pause(&TaskId::new("task-1")).unwrap();
    let stats = engine.stats(&TaskId::new("task-1")).unwrap();

    assert_eq!(stats.status, EngineTaskStatus::Paused);
    assert_eq!(stats.download_speed_bps, 0);
}

#[test]
fn completing_task_changes_stats_to_completed() {
    let mut engine = MemoryEngine::default();
    engine
        .add_torrent(TaskId::new("task-1"), "/tmp/a.torrent", "/tmp/downloads")
        .unwrap();

    engine.complete_for_test(&TaskId::new("task-1")).unwrap();
    let stats = engine.stats(&TaskId::new("task-1")).unwrap();

    assert_eq!(stats.status, EngineTaskStatus::Completed);
    assert_eq!(stats.progress, 1.0);
}

#[test]
fn missing_task_returns_typed_error() {
    let engine = MemoryEngine::default();

    let err = engine.stats(&TaskId::new("missing")).unwrap_err();

    assert!(matches!(err, AppError::TaskNotFound { .. }));
    assert_eq!(err.code(), AppErrorCode::TaskNotFound);
}

#[test]
fn engine_status_maps_to_task_status() {
    assert_eq!(
        EngineTaskStatus::Queued.as_task_status(),
        TaskStatus::Queued
    );
    assert_eq!(
        EngineTaskStatus::Downloading.as_task_status(),
        TaskStatus::Downloading
    );
    assert_eq!(
        EngineTaskStatus::Completed.as_task_status(),
        TaskStatus::Completed
    );
}
