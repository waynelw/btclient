#[cfg(feature = "bt")]
mod real {
    use std::collections::HashMap;
    use std::fmt;
    use std::path::PathBuf;
    use std::sync::Arc;

    use librqbit::{
        AddTorrent, AddTorrentOptions, AddTorrentResponse, ManagedTorrent, Session, SessionOptions,
        TorrentStats, TorrentStatsState,
    };
    use tokio::runtime::Runtime;

    use crate::domain::errors::AppError;
    use crate::domain::task::TaskId;
    use crate::engine::models::{EngineTaskRef, EngineTaskStats, EngineTaskStatus};
    use crate::engine::EngineAdapter;

    pub struct RqbitEngine {
        runtime: Runtime,
        session: Arc<Session>,
        handles: HashMap<TaskId, Arc<ManagedTorrent>>,
    }

    impl RqbitEngine {
        pub fn adapter_name() -> &'static str {
            "rqbit"
        }

        pub fn new(default_download_dir: impl Into<PathBuf>) -> Result<Self, AppError> {
            Self::new_with_session_options(
                default_download_dir,
                SessionOptions {
                    disable_dht_persistence: true,
                    ..Default::default()
                },
            )
        }

        pub fn new_with_session_options(
            default_download_dir: impl Into<PathBuf>,
            session_options: SessionOptions,
        ) -> Result<Self, AppError> {
            let runtime = Runtime::new().map_err(engine_error)?;
            let session = runtime
                .block_on(Session::new_with_opts(
                    default_download_dir.into(),
                    session_options,
                ))
                .map_err(engine_error)?;

            Ok(Self {
                runtime,
                session,
                handles: HashMap::new(),
            })
        }

