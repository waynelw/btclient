use crate::domain::task::{TaskId, TaskStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineTaskRef {
    pub task_id: TaskId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineTaskStatus {
    Queued,
    Downloading,
    Paused,
    Completed,
    Error,
}

impl EngineTaskStatus {
    pub fn as_task_status(self) -> TaskStatus {
        match self {
            Self::Queued => TaskStatus::Queued,
            Self::Downloading => TaskStatus::Downloading,
            Self::Paused => TaskStatus::Paused,
            Self::Completed => TaskStatus::Completed,
            Self::Error => TaskStatus::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineTaskStats {
    pub task_id: TaskId,
    pub status: EngineTaskStatus,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub progress: f32,
    pub download_speed_bps: u64,
    pub upload_speed_bps: u64,
    pub peers_connected: u32,
    pub peers_total: u32,
}
