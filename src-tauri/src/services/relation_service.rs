use chrono::Utc;
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::{
        card::CardDto,
        relation::{CardGraphDto, CardRelationDto, CardRelationInput, UpdateCardRelationInput},
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

fn normalize_optional_text(value: Option<String>, max_len: usize) -> AppResult<String> {
    let text = value.unwrap_or_default().trim().to_string();
    if text.chars().count() > max_len {
        return Err(AppError::invalid_input(format!(
            "关系说明不能超过 {} 个字符",
            max_len
        )));
    }
    Ok(text)
}

fn validate_direction(value: Option<String>) -> AppResult<String> {
    let normalized = value
        .unwrap_or_else(|| "directed".to_string())
        .trim()
        .to_string();
    match normalized.as_str() {
        "directed" | "undirected" | "bidirectional" => Ok(normalized),
        _ => Err(AppError::invalid_input(
            "关系方向必须是 directed / undirected / bidirectional",
        )),
    }
}

fn normalize_metadata_json(raw: Option<String>) -> AppResult<String> {
    match raw {
        Some(value) if !value.trim().is_empty() => {
            let parsed = serde_json::from_str::<Value>(&value).map_err(|error| {
                AppError::with_detail(
                    "INVALID_RELATION_METADATA_JSON",
                    "关系 metadata_json 无法解析",
                    error.to_string(),
                )
            })?;
            if !parsed.is_object() {
                return Err(AppError::invalid_input("metadata_json 必须是 JSON 对象"));
            }
            serde_json::to_string(&parsed).map_err(|error| {
                AppError::with_detail(
                    "INVALID_RELATION_METADATA_JSON",
                    "关系 metadata_json 无法序列化",
                    error.to_string(),
                )
            })
        }
        _ => Ok("{}".to_string()),
    }
}

async fn ensure_project_exists(pool: &SqlitePool, project_id: &str) -> AppResult<()> {
    let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1")
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
        return Err(AppError::invalid_input("关系两端必须是当前项目内的设定卡"));
    }

    Ok(())
}

