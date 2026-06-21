use tauri::State;

use crate::{
    errors::AppResult,
    models::search::{SearchProjectInput, SearchResultDto},
    services::search_service,
    AppState,
};

#[tauri::command]
pub async fn search_project(
    state: State<'_, AppState>,
    input: SearchProjectInput,
) -> AppResult<Vec<SearchResultDto>> {
    search_service::search_project(&state.db, input).await
}

#[tauri::command]
pub async fn rebuild_search_index(
    state: State<'_, AppState>,
    project_id: String,
) -> AppResult<bool> {
    search_service::rebuild_search_index(&state.db, project_id).await
}
