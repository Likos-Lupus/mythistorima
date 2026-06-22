use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CardRelationDto {
    pub id: String,
    pub project_id: String,
    pub source_card_id: String,
    pub target_card_id: String,
    pub relation_type: String,
    pub description: String,
    pub direction: String,
    pub metadata_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardRelationInput {
    pub project_id: String,
    pub source_card_id: String,
    pub target_card_id: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub direction: Option<String>,
    pub metadata_json: Option<String>,
}
