use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::domain::errors::AppError;
use crate::domain::task::{TaskId, TaskRecord, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskRepository {
    path: PathBuf,
}

impl TaskRepository {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Vec<TaskRecord>, AppError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let bytes = fs::read(&self.path).map_err(|err| AppError::Persistence {
            message: format!("failed to read {}: {err}", self.path.display()),
        })?;

        let text = String::from_utf8(bytes).map_err(|err| AppError::Persistence {
            message: format!("failed to decode {} as utf-8: {err}", self.path.display()),
        })?;

        parse_tasks_json(&text).map_err(|message| AppError::Persistence {
            message: format!("failed to parse {}: {message}", self.path.display()),
        })
    }

    pub fn save(&self, tasks: &[TaskRecord]) -> Result<(), AppError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| AppError::Persistence {
                message: format!("failed to create {}: {err}", parent.display()),
            })?;
        }

        let tmp_path = self.path.with_extension("json.tmp");
        let json = tasks_to_json(tasks).into_bytes();

        {
            let mut file = File::create(&tmp_path).map_err(|err| AppError::Persistence {
                message: format!("failed to create {}: {err}", tmp_path.display()),
            })?;
            file.write_all(&json).map_err(|err| AppError::Persistence {
                message: format!("failed to write {}: {err}", tmp_path.display()),
            })?;
            file.sync_all().map_err(|err| AppError::Persistence {
                message: format!("failed to sync {}: {err}", tmp_path.display()),
            })?;
        }

        fs::rename(&tmp_path, &self.path).map_err(|err| AppError::Persistence {
            message: format!(
                "failed to replace {} with {}: {err}",
                self.path.display(),
                tmp_path.display()
            ),
        })
    }
}

fn tasks_to_json(tasks: &[TaskRecord]) -> String {
    let mut out = String::from("[\n");
    for (index, task) in tasks.iter().enumerate() {
        if index > 0 {
            out.push_str(",\n");
        }
        out.push_str("  {\n");
        push_json_string_field(&mut out, "id", task.id.as_str(), true);
        push_json_string_field(&mut out, "name", &task.name, true);
        push_json_string_field(&mut out, "torrent_path", &task.torrent_path, true);
        push_json_string_field(&mut out, "download_dir", &task.download_dir, true);
        push_json_string_field(&mut out, "status", task.status.as_str(), true);
        out.push_str(&format!("    \"total_bytes\": {},\n", task.total_bytes));
        out.push_str(&format!(
            "    \"downloaded_bytes\": {},\n",
            task.downloaded_bytes
        ));
        push_json_string_field(&mut out, "created_at", &task.created_at, true);
        push_json_string_field(&mut out, "updated_at", &task.updated_at, false);
        out.push_str("\n  }");
    }
    out.push_str("\n]\n");
    out
}

fn push_json_string_field(out: &mut String, key: &str, value: &str, trailing_comma: bool) {
    out.push_str("    \"");
    out.push_str(key);
    out.push_str("\": \"");
    out.push_str(&escape_json_string(value));
    out.push('"');
    if trailing_comma {
        out.push(',');
    }
    out.push('\n');
}

fn escape_json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn parse_tasks_json(text: &str) -> Result<Vec<TaskRecord>, String> {
    let trimmed = text.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err("expected a JSON array".to_string());
    }

    let body = &trimmed[1..trimmed.len() - 1];
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut tasks = Vec::new();
    for object in split_objects(body)? {
        tasks.push(parse_task_object(object)?);
    }
    Ok(tasks)
}

fn split_objects(body: &str) -> Result<Vec<&str>, String> {
    let mut objects = Vec::new();
    let mut depth = 0usize;
    let mut start = None;
    let mut in_string = false;
    let mut escaped = false;

    for (index, ch) in body.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_string {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match ch {
            '{' => {
                if depth == 0 {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or_else(|| "unexpected }".to_string())?;
                if depth == 0 {
                    let start_index =
                        start.ok_or_else(|| "object ended before it started".to_string())?;
                    objects.push(&body[start_index..=index]);
                    start = None;
                }
            }
            _ => {}
        }
    }

    if depth != 0 || in_string {
        return Err("unterminated JSON object or string".to_string());
    }
    if objects.is_empty() {
        return Err("expected at least one task object".to_string());
    }
    Ok(objects)
}

fn parse_task_object(object: &str) -> Result<TaskRecord, String> {
    let id = get_string_field(object, "id")?;
    let name = get_string_field(object, "name")?;
    let torrent_path = get_string_field(object, "torrent_path")?;
    let download_dir = get_string_field(object, "download_dir")?;
    let status = get_string_field(object, "status")?;
    let total_bytes = get_u64_field(object, "total_bytes")?;
    let downloaded_bytes = get_u64_field(object, "downloaded_bytes")?;
    let created_at = get_string_field(object, "created_at")?;
    let updated_at = get_string_field(object, "updated_at")?;

    let mut task = TaskRecord::new(
        TaskId::new(id),
        name,
        torrent_path,
        download_dir,
        total_bytes,
    );
    task.status = TaskStatus::parse(&status).ok_or_else(|| format!("unknown status {status}"))?;
    task.downloaded_bytes = downloaded_bytes;
    task.created_at = created_at;
    task.updated_at = updated_at;
    Ok(task)
}

fn get_string_field(object: &str, key: &str) -> Result<String, String> {
    let marker = format!("\"{key}\":");
    let start = object
        .find(&marker)
        .ok_or_else(|| format!("missing string field {key}"))?
        + marker.len();
    let rest = object[start..].trim_start();
    if !rest.starts_with('"') {
        return Err(format!("field {key} is not a string"));
    }
    read_json_string(&rest[1..]).ok_or_else(|| format!("unterminated string field {key}"))
}

fn read_json_string(input: &str) -> Option<String> {
    let mut value = String::new();
    let mut escaped = false;
    for ch in input.chars() {
        if escaped {
            match ch {
                '"' => value.push('"'),
                '\\' => value.push('\\'),
                'n' => value.push('\n'),
                'r' => value.push('\r'),
                't' => value.push('\t'),
                _ => value.push(ch),
            }
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => return Some(value),
            _ => value.push(ch),
        }
    }
    None
}

fn get_u64_field(object: &str, key: &str) -> Result<u64, String> {
    let marker = format!("\"{key}\":");
    let start = object
        .find(&marker)
        .ok_or_else(|| format!("missing numeric field {key}"))?
        + marker.len();
    let rest = object[start..].trim_start();
    let end = rest
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(rest.len());
    if end == 0 {
        return Err(format!("field {key} is not a number"));
    }
    rest[..end]
        .parse()
        .map_err(|err| format!("field {key} is invalid: {err}"))
}
