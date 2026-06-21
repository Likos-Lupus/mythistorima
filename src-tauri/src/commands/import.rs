use tauri::State;

use crate::{
    errors::AppResult,
    models::import::{ImportResultDto, ImportTextFileInput},
    services::import_service,
    AppState,
};

#[tauri::command]
pub async fn import_text_file(
    state: State<'_, AppState>,
    input: ImportTextFileInput,
) -> AppResult<ImportResultDto> {
    import_service::import_text_file(&state.db, input).await
}
