use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDto {
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    #[serde(rename = "type")]
    pub document_type: String,
    pub title: String,
    pub sort_order: i64,
    pub status: String,
    pub summary: Option<String>,
    pub metadata_json: String,
    pub character_count: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContentDto {
    pub document_id: String,
    pub schema_version: i64,
    pub content_json: Value,
    pub content_text: String,
    pub content_html: String,
    pub character_count: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentInput {
    pub project_id: String,
    pub parent_id: Option<String>,
    #[serde(default = "default_document_type", rename = "type")]
    pub document_type: String,
    pub title: String,
    pub sort_order: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveDocumentInput {
    pub document_id: String,
    pub parent_id: Option<String>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentContentInput {
    pub document_id: String,
    pub content_json: Value,
    pub content_text: String,
    pub content_html: String,
    pub character_count: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentStatusInput {
    pub document_id: String,
    pub status: String,
}

fn default_document_type() -> String {
    "chapter".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentGoalInput {
    pub document_id: String,
    pub target_character_count: Option<i64>,
}
