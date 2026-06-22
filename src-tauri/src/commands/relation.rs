use tauri::State;

use crate::{
    errors::AppResult,
    models::relation::{CardGraphDto, CardRelationDto, CardRelationInput, UpdateCardRelationInput},
    services::relation_service,
    AppState,
};

#[tauri::command]
pub async fn create_card_relation(
    input: CardRelationInput,
    state: State<'_, AppState>,
) -> AppResult<CardRelationDto> {
    relation_service::create_card_relation(&state.db, input).await
}

#[tauri::command]
pub async fn update_card_relation(
    input: UpdateCardRelationInput,
    state: State<'_, AppState>,
) -> AppResult<CardRelationDto> {
    relation_service::update_card_relation(&state.db, input).await
}

#[tauri::command]
pub async fn delete_card_relation(
    relation_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    relation_service::delete_card_relation(&state.db, relation_id).await
}

#[tauri::command]
pub async fn list_card_relations(
    project_id: String,
    card_id: Option<String>,
    relation_type: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<CardRelationDto>> {
    relation_service::list_card_relations(&state.db, project_id, card_id, relation_type).await
}

#[tauri::command]
pub async fn list_card_graph(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<CardGraphDto> {
    relation_service::list_card_graph(&state.db, project_id).await
}
