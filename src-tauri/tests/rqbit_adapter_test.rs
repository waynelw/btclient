#![cfg(feature = "bt")]

use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use btclient::domain::task::TaskId;
use btclient::engine::models::EngineTaskStatus;
use btclient::engine::rqbit_adapter::{
    mib_per_second_to_bytes_per_second, rqbit_state_to_engine_status, RqbitEngine,
};
use btclient::engine::EngineAdapter;

#[test]
fn rqbit_adapter_exposes_real_adapter_name() {
    assert_eq!(RqbitEngine::adapter_name(), "rqbit");
}

#[test]
fn rqbit_state_maps_to_engine_status() {
    assert_eq!(
        rqbit_state_to_engine_status(librqbit::TorrentStatsState::Initializing, false),
        EngineTaskStatus::Queued
    );
    assert_eq!(
        rqbit_state_to_engine_status(librqbit::TorrentStatsState::Live, false),
        EngineTaskStatus::Downloading
    );
    assert_eq!(
        rqbit_state_to_engine_status(librqbit::TorrentStatsState::Paused, false),
        EngineTaskStatus::Paused
    );
    assert_eq!(
        rqbit_state_to_engine_status(librqbit::TorrentStatsState::Error, false),
        EngineTaskStatus::Error
    );
    assert_eq!(
        rqbit_state_to_engine_status(librqbit::TorrentStatsState::Live, true),
        EngineTaskStatus::Completed
    );
}

#[test]
fn rqbit_speed_in_mib_per_second_maps_to_bytes_per_second() {
    assert_eq!(mib_per_second_to_bytes_per_second(0.0), 0);
    assert_eq!(mib_per_second_to_bytes_per_second(1.0), 1024 * 1024);
    assert_eq!(
        mib_per_second_to_bytes_per_second(1.5),
        1024 * 1024 + 512 * 1024
    );
}

#[test]
fn rqbit_engine_adds_real_torrent_file_and_reports_completed_payload() {
    let root = unique_temp_dir("real-torrent");
    let seed_dir = root.join("seed");
    let download_dir = root.join("download");
    fs::create_dir_all(&seed_dir).unwrap();
    fs::create_dir_all(&download_dir).unwrap();

    let payload_path = seed_dir.join("payload.bin");
    let payload = b"btclient-rqbit-integration-payload";
    fs::write(&payload_path, payload).unwrap();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let torrent = runtime
        .block_on(librqbit::create_torrent(
            &payload_path,
            librqbit::CreateTorrentOptions {
                piece_length: Some(16 * 1024),
                ..Default::default()
            },
        ))
        .unwrap();
    let torrent_path = root.join("payload.torrent");
    fs::write(&torrent_path, torrent.as_bytes().unwrap()).unwrap();

    let mut engine = RqbitEngine::new_with_session_options(
        &download_dir,
        librqbit::SessionOptions {
            disable_dht: true,
            disable_dht_persistence: true,
            listen_port_range: None,
            enable_upnp_port_forwarding: false,
            ..Default::default()
        },
    )
    .unwrap();
    let task_id = TaskId::new("task-rqbit-real");
    engine
        .add_torrent(
            task_id.clone(),
            torrent_path.to_str().unwrap(),
            seed_dir.to_str().unwrap(),
        )
        .unwrap();
    engine.start(&task_id).unwrap();

    let stats = wait_for_completed_stats(&engine, &task_id);

    assert_eq!(stats.status, EngineTaskStatus::Completed);
    assert_eq!(stats.total_bytes, payload.len() as u64);
    assert_eq!(stats.downloaded_bytes, payload.len() as u64);
    assert_eq!(stats.progress, 1.0);

    engine.remove(&task_id, false).unwrap();
    let _ = fs::remove_dir_all(root);
}

fn wait_for_completed_stats(
    engine: &RqbitEngine,
    task_id: &TaskId,
) -> btclient::engine::models::EngineTaskStats {
    for _ in 0..100 {
        let stats = engine.stats(task_id).unwrap();
        if stats.status == EngineTaskStatus::Completed {
            return stats;
        }
        thread::sleep(Duration::from_millis(50));
    }

    panic!("rqbit engine did not report completed payload within timeout");
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("btclient-{name}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    root
}