        fn handle(&self, task_id: &TaskId) -> Result<&Arc<ManagedTorrent>, AppError> {
            self.handles
                .get(task_id)
                .ok_or_else(|| AppError::TaskNotFound {
                    task_id: task_id.as_str().to_string(),
                })
        }
    }

    impl fmt::Debug for RqbitEngine {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RqbitEngine")
                .field("adapter", &Self::adapter_name())
                .field("managed_tasks", &self.handles.len())
                .finish_non_exhaustive()
        }
    }

    impl Drop for RqbitEngine {
        fn drop(&mut self) {
            self.runtime.block_on(self.session.stop());
        }
    }

    impl EngineAdapter for RqbitEngine {
        fn add_torrent(
            &mut self,
            task_id: TaskId,
            torrent_path: &str,
            download_dir: &str,
        ) -> Result<EngineTaskRef, AppError> {
            if self.handles.contains_key(&task_id) {
                return Ok(EngineTaskRef { task_id });
            }

            let add = add_torrent_input(torrent_path)?;
            let options = AddTorrentOptions {
                paused: true,
                overwrite: true,
                output_folder: Some(download_dir.to_string()),
                ..Default::default()
            };
            let response = self
                .runtime
                .block_on(self.session.add_torrent(add, Some(options)))
                .map_err(engine_error)?;
            let handle = handle_from_response(response)?;
            self.handles.insert(task_id.clone(), handle);

            Ok(EngineTaskRef { task_id })
        }

        fn start(&mut self, task_id: &TaskId) -> Result<(), AppError> {
            let handle = self.handle(task_id)?.clone();
            self.runtime
                .block_on(self.session.unpause(&handle))
                .map_err(engine_error)
        }

        fn pause(&mut self, task_id: &TaskId) -> Result<(), AppError> {
            let handle = self.handle(task_id)?.clone();
            self.runtime
                .block_on(self.session.pause(&handle))
                .map_err(engine_error)
        }

        fn remove(&mut self, task_id: &TaskId, delete_files: bool) -> Result<(), AppError> {
            let handle = self.handle(task_id)?.clone();
            self.runtime
                .block_on(self.session.delete(
                    librqbit::api::TorrentIdOrHash::from(handle.id()),
                    delete_files,
                ))
                .map_err(engine_error)?;
            let removed_id = handle.id();
            self.handles
                .retain(|_, existing| existing.id() != removed_id);
            Ok(())
        }

        fn stats(&self, task_id: &TaskId) -> Result<EngineTaskStats, AppError> {
            let stats = self.handle(task_id)?.stats();
            Ok(rqbit_stats_to_engine_stats(task_id.clone(), stats))
        }
    }

    fn add_torrent_input(torrent_path: &str) -> Result<AddTorrent<'_>, AppError> {
        if torrent_path.starts_with("magnet:")
            || torrent_path.starts_with("http://")
            || torrent_path.starts_with("https://")
        {
            Ok(AddTorrent::from_url(torrent_path))
        } else {
            AddTorrent::from_local_filename(torrent_path).map_err(engine_error)
        }
    }

    fn handle_from_response(response: AddTorrentResponse) -> Result<Arc<ManagedTorrent>, AppError> {
        response
            .into_handle()
            .ok_or_else(|| AppError::EngineUnavailable {
                message: "rqbit returned list-only response while adding torrent".to_string(),
            })
    }

    fn rqbit_stats_to_engine_stats(task_id: TaskId, stats: TorrentStats) -> EngineTaskStats {
        let progress = if stats.total_bytes == 0 {
            0.0
        } else {
            stats.progress_bytes as f32 / stats.total_bytes as f32
        };
        let (download_speed_bps, upload_speed_bps) = stats.live.as_ref().map_or((0, 0), |live| {
            (
                mib_per_second_to_bytes_per_second(live.download_speed.mbps),
                mib_per_second_to_bytes_per_second(live.upload_speed.mbps),
            )
        });

        EngineTaskStats {
            task_id,
            status: rqbit_state_to_engine_status(stats.state, stats.finished),
            total_bytes: stats.total_bytes,
            downloaded_bytes: stats.progress_bytes,
            progress,
            download_speed_bps,
            upload_speed_bps,
            peers_connected: 0,
            peers_total: 0,
        }
    }

    pub fn rqbit_state_to_engine_status(
        state: TorrentStatsState,
        finished: bool,
    ) -> EngineTaskStatus {
        if finished {
            return EngineTaskStatus::Completed;
        }

        match state {
            TorrentStatsState::Initializing => EngineTaskStatus::Queued,
            TorrentStatsState::Live => EngineTaskStatus::Downloading,
            TorrentStatsState::Paused => EngineTaskStatus::Paused,
            TorrentStatsState::Error => EngineTaskStatus::Error,
        }
    }

    pub fn mib_per_second_to_bytes_per_second(mib_per_second: f64) -> u64 {
        (mib_per_second.max(0.0) * 1024.0 * 1024.0).round() as u64
    }

    fn engine_error(error: impl fmt::Display) -> AppError {
        AppError::EngineUnavailable {
            message: error.to_string(),
        }
    }
}

#[cfg(not(feature = "bt"))]
mod fallback {
    use crate::domain::errors::AppError;
    use crate::domain::task::TaskId;
    use crate::engine::models::{EngineTaskRef, EngineTaskStats};
    use crate::engine::EngineAdapter;

    #[derive(Debug, Default)]
    pub struct RqbitEngine;

    impl RqbitEngine {
        pub fn adapter_name() -> &'static str {
            "rqbit-unavailable"
        }
    }

    impl EngineAdapter for RqbitEngine {
        fn add_torrent(
            &mut self,
            _task_id: TaskId,
            _torrent_path: &str,
            _download_dir: &str,
        ) -> Result<EngineTaskRef, AppError> {
            Err(unavailable())
        }

        fn start(&mut self, _task_id: &TaskId) -> Result<(), AppError> {
            Err(unavailable())
        }

        fn pause(&mut self, _task_id: &TaskId) -> Result<(), AppError> {
            Err(unavailable())
        }

        fn remove(&mut self, _task_id: &TaskId, _delete_files: bool) -> Result<(), AppError> {
            Err(unavailable())
        }

        fn stats(&self, _task_id: &TaskId) -> Result<EngineTaskStats, AppError> {
            Err(unavailable())
        }
    }

    fn unavailable() -> AppError {
        AppError::EngineUnavailable {
            message: "librqbit integration requires the bt feature".to_string(),
        }
    }
}

#[cfg(not(feature = "bt"))]
pub use fallback::RqbitEngine;

#[cfg(feature = "bt")]
pub use real::{mib_per_second_to_bytes_per_second, rqbit_state_to_engine_status, RqbitEngine};
