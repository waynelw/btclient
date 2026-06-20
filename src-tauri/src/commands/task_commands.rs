use crate::domain::errors::AppError;
use crate::domain::task::{TaskId, TaskRecord, TaskStatus};
use crate::engine::models::EngineTaskStats;
use crate::engine::EngineAdapter;
use crate::service::task_service::TaskService;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct AppSnapshotDto {
    pub tasks: Vec<TaskDto>,
    pub settings: SettingsDto,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SettingsDto {
    pub default_download_dir: String,
    pub status_refresh_ms: u64,
    pub log_level: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct TaskDto {
    pub id: String,
    pub name: String,
    pub status: String,
    pub torrent_path: String,
    pub download_dir: String,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub progress: f32,
    pub download_speed_bps: u64,
    pub upload_speed_bps: u64,
    pub peers_connected: u32,
    pub peers_total: u32,
    pub eta_seconds: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
    pub error: Option<TaskErrorDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TaskErrorDto {
    pub code: String,
    pub message: String,
    pub occurred_at: String,
}

pub fn get_app_snapshot_handler<E: EngineAdapter>(service: &TaskService<E>) -> AppSnapshotDto {
    AppSnapshotDto {
        tasks: service
            .tasks()
            .iter()
            .map(|task| task_to_dto(task, service.task_stats(&task.id).ok()))
            .collect(),
        settings: SettingsDto {
            default_download_dir: "~/Downloads".to_string(),
            status_refresh_ms: 1000,
            log_level: "info".to_string(),
        },
    }
}

pub fn add_torrent_handler<E: EngineAdapter>(
    service: &mut TaskService<E>,
    task_id: String,
    name: String,
    torrent_path: String,
    download_dir: String,
    total_bytes: u64,
) -> Result<TaskDto, AppError> {
    let task = service.add_torrent(
        TaskId::new(task_id),
        name,
        torrent_path,
        download_dir,
        total_bytes,
    )?;
    let stats = service.task_stats(&task.id).ok();
    Ok(task_to_dto(&task, stats))
}

pub fn pause_task_handler<E: EngineAdapter>(
    service: &mut TaskService<E>,
    task_id: TaskId,
) -> Result<TaskDto, AppError> {
    let task = service.pause_task(&task_id)?;
    let stats = service.task_stats(&task.id).ok();
    Ok(task_to_dto(&task, stats))
}

pub fn start_task_handler<E: EngineAdapter>(
    service: &mut TaskService<E>,
    task_id: TaskId,
) -> Result<TaskDto, AppError> {
    let task = service.start_task(&task_id)?;
    let stats = service.task_stats(&task.id).ok();
    Ok(task_to_dto(&task, stats))
}

pub fn remove_task_handler<E: EngineAdapter>(
    service: &mut TaskService<E>,
    task_id: TaskId,
    delete_files: bool,
) -> Result<(), AppError> {
    service.remove_task(&task_id, delete_files)
}

fn task_to_dto(task: &TaskRecord, stats: Option<EngineTaskStats>) -> TaskDto {
    let status = stats
        .as_ref()
        .map(|stats| stats.status.as_task_status())
        .unwrap_or(task.status);
    let total_bytes = stats
        .as_ref()
        .map(|stats| stats.total_bytes)
        .unwrap_or(task.total_bytes);
    let downloaded_bytes = stats
        .as_ref()
        .map(|stats| stats.downloaded_bytes)
        .unwrap_or(task.downloaded_bytes);
    let progress = stats
        .as_ref()
        .map(|stats| stats.progress)
        .unwrap_or_else(|| {
            if total_bytes == 0 {
                0.0
            } else {
                downloaded_bytes as f32 / total_bytes as f32
            }
        });
    let download_speed_bps = stats.as_ref().map_or_else(
        || {
            if status == TaskStatus::Downloading {
                1024
            } else {
                0
            }
        },
        |stats| stats.download_speed_bps,
    );
    let upload_speed_bps = stats
        .as_ref()
        .map(|stats| stats.upload_speed_bps)
        .unwrap_or(0);
    let peers_connected = stats.as_ref().map_or_else(
        || {
            if status == TaskStatus::Downloading {
                3
            } else {
                0
            }
        },
        |stats| stats.peers_connected,
    );
    let peers_total = stats.as_ref().map_or_else(
        || {
            if status == TaskStatus::Downloading {
                8
            } else {
                0
            }
        },
        |stats| stats.peers_total,
    );

    TaskDto {
        id: task.id.as_str().to_string(),
        name: task.name.clone(),
        status: status_to_string(status),
        torrent_path: task.torrent_path.clone(),
        download_dir: task.download_dir.clone(),
        total_bytes,
        downloaded_bytes,
        progress,
        download_speed_bps,
        upload_speed_bps,
        peers_connected,
        peers_total,
        eta_seconds: None,
        created_at: task.created_at.clone(),
        updated_at: task.updated_at.clone(),
        error: task.error.as_ref().map(|error| TaskErrorDto {
            code: error.code.clone(),
            message: error.message.clone(),
            occurred_at: error.occurred_at.clone(),
        }),
    }
}

fn status_to_string(status: TaskStatus) -> String {
    status.as_str().to_string()
}
