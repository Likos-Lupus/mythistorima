use tauri::State;

use crate::{
    errors::AppResult,
    models::timeline::{
        AttachTimelineEventCardInput, ReorderTimelineEventInput, TimelineEventCardDto,
        TimelineEventDto, TimelineEventInput, UpdateTimelineEventInput,
    },
    services::timeline_service,
    AppState,
};

#[tauri::command]
pub async fn create_timeline_event(
    input: TimelineEventInput,
    state: State<'_, AppState>,
) -> AppResult<TimelineEventDto> {
    timeline_service::create_timeline_event(&state.db, input).await
}

#[tauri::command]
pub async fn update_timeline_event(
    input: UpdateTimelineEventInput,
    state: State<'_, AppState>,
) -> AppResult<TimelineEventDto> {
    timeline_service::update_timeline_event(&state.db, input).await
}

#[tauri::command]
pub async fn delete_timeline_event(
    timeline_event_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    timeline_service::delete_timeline_event(&state.db, timeline_event_id).await
}

#[tauri::command]
pub async fn list_timeline_events(
    project_id: String,
    card_id: Option<String>,
    location_card_id: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<TimelineEventDto>> {
    timeline_service::list_timeline_events(&state.db, project_id, card_id, location_card_id).await
}

#[tauri::command]
pub async fn attach_card_to_timeline_event(
    input: AttachTimelineEventCardInput,
    state: State<'_, AppState>,
) -> AppResult<TimelineEventCardDto> {
    timeline_service::attach_card_to_timeline_event(&state.db, input).await
}

#[tauri::command]
pub async fn detach_card_from_timeline_event(
    timeline_event_id: String,
    card_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    timeline_service::detach_card_from_timeline_event(&state.db, timeline_event_id, card_id).await
}

#[tauri::command]
pub async fn list_timeline_event_cards(
    project_id: String,
    timeline_event_id: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<Vec<TimelineEventCardDto>> {
    timeline_service::list_timeline_event_cards(&state.db, project_id, timeline_event_id).await
}

#[tauri::command]
pub async fn reorder_timeline_event(
    input: ReorderTimelineEventInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<TimelineEventDto>> {
    timeline_service::reorder_timeline_event(&state.db, input).await
}
