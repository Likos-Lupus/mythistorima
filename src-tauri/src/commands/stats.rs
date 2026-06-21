use tauri::State;

use crate::{
    AppState,
    errors::AppResult,
    models::stats::{DocumentStatsDto, ProjectStatsDto},
    services::stats_service,
};

#[tauri::command]
pub async fn get_project_stats(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<ProjectStatsDto> {
    stats_service::get_project_stats(&state.db, project_id).await
}

#[tauri::command]
pub async fn get_document_stats(
    document_id: String,
    state: State<'_, AppState>,
) -> AppResult<DocumentStatsDto> {
    stats_service::get_document_stats(&state.db, document_id).await
}
