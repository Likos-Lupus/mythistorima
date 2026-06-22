use chrono::Utc;
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::timeline::{
        AttachTimelineEventCardInput, ReorderTimelineEventInput, TimelineEventCardDto,
        TimelineEventDto, TimelineEventInput, UpdateTimelineEventInput,
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

fn normalize_optional_label(
    value: Option<String>,
    label: &str,
    max_len: usize,
) -> AppResult<Option<String>> {
    match value {
        Some(raw) => {
            let text = raw.trim().to_string();
            if text.is_empty() {
                return Ok(None);
            }
            if text.chars().count() > max_len {
                return Err(AppError::invalid_input(format!(
                    "{}不能超过 {} 个字符",
                    label, max_len
                )));
            }
            Ok(Some(text))
        }
        None => Ok(None),
    }
}

fn normalize_role(value: Option<String>) -> AppResult<String> {
    let role = value
        .unwrap_or_else(|| "participant".to_string())
        .trim()
        .to_string();
    if role.is_empty() {
        return Ok("participant".to_string());
    }
    if role.chars().count() > 64 {
        return Err(AppError::invalid_input("参与角色说明不能超过 64 个字符"));
    }
    Ok(role)
}

fn normalize_metadata_json(raw: Option<String>) -> AppResult<String> {
    match raw {
        Some(value) if !value.trim().is_empty() => {
            let parsed = serde_json::from_str::<Value>(&value).map_err(|error| {
                AppError::with_detail(
                    "INVALID_TIMELINE_METADATA_JSON",
                    "时间线 metadata_json 无法解析",
                    error.to_string(),
                )
            })?;
            if !parsed.is_object() {
                return Err(AppError::invalid_input("metadata_json 必须是 JSON 对象"));
            }
            serde_json::to_string(&parsed).map_err(|error| {
                AppError::with_detail(
                    "INVALID_TIMELINE_METADATA_JSON",
                    "时间线 metadata_json 无法序列化",
                    error.to_string(),
                )
            })
        }
        _ => Ok("{}".to_string()),
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

async fn ensure_card_in_project(
    pool: &SqlitePool,
    project_id: &str,
    card_id: &str,
) -> AppResult<()> {
    let exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM cards WHERE id = ?1 AND project_id = ?2 LIMIT 1")
            .bind(card_id)
            .bind(project_id)
            .fetch_optional(pool)
            .await?;

    if exists.is_none() {
        return Err(AppError::invalid_input("设定卡必须属于当前项目"));
    }

    Ok(())
}

async fn ensure_location_card_in_project(
    pool: &SqlitePool,
    project_id: &str,
    card_id: &str,
) -> AppResult<()> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT type FROM cards WHERE id = ?1 AND project_id = ?2 LIMIT 1")
            .bind(card_id)
            .bind(project_id)
            .fetch_optional(pool)
            .await?;

    match row {
        Some((card_type,)) if card_type == "location" => Ok(()),
        Some(_) => Err(AppError::invalid_input("地点必须选择 location 类型设定卡")),
        None => Err(AppError::invalid_input("地点设定卡必须属于当前项目")),
    }
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

pub async fn create_timeline_event(
    pool: &SqlitePool,
    input: TimelineEventInput,
) -> AppResult<TimelineEventDto> {
    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    let title = normalize_non_empty(&input.title, "事件标题", 160)?;
    let description = normalize_optional_text(input.description, "事件说明", 3000)?;
    let starts_at_label = normalize_optional_label(input.starts_at_label, "显示开始时间", 120)?;
    let ends_at_label = normalize_optional_label(input.ends_at_label, "显示结束时间", 120)?;
    let linked_card_id = normalize_optional_id(input.linked_card_id);
    let linked_document_id = normalize_optional_id(input.linked_document_id);
    let location_card_id = normalize_optional_id(input.location_card_id);
    let metadata_json = normalize_metadata_json(input.metadata_json)?;
    let sort_key = input.sort_key.unwrap_or(0);
    let event_id = Uuid::new_v4().to_string();
    let now = now_ms();

    ensure_project_exists(pool, &project_id).await?;
    if let Some(card_id) = linked_card_id.as_deref() {
        ensure_card_in_project(pool, &project_id, card_id).await?;
    }
    if let Some(document_id) = linked_document_id.as_deref() {
        ensure_document_in_project(pool, &project_id, document_id).await?;
    }
    if let Some(card_id) = location_card_id.as_deref() {
        ensure_location_card_in_project(pool, &project_id, card_id).await?;
    }

    sqlx::query(
        r#"
        INSERT INTO timeline_events
          (id, project_id, linked_card_id, linked_document_id, title, description, starts_at_label, ends_at_label, sort_key, location_card_id, metadata_json, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
        "#,
    )
    .bind(&event_id)
    .bind(&project_id)
    .bind(linked_card_id.as_deref())
    .bind(linked_document_id.as_deref())
    .bind(&title)
    .bind(&description)
    .bind(starts_at_label.as_deref())
    .bind(ends_at_label.as_deref())
    .bind(sort_key)
    .bind(location_card_id.as_deref())
    .bind(&metadata_json)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    touch_project(pool, &project_id, now).await?;
    upsert_timeline_search_index(
        pool,
        &project_id,
        &event_id,
        &title,
        &description,
        starts_at_label.as_deref(),
        ends_at_label.as_deref(),
        &metadata_json,
    )
    .await?;

    get_timeline_event(pool, event_id).await
}

pub async fn update_timeline_event(
    pool: &SqlitePool,
    input: UpdateTimelineEventInput,
) -> AppResult<TimelineEventDto> {
    let existing = get_timeline_event(pool, input.timeline_event_id.clone()).await?;
    let title = match input.title {
        Some(value) => normalize_non_empty(&value, "事件标题", 160)?,
        None => existing.title.clone(),
    };
    let description = match input.description {
        Some(value) => normalize_optional_text(Some(value), "事件说明", 3000)?,
        None => existing.description.clone(),
    };
    let starts_at_label = match input.starts_at_label {
        Some(value) => normalize_optional_label(value, "显示开始时间", 120)?,
        None => existing.starts_at_label.clone(),
    };
    let ends_at_label = match input.ends_at_label {
        Some(value) => normalize_optional_label(value, "显示结束时间", 120)?,
        None => existing.ends_at_label.clone(),
    };
    let linked_card_id = match input.linked_card_id {
        Some(value) => normalize_optional_id(value),
        None => existing.linked_card_id.clone(),
    };
    let linked_document_id = match input.linked_document_id {
        Some(value) => normalize_optional_id(value),
        None => existing.linked_document_id.clone(),
    };
    let location_card_id = match input.location_card_id {
        Some(value) => normalize_optional_id(value),
        None => existing.location_card_id.clone(),
    };
    let sort_key = input.sort_key.unwrap_or(existing.sort_key);
    let metadata_json = match input.metadata_json {
        Some(value) => normalize_metadata_json(Some(value))?,
        None => existing.metadata_json.clone(),
    };
    let now = now_ms();

    if let Some(card_id) = linked_card_id.as_deref() {
        ensure_card_in_project(pool, &existing.project_id, card_id).await?;
    }
    if let Some(document_id) = linked_document_id.as_deref() {
        ensure_document_in_project(pool, &existing.project_id, document_id).await?;
    }
    if let Some(card_id) = location_card_id.as_deref() {
        ensure_location_card_in_project(pool, &existing.project_id, card_id).await?;
    }

    sqlx::query(
        r#"
        UPDATE timeline_events
        SET linked_card_id = ?1,
            linked_document_id = ?2,
            title = ?3,
            description = ?4,
            starts_at_label = ?5,
            ends_at_label = ?6,
            sort_key = ?7,
            location_card_id = ?8,
            metadata_json = ?9,
            updated_at = ?10
        WHERE id = ?11
        "#,
    )
    .bind(linked_card_id.as_deref())
    .bind(linked_document_id.as_deref())
    .bind(&title)
    .bind(&description)
    .bind(starts_at_label.as_deref())
    .bind(ends_at_label.as_deref())
    .bind(sort_key)
    .bind(location_card_id.as_deref())
    .bind(&metadata_json)
    .bind(now)
    .bind(&input.timeline_event_id)
    .execute(pool)
    .await?;

    touch_project(pool, &existing.project_id, now).await?;
    upsert_timeline_search_index(
        pool,
        &existing.project_id,
        &input.timeline_event_id,
        &title,
        &description,
        starts_at_label.as_deref(),
        ends_at_label.as_deref(),
        &metadata_json,
    )
    .await?;

    get_timeline_event(pool, input.timeline_event_id).await
}

pub async fn delete_timeline_event(
    pool: &SqlitePool,
    timeline_event_id: String,
) -> AppResult<bool> {
    let existing = get_timeline_event(pool, timeline_event_id.clone()).await?;
    let now = now_ms();

    let result = sqlx::query("DELETE FROM timeline_events WHERE id = ?1")
        .bind(&timeline_event_id)
        .execute(pool)
        .await?;

    sqlx::query("DELETE FROM search_index WHERE target_type = 'timeline' AND target_id = ?1")
        .bind(&timeline_event_id)
        .execute(pool)
        .await?;

    touch_project(pool, &existing.project_id, now).await?;

    Ok(result.rows_affected() > 0)
}

pub async fn list_timeline_events(
    pool: &SqlitePool,
    project_id: String,
    card_id: Option<String>,
    location_card_id: Option<String>,
) -> AppResult<Vec<TimelineEventDto>> {
    ensure_project_exists(pool, &project_id).await?;
    let card_id = normalize_optional_id(card_id);
    let location_card_id = normalize_optional_id(location_card_id);

    if let Some(card_id) = card_id.as_deref() {
        ensure_card_in_project(pool, &project_id, card_id).await?;
    }
    if let Some(location_card_id) = location_card_id.as_deref() {
        ensure_location_card_in_project(pool, &project_id, location_card_id).await?;
    }

    match (card_id, location_card_id) {
        (Some(card_id), Some(location_card_id)) => sqlx::query_as::<_, TimelineEventDto>(
            r#"
            SELECT DISTINCT e.id, e.project_id, e.linked_card_id, e.linked_document_id, e.title, e.description,
                   e.starts_at_label, e.ends_at_label, e.sort_key, e.location_card_id, e.metadata_json,
                   e.created_at, e.updated_at
            FROM timeline_events e
            LEFT JOIN timeline_event_cards c ON c.timeline_event_id = e.id
            WHERE e.project_id = ?1
              AND e.location_card_id = ?2
              AND (e.linked_card_id = ?3 OR c.card_id = ?3)
            ORDER BY e.sort_key ASC, e.created_at ASC
            "#,
        )
        .bind(project_id)
        .bind(location_card_id)
        .bind(card_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from),
        (Some(card_id), None) => sqlx::query_as::<_, TimelineEventDto>(
            r#"
            SELECT DISTINCT e.id, e.project_id, e.linked_card_id, e.linked_document_id, e.title, e.description,
                   e.starts_at_label, e.ends_at_label, e.sort_key, e.location_card_id, e.metadata_json,
                   e.created_at, e.updated_at
            FROM timeline_events e
            LEFT JOIN timeline_event_cards c ON c.timeline_event_id = e.id
            WHERE e.project_id = ?1
              AND (e.linked_card_id = ?2 OR c.card_id = ?2)
            ORDER BY e.sort_key ASC, e.created_at ASC
            "#,
        )
        .bind(project_id)
        .bind(card_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from),
        (None, Some(location_card_id)) => sqlx::query_as::<_, TimelineEventDto>(
            r#"
            SELECT id, project_id, linked_card_id, linked_document_id, title, description,
                   starts_at_label, ends_at_label, sort_key, location_card_id, metadata_json,
                   created_at, updated_at
            FROM timeline_events
            WHERE project_id = ?1 AND location_card_id = ?2
            ORDER BY sort_key ASC, created_at ASC
            "#,
        )
        .bind(project_id)
        .bind(location_card_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from),
        (None, None) => sqlx::query_as::<_, TimelineEventDto>(
            r#"
            SELECT id, project_id, linked_card_id, linked_document_id, title, description,
                   starts_at_label, ends_at_label, sort_key, location_card_id, metadata_json,
                   created_at, updated_at
            FROM timeline_events
            WHERE project_id = ?1
            ORDER BY sort_key ASC, created_at ASC
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from),
    }
}

pub async fn get_timeline_event(
    pool: &SqlitePool,
    timeline_event_id: String,
) -> AppResult<TimelineEventDto> {
    sqlx::query_as::<_, TimelineEventDto>(
        r#"
        SELECT id, project_id, linked_card_id, linked_document_id, title, description,
               starts_at_label, ends_at_label, sort_key, location_card_id, metadata_json,
               created_at, updated_at
        FROM timeline_events
        WHERE id = ?1
        "#,
    )
    .bind(timeline_event_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("timeline_event"))
}

pub async fn attach_card_to_timeline_event(
    pool: &SqlitePool,
    input: AttachTimelineEventCardInput,
) -> AppResult<TimelineEventCardDto> {
    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    let event_id = normalize_non_empty(&input.timeline_event_id, "时间线事件", 120)?;
    let card_id = normalize_non_empty(&input.card_id, "设定卡", 120)?;
    let role = normalize_role(input.role)?;
    let event = get_timeline_event(pool, event_id.clone()).await?;

    if event.project_id != project_id {
        return Err(AppError::invalid_input("时间线事件必须属于当前项目"));
    }
    ensure_card_in_project(pool, &project_id, &card_id).await?;

    if let Some(existing) = get_timeline_event_card_by_pair(pool, &event_id, &card_id).await? {
        sqlx::query("UPDATE timeline_event_cards SET role = ?1 WHERE id = ?2")
            .bind(&role)
            .bind(&existing.id)
            .execute(pool)
            .await?;
        touch_project(pool, &project_id, now_ms()).await?;
        return get_timeline_event_card(pool, existing.id).await;
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO timeline_event_cards (id, project_id, timeline_event_id, card_id, role)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(&id)
    .bind(&project_id)
    .bind(&event_id)
    .bind(&card_id)
    .bind(&role)
    .execute(pool)
    .await?;

    touch_project(pool, &project_id, now_ms()).await?;
    get_timeline_event_card(pool, id).await
}

pub async fn detach_card_from_timeline_event(
    pool: &SqlitePool,
    timeline_event_id: String,
    card_id: String,
) -> AppResult<bool> {
    let event = get_timeline_event(pool, timeline_event_id.clone()).await?;
    let result = sqlx::query(
        "DELETE FROM timeline_event_cards WHERE timeline_event_id = ?1 AND card_id = ?2",
    )
    .bind(&timeline_event_id)
    .bind(&card_id)
    .execute(pool)
    .await?;
    touch_project(pool, &event.project_id, now_ms()).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn list_timeline_event_cards(
    pool: &SqlitePool,
    project_id: String,
    timeline_event_id: Option<String>,
) -> AppResult<Vec<TimelineEventCardDto>> {
    ensure_project_exists(pool, &project_id).await?;
    let timeline_event_id = normalize_optional_id(timeline_event_id);

    if let Some(event_id) = timeline_event_id {
        let event = get_timeline_event(pool, event_id.clone()).await?;
        if event.project_id != project_id {
            return Err(AppError::invalid_input("时间线事件必须属于当前项目"));
        }
        return sqlx::query_as::<_, TimelineEventCardDto>(
            r#"
            SELECT id, project_id, timeline_event_id, card_id, role
            FROM timeline_event_cards
            WHERE project_id = ?1 AND timeline_event_id = ?2
            ORDER BY role ASC, id ASC
            "#,
        )
        .bind(project_id)
        .bind(event_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from);
    }

    sqlx::query_as::<_, TimelineEventCardDto>(
        r#"
        SELECT id, project_id, timeline_event_id, card_id, role
        FROM timeline_event_cards
        WHERE project_id = ?1
        ORDER BY timeline_event_id ASC, role ASC, id ASC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

pub async fn reorder_timeline_event(
    pool: &SqlitePool,
    input: ReorderTimelineEventInput,
) -> AppResult<Vec<TimelineEventDto>> {
    let event = get_timeline_event(pool, input.timeline_event_id.clone()).await?;
    let now = now_ms();
    sqlx::query(
        r#"
        UPDATE timeline_events
        SET sort_key = ?1, updated_at = ?2
        WHERE id = ?3
        "#,
    )
    .bind(input.sort_key)
    .bind(now)
    .bind(&input.timeline_event_id)
    .execute(pool)
    .await?;

    touch_project(pool, &event.project_id, now).await?;
    list_timeline_events(pool, event.project_id, None, None).await
}

async fn get_timeline_event_card(pool: &SqlitePool, id: String) -> AppResult<TimelineEventCardDto> {
    sqlx::query_as::<_, TimelineEventCardDto>(
        r#"
        SELECT id, project_id, timeline_event_id, card_id, role
        FROM timeline_event_cards
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("timeline_event_card"))
}

async fn get_timeline_event_card_by_pair(
    pool: &SqlitePool,
    timeline_event_id: &str,
    card_id: &str,
) -> AppResult<Option<TimelineEventCardDto>> {
    sqlx::query_as::<_, TimelineEventCardDto>(
        r#"
        SELECT id, project_id, timeline_event_id, card_id, role
        FROM timeline_event_cards
        WHERE timeline_event_id = ?1 AND card_id = ?2
        LIMIT 1
        "#,
    )
    .bind(timeline_event_id)
    .bind(card_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
}

async fn touch_project(pool: &SqlitePool, project_id: &str, now: i64) -> AppResult<()> {
    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(project_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn upsert_timeline_search_index(
    pool: &SqlitePool,
    project_id: &str,
    event_id: &str,
    title: &str,
    description: &str,
    starts_at_label: Option<&str>,
    ends_at_label: Option<&str>,
    metadata_json: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_type = 'timeline' AND target_id = ?1")
        .bind(event_id)
        .execute(pool)
        .await?;

    let body = format!(
        "{}\n{}\n{}\n{}",
        starts_at_label.unwrap_or_default(),
        ends_at_label.unwrap_or_default(),
        description,
        metadata_json
    );
    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES ('timeline', ?1, ?2, ?3, ?4)
        "#,
    )
    .bind(event_id)
    .bind(project_id)
    .bind(title)
    .bind(body)
    .execute(pool)
    .await?;

    Ok(())
}
