use crate::domain::errors::AppError;
use crate::domain::task::{TaskErrorRecord, TaskId, TaskRecord, TaskStatus};
use crate::engine::models::EngineTaskStats;
use crate::engine::EngineAdapter;
use crate::persistence::task_repository::TaskRepository;
use crate::scheduler::single_active_scheduler::select_next_active_task;

#[derive(Debug)]
pub struct TaskService<E: EngineAdapter> {
    repo: TaskRepository,
    engine: E,
    tasks: Vec<TaskRecord>,
}

impl<E: EngineAdapter> TaskService<E> {
    pub fn new(repo: TaskRepository, engine: E) -> Result<Self, AppError> {
        let tasks = repo.load()?;
        Ok(Self {
            repo,
            engine,
            tasks,
        })
    }

    pub fn tasks(&self) -> &[TaskRecord] {
        &self.tasks
    }

    pub fn reload_from_disk(&self) -> Result<Vec<TaskRecord>, AppError> {
        self.repo.load()
    }

    pub fn task_stats(&self, task_id: &TaskId) -> Result<EngineTaskStats, AppError> {
        self.engine.stats(task_id)
    }

    pub fn add_torrent(
        &mut self,
        task_id: TaskId,
        name: impl Into<String>,
        torrent_path: impl Into<String>,
        download_dir: impl Into<String>,
        total_bytes: u64,
    ) -> Result<TaskRecord, AppError> {
        let task = TaskRecord::new(
            task_id.clone(),
            name,
            torrent_path,
            download_dir,
            total_bytes,
        );
        self.tasks.push(task);
        self.schedule_if_idle()?;
        self.persist()?;
        self.task(&task_id).cloned()
    }

    pub fn pause_task(&mut self, task_id: &TaskId) -> Result<TaskRecord, AppError> {
        let index = self.task_index(task_id)?;
        self.engine.pause(task_id)?;
        self.tasks[index].transition_to(TaskStatus::Paused)?;
        self.schedule_if_idle()?;
        self.persist()?;
        self.task(task_id).cloned()
    }

    pub fn start_task(&mut self, task_id: &TaskId) -> Result<TaskRecord, AppError> {
        let index = self.task_index(task_id)?;
        match self.tasks[index].status {
            TaskStatus::Downloading | TaskStatus::Checking => {
                return self.task(task_id).cloned();
            }
            TaskStatus::Queued => {}
            TaskStatus::Paused | TaskStatus::Error => {
                self.tasks[index].transition_to(TaskStatus::Queued)?;
                self.tasks[index].error = None;
            }
            TaskStatus::Completed => {
                return Err(AppError::InvalidTransition {
                    from: TaskStatus::Completed.as_str(),
                    to: TaskStatus::Queued.as_str(),
                });
            }
        }

        self.schedule_if_idle()?;
        self.persist()?;
        self.task(task_id).cloned()
    }

    pub fn retry_task(&mut self, task_id: &TaskId) -> Result<TaskRecord, AppError> {
        let index = self.task_index(task_id)?;
        self.tasks[index].transition_to(TaskStatus::Queued)?;
        self.tasks[index].error = None;
        self.schedule_if_idle()?;
        self.persist()?;
        self.task(task_id).cloned()
    }

    pub fn fail_task(
        &mut self,
        task_id: &TaskId,
        message: impl Into<String>,
    ) -> Result<TaskRecord, AppError> {
        let index = self.task_index(task_id)?;
        self.tasks[index].transition_to(TaskStatus::Error)?;
        self.tasks[index].error = Some(TaskErrorRecord {
            code: "engine_failed".to_string(),
            message: message.into(),
            occurred_at: "1970-01-01T00:00:00Z".to_string(),
        });
        self.persist()?;
        self.task(task_id).cloned()
    }

    pub fn remove_task(&mut self, task_id: &TaskId, delete_files: bool) -> Result<(), AppError> {
        let index = self.task_index(task_id)?;
        match self.engine.remove(task_id, delete_files) {
            Ok(()) | Err(AppError::TaskNotFound { .. }) => {}
            Err(err) => return Err(err),
        }
        self.tasks.remove(index);
        self.schedule_if_idle()?;
        self.persist()
    }

    fn schedule_if_idle(&mut self) -> Result<(), AppError> {
        let Some(task_id) = select_next_active_task(&self.tasks) else {
            return Ok(());
        };
        let index = self.task_index(&task_id)?;
        self.tasks[index].transition_to(TaskStatus::Checking)?;

        let torrent_path = self.tasks[index].torrent_path.clone();
        let download_dir = self.tasks[index].download_dir.clone();
        self.engine
            .add_torrent(task_id.clone(), &torrent_path, &download_dir)?;
        self.engine.start(&task_id)?;

        self.tasks[index].transition_to(TaskStatus::Downloading)?;
        Ok(())
    }

    fn persist(&self) -> Result<(), AppError> {
        self.repo.save(&self.tasks)
    }

    fn task_index(&self, task_id: &TaskId) -> Result<usize, AppError> {
        self.tasks
            .iter()
            .position(|task| &task.id == task_id)
            .ok_or_else(|| AppError::TaskNotFound {
                task_id: task_id.as_str().to_string(),
            })
    }

    fn task(&self, task_id: &TaskId) -> Result<&TaskRecord, AppError> {
        let index = self.task_index(task_id)?;
        Ok(&self.tasks[index])
    }
}