async fn refresh_relation_search_index(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    relation_id: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_type = 'relation' AND target_id = ?1")
        .bind(relation_id)
        .execute(&mut **tx)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        SELECT
          'relation',
          r.id,
          r.project_id,
          source.name || ' · ' || r.relation_type || ' · ' || target.name,
          source.name || char(10) ||
          target.name || char(10) ||
          r.direction || char(10) ||
          r.description || char(10) ||
          r.metadata_json
        FROM card_relations r
        JOIN cards source ON source.id = r.source_card_id
        JOIN cards target ON target.id = r.target_card_id
        WHERE r.id = ?1
        "#,
    )
    .bind(relation_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_card_relation(
    pool: &SqlitePool,
    input: CardRelationInput,
) -> AppResult<CardRelationDto> {
    let project_id = normalize_non_empty(&input.project_id, "项目 ID", 120)?;
    let source_card_id = normalize_non_empty(&input.source_card_id, "来源设定卡", 120)?;
    let target_card_id = normalize_non_empty(&input.target_card_id, "目标设定卡", 120)?;
    if source_card_id == target_card_id {
        return Err(AppError::invalid_input("关系两端不能是同一张设定卡"));
    }
    let relation_type = normalize_non_empty(&input.relation_type, "关系类型", 64)?;
    let description = normalize_optional_text(input.description, 1000)?;
    let direction = validate_direction(input.direction)?;
    let metadata_json = normalize_metadata_json(input.metadata_json)?;
    let relation_id = Uuid::new_v4().to_string();
    let now = now_ms();

    ensure_project_exists(pool, &project_id).await?;
    ensure_card_in_project(pool, &project_id, &source_card_id).await?;
    ensure_card_in_project(pool, &project_id, &target_card_id).await?;

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO card_relations
          (id, project_id, source_card_id, target_card_id, relation_type, description, direction, metadata_json, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
    )
    .bind(&relation_id)
    .bind(&project_id)
    .bind(&source_card_id)
    .bind(&target_card_id)
    .bind(&relation_type)
    .bind(&description)
    .bind(&direction)
    .bind(&metadata_json)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    refresh_relation_search_index(&mut tx, &relation_id).await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_card_relation(pool, relation_id).await
}

pub async fn update_card_relation(
    pool: &SqlitePool,
    input: UpdateCardRelationInput,
) -> AppResult<CardRelationDto> {
    let existing = get_card_relation(pool, input.relation_id.clone()).await?;
    let source_card_id = match input.source_card_id {
        Some(value) => normalize_non_empty(&value, "来源设定卡", 120)?,
        None => existing.source_card_id.clone(),
    };
    let target_card_id = match input.target_card_id {
        Some(value) => normalize_non_empty(&value, "目标设定卡", 120)?,
        None => existing.target_card_id.clone(),
    };
    if source_card_id == target_card_id {
        return Err(AppError::invalid_input("关系两端不能是同一张设定卡"));
    }
    let relation_type = match input.relation_type {
        Some(value) => normalize_non_empty(&value, "关系类型", 64)?,
        None => existing.relation_type.clone(),
    };
    let description = match input.description {
        Some(value) => normalize_optional_text(Some(value), 1000)?,
        None => existing.description.clone(),
    };
    let direction = match input.direction {
        Some(value) => validate_direction(Some(value))?,
        None => existing.direction.clone(),
    };
    let metadata_json = match input.metadata_json {
        Some(value) => normalize_metadata_json(Some(value))?,
        None => existing.metadata_json.clone(),
    };
    let now = now_ms();

    ensure_card_in_project(pool, &existing.project_id, &source_card_id).await?;
    ensure_card_in_project(pool, &existing.project_id, &target_card_id).await?;

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE card_relations
        SET source_card_id = ?1,
            target_card_id = ?2,
            relation_type = ?3,
            description = ?4,
            direction = ?5,
            metadata_json = ?6,
            updated_at = ?7
        WHERE id = ?8
        "#,
    )
    .bind(&source_card_id)
    .bind(&target_card_id)
    .bind(&relation_type)
    .bind(&description)
    .bind(&direction)
    .bind(&metadata_json)
    .bind(now)
    .bind(&input.relation_id)
    .execute(&mut *tx)
    .await?;

    refresh_relation_search_index(&mut tx, &input.relation_id).await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&existing.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_card_relation(pool, input.relation_id).await
}

