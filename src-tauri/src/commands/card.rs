use tauri::State;

use crate::{
    errors::AppResult,
    models::card::{CardDto, CardReferenceDto, CreateCardInput, UpdateCardInput},
    services::card_service,
    AppState,
};

#[tauri::command]
pub async fn create_card(input: CreateCardInput, state: State<'_, AppState>) -> AppResult<CardDto> {
    card_service::create_card(&state.db, input).await
}

#[tauri::command]
pub async fn update_card(input: UpdateCardInput, state: State<'_, AppState>) -> AppResult<CardDto> {
    card_service::update_card(&state.db, input).await
}

#[tauri::command]
pub async fn delete_card(card_id: String, state: State<'_, AppState>) -> AppResult<bool> {
    card_service::delete_card(&state.db, card_id).await
}

#[tauri::command]
pub async fn get_card(card_id: String, state: State<'_, AppState>) -> AppResult<CardDto> {
    card_service::get_card(&state.db, card_id).await
}

#[tauri::command]
pub async fn list_cards(
    project_id: String,
    card_type: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<CardDto>> {
    card_service::list_cards(&state.db, project_id, card_type).await
}

#[tauri::command]
pub async fn search_cards(
    project_id: String,
    query: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<CardDto>> {
    card_service::search_cards(&state.db, project_id, query).await
}

#[tauri::command]
pub async fn list_card_references(
    card_id: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<CardReferenceDto>> {
    card_service::list_card_references(&state.db, card_id).await
}
