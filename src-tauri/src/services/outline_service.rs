use chrono::Utc;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::outline::{
        CreateOutlineNodeInput, LinkOutlineNodeInput, MoveOutlineNodeInput, OutlineNodeDto,
        UpdateOutlineNodeInput,
    },
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn validate_title(title: &str) -> AppResult<String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("大纲标题不能为空"));
    }
    Ok(trimmed.to_string())
}

fn validate_node_type(node_type: &str) -> AppResult<String> {
    let normalized = node_type.trim();
    match normalized {
        "plot" | "conflict" | "twist" | "event" | "arc" | "theme" | "note" => {
            Ok(normalized.to_string())
        }
        _ => Err(AppError::invalid_input(
            "大纲类型必须是 plot / conflict / twist / event / arc / theme / note",
        )),
    }
}

fn validate_status(status: &str) -> AppResult<String> {
    let normalized = status.trim();
    match normalized {
        "planned" | "drafting" | "done" | "archived" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "大纲状态必须是 planned / drafting / done / archived",
        )),
    }
}

fn normalize_optional_id(value: Option<String>) -> Option<String> {
    value.and_then(|item| {
        let trimmed = item.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

pub async fn create_outline_node(
    pool: &SqlitePool,
    input: CreateOutlineNodeInput,
) -> AppResult<OutlineNodeDto> {
    let project_id = input.project_id.trim().to_string();
    if project_id.is_empty() {
        return Err(AppError::invalid_input("project_id 不能为空"));
    }

    let title = validate_title(&input.title)?;
    let node_type = validate_node_type(input.node_type.as_deref().unwrap_or("plot"))?;
    let status = validate_status(input.status.as_deref().unwrap_or("planned"))?;
    let summary = input.summary.unwrap_or_default().trim().to_string();
    let metadata_json = normalize_metadata_json(input.metadata_json)?;
    let parent_id = normalize_optional_id(input.parent_id);
    let linked_document_id = normalize_optional_id(input.linked_document_id);
    let node_id = Uuid::new_v4().to_string();
    let now = now_ms();
    let sort_order = match input.sort_order {
        Some(value) => value.max(0),
        None => next_sort_order(pool, &project_id, &parent_id).await?,
    };

    let mut tx = pool.begin().await?;
    ensure_project_exists(&mut tx, &project_id).await?;
    if let Some(parent) = parent_id.as_deref() {
        ensure_outline_parent_exists(&mut tx, &project_id, parent).await?;
    }
    if let Some(document_id) = linked_document_id.as_deref() {
        ensure_document_exists(&mut tx, &project_id, document_id).await?;
    }

    sqlx::query(
        r#"
        INSERT INTO outline_nodes
          (id, project_id, parent_id, linked_document_id, title, type, summary, status, sort_order,
           metadata_json, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
        "#,
    )
    .bind(&node_id)
    .bind(&project_id)
    .bind(parent_id.as_deref())
    .bind(linked_document_id.as_deref())
    .bind(&title)
    .bind(&node_type)
    .bind(&summary)
    .bind(&status)
    .bind(sort_order)
    .bind(&metadata_json)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    reorder_outline_in_parent(&mut tx, &project_id, &node_id, &parent_id, sort_order).await?;
    upsert_outline_search_index(&mut tx, &project_id, &node_id, &title, &summary).await?;
    touch_project(&mut tx, &project_id, now).await?;
    tx.commit().await?;

    get_outline_node(pool, node_id).await
}

pub async fn update_outline_node(
    pool: &SqlitePool,
    input: UpdateOutlineNodeInput,
) -> AppResult<OutlineNodeDto> {
    let existing = get_outline_node(pool, input.outline_node_id.clone()).await?;
    let title = match input.title {
        Some(value) => validate_title(&value)?,
        None => existing.title.clone(),
    };
    let node_type = match input.node_type {
        Some(value) => validate_node_type(&value)?,
        None => existing.node_type.clone(),
    };
    let summary = input
        .summary
        .unwrap_or(existing.summary.clone())
        .trim()
        .to_string();
    let status = match input.status {
        Some(value) => validate_status(&value)?,
        None => existing.status.clone(),
    };
    let metadata_json = match input.metadata_json {
        Some(value) => normalize_metadata_json(Some(value))?,
        None => existing.metadata_json.clone(),
    };
    let now = now_ms();

    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"
        UPDATE outline_nodes
        SET title = ?1,
            type = ?2,
            summary = ?3,
            status = ?4,
            metadata_json = ?5,
            updated_at = ?6
        WHERE id = ?7
        "#,
    )
    .bind(&title)
    .bind(&node_type)
    .bind(&summary)
    .bind(&status)
    .bind(&metadata_json)
    .bind(now)
    .bind(&input.outline_node_id)
    .execute(&mut *tx)
    .await?;

    upsert_outline_search_index(
        &mut tx,
        &existing.project_id,
        &input.outline_node_id,
        &title,
        &summary,
    )
    .await?;
    touch_project(&mut tx, &existing.project_id, now).await?;
    tx.commit().await?;

    get_outline_node(pool, input.outline_node_id).await
}

pub async fn delete_outline_node(pool: &SqlitePool, outline_node_id: String) -> AppResult<bool> {
    let existing = get_outline_node(pool, outline_node_id.clone()).await?;
    let deleting_ids = collect_outline_subtree(pool, &outline_node_id).await?;
    let now = now_ms();

    let mut tx = pool.begin().await?;
    for id in &deleting_ids {
        sqlx::query("DELETE FROM search_index WHERE target_type = 'outline' AND target_id = ?1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }

    sqlx::query("DELETE FROM outline_nodes WHERE id = ?1")
        .bind(&outline_node_id)
        .execute(&mut *tx)
        .await?;

    compact_outline_siblings(&mut tx, &existing.project_id, &existing.parent_id).await?;
    touch_project(&mut tx, &existing.project_id, now).await?;
    tx.commit().await?;

    Ok(true)
}

pub async fn list_outline_nodes(
    pool: &SqlitePool,
    project_id: String,
) -> AppResult<Vec<OutlineNodeDto>> {
    let nodes = sqlx::query_as::<_, OutlineNodeDto>(
        r#"
        SELECT
          id,
          project_id,
          parent_id,
          linked_document_id,
          title,
          type as node_type,
          summary,
          status,
          sort_order,
          metadata_json,
          created_at,
          updated_at
        FROM outline_nodes
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

    Ok(nodes)
}

pub async fn move_outline_node(
    pool: &SqlitePool,
    input: MoveOutlineNodeInput,
) -> AppResult<Vec<OutlineNodeDto>> {
    let node = get_outline_node(pool, input.outline_node_id.clone()).await?;
    let parent_id = normalize_optional_id(input.parent_id);
    let target_order = input.sort_order.max(0);
    let now = now_ms();

    let mut tx = pool.begin().await?;
    if let Some(parent) = parent_id.as_deref() {
        ensure_outline_parent_exists(&mut tx, &node.project_id, parent).await?;
        if parent == node.id {
            return Err(AppError::invalid_input("不能把大纲节点移动到自身下"));
        }
        if is_outline_descendant(&mut tx, &node.id, parent).await? {
            return Err(AppError::invalid_input(
                "不能把大纲节点移动到自己的子节点下",
            ));
        }
    }

    sqlx::query(
        r#"
        UPDATE outline_nodes
        SET parent_id = ?1,
            sort_order = ?2,
            updated_at = ?3
        WHERE id = ?4
        "#,
    )
    .bind(parent_id.as_deref())
    .bind(target_order)
    .bind(now)
    .bind(&node.id)
    .execute(&mut *tx)
    .await?;

    if node.parent_id.as_deref() != parent_id.as_deref() {
        compact_outline_siblings(&mut tx, &node.project_id, &node.parent_id).await?;
    }
    reorder_outline_in_parent(
        &mut tx,
        &node.project_id,
        &node.id,
        &parent_id,
        target_order,
    )
    .await?;
    touch_project(&mut tx, &node.project_id, now).await?;
    tx.commit().await?;

    list_outline_nodes(pool, node.project_id).await
}

pub async fn link_outline_node_to_document(
    pool: &SqlitePool,
    input: LinkOutlineNodeInput,
) -> AppResult<OutlineNodeDto> {
    let node = get_outline_node(pool, input.outline_node_id.clone()).await?;
    let now = now_ms();

    let mut tx = pool.begin().await?;
    ensure_document_exists(&mut tx, &node.project_id, &input.document_id).await?;
    sqlx::query(
        r#"
        UPDATE outline_nodes
        SET linked_document_id = ?1,
            updated_at = ?2
        WHERE id = ?3
        "#,
    )
    .bind(&input.document_id)
    .bind(now)
    .bind(&input.outline_node_id)
    .execute(&mut *tx)
    .await?;
    touch_project(&mut tx, &node.project_id, now).await?;
    tx.commit().await?;

    get_outline_node(pool, input.outline_node_id).await
}

pub async fn unlink_outline_node_document(
    pool: &SqlitePool,
    outline_node_id: String,
) -> AppResult<OutlineNodeDto> {
    let node = get_outline_node(pool, outline_node_id.clone()).await?;
    let now = now_ms();

    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"
        UPDATE outline_nodes
        SET linked_document_id = NULL,
            updated_at = ?1
        WHERE id = ?2
        "#,
    )
    .bind(now)
    .bind(&outline_node_id)
    .execute(&mut *tx)
    .await?;
    touch_project(&mut tx, &node.project_id, now).await?;
    tx.commit().await?;

    get_outline_node(pool, outline_node_id).await
}

pub async fn get_outline_node(
    pool: &SqlitePool,
    outline_node_id: String,
) -> AppResult<OutlineNodeDto> {
    sqlx::query_as::<_, OutlineNodeDto>(
        r#"
        SELECT
          id,
          project_id,
          parent_id,
          linked_document_id,
          title,
          type as node_type,
          summary,
          status,
          sort_order,
          metadata_json,
          created_at,
          updated_at
        FROM outline_nodes
        WHERE id = ?1
        "#,
    )
    .bind(outline_node_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("outline_node"))
}

async fn next_sort_order(
    pool: &SqlitePool,
    project_id: &str,
    parent_id: &Option<String>,
) -> AppResult<i64> {
    let row: (Option<i64>,) = if let Some(parent) = parent_id.as_deref() {
        sqlx::query_as(
            r#"
            SELECT MAX(sort_order)
            FROM outline_nodes
            WHERE project_id = ?1 AND parent_id = ?2
            "#,
        )
        .bind(project_id)
        .bind(parent)
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_as(
            r#"
            SELECT MAX(sort_order)
            FROM outline_nodes
            WHERE project_id = ?1 AND parent_id IS NULL
            "#,
        )
        .bind(project_id)
        .fetch_one(pool)
        .await?
    };

    Ok(row.0.unwrap_or(-1) + 1)
}

async fn ensure_project_exists(
    tx: &mut Transaction<'_, Sqlite>,
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

async fn ensure_outline_parent_exists(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    parent_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM outline_nodes WHERE id = ?1 AND project_id = ?2")
            .bind(parent_id)
            .bind(project_id)
            .fetch_optional(&mut **tx)
            .await?;

    if exists.is_none() {
        return Err(AppError::not_found("outline_parent"));
    }
    Ok(())
}

async fn ensure_document_exists(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    document_id: &str,
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

async fn reorder_outline_in_parent(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    moving_id: &str,
    parent_id: &Option<String>,
    target_order: i64,
) -> AppResult<()> {
    let rows = fetch_sibling_ids(tx, project_id, parent_id).await?;
    let mut ids: Vec<String> = rows.into_iter().filter(|id| id != moving_id).collect();
    let insert_at = target_order.clamp(0, ids.len() as i64) as usize;
    ids.insert(insert_at, moving_id.to_string());

    for (index, id) in ids.iter().enumerate() {
        sqlx::query("UPDATE outline_nodes SET sort_order = ?1 WHERE id = ?2")
            .bind(index as i64)
            .bind(id)
            .execute(&mut **tx)
            .await?;
    }

    Ok(())
}

async fn compact_outline_siblings(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    parent_id: &Option<String>,
) -> AppResult<()> {
    let ids = fetch_sibling_ids(tx, project_id, parent_id).await?;
    for (index, id) in ids.iter().enumerate() {
        sqlx::query("UPDATE outline_nodes SET sort_order = ?1 WHERE id = ?2")
            .bind(index as i64)
            .bind(id)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}

async fn fetch_sibling_ids(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    parent_id: &Option<String>,
) -> AppResult<Vec<String>> {
    let rows = if let Some(parent) = parent_id.as_deref() {
        sqlx::query(
            r#"
            SELECT id
            FROM outline_nodes
            WHERE project_id = ?1 AND parent_id = ?2
            ORDER BY sort_order ASC, created_at ASC
            "#,
        )
        .bind(project_id)
        .bind(parent)
        .fetch_all(&mut **tx)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id
            FROM outline_nodes
            WHERE project_id = ?1 AND parent_id IS NULL
            ORDER BY sort_order ASC, created_at ASC
            "#,
        )
        .bind(project_id)
        .fetch_all(&mut **tx)
        .await?
    };

    let mut ids = Vec::with_capacity(rows.len());
    for row in rows {
        ids.push(row.try_get::<String, _>("id")?);
    }
    Ok(ids)
}

async fn is_outline_descendant(
    tx: &mut Transaction<'_, Sqlite>,
    ancestor_id: &str,
    candidate_id: &str,
) -> AppResult<bool> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        WITH RECURSIVE descendants(id) AS (
          SELECT id FROM outline_nodes WHERE parent_id = ?1
          UNION ALL
          SELECT n.id FROM outline_nodes n
          INNER JOIN descendants d ON n.parent_id = d.id
        )
        SELECT id FROM descendants WHERE id = ?2 LIMIT 1
        "#,
    )
    .bind(ancestor_id)
    .bind(candidate_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.is_some())
}

async fn collect_outline_subtree(
    pool: &SqlitePool,
    outline_node_id: &str,
) -> AppResult<Vec<String>> {
    let rows = sqlx::query(
        r#"
        WITH RECURSIVE subtree(id) AS (
          SELECT id FROM outline_nodes WHERE id = ?1
          UNION ALL
          SELECT n.id FROM outline_nodes n
          INNER JOIN subtree s ON n.parent_id = s.id
        )
        SELECT id FROM subtree
        "#,
    )
    .bind(outline_node_id)
    .fetch_all(pool)
    .await?;

    let mut ids = Vec::with_capacity(rows.len());
    for row in rows {
        ids.push(row.try_get::<String, _>("id")?);
    }
    Ok(ids)
}

async fn upsert_outline_search_index(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    outline_node_id: &str,
    title: &str,
    summary: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_type = 'outline' AND target_id = ?1")
        .bind(outline_node_id)
        .execute(&mut **tx)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES ('outline', ?1, ?2, ?3, ?4)
        "#,
    )
    .bind(outline_node_id)
    .bind(project_id)
    .bind(title)
    .bind(summary)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn touch_project(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    now: i64,
) -> AppResult<()> {
    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(project_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

fn normalize_metadata_json(value: Option<String>) -> AppResult<String> {
    let raw = value.unwrap_or_else(|| "{}".to_string());
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok("{}".to_string());
    }
    serde_json::from_str::<serde_json::Value>(trimmed).map_err(|error| {
        AppError::with_detail(
            "INVALID_METADATA_JSON",
            "metadata_json 必须是合法 JSON",
            error.to_string(),
        )
    })?;
    Ok(trimmed.to_string())
}
