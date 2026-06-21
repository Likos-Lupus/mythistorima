use tauri::State;

use crate::{AppState, errors::AppResult, models::stats::AppInfoDto};

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
