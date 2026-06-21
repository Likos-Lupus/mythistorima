use sqlx::{Row, SqlitePool};

use crate::{
    errors::{AppError, AppResult},
    models::stats::{
        DocumentStatsDto, ProjectStatsDto, RecordWritingSessionInput, TodayWritingStatsDto,
        WritingSessionDto,
    },
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

pub async fn record_writing_session(
    pool: &SqlitePool,
    input: RecordWritingSessionInput,
) -> AppResult<WritingSessionDto> {
    let document_exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM documents WHERE id = ?1 AND project_id = ?2")
            .bind(&input.document_id)
            .bind(&input.project_id)
            .fetch_optional(pool)
            .await?;

    if document_exists.is_none() {
        return Err(AppError::not_found("document"));
    }

    let inserted_count = (input.characters_after - input.characters_before).max(0);
    let deleted_count = (input.characters_before - input.characters_after).max(0);

    sqlx::query(
        r#"
        INSERT INTO writing_sessions
          (id, project_id, document_id, started_at, ended_at, characters_before, characters_after, inserted_count, deleted_count)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ON CONFLICT(id) DO UPDATE SET
          ended_at = excluded.ended_at,
          characters_after = excluded.characters_after,
          inserted_count = excluded.inserted_count,
          deleted_count = excluded.deleted_count
        "#,
    )
    .bind(&input.session_id)
    .bind(&input.project_id)
    .bind(&input.document_id)
    .bind(input.started_at)
    .bind(input.ended_at)
    .bind(input.characters_before)
    .bind(input.characters_after)
    .bind(inserted_count)
    .bind(deleted_count)
    .execute(pool)
    .await?;

    let row = sqlx::query_as::<_, WritingSessionDto>(
        r#"
        SELECT
          id,
          project_id,
          document_id,
          started_at,
          ended_at,
          characters_before,
          characters_after,
          inserted_count,
          deleted_count
        FROM writing_sessions
        WHERE id = ?1
        "#,
    )
    .bind(input.session_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_today_writing_stats(
    pool: &SqlitePool,
    project_id: String,
    day_start: i64,
    day_end: i64,
) -> AppResult<TodayWritingStatsDto> {
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
          COALESCE(SUM(CASE WHEN characters_after > characters_before THEN characters_after - characters_before ELSE 0 END), 0) AS character_count,
          COALESCE(SUM(CASE WHEN ended_at IS NOT NULL AND ended_at > started_at THEN ended_at - started_at ELSE 0 END), 0) AS elapsed_ms
        FROM writing_sessions
        WHERE project_id = ?1
          AND started_at >= ?2
          AND started_at < ?3
        "#,
    )
    .bind(&project_id)
    .bind(day_start)
    .bind(day_end)
    .fetch_one(pool)
    .await?;

    Ok(TodayWritingStatsDto {
        project_id,
        character_count: row.try_get("character_count")?,
        elapsed_ms: row.try_get("elapsed_ms")?,
    })
}
