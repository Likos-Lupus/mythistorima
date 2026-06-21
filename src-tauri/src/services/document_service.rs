use chrono::Utc;
use serde_json::Value;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::document::{
        CreateDocumentInput, DocumentContentDto, DocumentDto, MoveDocumentInput,
        UpdateDocumentContentInput, UpdateDocumentStatusInput,
    },
};

const EMPTY_DOCUMENT_JSON: &str = r#"{"type":"doc","content":[{"type":"paragraph"}]}"#;

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn validate_title(title: &str) -> AppResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("文档标题不能为空"));
    }
    Ok(trimmed.to_string())
}

fn validate_document_type(document_type: &str) -> AppResult<String> {
    let normalized = document_type.trim();
    match normalized {
        "volume" | "chapter" | "scene" | "note" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "文档类型必须是 volume / chapter / scene / note",
        )),
    }
}

fn validate_status(status: &str) -> AppResult<String> {
    let normalized = status.trim();
    match normalized {
        "draft" | "writing" | "revised" | "done" | "archived" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input("文档状态无效")),
    }
}

fn same_parent(left: &Option<String>, right: &Option<String>) -> bool {
    left.as_deref() == right.as_deref()
}

pub async fn create_document(
    pool: &SqlitePool,
    input: CreateDocumentInput,
) -> AppResult<DocumentDto> {
    let title = validate_title(&input.title)?;
    let document_type = validate_document_type(&input.document_type)?;
    let document_id = Uuid::new_v4().to_string();
    let now = now_ms();
    let target_order = match input.sort_order {
        Some(value) => value.max(0),
        None => next_sort_order(pool, &input.project_id, &input.parent_id).await?,
    };

    let mut tx = pool.begin().await?;

    let project_exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1")
        .bind(&input.project_id)
        .fetch_optional(&mut *tx)
        .await?;

    if project_exists.is_none() {
        return Err(AppError::not_found("project"));
    }

    if let Some(parent_id) = input.parent_id.as_deref() {
        ensure_parent_exists(&mut tx, &input.project_id, parent_id).await?;
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
    .bind(input.parent_id.as_deref())
    .bind(&document_type)
    .bind(&title)
    .bind(target_order)
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

    reorder_document_in_parent(
        &mut tx,
        &input.project_id,
        &document_id,
        &input.parent_id,
        target_order,
    )
    .await?;

    upsert_document_search_index(
        &mut tx,
        &input.project_id,
        &document_id,
        &document_type,
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
        ORDER BY
          CASE WHEN parent_id IS NULL THEN 0 ELSE 1 END ASC,
          parent_id ASC,
          sort_order ASC,
          created_at ASC
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

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_document(pool, document_id).await
}

pub async fn move_document(
    pool: &SqlitePool,
    input: MoveDocumentInput,
) -> AppResult<Vec<DocumentDto>> {
    let target_order = input.sort_order.max(0);
    let now = now_ms();
    let mut tx = pool.begin().await?;

    let row = sqlx::query(
        r#"
        SELECT project_id, parent_id
        FROM documents
        WHERE id = ?1
        "#,
    )
    .bind(&input.document_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::not_found("document"))?;

    let project_id: String = row.try_get("project_id")?;
    let old_parent_id: Option<String> = row.try_get("parent_id")?;

    if let Some(parent_id) = input.parent_id.as_deref() {
        if parent_id == input.document_id {
            return Err(AppError::invalid_input("文档不能移动到自身下面"));
        }
        ensure_parent_exists(&mut tx, &project_id, parent_id).await?;
        if is_descendant(&mut tx, &input.document_id, parent_id).await? {
            return Err(AppError::invalid_input("文档不能移动到自己的子节点下面"));
        }
    }

    reorder_document_in_parent(
        &mut tx,
        &project_id,
        &input.document_id,
        &input.parent_id,
        target_order,
    )
    .await?;

    if !same_parent(&old_parent_id, &input.parent_id) {
        normalize_sibling_order(&mut tx, &project_id, &old_parent_id).await?;
    }

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    list_documents(pool, project_id).await
}

pub async fn update_document_status(
    pool: &SqlitePool,
    input: UpdateDocumentStatusInput,
) -> AppResult<DocumentDto> {
    let status = validate_status(&input.status)?;
    let now = now_ms();

    let mut tx = pool.begin().await?;

    let row = sqlx::query("SELECT project_id FROM documents WHERE id = ?1")
        .bind(&input.document_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::not_found("document"))?;

    let project_id: String = row.try_get("project_id")?;

    sqlx::query("UPDATE documents SET status = ?1, updated_at = ?2 WHERE id = ?3")
        .bind(&status)
        .bind(now)
        .bind(&input.document_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_document(pool, input.document_id).await
}

pub async fn delete_document(pool: &SqlitePool, document_id: String) -> AppResult<bool> {
    let now = now_ms();
    let mut tx = pool.begin().await?;

    let row = sqlx::query("SELECT project_id, parent_id FROM documents WHERE id = ?1")
        .bind(&document_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::not_found("document"))?;

    let project_id: String = row.try_get("project_id")?;
    let parent_id: Option<String> = row.try_get("parent_id")?;
    let ids = collect_document_descendants(&mut tx, &document_id).await?;

    for id in &ids {
        sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type IN ('volume', 'chapter', 'scene', 'note')")
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }

    let result = sqlx::query("DELETE FROM documents WHERE id = ?1")
        .bind(&document_id)
        .execute(&mut *tx)
        .await?;

    normalize_sibling_order(&mut tx, &project_id, &parent_id).await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(result.rows_affected() > 0)
}

async fn ensure_parent_exists(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    parent_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM documents WHERE id = ?1 AND project_id = ?2")
            .bind(parent_id)
            .bind(project_id)
            .fetch_optional(&mut **tx)
            .await?;

    if exists.is_none() {
        return Err(AppError::not_found("parent_document"));
    }

    Ok(())
}

async fn is_descendant(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ancestor_id: &str,
    possible_descendant_id: &str,
) -> AppResult<bool> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        WITH RECURSIVE descendants(id) AS (
          SELECT id FROM documents WHERE parent_id = ?1
          UNION ALL
          SELECT d.id FROM documents d
          INNER JOIN descendants ds ON d.parent_id = ds.id
        )
        SELECT id FROM descendants WHERE id = ?2 LIMIT 1
        "#,
    )
    .bind(ancestor_id)
    .bind(possible_descendant_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.is_some())
}

async fn collect_document_descendants(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    document_id: &str,
) -> AppResult<Vec<String>> {
    let rows = sqlx::query(
        r#"
        WITH RECURSIVE subtree(id) AS (
          SELECT id FROM documents WHERE id = ?1
          UNION ALL
          SELECT d.id FROM documents d
          INNER JOIN subtree s ON d.parent_id = s.id
        )
        SELECT id FROM subtree
        "#,
    )
    .bind(document_id)
    .fetch_all(&mut **tx)
    .await?;

    let mut ids = Vec::with_capacity(rows.len());
    for row in rows {
        ids.push(row.try_get("id")?);
    }
    Ok(ids)
}

async fn reorder_document_in_parent(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    document_id: &str,
    parent_id: &Option<String>,
    target_order: i64,
) -> AppResult<()> {
    let rows = sqlx::query(
        r#"
        SELECT id
        FROM documents
        WHERE project_id = ?1
          AND id != ?2
          AND ((?3 IS NULL AND parent_id IS NULL) OR parent_id = ?3)
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .bind(project_id)
    .bind(document_id)
    .bind(parent_id.as_deref())
    .fetch_all(&mut **tx)
    .await?;

    let mut ids: Vec<String> = Vec::with_capacity(rows.len() + 1);
    for row in rows {
        ids.push(row.try_get("id")?);
    }

    let insert_at = target_order.clamp(0, ids.len() as i64) as usize;
    ids.insert(insert_at, document_id.to_string());
    let now = now_ms();

    for (index, id) in ids.iter().enumerate() {
        sqlx::query(
            r#"
            UPDATE documents
            SET parent_id = ?1,
                sort_order = ?2,
                updated_at = CASE WHEN id = ?3 THEN ?4 ELSE updated_at END
            WHERE id = ?3
            "#,
        )
        .bind(parent_id.as_deref())
        .bind(index as i64)
        .bind(id)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

async fn normalize_sibling_order(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    parent_id: &Option<String>,
) -> AppResult<()> {
    let rows = sqlx::query(
        r#"
        SELECT id
        FROM documents
        WHERE project_id = ?1
          AND ((?2 IS NULL AND parent_id IS NULL) OR parent_id = ?2)
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .bind(project_id)
    .bind(parent_id.as_deref())
    .fetch_all(&mut **tx)
    .await?;

    for (index, row) in rows.iter().enumerate() {
        let id: String = row.try_get("id")?;
        sqlx::query("UPDATE documents SET sort_order = ?1 WHERE id = ?2")
            .bind(index as i64)
            .bind(id)
            .execute(&mut **tx)
            .await?;
    }

    Ok(())
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

async fn next_sort_order(
    pool: &SqlitePool,
    project_id: &str,
    parent_id: &Option<String>,
) -> AppResult<i64> {
    let row = sqlx::query(
        r#"
        SELECT COALESCE(MAX(sort_order), -1) + 1 AS next_order
        FROM documents
        WHERE project_id = ?1
          AND ((?2 IS NULL AND parent_id IS NULL) OR parent_id = ?2)
        "#,
    )
    .bind(project_id)
    .bind(parent_id.as_deref())
    .fetch_one(pool)
    .await?;

    Ok(row.try_get("next_order")?)
}
