use tauri::State;

use crate::{
    errors::AppResult,
    models::stats::{
        DocumentStatsDto, ProjectOverviewDto, ProjectOverviewInput, ProjectStatsDto,
        RecordWritingSessionInput, TodayWritingStatsDto, WritingSessionDto,
    },
    services::stats_service,
    AppState,
};

#[tauri::command]
pub async fn get_project_stats(
    project_id: String,
    state: State<'_, AppState>,
) -> AppResult<ProjectStatsDto> {
    stats_service::get_project_stats(&state.db, project_id).await
}

#[tauri::command]
pub async fn get_document_stats(
    document_id: String,
    state: State<'_, AppState>,
) -> AppResult<DocumentStatsDto> {
    stats_service::get_document_stats(&state.db, document_id).await
}

#[tauri::command]
pub async fn record_writing_session(
    input: RecordWritingSessionInput,
    state: State<'_, AppState>,
) -> AppResult<WritingSessionDto> {
    stats_service::record_writing_session(&state.db, input).await
}

#[tauri::command]
pub async fn get_today_writing_stats(
    project_id: String,
    day_start: i64,
    day_end: i64,
    state: State<'_, AppState>,
) -> AppResult<TodayWritingStatsDto> {
    stats_service::get_today_writing_stats(&state.db, project_id, day_start, day_end).await
}

#[tauri::command]
pub async fn get_project_overview(
    input: ProjectOverviewInput,
    state: State<'_, AppState>,
) -> AppResult<ProjectOverviewDto> {
    stats_service::get_project_overview(&state.db, input).await
}
