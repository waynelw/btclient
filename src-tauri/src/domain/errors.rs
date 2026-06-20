use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppErrorCode {
    InvalidTransition,
    TaskNotFound,
    Persistence,
    InvalidPath,
    EngineUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    InvalidTransition {
        from: &'static str,
        to: &'static str,
    },
    TaskNotFound {
        task_id: String,
    },
    Persistence {
        message: String,
    },
    InvalidPath {
        message: String,
    },
    EngineUnavailable {
        message: String,
    },
}

impl AppError {
    pub fn code(&self) -> AppErrorCode {
        match self {
            Self::InvalidTransition { .. } => AppErrorCode::InvalidTransition,
            Self::TaskNotFound { .. } => AppErrorCode::TaskNotFound,
            Self::Persistence { .. } => AppErrorCode::Persistence,
            Self::InvalidPath { .. } => AppErrorCode::InvalidPath,
            Self::EngineUnavailable { .. } => AppErrorCode::EngineUnavailable,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, to } => {
                write!(f, "invalid task transition from {from} to {to}")
            }
            Self::TaskNotFound { task_id } => write!(f, "task not found: {task_id}"),
            Self::Persistence { message } => write!(f, "persistence error: {message}"),
            Self::InvalidPath { message } => write!(f, "invalid path: {message}"),
            Self::EngineUnavailable { message } => write!(f, "engine unavailable: {message}"),
        }
    }
}

impl std::error::Error for AppError {}
