use sqlx::{Row, SqlitePool};

use crate::{
    errors::{AppError, AppResult},
    models::stats::{DocumentStatsDto, ProjectStatsDto},
};

pub async fn get_project_stats(
    pool: &SqlitePool,
    project_id: String,
) -> AppResult<ProjectStatsDto> {
    let project_exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1")
        .bind(&project_id)
        .fetch_optional(pool)
        .await?;

    if project_exists.is_none() {
        return Err(AppError::not_found("project"));
    }

    let row = sqlx::query(
        r#"
        SELECT
          COUNT(id) AS document_count,
          COALESCE(SUM(character_count), 0) AS character_count,
          MAX(updated_at) AS updated_at
        FROM documents
        WHERE project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_one(pool)
    .await?;

    Ok(ProjectStatsDto {
        project_id,
        document_count: row.try_get("document_count")?,
        character_count: row.try_get("character_count")?,
        updated_at: row.try_get("updated_at")?,
    })
}

pub async fn get_document_stats(
    pool: &SqlitePool,
    document_id: String,
) -> AppResult<DocumentStatsDto> {
    let row = sqlx::query(
        r#"
        SELECT id, character_count, updated_at
        FROM documents
        WHERE id = ?1
        "#,
    )
    .bind(&document_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("document"))?;

    Ok(DocumentStatsDto {
        document_id: row.try_get("id")?,
        character_count: row.try_get("character_count")?,
        updated_at: row.try_get("updated_at")?,
    })
}
