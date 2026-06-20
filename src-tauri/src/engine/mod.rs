use crate::domain::errors::AppError;
use crate::domain::task::TaskId;
use crate::engine::models::{EngineTaskRef, EngineTaskStats};

pub mod memory_engine;
pub mod models;
pub mod rqbit_adapter;

pub trait EngineAdapter {
    fn add_torrent(
        &mut self,
        task_id: TaskId,
        torrent_path: &str,
        download_dir: &str,
    ) -> Result<EngineTaskRef, AppError>;

    fn start(&mut self, task_id: &TaskId) -> Result<(), AppError>;

    fn pause(&mut self, task_id: &TaskId) -> Result<(), AppError>;

    fn remove(&mut self, task_id: &TaskId, delete_files: bool) -> Result<(), AppError>;

    fn stats(&self, task_id: &TaskId) -> Result<EngineTaskStats, AppError>;
}
