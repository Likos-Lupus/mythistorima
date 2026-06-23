use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::foreshadow::{
        ForeshadowThreadDto, ForeshadowThreadFromNoteInput, ForeshadowThreadInput,
        ListForeshadowThreadsInput, UpdateForeshadowThreadInput,
    },
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn normalize_non_empty(value: &str, label: &str, max_len: usize) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input(format!("{}不能为空", label)));
    }
    if trimmed.chars().count() > max_len {
        return Err(AppError::invalid_input(format!(
            "{}不能超过 {} 个字符",
            label, max_len
        )));
    }
    Ok(trimmed.to_string())
}

fn normalize_optional_text(
    value: Option<String>,
    label: &str,
    max_len: usize,
) -> AppResult<String> {
    let text = value.unwrap_or_default().trim().to_string();
    if text.chars().count() > max_len {
        return Err(AppError::invalid_input(format!(
            "{}不能超过 {} 个字符",
            label, max_len
        )));
    }
    Ok(text)
}

fn normalize_optional_id(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn validate_status(value: &str) -> AppResult<String> {
    let normalized = value.trim();
    match normalized {
        "open" | "planned" | "paid_off" | "abandoned" | "archived" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "伏笔状态必须是 open / planned / paid_off / abandoned / archived",
        )),
    }
}

fn validate_priority(value: &str) -> AppResult<String> {
    let normalized = value.trim();
    match normalized {
        "low" | "normal" | "high" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input("优先级必须是 low / normal / high")),
    }
}

async fn ensure_project_exists(pool: &SqlitePool, project_id: &str) -> AppResult<()> {
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1 LIMIT 1")
        .bind(project_id)
        .fetch_optional(pool)
        .await?;

    if exists.is_none() {
        return Err(AppError::not_found("project"));
    }

    Ok(())
}

async fn ensure_note_in_project(
    pool: &SqlitePool,
    project_id: &str,
    note_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM notes WHERE id = ?1 AND project_id = ?2 LIMIT 1")
            .bind(note_id)
            .bind(project_id)
            .fetch_optional(pool)
            .await?;

    if exists.is_none() {
        return Err(AppError::invalid_input("关联事项必须属于当前项目"));
    }

    Ok(())
}

async fn ensure_document_in_project(
    pool: &SqlitePool,
    project_id: &str,
    document_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM documents WHERE id = ?1 AND project_id = ?2 LIMIT 1")
            .bind(document_id)
            .bind(project_id)
            .fetch_optional(pool)
            .await?;

    if exists.is_none() {
        return Err(AppError::invalid_input("关联章节必须属于当前项目"));
    }

    Ok(())
}

