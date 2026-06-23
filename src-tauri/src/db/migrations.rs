use chrono::Utc;
use sqlx::{Row, SqlitePool};

const MIGRATION_0001_NAME: &str = "phase0_initial_local_writing_loop";
const MIGRATION_0002_NAME: &str = "phase1_workspace_foundation";
const MIGRATION_0003_NAME: &str = "phase2_advanced_authoring_foundation";

const MIGRATION_0001_STATEMENTS: &[&str] = &[
    r#"
PRAGMA foreign_keys = ON
    "#,
    r#"
CREATE TABLE IF NOT EXISTS projects (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  author TEXT,
  description TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS documents (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  parent_id TEXT,
  type TEXT NOT NULL,
  title TEXT NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'draft',
  character_count INTEGER NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (parent_id) REFERENCES documents(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS document_contents (
  document_id TEXT PRIMARY KEY,
  schema_version INTEGER NOT NULL DEFAULT 1,
  content_json TEXT NOT NULL,
  content_text TEXT NOT NULL DEFAULT '',
  content_html TEXT NOT NULL DEFAULT '',
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS writing_sessions (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  document_id TEXT NOT NULL,
  started_at INTEGER NOT NULL,
  ended_at INTEGER,
  characters_before INTEGER NOT NULL DEFAULT 0,
  characters_after INTEGER NOT NULL DEFAULT 0,
  inserted_count INTEGER NOT NULL DEFAULT 0,
  deleted_count INTEGER NOT NULL DEFAULT 0,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_documents_project_id
ON documents(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_documents_parent_id
ON documents(parent_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_writing_sessions_project_id
ON writing_sessions(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_writing_sessions_document_id
ON writing_sessions(document_id)
    "#,
];

const MIGRATION_0002_STATEMENTS: &[&str] = &[
    r#"
CREATE TABLE IF NOT EXISTS cards (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  type TEXT NOT NULL,
  name TEXT NOT NULL,
  aliases_json TEXT NOT NULL DEFAULT '[]',
  description TEXT NOT NULL DEFAULT '',
  fields_json TEXT NOT NULL DEFAULT '{}',
  avatar_asset_id TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_cards_project_type
ON cards(project_id, type)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_cards_project_name
ON cards(project_id, name)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS card_references (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  document_id TEXT NOT NULL,
  card_id TEXT NOT NULL,
  display_text TEXT NOT NULL,
  from_pos INTEGER,
  to_pos INTEGER,
  paragraph_id TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
  FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_card_references_card_id
ON card_references(card_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_card_references_document_id
ON card_references(document_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_card_references_project_document
ON card_references(project_id, document_id)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS notes (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  document_id TEXT,
  paragraph_id TEXT,
  type TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'open',
  priority TEXT NOT NULL DEFAULT 'normal',
  due_at INTEGER,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_notes_project_id
ON notes(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_notes_document_id
ON notes(document_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_notes_type
ON notes(project_id, type)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_notes_status
ON notes(project_id, status)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS tags (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  name TEXT NOT NULL,
  color TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE UNIQUE INDEX IF NOT EXISTS idx_tags_project_name
ON tags(project_id, name)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS taggings (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  target_type TEXT NOT NULL,
  target_id TEXT NOT NULL,
  created_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_taggings_project_target
ON taggings(project_id, target_type, target_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_taggings_tag_id
ON taggings(tag_id)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS app_settings (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL
)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS project_settings (
  project_id TEXT NOT NULL,
  key TEXT NOT NULL,
  value_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL,

  PRIMARY KEY (project_id, key),
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS assets (
  id TEXT PRIMARY KEY,
  project_id TEXT,
  type TEXT NOT NULL,
  filename TEXT NOT NULL,
  mime TEXT,
  path TEXT NOT NULL,
  hash TEXT,
  created_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_assets_project_id
ON assets(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_assets_type
ON assets(project_id, type)
    "#,
    r#"
CREATE VIRTUAL TABLE IF NOT EXISTS search_index
USING fts5(
  target_type,
  target_id,
  project_id UNINDEXED,
  title,
  body,
  tokenize = 'unicode61'
)
    "#,
];

const MIGRATION_0003_STATEMENTS: &[&str] = &[
    r#"
CREATE TABLE IF NOT EXISTS outline_nodes (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  parent_id TEXT,
  linked_document_id TEXT,
  title TEXT NOT NULL,
  type TEXT NOT NULL DEFAULT 'plot',
  summary TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'planned',
  sort_order INTEGER NOT NULL DEFAULT 0,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (parent_id) REFERENCES outline_nodes(id) ON DELETE CASCADE,
  FOREIGN KEY (linked_document_id) REFERENCES documents(id) ON DELETE SET NULL
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_outline_nodes_project_id
ON outline_nodes(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_outline_nodes_parent_id
ON outline_nodes(parent_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_outline_nodes_linked_document_id
ON outline_nodes(linked_document_id)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS card_relations (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  source_card_id TEXT NOT NULL,
  target_card_id TEXT NOT NULL,
  relation_type TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  direction TEXT NOT NULL DEFAULT 'directed',
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (source_card_id) REFERENCES cards(id) ON DELETE CASCADE,
  FOREIGN KEY (target_card_id) REFERENCES cards(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_card_relations_project_id
ON card_relations(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_card_relations_source
ON card_relations(source_card_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_card_relations_target
ON card_relations(target_card_id)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS timeline_events (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  linked_card_id TEXT,
  linked_document_id TEXT,
  title TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  starts_at_label TEXT,
  ends_at_label TEXT,
  sort_key INTEGER NOT NULL DEFAULT 0,
  location_card_id TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (linked_card_id) REFERENCES cards(id) ON DELETE SET NULL,
  FOREIGN KEY (linked_document_id) REFERENCES documents(id) ON DELETE SET NULL,
  FOREIGN KEY (location_card_id) REFERENCES cards(id) ON DELETE SET NULL
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_timeline_events_project_id
ON timeline_events(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_timeline_events_sort_key
ON timeline_events(project_id, sort_key)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS timeline_event_cards (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  timeline_event_id TEXT NOT NULL,
  card_id TEXT NOT NULL,
  role TEXT NOT NULL DEFAULT 'participant',

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (timeline_event_id) REFERENCES timeline_events(id) ON DELETE CASCADE,
  FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_timeline_event_cards_event
ON timeline_event_cards(timeline_event_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_timeline_event_cards_card
ON timeline_event_cards(card_id)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS foreshadow_threads (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'open',
  setup_note_id TEXT,
  payoff_note_id TEXT,
  setup_document_id TEXT,
  payoff_document_id TEXT,
  priority TEXT NOT NULL DEFAULT 'normal',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (setup_note_id) REFERENCES notes(id) ON DELETE SET NULL,
  FOREIGN KEY (payoff_note_id) REFERENCES notes(id) ON DELETE SET NULL,
  FOREIGN KEY (setup_document_id) REFERENCES documents(id) ON DELETE SET NULL,
  FOREIGN KEY (payoff_document_id) REFERENCES documents(id) ON DELETE SET NULL
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_foreshadow_threads_project_id
ON foreshadow_threads(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_foreshadow_threads_status
ON foreshadow_threads(project_id, status)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS appearance_stats (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  document_id TEXT NOT NULL,
  card_id TEXT NOT NULL,
  mention_count INTEGER NOT NULL DEFAULT 0,
  first_seen_position INTEGER,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
  FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE UNIQUE INDEX IF NOT EXISTS idx_appearance_stats_unique
ON appearance_stats(document_id, card_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_appearance_stats_project_card
ON appearance_stats(project_id, card_id)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS export_templates (
  id TEXT PRIMARY KEY,
  project_id TEXT,
  name TEXT NOT NULL,
  format TEXT NOT NULL,
  config_json TEXT NOT NULL DEFAULT '{}',
  is_builtin INTEGER NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_export_templates_project_id
ON export_templates(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_export_templates_format
ON export_templates(format)
    "#,
    r#"
CREATE TABLE IF NOT EXISTS proofreading_rules (
  id TEXT PRIMARY KEY,
  project_id TEXT,
  name TEXT NOT NULL,
  type TEXT NOT NULL,
  pattern TEXT,
  config_json TEXT NOT NULL DEFAULT '{}',
  severity TEXT NOT NULL DEFAULT 'warning',
  enabled INTEGER NOT NULL DEFAULT 1,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_proofreading_rules_project_id
ON proofreading_rules(project_id)
    "#,
    r#"
CREATE INDEX IF NOT EXISTS idx_proofreading_rules_type
ON proofreading_rules(project_id, type)
    "#,
];

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await?;

    ensure_migration_table(pool).await?;

    if !is_migration_applied(pool, 1).await? {
        execute_statements(pool, MIGRATION_0001_STATEMENTS).await?;
        record_migration(pool, 1, MIGRATION_0001_NAME).await?;
    }

    if !is_migration_applied(pool, 2).await? {
        run_migration_0002(pool).await?;
        record_migration(pool, 2, MIGRATION_0002_NAME).await?;
    }

    if !is_migration_applied(pool, 3).await? {
        run_migration_0003(pool).await?;
        record_migration(pool, 3, MIGRATION_0003_NAME).await?;
    }

    Ok(())
}

async fn ensure_migration_table(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
          version INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          applied_at INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn is_migration_applied(pool: &SqlitePool, version: i64) -> anyhow::Result<bool> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT version FROM schema_migrations WHERE version = ?1 LIMIT 1")
            .bind(version)
            .fetch_optional(pool)
            .await?;

    Ok(row.is_some())
}

async fn record_migration(pool: &SqlitePool, version: i64, name: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO schema_migrations (version, name, applied_at)
        VALUES (?1, ?2, ?3)
        "#,
    )
    .bind(version)
    .bind(name)
    .bind(now_ms())
    .execute(pool)
    .await?;

    Ok(())
}

async fn run_migration_0002(pool: &SqlitePool) -> anyhow::Result<()> {
    add_column_if_missing(
        pool,
        "PRAGMA table_info(projects)",
        "language",
        "ALTER TABLE projects ADD COLUMN language TEXT NOT NULL DEFAULT 'zh-CN'",
    )
    .await?;
    add_column_if_missing(
        pool,
        "PRAGMA table_info(projects)",
        "cover_asset_id",
        "ALTER TABLE projects ADD COLUMN cover_asset_id TEXT",
    )
    .await?;
    add_column_if_missing(
        pool,
        "PRAGMA table_info(projects)",
        "target_character_count",
        "ALTER TABLE projects ADD COLUMN target_character_count INTEGER",
    )
    .await?;
    add_column_if_missing(
        pool,
        "PRAGMA table_info(projects)",
        "daily_target_count",
        "ALTER TABLE projects ADD COLUMN daily_target_count INTEGER",
    )
    .await?;
    add_column_if_missing(
        pool,
        "PRAGMA table_info(projects)",
        "metadata_json",
        "ALTER TABLE projects ADD COLUMN metadata_json TEXT NOT NULL DEFAULT '{}'",
    )
    .await?;

    add_column_if_missing(
        pool,
        "PRAGMA table_info(documents)",
        "summary",
        "ALTER TABLE documents ADD COLUMN summary TEXT",
    )
    .await?;
    add_column_if_missing(
        pool,
        "PRAGMA table_info(documents)",
        "metadata_json",
        "ALTER TABLE documents ADD COLUMN metadata_json TEXT NOT NULL DEFAULT '{}'",
    )
    .await?;

    execute_statements(pool, MIGRATION_0002_STATEMENTS).await?;
    seed_search_index(pool).await?;
    seed_default_settings(pool).await?;

    Ok(())
}

async fn run_migration_0003(pool: &SqlitePool) -> anyhow::Result<()> {
    execute_statements(pool, MIGRATION_0003_STATEMENTS).await?;
    seed_phase2_defaults(pool).await?;
    Ok(())
}

async fn add_column_if_missing(
    pool: &SqlitePool,
    table_info_sql: &'static str,
    column: &str,
    alter_sql: &'static str,
) -> anyhow::Result<()> {
    if !column_exists(pool, table_info_sql, column).await? {
        sqlx::query(alter_sql).execute(pool).await?;
    }

    Ok(())
}

async fn column_exists(
    pool: &SqlitePool,
    table_info_sql: &'static str,
    column: &str,
) -> anyhow::Result<bool> {
    let rows = sqlx::query(table_info_sql).fetch_all(pool).await?;

    for row in rows {
        let name: String = row.try_get("name")?;
        if name == column {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn execute_statements(pool: &SqlitePool, statements: &[&'static str]) -> anyhow::Result<()> {
    for statement in statements {
        sqlx::query(*statement).execute(pool).await?;
    }

    Ok(())
}

async fn seed_search_index(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM search_index")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO search_index (target_type, target_id, project_id, title, body)
        SELECT
          d.type,
          d.id,
          d.project_id,
          d.title,
          COALESCE(c.content_text, '')
        FROM documents d
        LEFT JOIN document_contents c ON c.document_id = d.id
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_default_settings(pool: &SqlitePool) -> anyhow::Result<()> {
    let now = now_ms();
    let defaults = [
        ("theme", r#""paper""#),
        ("editor.fontSize", "18"),
        ("editor.lineHeight", "1.85"),
        ("editor.pageWidth", "820"),
        ("autosave.intervalMs", "1000"),
        ("language", r#""zh-CN""#),
    ];

    for (key, value_json) in defaults {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO app_settings (key, value_json, updated_at)
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(key)
        .bind(value_json)
        .bind(now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn seed_phase2_defaults(pool: &SqlitePool) -> anyhow::Result<()> {
    let now = now_ms();
    let templates = [
        (
            "builtin_txt_submission",
            "投稿 TXT 模板",
            "txt",
            r#"{"includeTitle":true,"includeAuthor":true,"includeChapterTitle":true,"paragraphSeparator":"\n\n"}"#,
        ),
        (
            "builtin_html_review",
            "审稿 HTML 模板",
            "html",
            r#"{"includeTitle":true,"includeAuthor":true,"includeChapterTitle":true,"fontFamily":"serif","lineHeight":1.8}"#,
        ),
        (
            "builtin_pixiv_basic",
            "Pixiv 基础模板",
            "pixiv",
            r#"{"includeTitle":true,"includeChapterTitle":true,"paragraphSeparator":"\n\n","pixivRuby":false}"#,
        ),
    ];

    for (id, name, format, config_json) in templates {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO export_templates
              (id, project_id, name, format, config_json, is_builtin, created_at, updated_at)
            VALUES (?1, NULL, ?2, ?3, ?4, 1, ?5, ?6)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(format)
        .bind(config_json)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
    }

    let rules = [
        (
            "builtin_duplicate_word",
            "重复词检查",
            "duplicate_word",
            None::<&str>,
            r#"{"maxUnitCharacters":4}"#,
            "warning",
        ),
        (
            "builtin_continuous_punctuation",
            "连续标点检查",
            "continuous_punctuation",
            Some(r#"[!！?？。…~、，,；;：:]{3,}"#),
            r#"{"minCount":3}"#,
            "warning",
        ),
        (
            "builtin_mixed_punctuation",
            "中英文标点混用检查",
            "mixed_punctuation",
            None::<&str>,
            r#"{"pairs":["，,","，.","。,","。.","！!","!！","？?","?？","：:",":：","；;",";；"]}"#,
            "info",
        ),
        (
            "builtin_long_sentence",
            "超长句检查",
            "long_sentence",
            None::<&str>,
            r#"{"maxCharacters":120}"#,
            "info",
        ),
        (
            "builtin_long_paragraph",
            "超长段落检查",
            "long_paragraph",
            None::<&str>,
            r#"{"maxCharacters":500}"#,
            "info",
        ),
        (
            "builtin_sensitive_word",
            "敏感词检查",
            "sensitive_word",
            Some("敏感词,违禁词"),
            r#"{"words":["敏感词","违禁词"]}"#,
            "warning",
        ),
        (
            "builtin_name_consistency",
            "设定名称一致性检查",
            "name_consistency",
            None::<&str>,
            r#"{"maxDistance":1,"minNameCharacters":2}"#,
            "warning",
        ),
    ];

    for (id, name, rule_type, pattern, config_json, severity) in rules {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO proofreading_rules
              (id, project_id, name, type, pattern, config_json, severity, enabled, created_at, updated_at)
            VALUES (?1, NULL, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?8)
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(rule_type)
        .bind(pattern)
        .bind(config_json)
        .bind(severity)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}
