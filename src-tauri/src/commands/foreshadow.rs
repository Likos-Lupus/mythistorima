use tauri::State;

use crate::{
    errors::AppResult,
    models::foreshadow::{
        ForeshadowThreadDto, ForeshadowThreadFromNoteInput, ForeshadowThreadInput,
        ListForeshadowThreadsInput, UpdateForeshadowThreadInput,
    },
    services::foreshadow_service,
    AppState,
};

#[tauri::command]
pub async fn create_foreshadow_thread(
    input: ForeshadowThreadInput,
    state: State<'_, AppState>,
) -> AppResult<ForeshadowThreadDto> {
    foreshadow_service::create_foreshadow_thread(&state.db, input).await
}

#[tauri::command]
pub async fn create_foreshadow_thread_from_note(
    input: ForeshadowThreadFromNoteInput,
    state: State<'_, AppState>,
) -> AppResult<ForeshadowThreadDto> {
    foreshadow_service::create_foreshadow_thread_from_note(&state.db, input).await
}

#[tauri::command]
pub async fn update_foreshadow_thread(
    input: UpdateForeshadowThreadInput,
    state: State<'_, AppState>,
) -> AppResult<ForeshadowThreadDto> {
    foreshadow_service::update_foreshadow_thread(&state.db, input).await
}

#[tauri::command]
pub async fn mark_foreshadow_paid_off(
    thread_id: String,
    payoff_document_id: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<ForeshadowThreadDto> {
    foreshadow_service::mark_foreshadow_paid_off(&state.db, thread_id, payoff_document_id).await
}

#[tauri::command]
pub async fn delete_foreshadow_thread(
    thread_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    foreshadow_service::delete_foreshadow_thread(&state.db, thread_id).await
}

#[tauri::command]
pub async fn list_foreshadow_threads(
    input: ListForeshadowThreadsInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<ForeshadowThreadDto>> {
    foreshadow_service::list_foreshadow_threads(&state.db, input).await
}
