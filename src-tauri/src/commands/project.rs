use tauri::State;

use crate::{
    AppState,
    errors::AppResult,
    models::project::{CreateProjectInput, ProjectDto},
    services::project_service,
};

#[tauri::command]
pub async fn create_project(
    input: CreateProjectInput,
    state: State<'_, AppState>,
) -> AppResult<ProjectDto> {
    project_service::create_project(&state.db, input).await
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> AppResult<Vec<ProjectDto>> {
    project_service::list_projects(&state.db).await
}

#[tauri::command]
pub async fn get_project(project_id: String, state: State<'_, AppState>) -> AppResult<ProjectDto> {
    project_service::get_project(&state.db, project_id).await
}

#[tauri::command]
pub async fn delete_project(project_id: String, state: State<'_, AppState>) -> AppResult<bool> {
    project_service::delete_project(&state.db, project_id).await
}
