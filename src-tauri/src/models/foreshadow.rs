use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ForeshadowThreadDto {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub setup_note_id: Option<String>,
    pub setup_note_title: Option<String>,
    pub setup_paragraph_id: Option<String>,
    pub payoff_note_id: Option<String>,
    pub payoff_note_title: Option<String>,
    pub payoff_paragraph_id: Option<String>,
    pub setup_document_id: Option<String>,
    pub setup_document_title: Option<String>,
    pub payoff_document_id: Option<String>,
    pub payoff_document_title: Option<String>,
    pub priority: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeshadowThreadInput {
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub setup_note_id: Option<String>,
    pub payoff_note_id: Option<String>,
    pub setup_document_id: Option<String>,
    pub payoff_document_id: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateForeshadowThreadInput {
    pub thread_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub setup_note_id: Option<Option<String>>,
    pub payoff_note_id: Option<Option<String>>,
    pub setup_document_id: Option<Option<String>>,
    pub payoff_document_id: Option<Option<String>>,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeshadowThreadFromNoteInput {
    pub note_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub payoff_document_id: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListForeshadowThreadsInput {
    pub project_id: String,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub only_unpaid: Option<bool>,
}
