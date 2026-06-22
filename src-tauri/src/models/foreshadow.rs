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
    pub payoff_note_id: Option<String>,
    pub setup_document_id: Option<String>,
    pub payoff_document_id: Option<String>,
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
