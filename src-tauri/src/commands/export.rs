use tauri::State;

use crate::{
    errors::AppResult,
    models::export::{ExportDocumentsInput, ExportProjectInput, ExportResultDto},
    services::export_service,
    AppState,
};

#[tauri::command]
pub async fn export_documents(
    state: State<'_, AppState>,
    input: ExportDocumentsInput,
) -> AppResult<ExportResultDto> {
    export_service::export_documents(&state.db, &state.database_path, input).await
}

#[tauri::command]
pub async fn export_project(
    state: State<'_, AppState>,
    input: ExportProjectInput,
) -> AppResult<ExportResultDto> {
    export_service::export_project(&state.db, &state.database_path, input).await
}
