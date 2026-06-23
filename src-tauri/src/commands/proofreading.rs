use tauri::State;

use crate::{
    errors::AppResult,
    models::proofreading::{
        CreateProofreadingRuleInput, ListProofreadingRulesInput, ProofreadingIssueDto,
        ProofreadingRuleDto, RunProofreadingOnDocumentInput, RunProofreadingOnProjectInput,
        UpdateProofreadingRuleInput,
    },
    services::proofreading_service,
    AppState,
};

#[tauri::command]
pub async fn create_proofreading_rule(
    input: CreateProofreadingRuleInput,
    state: State<'_, AppState>,
) -> AppResult<ProofreadingRuleDto> {
    proofreading_service::create_proofreading_rule(&state.db, input).await
}

#[tauri::command]
pub async fn update_proofreading_rule(
    input: UpdateProofreadingRuleInput,
    state: State<'_, AppState>,
) -> AppResult<ProofreadingRuleDto> {
    proofreading_service::update_proofreading_rule(&state.db, input).await
}

#[tauri::command]
pub async fn delete_proofreading_rule(
    rule_id: String,
    state: State<'_, AppState>,
) -> AppResult<bool> {
    proofreading_service::delete_proofreading_rule(&state.db, rule_id).await
}

#[tauri::command]
pub async fn list_proofreading_rules(
    input: ListProofreadingRulesInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<ProofreadingRuleDto>> {
    proofreading_service::list_proofreading_rules(&state.db, input).await
}

#[tauri::command]
pub async fn run_proofreading_on_document(
    input: RunProofreadingOnDocumentInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<ProofreadingIssueDto>> {
    proofreading_service::run_proofreading_on_document(&state.db, input).await
}

#[tauri::command]
pub async fn run_proofreading_on_project(
    input: RunProofreadingOnProjectInput,
    state: State<'_, AppState>,
) -> AppResult<Vec<ProofreadingIssueDto>> {
    proofreading_service::run_proofreading_on_project(&state.db, input).await
}