pub async fn delete_card_relation(pool: &SqlitePool, relation_id: String) -> AppResult<bool> {
    let existing = get_card_relation(pool, relation_id.clone()).await?;
    let now = now_ms();
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM search_index WHERE target_type = 'relation' AND target_id = ?1")
        .bind(&relation_id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM card_relations WHERE id = ?1")
        .bind(&relation_id)
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

pub async fn get_card_relation(
    pool: &SqlitePool,
    relation_id: String,
) -> AppResult<CardRelationDto> {
    sqlx::query_as::<_, CardRelationDto>(
        r#"
        SELECT
          id,
          project_id,
          source_card_id,
          target_card_id,
          relation_type,
          description,
          direction,
          metadata_json,
          created_at,
          updated_at
        FROM card_relations
        WHERE id = ?1
        "#,
    )
    .bind(relation_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("card_relation"))
}

pub async fn list_card_relations(
    pool: &SqlitePool,
    project_id: String,
    card_id: Option<String>,
    relation_type: Option<String>,
) -> AppResult<Vec<CardRelationDto>> {
    ensure_project_exists(pool, &project_id).await?;
    let normalized_relation_type = relation_type
        .as_deref()
        .filter(|value| !value.trim().is_empty() && value.trim() != "all")
        .map(|value| normalize_non_empty(value, "关系类型", 64))
        .transpose()?;
    let normalized_card_id = card_id
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .map(|value| normalize_non_empty(value, "设定卡", 120))
        .transpose()?;

    match (normalized_card_id, normalized_relation_type) {
        (Some(card_id), Some(relation_type)) => {
            sqlx::query_as::<_, CardRelationDto>(RELATION_SELECT_FILTER_CARD_TYPE)
                .bind(project_id)
                .bind(card_id)
                .bind(relation_type)
                .fetch_all(pool)
                .await
                .map_err(AppError::from)
        }
        (Some(card_id), None) => sqlx::query_as::<_, CardRelationDto>(RELATION_SELECT_FILTER_CARD)
            .bind(project_id)
            .bind(card_id)
            .fetch_all(pool)
            .await
            .map_err(AppError::from),
        (None, Some(relation_type)) => {
            sqlx::query_as::<_, CardRelationDto>(RELATION_SELECT_FILTER_TYPE)
                .bind(project_id)
                .bind(relation_type)
                .fetch_all(pool)
                .await
                .map_err(AppError::from)
        }
        (None, None) => sqlx::query_as::<_, CardRelationDto>(RELATION_SELECT_ALL)
            .bind(project_id)
            .fetch_all(pool)
            .await
            .map_err(AppError::from),
    }
}

pub async fn list_card_graph(pool: &SqlitePool, project_id: String) -> AppResult<CardGraphDto> {
    ensure_project_exists(pool, &project_id).await?;

    let cards = sqlx::query_as::<_, CardDto>(
        r#"
        SELECT
          id,
          project_id,
          type as card_type,
          name,
          aliases_json,
          description,
          fields_json,
          avatar_asset_id,
          created_at,
          updated_at
        FROM cards
        WHERE project_id = ?1
        ORDER BY
          CASE type
            WHEN 'character' THEN 0
            WHEN 'location' THEN 1
            WHEN 'organization' THEN 2
            WHEN 'item' THEN 3
            WHEN 'event' THEN 4
            WHEN 'concept' THEN 5
            ELSE 6
          END ASC,
          updated_at DESC,
          created_at DESC
        "#,
    )
    .bind(&project_id)
    .fetch_all(pool)
    .await?;

    let relations = list_card_relations(pool, project_id, None, None).await?;

    Ok(CardGraphDto { cards, relations })
}

const RELATION_SELECT_ALL: &str = r#"
SELECT
  id,
  project_id,
  source_card_id,
  target_card_id,
  relation_type,
  description,
  direction,
  metadata_json,
  created_at,
  updated_at
FROM card_relations
WHERE project_id = ?1
ORDER BY updated_at DESC, created_at DESC
"#;

const RELATION_SELECT_FILTER_CARD: &str = r#"
SELECT
  id,
  project_id,
  source_card_id,
  target_card_id,
  relation_type,
  description,
  direction,
  metadata_json,
  created_at,
  updated_at
FROM card_relations
WHERE project_id = ?1
  AND (source_card_id = ?2 OR target_card_id = ?2)
ORDER BY updated_at DESC, created_at DESC
"#;

const RELATION_SELECT_FILTER_TYPE: &str = r#"
SELECT
  id,
  project_id,
  source_card_id,
  target_card_id,
  relation_type,
  description,
  direction,
  metadata_json,
  created_at,
  updated_at
FROM card_relations
WHERE project_id = ?1
  AND relation_type = ?2
ORDER BY updated_at DESC, created_at DESC
"#;

const RELATION_SELECT_FILTER_CARD_TYPE: &str = r#"
SELECT
  id,
  project_id,
  source_card_id,
  target_card_id,
  relation_type,
  description,
  direction,
  metadata_json,
  created_at,
  updated_at
FROM card_relations
WHERE project_id = ?1
  AND (source_card_id = ?2 OR target_card_id = ?2)
  AND relation_type = ?3
ORDER BY updated_at DESC, created_at DESC
"#;
