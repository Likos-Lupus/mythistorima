use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::note::{CreateNoteInput, ListNotesInput, NoteDto, UpdateNoteInput},
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn validate_note_type(note_type: &str) -> AppResult<String> {
    let normalized = note_type.trim();
    match normalized {
        "memo" | "todo" | "foreshadow" | "issue" | "idea" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "事项类型必须是 memo / todo / foreshadow / issue / idea",
        )),
    }
}

fn validate_status(status: &str) -> AppResult<String> {
    let normalized = status.trim();
    match normalized {
        "open" | "doing" | "done" | "archived" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "事项状态必须是 open / doing / done / archived",
        )),
    }
}

fn validate_priority(priority: &str) -> AppResult<String> {
    let normalized = priority.trim();
    match normalized {
        "low" | "normal" | "high" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "事项优先级必须是 low / normal / high",
        )),
    }
}

fn validate_title(title: &str) -> AppResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("事项标题不能为空"));
    }
    if trimmed.chars().count() > 160 {
        return Err(AppError::invalid_input("事项标题不能超过 160 个字符"));
    }
    Ok(trimmed.to_string())
}

fn normalize_body(body: Option<String>) -> String {
    body.unwrap_or_default().trim().to_string()
}

fn normalize_optional_id(id: Option<String>) -> Option<String> {
    id.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

pub async fn create_note(pool: &SqlitePool, input: CreateNoteInput) -> AppResult<NoteDto> {
    let note_type = validate_note_type(&input.note_type)?;
    let title = validate_title(&input.title)?;
    let body = normalize_body(input.body);
    let priority = validate_priority(input.priority.as_deref().unwrap_or("normal"))?;
    let document_id = normalize_optional_id(input.document_id);
    let paragraph_id = normalize_optional_id(input.paragraph_id);
    let note_id = Uuid::new_v4().to_string();
    let now = now_ms();

    let mut tx = pool.begin().await?;

    ensure_project_exists(&mut tx, &input.project_id).await?;
    if let Some(document_id) = document_id.as_deref() {
        ensure_document_belongs_to_project(&mut tx, document_id, &input.project_id).await?;
    }

    sqlx::query(
        r#"
        INSERT INTO notes
          (id, project_id, document_id, paragraph_id, type, title, body, status, priority, due_at, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'open', ?8, ?9, ?10, ?11)
        "#,
    )
    .bind(&note_id)
    .bind(&input.project_id)
    .bind(document_id.as_deref())
    .bind(paragraph_id.as_deref())
    .bind(&note_type)
    .bind(&title)
    .bind(&body)
    .bind(&priority)
    .bind(input.due_at)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    upsert_note_search_index(
        &mut tx,
        &input.project_id,
        &note_id,
        &note_type,
        &title,
        &body,
        "open",
        &priority,
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&input.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_note(pool, note_id).await
}

pub async fn update_note(pool: &SqlitePool, input: UpdateNoteInput) -> AppResult<NoteDto> {
    let existing = get_note(pool, input.note_id.clone()).await?;
    let note_type = match input.note_type {
        Some(value) => validate_note_type(&value)?,
        None => existing.note_type,
    };
    let title = match input.title {
        Some(value) => validate_title(&value)?,
        None => existing.title,
    };
    let body = input.body.unwrap_or(existing.body).trim().to_string();
    let status = match input.status {
        Some(value) => validate_status(&value)?,
        None => existing.status,
    };
    let priority = match input.priority {
        Some(value) => validate_priority(&value)?,
        None => existing.priority,
    };
    let document_id = match input.document_id {
        Some(value) => normalize_optional_id(value),
        None => existing.document_id,
    };
    let paragraph_id = match input.paragraph_id {
        Some(value) => normalize_optional_id(value),
        None => existing.paragraph_id,
    };
    let due_at = match input.due_at {
        Some(value) => value,
        None => existing.due_at,
    };
    let now = now_ms();

    let mut tx = pool.begin().await?;

    if let Some(document_id) = document_id.as_deref() {
        ensure_document_belongs_to_project(&mut tx, document_id, &existing.project_id).await?;
    }

    sqlx::query(
        r#"
        UPDATE notes
        SET document_id = ?1,
            paragraph_id = ?2,
            type = ?3,
            title = ?4,
            body = ?5,
            status = ?6,
            priority = ?7,
            due_at = ?8,
            updated_at = ?9
        WHERE id = ?10
        "#,
    )
    .bind(document_id.as_deref())
    .bind(paragraph_id.as_deref())
    .bind(&note_type)
    .bind(&title)
    .bind(&body)
    .bind(&status)
    .bind(&priority)
    .bind(due_at)
    .bind(now)
    .bind(&input.note_id)
    .execute(&mut *tx)
    .await?;

    upsert_note_search_index(
        &mut tx,
        &existing.project_id,
        &input.note_id,
        &note_type,
        &title,
        &body,
        &status,
        &priority,
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&existing.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_note(pool, input.note_id).await
}

pub async fn update_note_status(
    pool: &SqlitePool,
    note_id: String,
    status: String,
) -> AppResult<NoteDto> {
    update_note(
        pool,
        UpdateNoteInput {
            note_id,
            document_id: None,
            paragraph_id: None,
            note_type: None,
            title: None,
            body: None,
            status: Some(status),
            priority: None,
            due_at: None,
        },
    )
    .await
}

pub async fn delete_note(pool: &SqlitePool, note_id: String) -> AppResult<bool> {
    let existing = get_note(pool, note_id.clone()).await?;
    let now = now_ms();
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = 'note'")
        .bind(&note_id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM notes WHERE id = ?1")
        .bind(&note_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&existing.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_note(pool: &SqlitePool, note_id: String) -> AppResult<NoteDto> {
    let note = sqlx::query_as::<_, NoteDto>(
        r#"
        SELECT
          n.id,
          n.project_id,
          n.document_id,
          d.title as document_title,
          n.paragraph_id,
          n.type as note_type,
          n.title,
          n.body,
          n.status,
          n.priority,
          n.due_at,
          n.created_at,
          n.updated_at
        FROM notes n
        LEFT JOIN documents d ON d.id = n.document_id
        WHERE n.id = ?1
        "#,
    )
    .bind(note_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("note"))?;

    Ok(note)
}

pub async fn list_notes(pool: &SqlitePool, input: ListNotesInput) -> AppResult<Vec<NoteDto>> {
    let note_type = match input.note_type.as_deref().filter(|value| *value != "all") {
        Some(value) => Some(validate_note_type(value)?),
        None => None,
    };
    let status = match input.status.as_deref().filter(|value| *value != "all") {
        Some(value) => Some(validate_status(value)?),
        None => None,
    };
    let document_id = normalize_optional_id(input.document_id);
    let paragraph_id = normalize_optional_id(input.paragraph_id);

    let notes = sqlx::query_as::<_, NoteDto>(
        r#"
        SELECT
          n.id,
          n.project_id,
          n.document_id,
          d.title as document_title,
          n.paragraph_id,
          n.type as note_type,
          n.title,
          n.body,
          n.status,
          n.priority,
          n.due_at,
          n.created_at,
          n.updated_at
        FROM notes n
        LEFT JOIN documents d ON d.id = n.document_id
        WHERE n.project_id = ?1
          AND (?2 IS NULL OR n.document_id = ?2)
          AND (?3 IS NULL OR n.paragraph_id = ?3)
          AND (?4 IS NULL OR n.type = ?4)
          AND (?5 IS NULL OR n.status = ?5)
        ORDER BY
          CASE n.status
            WHEN 'open' THEN 0
            WHEN 'doing' THEN 1
            WHEN 'done' THEN 2
            ELSE 3
          END ASC,
          CASE n.priority
            WHEN 'high' THEN 0
            WHEN 'normal' THEN 1
            ELSE 2
          END ASC,
          n.updated_at DESC,
          n.created_at DESC
        "#,
    )
    .bind(input.project_id)
    .bind(document_id.as_deref())
    .bind(paragraph_id.as_deref())
    .bind(note_type.as_deref())
    .bind(status.as_deref())
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

async fn ensure_project_exists(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1")
        .bind(project_id)
        .fetch_optional(&mut **tx)
        .await?;

    if exists.is_none() {
        return Err(AppError::not_found("project"));
    }
    Ok(())
}

async fn ensure_document_belongs_to_project(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_id: &str,
    project_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM documents WHERE id = ?1 AND project_id = ?2")
            .bind(document_id)
            .bind(project_id)
            .fetch_optional(&mut **tx)
            .await?;

    if exists.is_none() {
        return Err(AppError::not_found("document"));
    }
    Ok(())
}

async fn upsert_note_search_index(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    note_id: &str,
    note_type: &str,
    title: &str,
    body: &str,
    status: &str,
    priority: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = 'note'")
        .bind(note_id)
        .execute(&mut **tx)
        .await?;

    let search_body = format!("{}\n{}\n{}\n{}", note_type, status, priority, body);

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES ('note', ?1, ?2, ?3, ?4)
        "#,
    )
    .bind(note_id)
    .bind(project_id)
    .bind(title)
    .bind(search_body)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
