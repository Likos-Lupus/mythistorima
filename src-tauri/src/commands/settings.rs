use tauri::State;

use crate::{
    errors::AppResult,
    models::settings::{AppSettingDto, SetAppSettingInput},
    services::settings_service,
    AppState,
};

#[tauri::command]
pub async fn list_app_settings(state: State<'_, AppState>) -> AppResult<Vec<AppSettingDto>> {
    settings_service::list_app_settings(&state.db).await
}

#[tauri::command]
pub async fn get_app_setting(
    key: String,
    state: State<'_, AppState>,
) -> AppResult<Option<AppSettingDto>> {
    settings_service::get_app_setting(&state.db, key).await
}

#[tauri::command]
pub async fn set_app_setting(
    input: SetAppSettingInput,
    state: State<'_, AppState>,
) -> AppResult<AppSettingDto> {
    settings_service::set_app_setting(&state.db, input).await
}
