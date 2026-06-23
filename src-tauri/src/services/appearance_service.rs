use std::collections::HashSet;

use chrono::Utc;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::appearance::{
        CardAppearanceDto, CardAppearanceSummaryDto, DocumentAppearanceCellDto,
        DocumentAppearanceDocumentDto, ProjectAppearanceSummaryDto,
    },
};

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
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

pub async fn rebuild_document_appearance_stats_tx(
    tx: &mut Transaction<'_, Sqlite>,
    project_id: &str,
    document_id: &str,
    now: i64,
) -> AppResult<()> {
    sqlx::query("DELETE FROM appearance_stats WHERE document_id = ?1")
        .bind(document_id)
        .execute(&mut **tx)
        .await?;

    let rows = sqlx::query(
        r#"
        SELECT
          card_id,
          COUNT(id) AS mention_count,
          MIN(from_pos) AS first_seen_position
        FROM card_references
        WHERE project_id = ?1
          AND document_id = ?2
        GROUP BY card_id
        "#,
    )
    .bind(project_id)
    .bind(document_id)
    .fetch_all(&mut **tx)
    .await?;

    for row in rows {
        let card_id: String = row.try_get("card_id")?;
        let mention_count: i64 = row.try_get("mention_count")?;
        let first_seen_position: Option<i64> = row.try_get("first_seen_position")?;

        if mention_count <= 0 {
            continue;
        }

        sqlx::query(
            r#"
            INSERT INTO appearance_stats
              (id, project_id, document_id, card_id, mention_count, first_seen_position, updated_at)
            VALUES
              (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(project_id)
        .bind(document_id)
        .bind(&card_id)
        .bind(mention_count)
        .bind(first_seen_position)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

pub async fn rebuild_appearance_stats(
    pool: &SqlitePool,
    project_id: String,
) -> AppResult<ProjectAppearanceSummaryDto> {
    ensure_project_exists(pool, &project_id).await?;
    let now = now_ms();
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM appearance_stats WHERE project_id = ?1")
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    let rows = sqlx::query(
        r#"
        SELECT
          r.document_id,
          r.card_id,
          COUNT(r.id) AS mention_count,
          MIN(r.from_pos) AS first_seen_position
        FROM card_references r
        INNER JOIN documents d ON d.id = r.document_id AND d.project_id = r.project_id
        INNER JOIN cards c ON c.id = r.card_id AND c.project_id = r.project_id
        WHERE r.project_id = ?1
        GROUP BY r.document_id, r.card_id
        "#,
    )
    .bind(&project_id)
    .fetch_all(&mut *tx)
    .await?;

    for row in rows {
        let document_id: String = row.try_get("document_id")?;
        let card_id: String = row.try_get("card_id")?;
        let mention_count: i64 = row.try_get("mention_count")?;
        let first_seen_position: Option<i64> = row.try_get("first_seen_position")?;

        sqlx::query(
            r#"
            INSERT INTO appearance_stats
              (id, project_id, document_id, card_id, mention_count, first_seen_position, updated_at)
            VALUES
              (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&project_id)
        .bind(&document_id)
        .bind(&card_id)
        .bind(mention_count)
        .bind(first_seen_position)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query("UPDATE projects SET updated_at = ?1 WHERE id = ?2")
        .bind(now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    get_project_appearance_summary(pool, project_id).await
}

pub async fn list_card_appearances(
    pool: &SqlitePool,
    project_id: String,
    card_id: String,
) -> AppResult<Vec<CardAppearanceDto>> {
    ensure_project_exists(pool, &project_id).await?;

    let card_exists: Option<(String,)> =
        sqlx::query_as("SELECT id FROM cards WHERE id = ?1 AND project_id = ?2 LIMIT 1")
            .bind(&card_id)
            .bind(&project_id)
            .fetch_optional(pool)
            .await?;

    if card_exists.is_none() {
        return Err(AppError::not_found("card"));
    }

    let appearances = sqlx::query_as::<_, CardAppearanceDto>(
        r#"
        SELECT
          s.id,
          s.project_id,
          s.document_id,
          d.title AS document_title,
          d.type AS document_type,
          s.card_id,
          c.name AS card_name,
          c.type AS card_type,
          s.mention_count,
          s.first_seen_position,
          s.updated_at
        FROM appearance_stats s
        INNER JOIN documents d ON d.id = s.document_id
        INNER JOIN cards c ON c.id = s.card_id
        WHERE s.project_id = ?1
          AND s.card_id = ?2
        ORDER BY d.sort_order ASC, d.created_at ASC, s.first_seen_position ASC
        "#,
    )
    .bind(project_id)
    .bind(card_id)
    .fetch_all(pool)
    .await?;

    Ok(appearances)
}

pub async fn get_project_appearance_summary(
    pool: &SqlitePool,
    project_id: String,
) -> AppResult<ProjectAppearanceSummaryDto> {
    ensure_project_exists(pool, &project_id).await?;

    let totals = sqlx::query(
        r#"
        SELECT
          (SELECT COUNT(id) FROM cards WHERE project_id = ?1 AND type = 'character') AS total_cards,
          (SELECT COUNT(DISTINCT card_id) FROM appearance_stats s INNER JOIN cards c ON c.id = s.card_id WHERE s.project_id = ?1 AND c.type = 'character') AS appeared_cards,
          (SELECT COUNT(id) FROM documents WHERE project_id = ?1 AND type IN ('chapter', 'scene')) AS total_documents,
          (SELECT COUNT(DISTINCT s.document_id) FROM appearance_stats s INNER JOIN cards c ON c.id = s.card_id WHERE s.project_id = ?1 AND c.type = 'character') AS documents_with_appearances,
          (SELECT COALESCE(SUM(s.mention_count), 0) FROM appearance_stats s INNER JOIN cards c ON c.id = s.card_id WHERE s.project_id = ?1 AND c.type = 'character') AS total_mentions
        "#,
    )
    .bind(&project_id)
    .fetch_one(pool)
    .await?;

    let cards = sqlx::query_as::<_, CardAppearanceSummaryDto>(
        r#"
        WITH ranked AS (
          SELECT
            s.card_id,
            s.document_id,
            d.title AS document_title,
            ROW_NUMBER() OVER (
              PARTITION BY s.card_id
              ORDER BY d.sort_order ASC, d.created_at ASC, COALESCE(s.first_seen_position, 999999999) ASC
            ) AS rn
          FROM appearance_stats s
          INNER JOIN documents d ON d.id = s.document_id
          WHERE s.project_id = ?1
        )
        SELECT
          c.id AS card_id,
          c.name AS card_name,
          c.type AS card_type,
          COUNT(DISTINCT s.document_id) AS document_count,
          COALESCE(SUM(s.mention_count), 0) AS total_mentions,
          first.document_id AS first_document_id,
          first.document_title AS first_document_title
        FROM cards c
        LEFT JOIN appearance_stats s ON s.card_id = c.id AND s.project_id = c.project_id
        LEFT JOIN ranked first ON first.card_id = c.id AND first.rn = 1
        WHERE c.project_id = ?1
          AND c.type = 'character'
        GROUP BY c.id, c.name, c.type, first.document_id, first.document_title
        ORDER BY total_mentions DESC, document_count DESC, c.updated_at DESC
        "#,
    )
    .bind(&project_id)
    .fetch_all(pool)
    .await?;

    let documents = sqlx::query_as::<_, DocumentAppearanceDocumentDto>(
        r#"
        SELECT
          id AS document_id,
          title AS document_title,
          type AS document_type,
          sort_order,
          parent_id
        FROM documents
        WHERE project_id = ?1
          AND type IN ('chapter', 'scene')
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .bind(&project_id)
    .fetch_all(pool)
    .await?;

    let character_ids: HashSet<String> = cards.iter().map(|card| card.card_id.clone()).collect();
    let all_appearances = sqlx::query_as::<_, DocumentAppearanceCellDto>(
        r#"
        SELECT
          s.document_id,
          s.card_id,
          s.mention_count,
          s.first_seen_position
        FROM appearance_stats s
        INNER JOIN cards c ON c.id = s.card_id
        INNER JOIN documents d ON d.id = s.document_id
        WHERE s.project_id = ?1
          AND c.type = 'character'
          AND d.type IN ('chapter', 'scene')
        ORDER BY d.sort_order ASC, d.created_at ASC
        "#,
    )
    .bind(&project_id)
    .fetch_all(pool)
    .await?;

    let appearances = all_appearances
        .into_iter()
        .filter(|item| character_ids.contains(&item.card_id))
        .collect();

    Ok(ProjectAppearanceSummaryDto {
        project_id,
        total_cards: totals.try_get("total_cards")?,
        appeared_cards: totals.try_get("appeared_cards")?,
        total_documents: totals.try_get("total_documents")?,
        documents_with_appearances: totals.try_get("documents_with_appearances")?,
        total_mentions: totals.try_get("total_mentions")?,
        cards,
        documents,
        appearances,
    })
}
