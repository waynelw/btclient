use crate::domain::errors::AppError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskId(String);

impl TaskId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for TaskId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Queued,
    Checking,
    Downloading,
    Paused,
    Completed,
    Error,
}

impl TaskStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Checking => "checking",
            Self::Downloading => "downloading",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Error => "error",
        }
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Queued, Self::Checking)
                | (Self::Queued, Self::Paused)
                | (Self::Queued, Self::Error)
                | (Self::Checking, Self::Downloading)
                | (Self::Checking, Self::Paused)
                | (Self::Checking, Self::Error)
                | (Self::Downloading, Self::Paused)
                | (Self::Downloading, Self::Completed)
                | (Self::Downloading, Self::Error)
                | (Self::Paused, Self::Queued)
                | (Self::Paused, Self::Error)
                | (Self::Error, Self::Queued)
        )
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "queued" => Some(Self::Queued),
            "checking" => Some(Self::Checking),
            "downloading" => Some(Self::Downloading),
            "paused" => Some(Self::Paused),
            "completed" => Some(Self::Completed),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFileRecord {
    pub file_index: usize,
    pub relative_path: String,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskErrorRecord {
    pub code: String,
    pub message: String,
    pub occurred_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRecord {
    pub id: TaskId,
    pub name: String,
    pub torrent_path: String,
    pub download_dir: String,
    pub status: TaskStatus,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub files: Vec<TaskFileRecord>,
    pub error: Option<TaskErrorRecord>,
    pub created_at: String,
    pub updated_at: String,
}

impl TaskRecord {
    pub fn new(
        id: TaskId,
        name: impl Into<String>,
        torrent_path: impl Into<String>,
        download_dir: impl Into<String>,
        total_bytes: u64,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            torrent_path: torrent_path.into(),
            download_dir: download_dir.into(),
            status: TaskStatus::Queued,
            total_bytes,
            downloaded_bytes: 0,
            files: Vec::new(),
            error: None,
            created_at: "1970-01-01T00:00:00Z".to_string(),
            updated_at: "1970-01-01T00:00:00Z".to_string(),
        }
    }

    pub fn transition_to(&mut self, next: TaskStatus) -> Result<(), AppError> {
        if self.status == next {
            return Ok(());
        }

        if !self.status.can_transition_to(next) {
            return Err(AppError::InvalidTransition {
                from: self.status.as_str(),
                to: next.as_str(),
            });
        }

        self.status = next;
        Ok(())
    }
}
