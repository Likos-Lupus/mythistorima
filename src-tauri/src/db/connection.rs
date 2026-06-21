use std::path::PathBuf;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use tauri::{AppHandle, Manager};

use super::migrations;

pub struct DatabaseHandle {
    pub pool: SqlitePool,
    pub path: PathBuf,
}

pub async fn init_database(app: &AppHandle) -> anyhow::Result<DatabaseHandle> {
    let app_data_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;

    let database_path = app_data_dir.join("mythistorima.sqlite");
    let options = SqliteConnectOptions::new()
        .filename(&database_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    migrations::run_migrations(&pool).await?;

    Ok(DatabaseHandle {
        pool,
        path: database_path,
    })
}
