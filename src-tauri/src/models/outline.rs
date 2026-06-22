use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OutlineNodeDto {
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub linked_document_id: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub summary: String,
    pub status: String,
    pub sort_order: i64,
    pub metadata_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutlineNodeInput {
    pub project_id: String,
    pub parent_id: Option<String>,
    pub linked_document_id: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub summary: Option<String>,
    pub status: Option<String>,
    pub sort_order: Option<i64>,
    pub metadata_json: Option<String>,
}
