use std::collections::HashMap;

use crate::domain::errors::AppError;
use crate::domain::task::TaskId;
use crate::engine::models::{EngineTaskRef, EngineTaskStats, EngineTaskStatus};
use crate::engine::EngineAdapter;

#[derive(Debug, Clone)]
struct MemoryTask {
    task_id: TaskId,
    status: EngineTaskStatus,
    total_bytes: u64,
    downloaded_bytes: u64,
}

#[derive(Debug, Default)]
pub struct MemoryEngine {
    tasks: HashMap<TaskId, MemoryTask>,
}

impl MemoryEngine {
    pub fn complete_for_test(&mut self, task_id: &TaskId) -> Result<(), AppError> {
        let task = self.task_mut(task_id)?;
        task.status = EngineTaskStatus::Completed;
        task.downloaded_bytes = task.total_bytes;
        Ok(())
    }

    fn task_mut(&mut self, task_id: &TaskId) -> Result<&mut MemoryTask, AppError> {
        self.tasks
            .get_mut(task_id)
            .ok_or_else(|| AppError::TaskNotFound {
                task_id: task_id.as_str().to_string(),
            })
    }

    fn task(&self, task_id: &TaskId) -> Result<&MemoryTask, AppError> {
        self.tasks
            .get(task_id)
            .ok_or_else(|| AppError::TaskNotFound {
                task_id: task_id.as_str().to_string(),
            })
    }
}

impl EngineAdapter for MemoryEngine {
    fn add_torrent(
        &mut self,
        task_id: TaskId,
        _torrent_path: &str,
        _download_dir: &str,
    ) -> Result<EngineTaskRef, AppError> {
        let task = MemoryTask {
            task_id: task_id.clone(),
            status: EngineTaskStatus::Queued,
            total_bytes: 4096,
            downloaded_bytes: 0,
        };
        self.tasks.insert(task_id.clone(), task);
        Ok(EngineTaskRef { task_id })
    }

    fn start(&mut self, task_id: &TaskId) -> Result<(), AppError> {
        let task = self.task_mut(task_id)?;
        task.status = EngineTaskStatus::Downloading;
        task.downloaded_bytes = task.downloaded_bytes.max(1024);
        Ok(())
    }

    fn pause(&mut self, task_id: &TaskId) -> Result<(), AppError> {
        self.task_mut(task_id)?.status = EngineTaskStatus::Paused;
        Ok(())
    }

    fn remove(&mut self, task_id: &TaskId, _delete_files: bool) -> Result<(), AppError> {
        self.tasks
            .remove(task_id)
            .map(|_| ())
            .ok_or_else(|| AppError::TaskNotFound {
                task_id: task_id.as_str().to_string(),
            })
    }

    fn stats(&self, task_id: &TaskId) -> Result<EngineTaskStats, AppError> {
        let task = self.task(task_id)?;
        let progress = if task.total_bytes == 0 {
            0.0
        } else {
            task.downloaded_bytes as f32 / task.total_bytes as f32
        };

        Ok(EngineTaskStats {
            task_id: task.task_id.clone(),
            status: task.status,
            total_bytes: task.total_bytes,
            downloaded_bytes: task.downloaded_bytes,
            progress,
            download_speed_bps: if task.status == EngineTaskStatus::Downloading {
                1024
            } else {
                0
            },
            upload_speed_bps: 0,
            peers_connected: if task.status == EngineTaskStatus::Downloading {
                3
            } else {
                0
            },
            peers_total: 8,
        })
    }
}
