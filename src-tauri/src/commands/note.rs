use tauri::State;

use crate::{
    errors::AppResult,
    models::note::{CreateNoteInput, ListNotesInput, NoteDto, UpdateNoteInput},
    services::note_service,
    AppState,
};

#[tauri::command]
pub async fn create_note(input: CreateNoteInput, state: State<'_, AppState>) -> AppResult<NoteDto> {
    note_service::create_note(&state.db, input).await
}

#[tauri::command]
pub async fn update_note(input: UpdateNoteInput, state: State<'_, AppState>) -> AppResult<NoteDto> {
    note_service::update_note(&state.db, input).await
}

#[tauri::command]
pub async fn delete_note(note_id: String, state: State<'_, AppState>) -> AppResult<bool> {
    note_service::delete_note(&state.db, note_id).await
}

#[tauri::command]
pub async fn get_note(note_id: String, state: State<'_, AppState>) -> AppResult<NoteDto> {
    note_service::get_note(&state.db, note_id).await
}

#[tauri::command]
pub async fn list_notes(
    input: ListNotesInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<NoteDto>> {
    note_service::list_notes(&state.db, input).await
}

#[tauri::command]
pub async fn update_note_status(
    note_id: String,
    status: String,
    state: State<'_, AppState>,
) -> AppResult<NoteDto> {
    note_service::update_note_status(&state.db, note_id, status).await
}
