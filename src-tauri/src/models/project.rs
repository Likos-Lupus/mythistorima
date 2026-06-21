use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDto {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub language: String,
    pub cover_asset_id: Option<String>,
    pub target_character_count: Option<i64>,
    pub daily_target_count: Option<i64>,
    pub metadata_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectInput {
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
}
