use tauri::State;

use crate::{errors::AppResult, models::backup::BackupDto, services::backup_service, AppState};

#[tauri::command]
pub async fn create_backup(state: State<'_, AppState>, project_id: String) -> AppResult<BackupDto> {
    backup_service::create_backup(&state.database_path, project_id)
}

#[tauri::command]
pub async fn list_backups(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<Vec<BackupDto>> {
    backup_service::list_backups(&state.database_path, project_id)
}
