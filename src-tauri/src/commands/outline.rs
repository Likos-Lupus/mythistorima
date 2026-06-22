use tauri::State;

use crate::{
    errors::AppResult,
    models::outline::{
        CreateOutlineNodeInput, LinkOutlineNodeInput, MoveOutlineNodeInput, OutlineNodeDto,
        UpdateOutlineNodeInput,
    },
    services::outline_service,
    AppState,
};

#[tauri::command]
pub async fn create_outline_node(
    input: CreateOutlineNodeInput,
    state: State<'_, AppState>,
) -> AppResult<OutlineNodeDto> {
    outline_service::create_outline_node(&state.db, input).await
}

#[tauri::command]
pub async fn update_outline_node(
    input: UpdateOutlineNodeInput,
    state: State<'_, AppState>,
) -> AppResult<OutlineNodeDto> {
    outline_service::update_outline_node(&state.db, input).await
}

#[tauri::command]
pub async fn delete_outline_node(
    outline_node_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    outline_service::delete_outline_node(&state.db, outline_node_id).await
}

#[tauri::command]
pub async fn list_outline_nodes(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<OutlineNodeDto>> {
    outline_service::list_outline_nodes(&state.db, project_id).await
}

#[tauri::command]
pub async fn move_outline_node(
    input: MoveOutlineNodeInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<OutlineNodeDto>> {
    outline_service::move_outline_node(&state.db, input).await
}

#[tauri::command]
pub async fn link_outline_node_to_document(
    input: LinkOutlineNodeInput,
    state: State<'_, AppState>,
) -> AppResult<OutlineNodeDto> {
    outline_service::link_outline_node_to_document(&state.db, input).await
}

#[tauri::command]
pub async fn unlink_outline_node_document(
    outline_node_id: String,
    state: State<'_, AppState>,
) -> AppResult<OutlineNodeDto> {
    outline_service::unlink_outline_node_document(&state.db, outline_node_id).await
}
