use chrono::Utc;
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::card::{CardDto, CardReferenceDto, CreateCardInput, UpdateCardInput},
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn validate_card_type(card_type: &str) -> AppResult<String> {
    let normalized = card_type.trim();
    match normalized {
        "character" | "location" | "concept" => Ok(normalized.to_string()),
        _ => Err(AppError::invalid_input(
            "设定卡类型必须是 character / location / concept",
        )),
    }
}

fn validate_name(name: &str) -> AppResult<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(AppError::invalid_input("设定卡名称不能为空"));
    }
    if trimmed.chars().count() > 120 {
        return Err(AppError::invalid_input("设定卡名称不能超过 120 个字符"));
    }
    Ok(trimmed.to_string())
}

fn normalize_description(description: Option<String>) -> String {
    description.unwrap_or_default().trim().to_string()
}

fn normalize_aliases_json(raw: Option<String>) -> AppResult<String> {
    match raw {
        Some(value) if !value.trim().is_empty() => {
            let parsed = serde_json::from_str::<Value>(&value).map_err(|error| {
                AppError::with_detail(
                    "INVALID_ALIASES_JSON",
                    "别名 JSON 无法解析",
                    error.to_string(),
                )
            })?;
            if !parsed.is_array() {
                return Err(AppError::invalid_input("aliases_json 必须是 JSON 数组"));
            }
            serde_json::to_string(&parsed).map_err(|error| {
                AppError::with_detail(
                    "INVALID_ALIASES_JSON",
                    "别名 JSON 无法序列化",
                    error.to_string(),
                )
            })
        }
        _ => Ok("[]".to_string()),
    }
}

fn normalize_fields_json(raw: Option<String>, card_type: &str) -> AppResult<String> {
    match raw {
        Some(value) if !value.trim().is_empty() => {
            let parsed = serde_json::from_str::<Value>(&value).map_err(|error| {
                AppError::with_detail(
                    "INVALID_FIELDS_JSON",
                    "字段 JSON 无法解析",
                    error.to_string(),
                )
            })?;
            if !parsed.is_object() {
                return Err(AppError::invalid_input("fields_json 必须是 JSON 对象"));
            }
            serde_json::to_string(&parsed).map_err(|error| {
                AppError::with_detail(
                    "INVALID_FIELDS_JSON",
                    "字段 JSON 无法序列化",
                    error.to_string(),
                )
            })
        }
        _ => Ok(default_fields_json(card_type)),
    }
}

fn default_fields_json(card_type: &str) -> String {
    match card_type {
        "character" => serde_json::json!({
            "role": "",
            "motivation": "",
            "notes": ""
        })
        .to_string(),
        "location" => serde_json::json!({
            "atmosphere": "",
            "notes": ""
        })
        .to_string(),
        "concept" => serde_json::json!({
            "rules": "",
            "limits": "",
            "notes": ""
        })
        .to_string(),
        _ => "{}".to_string(),
    }
}

fn card_type_label(card_type: &str) -> &'static str {
    match card_type {
        "character" => "人物",
        "location" => "地点",
        "concept" => "概念",
        _ => "设定",
    }
}

pub async fn create_card(pool: &SqlitePool, input: CreateCardInput) -> AppResult<CardDto> {
    let card_type = validate_card_type(&input.card_type)?;
    let name = validate_name(&input.name)?;
    let aliases_json = normalize_aliases_json(input.aliases_json)?;
    let description = normalize_description(input.description);
    let fields_json = normalize_fields_json(input.fields_json, &card_type)?;
    let card_id = Uuid::new_v4().to_string();
    let now = now_ms();

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
        INSERT INTO cards
          (id, project_id, type, name, aliases_json, description, fields_json, avatar_asset_id, created_at, updated_at)
        VALUES
          (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        "#,
    )
    .bind(&card_id)
    .bind(&input.project_id)
    .bind(&card_type)
    .bind(&name)
    .bind(&aliases_json)
    .bind(&description)
    .bind(&fields_json)
    .bind(input.avatar_asset_id.as_deref())
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    upsert_card_search_index(
        &mut tx,
        &input.project_id,
        &card_id,
        &card_type,
        &name,
        &aliases_json,
        &description,
        &fields_json,
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&input.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_card(pool, card_id).await
}

pub async fn update_card(pool: &SqlitePool, input: UpdateCardInput) -> AppResult<CardDto> {
    let existing = get_card(pool, input.card_id.clone()).await?;
    let card_type = match input.card_type {
        Some(value) => validate_card_type(&value)?,
        None => existing.card_type,
    };
    let name = match input.name {
        Some(value) => validate_name(&value)?,
        None => existing.name,
    };
    let aliases_json = match input.aliases_json {
        Some(value) => normalize_aliases_json(Some(value))?,
        None => existing.aliases_json,
    };
    let description = input
        .description
        .unwrap_or(existing.description)
        .trim()
        .to_string();
    let fields_json = match input.fields_json {
        Some(value) => normalize_fields_json(Some(value), &card_type)?,
        None => existing.fields_json,
    };
    let avatar_asset_id = input.avatar_asset_id.or(existing.avatar_asset_id);
    let now = now_ms();

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE cards
        SET type = ?1,
            name = ?2,
            aliases_json = ?3,
            description = ?4,
            fields_json = ?5,
            avatar_asset_id = ?6,
            updated_at = ?7
        WHERE id = ?8
        "#,
    )
    .bind(&card_type)
    .bind(&name)
    .bind(&aliases_json)
    .bind(&description)
    .bind(&fields_json)
    .bind(avatar_asset_id.as_deref())
    .bind(now)
    .bind(&input.card_id)
    .execute(&mut *tx)
    .await?;

    upsert_card_search_index(
        &mut tx,
        &existing.project_id,
        &input.card_id,
        &card_type,
        &name,
        &aliases_json,
        &description,
        &fields_json,
    )
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&existing.project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_card(pool, input.card_id).await
}

