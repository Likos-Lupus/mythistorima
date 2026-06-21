use tauri::State;

use crate::{
    errors::AppResult,
    models::stats::{AppInfoDto, SchemaMigrationDto},
    AppState,
};

#[tauri::command]
pub async fn app_ping() -> AppResult<String> {
    Ok("Rust 已连接".to_string())
}

#[tauri::command]
pub async fn db_health_check(state: State<'_, AppState>) -> AppResult<bool> {
    sqlx::query("SELECT 1").execute(&state.db).await?;

    Ok(true)
}

#[tauri::command]
pub async fn get_app_info(state: State<'_, AppState>) -> AppResult<AppInfoDto> {
    Ok(AppInfoDto {
        name: "Mythistorima".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_path: state.database_path.display().to_string(),
    })
}

#[tauri::command]
pub async fn list_schema_migrations(
    state: State<'_, AppState>,
) -> AppResult<Vec<SchemaMigrationDto>> {
    let migrations = sqlx::query_as::<_, SchemaMigrationDto>(
        r#"
        SELECT version, name, applied_at
        FROM schema_migrations
        ORDER BY version ASC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(migrations)
}
