use chrono::Utc;
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::project::{CreateProjectInput, ProjectDto, UpdateProjectInput},
};

const EMPTY_DOCUMENT_JSON: &str = r#"{"type":"doc","content":[{"type":"paragraph"}]}"#;

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

fn normalize_language(value: Option<String>) -> String {
    let normalized = value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .unwrap_or_else(|| "zh-CN".to_string());
    match normalized.as_str() {
        "zh-CN" | "en" => normalized,
        _ => "zh-CN".to_string(),
    }
}

fn normalize_project_status(value: Option<String>) -> String {
    match value.as_deref().map(str::trim) {
        Some("archived") => "archived".to_string(),
        _ => "active".to_string(),
    }
}

fn normalize_optional_count(value: Option<i64>) -> Option<i64> {
    value.filter(|item| *item > 0)
}

fn normalize_metadata_json(value: Option<String>) -> AppResult<Option<String>> {
    let Some(raw) = value else {
        return Ok(None);
    };
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(Some("{}".to_string()));
    }

    let parsed: Value = serde_json::from_str(trimmed).map_err(|error| {
        AppError::invalid_input(format!("项目 metadata 不是有效 JSON：{error}"))
    })?;
    if !parsed.is_object() {
        return Err(AppError::invalid_input("项目 metadata 必须是 JSON 对象"));
    }

    serde_json::to_string(&parsed)
        .map(Some)
        .map_err(|error| AppError::invalid_input(format!("项目 metadata 无法序列化：{error}")))
}

fn validate_title(title: &str) -> AppResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("标题不能为空"));
    }
    Ok(trimmed.to_string())
}

pub async fn create_project(pool: &SqlitePool, input: CreateProjectInput) -> AppResult<ProjectDto> {
    let title = validate_title(&input.title)?;
    let now = now_ms();
    let project_id = Uuid::new_v4().to_string();
    let first_document_id = Uuid::new_v4().to_string();

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO projects
          (id, title, author, description, status, language, metadata_json, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, 'active', ?5, '{}', ?6, ?7)
        "#,
    )
    .bind(&project_id)
    .bind(&title)
    .bind(
        input
            .author
            .as_deref()
            .filter(|value| !value.trim().is_empty()),
    )
    .bind(
        input
            .description
            .as_deref()
            .filter(|value| !value.trim().is_empty()),
    )
    .bind(input.language.as_deref().unwrap_or("zh-CN"))
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO documents
          (id, project_id, parent_id, type, title, sort_order, status, character_count, created_at, updated_at)
        VALUES
          (?1, ?2, NULL, 'chapter', '第一章', 0, 'draft', 0, ?3, ?4)
        "#,
    )
    .bind(&first_document_id)
    .bind(&project_id)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO document_contents
          (document_id, schema_version, content_json, content_text, content_html, updated_at)
        VALUES
          (?1, 1, ?2, '', '', ?3)
        "#,
    )
    .bind(&first_document_id)
    .bind(EMPTY_DOCUMENT_JSON)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES ('chapter', ?1, ?2, '第一章', '')
        "#,
    )
    .bind(&first_document_id)
    .bind(&project_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    get_project(pool, project_id).await
}

pub async fn list_projects(pool: &SqlitePool) -> AppResult<Vec<ProjectDto>> {
    let rows = sqlx::query_as::<_, ProjectDto>(
        r#"
        SELECT
          id,
          title,
          author,
          description,
          status,
          language,
          cover_asset_id,
          target_character_count,
          daily_target_count,
          metadata_json,
          created_at,
          updated_at
        FROM projects
        ORDER BY updated_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_project(pool: &SqlitePool, project_id: String) -> AppResult<ProjectDto> {
    let project = sqlx::query_as::<_, ProjectDto>(
        r#"
        SELECT
          id,
          title,
          author,
          description,
          status,
          language,
          cover_asset_id,
          target_character_count,
          daily_target_count,
          metadata_json,
          created_at,
          updated_at
        FROM projects
        WHERE id = ?1
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("project"))?;

    Ok(project)
}

pub async fn update_project(pool: &SqlitePool, input: UpdateProjectInput) -> AppResult<ProjectDto> {
    let title = validate_title(&input.title)?;
    let author = normalize_optional_text(input.author);
    let description = normalize_optional_text(input.description);
    let language = normalize_language(input.language);
    let status = normalize_project_status(input.status);
    let target_character_count = normalize_optional_count(input.target_character_count);
    let daily_target_count = normalize_optional_count(input.daily_target_count);
    let metadata_json = normalize_metadata_json(input.metadata_json)?;
    let now = now_ms();

    let result = sqlx::query(
        r#"
        UPDATE projects SET
          title = ?2,
          author = ?3,
          description = ?4,
          language = ?5,
          status = ?6,
          target_character_count = ?7,
          daily_target_count = ?8,
          metadata_json = COALESCE(?9, metadata_json),
          updated_at = ?10
        WHERE id = ?1
        "#,
    )
    .bind(&input.project_id)
    .bind(&title)
    .bind(author.as_deref())
    .bind(description.as_deref())
    .bind(&language)
    .bind(&status)
    .bind(target_character_count)
    .bind(daily_target_count)
    .bind(metadata_json.as_deref())
    .bind(now)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("project"));
    }

    get_project(pool, input.project_id).await
}

pub async fn delete_project(pool: &SqlitePool, project_id: String) -> AppResult<bool> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM search_index WHERE project_id = ?1")
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM projects WHERE id = ?1")
        .bind(project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(result.rows_affected() > 0)
}
