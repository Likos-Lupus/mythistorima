use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::backup::BackupDto,
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn backups_root(database_path: &Path) -> PathBuf {
    database_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("backups")
}

fn backup_dir(database_path: &Path, project_id: &str) -> PathBuf {
    backups_root(database_path).join(project_id)
}

pub fn create_backup(database_path: &Path, project_id: String) -> AppResult<BackupDto> {
    if !database_path.exists() {
        return Err(AppError::with_detail(
            "BACKUP_SOURCE_NOT_FOUND",
            "数据库文件不存在，无法备份",
            database_path.to_string_lossy().to_string(),
        ));
    }

    let created_at = now_ms();
    let id = Uuid::new_v4().to_string();
    let dir = backup_dir(database_path, &project_id);
    fs::create_dir_all(&dir).map_err(|error| {
        AppError::with_detail("BACKUP_DIR_FAILED", "无法创建备份目录", error.to_string())
    })?;

    let db_path = dir.join(format!("{}-{}.sqlite", created_at, id));
    fs::copy(database_path, &db_path).map_err(|error| {
        AppError::with_detail(
            "BACKUP_COPY_FAILED",
            "复制数据库备份失败",
            error.to_string(),
        )
    })?;

    let size_bytes = fs::metadata(&db_path)
        .map(|metadata| metadata.len() as i64)
        .unwrap_or(0);

    let metadata_path = dir.join(format!("{}-{}.json", created_at, id));
    let metadata = json!({
        "id": id,
        "projectId": project_id.clone(),
        "path": db_path.to_string_lossy(),
        "sizeBytes": size_bytes,
        "createdAt": created_at
    });
    let _ = fs::write(
        &metadata_path,
        serde_json::to_string_pretty(&metadata).unwrap_or_else(|_| "{}".to_string()),
    );

    Ok(BackupDto {
        id,
        project_id,
        path: db_path.to_string_lossy().to_string(),
        size_bytes,
        created_at,
    })
}

pub fn list_backups(database_path: &Path, project_id: String) -> AppResult<Vec<BackupDto>> {
    let dir = backup_dir(database_path, &project_id);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut backups = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|error| {
        AppError::with_detail("BACKUP_LIST_FAILED", "读取备份目录失败", error.to_string())
    })? {
        let entry = entry.map_err(|error| {
            AppError::with_detail("BACKUP_LIST_FAILED", "读取备份项失败", error.to_string())
        })?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("sqlite") {
            continue;
        }
        let file_stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        let mut parts = file_stem.splitn(2, '-');
        let created_at = parts
            .next()
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(0);
        let id = parts.next().unwrap_or(file_stem).to_string();
        let size_bytes = fs::metadata(&path)
            .map(|metadata| metadata.len() as i64)
            .unwrap_or(0);
        backups.push(BackupDto {
            id,
            project_id: project_id.clone(),
            path: path.to_string_lossy().to_string(),
            size_bytes,
            created_at,
        });
    }
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(backups)
}
