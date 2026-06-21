mod commands;
mod db;
mod errors;
mod models;
mod services;

use std::path::PathBuf;

use sqlx::SqlitePool;
use tauri::Manager;

pub struct AppState {
    pub db: SqlitePool,
    pub database_path: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let database = tauri::async_runtime::block_on(async move {
                db::connection::init_database(&handle).await
            })?;

            app.manage(AppState {
                db: database.pool,
                database_path: database.path,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_ping,
            commands::app::db_health_check,
            commands::app::get_app_info,
            commands::app::list_schema_migrations,
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::get_project,
            commands::project::delete_project,
            commands::document::create_document,
            commands::document::list_documents,
            commands::document::get_document_content,
            commands::document::update_document_content,
            commands::document::rename_document,
            commands::document::delete_document,
            commands::stats::get_project_stats,
            commands::stats::get_document_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
