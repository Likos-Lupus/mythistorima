use chrono::Utc;
use serde_json::Value;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::document::{
        CreateDocumentInput, DocumentContentDto, DocumentDto, UpdateDocumentContentInput,
    },
};

const EMPTY_DOCUMENT_JSON: &str = r#"{"type":"doc","content":[{"type":"paragraph"}]}"#;

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn validate_title(title: &str) -> AppResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("章节标题不能为空"));
    }
    Ok(trimmed.to_string())
}

pub async fn create_document(
    pool: &SqlitePool,
    input: CreateDocumentInput,
) -> AppResult<DocumentDto> {
    let title = validate_title(&input.title)?;
    let document_id = Uuid::new_v4().to_string();
    let now = now_ms();
    let sort_order = match input.sort_order {
        Some(value) => value,
        None => next_sort_order(pool, &input.project_id).await?,
    };

    let mut tx = pool.begin().await?;

    let project_exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1")
        .bind(&input.project_id)
        .fetch_optional(&mut *tx)
        .await?;

    if project_exists.is_none() {
        return Err(AppError::not_found("project"));
    }

    sqlx::query(
        r#"
        INSERT INTO documents
          (id, project_id, parent_id, type, title, sort_order, status, character_count, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, 'draft', 0, ?7, ?8)
        "#,
    )
    .bind(&document_id)
    .bind(&input.project_id)
    .bind(&input.parent_id)
    .bind(&input.document_type)
    .bind(&title)
    .bind(sort_order)
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
    .bind(&document_id)
    .bind(EMPTY_DOCUMENT_JSON)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    upsert_document_search_index(
        &mut tx,
        &input.project_id,
        &document_id,
        &input.document_type,
        &title,
        "",
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&input.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_document(pool, document_id).await
}

pub async fn list_documents(pool: &SqlitePool, project_id: String) -> AppResult<Vec<DocumentDto>> {
    let documents = sqlx::query_as::<_, DocumentDto>(
        r#"
        SELECT
          id,
          project_id,
          parent_id,
          type as document_type,
          title,
          sort_order,
          status,
          summary,
          metadata_json,
          character_count,
          created_at,
          updated_at
        FROM documents
        WHERE project_id = ?1
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(documents)
}

pub async fn get_document(pool: &SqlitePool, document_id: String) -> AppResult<DocumentDto> {
    let document = sqlx::query_as::<_, DocumentDto>(
        r#"
        SELECT
          id,
          project_id,
          parent_id,
          type as document_type,
          title,
          sort_order,
          status,
          summary,
          metadata_json,
          character_count,
          created_at,
          updated_at
        FROM documents
        WHERE id = ?1
        "#,
    )
    .bind(document_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("document"))?;

    Ok(document)
}

pub async fn get_document_content(
    pool: &SqlitePool,
    document_id: String,
) -> AppResult<DocumentContentDto> {
    let row = sqlx::query(
        r#"
        SELECT
          c.document_id,
          c.schema_version,
          c.content_json,
          c.content_text,
          c.content_html,
          d.character_count,
          c.updated_at
        FROM document_contents c
        INNER JOIN documents d ON d.id = c.document_id
        WHERE c.document_id = ?1
        "#,
    )
    .bind(document_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("document"))?;

    let content_json_raw: String = row.try_get("content_json")?;
    let content_json = serde_json::from_str::<Value>(&content_json_raw).unwrap_or_else(|_| {
        serde_json::json!({
            "type": "doc",
            "content": [{ "type": "paragraph" }]
        })
    });

    Ok(DocumentContentDto {
        document_id: row.try_get("document_id")?,
        schema_version: row.try_get("schema_version")?,
        content_json,
        content_text: row.try_get("content_text")?,
        content_html: row.try_get("content_html")?,
        character_count: row.try_get("character_count")?,
        updated_at: row.try_get("updated_at")?,
    })
}

pub async fn update_document_content(
    pool: &SqlitePool,
    input: UpdateDocumentContentInput,
) -> AppResult<DocumentContentDto> {
    let now = now_ms();
    let content_json = serde_json::to_string(&input.content_json).map_err(|error| {
        AppError::with_detail(
            "INVALID_CONTENT_JSON",
            "正文 JSON 无法序列化",
            error.to_string(),
        )
    })?;

    let mut tx = pool.begin().await?;

    let document_project: Option<(String,)> =
        sqlx::query_as("SELECT project_id FROM documents WHERE id = ?1")
            .bind(&input.document_id)
            .fetch_optional(&mut *tx)
            .await?;

    let project_id = document_project
        .map(|item| item.0)
        .ok_or_else(|| AppError::not_found("document"))?;

    sqlx::query(
        r#"
        UPDATE document_contents
        SET content_json = ?1,
            content_text = ?2,
            content_html = ?3,
            updated_at = ?4
        WHERE document_id = ?5
        "#,
    )
    .bind(content_json)
    .bind(&input.content_text)
    .bind(&input.content_html)
    .bind(now)
    .bind(&input.document_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE documents
        SET character_count = ?1,
            updated_at = ?2
        WHERE id = ?3
        "#,
    )
    .bind(input.character_count)
    .bind(now)
    .bind(&input.document_id)
    .execute(&mut *tx)
    .await?;

    let search_row = sqlx::query(
        r#"
        SELECT project_id, type, title
        FROM documents
        WHERE id = ?1
        "#,
    )
    .bind(&input.document_id)
    .fetch_one(&mut *tx)
    .await?;

    let search_project_id: String = search_row.try_get("project_id")?;
    let search_document_type: String = search_row.try_get("type")?;
    let search_title: String = search_row.try_get("title")?;

    upsert_document_search_index(
        &mut tx,
        &search_project_id,
        &input.document_id,
        &search_document_type,
        &search_title,
        &input.content_text,
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_document_content(pool, input.document_id).await
}

pub async fn rename_document(
    pool: &SqlitePool,
    document_id: String,
    title: String,
) -> AppResult<DocumentDto> {
    let title = validate_title(&title)?;
    let now = now_ms();

    let mut tx = pool.begin().await?;

    let result = sqlx::query("UPDATE documents SET title = ?1, updated_at = ?2 WHERE id = ?3")
        .bind(&title)
        .bind(now)
        .bind(&document_id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("document"));
    }

    let row = sqlx::query(
        r#"
        SELECT d.project_id, d.type, COALESCE(c.content_text, '') AS content_text
        FROM documents d
        LEFT JOIN document_contents c ON c.document_id = d.id
        WHERE d.id = ?1
        "#,
    )
    .bind(&document_id)
    .fetch_one(&mut *tx)
    .await?;

    let project_id: String = row.try_get("project_id")?;
    let document_type: String = row.try_get("type")?;
    let content_text: String = row.try_get("content_text")?;

    upsert_document_search_index(
        &mut tx,
        &project_id,
        &document_id,
        &document_type,
        &title,
        &content_text,
    )
    .await?;

    tx.commit().await?;

    get_document(pool, document_id).await
}

pub async fn delete_document(pool: &SqlitePool, document_id: String) -> AppResult<bool> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type IN ('volume', 'chapter', 'scene', 'note')")
        .bind(&document_id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM documents WHERE id = ?1")
        .bind(document_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(result.rows_affected() > 0)
}

async fn upsert_document_search_index(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    document_id: &str,
    document_type: &str,
    title: &str,
    body: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = ?2")
        .bind(document_id)
        .bind(document_type)
        .execute(&mut **tx)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(document_type)
    .bind(document_id)
    .bind(project_id)
    .bind(title)
    .bind(body)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn next_sort_order(pool: &SqlitePool, project_id: &str) -> AppResult<i64> {
    let row = sqlx::query("SELECT COALESCE(MAX(sort_order), -1) + 1 AS next_order FROM documents WHERE project_id = ?1")
        .bind(project_id)
        .fetch_one(pool)
        .await?;

    Ok(row.try_get("next_order")?)
}
