use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::project::{CreateProjectInput, ProjectDto},
};

const EMPTY_DOCUMENT_JSON: &str = r#"{"type":"doc","content":[{"type":"paragraph"}]}"#;

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
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
        INSERT INTO projects (id, title, author, description, status, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, 'active', ?5, ?6)
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

    tx.commit().await?;

    get_project(pool, project_id).await
}

pub async fn list_projects(pool: &SqlitePool) -> AppResult<Vec<ProjectDto>> {
    let rows = sqlx::query_as::<_, ProjectDto>(
        r#"
        SELECT id, title, author, description, status, created_at, updated_at
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
        SELECT id, title, author, description, status, created_at, updated_at
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

pub async fn delete_project(pool: &SqlitePool, project_id: String) -> AppResult<bool> {
    let result = sqlx::query("DELETE FROM projects WHERE id = ?1")
        .bind(project_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
