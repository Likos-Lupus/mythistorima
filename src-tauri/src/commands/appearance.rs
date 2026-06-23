use tauri::State;

use crate::{
    errors::AppResult,
    models::appearance::{CardAppearanceDto, ProjectAppearanceSummaryDto},
    services::appearance_service,
    AppState,
};

#[tauri::command]
pub async fn rebuild_appearance_stats(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<ProjectAppearanceSummaryDto> {
    appearance_service::rebuild_appearance_stats(&state.db, project_id).await
}

#[tauri::command]
pub async fn list_card_appearances(
    project_id: String,
    card_id: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<CardAppearanceDto>> {
    appearance_service::list_card_appearances(&state.db, project_id, card_id).await
}

#[tauri::command]
pub async fn get_project_appearance_summary(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<ProjectAppearanceSummaryDto> {
    appearance_service::get_project_appearance_summary(&state.db, project_id).await
}
