use tauri::State;

use crate::{
    errors::AppResult,
    models::{
        export::ExportResultDto,
        export_template::{
            CreateExportTemplateInput, ExportPreviewDto, ExportTemplateDto,
            ExportWithTemplateInput, ListExportTemplatesInput, UpdateExportTemplateInput,
        },
    },
    services::export_template_service,
    AppState,
};

#[tauri::command]
pub async fn create_export_template(
    input: CreateExportTemplateInput,
    state: State<'_, AppState>,
) -> AppResult<ExportTemplateDto> {
    export_template_service::create_export_template(&state.db, input).await
}

#[tauri::command]
pub async fn update_export_template(
    input: UpdateExportTemplateInput,
    state: State<'_, AppState>,
) -> AppResult<ExportTemplateDto> {
    export_template_service::update_export_template(&state.db, input).await
}

#[tauri::command]
pub async fn delete_export_template(
    project_id: String,
    template_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    export_template_service::delete_export_template(&state.db, project_id, template_id).await
}

#[tauri::command]
pub async fn list_export_templates(
    input: ListExportTemplatesInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<ExportTemplateDto>> {
    export_template_service::list_export_templates(&state.db, input).await
}

#[tauri::command]
pub async fn export_with_template(
    input: ExportWithTemplateInput,
    state: State<'_, AppState>,
) -> AppResult<ExportResultDto> {
    export_template_service::export_with_template(&state.db, &state.database_path, input).await
}

#[tauri::command]
pub async fn preview_export_with_template(
    input: ExportWithTemplateInput,
    state: State<'_, AppState>,
) -> AppResult<ExportPreviewDto> {
    export_template_service::preview_export_with_template(&state.db, input).await
}
