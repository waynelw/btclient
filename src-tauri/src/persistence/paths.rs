use std::path::{Component, Path};

use crate::domain::errors::AppError;

pub fn validate_torrent_relative_path(path: &str) -> Result<(), AppError> {
    if path.is_empty() {
        return Err(AppError::InvalidPath {
            message: "torrent file path is empty".to_string(),
        });
    }

    if path.contains("//") {
        return Err(AppError::InvalidPath {
            message: format!("torrent file path contains an empty component: {path}"),
        });
    }

    let path = Path::new(path);
    if path.is_absolute() {
        return Err(AppError::InvalidPath {
            message: format!("torrent file path must be relative: {}", path.display()),
        });
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(AppError::InvalidPath {
                    message: format!(
                        "torrent file path escapes download directory: {}",
                        path.display()
                    ),
                });
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(AppError::InvalidPath {
                    message: format!("unsupported path component in {}", path.display()),
                });
            }
        }
    }

    Ok(())
}