pub async fn delete_card(pool: &SqlitePool, card_id: String) -> AppResult<bool> {
    let existing = get_card(pool, card_id.clone()).await?;
    let now = now_ms();
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = 'card'")
        .bind(&card_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM card_references WHERE card_id = ?1")
        .bind(&card_id)
        .execute(&mut *tx)
        .await?;

    let result = sqlx::query("DELETE FROM cards WHERE id = ?1")
        .bind(&card_id)
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

pub async fn get_card(pool: &SqlitePool, card_id: String) -> AppResult<CardDto> {
    let card = sqlx::query_as::<_, CardDto>(
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
        WHERE id = ?1
        "#,
    )
    .bind(card_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::not_found("card"))?;

    Ok(card)
}

pub async fn list_cards(
    pool: &SqlitePool,
    project_id: String,
    card_type: Option<String>,
) -> AppResult<Vec<CardDto>> {
    if let Some(value) = card_type.as_deref().filter(|value| *value != "all") {
        let normalized = validate_card_type(value)?;
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
            WHERE project_id = ?1 AND type = ?2
            ORDER BY updated_at DESC, created_at DESC
            "#,
        )
        .bind(project_id)
        .bind(normalized)
        .fetch_all(pool)
        .await?;
        return Ok(cards);
    }

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
            WHEN 'concept' THEN 2
            ELSE 3
          END ASC,
          updated_at DESC,
          created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    Ok(cards)
}

pub async fn search_cards(
    pool: &SqlitePool,
    project_id: String,
    query: String,
) -> AppResult<Vec<CardDto>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return list_cards(pool, project_id, None).await;
    }

    let pattern = format!("%{}%", trimmed);
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
          AND (
            name LIKE ?2
            OR aliases_json LIKE ?2
            OR description LIKE ?2
            OR fields_json LIKE ?2
          )
        ORDER BY updated_at DESC, created_at DESC
        "#,
    )
    .bind(project_id)
    .bind(pattern)
    .fetch_all(pool)
    .await?;

    Ok(cards)
}

pub async fn list_card_references(
    pool: &SqlitePool,
    card_id: String,
) -> AppResult<Vec<CardReferenceDto>> {
    let references = sqlx::query_as::<_, CardReferenceDto>(
        r#"
        SELECT
          r.id,
          r.project_id,
          r.document_id,
          d.title as document_title,
          r.card_id,
          r.display_text,
          r.from_pos,
          r.to_pos,
          r.paragraph_id,
          r.created_at,
          r.updated_at
        FROM card_references r
        LEFT JOIN documents d ON d.id = r.document_id
        WHERE r.card_id = ?1
        ORDER BY r.created_at DESC
        "#,
    )
    .bind(card_id)
    .fetch_all(pool)
    .await?;

    Ok(references)
}

async fn upsert_card_search_index(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    project_id: &str,
    card_id: &str,
    card_type: &str,
    name: &str,
    aliases_json: &str,
    description: &str,
    fields_json: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM search_index WHERE target_id = ?1 AND target_type = 'card'")
        .bind(card_id)
        .execute(&mut **tx)
        .await?;

    let search_body = format!(
        "{}\n{}\n{}\n{}",
        card_type_label(card_type),
        aliases_json,
        description,
        fields_json
    );

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        VALUES ('card', ?1, ?2, ?3, ?4)
        "#,
    )
    .bind(card_id)
    .bind(project_id)
    .bind(name)
    .bind(search_body)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