pub async fn create_foreshadow_thread(
    pool: &SqlitePool,
    input: ForeshadowThreadInput,
) -> AppResult<ForeshadowThreadDto> {
    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    let title = normalize_non_empty(&input.title, "伏笔标题", 160)?;
    let description = normalize_optional_text(input.description, "伏笔说明", 4000)?;
    let status = validate_status(input.status.as_deref().unwrap_or("open"))?;
    let priority = validate_priority(input.priority.as_deref().unwrap_or("normal"))?;
    let setup_note_id = normalize_optional_id(input.setup_note_id);
    let payoff_note_id = normalize_optional_id(input.payoff_note_id);
    let setup_document_id = normalize_optional_id(input.setup_document_id);
    let payoff_document_id = normalize_optional_id(input.payoff_document_id);
    let thread_id = Uuid::new_v4().to_string();
    let now = now_ms();

    ensure_project_exists(pool, &project_id).await?;
    if let Some(note_id) = setup_note_id.as_deref() {
        ensure_note_in_project(pool, &project_id, note_id).await?;
    }
    if let Some(note_id) = payoff_note_id.as_deref() {
        ensure_note_in_project(pool, &project_id, note_id).await?;
    }
    if let Some(document_id) = setup_document_id.as_deref() {
        ensure_document_in_project(pool, &project_id, document_id).await?;
    }
    if let Some(document_id) = payoff_document_id.as_deref() {
        ensure_document_in_project(pool, &project_id, document_id).await?;
    }

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO foreshadow_threads
          (id, project_id, title, description, status, setup_note_id, payoff_note_id, setup_document_id, payoff_document_id, priority, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
        "#,
    )
    .bind(&thread_id)
    .bind(&project_id)
    .bind(&title)
    .bind(&description)
    .bind(&status)
    .bind(setup_note_id.as_deref())
    .bind(payoff_note_id.as_deref())
    .bind(setup_document_id.as_deref())
    .bind(payoff_document_id.as_deref())
    .bind(&priority)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    upsert_foreshadow_search_index(
        &mut tx,
        &project_id,
        &thread_id,
        &title,
        &description,
        &status,
        &priority,
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_foreshadow_thread(pool, thread_id).await
}

pub async fn create_foreshadow_thread_from_note(
    pool: &SqlitePool,
    input: ForeshadowThreadFromNoteInput,
) -> AppResult<ForeshadowThreadDto> {
    let note_id = normalize_non_empty(&input.note_id, "事项 ID", 120)?;
    let note = sqlx::query(
        r#"
        SELECT id, project_id, document_id, title, body, priority
        FROM notes
        WHERE id = ?1
        "#,
    )
    .bind(&note_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("note"))?;

    let project_id: String = note.try_get("project_id")?;
    let note_title: String = note.try_get("title")?;
    let note_body: String = note.try_get("body")?;
    let note_priority: String = note.try_get("priority")?;
    let setup_document_id: Option<String> = note.try_get("document_id")?;
    let title = input
        .title
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(note_title);
    let description = input
        .description
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(note_body);
    let priority = input
        .priority
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(note_priority);

    create_foreshadow_thread(
        pool,
        ForeshadowThreadInput {
            project_id,
            title,
            description: Some(description),
            status: Some("open".to_string()),
            setup_note_id: Some(note_id),
            payoff_note_id: None,
            setup_document_id,
            payoff_document_id: input.payoff_document_id,
            priority: Some(priority),
        },
    )
    .await
}

pub async fn update_foreshadow_thread(
    pool: &SqlitePool,
    input: UpdateForeshadowThreadInput,
) -> AppResult<ForeshadowThreadDto> {
    let existing = get_foreshadow_thread(pool, input.thread_id.clone()).await?;
    let title = match input.title {
        Some(value) => normalize_non_empty(&value, "伏笔标题", 160)?,
        None => existing.title,
    };
    let description = match input.description {
        Some(value) => normalize_optional_text(Some(value), "伏笔说明", 4000)?,
        None => existing.description,
    };
    let status = match input.status {
        Some(value) => validate_status(&value)?,
        None => existing.status,
    };
    let priority = match input.priority {
        Some(value) => validate_priority(&value)?,
        None => existing.priority,
    };
    let setup_note_id = match input.setup_note_id {
        Some(value) => normalize_optional_id(value),
        None => existing.setup_note_id,
    };
    let payoff_note_id = match input.payoff_note_id {
        Some(value) => normalize_optional_id(value),
        None => existing.payoff_note_id,
    };
    let setup_document_id = match input.setup_document_id {
        Some(value) => normalize_optional_id(value),
        None => existing.setup_document_id,
    };
    let payoff_document_id = match input.payoff_document_id {
        Some(value) => normalize_optional_id(value),
        None => existing.payoff_document_id,
    };
    let now = now_ms();

    if let Some(note_id) = setup_note_id.as_deref() {
        ensure_note_in_project(pool, &existing.project_id, note_id).await?;
    }
    if let Some(note_id) = payoff_note_id.as_deref() {
        ensure_note_in_project(pool, &existing.project_id, note_id).await?;
    }
    if let Some(document_id) = setup_document_id.as_deref() {
        ensure_document_in_project(pool, &existing.project_id, document_id).await?;
    }
    if let Some(document_id) = payoff_document_id.as_deref() {
        ensure_document_in_project(pool, &existing.project_id, document_id).await?;
    }

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE foreshadow_threads
        SET title = ?1,
            description = ?2,
            status = ?3,
            setup_note_id = ?4,
            payoff_note_id = ?5,
            setup_document_id = ?6,
            payoff_document_id = ?7,
            priority = ?8,
            updated_at = ?9
        WHERE id = ?10
        "#,
    )
    .bind(&title)
    .bind(&description)
    .bind(&status)
    .bind(setup_note_id.as_deref())
    .bind(payoff_note_id.as_deref())
    .bind(setup_document_id.as_deref())
    .bind(payoff_document_id.as_deref())
    .bind(&priority)
    .bind(now)
    .bind(&input.thread_id)
    .execute(&mut *tx)
    .await?;

    upsert_foreshadow_search_index(
        &mut tx,
        &existing.project_id,
        &input.thread_id,
        &title,
        &description,
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

    get_foreshadow_thread(pool, input.thread_id).await
}

pub async fn mark_foreshadow_paid_off(
    pool: &SqlitePool,
    thread_id: String,
    payoff_document_id: Option<String>,
) -> AppResult<ForeshadowThreadDto> {
    update_foreshadow_thread(
        pool,
        UpdateForeshadowThreadInput {
            thread_id,
            title: None,
            description: None,
            status: Some("paid_off".to_string()),
            setup_note_id: None,
            payoff_note_id: None,
            setup_document_id: None,
            payoff_document_id: Some(payoff_document_id),
            priority: None,
        },
    )
    .await
}

pub async fn delete_foreshadow_thread(pool: &SqlitePool, thread_id: String) -> AppResult<bool> {
    let existing = get_foreshadow_thread(pool, thread_id.clone()).await?;
    let now = now_ms();
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = 'foreshadow'")
        .bind(&thread_id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM foreshadow_threads WHERE id = ?1")
        .bind(&thread_id)
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

pub async fn get_foreshadow_thread(
    pool: &SqlitePool,
    thread_id: String,
) -> AppResult<ForeshadowThreadDto> {
    let thread = sqlx::query_as::<_, ForeshadowThreadDto>(
        r#"
        SELECT
          f.id,
          f.project_id,
          f.title,
          f.description,
          f.status,
          f.setup_note_id,
          setup_note.title AS setup_note_title,
          f.payoff_note_id,
          payoff_note.title AS payoff_note_title,
          f.setup_document_id,
          setup_doc.title AS setup_document_title,
          f.payoff_document_id,
          payoff_doc.title AS payoff_document_title,
          f.priority,
          f.created_at,
          f.updated_at
        FROM foreshadow_threads f
        LEFT JOIN notes setup_note ON setup_note.id = f.setup_note_id
        LEFT JOIN notes payoff_note ON payoff_note.id = f.payoff_note_id
        LEFT JOIN documents setup_doc ON setup_doc.id = f.setup_document_id
        LEFT JOIN documents payoff_doc ON payoff_doc.id = f.payoff_document_id
        WHERE f.id = ?1
        LIMIT 1
        "#,
    )
    .bind(thread_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("foreshadow_thread"))?;

    Ok(thread)
}

pub async fn list_foreshadow_threads(
    pool: &SqlitePool,
    input: ListForeshadowThreadsInput,
) -> AppResult<Vec<ForeshadowThreadDto>> {
    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    ensure_project_exists(pool, &project_id).await?;

    let status = match input.status {
        Some(value) if value.trim().is_empty() || value == "all" => None,
        Some(value) => Some(validate_status(&value)?),
        None => None,
    };
    let priority = match input.priority {
        Some(value) if value.trim().is_empty() || value == "all" => None,
        Some(value) => Some(validate_priority(&value)?),
        None => None,
    };
    let only_unpaid = if input.only_unpaid.unwrap_or(false) {
        1_i64
    } else {
        0_i64
    };

    let threads = sqlx::query_as::<_, ForeshadowThreadDto>(
        r#"
        SELECT
          f.id,
          f.project_id,
          f.title,
          f.description,
          f.status,
          f.setup_note_id,
          setup_note.title AS setup_note_title,
          f.payoff_note_id,
          payoff_note.title AS payoff_note_title,
          f.setup_document_id,
          setup_doc.title AS setup_document_title,
          f.payoff_document_id,
          payoff_doc.title AS payoff_document_title,
          f.priority,
          f.created_at,
          f.updated_at
        FROM foreshadow_threads f
        LEFT JOIN notes setup_note ON setup_note.id = f.setup_note_id
        LEFT JOIN notes payoff_note ON payoff_note.id = f.payoff_note_id
        LEFT JOIN documents setup_doc ON setup_doc.id = f.setup_document_id
        LEFT JOIN documents payoff_doc ON payoff_doc.id = f.payoff_document_id
        WHERE f.project_id = ?1
          AND (?2 IS NULL OR f.status = ?2)
          AND (?3 IS NULL OR f.priority = ?3)
          AND (?4 = 0 OR f.status != 'paid_off')
        ORDER BY
          CASE f.status
            WHEN 'open' THEN 0
            WHEN 'planned' THEN 1
            WHEN 'paid_off' THEN 2
            WHEN 'abandoned' THEN 3
            WHEN 'archived' THEN 4
            ELSE 5
          END ASC,
          CASE f.priority
            WHEN 'high' THEN 0
            WHEN 'normal' THEN 1
            WHEN 'low' THEN 2
            ELSE 3
          END ASC,
          f.updated_at DESC
        "#,
    )
    .bind(project_id)
    .bind(status.as_deref())
    .bind(priority.as_deref())
    .bind(only_unpaid)
    .fetch_all(pool)
    .await?;

    Ok(threads)
}

async fn upsert_foreshadow_search_index(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    thread_id: &str,
    title: &str,
    description: &str,
    status: &str,
    priority: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = 'foreshadow'")
        .bind(thread_id)
        .execute(&mut **tx)
        .await?;

    let body = format!(
        "伏笔\n状态：{}\n优先级：{}\n{}",
        status, priority, description
    );
    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES ('foreshadow', ?1, ?2, ?3, ?4)
        "#,
    )
    .bind(thread_id)
    .bind(project_id)
    .bind(title)
    .bind(body)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
