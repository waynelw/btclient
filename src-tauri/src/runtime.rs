const REGISTERED_COMMAND_NAMES: &[&str] = &[
    "get_app_snapshot",
    "add_torrent",
    "start_task",
    "pause_task",
    "remove_task",
];

pub fn registered_command_names() -> &'static [&'static str] {
    REGISTERED_COMMAND_NAMES
}

pub fn default_repository_path() -> std::path::PathBuf {
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join(".local")
        .join("share")
        .join("btclient")
        .join("tasks.json")
}

#[cfg(feature = "desktop")]
mod desktop {
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::{Mutex, MutexGuard};
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::commands::task_commands::{
        add_torrent_handler, get_app_snapshot_handler, pause_task_handler, remove_task_handler,
        start_task_handler, AppSnapshotDto, TaskDto,
    };
    use crate::domain::errors::AppError;
    use crate::domain::task::TaskId;
    use crate::engine::rqbit_adapter::RqbitEngine;
    use crate::persistence::task_repository::TaskRepository;
    use crate::service::task_service::TaskService;

    pub struct DesktopState {
        service: Mutex<TaskService<RqbitEngine>>,
    }

    impl DesktopState {
        fn new() -> Result<Self, AppError> {
            let repo = TaskRepository::new(super::default_repository_path());
            let service = TaskService::new(repo, RqbitEngine::new(default_download_dir())?)?;
            Ok(Self {
                service: Mutex::new(service),
            })
        }
    }

    #[tauri::command]
    fn get_app_snapshot(state: tauri::State<'_, DesktopState>) -> Result<AppSnapshotDto, String> {
        let service = lock_service(&state)?;
        Ok(get_app_snapshot_handler(&service))
    }

    #[tauri::command]
    fn add_torrent(
        state: tauri::State<'_, DesktopState>,
        torrent_path: String,
        download_dir: String,
    ) -> Result<TaskDto, String> {
        let mut service = lock_service(&state)?;
        add_torrent_handler(
            &mut service,
            next_task_id(),
            torrent_name(&torrent_path),
            torrent_path.clone(),
            download_dir,
            torrent_metadata_len(&torrent_path),
        )
        .map_err(error_to_string)
    }

    #[tauri::command]
    fn start_task(
        state: tauri::State<'_, DesktopState>,
        task_id: String,
    ) -> Result<TaskDto, String> {
        let mut service = lock_service(&state)?;
        start_task_handler(&mut service, TaskId::new(task_id)).map_err(error_to_string)
    }

    #[tauri::command]
    fn pause_task(
        state: tauri::State<'_, DesktopState>,
        task_id: String,
    ) -> Result<TaskDto, String> {
        let mut service = lock_service(&state)?;
        pause_task_handler(&mut service, TaskId::new(task_id)).map_err(error_to_string)
    }

    #[tauri::command]
    fn remove_task(
        state: tauri::State<'_, DesktopState>,
        task_id: String,
        delete_files: bool,
    ) -> Result<(), String> {
        let mut service = lock_service(&state)?;
        remove_task_handler(&mut service, TaskId::new(task_id), delete_files)
            .map_err(error_to_string)
    }

    pub fn run() {
        let state = DesktopState::new().expect("failed to initialize btclient desktop state");
        tauri::Builder::default()
            .manage(state)
            .invoke_handler(tauri::generate_handler![
                get_app_snapshot,
                add_torrent,
                start_task,
                pause_task,
                remove_task,
            ])
            .run(tauri::generate_context!())
            .expect("failed to run btclient desktop application");
    }

    pub(super) fn engine_adapter_name() -> &'static str {
        RqbitEngine::adapter_name()
    }

    fn lock_service<'a>(
        state: &'a tauri::State<'_, DesktopState>,
    ) -> Result<MutexGuard<'a, TaskService<RqbitEngine>>, String> {
        state
            .service
            .lock()
            .map_err(|_| "task service lock poisoned".to_string())
    }

    fn next_task_id() -> String {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default();
        format!("task-{millis}")
    }

    fn torrent_name(torrent_path: &str) -> String {
        Path::new(torrent_path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .filter(|stem| !stem.is_empty())
            .unwrap_or("torrent")
            .to_string()
    }

    fn torrent_metadata_len(torrent_path: &str) -> u64 {
        std::fs::metadata(torrent_path)
            .map(|metadata| metadata.len())
            .unwrap_or(0)
    }

    fn default_download_dir() -> PathBuf {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join("Downloads")
    }

    fn error_to_string(error: AppError) -> String {
        error.to_string()
    }
}

#[cfg(feature = "desktop")]
pub use desktop::run;

#[cfg(feature = "desktop")]
pub fn desktop_engine_adapter_name() -> &'static str {
    desktop::engine_adapter_name()
}
