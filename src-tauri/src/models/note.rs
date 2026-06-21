use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct NoteDto {
    pub id: String,
    pub project_id: String,
    pub document_id: Option<String>,
    pub paragraph_id: Option<String>,
    #[serde(rename = "type")]
    pub note_type: String,
    pub title: String,
    pub body: String,
    pub status: String,
    pub priority: String,
    pub due_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteInput {
    pub project_id: String,
    pub document_id: Option<String>,
    pub paragraph_id: Option<String>,
    #[serde(rename = "type")]
    pub note_type: String,
    pub title: String,
    pub body: Option<String>,
    pub priority: Option<String>,
    pub due_at: Option<i64>,
}
