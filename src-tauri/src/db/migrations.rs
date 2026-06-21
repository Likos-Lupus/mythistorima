use sqlx::SqlitePool;

const MIGRATION_0001: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS projects (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  author TEXT,
  description TEXT,
  status TEXT NOT NULL DEFAULT 'active',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

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
);

CREATE TABLE IF NOT EXISTS document_contents (
  document_id TEXT PRIMARY KEY,
  schema_version INTEGER NOT NULL DEFAULT 1,
  content_json TEXT NOT NULL,
  content_text TEXT NOT NULL DEFAULT '',
  content_html TEXT NOT NULL DEFAULT '',
  updated_at INTEGER NOT NULL,

  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

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
);

CREATE INDEX IF NOT EXISTS idx_documents_project_id
ON documents(project_id);

CREATE INDEX IF NOT EXISTS idx_documents_parent_id
ON documents(parent_id);

CREATE INDEX IF NOT EXISTS idx_writing_sessions_project_id
ON writing_sessions(project_id);

CREATE INDEX IF NOT EXISTS idx_writing_sessions_document_id
ON writing_sessions(document_id);
"#;

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    for statement in MIGRATION_0001.split(';') {
        let statement = statement.trim();
        if !statement.is_empty() {
            sqlx::query(statement).execute(pool).await?;
        }
    }

    Ok(())
}
