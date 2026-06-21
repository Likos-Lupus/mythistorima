use tauri::State;

use crate::{
    errors::AppResult,
    models::document::{
        CreateDocumentInput, DocumentContentDto, DocumentDto, MoveDocumentInput,
        UpdateDocumentContentInput, UpdateDocumentGoalInput, UpdateDocumentStatusInput,
    },
    services::document_service,
    AppState,
};

#[tauri::command]
pub async fn create_document(
    input: CreateDocumentInput,
    state: State<'_, AppState>,
) -> AppResult<DocumentDto> {
    document_service::create_document(&state.db, input).await
}

#[tauri::command]
pub async fn list_documents(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<DocumentDto>> {
    document_service::list_documents(&state.db, project_id).await
}

#[tauri::command]
pub async fn get_document_tree(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<DocumentDto>> {
    document_service::list_documents(&state.db, project_id).await
}

#[tauri::command]
pub async fn get_document_content(
    document_id: String,
    state: State<'_, AppState>,
) -> AppResult<DocumentContentDto> {
    document_service::get_document_content(&state.db, document_id).await
}

#[tauri::command]
pub async fn update_document_content(
    input: UpdateDocumentContentInput,
    state: State<'_, AppState>,
) -> AppResult<DocumentContentDto> {
    document_service::update_document_content(&state.db, input).await
}

#[tauri::command]
pub async fn rename_document(
    document_id: String,
    title: String,
    state: State<'_, AppState>,
) -> AppResult<DocumentDto> {
    document_service::rename_document(&state.db, document_id, title).await
}

#[tauri::command]
pub async fn move_document(
    input: MoveDocumentInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<DocumentDto>> {
    document_service::move_document(&state.db, input).await
}

#[tauri::command]
pub async fn update_document_goal(
    input: UpdateDocumentGoalInput,
    state: State<'_, AppState>,
) -> AppResult<DocumentDto> {
    document_service::update_document_goal(&state.db, input).await
}

#[tauri::command]
pub async fn update_document_status(
    input: UpdateDocumentStatusInput,
    state: State<'_, AppState>,
) -> AppResult<DocumentDto> {
    document_service::update_document_status(&state.db, input).await
}

#[tauri::command]
pub async fn delete_document(document_id: String, state: State<'_, AppState>) -> AppResult<bool> {
    document_service::delete_document(&state.db, document_id).await
}
