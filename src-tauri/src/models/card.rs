use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CardDto {
    pub id: String,
    pub project_id: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub name: String,
    pub aliases_json: String,
    pub description: String,
    pub fields_json: String,
    pub avatar_asset_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCardInput {
    pub project_id: String,
    #[serde(rename = "type")]
    pub card_type: String,
    pub name: String,
    pub aliases_json: Option<String>,
    pub description: Option<String>,
    pub fields_json: Option<String>,
    pub avatar_asset_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCardInput {
    pub card_id: String,
    #[serde(rename = "type")]
    pub card_type: Option<String>,
    pub name: Option<String>,
    pub aliases_json: Option<String>,
    pub description: Option<String>,
    pub fields_json: Option<String>,
    pub avatar_asset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CardReferenceDto {
    pub id: String,
    pub project_id: String,
    pub document_id: String,
    pub document_title: Option<String>,
    pub card_id: String,
    pub display_text: String,
    pub from_pos: Option<i64>,
    pub to_pos: Option<i64>,
    pub paragraph_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
