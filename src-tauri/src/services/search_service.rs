use sqlx::{Row, SqlitePool};

use crate::{
    errors::{AppError, AppResult},
    models::search::{SearchProjectInput, SearchResultDto},
};

fn normalize_limit(limit: Option<i64>) -> i64 {
    limit.unwrap_or(50).clamp(1, 200)
}

fn scope_enabled(scopes: &[String], scope: &str) -> i64 {
    if scopes.is_empty()
        || scopes.iter().any(|value| value == "all")
        || scopes.iter().any(|value| value == scope)
    {
        1
    } else {
        0
    }
}

pub async fn search_project(
    pool: &SqlitePool,
    input: SearchProjectInput,
) -> AppResult<Vec<SearchResultDto>> {
    let trimmed_query = input.query.trim();
    if trimmed_query.is_empty() {
        return Err(AppError::invalid_input("搜索关键词不能为空"));
    }
    let limit = normalize_limit(input.limit);
    let include_documents = scope_enabled(&input.scopes, "documents");
    let include_cards = scope_enabled(&input.scopes, "cards");
    let include_notes = scope_enabled(&input.scopes, "notes");
    let include_outline = scope_enabled(&input.scopes, "outline");
    let include_timeline = scope_enabled(&input.scopes, "timeline");
    let include_foreshadow = scope_enabled(&input.scopes, "foreshadow");
    let include_proofreading = scope_enabled(&input.scopes, "proofreading");
    let include_export_templates = scope_enabled(&input.scopes, "exportTemplates");

    let like_query = format!("%{}%", trimmed_query);
    let rows = sqlx::query_as::<_, SearchResultDto>(
        r#"
        SELECT
          target_type,
          target_id,
          project_id,
          title,
          CASE
            WHEN body LIKE ?6 THEN substr(body, 1, 180)
            WHEN title LIKE ?6 THEN title
            ELSE substr(body, 1, 180)
          END AS snippet
        FROM search_index
        WHERE project_id = ?1
          AND (title LIKE ?6 OR body LIKE ?6)
          AND (
            (?2 = 1 AND target_type IN ('volume', 'chapter', 'scene', 'document'))
            OR (?3 = 1 AND target_type = 'card')
            OR (?4 = 1 AND target_type = 'note')
            OR (?7 = 1 AND target_type = 'outline')
            OR (?8 = 1 AND target_type = 'timeline')
            OR (?9 = 1 AND target_type = 'foreshadow')
            OR (?10 = 1 AND target_type = 'proofreadingRule')
            OR (?11 = 1 AND target_type = 'exportTemplate')
          )
        ORDER BY target_type ASC, title ASC
        LIMIT ?5
        "#,
    )
    .bind(input.project_id)
    .bind(include_documents)
    .bind(include_cards)
    .bind(include_notes)
    .bind(limit)
    .bind(like_query)
    .bind(include_outline)
    .bind(include_timeline)
    .bind(include_foreshadow)
    .bind(include_proofreading)
    .bind(include_export_templates)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn rebuild_search_index(pool: &SqlitePool, project_id: String) -> AppResult<bool> {
    let mut tx = pool.begin().await?;

    let project_exists: Option<(String,)> = sqlx::query_as("SELECT id FROM projects WHERE id = ?1")
        .bind(&project_id)
        .fetch_optional(&mut *tx)
        .await?;

    if project_exists.is_none() {
        return Err(AppError::not_found("project"));
    }

    sqlx::query("DELETE FROM search_index WHERE project_id = ?1")
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    let document_rows = sqlx::query(
        r#"
        SELECT
          d.id,
          d.type,
          d.title,
          COALESCE(c.content_text, '') AS content_text
        FROM documents d
        LEFT JOIN document_contents c ON c.document_id = d.id
        WHERE d.project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in document_rows {
        let id: String = row.try_get("id")?;
        let document_type: String = row.try_get("type")?;
        let title: String = row.try_get("title")?;
        let content_text: String = row.try_get("content_text")?;
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(document_type)
        .bind(id)
        .bind(&project_id)
        .bind(title)
        .bind(content_text)
        .execute(&mut *tx)
        .await?;
    }

    let card_rows = sqlx::query(
        r#"
        SELECT id, type, name, aliases_json, description, fields_json
        FROM cards
        WHERE project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in card_rows {
        let id: String = row.try_get("id")?;
        let card_type: String = row.try_get("type")?;
        let name: String = row.try_get("name")?;
        let aliases_json: String = row.try_get("aliases_json")?;
        let description: String = row.try_get("description")?;
        let fields_json: String = row.try_get("fields_json")?;
        let body = format!(
            "{}\n{}\n{}\n{}",
            card_type, aliases_json, description, fields_json
        );
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES ('card', ?1, ?2, ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(&project_id)
        .bind(name)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    let note_rows = sqlx::query(
        r#"
        SELECT id, type, title, body, status, priority
        FROM notes
        WHERE project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in note_rows {
        let id: String = row.try_get("id")?;
        let note_type: String = row.try_get("type")?;
        let title: String = row.try_get("title")?;
        let body_text: String = row.try_get("body")?;
        let status: String = row.try_get("status")?;
        let priority: String = row.try_get("priority")?;
        let body = format!("{}\n{}\n{}\n{}", note_type, status, priority, body_text);
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES ('note', ?1, ?2, ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(&project_id)
        .bind(title)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    let outline_rows = sqlx::query(
        r#"
        SELECT id, title, type, summary, status, metadata_json
        FROM outline_nodes
        WHERE project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in outline_rows {
        let id: String = row.try_get("id")?;
        let title: String = row.try_get("title")?;
        let node_type: String = row.try_get("type")?;
        let summary: String = row.try_get("summary")?;
        let status: String = row.try_get("status")?;
        let metadata_json: String = row.try_get("metadata_json")?;
        let body = format!("{}\n{}\n{}\n{}", node_type, status, summary, metadata_json);
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES ('outline', ?1, ?2, ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(&project_id)
        .bind(title)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    let timeline_rows = sqlx::query(
        r#"
        SELECT id, title, description, starts_at_label, ends_at_label, metadata_json
        FROM timeline_events
        WHERE project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in timeline_rows {
        let id: String = row.try_get("id")?;
        let title: String = row.try_get("title")?;
        let description: String = row.try_get("description")?;
        let starts_at_label: Option<String> = row.try_get("starts_at_label")?;
        let ends_at_label: Option<String> = row.try_get("ends_at_label")?;
        let metadata_json: String = row.try_get("metadata_json")?;
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
        .bind(id)
        .bind(&project_id)
        .bind(title)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    let foreshadow_rows = sqlx::query(
        r#"
        SELECT id, title, description, status, priority
        FROM foreshadow_threads
        WHERE project_id = ?1
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in foreshadow_rows {
        let id: String = row.try_get("id")?;
        let title: String = row.try_get("title")?;
        let description: String = row.try_get("description")?;
        let status: String = row.try_get("status")?;
        let priority: String = row.try_get("priority")?;
        let body = format!("{}\n{}\n{}", status, priority, description);
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES ('foreshadow', ?1, ?2, ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(&project_id)
        .bind(title)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    let template_rows = sqlx::query(
        r#"
        SELECT id, name, format, config_json
        FROM export_templates
        WHERE project_id = ?1 OR project_id IS NULL
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in template_rows {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let format: String = row.try_get("format")?;
        let config_json: String = row.try_get("config_json")?;
        let body = format!("{}\n{}", format, config_json);
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES ('exportTemplate', ?1, ?2, ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(&project_id)
        .bind(name)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    let proofreading_rows = sqlx::query(
        r#"
        SELECT id, name, type, pattern, config_json, severity
        FROM proofreading_rules
        WHERE project_id = ?1 OR project_id IS NULL
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in proofreading_rows {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let rule_type: String = row.try_get("type")?;
        let pattern: Option<String> = row.try_get("pattern")?;
        let config_json: String = row.try_get("config_json")?;
        let severity: String = row.try_get("severity")?;
        let body = format!(
            "{}\n{}\n{}\n{}",
            rule_type,
            severity,
            pattern.unwrap_or_default(),
            config_json
        );
        sqlx::query(
            r#"
            INSERT INTO search_index (target_type, target_id, project_id, title, body)
            VALUES ('proofreadingRule', ?1, ?2, ?3, ?4)
            "#,
        )
        .bind(id)
        .bind(&project_id)
        .bind(name)
        .bind(body)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(true)
}
