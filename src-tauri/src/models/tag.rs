use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TagDto {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub color: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TaggingDto {
    pub id: String,
    pub project_id: String,
    pub tag_id: String,
    pub target_type: String,
    pub target_id: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagInput {
    pub project_id: String,
    pub name: String,
    pub color: Option<String>,
}
